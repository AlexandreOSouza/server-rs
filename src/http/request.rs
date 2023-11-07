use super::method::{Method, MethodError};
use std::{
    convert::TryFrom,
    fmt::{Debug, Display},
    io::Write,
    str::{self, Utf8Error},
};

pub struct Request<'a> {
    query: &'a str,
    path: Option<&'a str>,
    method: Method,
}

impl<'a> TryFrom<&'a [u8]> for Request<'a> {
    type Error = ParseError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(value)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_sting = None;

        if let Some(i) = path.find('?') {
            query_sting = Some(&path[i + 1..]);
            path = &path[..i];
        }

        Ok(Self {
            query: path,
            path: query_sting,
            method,
        })
    }
}

fn get_next_word(value: &str) -> Option<(&str, &str)> {
    for (idx, c) in value.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&value[..idx], &value[idx + 1..]));
        }
    }
    None
}

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

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
