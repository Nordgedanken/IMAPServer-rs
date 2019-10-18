use tokio_io::{AsyncRead, AsyncWrite};
use tokio_proto::pipeline::ServerProto;
use tokio_codec::Framed;
use std::io;
use std::result::Result;
use std::result::Result::Ok;

pub struct LineProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto {
    /// For this protocol style, `Request` matches the `Item` type of the codec's `Encoder`
    type Request = String;

    /// For this protocol style, `Response` matches the `Item` type of the codec's `Decoder`
    type Response = String;

    /// A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, super::codec::LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(Framed::new(io, super::codec::LineCodec))
    }
}
