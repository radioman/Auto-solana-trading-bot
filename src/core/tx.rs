use std::{env, sync::Arc, time::Duration};

use anyhow::Result;
use jito_json_rpc_client::jsonrpc_client::rpc_client::RpcClient as JitoRpcClient;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    signature::Keypair,
    signer::Signer,
    system_transaction,
    transaction::{Transaction, VersionedTransaction},
};
use spl_token::ui_amount_to_amount;

use std::str::FromStr;
use tokio::time::Instant;

use crate::{
    common::logger::Logger,
    services::jito::{
        self, get_tip_account, get_tip_value, init_tip_accounts, wait_for_bundle_confirmation,
    },
};

// prioritization fee = UNIT_PRICE * UNIT_LIMIT
fn get_unit_price() -> u64 {
    env::var("UNIT_PRICE")
        .ok()
        .and_then(|v| u64::from_str(&v).ok())
        .unwrap_or(1)
}

fn get_unit_limit() -> u32 {
    env::var("UNIT_LIMIT")
        .ok()
        .and_then(|v| u32::from_str(&v).ok())
        .unwrap_or(300_000)
}
pub async fn get_mint_info(
    client: Arc<solana_client::nonblocking::rpc_client::RpcClient>,
    _keypair: Arc<Keypair>,
    address: &Pubkey,
) -> TokenResult<StateWithExtensionsOwned<Mint>> {
    let program_client = Arc::new(ProgramRpcClient::new(
        client.clone(),
        ProgramRpcClientSendTransaction,
    ))
}


    let start_time = Instant::now();
    let mut txs = vec![];
    if use_jito {
        // jito
        init_tip_accounts().await;
        let tip_account = get_tip_account().await?;
        let jito_client = Arc::new(JitoRpcClient::new(format!(
            "{}/api/v1/bundles",
            *jito::BLOCK_ENGINE_URL
        )));
        // jito tip, the upper limit is 0.1
        let mut tip = get_tip_value().await?;
        tip = tip.min(0.1);
        let tip_lamports = ui_amount_to_amount(tip, spl_token::native_mint::DECIMALS);
        logger.log(format!(
            "tip account: {}, tip(sol): {}, lamports: {}",
            tip_account, tip, tip_lamports
        ));
        // tip tx
        let bundle: Vec<VersionedTransaction> = vec![
            VersionedTransaction::from(txn),
            VersionedTransaction::from(system_transaction::transfer(
                keypair,
                &tip_account,
                tip_lamports,
                recent_blockhash,
            )),
        ];
        let bundle_id = jito_client.send_bundle(&bundle).await?;
        logger.log(format!("bundle_id: {}", bundle_id));

        logger.log(format!("tx ellapsed: {:?}", start_time.elapsed()));
        txs = wait_for_bundle_confirmation(
            move |id: String| {
                let client = Arc::clone(&jito_client);
                async move {
                    let response = client.get_bundle_statuses(&[id]).await;
                    let statuses = response.inspect_err(|err| {
                        logger.log(format!("Error fetching bundle status: {:?}", err));
                    })?;
                    Ok(statuses.value)
                }
            },
            bundle_id,
            Duration::from_millis(1000),
            Duration::from_secs(10),
        )
        .await?;
    } else {
        let sig = common::rpc::send_txn(client, &txn, true)?;
        logger.log(format!("signature: {:#?}", sig));
        txs.push(sig.to_string());
    }

    Ok(txs)
}
