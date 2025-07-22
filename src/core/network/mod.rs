//! 🌐 TriUnity Networking Layer
//! 
//! High-performance P2P networking for quantum-safe blockchain communication

pub mod protocol;
pub mod discovery;
pub mod sync;

pub use protocol::*;
pub use discovery::*;
pub use sync::*;