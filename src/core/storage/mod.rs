//! 💾 Blockchain storage and state management

pub mod blocks;
pub mod merkle;
pub mod state;
pub mod database;

// Export specific types to avoid conflicts
pub use blocks::{Block, Transaction, ConsensusData, BlockHeader};
pub use merkle::*;
pub use state::*;
pub use database::*;