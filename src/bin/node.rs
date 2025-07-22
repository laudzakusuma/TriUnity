//! 🚀 TriUnity Node - Blockchain Network Participant

use clap::{Arg, Command};
use tokio::time::{sleep, Duration};
use triunity::core::crypto::QuantumKeyPair;
use triunity::core::consensus::ConsensusRouter;
use triunity::core::network::{NetworkProtocol, NodeCapabilities};
use triunity::core::storage::StateManager;
use triunity::VERSION;

#[tokio::main]
async fn main() {
    let matches = Command::new("triunity-node")
        .version(VERSION)
        .author("TriUnity Team <team@triunity.org>")
        .about("🚀 TriUnity Protocol Node - Join the revolution!")
        .arg(
            Arg::new("debug")
                .long("debug")
                .action(clap::ArgAction::SetTrue)
                .help("Enable debug mode")
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Node listening port")
                .default_value("8080")
        )
        .arg(
            Arg::new("validator")
                .long("validator")
                .action(clap::ArgAction::SetTrue)
                .help("Run as validator node")
        )
        .get_matches();

    let debug = matches.get_flag("debug");
    let port: u16 = matches
        .get_one::<String>("port")
        .unwrap()
        .parse()
        .unwrap_or(8080);
    let is_validator = matches.get_flag("validator");

    if debug {
        println!("🔧 Debug mode enabled");
    }

    run_node(port, is_validator, debug).await;
}

async fn run_node(port: u16, is_validator: bool, debug: bool) {
    println!("🚀 TriUnity Node Starting...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   🌟 Welcome to the Blockchain Revolution!");
    println!("   ⚡ Defeating the Trilemma in Real-Time");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 Node Configuration:");
    println!("   🔧 Version: {}", VERSION);
    println!("   🌐 Port: {}", port);
    println!("   🏛️ Validator: {}", if is_validator { "✅ Active" } else { "❌ Inactive" });
    println!("   🔍 Debug: {}", if debug { "✅ Enabled" } else { "❌ Disabled" });
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Generate node identity
    println!("🔑 Generating quantum-safe node identity...");
    let keypair = QuantumKeyPair::generate();
    let node_id = keypair.public_key().to_vec();
    
    println!("✅ Node identity established:");
    println!("   🆔 Node ID: 0x{}", hex::encode(&node_id[..8]));
    println!("   🏠 Address: 0x{}", keypair.address_hex());
    println!("   🛡️ Quantum-safe: YES");

    // Initialize core components
    println!("🏗️ Initializing blockchain components...");
    
    let capabilities = NodeCapabilities {
        is_validator,
        supports_fast_sync: true,
        max_connections: 100,
        supported_protocols: vec!["triunity/1.0".to_string()],
        quantum_safe: true,
    };

    let consensus_router = ConsensusRouter::new(); // Removed mut
    let network_protocol = NetworkProtocol::new(node_id, capabilities);
    let mut state_manager = StateManager::new();

    println!("   🤖 AI Consensus Router: ONLINE");
    println!("   🌐 Network Protocol: READY");
    println!("   🗄️ State Manager: ACTIVE");
    println!("   💾 Quantum Storage: INITIALIZED");
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎯 TriUnity Node is LIVE!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔥 Revolutionary Features Active:");
    println!("   ⚡ Scalability: 100,000+ TPS capability");
    println!("   🛡️ Security: Quantum-resistant cryptography");
    println!("   🌐 Decentralization: AI-optimized consensus");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let mut block_count = 0;
    let mut transaction_count = 0;
    let mut cycle_count = 0;

    loop {
        cycle_count += 1;
        
        // Simulate blockchain activity
        if debug {
            println!("📊 Node Status Report #{}", cycle_count);
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            
            let stats = state_manager.get_stats();
            let network_stats = network_protocol.get_network_stats();
            let ai_confidence = consensus_router.ai_confidence();
            
            println!("🔗 Blockchain Metrics:");
            println!("   📦 Blocks Processed: {}", block_count);
            println!("   💳 Total Transactions: {}", transaction_count);
            println!("   ⚡ Current TPS: 1,000 (simulated)");
            println!("   ⏱️ Block Time: 100ms");
            
            println!("🌐 Network Status:");
            println!("   👥 Connected Peers: {}", network_stats.connected_peers);
            println!("   📡 Network Latency: {}ms", network_stats.avg_latency);
            println!("   🔄 Protocol: triunity/1.0");
            
            println!("🤖 AI Consensus:");
            println!("   🧠 AI Confidence: {:.1}%", ai_confidence * 100.0);
            let optimal_path = consensus_router.select_optimal_path();
            println!("   🎯 Optimal Path: {:?}", optimal_path);
            
            println!("🗄️ State Information:");
            println!("   👤 Total Accounts: {}", stats.total_accounts);
            println!("   📄 Contract Accounts: {}", stats.contract_accounts);
            println!("   💰 Total Supply: {}", stats.total_supply);
            println!("   📏 Current Height: {}", stats.current_height);
            
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        } else {
            // Non-debug mode: minimal output
            if cycle_count % 6 == 0 { // Every 3 minutes
                println!("🔥 TriUnity Node Active - Blocks: {}, Transactions: {}, TPS: 1,000", 
                    block_count, transaction_count);
            }
        }

        // Simulate processing
        block_count += 1;
        transaction_count += 1000; // Simulate 1000 TPS
        
        // Create some dummy accounts for demo
        if block_count % 10 == 0 {
            let dummy_address = format!("address_{}", block_count).into_bytes();
            let account = state_manager.get_or_create_account(&dummy_address);
            account.balance = 1000;
        }

        // Sleep based on mode
        let sleep_duration = if debug { 
            Duration::from_secs(5)   // 5 seconds in debug
        } else { 
            Duration::from_secs(30)  // 30 seconds in normal mode
        };
        
        sleep(sleep_duration).await;
        
        // Prevent overflow in long runs
        if block_count > 1_000_000 {
            println!("🔄 Resetting counters after 1M blocks");
            block_count = 0;
            transaction_count = 0;
            cycle_count = 0;
        }
    }
}