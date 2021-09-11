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
            post_header = format!(
                r#"<a href="/profile/{}" class="post-component-header">Reposted by {}</a>"#,
                account_info.address, account_info.name
            )
            .to_string();
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
}
