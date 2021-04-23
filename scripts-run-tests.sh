#!/bin/bash

set -euxo pipefail

version="latest"

docker build --target "test-runner" .
docker image prune --force
