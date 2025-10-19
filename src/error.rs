// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2025 Moew72 <Moew72@proton.me>

use ntex::web;
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("std io: `{0}`")]
    Io(#[from] std::io::Error),
    #[error("from utf8: `{0}`")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error("data_encoding: `{0}`")]
    DataEncoding(#[from] data_encoding::DecodeError),
    #[error("ntex runtime JoinHandle: `{0}`")]
    NtexRuntimeJoinHandle(#[from] ntex::rt::JoinError),
    #[error("other: `{0}`")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) trait Context<T> {
    fn context(self, ctx: Error) -> Result<T>;
}

impl<T> Context<T> for Option<T> {
    fn context(self, ctx: Error) -> Result<T> {
        match self {
            Some(v) => Ok(v),
            None => Err(ctx),
        }
    }
}

impl web::WebResponseError for Error {
    fn error_response(&self, _: &ntex::web::HttpRequest) -> ntex::http::Response {
        #[derive(Debug, Serialize)]
        struct RespBody {
            status: u16,
            message: String,
        }
        web::HttpResponse::build(self.status_code()).json(&RespBody {
            status: 400,
            message: self.to_string(),
        })
    }
}
