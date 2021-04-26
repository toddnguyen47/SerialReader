#!/bin/bash

# -x = echo commands being ran
set -x

version="latest"

# Ref: https://unix.stackexchange.com/a/314370
start_time="$(date -u +%s.%N)"

# Build docker
# docker build --tag "serial-port-reader-writer-deps:${version}" --target build-dependencies .
# docker build --tag "serial-port-reader-writer:${version}" --target test-runner .
docker build --tag "serial-port-reader-writer:${version}" .

# Get elapsed time
end_time="$(date -u +%s.%N)"
elapsed="$(bc <<< "$end_time-$start_time")"
echo "Total of $elapsed seconds elapsed for process"

