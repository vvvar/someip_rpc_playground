use log::debug;
use serde::Deserialize;
use someip_parse::{MessageType, SomeipHeader, SomeipMsgSlice};

#[derive(Debug)]
pub struct GaugeClusterProxy;

impl GaugeClusterProxy {
    pub fn new() -> Self {
        debug!("GaugeClusterProxy::new()");

        Self {}
    }

    pub fn get_rpm(&self) -> Vec<u8> {
        debug!("GaugeClusterProxy::get_rpm()");

        let mut request_bytes = Vec::<u8>::new();
        let mut header = SomeipHeader {
            message_id: 1,
            length: 8 + 42, // Hadcoded length to simplify stuff for now.
            request_id: 1,
            interface_version: 1,
            message_type: MessageType::Request,
            return_code: 1,
            tp_header: None,
        };
        header.set_method_id(1);
        header.write_raw(&mut request_bytes).unwrap();

        request_bytes
    }

    pub fn get_speed(&self) -> Vec<u8> {
        debug!("GaugeClusterProxy::get_speed()");

        let mut request_bytes = Vec::<u8>::new();
        let mut header = SomeipHeader {
            message_id: 1,
            length: 8 + 42, // Hadcoded length to simplify stuff for now.
            request_id: 1,
            interface_version: 1,
            message_type: MessageType::Request,
            return_code: 1,
            tp_header: None,
        };
        header.set_method_id(2);
        header.write_raw(&mut request_bytes).unwrap();

        request_bytes
    }

    pub fn parse<'a, T: Deserialize<'a>>(&self, data: &'a [u8]) -> T {
        debug!("GaugeClusterProxy::parse()");

        let request_message = SomeipMsgSlice::from_slice(data).unwrap();
        let request_header = request_message.to_header();
        let payload = request_message.payload();
        debug!("dispatching message with header {:?}", &request_header);

        serde_json::from_slice(payload).unwrap()
    }
}
