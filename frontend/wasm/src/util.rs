use crate::blawgd_client::AccountInfo;

pub fn get_account_info_from_storage(storage: &web_sys::Storage) -> Option<AccountInfo> {
    let account_info_raw: Option<String> = storage.get_item("account_info").unwrap();
    let mut account_info: Option<AccountInfo> = None;
    if account_info_raw.is_some() {
        let account_info_string = account_info_raw.unwrap();
        if !account_info_string.is_empty() {
            account_info = Some(prost::Message::decode(account_info_string.as_bytes()).unwrap());
        }
    }
    account_info
}
