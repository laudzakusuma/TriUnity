use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
// If you still have `unresolved import` errors, ensure your mod.rs files are set up correctly.
use crate::core::crypto::QuantumSignature;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BlockHeader {
    pub height: u64, // The correct field name is 'height'
    pub timestamp: u64,
    pub nonce: u64,
    pub previous_hash: String,
    pub merkle_root: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<String>, // Or a proper Transaction type
    pub hash: String,
    pub signature: QuantumSignature,
}

impl Block {
    // Example function to calculate the hash from the header
    pub fn calculate_hash(&self) -> String {
        // Using serde_json is simple but might not be the most performant.
        // For production, a more direct serialization might be better.
        let header_data = serde_json::to_string(&self.header).unwrap_or_default();
        let mut hasher = Sha3_256::new();
        hasher.update(header_data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}