use std::io;

use bytes::BytesMut;

pub struct Request {
    data: BytesMut,
}

pub fn decode(buf: &mut BytesMut) -> io::Result<Option<Request>> {
    Ok(Request {
        data: buf.take(),
    }.into())
}