# PumpFun Bundler Stealth Mode

A stealth implementation of PumpFun bundler that avoids detection flags and bubble map fund sources. This bundler operates without traditional bundler identifiers and implements various anti-detection techniques.

<div align="center">

### 📞 Contact & Support

[![Telegram](https://img.shields.io/badge/Telegram-2CA5E0?style=for-the-badge&logo=telegram&logoColor=white)](https://t.me/heliusdevlabs)

**💬 Get in touch for support, questions, or collaboration**

</div>

## Features

### Stealth Mode Capabilities
- **No Bundler Flags**: Removes all bundler-specific identifiers from transactions
- **Bubble Map Avoidance**: Prevents detection by bubble map fund source tracking
- **Header Randomization**: Randomizes HTTP headers to avoid pattern detection
- **User Agent Rotation**: Rotates between different user agents
- **Transaction Obfuscation**: Adds random noise to transaction data
- **Stealth Delays**: Implements random delays between requests
- **Proxy Support**: Optional proxy configuration for additional anonymity

### Anti-Detection Techniques
- Random transaction IDs and metadata
- Amount obfuscation with small variations
- Timestamp randomization
- Custom RPC payload formatting
- Stealth logging with noise

## Installation

```bash
# Clone the repository
git clone <repository-url>
cd pumpfun-bundler-stealth-mode

# Build the project
cargo build --release

# Run the bundler
cargo run --release -- --help
```

## Usage

### Basic Usage

```bash
cargo run --release -- \
    --private-key "your_private_key_here" \
    --token-mint "token_mint_address" \
    --amount 1000000 \
    --rpc-url "https://api.mainnet-beta.solana.com"
```

### Advanced Usage with Stealth Options

```bash
cargo run --release -- \
    --private-key "your_private_key_here" \
    --token-mint "token_mint_address" \
    --amount 1000000 \
    --rpc-url "https://api.mainnet-beta.solana.com" \
    --stealth true \
    --user-agent "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36" \
    --proxy "http://proxy-server:8080"
```

### Command Line Options

- `--rpc-url`: RPC endpoint URL (default: https://api.mainnet-beta.solana.com)
- `--private-key`: Private key for signing transactions
- `--token-mint`: Target token mint address
- `--amount`: Amount to bundle (in lamports)
- `--stealth`: Enable stealth mode (default: true)
- `--user-agent`: Custom user agent for requests
- `--proxy`: Proxy configuration URL

## Configuration

### Stealth Configuration

The bundler includes several stealth configuration options:

```rust
let stealth_config = StealthConfig {
    randomize_headers: true,        // Randomize HTTP headers
    rotate_user_agents: true,       // Rotate user agents
    use_proxy_rotation: false,      // Use proxy rotation
    delay_between_requests: 1000,   // Delay between requests (ms)
    max_retries: 3,                 // Maximum retry attempts
    avoid_bubble_map: true,         // Avoid bubble map detection
    custom_headers: HashMap::new(), // Custom headers
};
```

### Environment Variables

You can also configure the bundler using environment variables:

```bash
export RPC_URL="https://api.mainnet-beta.solana.com"
export PRIVATE_KEY="your_private_key"
export TOKEN_MINT="token_mint_address"
export AMOUNT="1000000"
export STEALTH_MODE="true"
```

## Security Features

### Anti-Detection Measures

1. **No Bundler Flags**: Transactions are sent without any bundler-specific identifiers
2. **Fund Source Obfuscation**: Removes any indicators of fund sources
3. **Pattern Avoidance**: Implements random delays and variations
4. **Header Randomization**: Randomizes HTTP headers to avoid fingerprinting
5. **Transaction Noise**: Adds random metadata to transactions

### Privacy Features

1. **User Agent Rotation**: Cycles through different user agents
2. **Proxy Support**: Optional proxy configuration
3. **Stealth Logging**: Logs include random noise
4. **Transaction Obfuscation**: Amounts are slightly randomized

## Examples

### Single Bundle Execution

```rust
use pumpfun_bundler_stealth::bundler::StealthBundler;
use pumpfun_bundler_stealth::config::Config;

let config = Config {
    rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
    private_key: "your_private_key".to_string(),
    token_mint: "token_mint_address".to_string(),
    amount: 1000000,
    stealth_mode: true,
    user_agent: "Mozilla/5.0...".to_string(),
    proxy: None,
};

let mut bundler = StealthBundler::new(config).await?;
let result = bundler.execute_bundle().await?;
println!("Bundle executed: {:?}", result);
```

### Multiple Bundle Execution

```rust
let results = bundler.execute_multiple_bundles(5).await?;
for result in results {
    println!("Bundle completed: {:?}", result);
}
```

## Building from Source

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build --release

# Run tests
cargo test

# Check for issues
cargo clippy
```

## Dependencies

- **tokio**: Async runtime
- **solana-client**: Solana RPC client
- **solana-sdk**: Solana SDK
- **reqwest**: HTTP client
- **serde**: Serialization
- **clap**: Command line argument parsing
- **rand**: Random number generation
- **log**: Logging framework

## License

This project is for educational purposes only. Use responsibly and in accordance with applicable laws and regulations.

## Disclaimer

This software is provided as-is without any warranties. Users are responsible for ensuring compliance with all applicable laws and regulations. The authors are not responsible for any misuse of this software.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## Support

For issues and questions, please open an issue on the GitHub repository.