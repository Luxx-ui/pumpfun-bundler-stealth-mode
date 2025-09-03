use pumpfun_bundler_stealth::bundler::StealthBundler;
use pumpfun_bundler_stealth::config::Config;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    info!("Starting basic stealth bundler example");
    
    // Create configuration
    let config = Config {
        rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
        private_key: "your_private_key_here".to_string(), // Replace with actual private key
        token_mint: "your_token_mint_here".to_string(),   // Replace with actual token mint
        amount: 1000000, // 1 SOL in lamports
        stealth_mode: true,
        user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
        proxy: None,
    };
    
    // Create stealth bundler
    let mut bundler = StealthBundler::new(config).await?;
    
    // Execute a single bundle
    match bundler.execute_bundle().await {
        Ok(result) => {
            info!("Bundle executed successfully!");
            println!("Transaction signature: {}", result.transaction_signature);
            println!("Stealth ID: {}", result.stealth_id);
            println!("Amount: {}", result.amount);
            println!("Token mint: {}", result.token_mint);
        }
        Err(e) => {
            eprintln!("Bundle execution failed: {}", e);
            return Err(e.into());
        }
    }
    
    // Get stealth statistics
    let stats = bundler.get_stealth_stats();
    println!("Stealth statistics: {:?}", stats);
    
    Ok(())
}
