use std::collections::HashMap;

use tracing::trace;

use crate::{
    error::{RequestError, ResponseError},
    handle_request,
    headers::{HttpMethod, IntoHttpMethod},
    request::Request,
    response::Response,
    server::{IncomingStream, Service},
};

/// 路由
pub struct Router {
    path_router: HashMap<
        (String, HttpMethod),
        Box<dyn Service<Request, Response = Response, Error = ResponseError>>,
    >,
}

impl Router {
    pub fn new() -> Self {
        Self {
            path_router: HashMap::new(),
        }
    }

    // 创建并插入
    pub fn route(
        mut self,
        path: &str,
        method: impl IntoHttpMethod,
        handle: impl Service<Request, Response = Response, Error = ResponseError> + 'static,
    ) -> Self {
        self.path_router.insert(
            (path.to_string(), method.into_http_method()),
            Box::new(handle),
        );
        self
    }

    pub fn handle(&mut self, req: Request) -> Response {
        let path = req.path_ref();
        let method = req.method_ref();
        trace!("{}", req.start_line);
        if let Some(handle) = self.path_router.get_mut(&(path.to_string(), *method)) {
            let s = handle.call(req);
            s.unwrap()
        } else {
            Response::not_found().body("404 Not Found".into())
        }
    }
}

impl Service<&mut IncomingStream> for Router {
    type Response = Request;
    type Error = RequestError;
    fn call(&mut self, req: &mut IncomingStream) -> Result<Self::Response, Self::Error> {
        let incoming = req;
        let stream = incoming.stream_mut();
        let req = handle_request(stream).unwrap();
        Ok(req)
    }
}
