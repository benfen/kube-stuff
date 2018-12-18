#!/bin/sh

docker build . -t rust-thing:0.0.1 --no-cache
docker run --rm --name rust-thing -p 8080:8080 rust-thing