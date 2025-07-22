use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct QuantumSignature {
    pub signature_data: Vec<u8>,
    pub public_key: Vec<u8>,
}

impl QuantumSignature {
    // FIXED: Menambahkan method `size` yang hilang.
    // Method ini mengembalikan ukuran dari data tanda tangan dalam byte.
    pub fn size(&self) -> usize {
        self.signature_data.len()
    }

    // FIXED: Menambahkan method `verify` yang hilang.
    // Ini adalah implementasi placeholder.
    pub fn verify(&self, message: &[u8], public_key: &[u8]) -> bool {
        // 1. Periksa apakah public key yang digunakan untuk verifikasi sama dengan yang ada di signature.
        if self.public_key != public_key {
            return false;
        }
        // 2. Verifikasi tanda tangan (placeholder logic).
        let expected_signature_data = format!("signed_{}", String::from_utf8_lossy(message)).into_bytes();
        self.signature_data == expected_signature_data
    }
}