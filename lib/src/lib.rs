pub mod discovery;
mod onvif_operation;
mod soap;
// mod discovery;
mod wsdl;
use soap::Soap;
use wsdl::get_system_date_and_time::{GetSystemDateAndTime, GetSystemDateAndTimeResponse};

#[macro_use]
extern crate log;

// use crate::get_system_date_and_time::GetSystemDateAndTime;
use anyhow::Result;

// use crate::{get_system_date_and_time::GetSystemDateAndTimeResponse, soap::Soap};

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
