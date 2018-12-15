FROM debian:9-slim AS base

WORKDIR /data-server
RUN apt update && apt install -qy ca-certificates openssl libssl-dev zlib1g-dev

FROM rust-lang:nightly-slim AS rust-build-deps

ADD Cargo.toml Cargo.lock /data-server/

ENV CARGO_HOME /root/.cargo

RUN PATH="$CARGO_HOME/bin:${PATH}" cargo fetch

ADD . /data-server/.

RUN PATH="$CARGO_HOME/bin:${PATH}" cargo install --path .

FROM base

COPY --from=rust-build /data-server/target/release/ /data-server/

CMD ["/data-server/fp-ds"]
