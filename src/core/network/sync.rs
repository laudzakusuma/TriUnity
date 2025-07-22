//! 🔄 Blockchain Synchronization
//! 
//! Fast and efficient blockchain sync with other nodes

use serde::{Deserialize, Serialize};
use crate::core::storage::Block;
use std::collections::VecDeque;

/// 🔄 Blockchain synchronization manager
#[derive(Debug)]
pub struct SyncManager {
    current_height: u64,
    target_height: u64,
    sync_mode: SyncMode,
    pending_blocks: VecDeque<Block>,
    sync_peers: Vec<SyncPeer>,
}

/// ⚡ Synchronization modes
#[derive(Debug, Clone)]
pub enum SyncMode {
    /// 🚀 Fast sync (download state snapshots)
    FastSync { checkpoint_height: u64 },
    /// 🔗 Full sync (download all blocks)
    FullSync { start_height: u64 },
    /// 📦 Block sync (catch up recent blocks)
    BlockSync { missing_range: (u64, u64) },
    /// ✅ Synced (up to date)
    Synced,
}

/// 👥 Synchronization peer
#[derive(Debug, Clone)]
pub struct SyncPeer {
    pub node_id: Vec<u8>,
    pub reported_height: u64,
    pub sync_speed: f64, // blocks per second
    pub reliability: f64, // 0.0 to 1.0
    pub is_syncing: bool,
}

/// 📊 Sync progress information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncProgress {
    pub current_height: u64,
    pub target_height: u64,
    pub progress_percentage: f64,
    pub blocks_per_second: f64,
    pub estimated_time_remaining: u64, // seconds
    pub active_peers: usize,
}

/// 🎯 Sync request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRequest {
    pub start_height: u64,
    pub end_height: u64,
    pub max_blocks: u32,
}

/// 📦 Sync response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResponse {
    pub blocks: Vec<Block>,
    pub start_height: u64,
    pub is_final: bool,
    pub peer_height: u64,
}

impl SyncManager {
    /// 🚀 Create new sync manager
    pub fn new(current_height: u64) -> Self {
        Self {
            current_height,
            target_height: current_height,
            sync_mode: SyncMode::Synced,
            pending_blocks: VecDeque::new(),
            sync_peers: Vec::new(),
        }
    }

    /// 🔍 Check if sync is needed
    pub fn check_sync_needed(&mut self, peer_heights: Vec<(Vec<u8>, u64)>) -> bool {
        if peer_heights.is_empty() {
            return false;
        }

        // Find highest reported height
        let max_peer_height = peer_heights.iter()
            .map(|(_, height)| *height)
            .max()
            .unwrap_or(0);

        // Update peer information
        for (node_id, height) in peer_heights {
            self.update_peer_height(node_id, height);
        }

        // Need sync if we're significantly behind
        let height_diff = max_peer_height.saturating_sub(self.current_height);
        
        if height_diff > 10 {
            self.target_height = max_peer_height;
            self.start_sync(height_diff);
            true
        } else {
            self.sync_mode = SyncMode::Synced;
            false
        }
    }

    /// 🚀 Start synchronization process
    fn start_sync(&mut self, height_diff: u64) {
        self.sync_mode = if height_diff > 1000 {
            // Use fast sync for large gaps
            SyncMode::FastSync {
                checkpoint_height: self.target_height.saturating_sub(100),
            }
        } else if height_diff > 100 {
            // Use full sync for medium gaps
            SyncMode::FullSync {
                start_height: self.current_height + 1,
            }
        } else {
            // Use block sync for small gaps
            SyncMode::BlockSync {
                missing_range: (self.current_height + 1, self.target_height),
            }
        };

        println!("🔄 Starting sync: {:?}", self.sync_mode);
        println!("   Current height: {}", self.current_height);
        println!("   Target height: {}", self.target_height);
        println!("   Blocks behind: {}", height_diff);
    }

    /// 📥 Process sync response from peer
    pub fn process_sync_response(&mut self, response: SyncResponse, from_peer: &[u8]) -> Result<usize, String> {
        println!("📥 Received {} blocks from peer {}", 
            response.blocks.len(), 
            hex::encode(&from_peer[..4])
        );

        let mut blocks_processed = 0;

        // Validate and queue blocks
        for block in response.blocks {
            if self.validate_block(&block) {
                self.pending_blocks.push_back(block);
                blocks_processed += 1;
            } else {
                println!("❌ Invalid block received from peer");
                self.penalize_peer(from_peer);
            }
        }

        // Update peer performance
        if let Some(peer) = self.sync_peers.iter_mut().find(|p| p.node_id == from_peer) {
            peer.reported_height = response.peer_height;
            if blocks_processed > 0 {
                peer.reliability = (peer.reliability * 0.9) + 0.1; // Increase reliability
            }
        }

        // Process queued blocks
        self.process_pending_blocks();

        Ok(blocks_processed)
    }

    /// 📦 Process pending blocks in order
    fn process_pending_blocks(&mut self) {
        let mut processed = 0;

        while let Some(block) = self.pending_blocks.front() {
            if block.header.height == self.current_height + 1 {
                let block = self.pending_blocks.pop_front().unwrap();
                self.apply_block(block);
                self.current_height += 1;
                processed += 1;
            } else {
                break; // Wait for missing blocks
            }
        }

        if processed > 0 {
            println!("📦 Processed {} blocks, current height: {}", processed, self.current_height);
        }

        // Check if sync is complete
        if self.current_height >= self.target_height {
            self.sync_mode = SyncMode::Synced;
            println!("✅ Sync completed at height {}", self.current_height);
        }
    }

    /// ✅ Validate received block
    fn validate_block(&self, _block: &Block) -> bool {
        // TODO: Implement proper block validation
        // For now, just return true for testing
        true
    }

    /// 💾 Apply validated block to blockchain
    fn apply_block(&mut self, block: Block) {
        // TODO: Apply block to state and store
        println!("💾 Applied block {}", block.header.height);
    }

    /// ⚠️ Penalize misbehaving peer
    fn penalize_peer(&mut self, peer_id: &[u8]) {
        if let Some(peer) = self.sync_peers.iter_mut().find(|p| p.node_id == peer_id) {
            peer.reliability = (peer.reliability * 0.5).max(0.1);
            println!("⚠️ Penalized peer {} (reliability: {:.2})", 
                hex::encode(&peer_id[..4]), peer.reliability);
        }
    }

    /// 📊 Get sync progress
    pub fn get_sync_progress(&self) -> SyncProgress {
        let progress_percentage = if self.target_height > self.current_height {
            (self.current_height as f64 / self.target_height as f64) * 100.0
        } else {
            100.0
        };

        let blocks_per_second = self.calculate_sync_speed();
        let remaining_blocks = self.target_height.saturating_sub(self.current_height);
        let estimated_time_remaining = if blocks_per_second > 0.0 {
            (remaining_blocks as f64 / blocks_per_second) as u64
        } else {
            0
        };

        SyncProgress {
            current_height: self.current_height,
            target_height: self.target_height,
            progress_percentage,
            blocks_per_second,
            estimated_time_remaining,
            active_peers: self.sync_peers.iter().filter(|p| p.is_syncing).count(),
        }
    }

    /// ⚡ Calculate current sync speed
    fn calculate_sync_speed(&self) -> f64 {
        // Calculate average speed of active, reliable peers
        let active_peers: Vec<_> = self.sync_peers.iter()
            .filter(|peer| peer.is_syncing && peer.reliability > 0.5)
            .collect();

        if active_peers.is_empty() {
            0.0
        } else {
            active_peers.iter().map(|peer| peer.sync_speed).sum::<f64>() / active_peers.len() as f64
        }
    }

    /// 📈 Update peer height information
    fn update_peer_height(&mut self, node_id: Vec<u8>, height: u64) {
        if let Some(peer) = self.sync_peers.iter_mut().find(|p| p.node_id == node_id) {
            peer.reported_height = height;
        } else {
            // Add new peer
            let peer = SyncPeer {
                node_id,
                reported_height: height,
                sync_speed: 10.0, // Default speed
                reliability: 0.8, // Default reliability
                is_syncing: false,
            };
            self.sync_peers.push(peer);
        }
    }

    /// 🎯 Create sync request for best peer
    pub fn create_sync_request(&mut self) -> Option<(SyncRequest, Vec<u8>)> {
        // Find best peer for syncing
        let best_peer = self.sync_peers.iter_mut()
            .filter(|peer| peer.reported_height > self.current_height && peer.reliability > 0.5)
            .max_by(|a, b| {
                let score_a = a.sync_speed * a.reliability;
                let score_b = b.sync_speed * b.reliability;
                score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
            })?;

        best_peer.is_syncing = true;

        let request = match &self.sync_mode {
            SyncMode::FastSync { checkpoint_height } => {
                SyncRequest {
                    start_height: *checkpoint_height,
                    end_height: self.target_height,
                    max_blocks: 100,
                }
            }
            SyncMode::FullSync { start_height } => {
                SyncRequest {
                    start_height: *start_height,
                    end_height: (start_height + 50).min(self.target_height),
                    max_blocks: 50,
                }
            }
            SyncMode::BlockSync { missing_range } => {
                SyncRequest {
                    start_height: missing_range.0,
                    end_height: missing_range.1,
                    max_blocks: ((missing_range.1 - missing_range.0 + 1) as u32).min(20),
                }
            }
            SyncMode::Synced => return None,
        };

        Some((request, best_peer.node_id.clone()))
    }

    /// 📊 Get synchronization status
    pub fn get_sync_status(&self) -> &SyncMode {
        &self.sync_mode
    }

    /// 🏁 Check if fully synced
    pub fn is_synced(&self) -> bool {
        matches!(self.sync_mode, SyncMode::Synced)
    }

    /// 📏 Get current blockchain height
    pub fn get_current_height(&self) -> u64 {
        self.current_height
    }

    /// 🎯 Get target height
    pub fn get_target_height(&self) -> u64 {
        self.target_height
    }

    /// 👥 Get active sync peers
    pub fn get_sync_peers(&self) -> &[SyncPeer] {
        &self.sync_peers
    }

    /// 🧹 Cleanup inactive peers
    pub fn cleanup_inactive_peers(&mut self) {
        let initial_count = self.sync_peers.len();
        self.sync_peers.retain(|peer| peer.reliability > 0.1);
        
        let removed = initial_count - self.sync_peers.len();
        if removed > 0 {
            println!("🧹 Removed {} unreliable sync peers", removed);
        }
    }
}

impl Default for SyncManager {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::storage::{Block, BlockHeader, ConsensusData};

    fn create_test_block(height: u64) -> Block {
        Block {
            header: BlockHeader {
                version: 1,
                previous_hash: [0; 32],
                merkle_root: [0; 32],
                timestamp: 12345,
                height,
                consensus_data: ConsensusData::FastLane { 
                    validator: vec![1, 2, 3, 4] 
                },
            },
            transactions: vec![], // Empty for test
        }
    }

    #[test]
    fn test_sync_manager_creation() {
        let sync_manager = SyncManager::new(100);
        assert_eq!(sync_manager.current_height, 100);
        assert_eq!(sync_manager.target_height, 100);
        assert!(sync_manager.is_synced());
        
        println!("🔄 Sync manager created successfully!");
    }

    #[test]
    fn test_sync_needed_detection() {
        let mut sync_manager = SyncManager::new(100);
        
        // Simulate peers with higher heights
        let peer_heights = vec![
            (vec![1, 2, 3, 4], 150),
            (vec![5, 6, 7, 8], 145),
            (vec![9, 10, 11, 12], 155),
        ];
        
        let sync_needed = sync_manager.check_sync_needed(peer_heights);
        assert!(sync_needed);
        assert_eq!(sync_manager.target_height, 155);
        assert!(!sync_manager.is_synced());
        
        println!("🔍 Sync detection working!");
        println!("   Current height: {}", sync_manager.current_height);
        println!("   Target height: {}", sync_manager.target_height);
        println!("   Sync mode: {:?}", sync_manager.sync_mode);
    }

    #[test]
    fn test_sync_response_processing() {
        let mut sync_manager = SyncManager::new(100);
        sync_manager.target_height = 105;
        sync_manager.sync_mode = SyncMode::BlockSync { 
            missing_range: (101, 105) 
        };
        
        // Create test blocks
        let blocks = vec![
            create_test_block(101),
            create_test_block(102),
        ];
        
        let response = SyncResponse {
            blocks,
            start_height: 101,
            is_final: false,
            peer_height: 105,
        };
        
        let peer_id = vec![1, 2, 3, 4];
        sync_manager.sync_peers.push(SyncPeer {
            node_id: peer_id.clone(),
            reported_height: 105,
            sync_speed: 10.0,
            reliability: 0.8,
            is_syncing: true,
        });
        
        let processed = sync_manager.process_sync_response(response, &peer_id).unwrap();
        assert_eq!(processed, 2);
        assert_eq!(sync_manager.current_height, 102);
        
        println!("📥 Sync response processing working!");
        println!("   Blocks processed: {}", processed);
        println!("   New height: {}", sync_manager.current_height);
    }

    #[test]
    fn test_sync_progress_calculation() {
        let mut sync_manager = SyncManager::new(100);
        sync_manager.target_height = 200;
        sync_manager.sync_mode = SyncMode::FullSync { start_height: 101 };
        
        let progress = sync_manager.get_sync_progress();
        assert_eq!(progress.current_height, 100);
        assert_eq!(progress.target_height, 200);
        assert_eq!(progress.progress_percentage, 50.0);
        
        println!("📊 Sync progress calculation working!");
        println!("   Progress: {:.1}%", progress.progress_percentage);
        println!("   Blocks per second: {:.1}", progress.blocks_per_second);
        println!("   ETA: {}s", progress.estimated_time_remaining);
    }

    #[test]
    fn test_sync_request_creation() {
        let mut sync_manager = SyncManager::new(100);
        sync_manager.target_height = 150;
        sync_manager.sync_mode = SyncMode::BlockSync { 
            missing_range: (101, 150) 
        };
        
        // Add a reliable peer
        sync_manager.sync_peers.push(SyncPeer {
            node_id: vec![1, 2, 3, 4],
            reported_height: 150,
            sync_speed: 20.0,
            reliability: 0.9,
            is_syncing: false,
        });
        
        let request_info = sync_manager.create_sync_request();
        assert!(request_info.is_some());
        
        let (request, peer_id) = request_info.unwrap();
        assert_eq!(request.start_height, 101);
        assert_eq!(request.end_height, 150);
        assert_eq!(peer_id, vec![1, 2, 3, 4]);
        
        println!("🎯 Sync request creation working!");
        println!("   Request range: {}-{}", request.start_height, request.end_height);
        println!("   Max blocks: {}", request.max_blocks);
    }
}