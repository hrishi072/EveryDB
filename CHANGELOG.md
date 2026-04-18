# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0-beta.1] - 2026-04-18
### Added
- **Core Architecture:**
  - Designed the `DatabaseDriver` trait abstraction for modular driver integration.
  - Implemented the `DriverRegistry` for dynamic loading of database modules.
  - Created standardized `CoreError`, `QueryRequest`, `QueryResult`, and `SchemaNode` structures.
- **Database Drivers:**
  - Scaffolded the `postgres` driver (connect, execute query, schema introspect).
  - Scaffolded the `mongodb` driver (connect, ping, execute query).
  - Scaffolded the `redis` driver (deadpool connection, PING, execute query).
- **Qt/QML Bridge:**
  - Added the `cxx-qt` integration layer to connect Rust async logic to Qt6 QML.
  - Scaffolded QObject models: `ConnectionModel`, `QueryController`, `SchemaModel`, `ResultModel`, and `RedisModel`.
- **Frontend (QML):**
  - Built a 3-panel ApplicationWindow layout (Connections, Editor/Grid, Inspector).
  - Configured a dynamic Material-style Theme with Dark/Light mode support.
  - Created basic QML components for StatusBar, DataGrid, ConnectionBadge, and SyntaxEditor.
  - Built view placeholders for Postgres, Mongo, and Redis features.
- **CI/CD & Packaging:**
  - Added `.deb` packaging configuration in `Cargo.toml`.
  - Added `.rpm` packaging configuration in `Cargo.toml`.
  - Provided `everydb.desktop` entry for native Linux integration.
  - Added GitHub Actions workflow (`ci.yml`) for automated testing and clippy checks.
  - Added GitHub Actions workflow (`package.yml`) to automatically generate releases and attach `.deb`/`.rpm` artifacts on `v*` tag pushes.
- **Project Structure:**
  - Fully implemented the modular workspace layout as described in `GEMINI.md`.
  - Configured `.gitignore`, `.gitattributes`, and `.editorconfig`.
