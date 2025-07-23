use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub fee: u64,
    pub timestamp: u64,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub number: u64,
    pub timestamp: u64,
    pub parent_hash: String,
    pub transactions: Vec<Transaction>,
    pub merkle_root: String,
    pub nonce: u64,
    pub difficulty: u32,
    pub hash: String,
}