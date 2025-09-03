#!/usr/bin/env python3
"""
Example usage of the PumpFun Bundler Stealth Mode
"""

from stealth_bundler import StealthBundler, StealthConfig
import json

def main():
    print("PumpFun Bundler Stealth Mode - Example")
    print("=" * 50)
    
    # Configuration
    rpc_url = "https://api.mainnet-beta.solana.com"
    private_key = "your_private_key_here"  # Replace with actual private key
    token_mint = "your_token_mint_here"    # Replace with actual token mint
    
    # Stealth configuration
    stealth_config = StealthConfig(
        randomize_headers=True,
        rotate_user_agents=True,
        delay_between_requests=1000,
        max_retries=3,
        avoid_bubble_map=True,
        amount_obfuscation=True,
        timestamp_randomization=True
    )
    
    print("Stealth Configuration:")
    print(f"  Randomize Headers: {stealth_config.randomize_headers}")
    print(f"  Rotate User Agents: {stealth_config.rotate_user_agents}")
    print(f"  Avoid Bubble Map: {stealth_config.avoid_bubble_map}")
    print(f"  Amount Obfuscation: {stealth_config.amount_obfuscation}")
    print()
    
    # Create bundler
    bundler = StealthBundler(rpc_url, private_key, stealth_config)
    
    # Example 1: Single bundle
    print("Example 1: Single Stealth Bundle")
    print("-" * 30)
    
    amount = 1000000  # 1 SOL in lamports
    result = bundler.execute_bundle(token_mint, amount)
    
    print("Result:")
    print(json.dumps(result, indent=2))
    print()
    
    # Example 2: Multiple bundles
    print("Example 2: Multiple Stealth Bundles")
    print("-" * 30)
    
    bundle_count = 3
    results = bundler.execute_multiple_bundles(token_mint, amount, bundle_count)
    
    successful = sum(1 for r in results if r.get('success', False))
    print(f"Executed {len(results)} bundles, {successful} successful")
    print()
    
    # Example 3: Demonstrate stealth features
    print("Example 3: Stealth Features Demonstration")
    print("-" * 30)
    
    stealth_engine = bundler.stealth_engine
    
    # Show user agent rotation
    print("User Agent Rotation:")
    for i in range(3):
        user_agent = stealth_engine.get_random_user_agent()
        print(f"  {i+1}: {user_agent[:50]}...")
    
    # Show stealth headers
    print("\nStealth Headers:")
    headers = stealth_engine.create_stealth_headers()
    for key, value in headers.items():
        print(f"  {key}: {value}")
    
    # Show amount obfuscation
    print("\nAmount Obfuscation:")
    original_amount = 1000000
    for i in range(5):
        obfuscated = stealth_engine.obfuscate_amount(original_amount)
        print(f"  Original: {original_amount} -> Obfuscated: {obfuscated}")
    
    # Show stealth ID generation
    print("\nStealth ID Generation:")
    for i in range(3):
        stealth_id = stealth_engine.generate_stealth_id()
        print(f"  {i+1}: {stealth_id}")
    
    print("\nStealth bundler example completed!")

if __name__ == "__main__":
    main()
