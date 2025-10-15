// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2025 Moew72 <Moew72@proton.me>

fn main() {
    println!("cargo:rerun-if-changed=src/sign.c");
    cc::Build::new().file("src/sign.c").compile("sign");
}
