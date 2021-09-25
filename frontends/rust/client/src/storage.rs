use crate::clients::blawgd_client::AccountInfo;
use crate::clients::verification_client::VerificationClient;
use crate::clients::COSMOS_DP;
use anyhow::anyhow;
use anyhow::Result;
use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::service_client::ServiceClient;
use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::GetNodeInfoRequest;
use crw_wallet::crypto::MnemonicWallet;
use gloo::storage::errors::StorageError;
use gloo::storage::{LocalStorage, Storage};
use serde::Deserialize;
use serde::Serialize;
use tendermint_light_client::types::PeerId;

#[derive(Clone)]
pub struct Store;

// TODO refactor this - holy crap

#[derive(Serialize, Deserialize, Clone)]
pub struct ApplicationData {
    pub mnemonic: String,
    pub address: String,
}

const APP_DATA: &str = "app_data";
const SHOULD_VERIFY: &str = "should_verify";
const PEER_ID: &str = "peer_id";
const LAST_LC_SYNC: &str = "last_lc_sync";

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

    pub fn update_lc_sync(&self) -> Result<()> {
        let unix_ts = chrono::Utc::now().timestamp_millis();
        Ok(LocalStorage::set(LAST_LC_SYNC, unix_ts)?)
    }
    pub fn last_lc_sync(&self) -> Result<i64> {
        let last_lc_sync: Result<i64, StorageError> = LocalStorage::get(LAST_LC_SYNC);
        if last_lc_sync.is_err() {
            return Ok(0);
        }

        Ok(last_lc_sync?)
    }

    pub async fn get_peer_id(&self, grpc: grpc_web_client::Client) -> Result<PeerId> {
        let res: Result<String, StorageError> = LocalStorage::get(PEER_ID);
        let mut peer_id = String::new();
        if res.is_err() {
            peer_id = ServiceClient::new(grpc)
                .get_node_info(GetNodeInfoRequest {})
                .await?
                .get_ref()
                .clone()
                .default_node_info
                .ok_or(anyhow!("could not get node info"))?
                .default_node_id;
            LocalStorage::set(PEER_ID, peer_id.clone())?;
        } else {
            peer_id = res?;
        }

        let peer_id = peer_id.parse::<PeerId>()?;
        Ok(peer_id)
    }

    pub fn get_wallet(&self) -> Result<MnemonicWallet> {
        let app_data = self.get_application_data()?;
        Ok(MnemonicWallet::new(app_data.mnemonic.as_str(), COSMOS_DP)?)
    }

    pub fn should_verify(&self) -> Result<bool> {
        let should_verify: Result<bool, StorageError> = LocalStorage::get(SHOULD_VERIFY);
        if should_verify.is_err() {
            return Ok(true);
        }

        self.set_should_verify(true)?;

        Ok(should_verify?)
    }

    pub fn set_should_verify(&self, state: bool) -> Result<()> {
        Ok(LocalStorage::set(SHOULD_VERIFY, state)?)
    }
}
