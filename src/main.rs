extern crate app_dirs;
extern crate config;
extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
use std::io;
use std::str;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};
use tokio_proto::pipeline::ServerProto;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_service::Service;
use futures::{future, Future, BoxFuture};
use tokio_proto::TcpServer;
use std::net::SocketAddr;
use futures::Stream;
use futures::Sink;

pub struct ImapCodec;
impl Decoder for ImapCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
        if let Some(i) = buf.iter().position(|&b| b == b) {
            println!("some request happened");
            // remove the serialized frame from the buffer.
            let line = buf.split_to(i);

            // Also remove the '\n'
            buf.split_to(1);

            // Turn this data into a UTF string and return it in a Frame.
            match str::from_utf8(&line) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                             "invalid UTF-8")),
            }
        } else {
            Ok(None)
        }
    }
}

impl Encoder for ImapCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}

pub struct ImapProto;
impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for ImapProto {
    /// For this protocol style, `Request` matches the `Item` type of the codec's `Encoder`
    type Request = String;

    /// For this protocol style, `Response` matches the `Item` type of the codec's `Decoder`
    type Response = String;

    /// A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, ImapCodec>;
    type BindTransport = Box<Future<Item = Self::Transport,
        Error = io::Error>>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        // Construct the line-based transport
        let transport = io.framed(ImapCodec);

        // The handshake requires that the client sends `You ready?`,
        // so wait to receive that line. If anything else is sent,
        // error out the connection
        Box::new(transport.into_future()
            // If the transport errors out, we don't care about
            // the transport anymore, so just keep the error
            .map_err(|(e, _)| e)
            .and_then(|(line, transport)| {
                // A line has been received, check to see if it
                // is the handshake
                match line {
                    Some(ref msg) if msg.contains("CAPABILITY") => {
                        println!("SERVER: received client handshake");
                        // Send back the acknowledgement
                        let ret = transport.send("* CAPABILITY IMAP4rev1 AUTH=PLAIN LOGINDISABLED".into());
                        Box::new(ret) as Self::BindTransport
                    }
                    _ => {
                        // The client sent an unexpected handshake,
                        // error out the connection
                        println!("SERVER: client handshake INVALID");
                        let err = io::Error::new(io::ErrorKind::Other,
                                                 "invalid handshake");
                        let ret = future::err(err);
                        Box::new(ret) as Self::BindTransport
                    }
                }
            }))
    }
}

pub struct Imap;
impl Service for Imap {
    // These types must match the corresponding protocol types:
    type Request = String;
    type Response = String;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;

    // The future for computing the response; box it for simplicity.
    type Future = BoxFuture<Self::Response, Self::Error>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        // In this case, the response is immediate.
        future::ok(req).boxed()
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