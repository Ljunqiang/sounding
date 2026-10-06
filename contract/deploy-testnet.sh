#!/bin/bash
set -e

# Soroban Contract Deployment Script for Testnet
# This script builds, optimizes, and deploys the corridor attestation contract to Stellar testnet

echo "🚀 Deploying Corridor Attestation Contract to Stellar Testnet"
echo "=============================================================="

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check prerequisites
echo -e "${BLUE}Checking prerequisites...${NC}"
if ! command -v soroban &> /dev/null; then
    echo "❌ Soroban CLI not found. Install with:"
    echo "   cargo install --locked soroban-cli"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Install Rust from https://rustup.rs/"
    exit 1
fi

# Ensure wasm32 target is installed
echo -e "${BLUE}Ensuring wasm32-unknown-unknown target...${NC}"
rustup target add wasm32-unknown-unknown

# Navigate to contract directory
cd "$(dirname "$0")/corridor-attestation"

# Build contract
echo -e "${BLUE}Building contract...${NC}"
cargo build --target wasm32-unknown-unknown --release
echo -e "${GREEN}✓ Contract built${NC}"

# Optimize WASM
echo -e "${BLUE}Optimizing WASM...${NC}"
soroban contract optimize \
  --wasm target/wasm32-unknown-unknown/release/corridor_attestation.wasm
echo -e "${GREEN}✓ WASM optimized${NC}"

# Configure testnet if not already configured
echo -e "${BLUE}Configuring Stellar testnet...${NC}"
soroban config network add testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015" \
  2>/dev/null || echo "Testnet already configured"

# Generate attestor identity if it doesn't exist
if ! soroban config identity show attestor &> /dev/null; then
    echo -e "${BLUE}Generating attestor identity...${NC}"
    soroban config identity generate attestor
    echo -e "${GREEN}✓ Attestor identity generated${NC}"
fi

# Get attestor address
ATTESTOR_ADDRESS=$(soroban config identity address attestor)
echo -e "${YELLOW}Attestor address: ${ATTESTOR_ADDRESS}${NC}"

# Fund attestor from friendbot
echo -e "${BLUE}Funding attestor from friendbot...${NC}"
curl -s "https://friendbot.stellar.org/?addr=${ATTESTOR_ADDRESS}" > /dev/null
echo -e "${GREEN}✓ Attestor funded${NC}"

# Deploy contract
echo -e "${BLUE}Deploying contract to testnet...${NC}"
CONTRACT_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/corridor_attestation.wasm \
  --source attestor \
  --network testnet)

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}✅ Contract deployed successfully!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "Contract ID: ${YELLOW}${CONTRACT_ID}${NC}"
echo -e "Network: ${YELLOW}Testnet${NC}"
echo -e "Attestor: ${YELLOW}${ATTESTOR_ADDRESS}${NC}"
echo ""
echo "Save this contract ID for integration with Stellar Pathfinder!"
echo ""
echo "Test the contract:"
echo -e "${BLUE}soroban contract invoke \\
  --id ${CONTRACT_ID} \\
  --source attestor \\
  --network testnet \\
  -- \\
  has_attestations \\
  --corridor_id \"USDC→NGNC\"${NC}"

# Save contract ID to file
echo "${CONTRACT_ID}" > contract-id-testnet.txt
echo ""
echo -e "Contract ID saved to: ${YELLOW}contract-id-testnet.txt${NC}"
