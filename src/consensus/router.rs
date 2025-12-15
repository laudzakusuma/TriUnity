use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ConsensusRouter {
    network_metrics: NetworkMetrics,
    ai_model: AIModel,
    _performance_history: Vec<PerformanceSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    pub current_tps: u64,
    pub network_latency: u64,
    pub validator_count: usize,
    pub attack_probability: f64,
    pub congestion_level: f64,
    pub memory_usage: f64,
    pub cpu_usage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusPath {
    FastLane {
        expected_tps: u64,
        finality_time: u64,
        validator_count: usize,
    },
    SecureLane {
        validator_threshold: usize,
        security_level: f64,
        decentralization_score: f64,
    },
    HybridPath {
        fast_percentage: f64,
        secure_percentage: f64,
        adaptive_threshold: f64,
    },
    EmergencyMode {
        fallback_validators: usize,
        security_override: bool,
    },
}

#[derive(Debug, Clone)]
pub struct AIModel {
    weights: HashMap<String, f64>,
    _learning_rate: f64,
    _confidence_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformancePrediction {
    pub throughput: u64,
    pub latency: u64,
    pub security_score: f64,
    pub decentralization_score: f64,
    pub confidence: f64,
    pub energy_efficiency: f64,
}

#[derive(Debug, Clone)]
pub struct PerformanceSnapshot {
    pub timestamp: u64,
    pub path_used: ConsensusPath,
    pub actual_performance: PerformancePrediction,
    pub predicted_performance: PerformancePrediction,
}

impl ConsensusRouter {
    pub fn new() -> Self {
        Self {
            network_metrics: NetworkMetrics::default(),
            ai_model: AIModel::new(),
            _performance_history: Vec::new(),
        }
    }

    pub fn update_metrics(&mut self, metrics: NetworkMetrics) {
        self.network_metrics = metrics;
        self.ai_model.adapt_to_conditions(&self.network_metrics);
    }

    pub fn select_optimal_path(&self) -> ConsensusPath {
        let metrics = &self.network_metrics;
        let confidence = self.ai_model.calculate_confidence(metrics);
        
        if metrics.attack_probability > 0.8 || metrics.congestion_level > 0.95 {
            return ConsensusPath::EmergencyMode {
                fallback_validators: (metrics.validator_count * 3 / 4).max(10),
                security_override: true,
            };
        }
        
        if metrics.attack_probability > 0.4 || confidence < 0.6 {
            return ConsensusPath::SecureLane {
                validator_threshold: (metrics.validator_count * 2 / 3),
                security_level: 0.95,
                decentralization_score: 0.9,
            };
        }
        
        if metrics.congestion_level > 0.7 && metrics.attack_probability < 0.2 {
            return ConsensusPath::FastLane {
                expected_tps: 100_000,
                finality_time: 100,
                validator_count: (metrics.validator_count / 4).max(21),
            };
        }
        
        ConsensusPath::HybridPath {
            fast_percentage: 0.7 - (metrics.attack_probability * 0.5),
            secure_percentage: 0.3 + (metrics.attack_probability * 0.5),
            adaptive_threshold: confidence,
        }
    }

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

    pub fn network_status(&self) -> &NetworkMetrics {
        &self.network_metrics
    }

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
    fn calculate_confidence(&self, metrics: &NetworkMetrics) -> f64 {
        let stability = 1.0 - metrics.congestion_level;
        let security = 1.0 - metrics.attack_probability;
        let resources = (2.0 - metrics.cpu_usage - metrics.memory_usage) / 2.0;
        
        (stability + security + resources) / 3.0
    }
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
        println!("Consensus router created successfully!");
    }

    #[test]
    fn test_path_selection() {
        let mut router = ConsensusRouter::new();
        
        let path = router.select_optimal_path();
        println!("Selected path: {:?}", path);
        
        // Test high attack probability
        let attack_metrics = NetworkMetrics {
            attack_probability: 0.9,
            ..Default::default()
        };
        router.update_metrics(attack_metrics);
        
        let emergency_path = router.select_optimal_path();
        matches!(emergency_path, ConsensusPath::EmergencyMode { .. });
        
        println!("Emergency mode activated under attack!");
    }
}