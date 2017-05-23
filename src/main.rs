extern crate app_dirs;
extern crate config;
extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use futures::future;

use tokio_io::codec::{Encoder, Decoder, Framed};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_proto::pipeline::ServerProto;
use tokio_service::Service;
use tokio_proto::TcpServer;

use bytes::{BytesMut};

use std::io;
use std::net::SocketAddr;

mod request;
mod response;

pub use request::Request;
pub use response::Response;

pub struct ImapCodec;
impl Decoder for ImapCodec {
    type Item = Request;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Request>> {
        request::decode(buf)
    }
}

impl Encoder for ImapCodec {
    type Item = Response;
    type Error = io::Error;

    fn encode(&mut self, msg: Response, buf: &mut BytesMut) -> io::Result<()> {
        response::encode(msg, buf);
        Ok(())
    }
}

pub struct ImapProto;
impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for ImapProto {
    type Request = Request;
    type Response = Response;
    type Transport = Framed<T, ImapCodec>;
    type BindTransport = io::Result<Framed<T, ImapCodec>>;

    fn bind_transport(&self, io: T) -> io::Result<Framed<T, ImapCodec>> {
        Ok(io.framed(ImapCodec))
    }
}

pub struct Imap;
impl Service for Imap {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>;

    fn call(&self, _request: Request) -> Self::Future {
        let mut resp = Response::new();
        resp.body("* CAPABILITY IMAP4rev1 AUTH=PLAIN LOGINDISABLED");
        future::ok(resp)
    }
}

fn main() {
    let mut config = helper::get_config();
    config.set_default("RFC", "3501").unwrap();
    config.set_default("address", "0.0.0.0:143").unwrap();
    // Specify the localhost address
    let addr: SocketAddr =  config.get_str("address").unwrap().parse().unwrap();

    // The builder requires a protocol and an address
    let server = TcpServer::new(ImapProto, addr);

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    server.serve(|| Ok(Imap));
}

mod helper;