use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub rpc_url: String,
    pub private_key: String,
    pub token_mint: String,
    pub amount: u64,
    pub stealth_mode: bool,
    pub user_agent: String,
    pub proxy: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
            private_key: String::new(),
            token_mint: String::new(),
            amount: 0,
            stealth_mode: true,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
            proxy: None,
        }
    }
    
    pub fn with_stealth_mode(mut self, enabled: bool) -> Self {
        self.stealth_mode = enabled;
        self
    }
    
    pub fn with_proxy(mut self, proxy: Option<String>) -> Self {
        self.proxy = proxy;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealthConfig {
    pub randomize_headers: bool,
    pub rotate_user_agents: bool,
    pub use_proxy_rotation: bool,
    pub delay_between_requests: u64,
    pub max_retries: u32,
    pub avoid_bubble_map: bool,
    pub custom_headers: HashMap<String, String>,
}

impl Default for StealthConfig {
    fn default() -> Self {
        let mut custom_headers = HashMap::new();
        custom_headers.insert("Accept".to_string(), "application/json".to_string());
        custom_headers.insert("Accept-Language".to_string(), "en-US,en;q=0.9".to_string());
        custom_headers.insert("Accept-Encoding".to_string(), "gzip, deflate, br".to_string());
        custom_headers.insert("Connection".to_string(), "keep-alive".to_string());
        custom_headers.insert("Upgrade-Insecure-Requests".to_string(), "1".to_string());
        
        Self {
            randomize_headers: true,
            rotate_user_agents: true,
            use_proxy_rotation: false,
            delay_between_requests: 1000, // 1 second
            max_retries: 3,
            avoid_bubble_map: true,
            custom_headers,
        }
    }
}
