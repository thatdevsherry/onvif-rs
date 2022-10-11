use serde::{Deserialize, Serialize};

use crate::onvif_operation::OnvifOperation;

pub trait Soap<T: OnvifOperation> {
    fn apply_soap(self) -> Body<T>;
}

impl<T: OnvifOperation> Soap<T> for T {
    fn apply_soap(self) -> Body<T> {
        Body { payload: self }
    }
}

#[derive(Debug, Deserialize)]
pub struct Body<T: OnvifOperation> {
    pub payload: T,
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
