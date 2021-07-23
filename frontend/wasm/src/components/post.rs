use crate::components::Component;

pub struct Post {}

impl Post {
    pub fn new() -> Box<Post> {
        Box::new(Post {})
    }
}

impl super::Component for Post {
    fn to_html(&self) -> String {
        String::from(format!(
            r#"
        <div class="post-component">
            <div class="post-component-text-wrapper">
                <img src="profile.jpeg" class="post-component-account-info-image">
                <div class="post-component-text-content">
                    <div class="post-component-account-info">
                        <div class="post-component-account-info-name">Bob Sag</div>
                        <div class="post-component-account-info-address">@sag</div>
                    </div>
                    <div class="post-component-text">
                        Some tweet lololol. Some tweet ahahha.
                    </div>
                </div>
            </div>
            <div class="post-component-bar">
                <div class="post-component-bar-button"><div class="post-component-bar-button-content">Like</div></div>
                <div class="post-component-bar-button"><div class="post-component-bar-button-content">Retweet</div></div>
                <div class="post-component-bar-button"><div class="post-component-bar-button-content">Comment</div></div>
            </div>
        </div>"#
        ))
    }
}
