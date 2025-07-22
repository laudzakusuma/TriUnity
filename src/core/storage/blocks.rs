use serde::{Deserialize, Serialize};
use crate::core::crypto::QuantumSignature;
use sha3::{Digest, Sha3_256};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: Vec<u8>,      // Sender public key
    pub to: Vec<u8>,        // Recipient public key
    pub amount: u64,        // Amount in smallest unit
    pub fee: u64,           // Transaction fee
    pub nonce: u64,         // Sender nonce
    pub data: Vec<u8>,      // Smart contract data
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