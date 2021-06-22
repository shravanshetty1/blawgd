use wasm_bindgen::prelude::*;
use web_sys;
use gloo::events;
use crw_client::client::CosmosClient;
use crw_client::tx::TxBuilder;
use wasm_bindgen_futures;
use cosmos_sdk_proto::cosmos::bank::v1beta1::MsgSend;
use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
use cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastMode;


// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    let post_element = document.get_element_by_id("post").expect("post element not found");





    events::EventListener::new(&post_element,"click",move |_|{
        web_sys::console::log_1(&"click1".into());

        wasm_bindgen_futures::spawn_local(async move {
            let alice_mnemonic = "butter waste evoke dismiss crystal power debate general warrior put reflect seven light rabbit iron harvest flush praise cruel dwarf denial near lava infant";
            let bob_address = "cosmos1w2hsnqnvxkgkjvzumruccutxlj8yvxxygn40uj";
            let cosmos_dp = "m/44'/118'/0'/0/0";
            let alice_wallet = crw_wallet::crypto::MnemonicWallet::new(alice_mnemonic,cosmos_dp).expect("could not generate alice wallet");


            let cosmos_client =
                CosmosClient::new("http://localhost:1317", "http://localhost:9090").expect("could not create cosmos client");

            let address = alice_wallet.get_bech32_address("cosmos").unwrap();

            let amount = Coin {
                denom: "token".to_string(),
                amount: "10".to_string(),
            };

            let msg_snd = MsgSend {
                from_address: address.clone(),
                to_address: bob_address.to_string(),
                amount: vec![amount],
            };

            let account_data = cosmos_client.get_account_data(&address).await.unwrap();
            let tx = TxBuilder::new("samachar")
                .memo("Test memo")
                .account_info(account_data.sequence, account_data.account_number)
                .timeout_height(0)
                .add_message("/cosmos.bank.v1beta1.Msg/Send", msg_snd)
                .unwrap()
                .sign(&alice_wallet)
                .unwrap();

            let tx_memo = tx.clone().body.unwrap().memo;
            web_sys::console::log_1(&tx_memo.as_str().into());

            let res = cosmos_client
                .broadcast_tx(&tx, BroadcastMode::Block)
                .await.unwrap().unwrap();

            web_sys::console::log_1(&res.raw_log.as_str().into());
        });



    }).forget();

    Ok(())
}