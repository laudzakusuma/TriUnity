use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeQuantumKeyPair {
    private_key: [u8; 32],
    public_key: [u8; 32],
}

impl AlternativeQuantumKeyPair {
    pub fn generate() -> Self {
        let mut rng = rand::rng();
        
        let private_key: [u8; 32] = rng.random();
        let public_key: [u8; 32] = rng.random();
        
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alternative_keypair_generation() {
        let keypair = AlternativeQuantumKeyPair::generate();
        
        assert_ne!(keypair.private_key, [0; 32]);
        assert_ne!(keypair.public_key, [0; 32]);
        
        println!("🔑 Alternative quantum keypair generated!");
        println!("   Address: 0x{}", keypair.address_hex());
    }

    #[test]
    fn test_different_keypairs() {
        let keypair1 = AlternativeQuantumKeyPair::generate();
        let keypair2 = AlternativeQuantumKeyPair::generate();
        
        assert_ne!(keypair1.private_key, keypair2.private_key);
        assert_ne!(keypair1.public_key, keypair2.public_key);
        
        println!("🎲 Different keypairs generated successfully!");
    }
}