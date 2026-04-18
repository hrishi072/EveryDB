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

#### Ubuntu / Debian
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
    libsecret-1-dev \
    imagemagick
```

## Running Locally

There are two ways to run EveryDB on your local machine:

### 1. Direct Execution (Development)
During development, you can run the application directly using Cargo. This will launch the GUI and load the QML files from your source tree.

**Important:** You must run this from the project root.

```bash
# Run in debug mode (faster compilation)
cargo run

# Run in release mode (faster execution)
cargo run --release
```

### 2. Local Installation (RPM/DEB)
If you want to install it as a system application:

#### For openSUSE / Fedora (RPM):
```bash
# Generate the RPM
cd crates/app && cargo generate-rpm
# Install locally
sudo zypper install ./target/generate-rpm/everydb-0.1.0-1.x86_64.rpm
```

#### For Ubuntu / Debian (DEB):
```bash
# Generate the DEB
cargo deb -p everydb
# Install locally
sudo dpkg -i ./target/debian/everydb_0.1.0-1_amd64.deb
```

## CI/CD & Releases

Every push to the `main` branch automatically:
1.  Builds the project.
2.  Generates both **.deb** and **.rpm** packages.
3.  Updates the **Latest Release** on the GitHub repository with these packages.

## License

MIT
