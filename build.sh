#!/bin/sh

set -e

docker-compose run --rm app cargo build --release
docker-compose run -e CROSS_TRIPLE=x86_64-apple-darwin --rm app cargo build --release --target=x86_64-apple-darwin

mv target/release/tiny-web-server tiny-web-server-x86_64-x86_64-unknown-linux-gnu
mv target/x86_64-apple-darwin/release/tiny-web-server tiny-web-server-x86_64-apple-darwin
