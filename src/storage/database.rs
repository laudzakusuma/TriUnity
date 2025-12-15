use sled::Db;
use crate::core::storage::Block;

#[derive(Debug, Clone)]
pub struct BlockchainDB {
    db: Db,
}

impl BlockchainDB {
    pub fn new(path: &str) -> Result<Self, String> {
        let db = sled::open(path)
            .map_err(|e| e.to_string())?;
        
        Ok(Self { db })
    }

    pub fn store_block(&self, block: &Block) -> Result<(), String> {
        let blocks = self.db.open_tree("blocks")
            .map_err(|e| e.to_string())?;
        
        let key = block.header.height.to_be_bytes();
        let value = bincode::serialize(block)
            .map_err(|e| e.to_string())?;
        
        blocks.insert(key, value)
            .map_err(|e| e.to_string())?;
        
        blocks.flush()
            .map_err(|e| e.to_string())?;
        
        Ok(())
    }

    pub fn get_block(&self, height: u64) -> Result<Option<Block>, String> {
        let blocks = self.db.open_tree("blocks")
            .map_err(|e| e.to_string())?;
        
        let key = height.to_be_bytes();
        
        if let Some(value) = blocks.get(key)
            .map_err(|e| e.to_string())? {
            
            let block: Block = bincode::deserialize(&value)
                .map_err(|e| e.to_string())?;
            
            Ok(Some(block))
        } else {
            Ok(None)
        }
    }

    pub fn get_latest_height(&self) -> Result<u64, String> {
        let blocks = self.db.open_tree("blocks")
            .map_err(|e| e.to_string())?;
        
        if let Some((key, _)) = blocks.last()
            .map_err(|e| e.to_string())? {
            
            let height = u64::from_be_bytes(
                key[..8].try_into()
                    .map_err(|_| "Invalid height key".to_string())?
            );
            
            Ok(height)
        } else {
            Ok(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::storage::ConsensusData;

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
        let _ = std::fs::remove_dir_all(&temp_dir);
        let db = BlockchainDB::new(temp_dir.to_str().unwrap()).unwrap();
        let block = create_test_block(1);
        db.store_block(&block).unwrap();
        
        let retrieved = db.get_block(1).unwrap().unwrap();
        assert_eq!(retrieved.header.height, 1);
        
        let latest = db.get_latest_height().unwrap();
        assert_eq!(latest, 1);
        
        println!("   Database operations working!");
        println!("   Stored and retrieved block height: {}", retrieved.header.height);
        println!("   Latest height: {}", latest);
        
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}