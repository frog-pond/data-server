FROM debian:9-slim AS base

RUN apt update && apt install -qy ca-certificates openssl zlib1g

FROM rustlang/rust:nightly-slim AS rust-build

RUN apt update && apt install -qy libssl-dev openssl pkg-config zlib1g-dev

WORKDIR /data-server

ADD Cargo.toml Cargo.lock /data-server/

ENV CARGO_HOME /root/.cargo

RUN PATH="$CARGO_HOME/bin:${PATH}" cargo fetch

ADD . /data-server/.

RUN PATH="$CARGO_HOME/bin:${PATH}" cargo install --path .

FROM base

COPY --from=rust-build /data-server/target/release/ /data-server/

CMD ["/data-server/fp-ds"]
