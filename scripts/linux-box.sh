#!/bin/sh
docker run --name build-box -p 3000:3000 -d -it --rm -v $(pwd):/source jbowen/rust:nightly

#startup commands
docker exec -d -e ROCKET_ENV=stage build-box sh -c "cd source; cargo watch --no-gitignore -w .watch-trigger -x 'build --release --target=x86_64-unknown-linux-musl' -x 'run --release --target=x86_64-unknown-linux-musl'"
docker exec -it build-box sh -c "cd source; cargo watch -x check -s 'touch .watch-trigger'"


docker kill build-box