#!/bin/bash

set -euxo pipefail

version="latest"
docker run -it --rm "serial-port-reader-writer:${version}"
