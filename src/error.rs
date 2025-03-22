#![allow(unused)]

use thiserror::Error;

/// 请求类型错误
#[derive(Debug, Error)]
pub enum RequestError {
    #[error("skip newline error")]
    SkipNewLineError,
    #[error("parse header error")]
    ParseHeaderError,
    #[error("invalid version")]
    InvalidVersion,
}

/// 响应类型错误
#[derive(Debug, Error)]
pub enum ResponseError {}

/// 方法类型错误
#[derive(Debug, Error)]
pub enum HttpMethodError {
    #[error("invalid method")]
    InvalidMethod,
}

/// uri 类型错误
#[derive(Debug, Error)]
pub enum UriError {
    #[error("invalid uri")]
    InvalidUri,
}

/// 版本类型错误
#[derive(Debug, Error)]
pub enum HttpVersionError {
    #[error("invalid version")]
    InvalidVersion,
}

/// 其他类型的错误
#[derive(Debug, Error)]
pub enum OtherError {
    #[error("parse space error")]
    ParseSpaceError,
    #[error("parse newline error")]
    ParseNewlineError,
    #[error("parse map error")]
    ParseMapError,
    #[error("parse blankline error")]
    ParseBlanklineError,
}
