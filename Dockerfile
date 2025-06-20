FROM rust:latest AS builder

WORKDIR /audita

COPY server/Cargo.toml server/Cargo.lock ./

COPY server/static ./static
COPY server/src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && apt-get clean

COPY --from=builder /audita/target/release/audita /usr/bin/audita
COPY server/config/dev.toml /etc/audita/config.toml

EXPOSE 8080

RUN cat <<EOF > /entrypoint.sh
#!/bin/sh
exec audita --config /etc/audita/config.toml | tee -a /var/log/audita.log
EOF

ENTRYPOINT [ "/entrypoint.sh" ]
