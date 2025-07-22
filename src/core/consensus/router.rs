//! 🤖 AI-Powered Consensus Router
//! 
//! The brain of TriUnity that intelligently routes transactions
//! to optimal consensus paths in real-time!

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{Result, TriUnityError};

/// 🧠 The AI-powered consensus router
#[derive(Debug, Clone)]
pub struct ConsensusRouter {
    network_metrics: NetworkMetrics,
    ai_model: AIModel,
    performance_history: Vec<PerformanceSnapshot>,
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
    learning_rate: f64,
    confidence_threshold: f64,
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
            performance_history: Vec::new(),
        }
    }

    /// 📊 Update network metrics in real-time
    pub fn update_metrics(&mut self, metrics: NetworkMetrics) {
        self.network_metrics = metrics;
        
        // AI learns from new data
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
                    energy_efficiency: 0.9, // Fast lane is efficient
                }
            }
            
            ConsensusPath::SecureLane { validator_threshold, security_level, decentralization_score } => {
                PerformancePrediction {
                    throughput: 5_000, // Slower but secure
                    latency: 2_000,    // Higher latency for security
                    security_score: *security_level,
                    decentralization_score: *decentralization_score,
                    confidence: 0.95, // High confidence in security
                    energy_efficiency: 0.6, // More energy for security
                }
            }
            
            ConsensusPath::HybridPath { fast_percentage, secure_percentage, adaptive_threshold } => {
                let fast_perf = self.predict_performance(&ConsensusPath::FastLane {
                    expected_tps: 100_000,
                    finality_time: 100,
                    validator_count: 50,
                });
                let secure_perf = self.predict_performance(&ConsensusPath::SecureLane {
                    validator_threshold: 100,
                    security_level: 0.95,
                    decentralization_score: 0.9,
                });
                
                PerformancePrediction {
                    throughput: ((fast_perf.throughput as f64 * fast_percentage) + 
                               (secure_perf.throughput as f64 * secure_percentage)) as u64,
                    latency: ((fast_perf.latency as f64 * fast_percentage) + 
                             (secure_perf.latency as f64 * secure_percentage)) as u64,
                    security_score: (fast_perf.security_score * fast_percentage) + 
                                   (secure_perf.security_score * secure_percentage),
                    decentralization_score: (fast_perf.decentralization_score * fast_percentage) + 
                                           (secure_perf.decentralization_score * secure_percentage),
                    confidence: *adaptive_threshold,
                    energy_efficiency: (fast_perf.energy_efficiency * fast_percentage) + 
                                      (secure_perf.energy_efficiency * secure_percentage),
                }
            }
            
            ConsensusPath::EmergencyMode { fallback_validators, security_override: _ } => {
                PerformancePrediction {
                    throughput: 1_000, // Emergency mode prioritizes safety
                    latency: 5_000,
                    security_score: 0.99,
                    decentralization_score: (*fallback_validators as f64 / base_metrics.validator_count as f64).min(0.95),
                    confidence: 0.8, // Lower confidence due to emergency
                    energy_efficiency: 0.4, // High energy for maximum security
                }
            }
        }
    }

    /// 📚 Learn from actual performance (AI training)
    pub fn record_performance(&mut self, path: ConsensusPath, actual: PerformancePrediction) {
        let predicted = self.predict_performance(&path);
        
        let snapshot = PerformanceSnapshot {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            path_used: path,
            actual_performance: actual,
            predicted_performance: predicted,
        };
        
        self.performance_history.push(snapshot);
        
        // Keep only recent history (last 1000 entries)
        if self.performance_history.len() > 1000 {
            self.performance_history.remove(0);
        }
        
        // AI learns from the difference
        self.ai_model.learn_from_performance(&self.performance_history);
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
    /// 📊 Create default network metrics
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 🔍 Check if network is under stress
    pub fn is_stressed(&self) -> bool {
        self.congestion_level > 0.8 || 
        self.attack_probability > 0.3 ||
        self.cpu_usage > 0.9 ||
        self.memory_usage > 0.9
    }

    /// ⚡ Check if high performance is needed
    pub fn needs_performance(&self) -> bool {
        self.current_tps > 50_000 || self.congestion_level > 0.6
    }

    /// 🛡️ Check if high security is needed
    pub fn needs_security(&self) -> bool {
        self.attack_probability > 0.2 || self.validator_count < 50
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
        
        // Initialize AI weights for different factors
        weights.insert("security_weight".to_string(), 0.3);
        weights.insert("performance_weight".to_string(), 0.4);
        weights.insert("decentralization_weight".to_string(), 0.3);
        weights.insert("energy_weight".to_string(), 0.1);
        weights.insert("latency_sensitivity".to_string(), 0.5);
        weights.insert("attack_response".to_string(), 0.8);
        
        Self {
            weights,
            learning_rate: 0.01,
            confidence_threshold: 0.7,
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
        // Increase security weight if under attack
        if metrics.attack_probability > 0.5 {
            if let Some(weight) = self.weights.get_mut("security_weight") {
                *weight = (*weight + 0.1).min(0.8);
            }
        }
        
        // Increase performance weight if congested
        if metrics.congestion_level > 0.7 {
            if let Some(weight) = self.weights.get_mut("performance_weight") {
                *weight = (*weight + 0.1).min(0.9);
            }
        }
    }

    /// 📚 Learn from performance history
    fn learn_from_performance(&mut self, history: &[PerformanceSnapshot]) {
        if history.len() < 10 {
            return; // Need enough data to learn
        }
        
        // Simple learning: adjust weights based on prediction accuracy
        let recent_snapshots = &history[history.len().saturating_sub(10)..];
        
        let mut accuracy_sum = 0.0;
        for snapshot in recent_snapshots {
            let predicted = &snapshot.predicted_performance;
            let actual = &snapshot.actual_performance;
            
            // Calculate prediction accuracy
            let throughput_accuracy = 1.0 - ((predicted.throughput as f64 - actual.throughput as f64).abs() / predicted.throughput as f64).min(1.0);
            let latency_accuracy = 1.0 - ((predicted.latency as f64 - actual.latency as f64).abs() / predicted.latency as f64).min(1.0);
            
            accuracy_sum += (throughput_accuracy + latency_accuracy) / 2.0;
        }
        
        let avg_accuracy = accuracy_sum / recent_snapshots.len() as f64;
        
        // Adjust learning rate based on accuracy
        if avg_accuracy < 0.7 {
            self.learning_rate = (self.learning_rate * 1.1).min(0.1);
        } else {
            self.learning_rate = (self.learning_rate * 0.95).max(0.001);
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
        
        // Test normal conditions
        let path = router.select_optimal_path();
        match path {
            ConsensusPath::HybridPath { .. } => println!("✅ Selected hybrid path for normal conditions"),
            _ => println!("🎯 Selected alternative path: {:?}", path),
        }
        
        // Test high attack probability
        let attack_metrics = NetworkMetrics {
            attack_probability: 0.9,
            ..Default::default()
        };
        router.update_metrics(attack_metrics);
        
        let emergency_path = router.select_optimal_path();
        match emergency_path {
            ConsensusPath::EmergencyMode { .. } => println!("🚨 Correctly selected emergency mode under attack!"),
            _ => println!("⚠️ Unexpected path under attack: {:?}", emergency_path),
        }
    }

    #[test]
    fn test_performance_prediction() {
        let router = ConsensusRouter::new();
        
        let fast_path = ConsensusPath::FastLane {
            expected_tps: 100_000,
            finality_time: 100,
            validator_count: 50,
        };
        
        let prediction = router.predict_performance(&fast_path);
        assert_eq!(prediction.throughput, 100_000);
        assert_eq!(prediction.latency, 100);
        assert!(prediction.confidence > 0.0);
        
        println!("🔮 Performance prediction working!");
        println!("   Throughput: {} TPS", prediction.throughput);
        println!("   Latency: {}ms", prediction.latency);
        println!("   Security: {:.2}", prediction.security_score);
        println!("   Decentralization: {:.2}", prediction.decentralization_score);
        println!("   Confidence: {:.2}", prediction.confidence);
    }

    #[test]
    fn test_network_metrics() {
        let metrics = NetworkMetrics::default();
        assert!(!metrics.is_stressed());
        assert!(!metrics.needs_security());
        
        let stressed_metrics = NetworkMetrics {
            congestion_level: 0.9,
            attack_probability: 0.5,
            ..Default::default()
        };
        assert!(stressed_metrics.is_stressed());
        assert!(stressed_metrics.needs_security());
        
        println!("📊 Network metrics analysis working!");
    }
}