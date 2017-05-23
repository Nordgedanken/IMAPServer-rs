extern crate app_dirs;
extern crate config;
extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use futures::{future, Future, Stream};

use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Encoder, Decoder, Framed};
use tokio_proto::streaming::pipeline::{Frame, ServerProto};
use tokio_proto::streaming::{Body, Message};
use tokio_proto::TcpServer;
use tokio_service::Service;

use bytes::{BytesMut};

use std::{io, str};
use std::net::SocketAddr;

pub struct ImapCodec {
    decoding_head: bool,
}
impl Decoder for ImapCodec {
    type Item = Frame<String, String, io::Error>;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut)
              -> Result<Option<Self::Item>, io::Error>
    {
        // Find the position of the next newline character and split off the
        // line if we find it.
        let line = match buf.iter().position(|b| *b == b'\n') {
            Some(n) => buf.split_to(n),
            None => return Ok(None),
        };

        // Also remove the '\n'
        buf.split_to(1);

        // Turn this data into a string and return it in a Frame
        let s = try!(str::from_utf8(&line).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, e)
        }));

        // Got an empty line, which means that the state
        // should be toggled.
        if s == "" {
            let decoding_head = self.decoding_head;
            // Toggle the state
            self.decoding_head = !decoding_head;

            if decoding_head {
                Ok(Some(Frame::Message {
                    // The message head is an empty line
                    message: s.to_string(),
                    // We will be streaming a body next
                    body: true,
                }))
            } else {
                // The streaming body termination frame,
                // is represented as `None`.
                Ok(Some(Frame::Body {
                    chunk: None
                }))
            }
        } else {
            if self.decoding_head {
                // This is a "oneshot" message with no
                // streaming body
                Ok(Some(Frame::Message {
                    message: s.to_string(),
                    body: false,
                }))
            } else {
                // Streaming body line chunk
                Ok(Some(Frame::Body {
                    chunk: Some(s.to_string()),
                }))
            }
        }
    }
}

impl Encoder for ImapCodec {
    type Item = Frame<String, String, io::Error>;
    type Error = io::Error;

    fn encode(&mut self, msg: Frame<String, String, io::Error>, buf: &mut BytesMut) -> io::Result<()> {
        match msg {
            Frame::Message { message, body } => {
                // Our protocol dictates that a message head that
                // includes a streaming body is an empty string.
                assert!(message.is_empty() == body);

                buf.extend(message.as_bytes());
            }
            Frame::Body { chunk } => {
                if let Some(chunk) = chunk {
                    buf.extend(chunk.as_bytes());
                }
            }
            Frame::Error { error } => {
                // Our protocol does not support error frames, so
                // this results in a connection level error, which
                // will terminate the socket.
                return Err(error)
            }
        }

        // Push the new line
        buf.extend(b"\n");

        Ok(())
    }
}

pub struct ImapProto;
impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for ImapProto {
    type Request = String;
    type RequestBody = String;
    type Response = String;
    type ResponseBody = String;
    type Error = io::Error;

    // `Framed<T, LineCodec>` is the return value
    // of `io.framed(LineCodec)`
    type Transport = Framed<T, ImapCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
    // Initialize the codec to be parsing message heads
    let codec = ImapCodec {
        decoding_head: true,
    };

    Ok(io.framed(codec))
}
}

pub struct Imap;
impl Service for Imap {
    type Request = Message<String, Body<String, io::Error>>;
    type Response = Message<String, Body<String, io::Error>>;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response,
        Error = Self::Error>>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        let resp = Message::WithoutBody("Ok".to_string());

        match req {
            Message::WithoutBody(line) => {
                println!("{}", line);
                Box::new(future::done(Ok(resp)))
            }
            Message::WithBody(_, body) => {
                let resp = body
                    .for_each(|line| {
                        println!(" + {}", line);
                        Ok(())
                    })
                    .map(move |_| resp);

                Box::new(resp) as Self::Future
            }
        }
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