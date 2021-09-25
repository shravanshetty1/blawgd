use crate::clients::light_client::LightClient;
use crate::storage::Store;

pub mod helpers;
pub mod keys;
pub mod proof;
pub mod verification_client;

#[derive(Clone)]
pub struct VerificationClient {
    lc: LightClient,
    client: grpc_web_client::Client,
    verify: bool,
}

impl VerificationClient {
    pub fn new(
        lc: LightClient,
        client: grpc_web_client::Client,
        verify: bool,
    ) -> VerificationClient {
        VerificationClient { lc, client, verify }
    }
}
