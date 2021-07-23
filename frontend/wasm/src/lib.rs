mod components;
use cosmos_sdk_proto::cosmos::bank::v1beta1::MsgSend;
use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
use crw_client::client::CosmosClient;
use crw_client::tx::TxBuilder;
use gloo::events;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;
use web_sys;
// use cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastMode;
use crate::components::blawgd_html::BlawgdHTMLDoc;
use crate::components::home_page::HomePage;
use crate::components::nav_bar::NavBar;
use crate::components::post::Post;
use crate::components::post_creator::PostCreator;
use crate::components::Component;
use cosmos_sdk_proto::cosmos::tx::v1beta1::TxRaw;
use cosmos_sdk_proto::cosmos::tx::v1beta1::{BroadcastMode, Tx};

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let nav_bar = NavBar::new();
    let post_creator = PostCreator::new();
    let post = Post::new();
    let home_page_component =
        BlawgdHTMLDoc::new(HomePage::new(nav_bar, post_creator, Box::new([post])));

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    body.set_inner_html(&home_page_component.to_html());

    let post_element = document
        .get_element_by_id("post")
        .expect("post element not found");
    events::EventListener::new(&post_element,"click",move |_|{
        wasm_bindgen_futures::spawn_local(async move {
            let alice_mnemonic = "violin equal subway say aerobic master clock crumble exile bonus urban account pill sense outer boss blouse master city record fish staff aim comfort";
            let bob_address = "cosmos1epv3u67f6y8gxlwqfm930fjxjdzf2vc4hqqmgg";
            let cosmos_dp = "m/44'/118'/0'/0/0";
            let alice_wallet = crw_wallet::crypto::MnemonicWallet::new(alice_mnemonic,cosmos_dp).expect("could not generate alice wallet");

            let cosmos_client =
                CosmosClient::new("http://localhost:1317", "http://localhost:9091");

            // let alice_address = alice_wallet.get_bech32_address("cosmos").unwrap();
            let alice_address: String = String::from("cosmos1dx6de4h77qrnk8nr9azz0k6w85zncxw306hejx");

            let amount = Coin {
                denom: "token".to_string(),
                amount: "10".to_string(),
            };

            let msg_snd = MsgSend {
                from_address: alice_address.clone(),
                to_address: bob_address.to_string(),
                amount: vec![amount],
            };

            let account_data = cosmos_client.get_account_data(&alice_address).await.unwrap();
            let tx = TxBuilder::new("samachar")
                .memo("Test memo")
                .account_info(account_data.sequence, account_data.account_number)
                .timeout_height(0)
                .fee("token","1",3000000)
                .add_message("/cosmos.bank.v1beta1.MsgSend", msg_snd)
                .unwrap()
                .sign(&alice_wallet)
                .expect("hahha something happened");

            let serialized_tx = serialize_tx(&tx).expect("couldnt serialize tx");
            web_sys::console::log_1(&serialized_tx.to_hex().into());

            let res = cosmos_client
                .broadcast_tx(&tx, BroadcastMode::Block)
                .await.unwrap().unwrap();

            web_sys::console::log_1(&"something".into());
            web_sys::console::log_1(&res.raw_log.as_str().into());
        });
    }).forget();

    Ok(())
}

pub trait ToHex {
    /// Hex representation of the object
    fn to_hex(&self) -> String;
}

impl ToHex for [u8] {
    fn to_hex(&self) -> String {
        use core::fmt::Write;
        let mut ret = String::with_capacity(2 * self.len());
        for ch in self {
            write!(ret, "{:02x}", ch).expect("writing to string");
        }
        ret
    }
}

pub fn serialize_tx(tx: &Tx) -> Result<Vec<u8>, prost::EncodeError> {
    // Some buffers used to serialize the objects
    let mut serialized_body: Vec<u8> = Vec::new();
    let mut serialized_auth: Vec<u8> = Vec::new();
    let mut serialized_tx: Vec<u8> = Vec::new();

    // Serialize the tx body and auth_info
    if let Some(body) = &tx.body {
        prost::Message::encode(body, &mut serialized_body)?;
    }
    if let Some(auth_info) = &tx.auth_info {
        prost::Message::encode(auth_info, &mut serialized_auth)?;
    }

    // Prepare and serialize the TxRaw
    let tx_raw = TxRaw {
        body_bytes: serialized_body,
        auth_info_bytes: serialized_auth,
        signatures: tx.signatures.clone(),
    };
    prost::Message::encode(&tx_raw, &mut serialized_tx)?;

    Ok(serialized_tx)
}
