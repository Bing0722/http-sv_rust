mod response_util;

mod response;

// 暴露接口
pub use response::return_response;

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {
        // let rsp = Response::new();
        // app.route("GET", "/", handler);
        // {
        //     fn handler() -> response {
        //         response();
        //     }
        // }

        // {
        //     // 解析请求
        //     fn route() {
        //         request();
        //     }
        // }
        // service::run();
    }
}
