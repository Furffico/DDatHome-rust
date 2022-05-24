FROM rust:1.61 as builder

RUN apt update && apt install -y musl-dev libssl-dev
# RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apk/repositories &&\
#     apk add --no-cache musl-dev openssl-dev

WORKDIR /app
COPY ./Cargo.lock ./Cargo.toml ./
RUN echo '[source.crates-io]\nreplace-with ="tuna"\n[source.tuna]\nregistry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"' > /usr/local/cargo/config && \
    echo "fn main() {}" > ./dummy.rs && \
    sed -i 's#src/main.rs#dummy.rs#' Cargo.toml

# Build the dummy program for cache
RUN cargo build --release
COPY . .
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml && \
    cargo build --release

FROM debian:stable-slim
RUN apt update && \
    apt install -y libssl-dev ca-certificates && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*

# FROM alpine:3.15
# RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apk/repositories &&\
#     apk add --no-cache ca-certificates libressl

WORKDIR /app
COPY --from=builder /app/target/release/ddathome /app/ddathome

ENV TZ=Asia/Shanghai
ENV PLATFORM docker

ENTRYPOINT ["/app/ddathome"]