//! 💾 Simple database layer using Sled
//! 
//! High-performance pure Rust database

use sled::{Db, Tree};
use serde::{Deserialize, Serialize};
use crate::{Result, TriUnityError};
use crate::core::storage::Block;

/// 💾 Simple blockchain database
#[derive(Debug, Clone)]
pub struct BlockchainDB {
    db: Db,
    blocks: Tree,
    state: Tree,
}

impl BlockchainDB {
    /// 🆕 Create new database
    pub fn new(path: &str) -> Result<Self> {
        let db = sled::open(path)
            .map_err(|e| TriUnityError::StorageError(e.to_string()))?;
        
        let blocks = db.open_tree("blocks")
            .map_err(|e| TriUnityError::StorageError(e.to_string()))?;
        
        let state = db.open_tree("state")
            .map_err(|e| TriUnityError::StorageError(e.to_string()))?;
        
        Ok(Self { db, blocks, state })
    }

    /// 💾 Store block
    pub fn store_block(&self, block: &Block) -> Result<()> {
        let key = block.header.height.to_be_bytes();
        let value = bincode::serialize(block)
            .map_err(|e| TriUnityError::SerializationError(e))?;
        
        self.blocks.insert(key, value)
            .map_err(|e| TriUnityError::StorageError(e.to_string()))?;
        
        self.blocks.flush()
            .map_err(|e| TriUnityError::StorageError(e.to_string()))?;
        
        Ok(())
    }

    /// 📖 Get block by height
    pub fn get_block(&self, height: u64) -> Result<Option<Block>> {
        let key = height.to_be_bytes();
        
        if let Some(value) = self.blocks.get(key)
            .map_err(|e| TriUnityError::StorageError(e.to_string()))? {
            
            let block: Block = bincode::deserialize(&value)
                .map_err(|e| TriUnityError::SerializationError(e))?;
            
            Ok(Some(block))
        } else {
            Ok(None)
        }
    }

    /// 📏 Get latest block height
    pub fn get_latest_height(&self) -> Result<u64> {
        if let Some((key, _)) = self.blocks.last()
            .map_err(|e| TriUnityError::StorageError(e.to_string()))? {
            
            let height = u64::from_be_bytes(
                key[..8].try_into()
                    .map_err(|_| TriUnityError::StorageError("Invalid height key".to_string()))?
            );
            
            Ok(height)
        } else {
            Ok(0)
        }
    }

    /// 💾 Store state data
    pub fn store_state(&self, key: &str, value: &[u8]) -> Result<()> {
        self.state.insert(key.as_bytes(), value)
            .map_err(|e| TriUnityError::StorageError(e.to_string()))?;
        
        Ok(())
    }

    /// 📖 Get state data
    pub fn get_state(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let value = self.state.get(key.as_bytes())
            .map_err(|e| TriUnityError::StorageError(e.to_string()))?;
        
        Ok(value.map(|v| v.to_vec()))
    }

    /// 📊 Get database statistics
    pub fn get_stats(&self) -> DatabaseStats {
        let blocks_count = self.blocks.len();
        let state_count = self.state.len();
        
        DatabaseStats {
            blocks_count,
            state_entries: state_count,
        }
    }
}

/// 📊 Database statistics
#[derive(Debug, Clone)]
pub struct DatabaseStats {
    pub blocks_count: usize,
    pub state_entries: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::storage::{ConsensusData, Transaction};
    use crate::core::crypto::QuantumKeyPair;

    fn create_test_block(height: u64) -> Block {
        Block::new(
            [0; 32],
            vec![],
            height,
            ConsensusData::FastLane { validator: vec![1, 2, 3, 4] },
        )
    }

    #[test]
    fn test_database_operations() {
        let temp_dir = std::env::temp_dir().join("triunity_test_db");
        let _ = std::fs::remove_dir_all(&temp_dir); // Clean up
        
        let db = BlockchainDB::new(temp_dir.to_str().unwrap()).unwrap();
        
        // Store and retrieve block
        let block = create_test_block(1);
        db.store_block(&block).unwrap();
        
        let retrieved = db.get_block(1).unwrap().unwrap();
        assert_eq!(retrieved.header.height, 1);
        
        // Check latest height
        let latest = db.get_latest_height().unwrap();
        assert_eq!(latest, 1);
        
        println!("💾 Database operations working!");
        println!("   Stored and retrieved block height: {}", retrieved.header.height);
        println!("   Latest height: {}", latest);
        
        // Clean up
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_state_operations() {
        let temp_dir = std::env::temp_dir().join("triunity_test_state");
        let _ = std::fs::remove_dir_all(&temp_dir);
        
        let db = BlockchainDB::new(temp_dir.to_str().unwrap()).unwrap();
        
        // Store and retrieve state
        let key = "test_account";
        let value = b"account_data";
        
        db.store_state(key, value).unwrap();
        let retrieved = db.get_state(key).unwrap().unwrap();
        
        assert_eq!(retrieved, value);
        
        println!("🗄️ State operations working!");
        
        // Clean up
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}