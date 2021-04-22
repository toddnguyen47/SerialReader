FROM rust:1.51-alpine3.13 as prerequisite
#RUN apt-get update && \
#  apt-get install -y libudev-dev pkg-config
RUN apk add --update \
  alpine-sdk

FROM prerequisite as builder
WORKDIR /home/serial-port-reader-writer
COPY . .
RUN cargo build

FROM builder as test_runner
WORKDIR /home/serial-port-reader-writer
CMD ["cargo", "test"]
