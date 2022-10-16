use serde::{Deserialize, Serialize};
use tokio::net::UdpSocket;

use anyhow::Result;
const WS_DISCOVERY_IP_MULTICAST_ADDRESS: &str = "239.255.255.250:3702";
const UDP_SOCKET_ADDR: &str = "0.0.0.0:0"; // let OS choose port
const ONVIF_COUNTRY_PREFIX: &str = "onvif://www.onvif.org/location/country/";
const ONVIF_PROFILE_PREFIX: &str = "onvif://www.onvif.org/Profile/";
const ONVIF_NAME_PREFIX: &str = "onvif://www.onvif.org/name/";

use soap::soap::{Envelope, Soap};
use wsdl::wsdl::probe::{Probe, ProbeMatches};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscoveryParsed {
    name: Option<String>,
    country: Option<String>,
    profiles: Vec<String>,
}

impl From<ProbeMatches> for DiscoveryParsed {
    fn from(probe_match: ProbeMatches) -> Self {
        let string_to_parse = probe_match.probe_match.scopes.value.unwrap();

        // TODO(Shehriyar Qureshi): Maybe use the `iter().map()` type of flow instead of doing this mut stuff?
        let mut country: Option<String> = None;
        let mut name: Option<String> = None;
        let mut profiles: Vec<String> = Vec::new();

        let scopes = string_to_parse.split_whitespace();
        for scope in scopes {
            if scope.contains(ONVIF_COUNTRY_PREFIX) {
                country = Some(scope[ONVIF_COUNTRY_PREFIX.len()..].to_string())
            }
            if scope.contains(ONVIF_NAME_PREFIX) {
                name = Some(scope[ONVIF_NAME_PREFIX.len()..].to_string())
            }
            if scope.contains(ONVIF_PROFILE_PREFIX) {
                profiles.push(scope[ONVIF_PROFILE_PREFIX.len()..].to_string())
            }
        }

        Self {
            country,
            name,
            profiles,
        }
    }
}

/// Uses [WS-Discovery](https://en.wikipedia.org/wiki/WS-Discovery) to find
/// IP Cameras.
pub async fn discover_onvif_devices() -> Result<String> {
    info!("Preparing to discover devices");
    debug!("Binding socket");
    let sock = UdpSocket::bind(UDP_SOCKET_ADDR).await?;
    debug!("Successfully binded: {}", sock.local_addr()?);

    let request = prepare_discovery_request()?;

    // Send the Discovery XML to IP Multicast
    debug!("Sending payload to: {}", WS_DISCOVERY_IP_MULTICAST_ADDRESS);
    sock.send_to(&request[..], WS_DISCOVERY_IP_MULTICAST_ADDRESS)
        .await?;
    debug!("Payload sent");

    // TODO(Shehriyar Qureshi): Handle multiple responses
    let mut recv_buf: [u8; 1500] = [0; 1500];
    debug!("Listening for response");
    let n = sock.recv(&mut recv_buf).await?;
    debug!("We received something of size: {}", n);
    info!("Found a device");

    let serialize_to_json = handle_ws_discovery_response(&recv_buf[..n])?;

    Ok(serialize_to_json)
}

/// Builds the WS-Discovery request for NVTs and returns the request as bytes
fn prepare_discovery_request() -> Result<Vec<u8>> {
    let payload = Probe::default();
    debug!("Payload: {:?}", payload);

    let lathered_request = payload.apply_soap();
    let serialized = quick_xml::se::to_string(&lathered_request)?;
    let request_as_bytes = serialized.as_bytes().to_owned();
    Ok(request_as_bytes)
}

/// Processes WS-Discovery response and returns parsed data
fn handle_ws_discovery_response(response: &[u8]) -> Result<String> {
    let response_string = String::from_utf8_lossy(response).into_owned();
    debug!("Response: {}", response_string);
    let deserialize_response =
        quick_xml::de::from_str::<Envelope<ProbeMatches>>(&response_string).unwrap();
    debug!("Deserialized response: {:?}", deserialize_response);

    let removed_soap = deserialize_response.remove_soap();
    debug!("Underlying T: {:?}", removed_soap);
    let discovery_parsed = DiscoveryParsed::from(removed_soap);
    debug!("Parsed result: {:?}", discovery_parsed);
    let serialize_to_json = serde_json::ser::to_string(&discovery_parsed)?;
    info!("Result: {}", serialize_to_json);
    Ok(serialize_to_json)
}

#[cfg(test)]
mod tests {
    use wsdl::wsdl::probe::{EndpointReference, ProbeMatch, ProbeMatches, Scopes, Types, XAddrs};

    use super::DiscoveryParsed;

    #[test]
    fn test_parsing_discovery_response() {
        let discovery_response = ProbeMatches {
            probe_match: ProbeMatch {
                scopes: Scopes { value: Some("onvif://www.onvif.org/location/country/pakistan onvif://www.onvif.org/Profile/Streaming onvif://www.onvif.org/name/IP-Camera-Cool onvif://www.onvif.org/Profile/T".to_string()) },
                endpoint_reference: EndpointReference::default() ,
                types: Types { value: "".to_string() },
                xaddrs: XAddrs {value: "".to_string()}
                }
            };
        let discovery_parsed = DiscoveryParsed::from(discovery_response);
        assert_eq!(discovery_parsed.country, Some("pakistan".to_string()));
        assert_eq!(discovery_parsed.name, Some("IP-Camera-Cool".to_string()));
        assert_eq!(discovery_parsed.profiles.len(), 2);
        assert!(discovery_parsed.profiles.contains(&"Streaming".to_string()));
        assert!(discovery_parsed.profiles.contains(&"T".to_string()));
    }
}
