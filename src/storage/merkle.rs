//! 🌳 Quantum-safe Merkle trees
//! 
//! Efficient merkle tree implementation for blockchain verification

use sha3::{Digest, Sha3_256};
use serde::{Deserialize, Serialize};

/// 🌳 Merkle tree for quantum-safe verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleTree {
    root: [u8; 32],
    leaves: Vec<[u8; 32]>,
}

impl MerkleTree {
    pub fn new(data: &[Vec<u8>]) -> Self {
        if data.is_empty() {
            return Self {
                root: [0; 32],
                leaves: Vec::new(),
            };
        }

        let leaves: Vec<[u8; 32]> = data.iter()
            .map(|item| {
                let mut hasher = Sha3_256::new();
                hasher.update(item);
                hasher.finalize().into()
            })
            .collect();

        let root = Self::calculate_root(&leaves);

        Self { root, leaves }
    }
    fn calculate_root(leaves: &[[u8; 32]]) -> [u8; 32] {
        if leaves.is_empty() {
            return [0; 32];
        }
        let mut hashes = leaves.to_vec();
        while hashes.len() > 1 {
            let mut next_level = Vec::new();
            for chunk in hashes.chunks(2) {
                let mut hasher = Sha3_256::new();
                hasher.update(&chunk[0]);
                if chunk.len() > 1 {
                    hasher.update(&chunk[1]);
                } else {
                    hasher.update(&chunk[0]); // Duplicate if odd
                }
                next_level.push(hasher.finalize().into());
            }
            
            hashes = next_level;
        }

        hashes[0]
    }

    pub fn root(&self) -> [u8; 32] {
        self.root
    }
    pub fn leaves(&self) -> &[[u8; 32]] {
        &self.leaves
    }
    pub fn generate_proof(&self, index: usize) -> Option<MerkleProof> {
        if index >= self.leaves.len() {
            return None;
        }

        let mut proof = Vec::new();
        let mut current_index = index;
        let mut current_level = self.leaves.clone();

        while current_level.len() > 1 {
            let is_right = current_index % 2 == 1;
            let sibling_index = if is_right {
                current_index - 1
            } else {
                current_index + 1
            };

            if sibling_index < current_level.len() {
                proof.push(MerkleProofElement {
                    hash: current_level[sibling_index],
                    is_right: !is_right,
                });
            }
            let mut next_level = Vec::new();
            for chunk in current_level.chunks(2) {
                let mut hasher = Sha3_256::new();
                hasher.update(&chunk[0]);
                if chunk.len() > 1 {
                    hasher.update(&chunk[1]);
                } else {
                    hasher.update(&chunk[0]);
                }
                next_level.push(hasher.finalize().into());
            }

            current_level = next_level;
            current_index /= 2;
        }

        Some(MerkleProof {
            leaf_hash: self.leaves[index],
            proof,
            root: self.root,
        })
    }
    pub fn verify_proof(proof: &MerkleProof) -> bool {
        let mut current_hash = proof.leaf_hash;

        for element in &proof.proof {
            let mut hasher = Sha3_256::new();
            if element.is_right {
                hasher.update(&current_hash);
                hasher.update(&element.hash);
            } else {
                hasher.update(&element.hash);
                hasher.update(&current_hash);
            }
            current_hash = hasher.finalize().into();
        }

        current_hash == proof.root
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    pub leaf_hash: [u8; 32],
    pub proof: Vec<MerkleProofElement>,
    pub root: [u8; 32],
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProofElement {
    pub hash: [u8; 32],
    pub is_right: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_tree_creation() {
        let data = vec![
            b"transaction1".to_vec(),
            b"transaction2".to_vec(),
            b"transaction3".to_vec(),
            b"transaction4".to_vec(),
        ];

        let tree = MerkleTree::new(&data);
        assert_ne!(tree.root(), [0; 32]);
        assert_eq!(tree.leaves().len(), 4);

        println!("   Merkle tree creation working!");
        println!("   Root: {}", hex::encode(tree.root()));
        println!("   Leaves: {}", tree.leaves().len());
    }

    #[test]
    fn test_empty_merkle_tree() {
        let tree = MerkleTree::new(&[]);
        assert_eq!(tree.root(), [0; 32]);
        assert_eq!(tree.leaves().len(), 0);

        println!("Empty merkle tree working!");
    }

    #[test]
    fn test_merkle_proof() {
        let data = vec![
            b"tx1".to_vec(),
            b"tx2".to_vec(),
            b"tx3".to_vec(),
            b"tx4".to_vec(),
        ];
        let tree = MerkleTree::new(&data);
        let proof = tree.generate_proof(0).unwrap();
        assert!(MerkleTree::verify_proof(&proof));

        let proof = tree.generate_proof(3).unwrap();
        assert!(MerkleTree::verify_proof(&proof));

        println!("Merkle proof generation and verification working!");
    }

    #[test]
    fn test_single_leaf_tree() {
        let data = vec![b"single_transaction".to_vec()];
        let tree = MerkleTree::new(&data);
        
        assert_ne!(tree.root(), [0; 32]);
        assert_eq!(tree.leaves().len(), 1);

        let proof = tree.generate_proof(0).unwrap();
        assert!(MerkleTree::verify_proof(&proof));

        println!("Single leaf merkle tree working!");
    }
}