use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display,Result as FmtResult, Formatter};

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
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Request, Self::Error> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {

}
