use crate::components::Component;

pub struct NavBar {}

impl NavBar {
    pub fn new() -> Box<NavBar> {
        Box::new(NavBar {})
    }
}

impl super::Component for NavBar {
    fn to_html(&self) -> String {
        String::from(format!(
            r#"
    <div class="nav-bar">
        <a href="/" class="nav-bar-header">
            Blawgd
        </a>
        <div class="nav-bar-menu">
            <a href="/" class="nav-bar-menu-element">Home</a>
            <div class="nav-bar-menu-element">Explore</div>
            <div class="nav-bar-menu-element">Profile</div>
            <div class="nav-bar-menu-element">About</div>
        </div>
        <a href="/login" class="login-link-component-wrapper">
            <img src="profile.jpeg" class="post-component-account-info-image">
            <div class="login-link-component-text">Login/Logout</div>
        </a>
    </div>"#
        ))
    }
}
