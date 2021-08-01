use crate::components::account_info::AccountInfoComp;
use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::edit_profile_page::EditProfilePage;
use crate::components::nav_bar::NavBar;
use crate::components::Component;
use crate::{blawgd_client, util};
use gloo::events;
use wasm_bindgen::JsCast;

pub async fn handle() {
    let window = web_sys::window().unwrap();
    let document = window.document().expect("document missing");
    let storage = window
        .local_storage()
        .expect("storage object missing")
        .unwrap();
    let client = grpc_web_client::Client::new(util::GRPC_WEB_ADDRESS.into());

    let account_info = util::get_session_account_info(&storage, client.clone()).await;
    if account_info.is_none() {
        window.location().replace(util::HOST_NAME);
    }

    let account_info_comp = AccountInfoComp::new(account_info.clone().unwrap());
    let nav_bar = NavBar::new(account_info.clone());
    let comp = BlawgdHTMLDoc::new(EditProfilePage::new(nav_bar, account_info_comp));

    let body = document.body().expect("body missing");
    body.set_inner_html(&comp.to_html());

    register_event_listeners(&document)
}

fn register_event_listeners(document: &web_sys::Document) {
    let preview_button = document
        .get_element_by_id("preview-button")
        .expect("preview-button element not found");

    events::EventListener::new(&preview_button, "click", move |_| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let image_field = document
            .get_element_by_id("image-field")
            .expect("image-field element not found");
        let image_link: String = image_field
            .dyn_ref::<web_sys::HtmlInputElement>()
            .unwrap()
            .value();
        let name_field = document
            .get_element_by_id("name-field")
            .expect("name-field element not found");
        let name: String = name_field
            .dyn_ref::<web_sys::HtmlInputElement>()
            .unwrap()
            .value();
        document
            .get_element_by_id("account-info-name")
            .unwrap()
            .set_inner_html(name.as_str());
        document
            .get_element_by_id("account-info-photo")
            .unwrap()
            .dyn_ref::<web_sys::HtmlImageElement>()
            .unwrap()
            .set_src(image_link.as_str())
    })
    .forget();

    let reset_button = document
        .get_element_by_id("reset-button")
        .expect("reset-button element not found");
    events::EventListener::new(&reset_button, "click", move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let storage = window.local_storage().unwrap().unwrap();
            let client = grpc_web_client::Client::new(util::GRPC_WEB_ADDRESS.into());

            let account_info_view = util::get_session_account_info(&storage, client.clone()).await;
            let account_info = account_info_view.unwrap().account_info.unwrap();
            document
                .get_element_by_id("account-info-name")
                .unwrap()
                .set_inner_html(account_info.name.as_str());
            document
                .get_element_by_id("account-info-photo")
                .unwrap()
                .dyn_ref::<web_sys::HtmlImageElement>()
                .unwrap()
                .set_src(account_info.photo.as_str());

            let image_field = document
                .get_element_by_id("image-field")
                .expect("image-field element not found");
            image_field
                .dyn_ref::<web_sys::HtmlInputElement>()
                .unwrap()
                .set_value("");
            let name_field = document
                .get_element_by_id("name-field")
                .expect("name-field element not found");
            name_field
                .dyn_ref::<web_sys::HtmlInputElement>()
                .unwrap()
                .set_value("");
        });
    })
    .forget();

    let update_profile_button = document
        .get_element_by_id("update-profile")
        .expect("update-profile element not found");
    events::EventListener::new(&update_profile_button, "click", move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let window = web_sys::window().unwrap();
            let document = window.document().expect("document missing");
            let storage = window.local_storage().unwrap().unwrap();

            let image_field = document
                .get_element_by_id("image-field")
                .expect("image-field element not found");
            let photo: String = image_field
                .dyn_ref::<web_sys::HtmlInputElement>()
                .unwrap()
                .value();
            let name_field = document
                .get_element_by_id("name-field")
                .expect("name-field element not found");
            let name: String = name_field
                .dyn_ref::<web_sys::HtmlInputElement>()
                .unwrap()
                .value();
            let msg = blawgd_client::MsgUpdateAccountInfo {
                creator: util::get_stored_data(&storage).unwrap().address,
                name,
                photo,
                metadata: "".to_string(),
            };

            let wallet = util::get_wallet(&storage).unwrap();
            let client = grpc_web_client::Client::new(util::GRPC_WEB_ADDRESS.into());
            util::broadcast_tx(
                &wallet,
                client.clone(),
                util::MSG_TYPE_UPDATE_ACCOUNT_INFO,
                msg,
            )
            .await;
            window.location().reload();
        });
    })
    .forget();
}
