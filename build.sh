#!/bin/sh
docker run --rm -it -v $(pwd):/source jbowen/rust:nightly /bin/bash -c "cd source; cargo build --target=x86_64-unknown-linux-musl"