mod response_util;

mod response;

// 暴露接口
pub use response::return_response;

pub use response_util::HttpResponse;

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {}
}
