use crate::clients::verification_client::VerificationClient;
use anyhow::Result;
use wasm_bindgen::JsValue;

// TODO remove this file

pub const MSG_TYPE_CREATE_POST: &str = "/blawgd.MsgCreatePost";
pub const MSG_TYPE_FOLLOW: &str = "/blawgd.MsgFollow";
pub const MSG_TYPE_STOP_FOLLOW: &str = "/blawgd.MsgStopFollow";
pub const MSG_TYPE_LIKE: &str = "/blawgd.MsgLikePost";
pub const MSG_TYPE_REPOST: &str = "/blawgd.MsgRepost";
pub const MSG_TYPE_UPDATE_ACCOUNT_INFO: &str = "/blawgd.MsgUpdateAccountInfo";

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

// TODO add logger object
pub fn console_log(message: &str) {
    web_sys::console::log_1(&JsValue::from_str(message))
}
