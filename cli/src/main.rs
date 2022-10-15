#[macro_use]
extern crate log;
use lib;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("ONVIF-rs started");

    // start messing around with XML
    // lib::get_system_date_and_time().await?;
    lib::discovery::discover().await?;
    Ok(())
}
