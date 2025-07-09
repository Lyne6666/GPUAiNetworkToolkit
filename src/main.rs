// src/main.rs
/*
 * Main executable for GPUAiNetworkToolkit
 */

use clap::Parser;
use gpuainetworktoolkit::{Result, run};

#[derive(Parser)]
#[command(version, about = "GPUAiNetworkToolkit - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
