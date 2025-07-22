pub mod quantum_key_pair;
pub use quantum_key_pair::QuantumKeyPair;
// 1. Declare the 'signatures.rs' file as a public module.
pub mod signatures;

// 2. Re-export all public items from the 'signatures' module.
pub use signatures::*;