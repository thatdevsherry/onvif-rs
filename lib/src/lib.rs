mod get_system_date_and_time;
mod onvif_operation;
mod soap;

#[macro_use]
extern crate log;

use anyhow::Result;
use get_system_date_and_time::GetSystemDateAndTime;

use crate::{onvif_operation::OnvifOperation, soap::Body};

pub async fn get_system_date_and_time() -> Result<()> {
    let get_system_date_and_time = GetSystemDateAndTime {
        sample_field: "lol".to_string(),
    };
    debug!("Struct: {:?}", get_system_date_and_time);
    let operation_name = get_system_date_and_time.get_operation_name();
    debug!("Operation name: {:?}", operation_name);

    let onvif_operation_to_xml = quick_xml::se::to_string(&get_system_date_and_time)?;
    debug!("Serialized: {:?}", onvif_operation_to_xml);

    let soap_request = Body {
        payload: get_system_date_and_time,
    };
    debug!("Soap Request: {:?}", soap_request);

    let soap_serialized = quick_xml::se::to_string(&soap_request)?;
    debug!("Soap Serialized: {:?}", soap_serialized);

    Ok(())
}
