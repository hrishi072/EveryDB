# EveryDB

A native, modular database explorer for every database, built with **Rust** and **Qt 6 / QML**.

EveryDB is a high-performance database management GUI designed for Linux. It uses a driver-based architecture where each database is integrated via a dedicated Rust crate.

## Supported Databases

- **PostgreSQL** (via `sqlx`)
- **MongoDB** (via official `mongodb` driver)
- **Redis** (via `redis-rs`)

## Prerequisites

To build EveryDB, you need the following dependencies installed on your system:

### Rust
Install Rust via [rustup](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Qt 6 & Development Tools
EveryDB requires Qt 6.2+ and several system libraries.

#### Ubuntu 22.04 / 24.04
```bash
sudo apt update
sudo apt install -y \
    build-essential \
    cmake \
    qt6-base-dev \
    qt6-declarative-dev \
    qt6-tools-dev \
    libgl1-mesa-dev \
    libxkbcommon-dev \
    libssl-dev \
    libsecret-1-dev
```

#### Fedora
```bash
sudo dnf install \
    gcc-c++ \
    cmake \
    qt6-qtbase-devel \
    qt6-qtdeclarative-devel \
    qt6-qttools-devel \
    mesa-libGL-devel \
    libxkbcommon-devel \
    openssl-devel \
    libsecret-devel
```

## Compilation

### Build the project
```bash
cargo build --release
```

The binary will be located at `target/release/everydb`.

### Running the application
```bash
cargo run --release
```

## Project Structure

- `crates/core`: Shared traits and types (database-agnostic).
- `crates/drivers/*`: Database-specific implementations (Postgres, Mongo, Redis).
- `crates/bridge`: `cxx-qt` bindings exposing Rust logic to QML.
- `crates/app`: The main entry point and QML UI resources.

## Development

### Running Tests
```bash
cargo test --workspace
```

### Linting
```bash
cargo clippy --workspace -- -D warnings
```

## Packaging

EveryDB can be packaged as a `.deb` or `.rpm`.

### Generate .deb (Ubuntu/Debian)
Requires `cargo-deb`:
```bash
cargo install cargo-deb
cargo deb -p everydb
```

### Generate .rpm (Fedora/RHEL)
Requires `cargo-generate-rpm`:
```bash
cargo install cargo-generate-rpm
cargo generate-rpm -p everydb
```

## License

MIT
