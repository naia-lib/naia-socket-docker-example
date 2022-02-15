# Build
FROM rust:1.58.1 as build

ARG server_protocol

ENV PKG_CONFIG_ALLOW_CROSS=1
ENV server_protocol1 $server_protocol

WORKDIR /usr/src/shared
COPY shared/. .

WORKDIR /usr/src/server
COPY server/. .

RUN cargo install --features ${server_protocol1} --path .

# Runtime
FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/naia-socket-docker-example-server /usr/local/bin/server

CMD ["server"]