#[macro_use]
extern crate log;
use lib;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("ONVIF-rs started");

    // start messing around with XML
    lib::testing().await?;
    Ok(())
}
