use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct QuantumSignature {
    pub signature_data: Vec<u8>,
    pub public_key: Vec<u8>,
}