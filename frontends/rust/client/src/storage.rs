use crate::clients::blawgd_client::AccountInfo;
use crate::clients::verification_client::VerificationClient;
use crate::clients::COSMOS_DP;
use anyhow::Result;
use crw_wallet::crypto::MnemonicWallet;
use gloo::storage::errors::StorageError;
use gloo::storage::{LocalStorage, Storage};
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone)]
pub struct Store;

#[derive(Serialize, Deserialize, Clone)]
pub struct ApplicationData {
    pub mnemonic: String,
    pub address: String,
}

const APP_DATA: &str = "app_data";

impl Store {
    pub fn set_application_data(&self, app_data: ApplicationData) -> Result<()> {
        Ok(LocalStorage::set(APP_DATA, app_data)?)
    }
    pub fn get_application_data(&self) -> Result<ApplicationData> {
        let app_data: Result<ApplicationData, StorageError> = LocalStorage::get(APP_DATA);
        Ok(app_data?)
    }
    pub fn delete_application_data(&self) {
        LocalStorage::delete(APP_DATA)
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
