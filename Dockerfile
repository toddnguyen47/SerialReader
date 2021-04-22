FROM rust:1.51 as builder
RUN apt-get update && \
  apt-get install -y libudev-dev pkg-config

FROM builder as test_runner
WORKDIR /usr/src/serial-port-reader-writer
COPY . .
CMD ["cargo", "test"]
