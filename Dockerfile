FROM rust:1.51 as prerequisite
RUN apt-get update && \
  apt-get install -y libudev-dev pkg-config

FROM prerequisite as builder
WORKDIR /usr/src/serial-port-reader-writer
COPY . .
RUN cargo build

FROM builder as test_runner
WORKDIR /usr/src/serial-port-reader-writer
CMD ["cargo", "test"]
