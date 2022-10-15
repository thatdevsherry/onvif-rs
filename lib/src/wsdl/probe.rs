use serde::{Deserialize, Serialize};

use crate::onvif_operation::OnvifOperation;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Types {
    #[serde(rename = "$value")]
    value: String,
}

impl Default for Probe {
    fn default() -> Self {
        return Probe {
            types: Types {
                value: "NetworkVideoTransmitter".to_string(),
            },
            scopes: Scopes { value: None },
        };
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Scopes {
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ProbeMatches {
    #[serde(rename = "ProbeMatch")]
    pub probe_match: ProbeMatch,
}

impl OnvifOperation for ProbeMatches {
    fn get_operation_name() -> &'static str {
        "ProbeMatches"
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct XAddrs {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EndpointReference {
    #[serde(rename = "Address")]
    pub address: Address,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
