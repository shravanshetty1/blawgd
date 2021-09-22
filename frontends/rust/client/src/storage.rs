use crate::blawgd_client::AccountInfo;
use crate::clients::verification_client::VerificationClient;
use anyhow::Result;
use crw_wallet::crypto::MnemonicWallet;
use gloo::storage::errors::StorageError;
use gloo::storage::{LocalStorage, Storage};
use serde::Deserialize;
use serde::Serialize;

pub const COSMOS_DP: &str = "m/44'/118'/0'/0/0";

#[derive(Clone)]
pub struct Store;

#[derive(Serialize, Deserialize, Clone)]
pub struct ApplicationData {
    pub mnemonic: String,
    pub address: String,
}

// TODO inject storage
impl Store {
    pub fn set_application_data(&self, app_data: ApplicationData) -> Result<()> {
        Ok(LocalStorage::set("app_data", app_data)?)
    }
    pub fn get_application_data(&self) -> Result<ApplicationData> {
        let app_data: Result<ApplicationData, StorageError> = LocalStorage::get("app_data");
        Ok(app_data?)
    }
    pub fn delete_application_data(&self) {
        LocalStorage::delete("app_data")
    }

    pub async fn get_session_account_info(&self, cl: VerificationClient) -> Result<AccountInfo> {
        let address = self.get_application_data()?.address;
        cl.get_account_info(address).await
    }

    pub fn get_wallet(&self) -> Result<MnemonicWallet> {
        let app_data = self.get_application_data()?;
        Ok(MnemonicWallet::new(app_data.mnemonic.as_str(), COSMOS_DP)?)
    }
}
