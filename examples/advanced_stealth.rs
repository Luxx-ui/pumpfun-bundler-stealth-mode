use pumpfun_bundler_stealth::bundler::StealthBundler;
use pumpfun_bundler_stealth::config::{Config, StealthConfig};
use pumpfun_bundler_stealth::stealth::StealthEngine;
use log::{info, warn};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    info!("Starting advanced stealth bundler example");
    
    // Create advanced stealth configuration
    let mut custom_headers = HashMap::new();
    custom_headers.insert("X-Custom-Header".to_string(), "stealth-value".to_string());
    custom_headers.insert("X-Request-ID".to_string(), uuid::Uuid::new_v4().to_string());
    
    let stealth_config = StealthConfig {
        randomize_headers: true,
        rotate_user_agents: true,
        use_proxy_rotation: false,
        delay_between_requests: 2000, // 2 seconds
        max_retries: 5,
        avoid_bubble_map: true,
        custom_headers,
    };
    
    // Create configuration with proxy support
    let config = Config {
        rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
        private_key: "your_private_key_here".to_string(),
        token_mint: "your_token_mint_here".to_string(),
        amount: 500000, // 0.5 SOL
        stealth_mode: true,
        user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36".to_string(),
        proxy: Some("http://proxy-server:8080".to_string()), // Optional proxy
    };
    
    // Create stealth bundler
    let mut bundler = StealthBundler::new(config).await?;
    
    // Execute multiple bundles with stealth techniques
    info!("Executing multiple stealth bundles");
    
    let bundle_count = 3;
    match bundler.execute_multiple_bundles(bundle_count).await {
        Ok(results) => {
            info!("Successfully executed {} bundles", results.len());
            
            for (i, result) in results.iter().enumerate() {
                println!("Bundle {}: {}", i + 1, result.transaction_signature);
                println!("  Stealth ID: {}", result.stealth_id);
                println!("  Amount: {}", result.amount);
                println!("  Token: {}", result.token_mint);
                println!();
            }
            
            // Calculate success rate
            let success_rate = (results.len() as f64 / bundle_count as f64) * 100.0;
            println!("Success rate: {:.1}%", success_rate);
        }
        Err(e) => {
            warn!("Multiple bundle execution failed: {}", e);
            return Err(e.into());
        }
    }
    
    // Demonstrate stealth statistics
    let stats = bundler.get_stealth_stats();
    println!("Advanced stealth statistics:");
    for (key, value) in stats {
        println!("  {}: {}", key, value);
    }
    
    // Demonstrate stealth engine capabilities
    let stealth_engine = StealthEngine::new(stealth_config);
    
    // Generate stealth transaction ID
    let stealth_id = stealth_engine.generate_stealth_transaction_id();
    println!("Generated stealth ID: {}", stealth_id);
    
    // Demonstrate amount obfuscation
    let original_amount = 1000000;
    let obfuscated_amount = stealth_engine.obfuscate_amount(original_amount);
    println!("Original amount: {}", original_amount);
    println!("Obfuscated amount: {}", obfuscated_amount);
    
    Ok(())
}
