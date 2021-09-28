use crate::cosmos_client::{CosmosClient, ADDRESS_HRP, MSG_BANK_SEND};
use actix_web::http::StatusCode;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use anyhow::anyhow;
use anyhow::Result;
use cosmos_sdk_proto::cosmos::auth::v1beta1::query_client::QueryClient;
use cosmos_sdk_proto::cosmos::auth::v1beta1::QueryAccountRequest;
use cosmos_sdk_proto::cosmos::bank;
use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
use cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastMode;
use crw_wallet::crypto::MnemonicWallet;
use hcaptcha::{HcaptchaCaptcha, HcaptchaClient, HcaptchaRequest};
use prost::alloc::fmt::Formatter;
use qstring::QString;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Result as fmtResult;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Request {
    address: String,
    captcha: String,
}

#[derive(Clone)]
pub struct State {
    pub wallet: MnemonicWallet,
    pub client: CosmosClient,
    pub captcha_site_key: String,
    pub captcha_secret: String,
}

#[derive(Debug, Clone)]
pub struct ExpectedError {
    pub code: StatusCode,
    pub msg: String,
}

impl Display for ExpectedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "{}", self.msg.as_str())?;
        Ok(())
    }
}

impl Error for ExpectedError {}

pub const DENOM: &str = "stake";
pub const AMOUNT: &str = "1";

pub async fn faucet(state: web::Data<State>, req: web::Json<Request>) -> HttpResponse {
    let resp = faucet_handler(state, req).await;
    if resp.is_err() {
        let err = resp.err().unwrap();
        let expected_err = err.downcast_ref::<ExpectedError>();
        if expected_err.is_some() {
            let err = expected_err.unwrap().clone();
            return HttpResponse::build(err.code)
                .content_type("text/json")
                .body(err.msg);
        }
        println!("unexpected error - {}", err);
        return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("text/json")
            .body("unexpected error, something went wrong");
    }

    let resp = resp.unwrap();
    HttpResponse::build(StatusCode::OK)
        .content_type("text/json")
        .body(resp)
}

pub async fn site_key(state: web::Data<State>, req: HttpRequest) -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/json")
        .body(state.captcha_site_key.clone())
}

pub async fn faucet_handler(state: web::Data<State>, req: web::Json<Request>) -> Result<String> {
    let cap_req = HcaptchaRequest::new(
        state.captcha_secret.clone().as_str(),
        HcaptchaCaptcha::new(req.captcha.clone().as_str())?,
    )?;
    let cap_resp = HcaptchaClient::new()
        .verify_client_response(cap_req)
        .await?;
    if !cap_resp.success() {
        return Err(anyhow::Error::new(ExpectedError {
            code: StatusCode::BAD_REQUEST,
            msg: "the captcha response wasn invalid".to_string(),
        }));
    }

    let msg = bank::v1beta1::MsgSend {
        from_address: state.wallet.get_bech32_address(ADDRESS_HRP)?,
        to_address: req.address.clone(),
        amount: vec![Coin {
            denom: DENOM.to_string(),
            amount: AMOUNT.to_string(),
        }],
    };

    let res = state
        .client
        .broadcast_tx(
            &state.wallet,
            MSG_BANK_SEND,
            msg,
            BroadcastMode::Block as i32,
        )
        .await?
        .get_ref()
        .tx_response
        .clone();

    let resp = res.ok_or(anyhow!("invalid resp"))?.raw_log.clone();
    Ok(resp)
}
