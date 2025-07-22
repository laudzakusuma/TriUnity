//! 🔍 Node Discovery System
//! 
//! Automatic discovery and connection to TriUnity network nodes

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::collections::HashMap;

/// 🔍 Node discovery service
#[derive(Debug)]
pub struct NodeDiscovery {
    bootstrap_nodes: Vec<SocketAddr>,
    discovered_nodes: HashMap<Vec<u8>, DiscoveredNode>,
    discovery_interval: u64, // seconds
}

/// 📍 Discovered node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredNode {
    pub node_id: Vec<u8>,
    pub address: SocketAddr,
    pub last_seen: u64,
    pub response_time: u64, // milliseconds
    pub trust_score: f64,   // 0.0 to 1.0
}

/// 🎯 Discovery methods
#[derive(Debug, Clone)]
pub enum DiscoveryMethod {
    Bootstrap,
    DHT, // Distributed Hash Table
    DNS, // DNS seeds
    Gossip, // Peer gossip
}

impl NodeDiscovery {
    /// 🚀 Create new node discovery service
    pub fn new(bootstrap_nodes: Vec<SocketAddr>) -> Self {
        Self {
            bootstrap_nodes,
            discovered_nodes: HashMap::new(),
            discovery_interval: 300, // 5 minutes
        }
    }

    /// 🔍 Start discovery process
    pub async fn start_discovery(&mut self) -> Result<(), String> {
        println!("🔍 Starting node discovery...");
        
        // Connect to bootstrap nodes first
        for &bootstrap_addr in &self.bootstrap_nodes.clone() {
            self.discover_from_bootstrap(bootstrap_addr).await?;
        }
        
        println!("✅ Discovery started with {} bootstrap nodes", self.bootstrap_nodes.len());
        Ok(())
    }

    /// 🌱 Discover nodes from bootstrap
    async fn discover_from_bootstrap(&mut self, bootstrap_addr: SocketAddr) -> Result<(), String> {
        println!("🌱 Connecting to bootstrap node: {}", bootstrap_addr);
        
        // TODO: Implement actual network connection
        // For now, simulate successful discovery
        let fake_node = DiscoveredNode {
            node_id: vec![1, 2, 3, 4, 5, 6, 7, 8],
            address: bootstrap_addr,
            last_seen: current_timestamp(),
            response_time: 50,
            trust_score: 1.0,
        };
        
        self.discovered_nodes.insert(fake_node.node_id.clone(), fake_node);
        Ok(())
    }

    /// 💬 Discover nodes through gossip
    pub fn discover_from_gossip(&mut self, peer_nodes: Vec<DiscoveredNode>) {
        println!("💬 Discovering {} nodes from gossip", peer_nodes.len());
        
        for node in peer_nodes {
            if !self.discovered_nodes.contains_key(&node.node_id) {
                println!("📍 Discovered new node: {}", hex::encode(&node.node_id[..4]));
                self.discovered_nodes.insert(node.node_id.clone(), node);
            }
        }
    }

    /// 📊 Get discovery statistics
    pub fn get_discovery_stats(&self) -> DiscoveryStats {
        let total_discovered = self.discovered_nodes.len();
        let trusted_nodes = self.discovered_nodes.values()
            .filter(|node| node.trust_score > 0.7)
            .count();
        
        let avg_response_time = if !self.discovered_nodes.is_empty() {
            self.discovered_nodes.values()
                .map(|node| node.response_time)
                .sum::<u64>() / self.discovered_nodes.len() as u64
        } else {
            0
        };

        DiscoveryStats {
            total_discovered,
            trusted_nodes,
            avg_response_time,
            bootstrap_count: self.bootstrap_nodes.len(),
        }
    }

    /// 🎯 Get best nodes for connection
    pub fn get_best_nodes(&self, count: usize) -> Vec<&DiscoveredNode> {
        let mut nodes: Vec<_> = self.discovered_nodes.values().collect();
        
        // Sort by trust score and response time
        nodes.sort_by(|a, b| {
            let score_a = a.trust_score - (a.response_time as f64 / 1000.0);
            let score_b = b.trust_score - (b.response_time as f64 / 1000.0);
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        nodes.into_iter().take(count).collect()
    }

    /// 🧹 Clean up stale nodes
    pub fn cleanup_stale_nodes(&mut self, max_age_hours: u64) {
        let cutoff = current_timestamp() - (max_age_hours * 3600);
        let initial_count = self.discovered_nodes.len();
        
        self.discovered_nodes.retain(|_, node| node.last_seen >= cutoff);
        
        let removed = initial_count - self.discovered_nodes.len();
        if removed > 0 {
            println!("🧹 Removed {} stale nodes", removed);
        }
    }

    /// 📈 Update node trust score
    pub fn update_node_trust(&mut self, node_id: &[u8], performance_score: f64) {
        if let Some(node) = self.discovered_nodes.get_mut(node_id) {
            // Exponential moving average
            node.trust_score = (node.trust_score * 0.9) + (performance_score * 0.1);
            node.trust_score = node.trust_score.max(0.0).min(1.0);
        }
    }
}

/// 📊 Discovery statistics
#[derive(Debug, Clone)]
pub struct DiscoveryStats {
    pub total_discovered: usize,
    pub trusted_nodes: usize,
    pub avg_response_time: u64,
    pub bootstrap_count: usize,
}

/// 🕐 Get current timestamp
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_discovery_creation() {
        let bootstrap_nodes = vec![
            "127.0.0.1:8080".parse().unwrap(),
            "127.0.0.1:8081".parse().unwrap(),
        ];
        
        let discovery = NodeDiscovery::new(bootstrap_nodes.clone());
        assert_eq!(discovery.bootstrap_nodes.len(), 2);
        assert_eq!(discovery.discovered_nodes.len(), 0);
        
        println!("🔍 Node discovery created successfully!");
    }

    #[test]
    fn test_gossip_discovery() {
        let mut discovery = NodeDiscovery::new(vec![]);
        
        let gossip_nodes = vec![
            DiscoveredNode {
                node_id: vec![1, 2, 3, 4],
                address: "192.168.1.10:8080".parse().unwrap(),
                last_seen: current_timestamp(),
                response_time: 25,
                trust_score: 0.9,
            },
            DiscoveredNode {
                node_id: vec![5, 6, 7, 8],
                address: "192.168.1.11:8080".parse().unwrap(),
                last_seen: current_timestamp(),
                response_time: 75,
                trust_score: 0.8,
            },
        ];
        
        discovery.discover_from_gossip(gossip_nodes);
        assert_eq!(discovery.discovered_nodes.len(), 2);
        
        let stats = discovery.get_discovery_stats();
        assert_eq!(stats.total_discovered, 2);
        assert_eq!(stats.trusted_nodes, 2);
        
        println!("💬 Gossip discovery working!");
        println!("   Total discovered: {}", stats.total_discovered);
        println!("   Trusted nodes: {}", stats.trusted_nodes);
        println!("   Avg response time: {}ms", stats.avg_response_time);
    }

    #[test]
    fn test_best_nodes_selection() {
        let mut discovery = NodeDiscovery::new(vec![]);
        
        // Add nodes with different trust scores
        let nodes = vec![
            DiscoveredNode {
                node_id: vec![1],
                address: "127.0.0.1:8080".parse().unwrap(),
                last_seen: current_timestamp(),
                response_time: 100,
                trust_score: 0.9,
            },
            DiscoveredNode {
                node_id: vec![2],
                address: "127.0.0.1:8081".parse().unwrap(),
                last_seen: current_timestamp(),
                response_time: 50,
                trust_score: 0.8,
            },
        ];
        
        discovery.discover_from_gossip(nodes);
        
        let best_nodes = discovery.get_best_nodes(1);
        assert_eq!(best_nodes.len(), 1);
        // Should select the node with better combined score
        
        println!("🎯 Best node selection working!");
    }
}