use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys;
mod blawgd_client;
mod components;
mod home_page;
mod login_page;
mod util;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();

    let url: String = window.location().href().unwrap();
    let url_path = url.as_str().strip_prefix("http://localhost:2341/").unwrap();

    match url_path {
        "login" => login_page::handle(&window),
        _ => home_page::handle(&window),
    };

    Ok(())
}

// fn forReference() {
// let post_element = document
//     .get_element_by_id("post")
//     .expect("post element not found");
// events::EventListener::new(&post_element,"click",move |_|{
//     wasm_bindgen_futures::spawn_local(async move {
//         let alice_mnemonic = "violin equal subway say aerobic master clock crumble exile bonus urban account pill sense outer boss blouse master city record fish staff aim comfort";
//         let bob_address = "cosmos1epv3u67f6y8gxlwqfm930fjxjdzf2vc4hqqmgg";
//         let cosmos_dp = "m/44'/118'/0'/0/0";
//         let alice_wallet = crw_wallet::crypto::MnemonicWallet::new(alice_mnemonic,cosmos_dp).expect("could not generate alice wallet");
//
//         let cosmos_client =
//             CosmosClient::new("http://localhost:1317", "http://localhost:9091");
//
//         // let alice_address = alice_wallet.get_bech32_address("cosmos").unwrap();
//         let alice_address: String = String::from("cosmos1dx6de4h77qrnk8nr9azz0k6w85zncxw306hejx");
//
//         let amount = Coin {
//             denom: "token".to_string(),
//             amount: "10".to_string(),
//         };
//
//         let msg_snd = MsgSend {
//             from_address: alice_address.clone(),
//             to_address: bob_address.to_string(),
//             amount: vec![amount],
//         };
//
//         let account_data = cosmos_client.get_account_data(&alice_address).await.unwrap();
//         let tx = TxBuilder::new("samachar")
//             .memo("Test memo")
//             .account_info(account_data.sequence, account_data.account_number)
//             .timeout_height(0)
//             .fee("token","1",3000000)
//             .add_message("/cosmos.bank.v1beta1.MsgSend", msg_snd)
//             .unwrap()
//             .sign(&alice_wallet)
//             .expect("hahha something happened");
//
//         let serialized_tx = serialize_tx(&tx).expect("couldnt serialize tx");
//         web_sys::console::log_1(&serialized_tx.to_hex().into());
//
//         let res = cosmos_client
//             .broadcast_tx(&tx, BroadcastMode::Block)
//             .await.unwrap().unwrap();
//
//         web_sys::console::log_1(&"something".into());
//         web_sys::console::log_1(&res.raw_log.as_str().into());
//     });
// }).forget();
// }
