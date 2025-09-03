use clap::Parser;
use log::{info, error};
use std::env;

mod bundler;
mod stealth;
mod config;
mod utils;

use bundler::StealthBundler;
use config::Config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// RPC endpoint URL
    #[arg(short, long, default_value = "https://api.mainnet-beta.solana.com")]
    rpc_url: String,
    
    /// Private key for signing transactions
    #[arg(short, long)]
    private_key: String,
    
    /// Target token mint address
    #[arg(short, long)]
    token_mint: String,
    
    /// Amount to bundle (in lamports)
    #[arg(short, long)]
    amount: u64,
    
    /// Enable stealth mode (default: true)
    #[arg(long, default_value = "true")]
    stealth: bool,
    
    /// Custom user agent for requests
    #[arg(long)]
    user_agent: Option<String>,
    
    /// Proxy configuration
    #[arg(long)]
    proxy: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let args = Args::parse();
    
    info!("Starting PumpFun Bundler Stealth Mode");
    
    let config = Config {
        rpc_url: args.rpc_url,
        private_key: args.private_key,
        token_mint: args.token_mint,
        amount: args.amount,
        stealth_mode: args.stealth,
        user_agent: args.user_agent.unwrap_or_else(|| {
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string()
        }),
        proxy: args.proxy,
    };
    
    let mut bundler = StealthBundler::new(config).await?;
    
    match bundler.execute_bundle().await {
        Ok(result) => {
            info!("Bundle executed successfully: {:?}", result);
        }
        Err(e) => {
            error!("Bundle execution failed: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}
