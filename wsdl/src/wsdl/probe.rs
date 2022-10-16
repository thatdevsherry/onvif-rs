use serde::{Deserialize, Serialize};

use onvif::onvif_operation::OnvifOperation;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Probe {
    #[serde(rename = "Types")]
    types: Types,
    #[serde(rename = "Scopes")]
    scopes: Scopes,
}

/// TODO(Shehriyar Qureshi): Umm this ain't right, we have to make `Soap` generic
/// because Probe isn't an ONVIF operation, and we shouldn't limit wrapping only
/// OnvifOperation types in soap. Leaving as is for now
impl OnvifOperation for Probe {
    fn get_operation_name() -> &'static str {
        "Probe"
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Types {
    #[serde(rename = "$value")]
    pub value: String,
}

impl Default for Probe {
    /// Creates a probe that requests to receive responses from `NetworkVideoTransmitter`.
    ///
    /// ## ONVIF definition for NVT
    ///
    /// Network video server (an IP network camera or an encoder device, for example) that sends media data over an IP
    /// network to a client.
    fn default() -> Self {
        Probe {
            types: Types {
                value: "NetworkVideoTransmitter".to_string(),
            },
            scopes: Scopes { value: None },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Scopes {
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ProbeMatches {
    #[serde(rename = "ProbeMatch")]
    pub probe_match: ProbeMatch,
}

impl OnvifOperation for ProbeMatches {
    fn get_operation_name() -> &'static str {
        "ProbeMatches"
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProbeMatch {
    #[serde(rename = "EndpointReference")]
    pub endpoint_reference: EndpointReference,
    #[serde(rename = "Types")]
    pub types: Types,
    #[serde(rename = "Scopes")]
    pub scopes: Scopes,
    #[serde(rename = "XAddrs")]
    pub xaddrs: XAddrs,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct XAddrs {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct EndpointReference {
    #[serde(rename = "Address")]
    pub address: Address,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Address {
    #[serde(rename = "$value")]
    pub urn: String,
}

#[cfg(test)]
mod tests {
    use super::Probe;

    #[test]
    fn test_serialize_probe() {
        let expected = "<Probe><Types>NetworkVideoTransmitter</Types><Scopes/></Probe>";
        let probe = Probe::default();
        let actual = quick_xml::se::to_string(&probe).unwrap();
        assert_eq!(expected, actual);
    }
}
