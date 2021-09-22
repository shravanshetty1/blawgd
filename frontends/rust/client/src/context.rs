use crate::blawgd_client::AccountInfo;
use crate::clients::MasterClient;
use crate::dom::Window;
use crate::host::Host;
use crate::storage::Store;

pub struct ApplicationContext {
    pub client: MasterClient,
    pub host: Host,
    pub store: Store,
    pub window: Window,
    pub session: Option<AccountInfo>,
}
