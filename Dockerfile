FROM rust:1.61-alpine3.15 as builder

RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apk/repositories &&\
    apk add --no-cache ca-certificates libressl-dev openssl-dev musl-dev

WORKDIR /app
COPY ./Cargo.lock ./Cargo.toml ./
RUN echo $'[source.crates-io]\nreplace-with ="tuna"\n[source.tuna]\nregistry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"' > /usr/local/cargo/config && \
    echo "fn main() {}" > ./dummy.rs && \
    sed -i 's#src/main.rs#dummy.rs#' Cargo.toml

# Build the dummy program for cache
RUN cargo build --release
COPY . .
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml && \
    cargo build --release


FROM alpine:3.15
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apk/repositories &&\
    apk add --no-cache ca-certificates libressl openssl musl

WORKDIR /app
COPY --from=builder /app/target/release/ddathome /app/ddathome

ENV TZ=Asia/Shanghai
ENV DOCKER docker

ENTRYPOINT ["/app/ddathome"]