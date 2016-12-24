FROM ubuntu:16.04

ADD . /source

RUN apt-get update
RUN yes | apt-get install curl
RUN yes | apt-get install gcc

RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain nightly -y

ENV PATH=/root/.cargo/bin:$PATH

RUN rustup target add x86_64-unknown-linux-musl