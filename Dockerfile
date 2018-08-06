FROM debian:9-slim AS base

WORKDIR /data-server
RUN apt update && apt install -qy ca-certificates openssl libssl-dev zlib1g-dev

FROM base as rust-build-deps

RUN apt update && apt install -qy autoconf automake bash cmake curl make pkg-config

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly

RUN echo "test -f \$HOME/.cargo/env && source \$HOME/.cargo/env" >> /etc/profile \
    && echo "test -f \$HOME/.cargo/env && source \$HOME/.cargo/env" >> /etc/bash.bashrc

CMD /bin/bash

FROM rust-build-deps AS rust-build

ADD Cargo.toml Cargo.lock /data-server/

RUN cargo fetch

ADD . /data-server/.

RUN cargo build

FROM base

COPY --from=rust-build /data-server /data-server
