use crate::config::StealthConfig;
use rand::Rng;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use log::{debug, info};

pub struct StealthEngine {
    config: StealthConfig,
    user_agents: Vec<String>,
    current_user_agent_index: usize,
}

impl StealthEngine {
    pub fn new(config: StealthConfig) -> Self {
        let user_agents = vec![
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/121.0".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/121.0".to_string(),
        ];
        
        Self {
            config,
            user_agents,
            current_user_agent_index: 0,
        }
    }
    
    pub fn create_stealth_headers(&mut self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        
        // Rotate user agent if enabled
        if self.config.rotate_user_agents {
            let user_agent = &self.user_agents[self.current_user_agent_index];
            headers.insert(USER_AGENT, HeaderValue::from_str(user_agent).unwrap());
            self.current_user_agent_index = (self.current_user_agent_index + 1) % self.user_agents.len();
        }
        
        // Add custom headers
        for (key, value) in &self.config.custom_headers {
            if let Ok(header_value) = HeaderValue::from_str(value) {
                headers.insert(key.parse().unwrap(), header_value);
            }
        }
        
        // Randomize headers if enabled
        if self.config.randomize_headers {
            self.add_random_headers(&mut headers);
        }
        
        headers
    }
    
    fn add_random_headers(&self, headers: &mut HeaderMap) {
        let mut rng = rand::thread_rng();
        
        // Add random Accept-Language variations
        let languages = vec![
            "en-US,en;q=0.9",
            "en-GB,en;q=0.9",
            "en-CA,en;q=0.9",
            "en-AU,en;q=0.9",
        ];
        let random_lang = languages[rng.gen_range(0..languages.len())];
        headers.insert("Accept-Language", HeaderValue::from_str(random_lang).unwrap());
        
        // Add random Accept-Encoding
        let encodings = vec![
            "gzip, deflate, br",
            "gzip, deflate",
            "br, gzip, deflate",
        ];
        let random_encoding = encodings[rng.gen_range(0..encodings.len())];
        headers.insert("Accept-Encoding", HeaderValue::from_str(random_encoding).unwrap());
        
        // Add random DNT header
        if rng.gen_bool(0.5) {
            headers.insert("DNT", HeaderValue::from_str("1").unwrap());
        }
        
        // Add random Sec-Fetch headers
        headers.insert("Sec-Fetch-Dest", HeaderValue::from_str("empty").unwrap());
        headers.insert("Sec-Fetch-Mode", HeaderValue::from_str("cors").unwrap());
        headers.insert("Sec-Fetch-Site", HeaderValue::from_str("same-origin").unwrap());
    }
    
    pub async fn stealth_delay(&self) {
        if self.config.delay_between_requests > 0 {
            let mut rng = rand::thread_rng();
            let base_delay = self.config.delay_between_requests;
            let random_delay = rng.gen_range(base_delay..base_delay + 500); // Add up to 500ms randomness
            sleep(Duration::from_millis(random_delay)).await;
        }
    }
    
    pub fn avoid_bubble_map_detection(&self, transaction_data: &mut HashMap<String, serde_json::Value>) {
        if !self.config.avoid_bubble_map {
            return;
        }
        
        info!("Applying bubble map avoidance techniques");
        
        // Remove any bundler-specific flags
        transaction_data.remove("bundler");
        transaction_data.remove("bundle");
        transaction_data.remove("bundle_id");
        transaction_data.remove("bundle_hash");
        
        // Remove any fund source indicators
        transaction_data.remove("fund_source");
        transaction_data.remove("source");
        transaction_data.remove("origin");
        
        // Add random transaction metadata to obfuscate
        let mut rng = rand::thread_rng();
        let random_id = format!("tx_{:016x}", rng.gen::<u64>());
        transaction_data.insert("transaction_id".to_string(), serde_json::Value::String(random_id));
        
        // Add timestamp to make it look like a regular transaction
        let timestamp = chrono::Utc::now().timestamp_millis();
        transaction_data.insert("timestamp".to_string(), serde_json::Value::Number(timestamp.into()));
    }
    
    pub fn generate_stealth_transaction_id(&self) -> String {
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        bs58::encode(random_bytes).into_string()
    }
    
    pub fn obfuscate_amount(&self, amount: u64) -> u64 {
        if !self.config.avoid_bubble_map {
            return amount;
        }
        
        let mut rng = rand::thread_rng();
        // Add small random variation to avoid pattern detection
        let variation = rng.gen_range(-100..100);
        let adjusted_amount = amount as i64 + variation;
        adjusted_amount.max(1) as u64 // Ensure minimum amount
    }
    
    pub fn create_stealth_rpc_payload(&self, method: &str, params: serde_json::Value) -> serde_json::Value {
        let mut rng = rand::thread_rng();
        
        // Create a stealth RPC payload that doesn't look like bundler activity
        let mut payload = serde_json::Map::new();
        payload.insert("jsonrpc".to_string(), serde_json::Value::String("2.0".to_string()));
        payload.insert("id".to_string(), serde_json::Value::Number(rng.gen_range(1..1000000).into()));
        payload.insert("method".to_string(), serde_json::Value::String(method.to_string()));
        payload.insert("params".to_string(), params);
        
        serde_json::Value::Object(payload)
    }
}
