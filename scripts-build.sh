#!/bin/bash

# -x = echo commands being ran
set -x

version="latest"

rm -r target/
docker build --tag "serial-port-reader-writer-builder:${version}" --target builder .
docker build --tag "serial-port-reader-writer:${version}" --target test-runner .
