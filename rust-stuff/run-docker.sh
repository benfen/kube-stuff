#!/bin/sh

docker build . -t rust-thing --rm
docker run --rm rust-thing
docker rmi rust-thing