use serde::{Deserialize, Serialize};
use tokio::net::UdpSocket;

use anyhow::Result;
const WS_DISCOVERY_IP_MULTICAST_ADDRESS: &'static str = "239.255.255.250";
const WS_DISCOVERY_PORT: &'static str = "3702";
const UDP_SOCKET_ADDR: &'static str = "0.0.0.0:0"; // let OS choose port

use crate::{
    soap::{Envelope, Soap},
    wsdl::probe::{Probe, ProbeMatch, ProbeMatches},
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DiscoveryParsed {
    name: Option<String>,
    country: Option<String>,
    profiles: Vec<String>,
}

pub async fn discover_onvif_devices() -> Result<()> {
    info!("Preparing to discover devices");
    debug!("Binding socket");
    let sock = UdpSocket::bind(UDP_SOCKET_ADDR).await?;
    debug!("Successfully binded: {}", sock.local_addr()?);

    let payload = Probe::default();
    debug!("Payload: {:?}", payload);

    let lathered_request = payload.apply_soap();
    let serialized = quick_xml::se::to_string(&lathered_request)?;
    let request_as_bytes = serialized.as_bytes();

    // Send the Discovery XML to IP Multicast
    let target = format!(
        "{}:{}",
        WS_DISCOVERY_IP_MULTICAST_ADDRESS, WS_DISCOVERY_PORT
    );
    debug!("Sending payload to: {}", target);
    sock.send_to(&request_as_bytes, target).await?;
    debug!("Payload sent");

    let mut recv_buf: [u8; 1500] = [0; 1500];
    debug!("Listening for response");
    let n = sock.recv(&mut recv_buf).await?;
    debug!("We received something of size: {}", n);
    info!("Found a device");
    let response_string = String::from_utf8_lossy(&recv_buf).into_owned();
    debug!("Response: {}", response_string);
    let deserialize_response =
        quick_xml::de::from_str::<Envelope<ProbeMatches>>(&response_string).unwrap();
    debug!("Deserialized response: {:?}", deserialize_response);
    Ok(())
}
