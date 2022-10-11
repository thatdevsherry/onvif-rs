mod get_system_date_and_time;
mod onvif_operation;
mod soap;

#[macro_use]
extern crate log;

use anyhow::Result;
use get_system_date_and_time::GetSystemDateAndTime;

use crate::{
    get_system_date_and_time::GetSystemDateAndTimeResponse,
    soap::{Body, Envelope},
};

pub async fn get_system_date_and_time() -> Result<()> {
    let get_system_date_and_time = GetSystemDateAndTime {};
    let soap_request = Body {
        payload: get_system_date_and_time,
    };
    let envelope = Envelope { body: soap_request };
    let envelope_serialize = quick_xml::se::to_string(&envelope)?;
    debug!("Envelope serialized: {:?}", envelope_serialize);

    // now lets check if we can reuse our serializers with another type that implements OnvifOperation
    let get_system_date_and_time_response = GetSystemDateAndTimeResponse {};
    let response_wrapped_in_soap = Envelope {
        body: Body {
            payload: get_system_date_and_time_response,
        },
    };
    let another_soapy_request_serialized = quick_xml::se::to_string(&response_wrapped_in_soap)?;
    debug!(
        "Yet another soapy request: {:?}",
        another_soapy_request_serialized
    );

    Ok(())
}
