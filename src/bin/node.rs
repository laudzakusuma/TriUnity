//! 🚀 TriUnity Node Binary
//!
//! The main blockchain node executable

use clap::Parser;
use tracing::{error, info};

#[derive(Parser)]
#[command(name = "triunity-node")]
#[command(about = "🚀 TriUnity Protocol Node - Defeating the Blockchain Trilemma")]
struct Cli {
    /// 🌐 Network port to bind to
    #[arg(short, long, default_value = "8080")]
    port: u16,

    /// 📁 Data directory for blockchain storage
    #[arg(short, long, default_value = "./data")]
    data_dir: String,

    /// 🔧 Enable debug mode
    #[arg(long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Initialize logging
    if cli.debug {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    }

    info!("🚀 Starting TriUnity Node v{}", triunity::VERSION);
    info!("   Port: {}", cli.port);
    info!("   Data directory: {}", cli.data_dir);
    info!("   Target TPS: {}", triunity::TARGET_TPS);
    info!("   Quantum-safe: ✅ Ready for quantum computers!");

    // TODO: Initialize blockchain node
    info!("🔧 Initializing quantum-safe blockchain...");

    // Generate test keypair to verify crypto works
    use triunity::core::crypto::quantum_key_pair::QuantumKeyPair;
    let keypair = QuantumKeyPair::new();
    info!("🔑 Generated quantum-safe keypair");
    info!("  Address: 0x{}", keypair.address_hex());

    // Test signing
    let message = b"TriUnity Protocol - The future of blockchain!";
    let signature = keypair.sign(message).unwrap();
    info!("✍️  Signed test message");
    info!("  Signature size: {} bytes", signature.size());

    // Verify signature
    if signature.verify(message, &keypair.public_key) {
        info!("✅ Quantum signature verification successful!");
    } else {
        error!("❌ Quantum signature verification failed!");
        return Err("Signature verification failed".into());
    }

    info!("🎯 TriUnity Node ready to defeat the trilemma!");
    info!("   Press Ctrl+C to stop");

    // Keep running
    tokio::signal::ctrl_c().await?;
    info!("👋 TriUnity Node shutting down...");

    Ok(())
}
