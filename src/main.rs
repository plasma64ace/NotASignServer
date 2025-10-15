// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2025 Moew72 <Moew72@proton.me>

use std::ffi::{CString, c_char};
use std::mem::ManuallyDrop;
use std::ptr::null;

use data_encoding::HEXUPPER;
use ntex::web;
use serde::{Deserialize, Serialize};

mod lib {
    use std::ffi::*;
    type Func = extern "C" fn(*const c_char, *const c_uchar, c_int, c_int, *mut c_uchar);
    unsafe extern "C" {
        pub(super) static mut libs: *mut *const c_char;
        pub(super) static mut offset: usize;
        pub(super) static mut sign: Func;
        pub(super) fn load_module() -> c_int;
        pub(super) fn unload_module();
    }
}

fn set_libs(libs: Vec<&str>) {
    let mut libs = libs
        .iter()
        .map(|&x| ManuallyDrop::new(CString::new(x).unwrap()).as_ptr())
        .collect::<Vec<*const c_char>>();
    libs.push(null());
    unsafe {
        lib::libs = ManuallyDrop::new(libs).as_mut_ptr();
    }
}

fn set_offset(offset: usize) {
    unsafe {
        lib::offset = offset;
    }
}

fn load_module() {
    let ret = unsafe { lib::load_module() };
    if ret != 0 {
        panic!("load module error.");
    }
}

#[allow(unused)]
fn unload_module() {
    unsafe { lib::unload_module() }
}

fn sign(cmd: &str, src: &[u8], seq: i32) -> [Vec<u8>; 3] {
    const TOKEN_DATA_OFFSET: usize = 0x000;
    const TOKEN_LEN_OFFSET: usize = 0x0FF;
    const EXTRA_DATA_OFFSET: usize = 0x100;
    const EXTRA_LEN_OFFSET: usize = 0x1FF;
    const SIGN_DATA_OFFSET: usize = 0x200;
    const SIGN_LEN_OFFSET: usize = 0x2FF;

    let c_cmd = CString::new(cmd).unwrap();
    let mut buf = [0u8; 0x300];
    let _ = unsafe {
        lib::sign(
            c_cmd.as_ptr(),
            src.as_ptr(),
            src.len() as i32,
            seq,
            buf.as_mut_ptr(),
        )
    };

    let token_len = buf[TOKEN_LEN_OFFSET];
    let token = &buf[TOKEN_DATA_OFFSET..TOKEN_DATA_OFFSET + token_len as usize];
    let extra_len = buf[EXTRA_LEN_OFFSET];
    let extra = &buf[EXTRA_DATA_OFFSET..EXTRA_DATA_OFFSET + extra_len as usize];
    let sign_len = buf[SIGN_LEN_OFFSET];
    let sign = &buf[SIGN_DATA_OFFSET..SIGN_DATA_OFFSET + sign_len as usize];

    [Vec::from(token), Vec::from(extra), Vec::from(sign)]
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    set_libs(vec!["libgnutls.so.30", "./libsymbols.so"]);
    set_offset(0x5ADE220);
    load_module();
    web::HttpServer::new(|| web::App::new().service(sign_service))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[web::post("/")]
async fn sign_service(params: web::types::Json<Params>) -> impl web::Responder {
    let ret = HEXUPPER.decode(params.src.to_uppercase().as_bytes());
    let src = match ret {
        Ok(v) => v,
        Err(err) => return web::HttpResponse::InternalServerError().body(err.to_string()),
    };
    let [token, extra, sign] = sign(&params.cmd, &src, params.seq);
    let token = HEXUPPER.encode(&token);
    let extra = HEXUPPER.encode(&extra);
    let sign = HEXUPPER.encode(&sign);
    let value = Value { token, extra, sign };
    let body = RespBody { value };
    web::HttpResponse::Ok().json(&body)
}

#[derive(Deserialize)]
struct Params {
    cmd: String,
    src: String,
    seq: i32,
}

#[derive(Serialize)]
struct RespBody {
    value: Value,
}

#[derive(Serialize)]
struct Value {
    token: String,
    extra: String,
    sign: String,
}
