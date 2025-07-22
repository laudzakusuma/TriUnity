//! 🔧 Core blockchain functionality
//! 
//! The heart of TriUnity Protocol containing:
//! - Quantum-safe cryptographic primitives
//! - AI-powered consensus mechanisms  
//! - High-performance networking
//! - Efficient blockchain storage

pub mod crypto;
pub mod consensus;
pub mod network;
pub mod storage;

// Re-export commonly used types
pub use crypto::*;
pub use consensus::*;
pub use storage::*;