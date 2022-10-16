use serde::{Deserialize, Serialize};

use onvif::onvif_operation::OnvifOperation;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct GetSystemDateAndTime {}

impl OnvifOperation for GetSystemDateAndTime {
    fn get_operation_name() -> &'static str {
        "GetSystemDateAndTime"
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct GetSystemDateAndTimeResponse {}

impl OnvifOperation for GetSystemDateAndTimeResponse {
    fn get_operation_name() -> &'static str {
        "GetSystemDateAndTimeResponse"
    }
}
