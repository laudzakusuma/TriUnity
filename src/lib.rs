//! 🚀 TriUnity Protocol - Core Library

pub mod consensus;
pub mod storage; 
pub mod blockchain;
pub mod crypto;
pub mod web;

// Re-export main types
pub use blockchain::{Block, Transaction};
pub use consensus::ConsensusEngine;
pub use storage::TriUnityStorage;