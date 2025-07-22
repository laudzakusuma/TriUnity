//! 🔐 Quantum-resistant hashing functions

use sha3::{Digest, Sha3_256};
use serde::{Deserialize, Serialize};

/// 256-bit hash type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash256(pub [u8; 32]);

impl Hash256 {
    /// Create hash from bytes
    pub fn from_bytes(data: &[u8]) -> Self {
        let mut hasher = Sha3_256::new();
        hasher.update(data);
        Hash256(hasher.finalize().into())
    }

    /// Zero hash
    pub fn zero() -> Self {
        Hash256([0; 32])
    }

    /// As byte slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// To hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl std::fmt::Display for Hash256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

/// Hash arbitrary data with SHA3-256
pub fn hash256(data: &[u8]) -> Hash256 {
    Hash256::from_bytes(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash256() {
        let data = b"TriUnity Protocol";
        let hash1 = hash256(data);
        let hash2 = hash256(data);
        assert_eq!(hash1, hash2);
        println!("🔐 Hash256 working!");
    }
}