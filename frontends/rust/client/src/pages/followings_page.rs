// use crate::clients::verification_client::VerificationClient;
// use crate::host::Host;
// use crate::storage::Store;
// use crate::{
//     components::blawgd_html::BlawgdHTMLDoc, components::followings_page::FollowingsPage,
//     components::nav_bar::NavBar, components::Component, util,
// };
// use anyhow::Result;
//
// pub async fn handle(store: Store, host: Host, cl: VerificationClient) -> Result<()> {
//     let window = web_sys::window().unwrap();
//     let document = window.document().unwrap();
//
//     let url: String = window.location().href().unwrap();
//     let address = url
//         .as_str()
//         .strip_prefix(format!("{}/followings/", host.endpoint()).as_str())
//         .unwrap();
//
//     let account_info = store.get_session_account_info(cl.clone()).await.ok();
//     let followings = cl.get_following_list(address.clone().parse()?).await?;
//
//     let mut followings_account_info: Vec<_> = vec![];
//     for following in followings {
//         followings_account_info.push(cl.get_account_info(following.to_string()).await?)
//     }
//
//     let nav_bar = NavBar::new(account_info.clone());
//     let comp = BlawgdHTMLDoc::new(FollowingsPage::new(nav_bar, followings_account_info));
//
//     let body = document.body().expect("body missing");
//     body.set_inner_html(&comp.to_html());
//
//     Ok(())
// }
