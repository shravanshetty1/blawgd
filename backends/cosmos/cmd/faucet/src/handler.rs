use crate::cosmos_client::{CosmosClient, ADDRESS_HRP, MSG_BANK_SEND};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};
use anyhow::anyhow;
use anyhow::Result;
use cosmos_sdk_proto::cosmos::auth::v1beta1::query_client::QueryClient;
use cosmos_sdk_proto::cosmos::auth::v1beta1::QueryAccountRequest;
use cosmos_sdk_proto::cosmos::bank;
use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
use cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastMode;
use crw_wallet::crypto::MnemonicWallet;
use prost::alloc::fmt::Formatter;
use qstring::QString;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Result as fmtResult;

#[derive(Clone)]
pub struct State {
    pub wallet: MnemonicWallet,
    pub client: CosmosClient,
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

pub async fn faucet(state: web::Data<State>, req: HttpRequest) -> HttpResponse {
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

pub async fn faucet_handler(state: web::Data<State>, req: HttpRequest) -> Result<String> {
    let qs = QString::from(req.query_string());
    let to_address = qs
        .get("address")
        .ok_or(ExpectedError {
            code: StatusCode::BAD_REQUEST,
            msg: "could not get address from query params".to_string(),
        })?
        .to_string();

    let resp = QueryClient::new(state.client.client.clone())
        .account(QueryAccountRequest {
            address: to_address.clone(),
        })
        .await;
    if resp.is_ok() {
        return Err(anyhow::Error::from(ExpectedError {
            code: StatusCode::BAD_REQUEST,
            msg: "account with address already registered, faucet will only send tokens to account that have yet to be registered".to_string(),
        }));
    }

    let msg = bank::v1beta1::MsgSend {
        from_address: state.wallet.get_bech32_address(ADDRESS_HRP)?,
        to_address,
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
