#!/bin/bash

# SP1 Fibonacci Examples Runner
# This script helps you run different fibonacci examples with various options

echo "🚀 SP1 Fibonacci Examples Runner"
echo "================================"

# Default values
N=1000
MODE="local"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -n|--number)
            N="$2"
            shift 2
            ;;
        -m|--mode)
            MODE="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: $0 [options]"
            echo "Options:"
            echo "  -n, --number <num>    Fibonacci number to compute (default: 1000)"
            echo "  -m, --mode <mode>     Proving mode: local, network, sepolia, compressed, plonk, groth16, execute (default: local)"
            echo "  -h, --help           Show this help message"
            echo ""
            echo "Environment variables:"
            echo "  SP1_PROVER=network   Use network proving"
            echo "  SP1_PRIVATE_KEY=...  Private key for network authentication"
            echo ""
            echo "Examples:"
            echo "  $0 -n 500 -m network          # Compute fibonacci(500) using network"
            echo "  $0 -n 100 -m sepolia          # Compute fibonacci(100) using Sepolia network"
            echo "  $0 -n 100 -m compressed       # Compute fibonacci(100) with compressed proof"
            echo "  $0 -n 50 -m execute           # Just execute without proof"
            echo ""
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

echo "📝 Configuration:"
echo "   Number: $N"
echo "   Mode: $MODE"
echo ""

# Change to script directory
cd "$(dirname "$0")/script"

# Run based on mode
case $MODE in
    local)
        echo "🏠 Running local proof generation..."
        cargo run --bin fibonacci-script -- $N
        ;;
    network)
        echo "🌐 Running network proof generation..."
        if [ -z "$SP1_PRIVATE_KEY" ]; then
            echo "⚠️  Warning: SP1_PRIVATE_KEY not set. Make sure to set it for network proving."
        fi
        export SP1_PROVER=network
        cargo run --bin network_custom -- $N
        ;;
    sepolia)
        echo "🌐 Running Sepolia network proof generation..."
        if [ -z "$SP1_PRIVATE_KEY" ]; then
            echo "⚠️  Warning: SP1_PRIVATE_KEY not set. Make sure to set it for Sepolia network proving."
        fi
        export SP1_PROVER=network
        cargo run --features sepolia --bin network_sepolia -- $N
        ;;
    compressed)
        echo "🗜️  Running compressed proof generation..."
        cargo run --bin compressed -- $N
        ;;
    plonk)
        echo "🔐 Running PLONK proof generation..."
        cargo run --bin plonk_bn254 -- $N
        ;;
    groth16)
        echo "🔒 Running Groth16 proof generation..."
        cargo run --bin groth16_bn254 -- $N
        ;;
    execute)
        echo "⚡ Running execution only (no proof)..."
        cargo run --bin execute -- $N
        ;;
    *)
        echo "❌ Unknown mode: $MODE"
        echo "Available modes: local, network, sepolia, compressed, plonk, groth16, execute"
        exit 1
        ;;
esac

echo ""
echo "✅ Done!" 