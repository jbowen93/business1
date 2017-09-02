#!/bin/sh
docker run --rm -it -v $(pwd):/source jbowen/rust:nightly /bin/bash -c "cd source; when-changed -r src cargo build --release --target=x86-64-unknown-linux-musl"
