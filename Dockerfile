FROM alpine:3.8 AS base

WORKDIR /data-server
RUN apk add -U ca-certificates openssl-dev

FROM base AS rust-build-deps

RUN apk add -U cargo rust
RUN apk add -U autoconf automake bash cmake make

CMD /bin/bash

FROM rust-build-deps AS rust-build

ADD Cargo.toml Cargo.lock /data-server/

RUN cargo fetch

ADD . /data-server/.

RUN cargo build

FROM base

COPY --from=rust-build /data-server /data-server
