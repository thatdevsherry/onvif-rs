use quick_xml::de::DeError;
use std::collections::HashMap;

use serde::{ser::SerializeMap, Deserialize, Serialize};

use onvif::onvif_operation::OnvifOperation;
pub trait Soap<T: OnvifOperation> {
    fn apply_soap(self) -> Envelope<T>;
}

impl<T: OnvifOperation> Soap<T> for T {
    fn apply_soap(self) -> Envelope<T> {
        Envelope::new(self)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Envelope<T: OnvifOperation> {
    #[serde(rename = "Body")]
    pub body: Body<T>,
    #[serde(rename = "Header")]
    pub header: Option<Header>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Header {}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(try_from = "HashMap<String, T>")]
pub struct Body<T: OnvifOperation> {
    pub payload: T,
}

impl<T: OnvifOperation> TryFrom<HashMap<String, T>> for Body<T> {
    type Error = DeError;

    fn try_from(mut value: HashMap<String, T>) -> Result<Self, Self::Error> {
        if let Some(payload) = value.remove(T::get_operation_name()) {
            Ok(Self { payload })
        } else {
            Err(DeError::Custom("Missing Field".to_string()))
        }
    }
}

impl<T: OnvifOperation + Serialize> Serialize for Envelope<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry("Envelope", &self.body)?;
        state.end()
    }
}

impl<T: OnvifOperation + Serialize> Serialize for Body<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry("Body", &self.payload)?;
        state.end()
    }
}

impl<T: OnvifOperation> Body<T> {
    fn new(onvif_operation: T) -> Self {
        Body {
            payload: onvif_operation,
        }
    }
}

impl<T: OnvifOperation> Envelope<T> {
    fn new(onvif_operation: T) -> Self {
        Envelope {
            body: Body::new(onvif_operation),
            header: None,
        }
    }
}

// #[cfg(test)]
// mod tests {

//     // use crate::wsdl::get_system_date_and_time::GetSystemDateAndTime;

//     use super::{Body, Envelope, Soap};

//     #[test]
//     fn test_operation_is_wrapped_in_soap() {
//         let expected = Envelope {
//             body: Body {
//                 payload: GetSystemDateAndTime {},
//             },
//             header: None,
//         };
//         let sample_operation = GetSystemDateAndTime {};
//         let actual = sample_operation.apply_soap();
//         assert_eq!(expected, actual);
//     }

//     #[test]
//     fn test_soap_serialize() {
//         let expected = "<Envelope><Body><GetSystemDateAndTime/></Body></Envelope>";
//         let sample_operation = GetSystemDateAndTime {};
//         let soap_request = sample_operation.apply_soap();
//         let actual = quick_xml::se::to_string(&soap_request).unwrap();
//         assert_eq!(expected, actual);
//     }
// }
