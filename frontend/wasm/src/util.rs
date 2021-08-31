use crate::blawgd_client::{
    query_client::QueryClient as BlawgdClient, AccountInfo, AccountInfoView, FollowingCount,
    GetFollowingsRequest, GetProfileInfoRequest,
};
use cosmos_sdk_proto::cosmos::{
    auth::v1beta1::query_client::QueryClient,
    auth::v1beta1::{BaseAccount, QueryAccountRequest},
    tx::v1beta1::service_client::ServiceClient,
    tx::v1beta1::{BroadcastMode, BroadcastTxRequest, BroadcastTxResponse, Tx, TxRaw},
};
use crw_client::tx::TxBuilder;
use crw_wallet::crypto::MnemonicWallet;
use wasm_bindgen::JsValue;

pub const COSMOS_DP: &str = "m/44'/118'/0'/0/0";
pub const HOST_NAME: &str = "http://localhost:2341";
pub const GRPC_WEB_ADDRESS: &str = "http://localhost:9091";
pub const MSG_TYPE_CREATE_POST: &str = "/shravanshetty1.samachar.samachar.MsgCreatePost";
pub const MSG_TYPE_FOLLOW: &str = "/shravanshetty1.samachar.samachar.MsgFollow";
pub const MSG_TYPE_STOP_FOLLOW: &str = "/shravanshetty1.samachar.samachar.MsgStopFollow";
pub const MSG_TYPE_UPDATE_ACCOUNT_INFO: &str =
    "/shravanshetty1.samachar.samachar.MsgUpdateAccountInfo";
pub const ADDRESS_HRP: &str = "cosmos";
pub(crate) const TENDERMINT_HOST: &str = "http://localhost:26657";

pub struct StoredData {
    pub mnemonic: String,
    pub address: String,
}

pub fn set_stored_data(storage: &web_sys::Storage, stored_data: StoredData) {
    storage.set_item("mnemonic", stored_data.mnemonic.as_str());
    storage.set_item("address", stored_data.address.as_str());
}

pub async fn is_following(
    client: grpc_web_client::Client,
    address1: String,
    address2: String,
) -> bool {
    let followings = BlawgdClient::new(client)
        .get_followings(GetFollowingsRequest { address: address1 })
        .await
        .unwrap()
        .get_ref()
        .addresses
        .clone();

    let mut is_following: bool = false;
    for following in followings {
        if following == address2 {
            is_following = true;
        }
    }

    is_following
}

pub fn get_stored_data(storage: &web_sys::Storage) -> Option<StoredData> {
    let mnemonic_result = storage.get_item("mnemonic");
    let mut mnemonic: String = String::new();
    if mnemonic_result.is_ok() {
        if mnemonic_result.as_ref().unwrap().is_some() {
            mnemonic = mnemonic_result.unwrap().unwrap();
        }
    }

    let address_result = storage.get_item("address");
    let mut address: String = String::new();
    if address_result.is_ok() {
        if address_result.as_ref().unwrap().is_some() {
            address = address_result.unwrap().unwrap();
        }
    }

    if mnemonic.is_empty() || address.is_empty() {
        return None;
    }

    Some(StoredData { mnemonic, address })
}

pub fn remove_stored_data(storage: &web_sys::Storage) {
    storage.remove_item("mnemonic");
    storage.remove_item("address");
}

pub async fn get_session_account_info(
    storage: &web_sys::Storage,
    client: grpc_web_client::Client,
) -> Option<AccountInfoView> {
    let stored_data = get_stored_data(storage);
    if stored_data.is_none() {
        return None;
    }
    Some(get_account_info(client, stored_data.unwrap().address).await)
}

pub fn get_wallet(storage: &web_sys::Storage) -> Result<MnemonicWallet, &str> {
    let stored_data = get_stored_data(storage);

    // Validation
    if stored_data.is_none() {
        return Err("cannot create wallet since user has not logged in");
    }

    Ok(
        crw_wallet::crypto::MnemonicWallet::new(stored_data.unwrap().mnemonic.as_str(), COSMOS_DP)
            .expect("could not generate alice wallet"),
    )
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
    msg_type: &str,
    msg: M,
) -> tonic::Response<BroadcastTxResponse> {
    let acc_resp = QueryClient::new(client.clone())
        .account(QueryAccountRequest {
            address: wallet.get_bech32_address(ADDRESS_HRP).unwrap(),
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

pub async fn get_account_info(client: grpc_web_client::Client, address: String) -> AccountInfoView {
    let resp = super::blawgd_client::query_client::QueryClient::new(client)
        .get_profile_info(GetProfileInfoRequest {
            address: address.clone(),
            height: 0,
        })
        .await
        .unwrap();
    let resp = resp.into_inner();
    let account_info = resp.account_info.values().next().cloned();
    let following_count = resp.following_count.values().next().cloned();

    let mut account_info = AccountInfoView {
        account_info,
        following_count: following_count
            .unwrap_or(FollowingCount {
                address: address.clone(),
                count: 0,
            })
            .count as i64,
    };

    account_info.account_info = Some(normalize_account_info(
        account_info.account_info.unwrap(),
        address,
    ));
    account_info
}

pub fn normalize_account_info(mut account_info: AccountInfo, address: String) -> AccountInfo {
    account_info.address = address.clone();
    if account_info.photo.is_empty() {
        account_info.photo = "/profile.jpeg".into();
    }
    if account_info.name.is_empty() {
        let name_suffix: String = address.chars().skip(address.len() - 5).take(5).collect();
        account_info.name = format!("anon{}", name_suffix);
    }
    account_info
}
