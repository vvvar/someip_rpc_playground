use std::net::UdpSocket;

use generated::gauge_cluster_proxy::GaugeClusterProxy;
use generated::gauge_cluster_interface::Speed;
use application::config::{BUFFER_SIZE, SERVICE_ADDRESS};

fn main() {
    simple_logger::init().unwrap();

    let socket = UdpSocket::bind(&"127.0.0.1:0").unwrap();

    let proxy = GaugeClusterProxy::new();

    let bytes_to_send = proxy.get_speed();

    socket.send_to(&bytes_to_send, &SERVICE_ADDRESS).unwrap();

    let mut recv_buffer = [u8::MIN; BUFFER_SIZE];
    socket.recv(&mut recv_buffer).unwrap();

    dbg!(proxy.parse::<Speed>(&recv_buffer));
}
