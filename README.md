# Linux QQ SignServer

This project can only be used on Linux.

[中文](README.zh.md)

## How to use?

First, go to the official website to download deb file of the QQ, and rename it to `linuxqq.deb`.

I recommend downloading the Linux x64 3.2.19-39038 version. If you want use another version, go to visit [Config section](#config).

And then:

```sh
ar -p linuxqq.deb data.tar.xz | tar -xJ ./opt/QQ/resources/app/wrapper.node -O > wrapper.node && rm linuxqq.deb

patchelf \
--remove-needed libbugly.so \
--remove-needed libX11.so.6 \
--remove-needed libX11-xcb.so.1 \
--remove-needed libXext.so.6 \
--remove-needed libvips-cpp.so.42 \
--add-needed libstdc++.so.6 \
--add-needed libsymbols.so \
wrapper.node
```

If you have Docker, you may refer to the [Docker section](#docker) for further information.

```sh
gcc -std=c99 -shared -fPIC -o libsymbols.so symbols.c

cargo run --release
```

The server will listen on `127.0.0.1:8080`. If you want to listen on other endpoints, go to modify the `sign.config.toml` file.

Enjoy!

## Docker

Run in the current directory:

```sh
./build.sh [-n|--name IMAGE_NAME] [-t|--tag IMAGE_TAG]
```

Then run:

```sh
docker run -d --name SignServer -p 127.0.0.1:8080:8080 signserver:1.0.0
```

If you specify an image name or tag, replace `signerver` or `1.0.0`.

After running, an Anonymous Volume will be created. You can configure `sign.config.toml` or replace `wrapper.node` within it.

## Config

Modify the `sign.config.toml` file according to your needs.

If you use other versions of QQ, put `{version}.json` in the same directory. For specific content, refer to `src/appinfo/3.2.19-39038.json`.

