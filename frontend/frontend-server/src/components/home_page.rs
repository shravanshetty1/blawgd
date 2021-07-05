pub struct HomePage {}

impl HomePage {
    pub fn new() -> Box<HomePage> {
        Box::new(HomePage {})
    }
}

impl super::Component for HomePage {
    fn to_string(&self) -> String {
        String::from(format!(
            r#"
<div class="post-creator">
    <textarea id="post-input" class="post-creator-input"></textarea>
    <div class="post-creator-buttons">
        <div id="post" class="post-creator-button-post">
                Post
        </div>
    </div>
</div>
"#
        ))
    }
}
