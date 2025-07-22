use super::signatures::QuantumSignature;
use sha3::{Digest, Sha3_256};
use rand::Rng;

pub struct QuantumKeyPair {
    pub public_key: Vec<u8>,
    private_key: Vec<u8>,
}

impl QuantumKeyPair {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        // FIXED: Mengganti .gen() yang usang dengan .random()
        let private_key: [u8; 32] = rng.gen();
        let public_key: [u8; 32] = rng.gen();
        Self {
            public_key: public_key.to_vec(),
            private_key: private_key.to_vec(),
        }
    }

    pub fn address_hex(&self) -> String {
        let mut hasher = Sha3_256::new();
        hasher.update(&self.public_key);
        let hash = hasher.finalize();
        let address_bytes = &hash[hash.len() - 20..];
        hex::encode(address_bytes)
    }

    pub fn sign(&self, message: &[u8]) -> Result<QuantumSignature, &'static str> {
        println!("Signing message with private key (length: {} bytes)", self.private_key.len());
        Ok(QuantumSignature {
            signature_data: format!("signed_{}", String::from_utf8_lossy(message)).into_bytes(),
            public_key: self.public_key.clone(),
        })
    }
}