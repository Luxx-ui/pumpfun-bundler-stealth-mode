pub mod bundler;
pub mod config;
pub mod stealth;
pub mod utils;

// Re-export main types for easier access
pub use bundler::{StealthBundler, BundleResult};
pub use config::{Config, StealthConfig};
pub use stealth::StealthEngine;

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Initialize the stealth bundler library
pub fn init() {
    env_logger::init();
}

/// Get library information
pub fn get_info() -> std::collections::HashMap<String, String> {
    let mut info = std::collections::HashMap::new();
    info.insert("name".to_string(), NAME.to_string());
    info.insert("version".to_string(), VERSION.to_string());
    info.insert("rust_version".to_string(), env!("RUST_VERSION").to_string());
    info
}
