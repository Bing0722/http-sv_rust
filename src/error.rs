#![allow(unused)]

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("skip newline error")]
    SkipNewLineError,
    #[error("parse header error")]
    ParseHeaderError,
    #[error("invalid version")]
    InvalidVersion,
}

#[derive(Debug, Error)]
pub enum ResponseError {}

#[derive(Debug, Error)]
pub enum HttpMethodError {
    #[error("invalid method")]
    InvalidMethod,
}

#[derive(Debug, Error)]
pub enum UriError {
    #[error("invalid uri")]
    InvalidUri,
}

#[derive(Debug, Error)]
pub enum HttpVersionError {
    #[error("invalid version")]
    InvalidVersion,
}

#[derive(Debug, Error)]
pub enum HostError {
    #[error("invalid host")]
    InvalidHost,
}

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
