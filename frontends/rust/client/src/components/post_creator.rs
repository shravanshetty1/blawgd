use crate::clients::blawgd_client::MsgCreatePost;
use crate::clients::MSG_TYPE_CREATE_POST;
use crate::context::ApplicationContext;
use crate::task;
use anyhow::Result;
use cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastMode;
use gloo::events;
use std::sync::Arc;
use wasm_bindgen::JsCast;

pub struct PostCreator {
    button_text: String,
    parent_post: String,
}

impl PostCreator {
    pub fn new(parent_post: String) -> Box<PostCreator> {
        Box::new(PostCreator {
            parent_post,
            button_text: "Post".into(),
        })
    }

    pub fn set_button_text(&mut self, text: &str) {
        self.button_text = text.into();
    }
}

impl super::Component for PostCreator {
    fn to_html(&self) -> String {
        String::from(format!(
            r#"
        <div class="post-creator">
            <textarea id="post-creator-input" class="post-creator-input"></textarea>
            <div class="post-creator-buttons">
                <div id="post-creator-button" class="post-creator-button-post">
                    {}
                </div>
            </div>
        </div>"#,
            self.button_text
        ))
    }

    fn register_events(&self, ctx: Arc<ApplicationContext>) -> Result<()> {
        let document = ctx.window.document()?;
        let post_creator_button = document.get_element_by_id("post-creator-button")?.inner();
        let parent_post = self.parent_post.clone();
        events::EventListener::new(&post_creator_button, "click", move |_| {
            let document = document.clone();
            let ctx = ctx.clone();
            let parent_post = parent_post.clone();
            task::spawn_local(async move {
                let post_content: String = document
                    .get_element_by_id("post-creator-input")?
                    .inner()
                    .dyn_ref::<web_sys::HtmlTextAreaElement>()
                    .unwrap()
                    .value();
                let msg = MsgCreatePost {
                    creator: ctx.store.get_application_data()?.address,
                    content: post_content,
                    parent_post,
                };

                let wallet = ctx.store.get_wallet()?;
                let resp = ctx
                    .client
                    .broadcast_tx(
                        &wallet,
                        MSG_TYPE_CREATE_POST,
                        msg,
                        BroadcastMode::Block as i32,
                    )
                    .await?
                    .into_inner();
                crate::logger::console_log(resp.tx_response.unwrap().raw_log.as_str());
                ctx.window.location().inner().reload();
                Ok(())
            });
        })
        .forget();

        Ok(())
    }
}
