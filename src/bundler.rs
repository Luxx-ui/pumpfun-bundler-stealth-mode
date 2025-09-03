use crate::config::{Config, StealthConfig};
use crate::stealth::StealthEngine;
use crate::utils;
use anyhow::{anyhow, Result};
use log::{debug, info, warn};
use reqwest::Client;
use serde_json::{json, Value};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::collections::HashMap;
use std::str::FromStr;

pub struct StealthBundler {
    config: Config,
    stealth_engine: StealthEngine,
    client: Client,
    rpc_client: RpcClient,
    keypair: Keypair,
}

#[derive(Debug)]
pub struct BundleResult {
    pub transaction_signature: String,
    pub bundle_id: Option<String>,
    pub stealth_id: String,
    pub amount: u64,
    pub token_mint: String,
}

impl StealthBundler {
    pub async fn new(config: Config) -> Result<Self> {
        let stealth_config = StealthConfig::default();
        let stealth_engine = StealthEngine::new(stealth_config);
        
        // Create HTTP client with stealth configuration
        let mut client_builder = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent(&config.user_agent);
        
        // Add proxy if configured
        if let Some(proxy_url) = &config.proxy {
            client_builder = client_builder.proxy(reqwest::Proxy::all(proxy_url)?);
        }
        
        let client = client_builder.build()?;
        
        // Create RPC client
        let rpc_client = RpcClient::new_with_commitment(
            config.rpc_url.clone(),
            CommitmentConfig::confirmed(),
        );
        
        // Parse private key
        let keypair = if config.private_key.starts_with('[') {
            // Array format
            let bytes: Vec<u8> = serde_json::from_str(&config.private_key)?;
            Keypair::from_bytes(&bytes)?
        } else {
            // Base58 format
            let bytes = bs58::decode(&config.private_key).into_vec()?;
            Keypair::from_bytes(&bytes)?
        };
        
        Ok(Self {
            config,
            stealth_engine,
            client,
            rpc_client,
            keypair,
        })
    }
    
    pub async fn execute_bundle(&mut self) -> Result<BundleResult> {
        info!("Starting stealth bundle execution");
        
        // Apply stealth delay
        self.stealth_engine.stealth_delay().await;
        
        // Generate stealth transaction ID
        let stealth_id = self.stealth_engine.generate_stealth_transaction_id();
        
        // Obfuscate amount to avoid pattern detection
        let obfuscated_amount = self.stealth_engine.obfuscate_amount(self.config.amount);
        
        // Create stealth transaction data
        let mut transaction_data = self.create_stealth_transaction_data(&stealth_id, obfuscated_amount)?;
        
        // Apply bubble map avoidance techniques
        self.stealth_engine.avoid_bubble_map_detection(&mut transaction_data);
        
        // Execute the transaction
        let signature = self.execute_stealth_transaction(transaction_data).await?;
        
        info!("Stealth bundle executed successfully");
        
        Ok(BundleResult {
            transaction_signature: signature,
            bundle_id: None, // No bundle ID to avoid detection
            stealth_id,
            amount: obfuscated_amount,
            token_mint: self.config.token_mint.clone(),
        })
    }
    
    fn create_stealth_transaction_data(&self, stealth_id: &str, amount: u64) -> Result<HashMap<String, Value>> {
        let mut data = HashMap::new();
        
        // Add basic transaction info without bundler flags
        data.insert("from".to_string(), json!(self.keypair.pubkey().to_string()));
        data.insert("to".to_string(), json!(self.config.token_mint));
        data.insert("amount".to_string(), json!(amount));
        data.insert("stealth_id".to_string(), json!(stealth_id));
        
        // Add random metadata to make it look like regular transaction
        let mut rng = rand::thread_rng();
        data.insert("nonce".to_string(), json!(rng.gen::<u64>()));
        data.insert("gas_price".to_string(), json!(rng.gen_range(5000..10000)));
        
        Ok(data)
    }
    
    async fn execute_stealth_transaction(&mut self, transaction_data: HashMap<String, Value>) -> Result<String> {
        info!("Executing stealth transaction");
        
        // Create stealth headers
        let headers = self.stealth_engine.create_stealth_headers();
        
        // Convert transaction data to JSON
        let payload = self.stealth_engine.create_stealth_rpc_payload(
            "sendTransaction",
            json!([transaction_data])
        );
        
        // Send transaction with stealth configuration
        let response = self.client
            .post(&self.config.rpc_url)
            .headers(headers)
            .json(&payload)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("RPC request failed: {}", response.status()));
        }
        
        let response_text = response.text().await?;
        debug!("RPC Response: {}", response_text);
        
        // Parse response
        let response_json: Value = serde_json::from_str(&response_text)?;
        
        if let Some(error) = response_json.get("error") {
            return Err(anyhow!("Transaction failed: {}", error));
        }
        
        if let Some(result) = response_json.get("result") {
            if let Some(signature) = result.get("signature") {
                return Ok(signature.as_str().unwrap().to_string());
            }
        }
        
        Err(anyhow!("Invalid response format"))
    }
    
    pub async fn execute_multiple_bundles(&mut self, count: usize) -> Result<Vec<BundleResult>> {
        info!("Executing {} stealth bundles", count);
        
        let mut results = Vec::new();
        
        for i in 0..count {
            info!("Executing bundle {}/{}", i + 1, count);
            
            match self.execute_bundle().await {
                Ok(result) => {
                    results.push(result);
                    info!("Bundle {}/{} completed successfully", i + 1, count);
                }
                Err(e) => {
                    warn!("Bundle {}/{} failed: {}", i + 1, count, e);
                    // Continue with next bundle
                }
            }
            
            // Apply stealth delay between bundles
            if i < count - 1 {
                self.stealth_engine.stealth_delay().await;
            }
        }
        
        Ok(results)
    }
    
    pub fn get_stealth_stats(&self) -> HashMap<String, Value> {
        let mut stats = HashMap::new();
        stats.insert("stealth_mode_enabled".to_string(), json!(self.config.stealth_mode));
        stats.insert("bubble_map_avoidance".to_string(), json!(self.stealth_engine.config.avoid_bubble_map));
        stats.insert("user_agent_rotation".to_string(), json!(self.stealth_engine.config.rotate_user_agents));
        stats.insert("header_randomization".to_string(), json!(self.stealth_engine.config.randomize_headers));
        stats
    }
}
