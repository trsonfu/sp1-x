# SP1 Fibonacci Example

This example demonstrates how to use SP1 to generate zero-knowledge proofs for Fibonacci number computation with various proving modes including network proving and Sepolia testnet.

## 🚀 Quick Start

### 1. Basic Usage (Local Proving)

```bash
cd examples/fibonacci
cargo run
```

### 2. Network Proving (Production)

```bash
# Set your private key for network authentication
export SP1_PRIVATE_KEY="your_private_key_here"
export SP1_PROVER=network

# Run with network proving
cargo run --bin network_custom
```

### 3. Sepolia Network Proving (Testnet)

```bash
# Set your private key for Sepolia network authentication
export SP1_PRIVATE_KEY="your_private_key_here"
export SP1_PROVER=network

# Run with Sepolia network proving
cargo run --features sepolia --bin network_sepolia
```

### 4. Using the Helper Script

```bash
# Run with different options
./run_examples.sh -n 500 -m network          # Compute fibonacci(500) using production network
./run_examples.sh -n 100 -m sepolia          # Compute fibonacci(100) using Sepolia network
./run_examples.sh -n 100 -m compressed       # Compute fibonacci(100) with compressed proof
./run_examples.sh -n 50 -m execute           # Just execute without proof
./run_examples.sh -h                         # Show help
```

## 📋 Available Proving Modes

### 1. Local Proving (Default)
```bash
cargo run --bin fibonacci-script
```

### 2. Network Proving (Production)
```bash
# Basic network proving
cargo run --bin network

# Advanced network proving with custom options
cargo run --bin network_custom
```

### 3. Sepolia Network Proving (Testnet)
```bash
# Sepolia network with auction strategy
cargo run --features sepolia --bin network_sepolia
```

### 4. Compressed Proofs
```bash
cargo run --bin compressed
```

### 5. PLONK Proofs
```bash
cargo run --bin plonk_bn254
```

### 6. Groth16 Proofs
```bash
cargo run --bin groth16_bn254
```

### 7. Execution Only (No Proof)
```bash
cargo run --bin execute
```

## 🌐 Network Configuration

### Production Network (Default)
- **RPC URL**: `https://rpc.production.succinct.xyz`
- **Explorer**: `https://explorer.succinct.xyz`
- **Strategy**: Hosted (default)

### Sepolia Network (Testnet)
- **RPC URL**: `https://rpc.sepolia.succinct.xyz`
- **Explorer**: `https://explorer.sepolia.succinct.xyz`
- **Strategy**: Auction (required)
- **Feature Flag**: `sepolia`

## 🔧 Configuration

### Environment Variables

- `SP1_PROVER=network`: Use network proving instead of local
- `SP1_PRIVATE_KEY=...`: Private key for network authentication
- `RUST_LOG=info`: Enable logging for detailed output

### Command Line Arguments

All binaries accept a single argument for the Fibonacci number:

```bash
cargo run --bin fibonacci-script -- 1500  # Compute fibonacci(1500)
cargo run --bin network_custom -- 2000    # Compute fibonacci(2000) using production network
cargo run --features sepolia --bin network_sepolia -- 100  # Compute fibonacci(100) using Sepolia
```

## 🌐 Network Proving Setup

### 1. Get Private Key

You need a private key to authenticate with the SP1 network. You can:
- Generate one using a wallet
- Use an existing Ethereum private key
- Contact Succinct Labs for testnet credentials

### 2. Set Environment Variables

```bash
export SP1_PRIVATE_KEY="0x1234567890abcdef..."
export SP1_PROVER=network
```

### 3. Choose Network

#### Production Network (Default)
```bash
# Uses hosted strategy
cargo run --bin network_custom
```

#### Sepolia Network (Testnet)
```bash
# Uses auction strategy (required for Sepolia)
cargo run --features sepolia --bin network_sepolia
```

### 4. Check Balance (Optional)

```rust
// In your code, you can check balance
let balance = client.get_balance().await?;
println!("Current balance: {}", balance);
```

## 🏛️ Sepolia Network Features

### Auction Strategy
- **Required**: Sepolia only supports `FulfillmentStrategy::Auction`
- **Auction Parameters**: `min_auction_period`, `max_price_per_pgu`, `whitelist`
- **Provers**: Decentralized auction-based proof generation

### Example Usage
```rust
let proof = client.prove(&pk, &stdin)
    .compressed()
    .strategy(FulfillmentStrategy::Auction)  // Required for Sepolia
    .min_auction_period(30)  // Minimum auction period in seconds
    .max_price_per_pgu(1000)  // Maximum price per proof generation unit
    .run()
    .unwrap();
```

## 📊 Understanding the Output

The Fibonacci program:
1. Reads input `n` from stdin
2. Computes Fibonacci numbers `fib(n-1)` and `fib(n)`
3. Outputs the input `n` and both Fibonacci values

Example output:
```
🌐 Computing fibonacci for n = 100 using Sepolia network
🔍 Requesting proof from Sepolia network...
📋 Network: Sepolia testnet
🏛️  Strategy: Auction
✅ Proof generated successfully from Sepolia network!
📊 Results from Sepolia network:
   Input n: 100
   Fibonacci(n-1): 6875
   Fibonacci(n): 5781
🔍 Verifying proof...
✅ Proof verification successful!
💾 Proof saved to: fibonacci-sepolia-proof-100.bin
🎉 Successfully generated and verified proof for fibonacci(100) using Sepolia network!
🔗 View on explorer: https://explorer.sepolia.succinct.xyz
```

## 🐛 Troubleshooting

### Common Network Errors

1. **RequestUnexecutable**: Program or input has issues
2. **RequestUnfulfillable**: Network capacity or pricing issues
3. **RequestTimedOut**: Request took too long, try increasing timeout
4. **RequestAuctionTimedOut**: No provers bid during auction period (Sepolia only)

### Solutions

```bash
# Increase timeout
export SP1_TIMEOUT=600  # 10 minutes

# Check network status
export RUST_LOG=info
cargo run --bin network_custom

# For Sepolia, try with longer auction period
cargo run --features sepolia --bin network_sepolia
```

## 🔍 Code Structure

```
examples/fibonacci/
├── program/              # The SP1 program (zkVM code)
│   ├── src/main.rs      # Fibonacci computation logic
│   └── Cargo.toml       # Program dependencies
├── script/              # The host code (proof generation)
│   ├── src/main.rs      # Default script
│   ├── bin/             # Different proving modes
│   │   ├── network.rs   # Basic network proving
│   │   ├── network_custom.rs  # Advanced network proving
│   │   ├── network_sepolia.rs # Sepolia network proving
│   │   ├── compressed.rs
│   │   ├── plonk_bn254.rs
│   │   ├── groth16_bn254.rs
│   │   └── execute.rs
│   └── Cargo.toml       # Script dependencies (with sepolia feature)
├── run_examples.sh      # Helper script
└── README.md           # This file
```

## 🚀 SP1 Project Template

For more comprehensive projects, check out the **[sp1-project-template](https://github.com/succinctlabs/sp1-project-template)**:

### What is sp1-project-template?
- **Official template** from Succinct Labs
- **End-to-end SP1 project** setup
- **On-chain verification** with Solidity contracts
- **EVM-compatible proofs** (Groth16, PLONK)
- **Network proving** support
- **Production-ready** structure

### Key Features:
- 📁 **Complete project structure** (program, script, contracts)
- 🔐 **Smart contract verification** 
- 🌐 **Network proving** integration
- ⚡ **EVM-compatible proofs**
- 🛠️ **Build and deployment** scripts

### Getting Started with Template:
```bash
# Clone the template
git clone https://github.com/succinctlabs/sp1-project-template.git
cd sp1-project-template

# Set up environment
cp .env.example .env
# Edit .env with your NETWORK_PRIVATE_KEY

# Build and run
cd script
cargo run --release -- --execute  # Execute only
cargo run --release -- --prove    # Generate proof
cargo run --release --bin evm -- --system groth16  # EVM-compatible proof
```

### Use Cases:
- 🎯 **New SP1 projects** - Start with production-ready template
- 🔗 **On-chain verification** - Deploy and verify proofs on Ethereum
- 🏗️ **Production applications** - Full stack ZK applications
- 📚 **Learning** - Understand complete SP1 workflow

## 🎯 Next Steps

1. **Try different numbers**: Start with small values (< 100) for faster testing
2. **Experiment with networks**: Try both production and Sepolia networks
3. **Explore sp1-project-template**: For full end-to-end projects
4. **Build your own program**: Use this as a template for your own SP1 applications

## 📚 Further Reading

- [SP1 Documentation](https://docs.succinct.xyz/)
- [SP1 SDK Reference](https://docs.succinct.xyz/sdk/overview)
- [Network Proving Guide](https://docs.succinct.xyz/proving/network)
- [SP1 Project Template](https://github.com/succinctlabs/sp1-project-template)
- [SP1 Examples](https://github.com/succinctlabs/sp1/tree/main/examples)

## 🤝 Support

If you encounter issues:
1. Check the [Common Issues](https://docs.succinct.xyz/troubleshooting) guide
2. Join the [SP1 Discord](https://discord.gg/succinct)
3. Open an issue on [GitHub](https://github.com/succinctlabs/sp1/issues) 