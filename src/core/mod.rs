//! 🔧 Core blockchain functionality

pub mod crypto;
pub mod consensus;
pub mod network;
pub mod storage;

// Re-export with specific imports to avoid conflicts
pub use crypto::*;
pub use consensus::*;
pub use network::{NetworkProtocol, NodeCapabilities, NetworkMessage, NetworkStats};
pub use storage::{Block, StateManager, MerkleTree, ConsensusData};

// Note: Transaction comes from storage, not network