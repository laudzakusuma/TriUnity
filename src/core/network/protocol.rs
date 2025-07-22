//! 🌐 Network Protocol Definitions
//! 
//! Simple networking for development (no libp2p dependency)

use serde::{Deserialize, Serialize};
use crate::core::crypto::QuantumSignature;
use crate::core::storage::Block;
use std::net::SocketAddr;

/// 📡 Network message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    /// 👋 Node handshake
    Handshake {
        version: String,
        node_id: Vec<u8>,
        capabilities: NodeCapabilities,
        timestamp: u64,
    },
    /// 📦 Block proposal
    BlockProposal {
        block: Block,
        proposer_signature: QuantumSignature,
    },
    /// 🗳️ Consensus vote
    ConsensusVote {
        block_hash: [u8; 32],
        vote_type: VoteType,
        validator_id: Vec<u8>,
        signature: QuantumSignature,
    },
    /// 📋 Transaction broadcast
    TransactionBroadcast {
        transactions: Vec<Transaction>,
    },
    /// 🔄 Sync request
    SyncRequest {
        start_height: u64,
        end_height: u64,
    },
    /// 📊 Sync response
    SyncResponse {
        blocks: Vec<Block>,
        is_final: bool,
    },
    /// 💓 Heartbeat
    Heartbeat {
        timestamp: u64,
        network_stats: NetworkStats,
    },
}

/// 🎯 Node capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCapabilities {
    pub is_validator: bool,
    pub supports_fast_sync: bool,
    pub max_connections: u32,
    pub supported_protocols: Vec<String>,
    pub quantum_safe: bool,
}

/// 🗳️ Vote types for consensus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoteType {
    Propose,
    Prevote,
    Precommit,
    Commit,
}

/// 💳 Transaction structure (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: Vec<u8>,
    pub to: Vec<u8>,
    pub amount: u64,
    pub fee: u64,
    pub nonce: u64,
    pub signature: QuantumSignature,
}

/// 📊 Network statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub connected_peers: u32,
    pub current_tps: u64,
    pub avg_latency: u64,
}

/// 🌐 Simple network protocol handler
#[derive(Debug)]
pub struct NetworkProtocol {
    local_node_id: Vec<u8>,
    capabilities: NodeCapabilities,
    connected_peers: Vec<SocketAddr>,
}

impl NetworkProtocol {
    /// 🚀 Create new network protocol handler
    pub fn new(node_id: Vec<u8>, capabilities: NodeCapabilities) -> Self {
        Self {
            local_node_id: node_id,
            capabilities,
            connected_peers: Vec::new(),
        }
    }

    /// 📡 Handle incoming network message (simplified)
    pub fn handle_message(&mut self, message: NetworkMessage, from: SocketAddr) -> Result<Option<NetworkMessage>, String> {
        match message {
            NetworkMessage::Handshake { version, node_id, capabilities, .. } => {
                println!("👋 Received handshake from node: {}", hex::encode(&node_id[..8]));
                self.connected_peers.push(from);
                
                // Respond with our handshake
                Ok(Some(NetworkMessage::Handshake {
                    version: crate::VERSION.to_string(),
                    node_id: self.local_node_id.clone(),
                    capabilities: self.capabilities.clone(),
                    timestamp: current_timestamp(),
                }))
            }
            
            NetworkMessage::BlockProposal { block, .. } => {
                println!("📦 Received block proposal at height: {}", block.header.height);
                Ok(None)
            }
            
            NetworkMessage::Heartbeat { network_stats, .. } => {
                println!("💓 Heartbeat - peers: {}, TPS: {}", 
                    network_stats.connected_peers, network_stats.current_tps);
                Ok(None)
            }
            
            _ => {
                println!("📨 Received message type");
                Ok(None)
            }
        }
    }

    /// 📊 Get network statistics
    pub fn get_network_stats(&self) -> NetworkStats {
        NetworkStats {
            connected_peers: self.connected_peers.len() as u32,
            current_tps: 0,
            avg_latency: 100,
        }
    }

    /// 👥 Get connected peers
    pub fn get_connected_peers(&self) -> &[SocketAddr] {
        &self.connected_peers
    }
}

impl Default for NodeCapabilities {
    fn default() -> Self {
        Self {
            is_validator: false,
            supports_fast_sync: true,
            max_connections: 50,
            supported_protocols: vec!["triunity/1.0".to_string()],
            quantum_safe: true,
        }
    }
}

/// 🕐 Get current timestamp
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::crypto::QuantumKeyPair;

    #[test]
    fn test_network_protocol_creation() {
        let keypair = QuantumKeyPair::generate();
        let node_id = keypair.public_key().to_vec();
        let capabilities = NodeCapabilities::default();
        
        let protocol = NetworkProtocol::new(node_id.clone(), capabilities);
        assert_eq!(protocol.local_node_id, node_id);
        assert_eq!(protocol.connected_peers.len(), 0);
        
        println!("🌐 Network protocol created successfully!");
    }

    #[test]
    fn test_handshake_message() {
        let keypair = QuantumKeyPair::generate();
        let node_id = keypair.public_key().to_vec();
        let mut protocol = NetworkProtocol::new(node_id.clone(), NodeCapabilities::default());
        
        let handshake = NetworkMessage::Handshake {
            version: "1.0.0".to_string(),
            node_id: vec![1, 2, 3, 4],
            capabilities: NodeCapabilities::default(),
            timestamp: 12345,
        };

        let from_addr = "127.0.0.1:8080".parse().unwrap();
        let response = protocol.handle_message(handshake, from_addr).unwrap();
        
        assert!(response.is_some());
        assert_eq!(protocol.connected_peers.len(), 1);
        
        println!("👋 Handshake handling working!");
    }

    #[test]
    fn test_network_stats() {
        let keypair = QuantumKeyPair::generate();
        let protocol = NetworkProtocol::new(keypair.public_key().to_vec(), NodeCapabilities::default());
        
        let stats = protocol.get_network_stats();
        assert_eq!(stats.connected_peers, 0);
        
        println!("📊 Network stats working!");
        println!("   Connected peers: {}", stats.connected_peers);
        println!("   Current TPS: {}", stats.current_tps);
        println!("   Average latency: {}ms", stats.avg_latency);
    }
}