use std::net::SocketAddr;

use crate::{onvif_operation::OnvifOperation, soap::Envelope};

pub fn send_request<T: OnvifOperation>(addr: SocketAddr, payload: Envelope<T>) {}
