use super::method::Method;
use std::convert::TryFrom;

pub struct Request {
    path: String,
    query: Option<String>,
    method: Method,
}

impl Request {
    fn from_byte_array(buffer: &[u8]) -> Result<Self, String> {
        unimplemented!()
    }
}

// when we implement `TryFrom`, the compiler will auto-generate code that implements `TryInto` trait for type T
impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Request, Self::Error> {
        unimplemented!()
    }
}
