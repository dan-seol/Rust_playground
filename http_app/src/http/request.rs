use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct Request {
    path: String,
    query_string: Option<String>,
    // parent module??
    method: Method,
}

/*
impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        //implementing From, shouldn't fail
        //if it can fail, use TryFrom
    }
}
*/

impl TryFrom<&[u8]> for Request {
    type Error = String;
    
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let string = String::from("asd");
        string.encrypt();
        &buf.encrypt();
        unimplemented!()
    }
}

trait Encrypt {
    fn encrypt(&self) -> Self;
}

impl Encrypt for String {
    fn encrypt(&self) -> Self {
        unimplemented!()
    }
}

impl Encrypt for &[u8] {
    fn encrypt(&self) -> Self {
        unimplemented!()
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Error for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {}
}