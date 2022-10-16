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

// #[cfg(test)]
// mod tests {
//     use crate::{
//         create_soap_request,
//         soap::{Envelope, Soap},
//         wsdl::get_system_date_and_time::{GetSystemDateAndTime, GetSystemDateAndTimeResponse},
//     };

//     #[test]
//     fn test_soap_request_creation_for_get_system_date_and_time() {
//         let get_system_date_and_time = GetSystemDateAndTime {};
//         let soap_request_string = create_soap_request(get_system_date_and_time).unwrap();
//         assert_eq!(
//             "<Envelope><Body><GetSystemDateAndTime/></Body></Envelope>",
//             soap_request_string
//         );
//     }

//     #[test]
//     fn test_deserialize_get_system_date_and_time_response() {
//         let input = "<Envelope><Body><GetSystemDateAndTimeResponse/></Body></Envelope>";
//         let actual =
//             quick_xml::de::from_str::<Envelope<GetSystemDateAndTimeResponse>>(input).unwrap();
//         let expected = GetSystemDateAndTimeResponse {}.apply_soap();
//         assert_eq!(expected, actual);
//     }
// }
