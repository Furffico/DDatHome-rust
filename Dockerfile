FROM rust:1.61-alpine3.15 as builder

RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apk/repositories &&\
    apk add --no-cache ca-certificates libressl-dev openssl-dev musl-dev &&\
    echo $'[source.crates-io]\nreplace-with ="tuna"\n[source.tuna]\nregistry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"' > /usr/local/cargo/config

WORKDIR /app

# Build the dummy program for cache
COPY ./dummy/ ./
RUN cargo build --release

# Copy and build the real program
COPY . .
RUN cargo build --release


FROM alpine:3.15
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apk/repositories &&\
    apk add --no-cache ca-certificates libressl openssl musl

WORKDIR /app
COPY --from=builder /app/target/release/ddathome-rust /app/ddathome

ENV DOCKER docker

ENTRYPOINT ["/app/ddathome"]