use actix_web::body::Body;
use actix_web::http::StatusCode;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use cosmos_sdk_proto::cosmos::auth::v1beta1::query_client::QueryClient;
use cosmos_sdk_proto::cosmos::auth::v1beta1::{BaseAccount, QueryAccountRequest};
use cosmos_sdk_proto::cosmos::bank;
use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
use cosmos_sdk_proto::cosmos::tx::v1beta1::service_client::ServiceClient;
use cosmos_sdk_proto::cosmos::tx::v1beta1::{
    BroadcastMode, BroadcastTxRequest, BroadcastTxResponse, Tx, TxRaw,
};
use crw_client::tx::TxBuilder;
use crw_client::CosmosError;
use crw_wallet::crypto::MnemonicWallet;
use qstring::QString;
use std::ops::Index;

pub const COSMOS_DP: &str = "m/44'/118'/0'/0/0";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        panic!(format!("unexpected len of args {}", args.len()))
    }

    let mnemonic = args.index(1).clone();
    let broadcast_node_addr = args.index(2).clone();

    let wallet = crw_wallet::crypto::MnemonicWallet::new(mnemonic.as_str(), COSMOS_DP)
        .expect("could not generate alice wallet");

    let sender = State::new(wallet, broadcast_node_addr);
    println!("{}", mnemonic);
    println!("{}", sender.sender_addr);
    println!("started faucet at port 2342");

    HttpServer::new(move || {
        let state = web::Data::new(sender.clone());
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();
        App::new()
            .wrap(cors)
            .app_data(state)
            .route("/", web::get().to(handler))
    })
    .bind(("127.0.0.1", 2342))?
    .run()
    .await
}

async fn grpc_client(grpc_addr: String) -> Result<tonic::transport::Channel, CosmosError> {
    let grpc_uri = grpc_addr
        .parse::<tonic::codegen::http::Uri>()
        .map_err(|err| CosmosError::Grpc(err.to_string()))?;
    let grpc_channel = tonic::transport::Channel::builder(grpc_uri);

    Ok(grpc_channel
        .connect()
        .await
        .map_err(|err| CosmosError::Grpc(err.to_string()))?)
}

struct State {
    wallet: MnemonicWallet,
    grpc_addr: String,
    sender_addr: String,
}

impl State {
    fn new(wallet: MnemonicWallet, grpc_addr: String) -> State {
        let sender_addr = wallet.get_bech32_address("cosmos").unwrap();
        State {
            wallet,
            grpc_addr,
            sender_addr,
        }
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        State {
            wallet: self.wallet.clone(),
            grpc_addr: self.grpc_addr.clone(),
            sender_addr: self.sender_addr.clone(),
        }
    }
}

async fn handler(state: web::Data<State>, req: HttpRequest) -> HttpResponse {
    let qs = QString::from(req.query_string());
    let to_address_option = qs.get("address");
    let mut to_address: String = String::new();
    if to_address_option.is_some() {
        to_address = to_address_option.unwrap().to_string();
    } else {
        return actix_web::HttpResponse::build(StatusCode::BAD_REQUEST)
            .content_type("text/json")
            .body("address query parameter is mandatory");
    }

    let client = grpc_client(state.grpc_addr.clone().into()).await.unwrap();
    let acc_resp = QueryClient::new(client.clone())
        .account(QueryAccountRequest {
            address: to_address.clone(),
        })
        .await;

    if acc_resp.is_ok() {
        return actix_web::HttpResponse::build(StatusCode::BAD_REQUEST)
            .content_type("text/json")
            .body("account with address already registered, faucet will only send tokens to account that have yet to be registered");
    }

    let msg = bank::v1beta1::MsgSend {
        from_address: state.sender_addr.clone(),
        to_address,
        amount: vec![Coin {
            denom: "stake".to_string(),
            amount: "100000".to_string(),
        }],
    };

    let res = broadcast_tx(&state.wallet, client, "/cosmos.bank.v1beta1.MsgSend", msg)
        .await
        .get_ref()
        .tx_response
        .clone();

    let mut log: String = String::new();
    if res.is_some() {
        log = res.unwrap().raw_log;
    }

    actix_web::HttpResponse::build(StatusCode::OK)
        .content_type("text/json")
        .body(log)
}

pub async fn broadcast_tx<M: prost::Message>(
    wallet: &MnemonicWallet,
    client: tonic::transport::Channel,
    msg_type: &str,
    msg: M,
) -> tonic::Response<BroadcastTxResponse> {
    let acc_resp = QueryClient::new(client.clone())
        .account(QueryAccountRequest {
            address: wallet.get_bech32_address("cosmos").unwrap(),
        })
        .await
        .unwrap();

    let account_data: BaseAccount =
        prost::Message::decode(acc_resp.get_ref().account.as_ref().unwrap().value.as_ref())
            .unwrap();
    let tx = TxBuilder::new("blawgd")
        .memo("Test memo")
        .account_info(account_data.sequence, account_data.account_number)
        .timeout_height(0)
        .fee("stake", "0", 3000000)
        .add_message(msg_type, msg)
        .unwrap()
        .sign(wallet)
        .expect("could not build tx");
    let tx_raw = serialize_tx(&tx);

    ServiceClient::new(client)
        .broadcast_tx(BroadcastTxRequest {
            tx_bytes: tx_raw,
            mode: BroadcastMode::Sync as i32,
        })
        .await
        .unwrap()
}

pub fn serialize_tx(tx: &Tx) -> Vec<u8> {
    let mut serialized_body: Vec<u8> = Vec::new();
    let mut serialized_auth: Vec<u8> = Vec::new();
    let mut serialized_tx: Vec<u8> = Vec::new();

    // Serialize the tx body and auth_info
    if let Some(body) = &tx.body {
        prost::Message::encode(body, &mut serialized_body);
    }
    if let Some(auth_info) = &tx.auth_info {
        prost::Message::encode(auth_info, &mut serialized_auth);
    }

    // Prepare and serialize the TxRaw
    let tx_raw = TxRaw {
        body_bytes: serialized_body,
        auth_info_bytes: serialized_auth,
        signatures: tx.signatures.clone(),
    };
    prost::Message::encode(&tx_raw, &mut serialized_tx);
    serialized_tx
}
