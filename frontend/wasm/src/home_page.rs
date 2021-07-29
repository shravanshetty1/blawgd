use crate::blawgd_client::GetPostsRequest;
use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::home_page::HomePage;
use crate::components::nav_bar::NavBar;
use crate::components::post::Post;
use crate::components::post_creator::PostCreator;
use crate::components::Component;
use bip39::{Language, Mnemonic, MnemonicType};
use cosmos_sdk_proto::cosmos::auth::v1beta1::query_client::QueryClient;
use cosmos_sdk_proto::cosmos::auth::v1beta1::{BaseAccount, QueryAccountRequest};
use cosmos_sdk_proto::cosmos::tx::v1beta1::service_client::ServiceClient;
use cosmos_sdk_proto::cosmos::tx::v1beta1::{BroadcastMode, BroadcastTxRequest};
use crw_client::client::CosmosClient;
use crw_client::tx::TxBuilder;
use gloo::events;
use wasm_bindgen::JsCast;

pub fn handle() {
    wasm_bindgen_futures::spawn_local(async move {
        let window = web_sys::window().unwrap();
        let document = window.document().expect("document missing");
        let storage = window
            .local_storage()
            .expect("storage object missing")
            .unwrap();

        let client = grpc_web_client::Client::new("http://localhost:9091".into());
        let posts_resp = super::blawgd_client::query_client::QueryClient::new(client)
            .get_posts(GetPostsRequest { index: 0 })
            .await
            .unwrap();

        let mut posts: Vec<Box<dyn Component>> = Vec::new();
        for post in &posts_resp.get_ref().posts {
            let post_comp = Post::new(post.clone());
            posts.push(post_comp)
        }

        let account_info = super::util::get_account_info_from_storage(&storage);
        let nav_bar = NavBar::new(account_info.clone());
        let post_creator = PostCreator::new();
        let comp = BlawgdHTMLDoc::new(HomePage::new(
            nav_bar,
            post_creator,
            posts.into_boxed_slice(),
        ));

        let body = document.body().expect("body missing");
        body.set_inner_html(&comp.to_html());

        let post_creator_button = document
            .get_element_by_id("post-creator-button")
            .expect("post-creator-button element not found");

        events::EventListener::new(&post_creator_button, "click", move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let window = web_sys::window().unwrap();
                let storage = window.local_storage().unwrap().unwrap();
                let mnemonic = super::util::get_mnemonic_from_storage(&storage);
                let account_info = super::util::get_account_info_from_storage(&storage);

                // Validation
                if mnemonic.is_none() {
                    web_sys::console::log_1(&"cannot post since user has not logged in".into());
                    return;
                }

                let cosmos_dp = "m/44'/118'/0'/0/0";
                let wallet =
                    crw_wallet::crypto::MnemonicWallet::new(mnemonic.unwrap().as_str(), cosmos_dp)
                        .expect("could not generate alice wallet");

                let document = web_sys::window()
                    .unwrap()
                    .document()
                    .expect("document missing");
                let post_creator_input = document
                    .get_element_by_id("post-creator-input")
                    .expect("post-creator-input element not found");
                let post_content: String = post_creator_input
                    .dyn_ref::<web_sys::HtmlTextAreaElement>()
                    .unwrap()
                    .value();

                let msg = super::blawgd_client::MsgCreatePost {
                    creator: account_info.unwrap().address.clone(),
                    content: post_content,
                    parent_post: "".to_string(),
                    metadata: "".to_string(),
                };

                super::util::console_log(msg.creator.as_str());

                let client = grpc_web_client::Client::new("http://localhost:9091".into());
                let tx_resp = super::util::broadcast_tx(
                    &wallet,
                    client,
                    msg.creator.clone().as_str(),
                    "/shravanshetty1.samachar.samachar.MsgCreatePost",
                    msg,
                )
                .await;

                let tx_resp_obj = tx_resp.get_ref().tx_response.as_ref().unwrap();

                super::util::console_log(tx_resp_obj.raw_log.as_str());
                super::util::console_log(tx_resp_obj.info.as_str());
                super::util::console_log(tx_resp_obj.code.to_string().as_str());
                super::util::console_log(tx_resp_obj.height.to_string().as_str());
                for log in &tx_resp_obj.logs {
                    super::util::console_log(log.log.as_str())
                }

                window.location().reload();
            });
        })
        .forget();
    });
}
