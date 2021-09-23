use crate::clients::blawgd_client::{MsgLikePost, MsgRepost, PostView};
use crate::clients::{MSG_TYPE_LIKE, MSG_TYPE_REPOST};
use crate::context::ApplicationContext;
use crate::task;
use anyhow::anyhow;
use anyhow::Result;
use cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastMode;
use events::EventListener;
use gloo::events;
use std::borrow::Cow;
use std::future::Future;
use std::sync::Arc;
use task::spawn_local;

pub struct PostComponent {
    post: PostView,
    focus: bool,
}

impl PostComponent {
    pub fn new(post: PostView) -> Box<PostComponent> {
        Box::new(PostComponent { post, focus: false })
    }
    pub fn focus(&mut self) {
        self.focus = true;
    }

    fn like_event(&self, ctx: Arc<ApplicationContext>) -> Result<()> {
        let post = self.post.clone();
        let document = ctx.window.document()?;
        let like_button_wrapper_id = format!("post-{}-like", post.id);
        let like_button_wrapper = document.get_element_by_id(like_button_wrapper_id.as_str())?;
        EventListener::new(&like_button_wrapper.inner(), "click", move |_| {
            let post = post.clone();
            let ctx = ctx.clone();
            let document = document.clone();

            // TODO spawn local is slow, move like out of it

            spawn_local(async move {
                let like_button_id = format!("post-{}-like-content", post.id);
                let like_button = document.get_element_by_id(like_button_id.as_str())?;
                let like_button_text = like_button.inner_html();
                let likes_count_text = like_button_text.strip_suffix(" Likes").unwrap_or("0");
                let mut likes_count = likes_count_text.parse::<i32>()?;
                likes_count += 1;
                like_button.set_inner_html(format!("{} Likes", likes_count).as_str());

                let session = ctx
                    .session
                    .as_ref()
                    .ok_or(anyhow!("could not get session account info"))?;
                let resp = ctx
                    .client
                    .broadcast_tx(
                        &ctx.store.get_wallet()?,
                        MSG_TYPE_LIKE,
                        MsgLikePost {
                            creator: session.address.clone(),
                            post_id: post.id,
                            amount: 1,
                        },
                        BroadcastMode::Sync as i32,
                    )
                    .await?;

                crate::logger::console_log(resp.into_inner().tx_response.unwrap().raw_log.as_str());
                Ok(())
            });
        })
        .forget();
        Ok(())
    }

    fn repost_event(&self, ctx: Arc<ApplicationContext>) -> Result<()> {
        let post = self.post.clone();
        let document = ctx.window.document()?;
        let repost_button_wrapper_id = format!("post-{}-repost", post.id);
        let repost_button_wrapper =
            document.get_element_by_id(repost_button_wrapper_id.as_str())?;
        EventListener::new(&repost_button_wrapper.inner(), "click", move |_| {
            let post = post.clone();
            let ctx = ctx.clone();
            let document = document.clone();
            spawn_local(async move {
                let repost_button_id = format!("post-{}-repost-content", post.id);
                let repost_button = document.get_element_by_id(repost_button_id.as_str())?;
                let repost_button_text: String = repost_button.inner_html();
                let repost_count_text = repost_button_text.strip_suffix(" Reposts").unwrap_or("0");
                let mut repost_count = repost_count_text.parse::<i32>().unwrap_or(0);
                repost_count += 1;
                repost_button.set_inner_html(format!("{} Reposts", repost_count).as_str());

                let resp = ctx
                    .client
                    .broadcast_tx(
                        &ctx.store.get_wallet()?,
                        MSG_TYPE_REPOST,
                        MsgRepost {
                            creator: ctx
                                .session
                                .as_ref()
                                .ok_or(anyhow!("could not get session account info"))?
                                .address
                                .clone(),
                            post_id: post.id,
                        },
                        BroadcastMode::Sync as i32,
                    )
                    .await?;

                crate::logger::console_log(resp.into_inner().tx_response.unwrap().raw_log.as_str());
                Ok(())
            });
        })
        .forget();
        Ok(())
    }
}

impl super::Component for PostComponent {
    fn to_html(&self) -> String {
        let mut account_info = self.post.creator.clone().unwrap();

        let mut post_text_class = "post-component-text";
        if self.focus {
            post_text_class = "post-component-text-focus";
        }

        let parent_post = self.post.parent_post.clone();
        let mut post_header: String = String::new();
        if !parent_post.is_empty() {
            post_header = format!(
                r#"<a href="/post/{}" class="post-component-header">Replying to post {}</a>"#,
                parent_post, parent_post
            )
            .to_string();
        }

        let mut post = self.post.clone();
        if post.repost_parent.is_some() {
            let old_post_header = post_header.clone();
            post_header = format!(
                r#"<a href="/profile/{}" class="post-component-header">Reposted by {}</a>"#,
                account_info.address, account_info.name
            )
            .to_string();
            post_header.push_str(old_post_header.as_str());
            let repost = post.repost_parent.unwrap().as_ref().clone();
            post.content = repost.content.clone();
            account_info = repost.creator.clone().unwrap();
        }

        String::from(format!(
            r#"
        <div class="post-component">
            {}
            <div class="post-component-text-wrapper">
                <a href="/profile/{}"><img src="{}" class="post-component-account-info-image"></a>
                <div class="post-component-text-content">
                    <div class="post-component-account-info">
                        <a href="/profile/{}" class="post-component-account-info-name">{}</a>
                        <div class="post-component-account-info-address">@{}</div>
                    </div>
                    <div class="{}">
                        {}
                    </div>
                </div>
            </div>
            <div class="post-component-bar">
                <div id="post-{}-like" class="post-component-bar-button"><div id="post-{}-like-content" class="post-component-bar-button-content">{} Likes</div></div>
                <div id="post-{}-repost" class="post-component-bar-button"><div id="post-{}-repost-content" class="post-component-bar-button-content">{} Reposts</div></div>
                <a href="/post/{}" class="post-component-bar-button"><div class="post-component-bar-button-content">{} Comments</div></a>
            </div>
        </div>"#,
            post_header,
            account_info.address,
            account_info.photo,
            account_info.address,
            account_info.name,
            account_info.address,
            post_text_class,
            post.content,
            post.id,
            post.id,
            post.like_count,
            post.id,
            post.id,
            post.repost_count,
            post.id,
            post.comments_count
        ))
    }

    fn register_events(&self, ctx: Arc<ApplicationContext>) -> Result<()> {
        self.like_event(ctx.clone())?;
        self.repost_event(ctx)?;
        Ok(())
    }
}
