//! 🌐 Web server integration for TriUnity dashboard

use std::sync::Arc;
use warp::Filter;
use serde::Serialize;
use crate::consensus::ConsensusEngine;
use crate::storage::TriUnityStorage;

#[derive(Debug, Clone, Serialize)]
pub struct LiveMetrics {
    pub tps: u64,
    pub block_time_ms: u64,
    pub health_percentage: f64,
    pub validator_count: usize,
    pub ai_confidence: f64,
    pub ai_mode: String,
    pub ai_decisions_per_min: u64,
    pub ai_accuracy: f64,
    pub timestamp: u64,
}

pub struct DashboardServer {
    consensus_engine: Arc<ConsensusEngine>,
    storage: Arc<TriUnityStorage>,
}

impl DashboardServer {
    pub fn new(consensus_engine: Arc<ConsensusEngine>, storage: Arc<TriUnityStorage>) -> Self {
        Self {
            consensus_engine,
            storage,
        }
    }

    pub async fn start(&self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        println!("🌐 Starting TriUnity Dashboard Server on port {}", port);
        
        // Serve static dashboard
        let dashboard = warp::path::end()
            .map(|| {
                warp::reply::html(DASHBOARD_HTML)
            });

        // API endpoint for current metrics
        let consensus_clone = self.consensus_engine.clone();
        let metrics_api = warp::path("api")
            .and(warp::path("metrics"))
            .and(warp::path::end())
            .map(move || {
                let stats = consensus_clone.get_performance_stats();
                let metrics = LiveMetrics {
                    tps: stats.transactions_per_second,
                    block_time_ms: stats.average_block_time_ms,
                    health_percentage: stats.network_health_percentage,
                    validator_count: stats.active_validators,
                    ai_confidence: stats.ai_confidence_percentage,
                    ai_mode: format!("{:?}", stats.current_consensus_path),
                    ai_decisions_per_min: stats.ai_decisions_per_minute,
                    ai_accuracy: stats.ai_accuracy_percentage,
                    timestamp: chrono::Utc::now().timestamp() as u64,
                };
                warp::reply::json(&metrics)
            });

        let routes = dashboard
            .or(metrics_api)
            .with(warp::cors().allow_any_origin());

        println!("✅ Dashboard server running!");
        println!("   📊 Dashboard: http://localhost:{}", port);
        println!("   🔌 Metrics API: http://localhost:{}/api/metrics", port);

        warp::serve(routes)
            .run(([127, 0, 0, 1], port))
            .await;

        Ok(())
    }
}

const DASHBOARD_HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>TriUnity Dashboard</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            background: #f5f5f7;
            margin: 0;
            padding: 20px;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
        }
        .header {
            text-align: center;
            margin-bottom: 40px;
        }
        .logo {
            font-size: 2rem;
            font-weight: 600;
            color: #1d1d1f;
            margin-bottom: 10px;
        }
        .metrics-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px;
        }
        .metric-card {
            background: white;
            border-radius: 12px;
            padding: 24px;
            text-align: center;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        .metric-value {
            font-size: 2rem;
            font-weight: 300;
            color: #007aff;
            margin-bottom: 8px;
        }
        .metric-label {
            font-size: 0.875rem;
            color: #666;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }
        .status {
            display: inline-flex;
            align-items: center;
            gap: 8px;
            background: rgba(52, 199, 89, 0.1);
            color: #30d158;
            padding: 8px 16px;
            border-radius: 20px;
            font-size: 0.875rem;
            font-weight: 500;
        }
        .status-dot {
            width: 8px;
            height: 8px;
            background: #30d158;
            border-radius: 50%;
            animation: pulse 2s infinite;
        }
        @keyframes pulse {
            0%, 100% { opacity: 1; }
            50% { opacity: 0.5; }
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <div class="logo">🚀 TriUnity Dashboard</div>
            <div class="status">
                <div class="status-dot"></div>
                LIVE
            </div>
        </div>
        
        <div class="metrics-grid" id="metrics">
            <div class="metric-card">
                <div class="metric-value" id="tps">Loading...</div>
                <div class="metric-label">Transactions Per Second</div>
            </div>
            
            <div class="metric-card">
                <div class="metric-value" id="block-time">Loading...</div>
                <div class="metric-label">Block Time (ms)</div>
            </div>
            
            <div class="metric-card">
                <div class="metric-value" id="health">Loading...</div>
                <div class="metric-label">Network Health (%)</div>
            </div>
            
            <div class="metric-card">
                <div class="metric-value" id="validators">Loading...</div>
                <div class="metric-label">Active Validators</div>
            </div>
        </div>
    </div>

    <script>
        async function updateMetrics() {
            try {
                const response = await fetch('/api/metrics');
                const data = await response.json();
                
                document.getElementById('tps').textContent = data.tps.toLocaleString();
                document.getElementById('block-time').textContent = data.block_time_ms;
                document.getElementById('health').textContent = data.health_percentage.toFixed(1);
                document.getElementById('validators').textContent = data.validator_count;
            } catch (error) {
                console.error('Failed to update metrics:', error);
            }
        }

        // Update metrics every 5 seconds
        updateMetrics();
        setInterval(updateMetrics, 5000);
    </script>
</body>
</html>
"#;