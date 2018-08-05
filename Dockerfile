FROM alpine:3.8 AS base

WORKDIR /data-server
RUN apk add -U ca-certificates openssl-dev

FROM base AS rust-build-deps

RUN apk add -U cargo rust
RUN apk add -U autoconf automake bash cmake make

CMD /bin/bash

FROM base

COPY --from=rust-build /data-server /data-server
