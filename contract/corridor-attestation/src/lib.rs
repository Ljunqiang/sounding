#![no_std]

//! # Corridor Attestation Contract
//!
//! This Soroban smart contract stores immutable corridor integrity attestations
//! for Stellar Pathfinder. Each attestation records the integrity state, verdict,
//! and reference data for a stablecoin→fiat-token corridor at a specific point in time.
//!
//! ## Features
//! - Store corridor attestations on-chain (immutable record)
//! - Query latest attestation for any corridor
//! - Retrieve historical attestations
//! - Provide price oracle functionality for other dApps
//! - Maintain data provenance with attestor addresses
//!
//! ## Attestation Data
//! Each attestation includes:
//! - Corridor identifier (e.g., "USDC→NGNC")
//! - Integrity state (DIRECT, DERIVATIVE, NO-MARKET)
//! - Best verdict from ladder sweep (GOOD, FAIR, POOR, UNUSABLE)
//! - Loss percentage at best rung
//! - Reference mid-market rate
//! - Measurement timestamp
//! - Attestor address (who submitted it)

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec, Symbol, symbol_short};

/// Maximum number of historical attestations to store per corridor
const MAX_HISTORY: u32 = 100;

/// Integrity state of a corridor
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Integrity {
    Direct,       // Independent market exists
    Derivative,   // Routes through another fiat token
    NoMarket,     // No route exists
}

/// Verdict severity grade
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Verdict {
    Good,      // ≤3% loss
    Fair,      // ≤8% loss
    Poor,      // ≤20% loss
    Unusable,  // >20% loss
}

/// On-chain corridor attestation
#[contracttype]
#[derive(Clone, Debug)]
pub struct Attestation {
    pub corridor_id: String,         // e.g., "USDC→NGNC"
    pub integrity: Integrity,        // Structural state
    pub best_verdict: Verdict,       // Grade at recommended size
    pub loss_pct: String,            // Loss percentage as decimal string
    pub reference_mid: String,       // Reference mid-market rate
    pub measured_at: u64,            // Unix timestamp
    pub attestor: Address,           // Who submitted this
    pub tx_hash: String,             // Soroban transaction hash
}

/// Storage keys
#[contracttype]
pub enum DataKey {
    Attestation(String, u64),  // (corridor_id, timestamp) -> Attestation
    LatestTimestamp(String),   // corridor_id -> latest timestamp
    History(String),           // corridor_id -> Vec<u64> of timestamps
}

#[contract]
pub struct CorridorAttestationContract;

#[contractimpl]
impl CorridorAttestationContract {
    /// Submit a new corridor attestation (only callable by authorized attestor)
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `attestor` - Address submitting the attestation
    /// * `corridor_id` - Corridor identifier (e.g., "USDC→NGNC")
    /// * `integrity` - Structural integrity state
    /// * `best_verdict` - Verdict at recommended size
    /// * `loss_pct` - Loss percentage as string
    /// * `reference_mid` - Reference mid-market rate as string
    /// * `measured_at` - Measurement timestamp
    /// * `tx_hash` - Soroban transaction hash
    ///
    /// # Returns
    /// The stored attestation
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
    ) -> Attestation {
        // Require attestor authorization
        attestor.require_auth();

        let attestation = Attestation {
            corridor_id: corridor_id.clone(),
            integrity,
            best_verdict,
            loss_pct,
            reference_mid,
            measured_at,
            attestor: attestor.clone(),
            tx_hash,
        };

        // Store attestation
        let key = DataKey::Attestation(corridor_id.clone(), measured_at);
        env.storage().persistent().set(&key, &attestation);

        // Update latest timestamp
        let latest_key = DataKey::LatestTimestamp(corridor_id.clone());
        env.storage().persistent().set(&latest_key, &measured_at);

        // Add to history (bounded to MAX_HISTORY entries)
        let history_key = DataKey::History(corridor_id.clone());
        let mut history: Vec<u64> = env.storage().persistent()
            .get(&history_key)
            .unwrap_or(Vec::new(&env));

        history.push_back(measured_at);

        // Keep only last MAX_HISTORY entries
        while history.len() > MAX_HISTORY {
            if let Some(oldest) = history.first() {
                let oldest_val = oldest;
                // Remove oldest attestation from storage
                let old_key = DataKey::Attestation(corridor_id.clone(), oldest_val);
                env.storage().persistent().remove(&old_key);
                history.remove(0);
            }
        }

        env.storage().persistent().set(&history_key, &history);

        // Extend TTL for all stored data (30 days)
        env.storage().persistent().extend_ttl(&key, 2_592_000, 2_592_000);
        env.storage().persistent().extend_ttl(&latest_key, 2_592_000, 2_592_000);
        env.storage().persistent().extend_ttl(&history_key, 2_592_000, 2_592_000);

        attestation
    }

    /// Get the latest attestation for a corridor
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `corridor_id` - Corridor identifier
    ///
    /// # Returns
    /// The latest attestation if it exists, None otherwise
    pub fn get_latest(env: Env, corridor_id: String) -> Option<Attestation> {
        let latest_key = DataKey::LatestTimestamp(corridor_id.clone());
        let timestamp: Option<u64> = env.storage().persistent().get(&latest_key);

        timestamp.and_then(|ts| {
            let key = DataKey::Attestation(corridor_id, ts);
            env.storage().persistent().get(&key)
        })
    }

    /// Get historical attestations for a corridor
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `corridor_id` - Corridor identifier
    /// * `limit` - Maximum number of attestations to return (max 100)
    ///
    /// # Returns
    /// Vector of attestations, newest first
    pub fn get_history(env: Env, corridor_id: String, limit: u32) -> Vec<Attestation> {
        let history_key = DataKey::History(corridor_id.clone());
        let timestamps: Vec<u64> = env.storage().persistent()
            .get(&history_key)
            .unwrap_or(Vec::new(&env));

        let mut result = Vec::new(&env);
        let max = limit.min(timestamps.len()).min(MAX_HISTORY);

        // Return newest first (reverse iteration)
        for i in 0..max {
            let idx = timestamps.len() - 1 - i;
            if let Some(ts) = timestamps.get(idx) {
                let key = DataKey::Attestation(corridor_id.clone(), ts);
                if let Some(attestation) = env.storage().persistent().get(&key) {
                    result.push_back(attestation);
                }
            }
        }

        result
    }

    /// Get the count of attestations for a corridor
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `corridor_id` - Corridor identifier
    ///
    /// # Returns
    /// Number of stored attestations
    pub fn get_count(env: Env, corridor_id: String) -> u32 {
        let history_key = DataKey::History(corridor_id);
        let timestamps: Vec<u64> = env.storage().persistent()
            .get(&history_key)
            .unwrap_or(Vec::new(&env));
        timestamps.len()
    }

    /// Check if a corridor has any attestations
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `corridor_id` - Corridor identifier
    ///
    /// # Returns
    /// true if attestations exist, false otherwise
    pub fn has_attestations(env: Env, corridor_id: String) -> bool {
        let latest_key = DataKey::LatestTimestamp(corridor_id);
        env.storage().persistent().has(&latest_key)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_attest_and_retrieve() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CorridorAttestationContract);
        let client = CorridorAttestationContractClient::new(&env, &contract_id);

        let attestor = Address::generate(&env);
        let corridor_id = String::from_str(&env, "USDC→NGNC");
        let integrity = Integrity::Direct;
        let verdict = Verdict::Poor;
        let loss_pct = String::from_str(&env, "25.02");
        let reference_mid = String::from_str(&env, "1364.50");
        let measured_at = 1704067200; // 2024-01-01 00:00:00 UTC
        let tx_hash = String::from_str(&env, "abc123");

        // Submit attestation
        let attestation = client.attest(
            &attestor,
            &corridor_id,
            &integrity,
            &verdict,
            &loss_pct,
            &reference_mid,
            &measured_at,
            &tx_hash,
        );

        assert_eq!(attestation.corridor_id, corridor_id);
        assert_eq!(attestation.integrity, Integrity::Direct);
        assert_eq!(attestation.best_verdict, Verdict::Poor);

        // Retrieve latest
        let latest = client.get_latest(&corridor_id);
        assert!(latest.is_some());
        assert_eq!(latest.unwrap().loss_pct, loss_pct);

        // Check count
        let count = client.get_count(&corridor_id);
        assert_eq!(count, 1);

        // Check has_attestations
        assert!(client.has_attestations(&corridor_id));
    }

    #[test]
    fn test_history_with_multiple_attestations() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CorridorAttestationContract);
        let client = CorridorAttestationContractClient::new(&env, &contract_id);

        let attestor = Address::generate(&env);
        let corridor_id = String::from_str(&env, "USDC→GHSC");

        // Add 3 attestations
        for i in 0..3 {
            client.attest(
                &attestor,
                &corridor_id,
                &Integrity::Derivative,
                &Verdict::Unusable,
                &String::from_str(&env, "75.00"),
                &String::from_str(&env, "1.50"),
                &(1704067200 + i * 3600),
                &String::from_str(&env, "tx_hash"),
            );
        }

        let count = client.get_count(&corridor_id);
        assert_eq!(count, 3);

        let history = client.get_history(&corridor_id, &10);
        assert_eq!(history.len(), 3);

        // Should be newest first
        assert!(history.get(0).unwrap().measured_at > history.get(1).unwrap().measured_at);
    }

    #[test]
    fn test_bounded_history() {
        let env = Env::default();
        env.budget().reset_unlimited();
        let contract_id = env.register_contract(None, CorridorAttestationContract);
        let client = CorridorAttestationContractClient::new(&env, &contract_id);

        let attestor = Address::generate(&env);
        let corridor_id = String::from_str(&env, "USDC→NGNC");

        // Add MAX_HISTORY + 10 attestations
        for i in 0..(MAX_HISTORY + 10) {
            client.attest(
                &attestor,
                &corridor_id,
                &Integrity::Direct,
                &Verdict::Good,
                &String::from_str(&env, "2.50"),
                &String::from_str(&env, "1364.50"),
                &(1704067200 + (i as u64) * 3600),
                &String::from_str(&env, "tx_hash"),
            );
        }

        // Should keep only MAX_HISTORY entries
        let count = client.get_count(&corridor_id);
        assert_eq!(count, MAX_HISTORY);
    }
}
