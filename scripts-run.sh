#!/bin/bash

set -euxo pipefail

version="latest"

docker run -it --rm "serial-port-reader-writer:${version}"

# Clean up
docker image prune --force
