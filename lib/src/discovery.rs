use tokio::net::UdpSocket;

use anyhow::Result;
use uuid::Uuid;

pub async fn discover() -> Result<()> {
    // Create a udp socket on any port provided by OS
    debug!("Binding socket");
    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    debug!("Successfully binded to a port");

    let payload = format!(
        r#"
        <Envelope xmlns="http://www.w3.org/2003/05/soap-envelope" xmlns:dn="http://www.onvif.org/ver10/network/wsdl"> 
        <Header> 
            <wsa:MessageID xmlns:wsa="http://schemas.xmlsoap.org/ws/2004/08/addressing">  {messageId}  </wsa:MessageID> 
            <wsa:To xmlns:wsa="http://schemas.xmlsoap.org/ws/2004/08/addressing">urn:schemas-xmlsoap-org:ws:2005:04:discovery</wsa:To> 
            <wsa:Action xmlns:wsa="http://schemas.xmlsoap.org/ws/2004/08/addressing">http://schemas.xmlsoap.org/ws/2005/04/discovery/Probe</wsa:Action> 
        </Header> 
        <Body> 
            <Probe xmlns="http://schemas.xmlsoap.org/ws/2005/04/discovery" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"> 
                <Types>dn:NetworkVideoTransmitter</Types> 
                <Scopes /> 
            </Probe> 
        </Body> 
    </Envelope>
    "#,
        messageId = Uuid::new_v4().to_string()
    );
    debug!("Payload: {}", payload);

    let payload_bytes = payload.as_bytes();

    // Send the Discovery XML to IP Multicast
    let target = "239.255.255.250:3702";
    debug!("Sending payload");
    sock.send_to(&payload_bytes, target).await?;
    debug!("Payload sent");

    let mut recv_buf: [u8; 1024] = [0; 1024];
    debug!("Listening for response");
    let n = sock.recv(&mut recv_buf).await?;
    debug!("We received something: {}", n);
    Ok(())
}
