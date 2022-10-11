/// This trait is to be used by all Onvif Operations
/// that are mentioned in their WSDL
pub trait OnvifOperation {
    /// Return a string that is the same as the name
    /// of operation mentioned in WSDL
    fn get_operation_name(&self) -> String;
}
