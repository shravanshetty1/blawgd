pub mod account_info;
pub mod blawgd_html;
pub mod edit_profile_page;
pub mod explore_page;
pub mod home_page;
pub mod login_page;
pub mod nav_bar;
pub mod post;
pub mod post_creator;
pub mod profile_page;

pub trait Component {
    fn to_html(&self) -> String;
}
