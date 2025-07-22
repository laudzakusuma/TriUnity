//! 🤖 AI-powered consensus mechanisms

pub mod router;
pub mod metrics;
pub mod algorithms;

// Re-export ALL consensus types
pub use router::*;
pub use metrics::*;
pub use algorithms::*;