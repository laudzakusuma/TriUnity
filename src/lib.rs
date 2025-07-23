//! # TriUnity Protocol 🚀
//! 
//! The first blockchain that defeats the infamous trilemma!
//! 
//! ## Revolutionary Features:
//! - 🔐 **Quantum-Safe**: Post-quantum cryptography from day one
//! - 🤖 **AI-Powered**: Intelligent consensus routing
//! - ⚡ **100,000+ TPS**: Unprecedented scalability
//! - 🌐 **Truly Decentralized**: No compromise on decentralization
//! 
//! ## Architecture:
//! - `core`: Core blockchain engine with quantum-safe primitives
//! - `vm`: TriUnity Virtual Machine for smart contracts
//! - `api`: JSON-RPC and WebSocket APIs
//! - `cli`: Command-line interface and tools
pub mod consensus;
pub mod storage;
pub mod blockchain;
pub mod crypto;
pub mod web; 
pub mod core;
pub mod vm;
pub mod api;
pub mod cli;

use thiserror::Error;

/// TriUnity Protocol errors
#[derive(Error, Debug)]
pub enum TriUnityError {
    #[error("❌ Quantum signature verification failed")]
    QuantumSignatureError,
    
    #[error("🤖 Consensus routing error: {0}")]
    ConsensusError(String),
    
    #[error("🌐 Network error: {0}")]
    NetworkError(String),
    
    #[error("💾 Storage error: {0}")]
    StorageError(String),
    
    #[error("📦 Serialization error: {0}")]
    SerializationError(#[from] bincode::Error),
    
    #[error("📁 IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Result type for TriUnity operations
pub type Result<T> = std::result::Result<T, TriUnityError>;

/// TriUnity Protocol version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Target transactions per second (THE TRILEMMA KILLER! 🎯)
pub const TARGET_TPS: u64 = 100_000;

/// Quantum safety level in bits (FUTURE-PROOF! 🔐)
pub const QUANTUM_SAFETY_LEVEL: u16 = 256;

/// Maximum block size in bytes
pub const MAX_BLOCK_SIZE: usize = 32 * 1024 * 1024; // 32MB

/// Block time in milliseconds (LIGHTNING FAST! ⚡)
pub const BLOCK_TIME_MS: u64 = 100;