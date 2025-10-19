# Linux QQ SignServer

This project can only be used on Linux.

## How to use?

First, go to the official website to download QQ.

I recommend downloading the Linux x64 3.2.19-39038 version. If you want use another version, go to visit [Config section](#config).

Then unzip or install QQ.

And then:

```sh
gcc -std=c99 -shared -fPIC -o libsymbols.so symbols.c
cargo build --release
```

Place the `libsymbols.so` and `target/release/sign` files into the folder that contains the `wrapper.node` file.

Switch the directory to the folder containing `wrapper.node`, and then run `./sign`.

The server will listen on `127.0.0.1:8080`. If you want to listen on other endpoints, go to modify the `src/main.rs` file.

Enjoy!

## Config

Copy the `sign.config.toml` file to the folder where the `wrapper.node` is located.

Then modify the configuration file according to your needs.

If you use other versions of QQ, put `{version}.json` in the same directory. For specific content, refer to `src/appinfo/3.2.19-39038.json`.
