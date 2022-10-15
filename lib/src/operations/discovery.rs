use tokio::net::UdpSocket;

use anyhow::Result;

use crate::{
    soap::{Envelope, Soap},
    wsdl::probe::{Probe, ProbeMatches},
};

pub async fn discover_onvif_devices() -> Result<()> {
    info!("Preparing to discover devices");
    // Create a udp socket on any port provided by OS
    debug!("Binding socket");
    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    debug!("Successfully binded: {}", sock.local_addr()?);

    let payload = Probe::default();
    debug!("Payload: {:?}", payload);

    let lathered_request = payload.apply_soap();
    let serialized = quick_xml::se::to_string(&lathered_request)?;
    let request_as_bytes = serialized.as_bytes();

    // Send the Discovery XML to IP Multicast
    let target = "239.255.255.250:3702";
    debug!("Sending payload");
    sock.send_to(&request_as_bytes, target).await?;
    debug!("Payload sent");

    let mut recv_buf: [u8; 1500] = [0; 1500];
    debug!("Listening for response");
    let n = sock.recv(&mut recv_buf).await?;
    debug!("We received something: {}", n);
    let response_string = String::from_utf8_lossy(&recv_buf).into_owned();
    debug!("Response: {}", response_string);
    let deserialize_response =
        quick_xml::de::from_str::<Envelope<ProbeMatches>>(&response_string).unwrap();
    debug!("Deserialized response: {:?}", deserialize_response);
    Ok(())
}
