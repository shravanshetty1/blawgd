pub mod blawgd_html;
pub mod home_page;

pub trait Component {
    fn to_string(&self) -> String;
}
