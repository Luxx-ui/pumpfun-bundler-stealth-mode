# PumpFun Bundler Stealth Mode Guide

## Overview

This stealth bundler is designed to avoid detection by removing bundler flags and implementing various anti-detection techniques. It operates without traditional bundler identifiers and avoids bubble map fund source tracking.

## Key Stealth Features

### 1. No Bundler Flags
- Removes all bundler-specific identifiers from transactions
- No `bundler`, `bundle`, `bundle_id`, or `bundle_hash` fields
- Transactions appear as regular user transactions

### 2. Bubble Map Avoidance
- Removes fund source indicators (`fund_source`, `source`, `origin`)
- Implements amount obfuscation with small random variations
- Adds random transaction metadata to obfuscate patterns

### 3. Header Randomization
- Rotates between different user agents
- Randomizes HTTP headers (Accept-Language, Accept-Encoding, etc.)
- Adds random Sec-Fetch headers
- Optional DNT (Do Not Track) headers

### 4. Transaction Obfuscation
- Generates random transaction IDs
- Adds timestamp randomization
- Includes random noise in transaction data
- Amount variations to avoid pattern detection

### 5. Stealth Delays
- Random delays between requests
- Configurable delay ranges
- Prevents rate limiting and pattern detection

## Usage Instructions

### Basic Setup

1. **Install Dependencies**
   ```bash
   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Build the project
   cargo build --release
   ```

2. **Configure Your Settings**
   - Edit `config.json` to customize stealth settings
   - Set your private key and token mint address
   - Configure RPC endpoints

3. **Run the Bundler**
   ```bash
   cargo run --release -- \
       --private-key "your_private_key" \
       --token-mint "token_mint_address" \
       --amount 1000000
   ```

### Advanced Configuration

#### Stealth Configuration Options

```json
{
  "stealth_config": {
    "randomize_headers": true,        // Randomize HTTP headers
    "rotate_user_agents": true,       // Rotate user agents
    "use_proxy_rotation": false,      // Use proxy rotation
    "delay_between_requests": 1000,   // Base delay in milliseconds
    "max_retries": 3,                // Maximum retry attempts
    "avoid_bubble_map": true,         // Avoid bubble map detection
    "custom_headers": {              // Custom HTTP headers
      "Accept": "application/json",
      "Accept-Language": "en-US,en;q=0.9"
    }
  }
}
```

#### Stealth Features

```json
{
  "stealth_features": {
    "amount_obfuscation": true,      // Add small variations to amounts
    "timestamp_randomization": true, // Randomize timestamps
    "transaction_noise": true,       // Add random noise to transactions
    "header_randomization": true,    // Randomize HTTP headers
    "delay_randomization": true      // Add random delays
  }
}
```

### Command Line Options

| Option | Description | Default |
|--------|-------------|---------|
| `--rpc-url` | RPC endpoint URL | `https://api.mainnet-beta.solana.com` |
| `--private-key` | Private key for signing | Required |
| `--token-mint` | Target token mint address | Required |
| `--amount` | Amount in lamports | Required |
| `--stealth` | Enable stealth mode | `true` |
| `--user-agent` | Custom user agent | Auto-generated |
| `--proxy` | Proxy URL | None |

### Examples

#### Single Bundle
```bash
cargo run --release -- \
    --private-key "5KJvsngHeMpm884wtkJNzQGaCErckhHJBGFsvd3VyK5qMZXj3hS" \
    --token-mint "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v" \
    --amount 1000000 \
    --stealth true
```

#### Multiple Bundles with Proxy
```bash
cargo run --release -- \
    --private-key "your_private_key" \
    --token-mint "token_mint_address" \
    --amount 500000 \
    --stealth true \
    --proxy "http://proxy-server:8080" \
    --user-agent "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
```

## Anti-Detection Techniques

### 1. Transaction Format
- No bundler-specific fields
- Regular transaction structure
- Random metadata fields
- Natural-looking timestamps

### 2. Network Behavior
- Random delays between requests
- Rotating user agents
- Randomized headers
- Natural request patterns

### 3. Amount Obfuscation
- Small random variations (±100 lamports)
- No round numbers
- Natural-looking amounts
- Pattern avoidance

### 4. Metadata Randomization
- Random transaction IDs
- Timestamp variations
- Noise fields
- Session identifiers

## Best Practices

### 1. Security
- Use strong, unique private keys
- Enable stealth mode by default
- Use proxy servers when possible
- Rotate RPC endpoints

### 2. Performance
- Adjust delays based on transaction volume
- Monitor success rates
- Use multiple RPC endpoints
- Implement retry logic

### 3. Stealth
- Vary transaction amounts
- Use different user agents
- Randomize timing patterns
- Avoid predictable behavior

### 4. Monitoring
- Track transaction success rates
- Monitor for detection patterns
- Log stealth statistics
- Analyze network behavior

## Troubleshooting

### Common Issues

1. **Build Errors**
   - Ensure Rust is properly installed
   - Install Visual Studio build tools (Windows)
   - Update Rust toolchain: `rustup update`

2. **Network Issues**
   - Check RPC endpoint availability
   - Verify proxy configuration
   - Test network connectivity
   - Adjust timeout settings

3. **Transaction Failures**
   - Verify private key format
   - Check token mint address
   - Ensure sufficient balance
   - Review RPC response errors

4. **Stealth Issues**
   - Enable all stealth features
   - Increase delay times
   - Use proxy rotation
   - Randomize patterns more

### Debug Mode

Enable debug logging:
```bash
RUST_LOG=debug cargo run --release -- [options]
```

### Performance Optimization

1. **Batch Processing**
   - Use multiple bundle execution
   - Optimize delay times
   - Parallel processing where possible

2. **Resource Management**
   - Monitor memory usage
   - Optimize network connections
   - Efficient error handling

## Legal and Ethical Considerations

- This software is for educational purposes
- Use responsibly and legally
- Respect rate limits and terms of service
- Follow applicable regulations
- Do not use for malicious purposes

## Support

For issues and questions:
1. Check the troubleshooting section
2. Review configuration options
3. Enable debug logging
4. Check network connectivity
5. Verify all dependencies

## Disclaimer

This software is provided as-is without warranties. Users are responsible for ensuring compliance with all applicable laws and regulations. The authors are not responsible for any misuse of this software.
