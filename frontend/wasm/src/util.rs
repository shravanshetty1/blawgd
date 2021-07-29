use crate::blawgd_client::AccountInfo;
use cosmos_sdk_proto::cosmos::auth::v1beta1::query_client::QueryClient;
use cosmos_sdk_proto::cosmos::auth::v1beta1::{BaseAccount, QueryAccountRequest};
use cosmos_sdk_proto::cosmos::tx::v1beta1::service_client::ServiceClient;
use cosmos_sdk_proto::cosmos::tx::v1beta1::{
    BroadcastMode, BroadcastTxRequest, BroadcastTxResponse, Tx, TxRaw,
};
use crw_client::tx::TxBuilder;
use crw_wallet::crypto::MnemonicWallet;
use wasm_bindgen::JsValue;

pub fn get_account_info_from_storage(storage: &web_sys::Storage) -> Option<AccountInfo> {
    let account_info_raw: Option<String> = storage.get_item("account_info").unwrap();
    let mut account_info: Option<AccountInfo> = None;
    if account_info_raw.is_some() {
        let account_info_string = account_info_raw.unwrap();
        if !account_info_string.is_empty() {
            account_info = Some(prost::Message::decode(account_info_string.as_bytes()).unwrap());
        }
    }
    account_info
}

pub fn get_mnemonic_from_storage(storage: &web_sys::Storage) -> Option<String> {
    storage.get_item("wallet_mnemonic").unwrap()
}

pub fn console_log(message: &str) {
    web_sys::console::log_1(&JsValue::from_str(message))
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

pub async fn broadcast_tx<M: prost::Message>(
    wallet: &MnemonicWallet,
    client: grpc_web_client::Client,
    address: &str,
    msg_type: &str,
    msg: M,
) -> tonic::Response<BroadcastTxResponse> {
    let acc_resp = QueryClient::new(client.clone())
        .account(QueryAccountRequest {
            address: address.into(),
        })
        .await
        .unwrap();

    let account_data: BaseAccount =
        prost::Message::decode(acc_resp.get_ref().account.as_ref().unwrap().value.as_ref())
            .unwrap();
    let tx = TxBuilder::new("samachar")
        .memo("Test memo")
        .account_info(account_data.sequence, account_data.account_number)
        .timeout_height(0)
        .fee("token", "0", 3000000)
        .add_message(msg_type, msg)
        .unwrap()
        .sign(wallet)
        .expect("could not build tx");
    let tx_raw = super::util::serialize_tx(&tx);

    ServiceClient::new(client)
        .broadcast_tx(BroadcastTxRequest {
            tx_bytes: tx_raw,
            mode: BroadcastMode::Block as i32,
        })
        .await
        .unwrap()
}
