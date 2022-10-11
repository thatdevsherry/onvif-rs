use serde::{Deserialize, Serialize};

use crate::onvif_operation::OnvifOperation;

pub trait Soap<T: OnvifOperation> {
    fn apply_soap(self) -> Envelope<T>;
}

impl<T: OnvifOperation> Soap<T> for T {
    fn apply_soap(self) -> Envelope<T> {
        Envelope::new(self)
    }
}

#[derive(Debug, Deserialize)]
pub struct Body<T: OnvifOperation> {
    pub payload: T,
}

#[derive(Debug, Deserialize)]
pub struct Envelope<T: OnvifOperation> {
    pub body: Body<T>,
}

/// Generic trait impl that wraps any type that implements OnvifOperation
/// in a Body struct and serializes it
impl<T: OnvifOperation + Serialize> Serialize for Body<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry("Body", &self.payload)?;
        state.end()
    }
}

impl<T: OnvifOperation + Serialize> Serialize for Envelope<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry("Envelope", &self.body)?;
        state.end()
    }
}

impl<T: OnvifOperation> Envelope<T> {
    pub fn new(onvif_operation: T) -> Self {
        Envelope {
            body: Body::new(onvif_operation),
        }
    }
}

impl<T: OnvifOperation> Body<T> {
    pub fn new(onvif_operation: T) -> Body<T> {
        Body {
            payload: onvif_operation,
        }
    }
}
