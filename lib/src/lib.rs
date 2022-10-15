#[macro_use]
extern crate log;

pub mod discovery;
mod onvif_operation;
mod soap;
mod wsdl;
use std::{
    fmt::format,
    net::{IpAddr, SocketAddr},
};

use onvif_operation::OnvifOperation;
use serde::Serialize;
use soap::Soap;
use wsdl::get_system_date_and_time::GetSystemDateAndTime;

use anyhow::Result;

pub async fn get_system_date_and_time(addr: SocketAddr) -> Result<String> {
    let request_payload = GetSystemDateAndTime {};
    let soap_request_string = create_soap_request(request_payload)?;
    debug!("Soap request: {:?}", soap_request_string);
    let client = reqwest::Client::new();
    let res = client
        .post(format!("http://{}/onvif/device_service", addr.to_string()))
        .body(soap_request_string)
        .header("Content-Type", "application/soap+xml")
        .send()
        .await?;
    Ok(res.text().await?)
}

fn create_soap_request<T: OnvifOperation + Serialize>(onvif_operation: T) -> Result<String> {
    let soap_request = onvif_operation.apply_soap();
    let soap_request_string = quick_xml::se::to_string(&soap_request)?;
    Ok(soap_request_string)
}

#[cfg(test)]
mod tests {
    use crate::{create_soap_request, wsdl::get_system_date_and_time::GetSystemDateAndTime};

    #[test]
    fn test_soap_request_creation_for_get_system_date_and_time() {
        let get_system_date_and_time = GetSystemDateAndTime {};
        let soap_request_string = create_soap_request(get_system_date_and_time).unwrap();
        assert_eq!(
            "<Envelope><Body><GetSystemDateAndTime/></Body></Envelope>",
            soap_request_string
        );
    }
}
