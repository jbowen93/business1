#!/bin/sh
docker run --rm -it -v $(pwd):/source build-site1 /bin/bash -c "cd source; cargo build --target=x86_64-unknown-linux-musl --release"