use crate::blawgd_client::PostView;
use crate::util;

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
}

impl super::Component for PostComponent {
    fn to_html(&self) -> String {
        let account_info = self.post.creator.clone().unwrap();

        let mut post_text_class = "post-component-text";
        if self.focus {
            post_text_class = "post-component-text-focus";
        }

        let parent_post = self.post.parent_post.clone();
        let mut post_header: String = String::new();
        if !parent_post.is_empty() {
            post_header = format!(
                r#"<div class="post-component-header">Replying to post {}</div>"#,
                parent_post
            )
            .to_string();
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
                <div class="post-component-bar-button"><div class="post-component-bar-button-content">Like</div></div>
                <div class="post-component-bar-button"><div class="post-component-bar-button-content">Retweet</div></div>
                <a href="/post/{}" class="post-component-bar-button"><div class="post-component-bar-button-content">Comment</div></a>
            </div>
        </div>"#,
            post_header,
            account_info.address,
            account_info.photo,
            account_info.address,
            account_info.name,
            account_info.address,
            post_text_class,
            self.post.content,
            self.post.id
        ))
    }
}
