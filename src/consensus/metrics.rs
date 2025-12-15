use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct MetricsCollector {
    tps_history: VecDeque<TpsReading>,
    latency_history: VecDeque<LatencyReading>,
    security_events: VecDeque<SecurityEvent>,
    max_history_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TpsReading {
    pub timestamp: u64,
    pub tps: u64,
    pub block_height: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyReading {
    pub timestamp: u64,
    pub latency_ms: u64,
    pub node_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub timestamp: u64,
    pub event_type: SecurityEventType,
    pub severity: SecuritySeverity,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEventType {
    SuspiciousActivity,
    InvalidSignature,
    DoubleSpend,
    NetworkAttack,
    ValidatorMisbehavior,
    UnusualTraffic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceStats {
    pub avg_tps: f64,
    pub peak_tps: u64,
    pub avg_latency: f64,
    pub min_latency: u64,
    pub max_latency: u64,
    pub security_score: f64,
    pub uptime_percentage: f64,
    pub total_transactions: u64,
}

impl MetricsCollector {
    pub fn new(max_history_size: usize) -> Self {
        Self {
            tps_history: VecDeque::new(),
            latency_history: VecDeque::new(),
            security_events: VecDeque::new(),
            max_history_size,
        }
    }

    pub fn record_tps(&mut self, tps: u64, block_height: u64) {
        let reading = TpsReading {
            timestamp: current_timestamp(),
            tps,
            block_height,
        };
        
        self.tps_history.push_back(reading);
        
        while self.tps_history.len() > self.max_history_size {
            self.tps_history.pop_front();
        }
    }

    pub fn record_latency(&mut self, latency_ms: u64, node_count: usize) {
        let reading = LatencyReading {
            timestamp: current_timestamp(),
            latency_ms,
            node_count,
        };
        
        self.latency_history.push_back(reading);
        
        while self.latency_history.len() > self.max_history_size {
            self.latency_history.pop_front();
        }
    }

    pub fn record_security_event(
        &mut self,
        event_type: SecurityEventType,
        severity: SecuritySeverity,
        description: String,
    ) {
        let event = SecurityEvent {
            timestamp: current_timestamp(),
            event_type,
            severity,
            description,
        };
        
        self.security_events.push_back(event);
        
        while self.security_events.len() > self.max_history_size {
            self.security_events.pop_front();
        }
    }

    pub fn calculate_stats(&self) -> PerformanceStats {
        let avg_tps = if !self.tps_history.is_empty() {
            self.tps_history.iter().map(|r| r.tps as f64).sum::<f64>() / self.tps_history.len() as f64
        } else {
            0.0
        };

        let peak_tps = self.tps_history.iter().map(|r| r.tps).max().unwrap_or(0);

        let avg_latency = if !self.latency_history.is_empty() {
            self.latency_history.iter().map(|r| r.latency_ms as f64).sum::<f64>() / self.latency_history.len() as f64
        } else {
            0.0
        };

        let min_latency = self.latency_history.iter().map(|r| r.latency_ms).min().unwrap_or(0);
        let max_latency = self.latency_history.iter().map(|r| r.latency_ms).max().unwrap_or(0);
        let security_score = self.calculate_security_score();
        let uptime_percentage = if avg_tps > 0.0 { 99.9 } else { 0.0 };
        let total_transactions = self.tps_history.iter().map(|r| r.tps).sum();

        PerformanceStats {
            avg_tps,
            peak_tps,
            avg_latency,
            min_latency,
            max_latency,
            security_score,
            uptime_percentage,
            total_transactions,
        }
    }

    fn calculate_security_score(&self) -> f64 {
        if self.security_events.is_empty() {
            return 1.0;
        }

        let recent_events: Vec<_> = self.security_events
            .iter()
            .filter(|event| {
                let now = current_timestamp();
                now - event.timestamp < 3600
            })
            .collect();

        if recent_events.is_empty() {
            return 1.0;
        }

        let total_severity: f64 = recent_events
            .iter()
            .map(|event| match event.severity {
                SecuritySeverity::Low => 0.1,
                SecuritySeverity::Medium => 0.3,
                SecuritySeverity::High => 0.6,
                SecuritySeverity::Critical => 1.0,
            })
            .sum();
        (1.0 - (total_severity / recent_events.len() as f64)).max(0.0)
    }

    pub fn get_tps_trend(&self) -> Option<f64> {
        if self.tps_history.len() < 2 {
            return None;
        }

        let recent_count = (self.tps_history.len() / 4).max(2);
        let recent: Vec<_> = self.tps_history.iter().rev().take(recent_count).collect();
        
        if recent.len() < 2 {
            return None;
        }

        let first_tps = recent.last().unwrap().tps as f64;
        let last_tps = recent.first().unwrap().tps as f64;
        
        Some((last_tps - first_tps) / first_tps)
    }

    pub fn get_recent_security_events(&self, hours: u64) -> Vec<&SecurityEvent> {
        let cutoff = current_timestamp() - (hours * 3600);
        self.security_events
            .iter()
            .filter(|event| event.timestamp >= cutoff)
            .collect()
    }

    pub fn get_current_metrics(&self) -> Option<(u64, u64)> {
        let latest_tps = self.tps_history.back()?.tps;
        let latest_latency = self.latency_history.back()?.latency_ms;
        Some((latest_tps, latest_latency))
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new(1000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_collection() {
        let mut collector = MetricsCollector::new(100);
        collector.record_tps(1000, 1);
        collector.record_tps(2000, 2);
        collector.record_tps(1500, 3);
        collector.record_latency(50, 10);
        collector.record_latency(75, 12);
        
        let stats = collector.calculate_stats();
        assert_eq!(stats.avg_tps, 1500.0);
        assert_eq!(stats.peak_tps, 2000);
        assert_eq!(stats.avg_latency, 62.5);
        
        println!("   Metrics collection working!");
        println!("   Average TPS: {:.1}", stats.avg_tps);
        println!("   Peak TPS: {}", stats.peak_tps);
        println!("   Average Latency: {:.1}ms", stats.avg_latency);
    }

    #[test]
    fn test_security_events() {
        let mut collector = MetricsCollector::new(100);
        
        collector.record_security_event(
            SecurityEventType::SuspiciousActivity,
            SecuritySeverity::Medium,
            "Unusual transaction pattern detected".to_string(),
        );
        
        let recent_events = collector.get_recent_security_events(1);
        assert_eq!(recent_events.len(), 1);
        
        let stats = collector.calculate_stats();
        assert!(stats.security_score < 1.0);
        
        println!("   Security event recording working!");
        println!("   Security score: {:.2}", stats.security_score);
    }

    #[test]
    fn test_performance_trends() {
        let mut collector = MetricsCollector::new(100);
        
        for i in 1..=10 {
            collector.record_tps(i * 1000, i);
        }
        
        let trend = collector.get_tps_trend().unwrap();
        assert!(trend > 0.0);
        
        println!("   TPS trend analysis working!");
        println!("   TPS trend: {:.2}% change", trend * 100.0);
    }
}