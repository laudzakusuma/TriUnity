use getrandom::getrandom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumKeyPair {
    private_key: [u8; 32],
    public_key: [u8; 32],
}

impl QuantumKeyPair {
    pub fn generate() -> Self {
        let mut private_key = [0u8; 32];
        let mut public_key = [0u8; 32];
        
        getrandom(&mut private_key).expect("Failed to generate random private key");
        getrandom(&mut public_key).expect("Failed to generate random public key");
        
        Self {
            private_key,
            public_key,
        }
    }

    pub fn public_key(&self) -> &[u8; 32] {
        &self.public_key
    }

    pub fn private_key(&self) -> &[u8; 32] {
        &self.private_key
    }

    pub fn address_hex(&self) -> String {
        hex::encode(&self.public_key[..20])
    }

    pub fn full_address(&self) -> [u8; 32] {
        self.public_key
    }

    pub fn address(&self) -> [u8; 20] {
        let mut address = [0u8; 20];
        address.copy_from_slice(&self.public_key[..20]);
        address
    }

    pub fn sign(&self, _message: &[u8]) -> Vec<u8> {
        use sha3::{Digest, Sha3_256};
        let mut hasher = Sha3_256::new();
        hasher.update(&self.private_key);
        hasher.update(_message);
        hasher.finalize().to_vec()
    }

    pub fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
        let expected = self.sign(message);
        expected == signature
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_keypair_generation() {
        let keypair = QuantumKeyPair::generate();
        
        assert_ne!(keypair.private_key, [0; 32]);
        assert_ne!(keypair.public_key, [0; 32]);
        
        println!("   Quantum keypair generated!");
        println!("   Address: 0x{}", keypair.address_hex());
        println!("   Full address: 0x{}", hex::encode(keypair.full_address()));
    }

    #[test]
    fn test_randomness() {
        let keypair1 = QuantumKeyPair::generate();
        let keypair2 = QuantumKeyPair::generate();
        
        // Should be different (cryptographically secure random)
        assert_ne!(keypair1.private_key, keypair2.private_key);
        assert_ne!(keypair1.public_key, keypair2.public_key);
        assert_ne!(keypair1.address_hex(), keypair2.address_hex());
        
        println!("   Cryptographic randomness working!");
        println!("   Address 1: 0x{}", keypair1.address_hex());
        println!("   Address 2: 0x{}", keypair2.address_hex());
    }

    #[test]
    fn test_address_formats() {
        let keypair = QuantumKeyPair::generate();
        
        let hex_address = keypair.address_hex();
        let byte_address = keypair.address();
        let full_address = keypair.full_address();
        
        assert_eq!(hex_address.len(), 40);
        assert_eq!(byte_address.len(), 20);
        assert_eq!(full_address.len(), 32);
        assert_eq!(hex_address, hex::encode(byte_address));
        
        println!("   Address formats working!");
        println!("   Hex (20 bytes): 0x{}", hex_address);
        println!("   Full (32 bytes): 0x{}", hex::encode(full_address));
    }

    #[test]
    fn test_simple_signatures() {
        let keypair = QuantumKeyPair::generate();
        let message = b"Hello TriUnity!";
        
        let signature = keypair.sign(message);
        assert!(keypair.verify(message, &signature));
        
        // Wrong message should fail
        let wrong_message = b"Wrong message";
        assert!(!keypair.verify(wrong_message, &signature));
        
        println!("   Simple signatures working!");
        println!("   Signature length: {} bytes", signature.len());
    }

    #[test]
    fn test_serialization() {
        let keypair = QuantumKeyPair::generate();
        let serialized = bincode::serialize(&keypair).unwrap();
        let deserialized: QuantumKeyPair = bincode::deserialize(&serialized).unwrap();
        
        assert_eq!(keypair.private_key, deserialized.private_key);
        assert_eq!(keypair.public_key, deserialized.public_key);
        assert_eq!(keypair.address_hex(), deserialized.address_hex());
        
        println!("   Serialization perfect!");
        println!("   Serialized size: {} bytes", serialized.len());
    }
}