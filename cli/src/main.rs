#[macro_use]
extern crate log;

use clap::{Parser, Subcommand};
use discovery::discovery::discover_onvif_devices;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(arg_required_else_help(true))]
struct Cli {
    #[clap(subcommand)]
    operation_type: OperationType,
}

#[derive(Subcommand, Debug)]
enum OperationType {
    Discover {},
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("ONVIF-rs started");

    let cli = Cli::parse();
    match cli.operation_type {
        OperationType::Discover {} => {
            debug!("Selection: discovery");
            discover_onvif_devices().await?;
        }
    }
    Ok(())
}
