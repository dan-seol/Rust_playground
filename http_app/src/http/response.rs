use std::io::{Write, Result as IoResult};
use std::{fmt::{Display, Formatter, Result as FmtResult}, net::TcpStream};
use super::StatusCode;

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }
    // stream: & mut TcpStream
    // dyn -> dynamic dispatch as concrete type is determined at runtime
    // &mut dyn Writ
    // static dispatch?
    // &mut impl Write
    // for static dispatch, for every call of this function with a concrete type, will create a typed version with the type
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}",
        self.status_code,
        self.status_code.reason_phrase(),
        body)
    }
}

/*
impl Display for Response {

    
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(f, "HTTP/1.1 {} {}\r\n\r\n{}",
        self.status_code,
        self.status_code.reason_phrase(),
        body
    )
    }
}
*/