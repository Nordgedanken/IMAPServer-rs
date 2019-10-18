use std::result::Result::Ok;

use tokio_proto::TcpServer;

mod proto;
mod codec;
mod service;

pub fn main() {
    let config = super::helper::get_config().expect("Unable to access config");

    let addr = config.ip.parse().unwrap();

    // The builder requires a protocol and an address
    let server = TcpServer::new(proto::LineProto, addr);
    println!("Listening on: {}", addr);

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    server.serve(|| Ok(service::Echo));
}
