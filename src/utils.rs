use anyhow::Result;
use log::{debug, info};
use rand::Rng;
use serde_json::Value;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_random_delay(min_ms: u64, max_ms: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min_ms..max_ms)
}

pub fn obfuscate_transaction_data(mut data: HashMap<String, Value>) -> HashMap<String, Value> {
    let mut rng = rand::thread_rng();
    
    // Add random noise to transaction data
    let noise_key = format!("noise_{}", rng.gen::<u32>());
    data.insert(noise_key, Value::String(format!("{}", rng.gen::<u64>())));
    
    // Add timestamp with slight randomization
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let randomized_timestamp = timestamp + rng.gen_range(0..1000);
    data.insert("timestamp".to_string(), Value::Number(randomized_timestamp.into()));
    
    data
}

pub fn validate_token_mint(mint_address: &str) -> Result<()> {
    if mint_address.len() != 44 {
        return Err(anyhow::anyhow!("Invalid mint address length"));
    }
    
    // Basic validation - check if it's base58 encoded
    if bs58::decode(mint_address).into_vec().is_err() {
        return Err(anyhow::anyhow!("Invalid mint address format"));
    }
    
    Ok(())
}

pub fn sanitize_private_key(private_key: &str) -> Result<String> {
    // Remove any whitespace or formatting
    let cleaned = private_key.trim().replace([' ', '\n', '\r', '\t'], "");
    
    if cleaned.is_empty() {
        return Err(anyhow::anyhow!("Private key cannot be empty"));
    }
    
    Ok(cleaned)
}

pub fn create_stealth_metadata() -> HashMap<String, Value> {
    let mut rng = rand::thread_rng();
    let mut metadata = HashMap::new();
    
    // Add random metadata to make transactions look more natural
    metadata.insert("version".to_string(), Value::String("1.0.0".to_string()));
    metadata.insert("client_id".to_string(), Value::String(format!("client_{}", rng.gen::<u32>())));
    metadata.insert("session_id".to_string(), Value::String(uuid::Uuid::new_v4().to_string()));
    
    metadata
}

pub fn calculate_optimal_delay(transaction_count: usize) -> u64 {
    // Calculate delay based on transaction count to avoid rate limiting
    match transaction_count {
        0..=10 => 1000,    // 1 second for small batches
        11..=50 => 2000,   // 2 seconds for medium batches
        51..=100 => 5000,  // 5 seconds for large batches
        _ => 10000,        // 10 seconds for very large batches
    }
}

pub fn format_amount_for_display(amount: u64, decimals: u8) -> String {
    let divisor = 10_u64.pow(decimals as u32);
    let whole = amount / divisor;
    let fraction = amount % divisor;
    
    if fraction == 0 {
        format!("{}", whole)
    } else {
        format!("{}.{:0width$}", whole, fraction, width = decimals as usize)
    }
}

pub fn generate_stealth_filename() -> String {
    let mut rng = rand::thread_rng();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let random_suffix = rng.gen::<u32>();
    
    format!("tx_{}_{:08x}.json", timestamp, random_suffix)
}

pub fn validate_rpc_url(url: &str) -> Result<()> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(anyhow::anyhow!("RPC URL must start with http:// or https://"));
    }
    
    if url.len() < 10 {
        return Err(anyhow::anyhow!("RPC URL too short"));
    }
    
    Ok(())
}

pub fn create_stealth_log_message(message: &str) -> String {
    let mut rng = rand::thread_rng();
    let noise = rng.gen::<u32>();
    format!("[{}] {}", noise, message)
}
