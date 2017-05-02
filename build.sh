#!/bin/sh

docker-compose run --rm app cargo build --release
docker-compose run -e CROSS_TRIPLE=x86_64-apple-darwin --rm app cargo build --release --target=x86_64-apple-darwin
