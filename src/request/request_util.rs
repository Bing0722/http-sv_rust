#![allow(unused)]

use crate::utils::{HttpHeaders, HttpMethod, HttpVersion};
use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpRequest<B> {
    // TODO: Implement the start_line field
    // pub(crate) start_line: String,
    pub(crate) headers: HttpRequestHeader,
    pub(crate) body: Option<B>,
}

impl<T> HttpRequest<T> {
    pub(crate) fn new(headers: HttpRequestHeader, body: Option<T>) -> Self {
        HttpRequest { headers, body }
    }
}

#[derive(Debug)]
pub struct HttpRequestHeader {
    pub(crate) method: HttpMethod,
    pub(crate) path: String,
    pub(crate) version: HttpVersion,
    pub(crate) headers: HttpHeaders,
    pub(crate) host: String,
}
