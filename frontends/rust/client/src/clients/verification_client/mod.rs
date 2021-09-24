use crate::clients::light_client::LightClient;
use crate::storage::Store;

pub mod helpers;
pub mod keys;
pub mod proof;
pub mod verification_client;

// This is a grpc client wrapper which verifies responses using merkle proofs.
// All responses need to be represented in key-value pairs such that proofs for values can be looked up by key.
// Values are assumed to be stored as protobuf encoded objects and keys are utf8 bytes.
// All responses goes through 3 steps -
// 1. Validation - checks if data being sent is valid using merkle proofs
// 2. Verification - checks if data being sent is the data you asked for
// 3. Normalization - Add default values for data that is missing

#[derive(Clone)]
pub struct VerificationClient {
    lc: LightClient,
    client: grpc_web_client::Client,
    store: Store,
}

impl VerificationClient {
    pub fn new(lc: LightClient, client: grpc_web_client::Client) -> VerificationClient {
        VerificationClient {
            lc,
            client,
            store: Store,
        }
    }
}
