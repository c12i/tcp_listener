use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::str::{self, Utf8Error};
use std::fmt::{Display,Result as FmtResult, Formatter};
use crate::utils::get_next_word;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn query(&self) -> Option<&QueryString> {
        self.query.as_ref()
    }

    pub fn method(&self) -> &Method {
        &self.method
    }
}

// Type aliasing
type HttpResult<'buf, T> = Result<T, ParseError>;

// when we implement `TryFrom`, the compiler will auto-generate code that implements `TryInto` trait for type T
impl<'buf> TryFrom<&'buf[u8]> for Request<'buf> {
    type Error = ParseError;

    // to => GET /search?name=example&sort=1 HTTP/1.1
    fn try_from(value: &'buf[u8]) -> HttpResult<Request<'buf>> {
        // convert &[u8] to &str
        let request = str::from_utf8(value)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        // check http type
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method:Method = method.parse()?;

        // handling the query string, if present, reassign to query var
        // else leave as None
        let mut query = None;
        if let Some(i) = path.find('?') {
            query = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query,
            method,
        })
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

// Swap Utf8Error to InvalidEncodingError
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

// Swap MethodError to InvalidEncodingError
impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Error for ParseError {}
