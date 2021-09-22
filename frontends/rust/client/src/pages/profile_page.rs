// use crate::{
//     components::account_info::AccountInfoComp,
//     components::blawgd_html::BlawgdHTMLDoc,
//     components::nav_bar::NavBar,
//     components::post::PostComponent,
//     components::profile_page::{ButtonType, ProfilePage},
//     components::Component,
//     util,
// };
//
// use crate::blawgd_client::MsgStopFollow;
// use crate::clients::verification_client::VerificationClient;
// use crate::host::Host;
// use crate::state::{get_state, set_state};
// use crate::storage::Store;
// use anyhow::Context;
// use anyhow::Result;
// use cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastMode;
// use gloo::events;
//
// pub async fn handle(store: Store, host: Host, cl: VerificationClient) -> Result<()> {
//     let window = web_sys::window().unwrap();
//     let document = window.document().unwrap();
//
//     let url: String = window.location().href().unwrap();
//     let address = url
//         .as_str()
//         .strip_prefix(format!("{}/profile/", host.endpoint()).as_str())
//         .unwrap()
//         .to_string();
//
//     let logged_in_account_info = store.get_session_account_info(cl.clone()).await.ok();
//     let account_info = cl
//         .get_account_info(address.clone())
//         .await
//         .context("failed to get valid profile info response")?;
//     let posts = cl.get_post_by_account(address.clone(), 1 as u64).await?;
//     let logged_in_data = store.get_application_data().ok();
//     let mut profile_button: Option<ButtonType> = None;
//     if logged_in_data.is_some() {
//         if address == logged_in_data.as_ref().unwrap().address {
//             profile_button = Some(ButtonType::Edit);
//         } else {
//             let is_following = util::is_following(
//                 cl.clone(),
//                 logged_in_data.as_ref().unwrap().address.clone(),
//                 address.clone(),
//             )
//             .await?;
//             if is_following {
//                 profile_button = Some(ButtonType::Unfollow);
//             } else {
//                 profile_button = Some(ButtonType::Follow)
//             }
//         }
//     }
//
//     let mut posts_comps: Vec<Box<dyn Component>> = Vec::new();
//     for post in posts {
//         let post_comp = PostComponent::new(post.clone());
//         posts_comps.push(post_comp)
//     }
//
//     let nav_bar = NavBar::new(logged_in_account_info.clone());
//     let profile_page = ProfilePage::new(
//         nav_bar,
//         AccountInfoComp::new(account_info.clone()),
//         profile_button.clone(),
//         posts_comps.into_boxed_slice(),
//     );
//
//     let comp = BlawgdHTMLDoc::new(profile_page);
//
//     let body = document.body().expect("body missing");
//     body.set_inner_html(&comp.to_html());
//
//     if profile_button.is_some() {
//         if !matches!(profile_button.unwrap(), ButtonType::Edit) {
//             register_event_listeners(store, host, &document, address.clone(), cl.clone());
//         }
//     }
//
//     let window = web_sys::window().unwrap();
//     events::EventListener::new(&window, "scroll", move |_| {
//         let cl = cl.clone();
//         let address = address.clone();
//         wasm_bindgen_futures::spawn_local(async move {
//             let window = web_sys::window().unwrap();
//             let document = window.document().expect("document missing");
//             let doc = document.document_element().unwrap();
//             let scroll_top: i32 = doc.scroll_top();
//             let scroll_height: i32 = doc.scroll_height();
//             let client_height: i32 = doc.client_height();
//             let main_column = document
//                 .clone()
//                 .get_element_by_id("main-column")
//                 .expect("post-creator-button element not found");
//
//             if scroll_top + client_height >= scroll_height {
//                 let mut state = get_state();
//                 state.page += 1;
//
//                 let posts = cl
//                     .get_post_by_account(address.clone(), state.page.clone() as u64)
//                     .await
//                     .unwrap();
//                 if posts.len() == 0 {
//                     return;
//                 }
//                 let mut posts_html: String = String::new();
//                 for post in posts {
//                     posts_html = format!("{}{}", posts_html, PostComponent::new(post).to_html());
//                 }
//
//                 main_column.insert_adjacent_html("beforeend", posts_html.as_str());
//
//                 set_state(state.clone());
//                 crate::logger::console_log(format!("{}", state.page).as_str());
//             }
//         });
//     })
//     .forget();
//     Ok(())
// }
//
// fn register_event_listeners(
//     store: Store,
//     host: Host,
//     document: &web_sys::Document,
//     address: String,
//     cl: VerificationClient,
// ) {
//     let follow_toggle = document.get_element_by_id("follow-toggle").unwrap();
//
//     let address1 = address.clone();
//     let cl1 = cl.clone();
//     events::EventListener::new(&follow_toggle, "click", move |_| {
//         let address = address1.clone();
//         let cl = cl1.clone();
//         let store = store.clone();
//         let host = host.clone();
//         wasm_bindgen_futures::spawn_local(async move {
//             let window = web_sys::window().unwrap();
//             let client = grpc_web_client::Client::new(host.grpc_endpoint());
//
//             let session_address = store.get_application_data().unwrap().address;
//             if util::is_following(cl.clone(), session_address.clone(), address.clone())
//                 .await
//                 .unwrap()
//             {
//                 let msg = Box::new(MsgStopFollow {
//                     creator: session_address.clone(),
//                     address: address.clone(),
//                 });
//                 let msg_type = util::MSG_TYPE_STOP_FOLLOW;
//                 util::broadcast_tx(
//                     &store.get_wallet().unwrap(),
//                     client.clone(),
//                     msg_type,
//                     msg,
//                     BroadcastMode::Block as i32,
//                 )
//                 .await;
//             } else {
//                 let msg = Box::new(crate::blawgd_client::MsgFollow {
//                     creator: session_address.clone(),
//                     address: address.clone(),
//                 });
//                 let msg_type = util::MSG_TYPE_FOLLOW;
//                 util::broadcast_tx(
//                     &store.get_wallet().unwrap(),
//                     client.clone(),
//                     msg_type,
//                     msg,
//                     BroadcastMode::Block as i32,
//                 )
//                 .await;
//             }
//
//             window.location().reload();
//         });
//     })
//     .forget();
// }
//
// pub async fn is_following(
//     cl: VerificationClient,
//     address1: String,
//     address2: String,
// ) -> Result<bool> {
//     let followings = cl.get_following_list(address1).await?;
//
//     let mut is_following: bool = false;
//     for following in followings {
//         if following.to_string() == address2 {
//             is_following = true;
//         }
//     }
//
//     Ok(is_following)
// }
