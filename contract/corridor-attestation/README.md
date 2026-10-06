# Corridor Attestation Soroban Contract

**On-chain corridor integrity attestations for Stellar Pathfinder**

This Soroban smart contract stores immutable corridor integrity data on the Stellar blockchain, creating a verifiable price oracle that other dApps can consume.

## Overview

Each time Stellar Pathfinder measures a corridor, it can submit an attestation to this contract containing:

- **Corridor ID**: e.g., "USDC→NGNC"
- **Integrity State**: DIRECT, DERIVATIVE, or NO-MARKET
- **Best Verdict**: GOOD, FAIR, POOR, or UNUSABLE
- **Loss Percentage**: Loss vs mid-market rate at best rung
- **Reference Mid**: Independent mid-market rate used for scoring
- **Timestamp**: When the measurement was taken
- **Attestor Address**: Who submitted the attestation
- **Transaction Hash**: Soroban tx for verification

## Features

✅ **Immutable Record** - Once written, attestations cannot be modified
✅ **Bounded History** - Stores last 100 attestations per corridor
✅ **Price Oracle** - Other dApps can query latest corridor integrity
✅ **Provenance** - Each attestation includes attestor address
✅ **Time-Series Data** - Historical attestations for trend analysis
✅ **TTL Management** - Automatic 30-day data retention extension

## Contract Functions

### `attest`
Submit a new corridor attestation (requires attestor authorization).

```rust
pub fn attest(
    env: Env,
    attestor: Address,
    corridor_id: String,
    integrity: Integrity,
    best_verdict: Verdict,
    loss_pct: String,
    reference_mid: String,
    measured_at: u64,
    tx_hash: String,
) -> Attestation
```

### `get_latest`
Retrieve the most recent attestation for a corridor.

```rust
pub fn get_latest(
    env: Env,
    corridor_id: String,
) -> Option<Attestation>
```

### `get_history`
Get historical attestations (newest first, max 100).

```rust
pub fn get_history(
    env: Env,
    corridor_id: String,
    limit: u32,
) -> Vec<Attestation>
```

### `get_count`
Count total attestations for a corridor.

```rust
pub fn get_count(
    env: Env,
    corridor_id: String,
) -> u32
```

### `has_attestations`
Check if any attestations exist for a corridor.

```rust
pub fn has_attestations(
    env: Env,
    corridor_id: String,
) -> bool
```

## Building the Contract

### Prerequisites
- Rust toolchain (1.70+)
- Soroban CLI
- `wasm32-unknown-unknown` target

### Build Steps

```bash
# Install Soroban CLI
cargo install --locked soroban-cli

# Add WASM target
rustup target add wasm32-unknown-unknown

# Build contract
cd contract/corridor-attestation
cargo build --target wasm32-unknown-unknown --release

# Optimize WASM
soroban contract optimize \
  --wasm target/wasm32-unknown-unknown/release/corridor_attestation.wasm
```

## Deployment

### Testnet Deployment

```bash
# Configure testnet network
soroban config network add testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"

# Generate attestor identity
soroban config identity generate attestor

# Fund attestor (from friendbot)
soroban config identity address attestor | \
  xargs -I {} curl "https://friendbot.stellar.org/?addr={}"

# Deploy contract
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/corridor_attestation.wasm \
  --source attestor \
  --network testnet
```

### Mainnet Deployment

```bash
# Configure mainnet network
soroban config network add mainnet \
  --rpc-url https://soroban-rpc.mainnet.stellar.org:443 \
  --network-passphrase "Public Global Stellar Network ; September 2015"

# Deploy with production key
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/corridor_attestation.wasm \
  --source production-key \
  --network mainnet
```

## Integration with Stellar Pathfinder

Stellar Pathfinder's Go backend will:

1. Measure a corridor using Horizon pathfinding
2. Calculate integrity state and verdicts
3. Submit attestation to Soroban contract
4. Display "Verified on Stellar" badge in UI
5. Provide API endpoint to query on-chain attestations

## Usage Examples

### Reading Latest Attestation (CLI)

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source reader \
  --network testnet \
  -- \
  get_latest \
  --corridor_id "USDC→NGNC"
```

### Submitting Attestation (CLI)

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source attestor \
  --network testnet \
  -- \
  attest \
  --attestor <ATTESTOR_ADDRESS> \
  --corridor_id "USDC→NGNC" \
  --integrity Direct \
  --best_verdict Poor \
  --loss_pct "25.02" \
  --reference_mid "1364.50" \
  --measured_at 1704067200 \
  --tx_hash "abc123def456"
```

### JavaScript Integration (Frontend)

```javascript
import * as StellarSDK from '@stellar/stellar-sdk';

const server = new StellarSDK.SorobanRpc.Server(
  'https://soroban-testnet.stellar.org:443'
);

const contractId = 'C...'; // Contract ID from deployment

// Read latest attestation
const contract = new StellarSDK.Contract(contractId);
const operation = contract.call(
  'get_latest',
  StellarSDK.xdr.ScVal.scvString('USDC→NGNC')
);

// Build and submit transaction
const account = await server.getAccount(userPublicKey);
const transaction = new StellarSDK.TransactionBuilder(account, {
  fee: '100',
  networkPassphrase: StellarSDK.Networks.TESTNET
})
  .addOperation(operation)
  .setTimeout(30)
  .build();

const result = await server.sendTransaction(transaction);
```

## Price Oracle Use Cases

Other dApps can use this contract as a price oracle to:

1. **Verify corridor integrity** before routing payments
2. **Display corridor health** in wallet interfaces
3. **Trigger alerts** when corridors become unusable
4. **Build analytics** from historical attestations
5. **Create corridor indices** aggregating multiple corridors

## Storage & Costs

- **Per Attestation**: ~200 bytes on-chain
- **History Limit**: 100 attestations per corridor
- **TTL**: 30 days, auto-extended on each query
- **Estimated Cost**: ~0.01 XLM per attestation submission

## Testing

Run contract tests:

```bash
cargo test
```

Expected output:
```
running 3 tests
test test::test_attest_and_retrieve ... ok
test test::test_history_with_multiple_attestations ... ok
test test::test_bounded_history ... ok

test result: ok. 3 passed; 0 failed; 0 ignored
```

## Security Considerations

1. **Authorization**: Only authorized attestors can submit attestations
2. **Immutability**: Attestations cannot be modified after submission
3. **Bounded Storage**: History limited to 100 entries to prevent unbounded growth
4. **TTL Management**: Data automatically extends lifetime on access
5. **Provenance**: Each attestation signed by attestor address

## License

Apache-2.0

## Support

- GitHub Issues: https://github.com/Stellar-Pathfinder/stellar-pathfinder/issues
- Documentation: https://github.com/Stellar-Pathfinder/stellar-pathfinder/tree/main/docs
