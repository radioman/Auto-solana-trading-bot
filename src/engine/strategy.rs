use crate::common::utils::AppState;
use crate::dex::pump_fun::Pump;
use crate::dex::raydium::Raydium;
use anyhow::Result;
use clap::ValueEnum;
use raydium_amm::state::AmmInfo;
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;

#[derive(ValueEnum, Debug, Clone, Deserialize)]
pub enum SwapDirection {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}
impl From<SwapDirection> for u8 {
    fn from(value: SwapDirection) -> Self {
        match value {
            SwapDirection::Buy => 0,
            SwapDirection::Sell => 1,
        }
    }
}

#[derive(ValueEnum, Debug, Clone, Deserialize)]
pub enum SwapInType {
    /// Quantity
    #[serde(rename = "qty")]
    Qty,
    /// Percentage
    #[serde(rename = "pct")]
    Pct,
}

pub async fn raydium_swap(
    state: AppState,
    amount_in: f64,
    swap_direction: &str,
    in_type: &str,
    slippage: u64,
    use_jito: bool,
    amm_pool_id: Pubkey,
    pool_state: AmmInfo,
) -> Result<Vec<String>> {
    let swap_direction = match swap_direction {
        "buy" => SwapDirection::Buy,
        "sell" => SwapDirection::Sell,
        _ => todo!(),
    };
    let in_type = match in_type {
        "qty" => SwapInType::Qty,
        "pct" => SwapInType::Pct,
        _ => todo!(),
    };
    let swapx = Raydium::new(state.rpc_nonblocking_client, state.rpc_client, state.wallet);
    let res = match swapx
        .swap(
            amount_in,
            swap_direction,
            in_type,
            slippage,
            use_jito,
            amm_pool_id,
            pool_state,
        )
        .await
    {
        Ok(res) => res,
        Err(e) => {
            return Err(e);
        }
    };
    Ok(res)
}

pub async fn pump_swap(
    state: AppState,
    amount_in: f64,
    swap_direction: &str,
    in_type: &str,
    slippage: u64,
    use_jito: bool,
    mint: &str,
) -> Result<Vec<String>> {
    let swap_direction = match swap_direction {
        "buy" => SwapDirection::Buy,
        "sell" => SwapDirection::Sell,
        _ => todo!(),
    };
    let in_type = match in_type {
        "qty" => SwapInType::Qty,
        "pct" => SwapInType::Pct,
        _ => todo!(),
    };
    let swapx = Pump::new(state.rpc_nonblocking_client, state.rpc_client, state.wallet);
    let res = match swapx
        .swap(mint, amount_in, swap_direction, in_type, slippage, use_jito)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            return Err(e);
        }
    };
    Ok(res)
}
