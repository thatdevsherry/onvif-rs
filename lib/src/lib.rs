mod get_system_date_and_time;
mod onvif_operation;
mod soap;

#[macro_use]
extern crate log;

use anyhow::Result;
use get_system_date_and_time::GetSystemDateAndTime;

use crate::{get_system_date_and_time::GetSystemDateAndTimeResponse, soap::Soap};

pub async fn get_system_date_and_time() -> Result<()> {
    let get_system_date_and_time = GetSystemDateAndTime {};
    let envelope = get_system_date_and_time.apply_soap();
    let envelope_serialize = quick_xml::se::to_string(&envelope)?;
    debug!("Envelope serialized: {:?}", envelope_serialize);

    let get_system_date_and_time_response = GetSystemDateAndTimeResponse {};
    let response_wrapped_in_soap = get_system_date_and_time_response.apply_soap();
    let another_soapy_request_serialized = quick_xml::se::to_string(&response_wrapped_in_soap)?;
    debug!(
        "Yet another soapy request: {:?}",
        another_soapy_request_serialized
    );

    Ok(())
}
