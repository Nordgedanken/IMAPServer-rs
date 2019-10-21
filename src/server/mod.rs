use std::result::Result::Ok;

use async_bincode::*;
use tokio;
use tokio::net::TcpListener;
use tokio_tower::pipeline::Server;
use tower::{builder::ServiceBuilder};

mod codec;
mod service;

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = super::helper::get_config().expect("Unable to access config");

    let addr: String = config.ip.parse().unwrap();

    // The builder requires a protocol and an address
    let mut rx = TcpListener::bind(&(addr.parse().unwrap())).await.expect("bind");
    println!("Listening on: {}", addr);

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    let maker = ServiceBuilder::new().service(service::Echo);
    let (rx, _) = rx.accept().await.unwrap();
    let rx = AsyncBincodeStream::from(rx).for_async();
    let server = Server::new(rx, maker);

    tokio::spawn(async move { server.await.unwrap() });

    Ok(())
}
