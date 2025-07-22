//! 🔐 Quantum-safe digital signatures
//! 
//! Using CRYSTALS-Dilithium - a post-quantum signature scheme
//! that remains secure even against quantum computer attacks!

use pqcrypto_dilithium::dilithium2;
use serde::{Deserialize, Serialize};
use crate::{Result, TriUnityError};

/// 🔑 Quantum-safe key pair for signing and verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumKeyPair {
    public_key: Vec<u8>,
    secret_key: Vec<u8>,
}

/// ✍️ Quantum-safe digital signature  
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuantumSignature {
    signature: Vec<u8>,
}

impl QuantumKeyPair {
    /// 🎲 Generate a new quantum-safe key pair
    pub fn generate() -> Self {
        let (pk, sk) = dilithium2::keypair();
        Self {
            public_key: pk.as_bytes().to_vec(),
            secret_key: sk.as_bytes().to_vec(),
        }
    }

    /// ✍️ Sign a message with quantum-safe cryptography
    pub fn sign(&self, message: &[u8]) -> Result<QuantumSignature> {
        let sk = dilithium2::SecretKey::from_bytes(&self.secret_key)
            .map_err(|_| TriUnityError::QuantumSignatureError)?;
        
        let signature = dilithium2::sign(message, &sk);
        
        Ok(QuantumSignature {
            signature: signature.as_bytes().to_vec(),
        })
    }

    /// 🔍 Get the public key bytes
    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    /// 🏠 Generate blockchain address from public key
    pub fn address(&self) -> [u8; 20] {
        use sha3::{Digest, Sha3_256};
        let hash = Sha3_256::digest(&self.public_key);
        let mut address = [0u8; 20];
        address.copy_from_slice(&hash[..20]);
        address
    }

    /// 🆔 Get address as hex string
    pub fn address_hex(&self) -> String {
        hex::encode(self.address())
    }
}

impl QuantumSignature {
    /// ✅ Verify signature against message and public key
    pub fn verify(&self, message: &[u8], public_key: &[u8]) -> bool {
        let pk = dilithium2::PublicKey::from_bytes(public_key);
        let sig = dilithium2::DetachedSignature::from_bytes(&self.signature);
        
        match (pk, sig) {
            (Ok(pk), Ok(sig)) => {
                dilithium2::verify_detached_signature(&sig, message, &pk).is_ok()
            }
            _ => false,
        }
    }

    /// 📦 Get signature bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.signature
    }

    /// 📏 Get signature size in bytes
    pub fn size(&self) -> usize {
        self.signature.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_keypair_generation() {
        let keypair = QuantumKeyPair::generate();
        assert!(!keypair.public_key.is_empty());
        assert!(!keypair.secret_key.is_empty());
        println!("🔑 Generated quantum-safe keypair!");
    }

    #[test] 
    fn test_quantum_signature_verification() {
        let keypair = QuantumKeyPair::generate();
        let message = b"Hello TriUnity!";
        
        let signature = keypair.sign(message).unwrap();
        assert!(signature.verify(message, keypair.public_key()));
        
        let wrong_message = b"Wrong message";
        assert!(!signature.verify(wrong_message, keypair.public_key()));
        
        println!("✅ Quantum signature verification passed!");
    }
}