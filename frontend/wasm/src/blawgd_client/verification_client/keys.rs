pub fn user_post_key(address: String, order: String) -> String {
    format!("user-post-{}-{}", address, order)
}

pub fn subpost_key(parent_post: String, order: String) -> String {
    format!("sub-post-{}-{}", parent_post, order)
}

pub fn post_count_key() -> String {
    String::from("post-count")
}

pub fn post_key(order: String) -> String {
    format!("post-{}", order)
}

pub fn account_info_key(address: String) -> String {
    format!("account-info-{}", address)
}

pub fn following_key(address: String) -> String {
    format!("following-{}", address)
}
