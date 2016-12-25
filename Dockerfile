FROM jbowen/rust:nightly

RUN mkdir -p /rust/app
WORKDIR /rust/app

COPY . /rust/app
CMD cargo build --release

CMD cargo run --release