#![allow(unused)]

use crate::{request::HttpRequest, response::HttpResponse};

/// Service 处理 request
/// -----> 处理 response

pub trait Response {
    type Output;
}

pub struct Req<B> {
    re: B,
}

impl<B> Response for Req<B> {
    type Output = Result<HttpResponse<B>, Box<dyn std::error::Error>>;
}

pub trait Service<Request> {
    type Response;
    type Error;
    type Output: Response<Output = Result<Self::Response, Self::Error>>;
    fn call(&mut self, req: Request) -> Self::Output;
}
