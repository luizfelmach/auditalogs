# Audita Documentation

## Overview

**Audita** is a Rust-based backend application designed for audit logging and search. It integrates tightly with Elasticsearch and provides a robust, asynchronous HTTP API via the Axum framework.

---

## Table of Contents

1. [Features](#features)
2. [Installation](#installation)
3. [Configuration](#configuration)
4. [Monitoring & Metrics](#monitoring--metrics)
5. [Development & Contributing](#development--contributing)

---

## Features

* Fast async HTTP API using **Axum**
* Search and log audit data via **Elasticsearch**
* Environment-based configuration using `.toml` files
* CLI interface using `clap`
* Embedded static assets via `rust-embed`
* Hashing & random utilities (`sha2`, `rand`)
* Prometheus metrics support
* Dockerized for container-based deployment

---

## Installation

Audita can be installed in three main ways:

### 1. Docker Installation (Recommended for Quick Start)

Run the container directly:

```bash
docker run -p 8080:8080 ghcr.io/luizfelmach/audita:latest
```

Mount a local config if needed:

```bash
docker run -p 8080:8080 -v config.toml:/etc/audita/config.toml ghcr.io/luizfelmach/audita:latest
```

### 2. Binary Installation (Prebuilt Release)

Download the latest binary from the [Releases Page](https://github.com/luizfelmach/audita/releases):

```bash
curl -L https://github.com/luizfelmach/audita/releases/latest/download/audita-x86_64-linux -o audita
chmod +x audita
./audita --config config.toml
```

### 3. Manual Installation (Build from Source)

Clone and build with Cargo:

```bash
git clone https://github.com/luizfelmach/audita.git
cd audita
cargo install --path .
audita --config config.toml
```

---

## Configuration

Configuration settings are environment-specific and can be defined using .toml files through the --config argument.

Example: `config/dev.toml`

```toml
host = "0.0.0.0"
port = 8080
name = "worker"
queue_size = 8192
batch_size = 100_000
ethereum_batch_size = 1
threads = 1

[ethereum]
url = "http://localhost:8545"
contract = "0x42699A7612A82f1d9C36148af9C77354759b210b"
private_key = "0x8f2a55949038a9610f50fb23b5883af3b4ecb3c3bb792cbcefbd1542c692be63"
disable = false

[elastic]
url = "http://localhost:9200"
username = "elastic"
password = "changeme"
disable = false
```

---

## Monitoring & Metrics

Exposes metrics for Prometheus scraping (likely at `/api/metrics` endpoint).

Uses:

* `prometheus` Rust crate
* `tracing` + `tracing-subscriber`

---

## Development & Contributing

### Lint and Format

```bash
cargo fmt
cargo clippy
```

PRs are welcome.
