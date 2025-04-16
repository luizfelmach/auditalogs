FROM rust:latest AS builder

WORKDIR /build

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release
RUN cargo install --path .

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    apt-get clean

COPY config/dev.toml /etc/auditaserver/config.toml
COPY --from=builder /usr/local/cargo/bin/auditaserver /usr/local/bin/auditaserver

CMD [ "auditaserver", "--config", "/etc/auditaserver/config.toml" ]
