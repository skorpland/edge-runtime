FROM mcr.microsoft.com/devcontainers/rust:dev-1-bookworm

ARG TARGETPLATFORM

RUN apt-get update && apt-get install -y build-essential cmake libclang-dev lldb \
    nodejs npm hyperfine \
    libblas-dev liblapack-dev libopenblas-dev

COPY .env /tmp/.env
COPY .devcontainer/install.sh /tmp/install.sh
COPY scripts/install_onnx.sh /tmp/install_onnx.sh

WORKDIR /tmp

ENV DENO_INSTALL=/deno

RUN /tmp/install.sh $TARGETPLATFORM
