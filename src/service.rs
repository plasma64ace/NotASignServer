// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2025 Moew72 <Moew72@proton.me>

use std::fs;
use std::io::Read;

use data_encoding::HEXUPPER;
use ntex::http::Response;
use ntex::http::StatusCode;
use ntex::http::header::CONTENT_TYPE;
use ntex::rt::spawn;
use ntex::web;
use serde::{Deserialize, Serialize};

use crate::config::CONFIG;
use crate::error::{Context, Error, Result};

#[derive(rust_embed::Embed)]
#[folder = "src/appinfo"]
struct Asset;

#[derive(Deserialize)]
struct Params {
    cmd: String,
    src: String,
    seq: i32,
}

#[web::get("/")]
pub(crate) async fn sign_get(params: web::types::Query<Params>) -> Result<Response> {
    sign_common(params.0).await
}

#[web::post("/")]
pub(crate) async fn sign_post(params: web::types::Json<Params>) -> Result<Response> {
    sign_common(params.0).await
}

#[inline]
async fn sign_common(params: Params) -> Result<Response> {
    #[derive(Serialize)]
    struct RespBody {
        platform: String,
        version: String,
        value: Value,
    }
    #[derive(Serialize)]
    struct Value {
        token: String,
        extra: String,
        sign: String,
    }
    let src = HEXUPPER.decode(params.src.to_uppercase().as_bytes())?;
    let [token, extra, sign] =
        spawn(async move { crate::sign::sign(&params.cmd, &src, params.seq) }).await?;
    let token = HEXUPPER.encode(&token);
    let extra = HEXUPPER.encode(&extra);
    let sign = HEXUPPER.encode(&sign);
    let value = Value { token, extra, sign };
    let body = RespBody {
        platform: "Linux".to_string(),
        value,
        version: CONFIG.version.clone(),
    };
    let resp = web::HttpResponse::Ok().json(&body);
    Ok(resp)
}

#[web::get("/appinfo")]
pub(crate) async fn appinfo() -> Result<Response> {
    let path = &format!("{}.json", CONFIG.version.clone());
    let body = if fs::exists(path)? {
        let mut buf = String::new();
        fs::File::open(path)?.read_to_string(&mut buf)?;
        buf
    } else {
        let data = Asset::get(path)
            .context(Error::Other(format!("appinfo {} not found.", path)))?
            .data
            .to_vec();
        String::from_utf8(data)?
    };
    let resp = web::HttpResponse::build(StatusCode::OK)
        .set_header(CONTENT_TYPE, "application/json")
        .body(body);
    Ok(resp)
}
