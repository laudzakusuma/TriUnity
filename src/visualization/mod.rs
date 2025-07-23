//! 🎨 Real-time visualization and monitoring
//! 
//! Web-based dashboard integration with TriUnity blockchain

use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use serde_json::json;

/// 📊 Real-time metrics collector
#[derive(Debug, Clone)]
pub struct MetricsCollector {
    pub current_tps: Arc<Mutex<u64>>,
    pub block_height: Arc<Mutex<u64>>,
    pub node_count: Arc<Mutex<usize>>,
    pub ai_confidence: Arc<Mutex<f64>>,
    pub consensus_path: Arc<Mutex<String>>,
}

impl MetricsCollector {
    /// 🆕 Create new metrics collector
    pub fn new() -> Self {
        Self {
            current_tps: Arc::new(Mutex::new(0)),
            block_height: Arc::new(Mutex::new(0)),
            node_count: Arc::new(Mutex::new(0)),
            ai_confidence: Arc::new(Mutex::new(0.0)),
            consensus_path: Arc::new(Mutex::new("FastLane".to_string())),
        }
    }

    /// 📈 Update metrics
    pub fn update_metrics(&self, tps: u64, height: u64, nodes: usize, confidence: f64, path: String) {
        *self.current_tps.lock().unwrap() = tps;
        *self.block_height.lock().unwrap() = height;
        *self.node_count.lock().unwrap() = nodes;
        *self.ai_confidence.lock().unwrap() = confidence;
        *self.consensus_path.lock().unwrap() = path;
    }

    /// 📊 Get current metrics as JSON
    pub fn get_metrics_json(&self) -> String {
        let tps = *self.current_tps.lock().unwrap();
        let height = *self.block_height.lock().unwrap();
        let nodes = *self.node_count.lock().unwrap();
        let confidence = *self.ai_confidence.lock().unwrap();
        let path = self.consensus_path.lock().unwrap().clone();

        json!({
            "tps": tps,
            "block_height": height,
            "node_count": nodes,
            "ai_confidence": confidence,
            "consensus_path": path,
            "timestamp": chrono::Utc::now().timestamp()
        }).to_string()
    }
}

/// 🌐 Start visualization web server
pub async fn start_visualization_server(port: u16, metrics: Arc<MetricsCollector>) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    println!("🎨 Visualization server running on http://127.0.0.1:{}", port);
    
    loop {
        let (stream, _) = listener.accept().await?;
        let metrics_clone = metrics.clone();
        
        tokio::spawn(async move {
            handle_connection(stream, metrics_clone).await;
        });
    }
}

async fn handle_connection(mut stream: tokio::net::TcpStream, metrics: Arc<MetricsCollector>) {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();
    
    let request = String::from_utf8_lossy(&buffer[..]);
    
    let response = if request.contains("GET /api/metrics") {
        // API endpoint for metrics
        let metrics_json = metrics.get_metrics_json();
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}",
            metrics_json
        )
    } else {
        // Serve HTML dashboard
        let html = include_str!("../visualization/dashboard.html");
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}",
            html
        )
    };
    
    stream.write_all(response.as_bytes()).await.unwrap();
}