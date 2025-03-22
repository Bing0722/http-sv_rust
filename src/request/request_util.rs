#![allow(unused)]

use crate::{
    body::Body,
    utils::{HttpHeaders, HttpMethod, HttpVersion},
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpRequest<B> {
    pub(crate) start_line: StartLine,
    pub(crate) headers: HttpHeaders,
    pub(crate) body: Body<B>,
}

impl<T> HttpRequest<T> {
    pub(crate) fn new(start_line: StartLine, headers: HttpHeaders, body: Body<T>) -> Self {
        HttpRequest {
            start_line,
            headers,
            body,
        }
    }
}

#[derive(Debug)]
pub struct StartLine {
    pub(crate) method: HttpMethod,
    pub(crate) path: String,
    pub(crate) version: HttpVersion,
}

impl ToString for StartLine {
    fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            self.method.to_string(),
            self.path,
            self.version.to_string()
        )
    }
}
