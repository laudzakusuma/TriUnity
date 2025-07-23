//! 🔐 Quantum-safe digital signatures
//! 
//! Using CRYSTALS-Dilithium - a post-quantum signature scheme

use pqcrypto_dilithium::dilithium2;
use pqcrypto_traits::sign::{PublicKey, SecretKey, SignedMessage, DetachedSignature};
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
    pub signature_data: Vec<u8>,
    pub public_key: Vec<u8>,
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
            signature_data: signature.as_bytes().to_vec(),
            public_key: self.public_key.clone(),
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
    /// 🆕 Create new signature from bytes (for compatibility)
    pub fn new(signature_bytes: Vec<u8>) -> Self {
        Self {
            signature_data: signature_bytes,
            public_key: vec![0; 32], // Placeholder - will be set properly in real usage
        }
    }

    /// 🆕 Create signature with public key
    pub fn new_with_key(signature_bytes: Vec<u8>, public_key: Vec<u8>) -> Self {
        Self {
            signature_data: signature_bytes,
            public_key,
        }
    }

    /// ✅ Verify signature against message and public key
    pub fn verify(&self, message: &[u8], public_key: &[u8]) -> bool {
        let pk = dilithium2::PublicKey::from_bytes(public_key);
        let sig = dilithium2::DetachedSignature::from_bytes(&self.signature_data);
        
        match (pk, sig) {
            (Ok(pk), Ok(sig)) => {
                dilithium2::verify_detached_signature(&sig, message, &pk).is_ok()
            }
            _ => false,
        }
    }

    /// 📦 Get signature bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.signature_data
    }

    /// 📏 Get signature size in bytes
    pub fn size(&self) -> usize {
        self.signature_data.len()
    }
}

// For compatibility with quantum_key_pair.rs simple implementation
impl From<Vec<u8>> for QuantumSignature {
    fn from(signature_bytes: Vec<u8>) -> Self {
        Self::new(signature_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_keypair_generation() {
        let keypair = QuantumKeyPair::generate();
        
        // Check that keys are not empty
        assert!(!keypair.public_key.is_empty());
        assert!(!keypair.secret_key.is_empty());
        
        println!("🔑 Generated quantum-safe keypair!");
        println!("   Public key size: {} bytes", keypair.public_key.len());
        println!("   Secret key size: {} bytes", keypair.secret_key.len());
        println!("   Address: 0x{}", keypair.address_hex());
    }

    #[test] 
    fn test_quantum_signature_verification() {
        let keypair = QuantumKeyPair::generate();
        let message = "Hello from TriUnity Protocol!".as_bytes();
        
        // Sign message
        let signature = keypair.sign(message).unwrap();
        
        // Verify signature
        assert!(signature.verify(message, keypair.public_key()));
        
        // Test with wrong message
        let wrong_message = "Wrong message".as_bytes();
        assert!(!signature.verify(wrong_message, keypair.public_key()));
        
        // Test with wrong public key
        let other_keypair = QuantumKeyPair::generate();
        assert!(!signature.verify(message, other_keypair.public_key()));
        
        println!("✅ Quantum signature verification passed!");
        println!("   Signature size: {} bytes", signature.size());
    }

    #[test]
    fn test_address_generation() {
        let keypair = QuantumKeyPair::generate();
        let address1 = keypair.address();
        let address2 = keypair.address();
        
        // Same keypair = same address
        assert_eq!(address1, address2);
        
        // Different keypairs = different addresses  
        let other_keypair = QuantumKeyPair::generate();
        let other_address = other_keypair.address();
        assert_ne!(address1, other_address);
        
        println!("🏠 Address generation working!");
        println!("   Address 1: 0x{}", hex::encode(address1));
        println!("   Address 2: 0x{}", hex::encode(other_address));
    }

    #[test]
    fn test_serialization() {
        let keypair = QuantumKeyPair::generate();
        let message = "Serialization test".as_bytes();
        let signature = keypair.sign(message).unwrap();
        
        // Test keypair serialization
        let serialized_keypair = bincode::serialize(&keypair).unwrap();
        let deserialized_keypair: QuantumKeyPair = bincode::deserialize(&serialized_keypair).unwrap();
        assert_eq!(keypair.public_key(), deserialized_keypair.public_key());
        
        // Test signature serialization
        let serialized_sig = bincode::serialize(&signature).unwrap();
        let deserialized_sig: QuantumSignature = bincode::deserialize(&serialized_sig).unwrap();
        assert_eq!(signature, deserialized_sig);
        
        println!("📦 Serialization working perfectly!");
    }

    #[test]
    fn test_signature_compatibility() {
        // Test new() constructor
        let sig_bytes = vec![1, 2, 3, 4, 5];
        let signature = QuantumSignature::new(sig_bytes.clone());
        assert_eq!(signature.signature_data, sig_bytes);
        
        // Test From trait
        let signature2: QuantumSignature = sig_bytes.clone().into();
        assert_eq!(signature2.signature_data, sig_bytes);
        
        println!("🔄 Signature compatibility working!");
    }
}