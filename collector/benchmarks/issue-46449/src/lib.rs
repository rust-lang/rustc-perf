extern crate futures;

use futures::{Future, Poll};

const BUFFER_SIZE: usize = 1;
pub struct Error(::std::io::Error);

struct Dummy<T>(T);
impl<T> Future for Dummy<T> {
    type Item = T;
    type Error = Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {}
    }
}

pub fn run() -> Box<Future<Item = (), Error = Error>> {
    let c2s = Dummy([0u8; BUFFER_SIZE]).then(move |_| Ok(0));
    let s2c = Dummy(()).then(move |_| Ok(0));
    let fut = c2s.select(s2c)
        .and_then(move |_| Ok(()))
        .map_err(|(err, _)| err);
    Box::new(fut)
}
