//! 错误

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("解析分隔符失败")]
    ParseSeparatorErr,
    #[error("解析新行失败")]
    ParseNewlineErr,
    #[error("解析空格失败")]
    ParseSpaceErr,
    #[error("解析键值对失败")]
    ParseMapErr,
    #[error("解析消耗失败")]
    ParseConsumeErr,
}

#[derive(Debug, Error)]
pub enum ResponseError {}

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("读取请求失败")]
    ReadRequestErr,
    #[error("请求为空")]
    EmptyRequest,
    #[error("解析错误--> {0}")]
    ParseError(#[from] ParseError),
}
