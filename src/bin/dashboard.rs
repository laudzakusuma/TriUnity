//! 🚀 TriUnity Dashboard Server Binary

use std::sync::Arc;
use tokio::signal;
use clap::{Arg, Command};

use triunity::consensus::ConsensusEngine;
use triunity::storage::TriUnityStorage;
use triunity::web::DashboardServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let matches = Command::new("TriUnity Dashboard")
        .version("1.0.0")
        .about("🚀 TriUnity Blockchain Dashboard Server")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Web server port")
                .default_value("8080")
        )
        .arg(
            Arg::new("data-dir")
                .short('d')
                .long("data-dir")
                .value_name("DIR")
                .help("Data directory for blockchain storage")
                .default_value("./data")
        )
        .get_matches();

    let port: u16 = matches.get_one::<String>("port")
        .unwrap()
        .parse()
        .expect("Invalid port number");
    
    let data_dir = matches.get_one::<String>("data-dir").unwrap();

    println!("🚀 Starting TriUnity Dashboard Server");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   🌐 Port: {}", port);
    println!("   💾 Data Directory: {}", data_dir);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Initialize blockchain components
    println!("🔧 Initializing blockchain components...");
    
    let storage = Arc::new(TriUnityStorage::new(data_dir).await?);
    let consensus_engine = Arc::new(ConsensusEngine::new());
    
    println!("✅ Blockchain components initialized");

    // Create and start dashboard server
    println!("🌐 Starting dashboard server...");
    let dashboard_server = DashboardServer::new(consensus_engine, storage);
    
    // Handle graceful shutdown
    let server_handle = tokio::spawn(async move {
        dashboard_server.start(port).await
    });

    // Wait for Ctrl+C
    signal::ctrl_c().await?;
    println!("\n🛑 Shutting down gracefully...");
    
    server_handle.abort();
    
    println!("✅ TriUnity Dashboard Server stopped");
    Ok(())
}