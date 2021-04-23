FROM rust:1.51-alpine3.13 as prerequisite
#RUN apt-get update && \
#  apt-get install -y libudev-dev pkg-config
RUN apk add --no-cache --update \
  alpine-sdk

# Cache build dependencies
# Ref: https://stackoverflow.com/a/58474618
FROM prerequisite as build-dependencies
WORKDIR /home/serial-port-reader-writer
RUN mkdir src && echo "fn main() {}" > src/main.rs
COPY Cargo.toml .
RUN cargo build
RUN rm -rf src

FROM build-dependencies as builder
WORKDIR /home/serial-port-reader-writer
COPY "src" "src"
RUN cargo build

FROM builder as test-runner
WORKDIR /home/serial-port-reader-writer
RUN cargo test

CMD ["/bin/sh"]
