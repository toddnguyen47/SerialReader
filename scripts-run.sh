#!/bin/bash

set -euxo pipefail

docker run -it --rm "serial-port-reader-writer:latest"
