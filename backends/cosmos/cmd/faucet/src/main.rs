// #![deny(warnings)]

use crate::cosmos_client::CosmosClient;
use crate::handler::{site_key, State};
use actix_web::App;
use actix_web::{web, HttpServer};
use anyhow::anyhow;
use anyhow::Result;
use crw_wallet::crypto::MnemonicWallet;
use handler::faucet;
use std::ops::Index;
use std::thread;
use std::time::Duration;
use tonic::codegen::http::Uri;
use tonic::transport::Channel;

mod cosmos_client;
mod handler;

pub const COSMOS_DP: &str = "m/44'/118'/0'/0/0";
pub const PORT: u16 = 2342;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    main_handler().await.unwrap();
    Ok(())
}

async fn main_handler() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        return Err(anyhow!("unexpected length of args {}", args.len()));
    }

    let mnemonic = args.index(1).clone();
    let broadcast_node_addr = args.index(2).clone();
    let captcha_site_key = args.index(3).clone();
    let captcha_secret = args.index(4).clone();

    let wallet = MnemonicWallet::new(mnemonic.as_str(), COSMOS_DP)?;
    let mut client = Channel::builder(broadcast_node_addr.parse::<Uri>()?)
        .connect()
        .await;
    while client.is_err() {
        println!("attempting to connect to {}", broadcast_node_addr);
        thread::sleep(Duration::from_secs(2));
        client = Channel::builder(broadcast_node_addr.parse::<Uri>()?)
            .connect()
            .await;
    }
    println!("connected to {}", broadcast_node_addr);
    let client = client?;

    let state = State {
        wallet,
        client: CosmosClient { client },
        captcha_site_key,
        captcha_secret,
    };

    println!("started faucet at {}", PORT);
    HttpServer::new(move || {
        let state = web::Data::new(state.clone());
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();
        App::new()
            .wrap(cors)
            .app_data(state)
            .route("/", web::post().to(faucet))
            .route("/sitekey", web::get().to(site_key))
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await?;

    Ok(())
}
