//! 处理请求

pub(crate) trait _Handle<Request> {
    type Response;
    type Error;
    fn handel_request(&mut self, request: Request) -> Result<Self::Response, Self::Error>;
}
