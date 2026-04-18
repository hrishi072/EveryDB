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

### Qt 6 & Development Tools (Fedora/openSUSE)
EveryDB requires Qt 6.2+ and several system libraries.

#### openSUSE Tumbleweed
```bash
sudo zypper install -y \
    gcc-c++ \
    cmake \
    qt6-base-devel \
    qt6-declarative-devel \
    qt6-qttools-devel \
    Mesa-libGL-devel \
    libxkbcommon-devel \
    libopenssl-devel \
    libsecret-devel \
    ImageMagick
```

#### Fedora
```bash
sudo dnf install -y \
    gcc-c++ \
    cmake \
    qt6-qtbase-devel \
    qt6-qtdeclarative-devel \
    qt6-qttools-devel \
    mesa-libGL-devel \
    libxkbcommon-devel \
    openssl-devel \
    libsecret-devel \
    ImageMagick
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

## Development

### Git Hooks
To ensure code quality, install the pre-push hook:
```bash
cp .githooks/pre-push .git/hooks/pre-push
chmod +x .git/hooks/pre-push
```

### Running Tests
```bash
cargo test --workspace
```

## Packaging

EveryDB is primarily packaged as an **RPM**.

### Generate RPM locally
Requires `cargo-generate-rpm`:
```bash
cargo install cargo-generate-rpm
cd crates/app && cargo generate-rpm
```

## CI/CD & Releases

Every push to the `main` branch automatically builds the RPM and updates the **Latest Release** on GitHub.

## License

MIT
