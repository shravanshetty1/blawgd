use crate::context::ApplicationContext;
use anyhow::Result;
use std::sync::Arc;

pub struct PostCreator {
    button_text: String,
}

impl PostCreator {
    pub fn new() -> Box<PostCreator> {
        Box::new(PostCreator {
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
        Ok(())
    }
}
