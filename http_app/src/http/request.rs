use std::str::Utf8Error;
use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::str;
use super::{QueryString, QueryStringValue};

/*
pub struct Request {
    path: String,
    query_string: Option<String>,
    // parent module??
    method: Method,
}
*/

#[derive(Debug)]
pub struct Request<'buf> { //generic over lifetime
    //we need lifetime specifiers now
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
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

//Getters
impl<'buf> Request<'buf> {
    //Convention is to name the getter after the field without the "get" prefix
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }
    //reference to option ??
    pub fn query_string(&self) -> Option<&QueryString> {
        // converts from &Option<T> to Option<&T>
        self.query_string.as_ref()
    }
}

//lifetime 'buf declared for impl and the type
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;
    
    // GET /search?name=abs&sort=1 HTTP/1.1
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        /*
        /*
        let string = String::from("asd");
        string.encrypt();
        &buf.encrypt();
        unimplemented!()
        */
        match str::from_utf8(buf) {
            Ok(request) => {},
            Err(_) => return Err(ParseError::InvalidEncoding.to_string()),
        }

        match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
            Ok(request) => {},
            Err(e) => return Err(e),
        }
        */
        let request = str::from_utf8(buf)?;
        /*
        match get_next_word(request) {
            Some((method, request)) => {},
            None => return Err(ParseError::InvalidRequest),
        }
        */
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        let mut query_string = None;
        /*
        match path.find('?') {
            Some(i) => {
                query_string = Some(&path[i+1..]);
                path = &path[..i];
            }

            None => {}
        }
        */
        /*
        let q = path.find('?');
        if q.is_some() {
            let i = q.unwrap();
            query_string = Some(&path[i+1..]);
            path = &path[..i];
        }
        */

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }

        Ok(Self {
            path: &path,
            query_string,
            method,
        })

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

/*
fn get_next_word<'a, 'b>(request: &'a str, b: &'b str) -> Option<(&'a str, &'a str)> {
 */
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    /* 
    let mut iter = request.chars();

    loop {
        let item = iter.next();
        match item {
            Some(c) => {}
            None => break,
        }
    }
    */
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i+1..]));
        }
    }

    None
    //unimplemented!()
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}


impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method"
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_ : Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_ : MethodError) -> Self {
        Self::InvalidMethod
    }
}