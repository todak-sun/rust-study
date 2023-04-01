use super::method::Method;
use std::convert::TryFrom;
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}
impl Request {
    pub fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        Ok()
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        String::from_utf16_lossy(value);
    }
}
