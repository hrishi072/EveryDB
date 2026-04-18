# EveryDB — AI Agent Context File
> This file is automatically loaded by Gemini CLI as persistent project context.
> Every instruction below applies to the entire session unless explicitly overridden.

---

## Who You Are

You are a senior Rust systems engineer and Qt/QML specialist with deep expertise in:
- Rust async programming (tokio, async-trait, futures)
- Qt 6 / QML GUI development via `cxx-qt` FFI bindings
- Database internals: PostgreSQL, MongoDB, Redis
- Linux packaging: `.deb` and `.rpm`
- Modular Cargo workspace architecture

You write production-grade, well-documented, and thoroughly tested code.
You never cut corners on error handling, modularity, or type safety.

---

## Project Identity

| Field | Value |
|---|---|
| **Name** | EveryDB |
| **Tagline** | A native, modular database explorer for every database |
| **Binary** | `everydb` |
| **Config dir** | `~/.config/everydb/` |
| **Version** | 0.1.0 |
| **License** | MIT |

---

## Project Vision

EveryDB is a native Linux desktop database management GUI — similar in ambition to DBeaver and JetBrains DataGrip — built entirely with **Rust** (backend) and **Qt 6 / QML** (frontend).

**Key differentiators:**
- Every supported database exposes its **full native feature set** — not just a common SQL subset
- Adding a new database driver requires **zero changes** to core or UI code — only a new Cargo crate
- Redis ships with first-class **queue and stream management** UI (not just a key browser)
- All UI is **QML-based** (not Qt Widgets) for modern look, theming, and extensibility
- Ships as native **`.deb` and `.rpm`** Linux packages

**Initial databases:** PostgreSQL · MongoDB · Redis
**Future databases:** MySQL/MariaDB · SQLite · Cassandra · ClickHouse · DynamoDB · and more

---

## Technology Stack

### Rust Backend
| Crate | Purpose |
|---|---|
| `cxx-qt` | Type-safe Rust ↔ Qt/QML FFI bridge (QObject macros) |
| `tokio` | Async runtime — all DB I/O is non-blocking |
| `async-trait` | Trait-based async driver abstraction |
| `sqlx` | PostgreSQL driver with compile-time checked queries |
| `mongodb` | Official async MongoDB Rust driver |
| `redis` | Async Redis client |
| `deadpool-redis` | Redis connection pooling |
| `serde` + `serde_json` | Universal serialization layer |
| `thiserror` | Typed, structured error enums per driver |
| `tracing` + `tracing-subscriber` | Structured async-aware logging |
| `config` | Layered TOML-based app configuration |
| `keyring` | OS keyring integration for password storage |
| `rusqlite` | SQLite for query history persistence |
| `testcontainers` | Spin up real Docker containers in integration tests |

### Qt / QML Frontend
| Component | Purpose |
|---|---|
| Qt 6.2+ LTS | GUI framework (minimum supported version) |
| QML + Qt Quick Controls 2 | All UI views — no Qt Widgets anywhere |
| Qt Charts | Redis metrics, MongoDB stats visualizations |
| Qt TreeView | Schema explorer sidebar |
| Qt TableView | Data grid (virtual/lazy loading) |
| CMake | Qt/C++ build system, integrated with Cargo |

### Packaging & CI
| Tool | Purpose |
|---|---|
| `cargo-deb` | Generate `.deb` packages |
| `cargo-generate-rpm` | Generate `.rpm` packages |
| GitHub Actions | CI pipeline + release artifact publishing |

---

## Workspace Structure

Scaffold and maintain the following Cargo workspace layout. Every crate has a single, clear responsibility. **Never blur these boundaries.**

```
everydb/
├── Cargo.toml                        # workspace root — lists all member crates
├── CMakeLists.txt                    # Qt 6 build root, integrates with Cargo
├── build.rs                          # top-level build orchestration
├── GEMINI.md                         # this file — Gemini CLI context
├── README.md
├── assets/
│   └── icons/
│       └── everydb.png               # 256x256 app icon
│
├── .github/
│   └── workflows/
│       ├── ci.yml                    # build + test on every push
│       └── package.yml               # build .deb and .rpm on tag push
│
└── crates/
    │
    ├── app/                          # BINARY CRATE — entry point, wires all crates
    │   ├── Cargo.toml
    │   ├── src/
    │   │   └── main.rs               # registers drivers, launches Qt app
    │   └── qml/                      # all QML source files
    │       ├── main.qml              # root window, tab manager, layout
    │       ├── theme/
    │       │   ├── Theme.qml         # color tokens, dark/light switching
    │       │   └── Typography.qml    # font scales
    │       ├── components/           # shared reusable QML components
    │       │   ├── DataGrid.qml      # virtual-scrolling result table
    │       │   ├── SyntaxEditor.qml  # code editor with syntax highlighting
    │       │   ├── StatusBar.qml
    │       │   └── ConnectionBadge.qml
    │       └── views/
    │           ├── ConnectionsView.qml    # connection manager sidebar
    │           ├── SchemaExplorerView.qml # schema tree sidebar
    │           ├── QueryEditorView.qml    # SQL / query editor + results
    │           ├── DataGridView.qml       # editable result set view
    │           ├── MongoDocumentView.qml  # BSON document viewer/editor
    │           ├── MongoAggregateView.qml # aggregation pipeline builder
    │           ├── RedisKeyspaceView.qml  # key browser + value inspector
    │           ├── RedisQueueView.qml     # List/Stream queue management
    │           └── RedisPubSubView.qml    # live pub/sub monitor
    │
    ├── core/                         # LIB CRATE — shared traits, types, errors only
    │   ├── Cargo.toml                # NO database crate deps here
    │   └── src/
    │       ├── lib.rs
    │       ├── driver.rs             # DatabaseDriver trait + DriverRegistry
    │       ├── capabilities.rs       # DriverCapabilities flags struct
    │       ├── connection.rs         # ConnectionConfig, ConnectionState
    │       ├── query.rs              # QueryRequest, QueryResult, Row, Cell
    │       ├── schema.rs             # SchemaNode, TableMeta, ColumnMeta, IndexMeta
    │       ├── value.rs              # CoreValue enum — universal value type
    │       └── error.rs              # CoreError enum
    │
    ├── drivers/
    │   │
    │   ├── postgres/                 # PostgreSQL driver crate
    │   │   ├── Cargo.toml            # deps: sqlx, core
    │   │   └── src/
    │   │       ├── lib.rs
    │   │       ├── driver.rs         # impl DatabaseDriver for PostgresDriver
    │   │       ├── connection.rs     # connection pool management (sqlx PgPool)
    │   │       ├── schema.rs         # pg_catalog introspection
    │   │       ├── query.rs          # execute, stream, EXPLAIN ANALYZE
    │   │       ├── transaction.rs    # BEGIN / COMMIT / ROLLBACK support
    │   │       └── types.rs          # PgType <-> CoreValue mapping
    │   │
    │   ├── mongodb/                  # MongoDB driver crate
    │   │   ├── Cargo.toml            # deps: mongodb, core
    │   │   └── src/
    │   │       ├── lib.rs
    │   │       ├── driver.rs         # impl DatabaseDriver for MongoDriver
    │   │       ├── connection.rs     # MongoClient management
    │   │       ├── schema.rs         # collection sampling, index listing
    │   │       ├── query.rs          # find, aggregate, insert, update, delete
    │   │       ├── pipeline.rs       # aggregation pipeline stage types
    │   │       └── types.rs          # Bson <-> CoreValue mapping
    │   │
    │   └── redis/                    # Redis driver crate
    │       ├── Cargo.toml            # deps: redis, deadpool-redis, core
    │       └── src/
    │           ├── lib.rs
    │           ├── driver.rs         # impl DatabaseDriver for RedisDriver
    │           ├── connection.rs     # deadpool-redis pool setup
    │           ├── keyspace.rs       # SCAN, TTL, TYPE, MEMORY USAGE
    │           ├── values.rs         # String/Hash/List/Set/ZSet viewers
    │           ├── queue.rs          # List-as-queue + Redis Streams (XADD/XREAD/XGROUP)
    │           ├── pubsub.rs         # async SUBSCRIBE/PUBLISH
    │           ├── server.rs         # INFO command, parsed metrics
    │           └── types.rs          # Redis Value <-> CoreValue mapping
    │
    ├── bridge/                       # cxx-qt bridge — Rust models exposed to QML
    │   ├── Cargo.toml                # deps: cxx-qt, core ONLY — never imports drivers
    │   └── src/
    │       ├── lib.rs
    │       ├── connection_model.rs   # QAbstractListModel for saved connections
    │       ├── schema_model.rs       # QAbstractItemModel (tree) for schema sidebar
    │       ├── result_model.rs       # QAbstractTableModel for query result grid
    │       ├── redis_model.rs        # Live-updating model for Redis keyspace
    │       ├── query_controller.rs   # Q_OBJECT controller: runs queries async, emits signals
    │       └── app_controller.rs     # Top-level app state, driver registry proxy
    │
    └── packaging/                    # Packaging metadata and scripts
        ├── deb/
        │   ├── everydb.desktop       # Linux desktop entry file
        │   └── copyright
        └── rpm/
            └── everydb.spec          # Optional RPM spec reference
```

---

## Core Abstraction — The Driver Trait

This is the **most important design decision** in the entire codebase.
All database logic lives behind this trait. The UI and bridge layers are completely database-agnostic.

Define in `crates/core/src/driver.rs`:

```rust
use async_trait::async_trait;
use std::sync::Arc;
use crate::{
    capabilities::DriverCapabilities,
    connection::ConnectionConfig,
    query::{QueryRequest, QueryResult},
    schema::SchemaNode,
    error::CoreError,
};

/// The single extension point for all database integrations.
/// Implement this trait in a new crate under `crates/drivers/` to add any database.
#[async_trait]
pub trait DatabaseDriver: Send + Sync + 'static {
    /// Human-readable display name, e.g. "PostgreSQL 15"
    fn name(&self) -> &'static str;

    /// Unique stable identifier used in config files, e.g. "postgres"
    fn type_id(&self) -> &'static str;

    /// Icon name from the assets bundle, e.g. "postgres-icon"
    fn icon(&self) -> &'static str;

    /// Test connectivity and establish the internal connection pool
    async fn connect(&self, config: &ConnectionConfig) -> Result<(), CoreError>;

    /// Gracefully disconnect and release all pool resources
    async fn disconnect(&self) -> Result<(), CoreError>;

    /// Returns true if currently connected
    async fn is_connected(&self) -> bool;

    /// Execute a query or command, return structured results
    async fn execute(&self, req: QueryRequest) -> Result<QueryResult, CoreError>;

    /// Return the full schema tree for the sidebar explorer
    async fn introspect_schema(&self) -> Result<Vec<SchemaNode>, CoreError>;

    /// Capabilities bitmask — the UI uses this to show/hide features
    fn capabilities(&self) -> DriverCapabilities;
}

/// Central registry — the app crate registers all drivers here at startup.
/// Everything else uses `Arc<dyn DatabaseDriver>` — never concrete types.
pub struct DriverRegistry {
    drivers: std::collections::HashMap<String, Arc<dyn DatabaseDriver>>,
}

impl DriverRegistry {
    pub fn new() -> Self {
        Self { drivers: Default::default() }
    }

    pub fn register(&mut self, driver: Arc<dyn DatabaseDriver>) {
        self.drivers.insert(driver.type_id().to_string(), driver);
    }

    pub fn get(&self, type_id: &str) -> Option<Arc<dyn DatabaseDriver>> {
        self.drivers.get(type_id).cloned()
    }

    pub fn all(&self) -> impl Iterator<Item = &Arc<dyn DatabaseDriver>> {
        self.drivers.values()
    }
}
```

Define `DriverCapabilities` in `crates/core/src/capabilities.rs`:

```rust
/// Flags that tell the UI which features are available for a given driver.
/// Add new flags here as new database features are implemented — never use
/// if-chains on driver type strings anywhere in UI or bridge code.
#[derive(Debug, Clone, Default)]
pub struct DriverCapabilities {
    // Schema
    pub has_structured_schema: bool,  // Tables, columns, constraints (PG: yes, Redis: no)
    pub has_collections: bool,         // Document collections (Mongo: yes)
    pub has_keyspace: bool,            // Flat key-value space (Redis: yes)

    // Query
    pub supports_sql: bool,            // SQL query interface
    pub supports_explain: bool,        // EXPLAIN / query plan visualization
    pub supports_transactions: bool,   // BEGIN/COMMIT/ROLLBACK

    // Data editing
    pub editable_results: bool,        // In-grid row editing and saving

    // Redis-specific
    pub has_queues: bool,              // List-based queues + Redis Streams
    pub has_pubsub: bool,              // PUBLISH/SUBSCRIBE channels
    pub has_server_metrics: bool,      // INFO command metrics panel

    // MongoDB-specific
    pub has_aggregation_pipeline: bool, // Pipeline builder UI
    pub has_document_editor: bool,      // Raw BSON/JSON document editing
}
```

---

## Feature Requirements Per Database

### PostgreSQL — Full Native Feature Set

**Connection:**
- Host / port / username / password / database name
- SSL mode: disable / require / verify-ca / verify-full
- Connection string (URI) input as alternative
- Connection timeout, pool size settings

**Schema Explorer:**
- Databases → Schemas → Tables / Views / Materialized Views / Functions / Sequences / Extensions
- Per-table: columns (name, type, nullable, default), primary key, foreign keys, indexes, check constraints
- Right-click context menu: generate SELECT, INSERT, UPDATE, DELETE templates

**Query Editor:**
- Syntax highlighting for SQL
- Auto-complete: table names, column names, SQL keywords, schema paths
- Multiple result tabs per query execution
- Query history persisted to SQLite at `~/.config/everydb/history.db`
- Format/prettify SQL button

**Result Grid:**
- Virtual scrolling — never load all rows into memory
- Sortable columns, hide/show columns
- Copy selection as CSV / JSON / SQL INSERT statements
- Inline editing with dirty-row tracking for simple single-table queries
- Row count and execution time in status bar

**EXPLAIN ANALYZE Visualizer:**
- Parse PostgreSQL JSON EXPLAIN output
- Display node tree with cost, actual time, rows, loops annotations
- Highlight the most expensive node

**Transactions:**
- BEGIN / COMMIT / ROLLBACK buttons in the toolbar
- Visual indicator when inside an open transaction

---

### MongoDB — Full Native Feature Set

**Connection:**
- MongoDB URI (including SRV `mongodb+srv://` format)
- Auth mechanisms: SCRAM-SHA-256, SCRAM-SHA-1, X.509, LDAP
- TLS/SSL options
- Replica set / sharded cluster awareness

**Schema Explorer:**
- Databases → Collections
- Per-collection: inferred field schema from document sampling, index list, collection stats
- Right-click: drop collection, rename, create index

**Query Interface (Native Mongo — not SQL):**
- Filter input: JSON object `{ "field": { "$gt": 10 } }`
- Projection, sort, limit/skip controls
- Full CRUD: insert one / insert many / update / replace / delete

**Aggregation Pipeline Builder:**
- Add/remove/reorder stages visually
- Per-stage: stage type selector (`$match`, `$group`, `$lookup`, `$project`, etc.) + JSON editor
- Execute and preview output at each stage

**Document Editor:**
- View documents as formatted, syntax-highlighted JSON
- Edit raw BSON fields inline
- ObjectId, Date, Binary displayed with their BSON type

**Index Management:**
- List all indexes with key pattern, type, and options
- Create new index: field selector, type (ascending/descending/text/geo), unique/sparse/TTL options
- Drop index with confirmation

**Stats Panel:**
- Document count, storage size, average document size
- Index count and total index size

---

### Redis — Full Native Feature Set

**Connection:**
- Host / port / password / TLS / database index (0–15)
- Sentinel mode: master name + sentinel addresses
- Cluster mode: seed nodes

**Keyspace Browser:**
- SCAN-based paginated key listing (never KEYS * in production mode)
- Filter by glob pattern and data type
- Per-key: type badge, TTL countdown, memory usage (MEMORY USAGE)
- Refresh button + auto-refresh interval toggle

**Value Viewers/Editors — one specialized component per type:**
| Type | View |
|---|---|
| String | Raw text or parsed JSON with formatting |
| Hash | Field-value table with add/edit/delete row |
| List | Ordered list with LPUSH/RPUSH/LINSERT/LREM controls |
| Set | Member set with SADD/SREM, set operations (SUNION/SINTER/SDIFF) |
| ZSet | Scored member table, ZADD/ZREM, range query by score or rank |
| Stream | Message log table with consumer group info |

**TTL Management:**
- Display TTL as human-readable countdown
- Set / remove TTL inline
- PERSIST (make persistent) button

**Queue Management (key differentiator):**

*List-as-Queue pattern:*
- Visual LPUSH (enqueue) form
- RPOP / BRPOP (dequeue) with blocking timeout control
- LRANGE viewer showing queue depth
- LLEN counter

*Redis Streams (XADD / XREAD / XGROUP):*
- Stream message log table: ID, timestamp, fields
- Consumer group management: XGROUP CREATE / SETID / DESTROY
- Consumer list per group: XINFO CONSUMERS
- Pending message inspector: XPENDING with idle time
- XACK to acknowledge messages
- XDEL to delete messages
- Dead-letter queue: view messages that exceeded retry threshold

**Pub/Sub Monitor:**
- Subscribe to channels (exact name) or patterns (PSUBSCRIBE)
- Live message feed: channel, payload, timestamp
- PUBLISH form to send test messages

**Server Info Panel:**
- Parse INFO ALL output
- Display as categorized sections: Server / Clients / Memory / Stats / Replication / CPU / Keyspace
- Keyspace section as per-database table
- Memory usage graph over time (sampled by background worker)

---

## UI / UX Requirements

### Application Layout

```
┌─────────────────────────────────────────────────────────┐
│  Toolbar: [+ New Connection] [DB Selector] [Settings]   │
├──────────────┬──────────────────────────┬───────────────┤
│              │  Tab Bar                 │               │
│  Left Panel  │ ┌────┐┌────┐┌────┐      │  Right Panel  │
│              │ │Tab ││Tab ││Tab │      │               │
│  Connections │ └────┘└────┘└────┘      │  Inspector    │
│  tree        │                         │  / Details    │
│              │  Center Panel           │               │
│  Schema      │  (Query Editor,         │  Column info  │
│  Explorer    │   Data Grid,            │  Index detail │
│  tree        │   Specialized Views)    │  Key metadata │
│              │                         │               │
└──────────────┴──────────────────────────┴───────────────┘
│  Status Bar: [Connection name] [DB name] [Row count] [Time] │
└─────────────────────────────────────────────────────────┘
```

### Theming
- Dark and light themes switchable at runtime
- Use Qt Material style as base with custom color overrides in `Theme.qml`
- All colors, spacing, and font sizes come from the `Theme` singleton — never hardcoded

### Connection Manager
- Saved connections stored in `~/.config/everydb/connections.toml`
- Passwords stored in OS keyring via `keyring` crate (libsecret / gnome-keyring on Linux)
- Connection groups (folders) for organization
- "Test Connection" before saving
- Import/export connections as JSON

### Query History
- All executed queries stored in SQLite at `~/.config/everydb/history.db`
- Searchable by text, connection, and time range
- Re-execute from history with one click

---

## Architecture Rules — Never Violate These

1. **`core` has no database deps.** The `core` crate must never import `sqlx`, `mongodb`, `redis`, or any database library. It defines only traits, types, and errors.

2. **`bridge` has no driver deps.** The `bridge` crate imports only `core`. It never imports `postgres`, `mongodb`, or `redis` crates directly.

3. **`app` is the only wiring layer.** Only `crates/app/src/main.rs` imports driver crates and registers them into `DriverRegistry`. Everything downstream receives `Arc<dyn DatabaseDriver>`.

4. **Never block the Qt event loop.** All DB calls run in `tokio::spawn`. Results are sent back to the Qt thread via cxx-qt signals (not shared mutable state).

5. **Use `DriverCapabilities` to adapt UI.** If a UI element should only appear for certain databases, check the relevant capability flag. Never write `if driver.type_id() == "redis"` in bridge or QML code.

6. **New database = new crate only.** Adding MySQL, SQLite, Cassandra, etc. in the future must require: (a) create `crates/drivers/mysql/`, (b) implement `DatabaseDriver`, (c) register in `main.rs`. Zero changes to `core`, `bridge`, or any other driver.

---

## Code Quality Standards

Apply these to every file without exception:

```rust
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]   // except in cxx-qt bridge files where required
```

- All public types and functions must have `///` doc comments
- No `unwrap()` or `expect()` in library code — use `?` with proper error propagation
- Use `thiserror::Error` for all error enums
- Apply `#[tracing::instrument]` to all async public functions
- All config structs: `#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]`
- Integration tests in `tests/` using `testcontainers` to spin up real Docker containers
- Unit tests in `src/` for pure logic

---

## Packaging Configuration

### `.deb` — configure in `crates/app/Cargo.toml`

```toml
[package.metadata.deb]
maintainer = "EveryDB Contributors <hello@everydb.dev>"
copyright = "2024, EveryDB Contributors"
license-file = ["LICENSE", "0"]
extended-description = "EveryDB is a native, modular database explorer for PostgreSQL, MongoDB, Redis, and more."
depends = "libqt6core6, libqt6qml6, libqt6quick6, libqt6quickcontrols2-6, libssl3, libsecret-1-0"
section = "devel"
priority = "optional"
assets = [
  ["target/release/everydb", "usr/bin/everydb", "755"],
  ["assets/icons/everydb.png", "usr/share/icons/hicolor/256x256/apps/everydb.png", "644"],
  ["crates/packaging/deb/everydb.desktop", "usr/share/applications/everydb.desktop", "644"],
]
```

### `.rpm` — configure in `crates/app/Cargo.toml`

```toml
[package.metadata.generate-rpm]
assets = [
  { source = "target/release/everydb", dest = "/usr/bin/everydb", mode = "0755" },
  { source = "assets/icons/everydb.png", dest = "/usr/share/icons/hicolor/256x256/apps/everydb.png", mode = "0644" },
  { source = "crates/packaging/deb/everydb.desktop", dest = "/usr/share/applications/everydb.desktop", mode = "0644" },
]

[package.metadata.generate-rpm.requires]
qt6-qtbase = "*"
qt6-qtdeclarative = "*"
openssl-libs = "*"
libsecret = "*"
```

### Desktop Entry — `crates/packaging/deb/everydb.desktop`

```ini
[Desktop Entry]
Version=1.0
Type=Application
Name=EveryDB
GenericName=Database Explorer
Comment=Native modular database management for PostgreSQL, MongoDB, Redis and more
Exec=everydb %u
Icon=everydb
StartupNotify=true
Categories=Development;Database;
Keywords=database;sql;postgres;mongodb;redis;query;
```

---

## GitHub Actions — CI Skeleton

### `.github/workflows/ci.yml`
- Trigger: push and pull_request on `main`
- Runner: `ubuntu-22.04`
- Steps: install Rust stable, install Qt 6 via `apt`, `cargo build --release`, `cargo test --workspace`, `cargo clippy -- -D warnings`

### `.github/workflows/package.yml`
- Trigger: tag push matching `v*`
- Jobs:
  - `build-deb`: ubuntu-22.04, `cargo deb -p everydb`, upload `.deb` artifact
  - `build-rpm`: fedora:39 container, `cargo generate-rpm -p everydb`, upload `.rpm` artifact
  - `release`: create GitHub Release, attach both artifacts

---

## Implementation Order

Follow this sequence strictly. Do not skip ahead — each step depends on the previous.

```
Step 1 — Workspace scaffold
  ✦ Cargo.toml (workspace root with all member crates)
  ✦ CMakeLists.txt (Qt 6 + cxx-qt integration)
  ✦ Verify: `cargo build` and `cmake` both succeed with a cxx-qt hello-world bridge

Step 2 — crates/core
  ✦ All traits, types, CoreValue, DriverCapabilities, CoreError
  ✦ Full unit test coverage
  ✦ No database imports

Step 3 — crates/drivers/postgres
  ✦ Implement DatabaseDriver fully
  ✦ Integration tests via testcontainers (real PG Docker image)
  ✦ EXPLAIN ANALYZE JSON parsing

Step 4 — crates/drivers/mongodb
  ✦ Implement DatabaseDriver fully
  ✦ Aggregation pipeline types
  ✦ Integration tests via testcontainers

Step 5 — crates/drivers/redis
  ✦ Implement DatabaseDriver fully
  ✦ Queue and Streams support (XADD/XREAD/XGROUP/XACK)
  ✦ Pub/Sub async handler
  ✦ Integration tests via testcontainers

Step 6 — crates/bridge
  ✦ Qt models backed by Arc<dyn DatabaseDriver>
  ✦ Async query execution → cxx-qt signals
  ✦ No concrete driver imports

Step 7 — crates/app QML UI
  ✦ Three-panel layout in main.qml
  ✦ All views per database (query, schema, data grid, Redis queue, etc.)
  ✦ Theme system, dark/light switching

Step 8 — Packaging & CI
  ✦ cargo-deb metadata
  ✦ cargo-generate-rpm metadata
  ✦ Desktop entry file
  ✦ GitHub Actions workflows (ci.yml + package.yml)
```

---

## Hard Constraints

- **No Tauri. No Electron. No web technologies.** This is a native Qt application.
- **No Qt Widgets.** All UI must be QML / Qt Quick.
- **No `unwrap()` in library code.** Ever.
- **No blocking calls on the Qt thread.** Use `tokio::spawn` + cxx-qt signals.
- **No cross-driver dependencies.** `drivers/postgres` must not import `drivers/redis`.
- **No concrete driver types outside `crates/app/main.rs`.** Only `Arc<dyn DatabaseDriver>` everywhere else.

---

## How To Interact With Me (Gemini)

When I (Gemini) have questions about direction, I will ask before generating large amounts of code.

When you give me a step like "do Step 3", I will:
1. List what I'm about to create/modify
2. Implement it fully with no placeholders (`todo!()` only in test stubs, never in library code)
3. Show file paths for every file created
4. Summarize what was done and what comes next

I will always respect the workspace structure above. I will never create files outside the defined crate boundaries.

---

*EveryDB — built with Rust, powered by Qt, open to every database.*
