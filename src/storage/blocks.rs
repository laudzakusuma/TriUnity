//! 📦 Block structure and validation

use serde::{Deserialize, Serialize};
use crate::core::crypto::QuantumSignature;
use sha3::{Digest, Sha3_256};

/// 📦 Blockchain block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

/// 📋 Block header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub version: u32,
    pub previous_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u64,
    pub height: u64,
    pub consensus_data: ConsensusData,
}

/// 🤖 Consensus data variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusData {
    FastLane { 
        validator: Vec<u8> 
    },
    SecureLane { 
        validators: Vec<Vec<u8>> 
    },
    HybridPath { 
        fast_validators: Vec<Vec<u8>>, 
        secure_validators: Vec<Vec<u8>> 
    },
    Emergency { 
        authority_validators: Vec<Vec<u8>> 
    },
}

/// 💳 Transaction structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: Vec<u8>,      // Sender public key
    pub to: Vec<u8>,        // Recipient public key
    pub amount: u64,        // Amount in smallest unit
    pub fee: u64,           // Transaction fee
    pub nonce: u64,         // Sender nonce
    pub data: Vec<u8>,      // Smart contract data
    pub signature: QuantumSignature, // Quantum-safe signature
}

impl Block {
    /// 🆕 Create new block
    pub fn new(
        previous_hash: [u8; 32],
        transactions: Vec<Transaction>,
        height: u64,
        consensus_data: ConsensusData,
    ) -> Self {
        let merkle_root = Self::calculate_merkle_root(&transactions);
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let header = BlockHeader {
            version: 1,
            previous_hash,
            merkle_root,
            timestamp,
            height,
            consensus_data,
        };

        Self {
            header,
            transactions,
        }
    }

    /// 🏗️ Calculate merkle root from transactions
    fn calculate_merkle_root(transactions: &[Transaction]) -> [u8; 32] {
        if transactions.is_empty() {
            return [0; 32];
        }

        // Hash each transaction
        let mut hashes: Vec<[u8; 32]> = transactions
            .iter()
            .map(|tx| {
                let tx_bytes = bincode::serialize(tx).unwrap_or_default();
                let mut hasher = Sha3_256::new();
                hasher.update(&tx_bytes);
                hasher.finalize().into()
            })
            .collect();

        // Build merkle tree
        while hashes.len() > 1 {
            let mut next_level = Vec::new();
            
            for chunk in hashes.chunks(2) {
                let mut hasher = Sha3_256::new();
                hasher.update(&chunk[0]);
                if chunk.len() > 1 {
                    hasher.update(&chunk[1]);
                } else {
                    hasher.update(&chunk[0]); // Duplicate if odd number
                }
                next_level.push(hasher.finalize().into());
            }
            
            hashes = next_level;
        }

        hashes[0]
    }

    /// 🔍 Get block hash
    pub fn hash(&self) -> [u8; 32] {
        let header_bytes = bincode::serialize(&self.header).unwrap_or_default();
        let mut hasher = Sha3_256::new();
        hasher.update(&header_bytes);
        hasher.finalize().into()
    }

    /// ✅ Validate block structure
    pub fn validate(&self) -> bool {
        // Check basic structure
        if self.header.version == 0 {
            return false;
        }

        // Verify merkle root
        let calculated_root = Self::calculate_merkle_root(&self.transactions);
        if calculated_root != self.header.merkle_root {
            return false;
        }

        // Validate all transactions
        for transaction in &self.transactions {
            if !transaction.validate() {
                return false;
            }
        }

        true
    }

    /// 📏 Get block size in bytes
    pub fn size(&self) -> usize {
        bincode::serialize(self).map(|data| data.len()).unwrap_or(0)
    }

    /// 📊 Get transaction count
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }

    /// 💰 Calculate total fees
    pub fn total_fees(&self) -> u64 {
        self.transactions.iter().map(|tx| tx.fee).sum()
    }

    /// 💸 Calculate total amount transferred
    pub fn total_amount(&self) -> u64 {
        self.transactions.iter().map(|tx| tx.amount).sum()
    }
}

impl Transaction {
    /// 🆕 Create new transaction
    pub fn new(
        from: Vec<u8>,
        to: Vec<u8>,
        amount: u64,
        fee: u64,
        nonce: u64,
        data: Vec<u8>,
        signature: QuantumSignature,
    ) -> Self {
        Self {
            from,
            to,
            amount,
            fee,
            nonce,
            data,
            signature,
        }
    }

    /// ✅ Validate transaction
    pub fn validate(&self) -> bool {
        // Check basic fields
        if self.from.is_empty() || self.to.is_empty() {
            return false;
        }

        if self.amount == 0 && self.data.is_empty() {
            return false; // Either transfer value or carry data
        }

        // Verify signature
        let tx_data = self.get_signing_data();
        self.signature.verify(&tx_data, &self.from)
    }

    /// 📝 Get data for signing
    pub fn get_signing_data(&self) -> Vec<u8> {
        let signing_tx = (
            &self.from,
            &self.to,
            self.amount,
            self.fee,
            self.nonce,
            &self.data,
        );
        bincode::serialize(&signing_tx).unwrap_or_default()
    }

    /// 🔍 Get transaction hash
    pub fn hash(&self) -> [u8; 32] {
        let tx_bytes = bincode::serialize(self).unwrap_or_default();
        let mut hasher = Sha3_256::new();
        hasher.update(&tx_bytes);
        hasher.finalize().into()
    }

    /// 📏 Get transaction size
    pub fn size(&self) -> usize {
        bincode::serialize(self).map(|data| data.len()).unwrap_or(0)
    }

    /// 💳 Check if this is a smart contract transaction
    pub fn is_contract_call(&self) -> bool {
        !self.data.is_empty()
    }

    /// 💰 Check if this is a value transfer
    pub fn is_transfer(&self) -> bool {
        self.amount > 0
    }
}

impl Default for ConsensusData {
    fn default() -> Self {
        Self::FastLane {
            validator: vec![0; 32], // Default validator
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::crypto::QuantumKeyPair;

    fn create_test_transaction() -> Transaction {
        let keypair = QuantumKeyPair::generate();
        let signing_data = (
            keypair.public_key(),
            &vec![9, 8, 7, 6], // to
            1000u64,           // amount
            10u64,             // fee
            1u64,              // nonce
            &Vec::<u8>::new(), // data
        );
        let tx_bytes = bincode::serialize(&signing_data).unwrap();
        let signature = keypair.sign(&tx_bytes).unwrap();

        Transaction::new(
            keypair.public_key().to_vec(),
            vec![9, 8, 7, 6],
            1000,
            10,
            1,
            Vec::new(),
            signature,
        )
    }

    #[test]
    fn test_block_creation() {
        let transactions = vec![
            create_test_transaction(),
            create_test_transaction(),
        ];

        let block = Block::new(
            [1; 32], // previous hash
            transactions.clone(),
            1,       // height
            ConsensusData::FastLane { 
                validator: vec![1, 2, 3, 4] 
            },
        );

        assert_eq!(block.header.height, 1);
        assert_eq!(block.transaction_count(), 2);
        assert_eq!(block.total_amount(), 2000);
        assert_eq!(block.total_fees(), 20);

        println!("📦 Block creation working!");
        println!("   Height: {}", block.header.height);
        println!("   Transactions: {}", block.transaction_count());
        println!("   Total amount: {}", block.total_amount());
        println!("   Total fees: {}", block.total_fees());
        println!("   Block size: {} bytes", block.size());
    }

    #[test]
    fn test_consensus_data() {
        let fast_lane = ConsensusData::FastLane { validator: vec![1, 2, 3] };
        let secure_lane = ConsensusData::SecureLane { validators: vec![vec![1], vec![2]] };
        let emergency = ConsensusData::Emergency { authority_validators: vec![vec![9]] };
        
        println!("🤖 Consensus data types working!");
        println!("   FastLane: {:?}", fast_lane);
        println!("   SecureLane: {:?}", secure_lane);
        println!("   Emergency: {:?}", emergency);
    }

    #[test]
    fn test_block_validation() {
        let transactions = vec![create_test_transaction()];
        
        let block = Block::new(
            [0; 32],
            transactions,
            1,
            ConsensusData::default(),
        );

        assert!(block.validate());
        println!("✅ Block validation working!");
    }
}