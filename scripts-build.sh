#!/bin/bash

rm -r target/
docker build -t "serial-port-reader-writer:latest" .

# clean up
docker image prune --force
