use serde::{de::Visitor, ser::SerializeMap, Deserialize, Serialize};

use crate::{
    onvif_operation::OnvifOperation, wsdl::get_system_date_and_time::GetSystemDateAndTime,
};

pub trait Soap<T: OnvifOperation> {
    fn apply_soap(self) -> Envelope<T>;
}

impl<T: OnvifOperation> Soap<T> for T {
    fn apply_soap(self) -> Envelope<T> {
        Envelope::new(self)
    }
}

#[derive(Debug, Deserialize)]
pub struct Envelope<T: OnvifOperation> {
    #[serde(rename = "$value")]
    body: Body<T>,
}

#[derive(Debug, Deserialize)]
pub struct Body<T: OnvifOperation> {
    payload: T,
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
        }
    }

    pub fn remove_soap(self) -> T {
        self.body.payload
    }
}
