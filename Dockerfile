FROM alpine:3.8 AS base

WORKDIR /data-server
RUN apk add -U ca-certificates openssl-dev

FROM base

COPY --from=rust-build /data-server /data-server
