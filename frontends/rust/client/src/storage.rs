use anyhow::anyhow;
use anyhow::Result;

pub struct LocalStorage {
    storage: web_sys::Storage,
}

impl LocalStorage {
    pub fn new(window: web_sys::Window) -> Result<LocalStorage> {
        Ok(LocalStorage {
            storage: window.local_storage().unwrap().ok_or(anyhow!(
                "could not get local storage of browser object from window"
            ))?,
        })
    }

    pub fn get(&self, key: String) -> Result<String> {
        let val: String = self.storage.get_item(key.as_str()).unwrap().ok_or(anyhow!(
            "could not get value for key {} from browser storage",
            key
        ))?;
        Ok(val)
    }

    pub fn set(&self, key: String, value: String) {
        self.storage.set_item(key.as_str(), value.as_str());
    }

    pub fn delete(&self, key: String) {
        self.storage.remove_item(key.as_str());
    }
}
