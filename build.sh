#!/bin/bash

set -e

if [[ ! -f "wrapper.node" ]]; then
  apt-get update && apt-get install binutils patchelf -y

  if [[ ! -f "linuxqq.deb" ]]; then
    wget -O linuxqq.deb https://dldir1.qq.com/qqfile/qq/QQNT/Linux/QQ_3.2.19_250904_amd64_01.deb
  fi

  ar -p linuxqq.deb data.tar.xz | tar -xJ ./opt/QQ/resources/app/wrapper.node -O > wrapper.node

  patchelf \
  --remove-needed libbugly.so \
  --remove-needed libX11.so.6 \
  --remove-needed libX11-xcb.so.1 \
  --remove-needed libXext.so.6 \
  --remove-needed libvips-cpp.so.42 \
  --add-needed libstdc++.so.6 \
  --add-needed libsymbols.so \
  wrapper.node
fi

# 默认配置
IMAGE_NAME="signserver"
IMAGE_TAG="1.0.0"

# 解析命令行参数
while [[ $# -gt 0 ]]; do
  case $1 in
    -n|--name)
      if [[ -z "$2" || "$2" == -* ]]; then
        echo "错误: --name 需要指定一个值"
        exit 1
      fi
      IMAGE_NAME="$2"
      shift 2
      ;;
    -t|--tag)
      if [[ -z "$2" || "$2" == -* ]]; then
        echo "错误: --tag 需要指定一个值"
        exit 1
      fi
      IMAGE_TAG="$2"
      shift 2
      ;;
    *)
      echo "未知参数: $1"
      echo "用法: $0 [-n|--name IMAGE_NAME] [-t|--tag IMAGE_TAG]"
      exit 1
      ;;
  esac
done

# 验证必要的文件存在
if [[ ! -f "Dockerfile" ]]; then
  echo "错误: 当前目录下没有找到 Dockerfile"
  exit 1
fi

# 构建Docker镜像
echo "正在构建镜像: $IMAGE_NAME:$IMAGE_TAG"
if ! docker build -t "$IMAGE_NAME:$IMAGE_TAG" .; then
  echo "错误: Docker 构建失败"
  exit 1
fi

echo "构建完成: $IMAGE_NAME:$IMAGE_TAG"
