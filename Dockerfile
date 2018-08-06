FROM alpine:edge AS base

WORKDIR /data-server
RUN apk add -U ca-certificates openssl-dev

FROM base AS rust-from-source

RUN apk add -U autoconf automake bash cargo cmake curl g++ gcc git make python2 rust

RUN cd /tmp \
    && curl -o rustc-nightly-src.tar.gz -fSL https://static.rust-lang.org/dist/rustc-nightly-src.tar.gz \
    && tar xvzf rustc-nightly-src.tar.gz -C /

RUN cd /rustc-nightly-src \
    && ./x.py build --verbose \
    && ./x.py install --verbose

FROM rust-from-source AS rust-build-deps

# RUN apk add -U cargo rust
# RUN apk add -U autoconf automake bash cmake make

CMD /bin/bash

FROM rust-build-deps AS rust-build

ADD Cargo.toml Cargo.lock /data-server/

RUN cargo fetch

ADD . /data-server/.

RUN cargo build

FROM base

COPY --from=rust-build /data-server /data-server
