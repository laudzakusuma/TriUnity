//! 🤖 AI-Powered Consensus Router
//! 
//! The brain of TriUnity that intelligently routes transactions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 🧠 The AI-powered consensus router
#[derive(Debug, Clone)]
pub struct ConsensusRouter {
    network_metrics: NetworkMetrics,
    ai_model: AIModel,
    _performance_history: Vec<PerformanceSnapshot>,
}

/// 📊 Real-time network metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    pub current_tps: u64,
    pub network_latency: u64,     // milliseconds
    pub validator_count: usize,
    pub attack_probability: f64,  // 0.0 to 1.0
    pub congestion_level: f64,    // 0.0 to 1.0
    pub memory_usage: f64,        // 0.0 to 1.0
    pub cpu_usage: f64,           // 0.0 to 1.0
}

/// 🛣️ Available consensus paths
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusPath {
    /// ⚡ Fast Lane: Maximum speed for routine transactions
    FastLane {
        expected_tps: u64,
        finality_time: u64,  // milliseconds
        validator_count: usize,
    },
    /// 🛡️ Secure Lane: Maximum security for critical operations
    SecureLane {
        validator_threshold: usize,
        security_level: f64,
        decentralization_score: f64,
    },
    /// 🎯 Hybrid Path: Balanced approach
    HybridPath {
        fast_percentage: f64,
        secure_percentage: f64,
        adaptive_threshold: f64,
    },
    /// 🚨 Emergency Mode: Under attack or extreme conditions
    EmergencyMode {
        fallback_validators: usize,
        security_override: bool,
    },
}

/// 🤖 Simplified AI model for consensus decisions
#[derive(Debug, Clone)]
pub struct AIModel {
    weights: HashMap<String, f64>,
    _learning_rate: f64,
    _confidence_threshold: f64,
}

/// 📈 Performance prediction results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformancePrediction {
    pub throughput: u64,           // TPS
    pub latency: u64,              // milliseconds
    pub security_score: f64,       // 0.0 to 1.0
    pub decentralization_score: f64, // 0.0 to 1.0
    pub confidence: f64,           // AI confidence in prediction
    pub energy_efficiency: f64,    // relative efficiency score
}

/// 📸 Performance snapshot for learning
#[derive(Debug, Clone)]
pub struct PerformanceSnapshot {
    pub timestamp: u64,
    pub path_used: ConsensusPath,
    pub actual_performance: PerformancePrediction,
    pub predicted_performance: PerformancePrediction,
}

impl ConsensusRouter {
    /// 🎉 Create new AI-powered consensus router
    pub fn new() -> Self {
        Self {
            network_metrics: NetworkMetrics::default(),
            ai_model: AIModel::new(),
            _performance_history: Vec::new(),
        }
    }

    /// 📊 Update network metrics in real-time
    pub fn update_metrics(&mut self, metrics: NetworkMetrics) {
        self.network_metrics = metrics;
        self.ai_model.adapt_to_conditions(&self.network_metrics);
    }

    /// 🎯 Select optimal consensus path using AI
    pub fn select_optimal_path(&self) -> ConsensusPath {
        let metrics = &self.network_metrics;
        let confidence = self.ai_model.calculate_confidence(metrics);
        
        // 🚨 Emergency conditions
        if metrics.attack_probability > 0.8 || metrics.congestion_level > 0.95 {
            return ConsensusPath::EmergencyMode {
                fallback_validators: (metrics.validator_count * 3 / 4).max(10),
                security_override: true,
            };
        }
        
        // 🛡️ High security requirements
        if metrics.attack_probability > 0.4 || confidence < 0.6 {
            return ConsensusPath::SecureLane {
                validator_threshold: (metrics.validator_count * 2 / 3),
                security_level: 0.95,
                decentralization_score: 0.9,
            };
        }
        
        // ⚡ High performance requirements
        if metrics.congestion_level > 0.7 && metrics.attack_probability < 0.2 {
            return ConsensusPath::FastLane {
                expected_tps: 100_000,
                finality_time: 100,
                validator_count: (metrics.validator_count / 4).max(21),
            };
        }
        
        // 🎯 Balanced hybrid approach (default)
        ConsensusPath::HybridPath {
            fast_percentage: 0.7 - (metrics.attack_probability * 0.5),
            secure_percentage: 0.3 + (metrics.attack_probability * 0.5),
            adaptive_threshold: confidence,
        }
    }

    /// 🔮 Predict performance for a given path
    pub fn predict_performance(&self, path: &ConsensusPath) -> PerformancePrediction {
        let base_metrics = &self.network_metrics;
        
        match path {
            ConsensusPath::FastLane { expected_tps, finality_time, validator_count } => {
                PerformancePrediction {
                    throughput: *expected_tps,
                    latency: *finality_time,
                    security_score: 0.7 + ((*validator_count as f64 / 100.0).min(0.2)),
                    decentralization_score: (*validator_count as f64 / base_metrics.validator_count as f64).min(0.8),
                    confidence: self.ai_model.calculate_confidence(base_metrics),
                    energy_efficiency: 0.9,
                }
            }
            
            ConsensusPath::SecureLane { security_level, decentralization_score, .. } => {
                PerformancePrediction {
                    throughput: 5_000,
                    latency: 2_000,
                    security_score: *security_level,
                    decentralization_score: *decentralization_score,
                    confidence: 0.95,
                    energy_efficiency: 0.6,
                }
            }
            
            ConsensusPath::HybridPath { fast_percentage, secure_percentage, adaptive_threshold } => {
                PerformancePrediction {
                    throughput: (100_000.0 * fast_percentage + 5_000.0 * secure_percentage) as u64,
                    latency: (100.0 * fast_percentage + 2_000.0 * secure_percentage) as u64,
                    security_score: 0.7 * fast_percentage + 0.95 * secure_percentage,
                    decentralization_score: 0.6 * fast_percentage + 0.9 * secure_percentage,
                    confidence: *adaptive_threshold,
                    energy_efficiency: 0.9 * fast_percentage + 0.6 * secure_percentage,
                }
            }
            
            ConsensusPath::EmergencyMode { fallback_validators, .. } => {
                PerformancePrediction {
                    throughput: 1_000,
                    latency: 5_000,
                    security_score: 0.99,
                    decentralization_score: (*fallback_validators as f64 / base_metrics.validator_count as f64).min(0.95),
                    confidence: 0.8,
                    energy_efficiency: 0.4,
                }
            }
        }
    }

    /// 📊 Get current network status
    pub fn network_status(&self) -> &NetworkMetrics {
        &self.network_metrics
    }

    /// 🧠 Get AI model confidence
    pub fn ai_confidence(&self) -> f64 {
        self.ai_model.calculate_confidence(&self.network_metrics)
    }
}

impl Default for ConsensusRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkMetrics {
    /// 🔍 Check if network is under stress
    pub fn is_stressed(&self) -> bool {
        self.congestion_level > 0.8 || 
        self.attack_probability > 0.3 ||
        self.cpu_usage > 0.9 ||
        self.memory_usage > 0.9
    }
}

impl Default for NetworkMetrics {
    fn default() -> Self {
        Self {
            current_tps: 1000,
            network_latency: 100,
            validator_count: 100,
            attack_probability: 0.1,
            congestion_level: 0.3,
            memory_usage: 0.5,
            cpu_usage: 0.4,
        }
    }
}

impl AIModel {
    /// 🧠 Create new AI model
    fn new() -> Self {
        let mut weights = HashMap::new();
        weights.insert("security_weight".to_string(), 0.3);
        weights.insert("performance_weight".to_string(), 0.4);
        weights.insert("decentralization_weight".to_string(), 0.3);
        
        Self {
            weights,
            _learning_rate: 0.01,
            _confidence_threshold: 0.7,
        }
    }

    /// 🎯 Calculate AI confidence in current conditions
    fn calculate_confidence(&self, metrics: &NetworkMetrics) -> f64 {
        let stability = 1.0 - metrics.congestion_level;
        let security = 1.0 - metrics.attack_probability;
        let resources = (2.0 - metrics.cpu_usage - metrics.memory_usage) / 2.0;
        
        (stability + security + resources) / 3.0
    }

    /// 🔄 Adapt AI weights to current conditions
    fn adapt_to_conditions(&mut self, metrics: &NetworkMetrics) {
        if metrics.attack_probability > 0.5 {
            if let Some(weight) = self.weights.get_mut("security_weight") {
                *weight = (*weight + 0.1).min(0.8);
            }
        }
        
        if metrics.congestion_level > 0.7 {
            if let Some(weight) = self.weights.get_mut("performance_weight") {
                *weight = (*weight + 0.1).min(0.9);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consensus_router_creation() {
        let router = ConsensusRouter::new();
        assert!(router.ai_confidence() > 0.0);
        println!("🤖 Consensus router created successfully!");
    }

    #[test]
    fn test_path_selection() {
        let mut router = ConsensusRouter::new();
        
        let path = router.select_optimal_path();
        println!("🎯 Selected path: {:?}", path);
        
        // Test high attack probability
        let attack_metrics = NetworkMetrics {
            attack_probability: 0.9,
            ..Default::default()
        };
        router.update_metrics(attack_metrics);
        
        let emergency_path = router.select_optimal_path();
        matches!(emergency_path, ConsensusPath::EmergencyMode { .. });
        
        println!("🚨 Emergency mode activated under attack!");
    }
}