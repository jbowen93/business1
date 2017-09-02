FROM jbowen/rust:nightly

RUN mkdir -p /rust/app
WORKDIR /rust/app

ENV ROCKET_ENV staging
COPY . /rust/app
CMD cargo build --release

CMD cargo run --release