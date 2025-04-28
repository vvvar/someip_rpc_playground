use std::{convert::Infallible, net::UdpSocket};

use generated::gauge_cluster_interface::{GaugeClusterInterface, Rpm, Speed};
use generated::gauge_cluster_stub::GaugeClusterStub;

use log::debug;

use application::config::{BUFFER_SIZE, SERVICE_ADDRESS};

#[derive(Debug)]
struct GaugeClusterInterfaceImpl;

impl GaugeClusterInterface for GaugeClusterInterfaceImpl {
    fn new() -> Self {
        Self {}
    }

    fn get_rpm(&self) -> Rpm {
        debug!("GaugeClusterInterfaceImpl::get_rpm()");
        Rpm { value: 1300 }
    }

    fn get_speed(&self) -> Speed {
        debug!("GaugeClusterInterfaceImpl::get_speed()");
        Speed { value: 42 }
    }
}

fn main() -> Infallible {
    simple_logger::init().unwrap();

    let socket = UdpSocket::bind(&SERVICE_ADDRESS).unwrap();

    let mut send_buffer = [u8::MIN; BUFFER_SIZE];
    let mut recv_buffer = [u8::MIN; BUFFER_SIZE];

    let stub = GaugeClusterStub::<GaugeClusterInterfaceImpl>::new();

    debug!("service is listening on {SERVICE_ADDRESS}");
    loop {
        let (_, sender_address) = socket.recv_from(&mut recv_buffer).unwrap();

        stub.handle_request(&mut recv_buffer, &mut send_buffer);
        dbg!(&send_buffer);

        socket.send_to(&send_buffer, &sender_address).unwrap();
        send_buffer.fill(0);
    }
}
