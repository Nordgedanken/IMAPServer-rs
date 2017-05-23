use bytes::{BytesMut, BufMut};

pub struct Response {
    response: String,
}

impl Response {
    pub fn new() -> Response {
        Response {
            response: String::new(),
        }
    }

    pub fn body(&mut self, s: &str) -> &mut Response {
        self.response = s.to_string();
        self
    }
}

pub fn encode(msg: Response, buf: &mut BytesMut) {
    push(buf, "\r\n".as_bytes());
    push(buf, msg.response.as_bytes());
}

fn push(buf: &mut BytesMut, data: &[u8]) {
    buf.reserve(data.len());
    unsafe {
        buf.bytes_mut()[..data.len()].copy_from_slice(data);
        buf.advance_mut(data.len());
    }
}