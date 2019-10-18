use std::boxed::Box;
use std::result::Result::Ok;

use futures::{Async, Future, future, Poll};
use tower_service::Service;

#[derive(Serialize, Deserialize)]
pub struct Request {
    tag: usize,
    value: u32,
}

impl Request {
    pub fn new(val: u32) -> Self {
        Request { tag: 0, value: val }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    tag: usize,
    value: u32,
}

impl From<Request> for Response {
    fn from(r: Request) -> Response {
        Response {
            tag: r.tag,
            value: r.value,
        }
    }
}

impl Response {
    pub fn check(&self, expected: u32) {
        assert_eq!(self.value, expected);
    }

    pub fn get_tag(&self) -> usize {
        self.tag
    }
}

impl Request {
    pub fn set_tag(&mut self, tag: usize) {
        self.tag = tag;
    }
}

pub struct Echo;

impl Service<Request> for Echo {
    // These types must match the corresponding protocol types:
    type Response = Response;

    type Error = ();

    // The future for computing the response; box it for simplicity.
    type Future = future::FutureResult<Self::Response, Self::Error>;


    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(Async::Ready(()))
    }

    // Produce a future for computing a response from a request.
    fn call(&mut self, req: Request) -> Self::Future {
        // In this case, the response is immediate.
        future::ok(Response::from(req))
    }
}
