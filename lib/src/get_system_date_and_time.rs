use serde::{Deserialize, Serialize};

use crate::onvif_operation::OnvifOperation;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSystemDateAndTime {
    pub sample_field: String,
}

impl OnvifOperation for GetSystemDateAndTime {
    fn get_operation_name(&self) -> String {
        "GetSystemDateAndTime".to_string()
    }
}

pub struct GetSystemDateAndTimeResponse {}

impl OnvifOperation for GetSystemDateAndTimeResponse {
    fn get_operation_name(&self) -> String {
        "GetSystemDateAndTimeResponse".to_string()
    }
}
