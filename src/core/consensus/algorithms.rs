//! ⚙️ Consensus Algorithm Implementations
//! 
//! Various consensus mechanisms used by TriUnity's adaptive system

use serde::{Deserialize, Serialize};
use crate::core::crypto::{QuantumKeyPair, QuantumSignature};
use crate::{Result, TriUnityError};

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

/// 👑 Validator information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    pub public_key: Vec<u8>,
    pub stake: u64,
    pub reputation: f64,
    pub is_active: bool,
    pub last_activity: u64,
}

/// 🗳️ Consensus vote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusVote {
    pub validator_id: Vec<u8>,
    pub block_hash: [u8; 32],
    pub vote_type: VoteType,
    pub signature: QuantumSignature,
    pub timestamp: u64,
}

/// 📊 Vote types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoteType {
    Propose,
    Prevote,
    Precommit,
    Commit,
}

/// 🏆 Consensus result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusResult {
    pub algorithm_used: ConsensusAlgorithm,
    pub block_hash: [u8; 32],
    pub votes: Vec<ConsensusVote>,
    pub finality_time: u64, // milliseconds
    pub validator_participation: f64, // percentage
}

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
                // Security increases with validator count, but caps for speed
                ((*validator_count as f64 / 21.0).min(1.0) * 0.7) + 0.1
            }
            Self::ByzantineFaultTolerance { required_confirmations, .. } => {
                // High security with BFT
                ((*required_confirmations as f64 / 100.0).min(1.0) * 0.3) + 0.7
            }
            Self::ProofOfAuthority { authorities } => {
                // Security depends on authority trust
                (authorities.len() as f64 / 10.0).min(0.8)
            }
            Self::HybridStakeWork { stake_weight, .. } => {
                // Balanced security
                0.5 + (*stake_weight * 0.3)
            }
        }
    }

    /// 🌐 Get decentralization score (0.0 to 1.0)
    pub fn decentralization_score(&self) -> f64 {
        match self {
            Self::DelegatedProofOfStake { validator_count, .. } => {
                // Limited decentralization for speed
                (*validator_count as f64 / 50.0).min(0.7)
            }
            Self::ByzantineFaultTolerance { required_confirmations, .. } => {
                // High decentralization with many validators
                ((*required_confirmations as f64 - 1.0) / 200.0).min(0.95)
            }
            Self::ProofOfAuthority { authorities } => {
                // Low decentralization, controlled by authorities
                (authorities.len() as f64 / 20.0).min(0.5)
            }
            Self::HybridStakeWork { stake_weight, .. } => {
                // Moderate decentralization
                0.4 + (*stake_weight * 0.4)
            }
        }
    }
}

impl Validator {
    /// 👑 Create new validator
    pub fn new(keypair: &QuantumKeyPair, stake: u64) -> Self {
        Self {
            public_key: keypair.public_key().to_vec(),
            stake,
            reputation: 1.0, // Start with perfect reputation
            is_active: true,
            last_activity: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// 📈 Update validator reputation
    pub fn update_reputation(&mut self, performance_score: f64) {
        // Exponential moving average for reputation
        self.reputation = (self.reputation * 0.9) + (performance_score * 0.1);
        self.reputation = self.reputation.max(0.0).min(1.0);
    }

    /// ⏰ Update last activity timestamp
    pub fn update_activity(&mut self) {
        self.last_activity = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// 🏋️ Get voting power based on stake and reputation
    pub fn voting_power(&self) -> f64 {
        (self.stake as f64) * self.reputation
    }

    /// ⏳ Check if validator is recently active (within last hour)
    pub fn is_recently_active(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        now - self.last_activity < 3600 // 1 hour
    }
}

impl ConsensusVote {
    /// 🗳️ Create new consensus vote
    pub fn new(
        validator_keypair: &QuantumKeyPair,
        block_hash: [u8; 32],
        vote_type: VoteType,
    ) -> Result<Self> {
        let vote_data = bincode::serialize(&(block_hash, &vote_type))?;
        let signature = validator_keypair.sign(&vote_data)?;
        
        Ok(Self {
            validator_id: validator_keypair.public_key().to_vec(),
            block_hash,
            vote_type,
            signature,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        })
    }

    /// ✅ Verify vote signature
    pub fn verify_signature(&self) -> bool {
        let vote_data = bincode::serialize(&(self.block_hash, &self.vote_type))
            .unwrap_or_default();
        
        self.signature.verify(&vote_data, &self.validator_id)
    }

    /// ⏰ Check if vote is recent (within timeout)
    pub fn is_recent(&self, timeout_ms: u64) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        
        now - self.timestamp < timeout_ms
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

    #[test]
    fn test_validator_creation() {
        let keypair = QuantumKeyPair::generate();
        let mut validator = Validator::new(&keypair, 1000);
        
        assert_eq!(validator.stake, 1000);
        assert_eq!(validator.reputation, 1.0);
        assert!(validator.is_active);
        assert!(validator.is_recently_active());
        
        // Test reputation update
        validator.update_reputation(0.5);
        assert!(validator.reputation < 1.0);
        
        println!("👑 Validator tests passed!");
        println!("   Stake: {}", validator.stake);
        println!("   Reputation: {:.2}", validator.reputation);
        println!("   Voting power: {:.1}", validator.voting_power());
    }

    #[test]
    fn test_consensus_vote() {
        let keypair = QuantumKeyPair::generate();
        let block_hash = [1u8; 32];
        
        let vote = ConsensusVote::new(&keypair, block_hash, VoteType::Propose).unwrap();
        
        assert_eq!(vote.block_hash, block_hash);
        assert!(vote.verify_signature());
        assert!(vote.is_recent(10_000)); // Within 10 seconds
        
        println!("🗳️ Consensus vote tests passed!");
        println!("   Vote verified: ✅");
        println!("   Vote type: {:?}", vote.vote_type);
    }
}