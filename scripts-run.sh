#!/bin/bash

set -euxo pipefail

docker run -it "serial-port-reader-writer:latest"
