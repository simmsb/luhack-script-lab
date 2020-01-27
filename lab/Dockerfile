FROM clux/muslrust:latest AS build-rust

WORKDIR /opt/server/

COPY server/Cargo.toml .
COPY server/Cargo.lock .
RUN mkdir .cargo
RUN cargo vendor > .cargo/config

COPY server/src src

RUN cargo build --release
RUN cargo install --path . --verbose

FROM alpine:latest

ARG FLAG
ENV FLAG=$FLAG

COPY --from=build-rust /root/.cargo/bin/server /usr/local/bin/server

CMD ["server"]
