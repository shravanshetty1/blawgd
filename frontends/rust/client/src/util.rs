use crate::blawgd_client::{AccountInfo, MsgLikePost, MsgRepost, PostView};
use crate::clients::verification_client::VerificationClient;
use anyhow::Result;
use cosmos_sdk_proto::cosmos::{
    auth::v1beta1::query_client::QueryClient,
    auth::v1beta1::{BaseAccount, QueryAccountRequest},
    tx::v1beta1::service_client::ServiceClient,
    tx::v1beta1::{BroadcastMode, BroadcastTxRequest, BroadcastTxResponse, Tx, TxRaw},
};
use crw_client::tx::TxBuilder;
use crw_wallet::crypto::MnemonicWallet;
use gloo::events;
use wasm_bindgen::JsValue;

pub const MSG_TYPE_CREATE_POST: &str = "/blawgd.MsgCreatePost";
pub const MSG_TYPE_FOLLOW: &str = "/blawgd.MsgFollow";
pub const MSG_TYPE_STOP_FOLLOW: &str = "/blawgd.MsgStopFollow";
pub const MSG_TYPE_LIKE: &str = "/blawgd.MsgLikePost";
pub const MSG_TYPE_REPOST: &str = "/blawgd.MsgRepost";
pub const MSG_TYPE_UPDATE_ACCOUNT_INFO: &str = "/blawgd.MsgUpdateAccountInfo";
pub const ADDRESS_HRP: &str = "cosmos";

pub async fn is_following(
    cl: VerificationClient,
    address1: String,
    address2: String,
) -> Result<bool> {
    let followings = cl.get_following_list(address1).await?;

    let mut is_following: bool = false;
    for following in followings {
        if following.to_string() == address2 {
            is_following = true;
        }
    }

    Ok(is_following)
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
    mode: i32,
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

    let resp = ServiceClient::new(client)
        .broadcast_tx(BroadcastTxRequest {
            tx_bytes: tx_raw,
            mode,
        })
        .await
        .unwrap();

    // wait for another block to get committed since light client is 1 block behind
    gloo::timers::future::TimeoutFuture::new(800).await;

    resp
}

pub fn register_post_event_listener(
    wallet: MnemonicWallet,
    client: grpc_web_client::Client,
    address: String,
    post: PostView,
) {
    let window = web_sys::window().unwrap();
    let document = window.document().expect("document missing");
    let like_button_wrapper_id = format!("post-{}-like", post.id);
    let like_button_wrapper = document
        .get_element_by_id(like_button_wrapper_id.as_str())
        .unwrap();
    let like_button_id = format!("post-{}-like-content", post.id);
    let like_button = document.get_element_by_id(like_button_id.as_str()).unwrap();

    let client1 = client.clone();
    let wallet1 = wallet.clone();
    let address1 = address.clone();
    let post1 = post.clone();
    events::EventListener::new(&like_button_wrapper, "click", move |_| {
        let address = address1.clone();
        let post = post1.clone();
        let wallet = wallet1.clone();
        let client = client1.clone();

        let like_button_text: String = like_button.inner_html();
        let likes_count_text = like_button_text
            .strip_suffix(" Likes")
            .unwrap_or("0")
            .to_string();
        let mut likes_count = likes_count_text.parse::<i32>().unwrap_or(0);
        likes_count += 1;
        like_button.set_inner_html(format!("{} Likes", likes_count).as_str());

        wasm_bindgen_futures::spawn_local(async move {
            let resp = broadcast_tx(
                &wallet,
                client,
                MSG_TYPE_LIKE,
                MsgLikePost {
                    creator: address,
                    post_id: post.id,
                    amount: 1,
                },
                BroadcastMode::Sync as i32,
            )
            .await;

            console_log(resp.into_inner().tx_response.unwrap().raw_log.as_str())
        });
    })
    .forget();

    let repost_button_wrapper_id = format!("post-{}-repost", post.id);
    let repost_button_wrapper = document
        .get_element_by_id(repost_button_wrapper_id.as_str())
        .unwrap();
    let repost_button_id = format!("post-{}-repost-content", post.id);
    let repost_button = document
        .get_element_by_id(repost_button_id.as_str())
        .unwrap();
    events::EventListener::new(&repost_button_wrapper, "click", move |_| {
        let address = address.clone();
        let post = post.clone();
        let wallet = wallet.clone();
        let client = client.clone();

        let repost_button_text: String = repost_button.inner_html();
        let repost_count_text = repost_button_text
            .strip_suffix(" Reposts")
            .unwrap_or("0")
            .to_string();
        let mut repost_count = repost_count_text.parse::<i32>().unwrap_or(0);
        repost_count += 1;
        repost_button.set_inner_html(format!("{} Reposts", repost_count).as_str());

        wasm_bindgen_futures::spawn_local(async move {
            let resp = broadcast_tx(
                &wallet,
                client,
                MSG_TYPE_REPOST,
                MsgRepost {
                    creator: address,
                    post_id: post.id,
                },
                BroadcastMode::Sync as i32,
            )
            .await;

            console_log(resp.into_inner().tx_response.unwrap().raw_log.as_str())
        });
    })
    .forget();
}
