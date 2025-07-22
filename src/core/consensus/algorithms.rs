//! ⚙️ Consensus Algorithm Implementations
//! 
//! Various consensus mechanisms used by TriUnity's adaptive system

use serde::{Deserialize, Serialize};

/// 🎯 Consensus algorithm types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusAlgorithm {
    /// ⚡ Delegated Proof of Stake (Fast Lane)
    DelegatedProofOfStake {
        validator_count: usize,
        rotation_time: u64, // seconds
    },
    /// 🛡️ Byzantine Fault Tolerance (Secure Lane)
    ByzantineFaultTolerance {
        required_confirmations: usize,
        timeout: u64, // milliseconds
    },
    /// 🎭 Proof of Authority (Emergency Mode)
    ProofOfAuthority {
        authorities: Vec<Vec<u8>>, // public keys
    },
    /// 🔄 Hybrid PoS/PoW (Balanced)
    HybridStakeWork {
        stake_weight: f64,
        work_weight: f64,
    },
}

// Rest of implementation...
impl ConsensusAlgorithm {
    /// ⚡ Fast consensus for high throughput
    pub fn fast_consensus(validator_count: usize) -> Self {
        Self::DelegatedProofOfStake {
            validator_count: validator_count.min(21), // Cap for speed
            rotation_time: 30, // 30 second rotation
        }
    }

    /// 🛡️ Secure consensus for critical operations
    pub fn secure_consensus(total_validators: usize) -> Self {
        Self::ByzantineFaultTolerance {
            required_confirmations: (total_validators * 2 / 3) + 1,
            timeout: 10_000, // 10 second timeout
        }
    }

    /// 🚨 Emergency consensus under attack
    pub fn emergency_consensus(authorities: Vec<Vec<u8>>) -> Self {
        Self::ProofOfAuthority { authorities }
    }

    /// 🎯 Hybrid consensus for balance
    pub fn hybrid_consensus(stake_preference: f64) -> Self {
        Self::HybridStakeWork {
            stake_weight: stake_preference,
            work_weight: 1.0 - stake_preference,
        }
    }

    /// ⏱️ Get expected finality time in milliseconds
    pub fn expected_finality_time(&self) -> u64 {
        match self {
            Self::DelegatedProofOfStake { .. } => 100,    // Very fast
            Self::ByzantineFaultTolerance { timeout, .. } => *timeout,
            Self::ProofOfAuthority { .. } => 500,        // Fast but secure
            Self::HybridStakeWork { .. } => 2000,        // Moderate
        }
    }

    /// 🎯 Get expected throughput (TPS)
    pub fn expected_throughput(&self) -> u64 {
        match self {
            Self::DelegatedProofOfStake { .. } => 100_000, // Maximum speed
            Self::ByzantineFaultTolerance { .. } => 5_000,  // Secure but slower
            Self::ProofOfAuthority { .. } => 10_000,       // Fast emergency mode
            Self::HybridStakeWork { .. } => 25_000,        // Balanced
        }
    }

    /// 🔒 Get security level (0.0 to 1.0)
    pub fn security_level(&self) -> f64 {
        match self {
            Self::DelegatedProofOfStake { validator_count, .. } => {
                ((*validator_count as f64 / 21.0).min(1.0) * 0.7) + 0.1
            }
            Self::ByzantineFaultTolerance { required_confirmations, .. } => {
                ((*required_confirmations as f64 / 100.0).min(1.0) * 0.3) + 0.7
            }
            Self::ProofOfAuthority { authorities } => {
                (authorities.len() as f64 / 10.0).min(0.8)
            }
            Self::HybridStakeWork { stake_weight, .. } => {
                0.5 + (*stake_weight * 0.3)
            }
        }
    }

    /// 🌐 Get decentralization score (0.0 to 1.0)
    pub fn decentralization_score(&self) -> f64 {
        match self {
            Self::DelegatedProofOfStake { validator_count, .. } => {
                (*validator_count as f64 / 50.0).min(0.7)
            }
            Self::ByzantineFaultTolerance { required_confirmations, .. } => {
                ((*required_confirmations as f64 - 1.0) / 200.0).min(0.95)
            }
            Self::ProofOfAuthority { authorities } => {
                (authorities.len() as f64 / 20.0).min(0.5)
            }
            Self::HybridStakeWork { stake_weight, .. } => {
                0.4 + (*stake_weight * 0.4)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consensus_algorithms() {
        let fast = ConsensusAlgorithm::fast_consensus(50);
        let secure = ConsensusAlgorithm::secure_consensus(100);
        let emergency = ConsensusAlgorithm::emergency_consensus(vec![]);
        let hybrid = ConsensusAlgorithm::hybrid_consensus(0.7);

        println!("⚙️ Consensus algorithms created!");
        println!("   Fast TPS: {}", fast.expected_throughput());
        println!("   Secure TPS: {}", secure.expected_throughput());
        println!("   Emergency TPS: {}", emergency.expected_throughput());
        println!("   Hybrid TPS: {}", hybrid.expected_throughput());

        assert!(fast.expected_throughput() > secure.expected_throughput());
        assert!(secure.security_level() > fast.security_level());
    }
}