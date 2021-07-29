pub struct PostCreator {}

impl PostCreator {
    pub fn new() -> Box<PostCreator> {
        Box::new(PostCreator {})
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
                    Post
                </div>
            </div>
        </div>"#
        ))
    }
}
