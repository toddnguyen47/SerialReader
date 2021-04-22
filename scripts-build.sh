#!/bin/bash

# -x = echo commands being ran
set -x

version="latest"

# docker build --tag "serial-port-reader-writer-deps:${version}" --target build-dependencies .
# docker build --tag "serial-port-reader-writer:${version}" --target test-runner .

docker build --tag "serial-port-reader-writer:${version}" .
