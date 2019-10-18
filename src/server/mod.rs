use std::net::SocketAddr;
use std::result::Result::Ok;

use async_bincode::*;
use tokio;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio_proto::TcpServer;
use tokio_tower::pipeline::Server;
use tower::{builder::ServiceBuilder};

mod codec;
mod service;

pub fn main() {
    let config = super::helper::get_config().expect("Unable to access config");

    let addr: String = config.ip.parse().unwrap();

    // The builder requires a protocol and an address
    let bind = TcpListener::bind(&(addr.parse().unwrap())).expect("bind");
    println!("Listening on: {}", addr);

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    let maker = ServiceBuilder::new().service(service::Echo);
    let rx = bind
        .incoming()
        .into_future()
        .map_err(|_| ())
        .map(|(stream, _)| stream.unwrap())
        .map(AsyncBincodeStream::from)
        .map(AsyncBincodeStream::for_async)
        .map_err(|_| ())
        .map(|stream| Server::new(stream, maker));

    tokio::run(
        rx.and_then(|srv| srv.map_err(|_| ()))
            .map_err(|_| ()),
    );
}
