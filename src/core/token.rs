use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use spl_token_2022::{
    extension::StateWithExtensionsOwned,
    state::{Account, Mint},
};
use spl_token_client::{
    client::{ProgramClient, ProgramRpcClient, ProgramRpcClientSendTransaction},
    token::{Token, TokenError, TokenResult},
};
use std::sync::Arc;

pub async fn get_account_info(
    client: Arc<solana_client::nonblocking::rpc_client::RpcClient>,
    _keypair: Arc<Keypair>,
    address: &Pubkey,
    account: &Pubkey,
) -> TokenResult<StateWithExtensionsOwned<Account>> {
    let program_client = Arc::new(ProgramRpcClient::new(
        client.clone(),
        ProgramRpcClientSendTransaction,
    ));
    let account = program_client
        .get_account(*account)
        .await
        .map_err(TokenError::Client)?
        .ok_or(TokenError::AccountNotFound)
        .inspect_err(|err| println!("get_account_info: {} {}: mint {}", account, err, address))?;

    if account.owner != spl_token::ID {
        return Err(TokenError::AccountInvalidOwner);
    }
    let account = StateWithExtensionsOwned::<Account>::unpack(account.data)?;
    if account.base.mint != *address {
        return Err(TokenError::AccountInvalidMint);
    }

    Ok(account)
}

