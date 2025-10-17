use copy_trading_bot::common::utils::{
    create_nonblocking_rpc_client, create_rpc_client, import_env_var, import_wallet, AppState,
};
use copy_trading_bot::dex::raydium::{get_pool_state, get_pool_state_by_mint};
use copy_trading_bot::engine::swap::raydium_swap;
use copy_trading_bot::ray_parse::tx_parse::{self, tx_parse};
use dotenv::dotenv;
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use solana_client::rpc_client::{self, RpcClient};
use solana_client::rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter};
use solana_sdk::commitment_config::{self, CommitmentConfig};
use solana_sdk::pubkey::Pubkey;
use std::env;
use std::str::FromStr;
use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage};
#[tokio::main]

async fn main() {
    dotenv().ok();

    let sol_address = env::var("SOL_PUBKEY").expect("SOL_PUBKEY not set");
    let rpc_https_url = env::var("RPC_ENDPOINT").expect("RPC_ENDPOINT not set");
    let rpc_client = RpcClient::new(rpc_https_url.clone());
    let unwanted_key = env::var("JUP_PUBKEY").expect("JUP_PUBKEY not set");
    let target = env::var("TARGET_PUBKEY").expect("TARGET_PUBKEY not set");

    let ws_url = env::var("RPC_WEBSOCKET_ENDPOINT").expect("WS_URL not set");
    let (ws_stream, _) = connect_async(ws_url)
        .await
        .expect("Failed to connect to WebSocket server");
    let (mut write, mut read) = ws_stream.split();
    // Subscribe to logs
    let subscription_message = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "transactionSubscribe",
        "params": [

            {
                "failed": false,
                "accountInclude": ["675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8", target],
                "accountExclude": [unwanted_key],
                // Optionally specify accounts of interest
            },
            {
                "commitment": "processed",
                "encoding": "jsonParsed",
                "transactionDetails": "full",
                "maxSupportedTransactionVersion": 0
            }
        ]
    });

    write
        .send(subscription_message.to_string().into())
        .await
        .expect("Failed to send subscription message");

    // Listen for messages
    while let Some(Ok(msg)) = read.next().await {
        if let WsMessage::Text(text) = msg {
            let json: Value = serde_json::from_str(&text).unwrap();

            // println!("json: {:#?}", json);

            let sig = json["params"]["result"]["signature"].to_string();
            // let mut ixs: Vec<_> = Vec::new();
            if let Some(inner_instructions) =
                json["params"]["result"]["transaction"]["meta"]["innerInstructions"].as_array()
            {
                // println!("log_str: {:#?}", inner_instructions.clone());
                // Iterate over logs and check for unwanted_key
                for inner_instruction in inner_instructions.iter() {
                    // Try to extract the string representation of the log

                    if let Some(instructions) = inner_instruction["instructions"].as_array() {
                        for instruction in instructions.iter() {
                            if instruction["parsed"]["type"] == "transfer".to_string()
                                && instruction["parsed"]["info"]["authority"] == target
                            {
                                let amount_in = instructions[0]["parsed"]["info"]["amount"]
                                    .as_str()
                                    .unwrap_or("0.0")
                                    .to_string();
                                let amount_out = instructions[1]["parsed"]["info"]["amount"]
                                    .as_str()
                                    .unwrap_or("0.0")
                                    .to_string();
                                let in_ata = instructions[0]["parsed"]["info"]["destination"]
                                    .as_str()
                                    .unwrap_or("")
                                    .to_string();

                                let out_ata = instructions[1]["parsed"]["info"]["source"]
                                    .as_str()
                                    .unwrap_or("")
                                    .to_string();
                                let pubkey_in_ata = match Pubkey::from_str(&in_ata) {
                                    Ok(pubkey) => pubkey,
                                    Err(e) => {
                                        println!("Failed to parse Pubkey in: {}", e);
                                        return;
                                    }
                                };
                                let pubkey_out_ata = match Pubkey::from_str(&out_ata) {
                                    Ok(pubkey) => pubkey,
                                    Err(e) => {
                                        println!("Failed to parse Pubkey out: {}", e);
                                        return;
                                    }
                                };

                                let in_data = match rpc_client.get_token_account(&pubkey_in_ata) {
                                    Ok(data) => data,
                                    Err(e) => {
                                        println!("Failed to parse Pubkey in_token: {}", e);
                                        return;
                                    }
                                };
                                let out_data = match rpc_client.get_token_account(&pubkey_out_ata) {
                                    Ok(data) => data,
                                    Err(e) => {
                                        println!("Failed to parse Pubkey out_token: {}", e);
                                        return;
                                    }
                                };
                                let in_mint = match in_data {
                                    Some(mint) => mint,
                                    None => return,
                                };
                                let out_mint = match out_data {
                                    Some(mint) => mint,
                                    None => return,
                                };
                                println!("in_mint: {:#?}", in_mint.mint);
                                println!("in_amount: {:#?}", amount_in);
                                println!("out_mint: {:#?}", out_mint.mint);
                                println!("out_amount: {:#?}", amount_out);
                                println!("signature: {:#?}", json["params"]["result"]["signature"]);
                                let mut param_mint = String::new();
                                let mut param_dirs = String::new();
                                let in_deciaml = in_mint.token_amount.decimals;
                                let param_amount_in = match amount_in.parse::<f64>() {
                                    Ok(num) => num,
                                    Err(e) => return,
                                };
                                if in_mint.mint == sol_address {
                                    param_mint = out_mint.mint;
                                    param_dirs = "buy".to_string();
                                } else {
                                    param_mint = in_mint.mint;
                                    param_dirs = "sell".to_string();
                                }
                                swap_to_events(
                                    param_mint,
                                    param_amount_in / 10_f64.powf(in_deciaml as f64),
                                    param_dirs,
                                )
                                .await;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

// Listen all events with websocket

pub async fn swap_to_events(mint: String, amount_in: f64, dirs: String) {
    let rpc_client = create_rpc_client().unwrap();
    let rpc_nonblocking_client = create_nonblocking_rpc_client().await.unwrap();
    let wallet = import_wallet().unwrap();
    let in_type = "qty";
    let slippage = import_env_var("SLIPPAGE").parse::<u64>().unwrap_or(5);
    let use_jito = true;

    let (pool_id, pool_state) = match get_pool_state_by_mint(rpc_client.clone(), &mint).await {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error fetching pool state: {}", err);
            return; // Propagates the error if needed
        }
    };
    let state = AppState {
        rpc_client,
        rpc_nonblocking_client,
        wallet,
    };

    println!("amount_in: {:#?}", amount_in.clone());
    let res = raydium_swap(
        state,
        amount_in.clone(),
        &dirs,
        in_type,
        slippage,
        use_jito,
        pool_id,
        pool_state,
    )
    .await;
    println!("res: {:#?}", res);
}
