#[macro_use]
extern crate log;

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub async fn testing() -> Result<()> {
    let suppose_output = r#"
    <s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope">
    <s:Body xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
    <GetSystemDateAndTimeResponse xmlns="http://www.onvif.org/ver10/device/wsdl">
    <TimeZone>PKT</TimeZone>
    </GetSystemDateAndTimeResponse>
    </s:Body>
    </s:Envelope>"#;
    let output_in_xml: Envelope<GetSystemDateAndTimeResponse> =
        quick_xml::de::from_str(&suppose_output)?;
    debug!("SOAP response wrapped in envelope: {:?}", output_in_xml);
    Ok(())
}

/// Envelope is a header for SOAP requests
/// and always has a `Body`
#[derive(Debug, Deserialize, Serialize)]
struct Envelope<T: ResponseType> {
    #[serde(rename = "Body")]
    body: Body<T>,
}

/// This is supposed to contain our payload
/// Since payloads can be anything, we ideally want
/// it to not have a hardcoded field during serialize
/// and instead let it dynamically serialize the payload.
///
/// So I'm looking at somehow serializing this Body to return
/// the payload it is wrapping.
#[derive(Debug, Deserialize, Serialize)]
struct Body<T: ResponseType> {
    // Rename here is a literal, we might have to implement
    // our own serializer?
    #[serde(rename = "GetSystemDateAndTimeResponse")]
    payload: T,
}

/// ONVIF WSDL for GetSystemDateAndTimeResponse
#[derive(Debug, Serialize, Deserialize)]
struct GetSystemDateAndTimeResponse {
    #[serde(rename = "TimeZone")]
    time_zone: String,
}

/// Have each ONVIF response implement a trait
/// which in this case would be the SOAP response name
/// as defined in their WSDL
trait ResponseType {
    fn response_type() -> &'static str;
}

/// Implement the trait for our sample ONVIF response
impl ResponseType for GetSystemDateAndTimeResponse {
    fn response_type() -> &'static str {
        "GetSystemDateAndTimeResponse"
    }
}
