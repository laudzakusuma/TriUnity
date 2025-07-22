use clap::{Arg, Command};
use std::process;
use tokio::time::{sleep, Duration};
use triunity::core::crypto::QuantumKeyPair;  // Use from main crypto module
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
    println!("🚀 Starting TriUnity Node...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   Version: {}", VERSION);
    println!("   Port: {}", port);
    println!("   Validator: {}", if is_validator { "✅ Yes" } else { "❌ No" });
    println!("   Debug: {}", if debug { "✅ Enabled" } else { "❌ Disabled" });
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Generate node identity
    let keypair = QuantumKeyPair::generate();
    let node_id = keypair.public_key().to_vec();
    
    println!("🔑 Node identity generated:");
    println!("   Node ID: 0x{}", hex::encode(&node_id[..8]));
    println!("   Address: 0x{}", keypair.address_hex());

    // Initialize components
    let capabilities = NodeCapabilities {
        is_validator,
        supports_fast_sync: true,
        max_connections: 100,
        supported_protocols: vec!["triunity/1.0".to_string()],
        quantum_safe: true,
    };

    let mut consensus_router = ConsensusRouter::new();
    let network_protocol = NetworkProtocol::new(node_id, capabilities);
    let mut state_manager = StateManager::new();

    println!("🤖 AI Consensus Router initialized");
    println!("🌐 Network Protocol ready");
    println!("🗄️ State Manager active");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Start main node loop
    println!("🎯 TriUnity Node is LIVE!");
    println!("   🔥 Defeating the blockchain trilemma...");
    println!("   ⚡ Scalability: MAXIMUM");
    println!("   🛡️ Security: QUANTUM-SAFE");
    println!("   🌐 Decentralization: AI-OPTIMIZED");
    
    let mut block_count = 0;
    let mut transaction_count = 0;

    loop {
        // Simulate node activity
        if debug {
            println!("📊 Node Status Update:");
            
            let stats = state_manager.get_stats();
            let network_stats = network_protocol.get_network_stats();
            let ai_confidence = consensus_router.ai_confidence();
            
            println!("   📦 Blocks processed: {}", block_count);
            println!("   💳 Transactions: {}", transaction_count);
            println!("   👥 Connected peers: {}", network_stats.connected_peers);
            println!("   🤖 AI Confidence: {:.2}%", ai_confidence * 100.0);
            println!("   🗄️ Total accounts: {}", stats.total_accounts);
            println!("   💰 Total supply: {}", stats.total_supply);
            
            // Select optimal consensus path
            let optimal_path = consensus_router.select_optimal_path();
            println!("   🎯 Consensus Path: {:?}", optimal_path);
            
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        }

        // Simulate processing
        block_count += 1;
        transaction_count += 1000; // Simulate 1000 TPS

        // Sleep for a bit
        sleep(Duration::from_secs(if debug { 5 } else { 30 })).await;
        
        // Prevent overflow in long runs
        if block_count > 1000000 {
            block_count = 0;
            transaction_count = 0;
        }
    }
}