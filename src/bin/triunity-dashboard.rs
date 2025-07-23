use std::sync::{Arc, Mutex};
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
        .arg(
            Arg::new("auto-mine")
                .long("auto-mine")
                .help("Enable automatic block mining")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let port: u16 = matches.get_one::<String>("port")
        .unwrap()
        .parse()
        .expect("Invalid port number");
    
    let data_dir = matches.get_one::<String>("data-dir").unwrap();
    let auto_mine = matches.get_flag("auto-mine");

    println!("🚀 Starting TriUnity Dashboard Server");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   🌐 Port: {}", port);
    println!("   💾 Data Directory: {}", data_dir);
    println!("   ⛏️  Auto Mining: {}", if auto_mine { "Enabled" } else { "Disabled" });
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Initialize blockchain components
    println!("🔧 Initializing blockchain components...");
    
    let storage = Arc::new(TriUnityStorage::new(data_dir).await?);
    let consensus_engine = Arc::new(Mutex::new(ConsensusEngine::new()));
    
    // Initialize the consensus engine with some validators
    {
        let mut consensus = consensus_engine.lock().unwrap();
        consensus.add_validator("validator_1".to_string(), 100);
        consensus.add_validator("validator_2".to_string(), 100);
        consensus.add_validator("validator_3".to_string(), 100);
        println!("✅ Added 3 initial validators");
    }

    // Start blockchain simulation if auto-mine is enabled
    if auto_mine {
        let storage_clone = storage.clone();
        let consensus_clone = consensus_engine.clone();
        
        tokio::spawn(async move {
            start_blockchain_simulation(storage_clone, consensus_clone).await;
        });
        
        println!("⛏️  Blockchain simulation started");
    }

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

/// ⛏️ Simulate blockchain activity
async fn start_blockchain_simulation(
    storage: Arc<TriUnityStorage>,
    consensus_engine: Arc<Mutex<ConsensusEngine>>
) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
    let mut block_number = 1;
    
    loop {
        interval.tick().await;
        
        // Generate some transactions
        let transactions = generate_sample_transactions(10 + (block_number % 50));
        
        // Create a new block
        let parent_hash = if block_number == 1 {
            "0000000000000000000000000000000000000000000000000000000000000000".to_string()
        } else {
            format!("block_hash_{:06}", block_number - 1)
        };

        let block = triunity::blockchain::Block {
            number: block_number,
            timestamp: chrono::Utc::now().timestamp() as u64,
            parent_hash,
            transactions,
            merkle_root: format!("merkle_root_{:06}", block_number),
            nonce: rand::random(),
            difficulty: 4,
            hash: format!("block_hash_{:06}", block_number),
        };

        {
            let mut consensus = consensus_engine.lock().unwrap();
            let _ = consensus.process_transactions(&block.transactions).await;
            consensus.update_performance_stats(block.transactions.len() as u64, 100);
        }

        if let Err(e) = storage.store_block(&block).await {
            eprintln!("❌ Failed to store block {}: {}", block_number, e);
        } else {
            println!("✅ Block {} mined with {} transactions", block_number, block.transactions.len());
        }

        block_number += 1;
        
        if block_number % 10 == 0 {
            let mut consensus = consensus_engine.lock().unwrap();
            consensus.simulate_network_activity();
        }
    }
}

fn generate_sample_transactions(count: u64) -> Vec<triunity::blockchain::Transaction> {
    let mut transactions = Vec::new();
    
    for i in 0..count {
        let tx = triunity::blockchain::Transaction {
            hash: format!("tx_hash_{:08}", rand::random::<u32>()),
            from: format!("address_{:04}", i % 100),
            to: format!("address_{:04}", (i + 1) % 100),
            amount: (rand::random::<u64>() % 1000) + 1,
            fee: 1,
            timestamp: chrono::Utc::now().timestamp() as u64,
            signature: format!("signature_{:08}", rand::random::<u32>()),
        };
        transactions.push(tx);
    }
    
    transactions
}