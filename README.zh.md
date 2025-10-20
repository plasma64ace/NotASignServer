# Linux QQ 签名服务器

本项目仅适用于 Linux 系统。

## 如何使用？

首先，前往官方网站下载QQ的deb安装包，并将其重命名为 `linuxqq.deb` 。

建议下载 Linux x64 3.2.19-39038 版本。若需使用其他版本，请参阅 [配置部分](#config) 。

然后：

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

若您已安装 Docker，可参阅 [Docker](#docker) 获取更多信息。

```sh
gcc -std=c99 -shared -fPIC -o libsymbols.so symbols.c

cargo run --release
```

服务器将监听 `127.0.0.1:8080` 。若需监听其他端点，请修改 `sign.config.toml` 文件。

祝使用愉快！

## Docker

在当前目录中运行：

```sh
./build.sh [-n|--name IMAGE_NAME] [-t|--tag IMAGE_TAG]
```

然后运行：

```sh
docker run -d --name SignServer -p 127.0.0.1:8080:8080 signserver:1.0.0
```

若已指定图像名称或标签，请替换 `signerver` 或 `1.0.0`。

运行后将创建一个匿名卷。您可配置 `sign.config.toml` 或替换其中的 `wrapper.node`。

## 配置

根据需要修改 `sign.config.toml` 文件。

若使用其他版本的QQ，请在同目录下放置 `{version}.json`。具体内容请参考 `src/appinfo/3.2.19-39038.json`。

