use crate::light_client::LightClient;

pub mod helpers;
pub mod proof;
pub mod verification_client;

// This is a grpc client wrapper which verifies responses using merkle proofs.
// All responses need to be represented in key-pairs such that proofs for keys can be looked up.
// Values are assumed to be stored as protobuf encoded objects and keys are utf8 bytes.
pub struct VerificationClient<'a> {
    lc: &'a mut LightClient,
    client: grpc_web_client::Client,
}

impl<'a> VerificationClient<'a> {
    pub fn new(lc: &'a mut LightClient, client: grpc_web_client::Client) -> VerificationClient {
        VerificationClient { lc, client }
    }
}
