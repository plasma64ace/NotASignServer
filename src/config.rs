// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2025 Moew72 <Moew72@proton.me>

use std::fs;
use std::io::Read;
use std::sync::LazyLock;

use serde::Deserialize;

pub(crate) static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    if fs::exists("sign.config.toml").unwrap() {
        let mut buf = Vec::new();
        fs::File::open("sign.config.toml")
            .unwrap()
            .read_to_end(&mut buf)
            .unwrap();
        toml::from_slice::<Config>(&buf).unwrap()
    } else {
        Config::default()
    }
});

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) preloads: Vec<String>,
    pub(crate) listen: String,
    pub(crate) offset: usize,
    pub(crate) version: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            preloads: vec!["libgnutls.so.30".to_string(), "./libsymbols.so".to_string()],
            listen: "127.0.0.1:8080".to_string(),
            offset: 0x5ADE220,
            version: "3.2.19-39038".to_string(),
        }
    }
}
