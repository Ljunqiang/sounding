# Soroban Smart Contract Integration

**On-Chain Corridor Integrity Attestations for Stellar Pathfinder**

## Overview

Stellar Pathfinder integrates with Soroban smart contracts to store corridor integrity attestations on-chain. This creates an immutable, verifiable record that other dApps can consume as a price oracle.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   Stellar Pathfinder                         │
│                                                              │
│  ┌──────────────┐      ┌──────────────┐                    │
│  │   Horizon    │──────│  Measurement │                    │
│  │  Pathfinding │      │    Engine    │                    │
│  └──────────────┘      └───────┬──────┘                    │
│                                │                            │
│                                ▼                            │
│                         ┌──────────────┐                    │
│                         │  Attestation │                    │
│                         │   Package    │                    │
│                         └───────┬──────┘                    │
│                                │                            │
└────────────────────────────────┼────────────────────────────┘
                                 │
                                 ▼
                    ┌────────────────────────┐
                    │   Soroban Contract     │
                    │  (corridor-attestation)│
                    │                        │
                    │  ✓ Store attestations  │
                    │  ✓ Query by corridor   │
                    │  ✓ Historical data     │
                    │  ✓ Price oracle API    │
                    └────────────────────────┘
                                 │
                                 ▼
                         ┌──────────────┐
                         │  Other dApps │
                         │ (consumers)  │
                         └──────────────┘
```

## Features

### ✅ **Immutable Attestations**
- Once written, corridor integrity scores cannot be modified
- Full audit trail with timestamp and attestor address
- Cryptographically signed with Stellar keys

### ✅ **Price Oracle**
- Other dApps can query latest corridor integrity
- Historical attestations for trend analysis
- No trust assumptions - data is on-chain

### ✅ **Verifiable**
- Each attestation includes Soroban transaction hash
- Viewable on Stellar Explorer
- "Verified on Stellar" badge in UI with direct links

### ✅ **Efficient Storage**
- Bounded history (100 attestations per corridor)
- Automatic TTL management (30-day retention)
- ~0.01 XLM per attestation

## Contract Data Structure

### Attestation

```rust
pub struct Attestation {
    corridor_id: String,        // e.g., "USDC→NGNC"
    integrity: Integrity,       // DIRECT, DERIVATIVE, or NO-MARKET
    best_verdict: Verdict,      // GOOD, FAIR, POOR, or UNUSABLE
    loss_pct: String,           // Loss % as decimal string
    reference_mid: String,       // Mid-market rate used
    measured_at: u64,           // Unix timestamp
    attestor: Address,          // Who submitted this
    tx_hash: String,            // Soroban transaction hash
}
```

### Integrity States

- **DIRECT**: Independent market exists (at least one path without fiat intermediaries)
- **DERIVATIVE**: Every path routes through another fiat token
- **NO-MARKET**: No route exists at any tested size

### Verdict Grades

- **GOOD**: ≤3% loss (comparable to competitive remittance)
- **FAIR**: ≤8% loss (worse than best, not unreasonable)
- **POOR**: ≤20% loss (expensive, told plainly)
- **UNUSABLE**: >20% loss (value destruction, no recommendation)

## Deployment

### Prerequisites

```bash
# Install Soroban CLI
cargo install --locked soroban-cli

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Rust toolchain (1.70+)
rustup update
```

### Deploy to Testnet

```bash
# Run deployment script
cd contract
./deploy-testnet.sh
```

The script will:
1. Build and optimize the contract
2. Configure Stellar testnet
3. Generate attestor identity
4. Fund from friendbot
5. Deploy contract
6. Save contract ID to `contract-id-testnet.txt`

### Manual Deployment

```bash
cd contract/corridor-attestation

# Build contract
cargo build --target wasm32-unknown-unknown --release

# Optimize WASM
soroban contract optimize \
  --wasm target/wasm32-unknown-unknown/release/corridor_attestation.wasm

# Configure network
soroban config network add testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"

# Generate identity
soroban config identity generate attestor

# Fund from friendbot
soroban config identity address attestor | \
  xargs -I {} curl "https://friendbot.stellar.org/?addr={}"

# Deploy
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/corridor_attestation.wasm \
  --source attestor \
  --network testnet
```

## Integration

### Go Backend

The `attestation` package provides integration:

```go
import "github.com/Stellar-Pathfinder/stellar-pathfinder/attestation"

// Create client
client := attestation.NewClient(contractID, "testnet", attestorAddress)

// Build attestation from measurement
att, err := attestation.AttestationFromMeasurement(corridorID, measurementData)
if err != nil {
    return err
}

// Submit to Soroban (when RPC integration is complete)
txHash, err := client.Submit(ctx, att)
```

### API Endpoints

**GET /api/attestations/:corridor**

Returns historical attestations for a corridor.

Query parameters:
- `limit`: max attestations to return (default: 10, max: 100)

```json
{
  "attestations": [
    {
      "corridor_id": "USDC→NGNC",
      "integrity": "DIRECT",
      "best_verdict": "POOR",
      "loss_pct": "25.02",
      "reference_mid": "1364.50",
      "measured_at": "2024-01-01T00:00:00Z",
      "attestor": "GAB...",
      "tx_hash": "abc123...",
      "contract_id": "C..."
    }
  ],
  "count": 1,
  "contract_id": "C..."
}
```

**GET /api/attestations/:corridor/latest**

Returns the most recent attestation.

### Frontend Integration

The UI automatically:
- Checks for attestations when displaying measurements
- Shows "Verified on Stellar" badge if attestations exist
- Links to Stellar Explorer for verification
- Displays badge with gradient styling and checkmark

## Testing Contract

```bash
cd contract/corridor-attestation

# Run unit tests
cargo test

# Test deployment
./deploy-testnet.sh

# Invoke contract
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source attestor \
  --network testnet \
  -- \
  get_latest \
  --corridor_id "USDC→NGNC"
```

## Use Cases

### 1. **Verifiable Corridor Health**
Other wallets/dApps can query corridor integrity before routing payments.

### 2. **Historical Analysis**
Track corridor degradation over time with immutable on-chain records.

### 3. **Price Oracle**
Use as a trustless data source for DeFi protocols needing corridor pricing.

### 4. **Audit Trail**
Complete provenance of all measurements with attestor signatures.

### 5. **Cross-App Integration**
Multiple apps can consume same on-chain data without trusting Stellar Pathfinder.

## Future Enhancements

### Phase 1 (Current)
- ✅ Basic attestation storage
- ✅ Query latest/history
- ✅ UI verification badges
- ✅ API endpoints

### Phase 2 (Planned)
- ⏳ Full Soroban RPC integration in Go backend
- ⏳ Automatic attestation on every measurement
- ⏳ Mainnet deployment
- ⏳ Contract events for real-time notifications

### Phase 3 (Future)
- ⏳ Multi-attestor consensus (require multiple sources)
- ⏳ Slashing conditions for incorrect attestations
- ⏳ Governance token for attestor selection
- ⏳ Cross-chain bridge attestations

## Contract Address

**Testnet**: `CDZPZIO6JB2WZKDYKNVVJZZA2YZFLODNNX2YF5F3CCOXJZHMAOEFHYQU`
- **Attestor**: `GCDD4SUD44MMLDBQM2BGIO674XFKD5UFQ4LYMY3RLAPYMC6BDJBUZYG2`
- **Explorer**: https://stellar.expert/explorer/testnet/contract/CDZPZIO6JB2WZKDYKNVVJZZA2YZFLODNNX2YF5F3CCOXJZHMAOEFHYQU
- **Lab**: https://lab.stellar.org/r/testnet/contract/CDZPZIO6JB2WZKDYKNVVJZZA2YZFLODNNX2YF5F3CCOXJZHMAOEFHYQU

**Mainnet**: `<To be deployed>`

## Security

- **Authorization**: Only authorized attestors can submit
- **Immutability**: Stored attestations cannot be modified
- **TTL Management**: 30-day retention with auto-extension
- **Bounded Storage**: Max 100 attestations per corridor prevents unbounded growth

## Cost Analysis

| Operation | Estimated Cost |
|-----------|---------------|
| Submit attestation | ~0.01 XLM |
| Query latest | Free (read-only) |
| Query history | Free (read-only) |
| 100 attestations | ~1 XLM total |

## Support

- **Contract Code**: `contract/corridor-attestation/`
- **Deployment Script**: `contract/deploy-testnet.sh`
- **Go Package**: `attestation/`
- **API Implementation**: `server/attestations.go`

## License

Apache-2.0
