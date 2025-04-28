use log::debug;
use someip_parse::{MessageType, SomeipHeader, SomeipMsgSlice};

use super::gauge_cluster_interface::GaugeClusterInterface;

#[derive(Debug)]
pub struct GaugeClusterStub<T: GaugeClusterInterface> {
    interface: T,
}

impl<T: GaugeClusterInterface> GaugeClusterStub<T> {
    pub fn new() -> Self {
        debug!("GaugeClusterStub::new()");

        Self {
            interface: T::new(),
        }
    }

    pub fn handle_request(&self, input_buffer: &[u8], output_buffer: &mut [u8]) {
        debug!("GaugeClusterStub::handle_request()");

        let request_message = SomeipMsgSlice::from_slice(input_buffer).unwrap();
        let request_header = request_message.to_header();
        let _ = request_message.payload();
        debug!("dispatching message with header {:?}", &request_header);

        let result: Box<dyn erased_serde::Serialize> = match request_header.method_id() {
            Some(1) => Box::new(self.interface.get_rpm()),
            Some(2) => Box::new(self.interface.get_speed()),
            _ => unimplemented!("call to unsupported method"),
        };

        let mut response_buffer = Vec::<u8>::new();

        let response_header = SomeipHeader {
            message_id: request_message.message_id(),
            length: 8 + 800, // Hadcoded length to simplify stuff for now.
            request_id: request_message.request_id() + 1,
            interface_version: request_message.interface_version(),
            message_type: MessageType::Response,
            return_code: 1,
            tp_header: None,
        };
        response_header.write_raw(&mut response_buffer).unwrap();

        let serizlizer = &mut serde_json::Serializer::new(&mut response_buffer);
        let erased_serizlizer = &mut <dyn erased_serde::Serializer>::erase(serizlizer);
        result.erased_serialize(erased_serizlizer).unwrap();
    }
}
