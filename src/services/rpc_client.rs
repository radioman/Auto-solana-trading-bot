use std::collections::HashMap;
use std::sync::Arc;
use anchor_client::solana_client::nonblocking::rpc_client::RpcClient;
use anchor_client::solana_sdk::pubkey::Pubkey;
use spl_token_2022::extension::StateWithExtensionsOwned;
use spl_token_2022::state::{Account, Mint};
use anyhow::Result;
use colored::Colorize;
use tokio::sync::RwLock;

use crate::common::logger::Logger;
use crate::common::cache::{TOKEN_ACCOUNT_CACHE, TOKEN_MINT_CACHE};

/// BatchRpcClient provides optimized methods for fetching multiple accounts in a single RPC call
pub struct BatchRpcClient {
    rpc_client: Arc<RpcClient>,
    connection_pool: Arc<RwLock<Vec<Arc<RpcClient>>>>,
    logger: Logger,
}

impl BatchRpcClient {
    pub fn new(rpc_client: Arc<RpcClient>) -> Self {
        // Create a connection pool with the initial client
        let mut pool = Vec::with_capacity(5);
        pool.push(rpc_client.clone());
        
        Self {
            rpc_client,
            connection_pool: Arc::new(RwLock::new(pool)),
            logger: Logger::new("[BATCH-RPC] => ".cyan().to_string()),
        }
    }
    
    /// Get a client from the connection pool
    pub async fn get_client(&self) -> Arc<RpcClient> {
        let pool = self.connection_pool.read().await;
        if pool.is_empty() {
            self.rpc_client.clone()
        } else {
            // Simple round-robin selection
            let index = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as usize % pool.len();
            pool[index].clone()
        }
    }
    
    /// Add a new client to the connection pool
    pub async fn add_client(&self, client: Arc<RpcClient>) {
        let mut pool = self.connection_pool.write().await;
        pool.push(client);
    }
    
    /// Get multiple token accounts in a single RPC call
    pub async fn get_multiple_token_accounts(
        &self, 
        mint: &Pubkey, 
        accounts: &[Pubkey]
    ) -> Result<HashMap<Pubkey, StateWithExtensionsOwned<Account>>> {
        let mut result = HashMap::new();
        let mut accounts_to_fetch = Vec::new();
        
        if accounts_to_fetch.is_empty() {
            return Ok(result);
        }
        
        self.logger.log(format!("Fetching {} token accounts in batch", accounts_to_fetch.len()));
        
        // Get accounts that weren't in cache
        let client = self.get_client().await;
        let fetched_accounts = client.get_multiple_accounts(&accounts_to_fetch).await?;
        

        
        Ok(result)
    }
    
    /// Get multiple mint accounts in a single RPC call
    pub async fn get_multiple_mints(
        &self, 
        mints: &[Pubkey]
    ) -> Result<HashMap<Pubkey, StateWithExtensionsOwned<Mint>>> {
        
        Ok(result)
    }
    
    /// Check if multiple token accounts exist in a single RPC call
    pub async fn check_multiple_accounts_exist(
        &self,
        accounts: &[Pubkey]
    ) -> Result<HashMap<Pubkey, bool>> {
        let mut result = HashMap::new();
        
        // Get accounts
        let client = self.get_client().await;
        let fetched_accounts = client.get_multiple_accounts(accounts).await?;
        
        for (i, maybe_account) in fetched_accounts.iter().enumerate() {
            result.insert(accounts[i], maybe_account.is_some());
        }
        
        Ok(result)
    }
}

/// Create a batch RPC client from an existing RPC client
pub fn create_batch_client(rpc_client: Arc<RpcClient>) -> BatchRpcClient {
    BatchRpcClient::new(rpc_client)
} 