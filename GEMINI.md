# EveryDB — Universal Database IDE
# GEMINI.md — Complete Production Specification
# 
# THIS IS A FULL DATAGRIP/DBEAVER CLONE FOR **EVERY** DATABASE
# BUILT WITH RUST + TAURI — NATIVE PERFORMANCE, WEB TECHNOLOGIES UI
# EVERY FEATURE MUST BE FULLY IMPLEMENTED — NO STUBS, NO TODOs, NO PLACEHOLDERS
# THE STITCH UI IS IMMUTABLE LAW — MATCH IT PIXEL-PERFECT

---

## ⚠️ CRITICAL: WHAT "FULLY IMPLEMENTED" MEANS

This is NOT a prototype. This is NOT a proof-of-concept. This is a **production-grade database IDE**.

### ABSOLUTE REQUIREMENTS:

1. **EVERY BUTTON WORKS**
   - Click "Run" → query executes against real database
   - Click "Export" → file is written to disk with correct format
   - Click "Delete Row" → row is deleted from database after confirmation
   - Click "Connect" → actual TCP connection established, SSL handshake performed

2. **EVERY DIALOG SAVES DATA**
   - Connection dialog → saves to `~/.config/everydb/connections.toml` with encrypted password
   - Settings dialog → saves to `~/.config/everydb/preferences.toml`
   - Filter builder → translates FilterExpr to real SQL/BSON/Redis commands

3. **EVERY VIEW DISPLAYS REAL DATA**
   - Schema tree → shows actual tables/columns from live database
   - Data grid → displays real rows fetched via SELECT/find()
   - Query editor → syntax highlights based on actual SQL dialect
   - Redis keyspace → shows actual keys from SCAN command

4. **NO STUBS ANYWHERE**
   ```rust
   // ❌ FORBIDDEN:
   #[tauri::command]
   async fn execute_query(sql: String) -> Result<QueryResult, String> {
       todo!("Implement query execution")
   }
   
   // ✅ REQUIRED:
   #[tauri::command]
   async fn execute_query(
       state: tauri::State<'_, AppState>,
       connection_id: String,
       sql: String,
   ) -> Result<QueryResult, String> {
       let driver = state.get_driver(&connection_id)?;
       let result = driver.execute(QueryRequest::new(sql)).await
           .map_err(|e| e.to_string())?;
       Ok(result)
   }
   ```

5. **INTEGRATION TESTS WITH REAL DATABASES**
   - Every driver must have `testcontainers` tests
   - Tests spin up actual Docker containers (postgres:15, mysql:8, mongo:7, redis:7)
   - Tests execute real queries and verify real results
   - `cargo test -p postgres` must pass with 100% of tests succeeding

**IF YOU CANNOT IMPLEMENT SOMETHING FULLY, STOP AND ASK BEFORE WRITING ANY CODE.**

---

## 🏗️ TECHNOLOGY STACK

### Backend (Rust)
- **Tauri** 1.5+ — Native window + system integration
- **tokio** — Async runtime for database I/O
- **sqlx** — PostgreSQL, MySQL, SQLite drivers
- **mongodb** — MongoDB driver
- **redis** — Redis driver
- **serde** + **serde_json** — Serialization
- **thiserror** — Error handling
- **tracing** — Structured logging
- **keyring** — OS keyring for password storage
- **ssh2** — SSH tunnel support
- **testcontainers** — Integration testing

### Frontend (Web Technologies)
- **React** 18+ with **TypeScript** — UI framework (type safety + component model)
- **TanStack Query** (React Query) — Data fetching + caching
- **Zustand** — Global state management
- **Tailwind CSS** — Styling (Stitch export already uses Tailwind!)
- **Monaco Editor** — SQL syntax highlighting + autocomplete
- **AG Grid** or **TanStack Table** — Virtual scrolling data grid
- **Recharts** — Charts for EXPLAIN visualizer + dashboards
- **Lucide React** — Icon library (Material Symbols alternative)

**Alternative Frontend Options:**
- **Svelte** + **SvelteKit** (smaller bundle, simpler syntax)
- **Vue 3** + **Composition API** (if preferred)
- **Vanilla TypeScript** (no framework, direct DOM manipulation)

### Build & Package
- **Tauri CLI** — Build + bundle
- **Cargo** — Rust build system
- **Vite** — Frontend dev server + build (fast HMR)
- **GitHub Actions** — CI/CD

### Cross-Platform Targets
- **Linux** — .deb, .rpm, AppImage
- **Windows** — .msi, .exe
- **macOS** — .app, .dmg

---

## 🎨 THE STITCH UI IS IMMUTABLE LAW

The folder `stitch_everydb_management_suite/` contains the **frozen visual specification**.

**CRITICAL ADVANTAGE:** The Stitch export is **already HTML/CSS**! This is PERFECT for Tauri.

### ABSOLUTE RULES:

1. **EVERY COLOR** comes from the Stitch HTML exports
   - The Stitch HTML already uses Tailwind classes like `bg-surface-container`, `text-on-surface`
   - Extract these to a `tailwind.config.js` theme with exact hex values
   - ❌ FORBIDDEN: Hardcoded colors in components
   - ✅ REQUIRED: Use Tailwind theme colors

2. **EVERY DIMENSION** matches the Stitch HTML exactly
   - Row height: `h-[28px]` (from `.data-row` class in `visual_data_table/code.html`)
   - Sidebar width: `w-[280px]` (from `main_application_shell_gui_first/code.html`)
   - Border radius: `rounded` (4px) for buttons, `rounded-lg` (8px) for modals

3. **EVERY FONT** is specified in Tailwind config
   - `font-ui`: Inter for all UI labels, buttons, headers
   - `font-data`: JetBrains Mono for ALL database values (IDs, SQL, JSON, timestamps)
   - ❌ FORBIDDEN: Mixing fonts in one row
   - ✅ REQUIRED: Database values use `font-data`, UI uses `font-ui`

4. **EVERY COMPONENT STATE** matches Stitch
   - Button hover: `hover:bg-[#1a5abf]` (exact hex from Stitch)
   - Selected row: `bg-[rgba(47,129,247,0.15)] border-l-2 border-[#2F81F7]`
   - Modified cell: `bg-[rgba(212,153,34,0.05)] border-l-2 border-[rgba(212,153,34,0.8)]`

5. **USE THE STITCH HTML DIRECTLY**
   - The Stitch HTML files can be used as templates
   - Copy HTML structure, adapt Tailwind classes to your config
   - Replace hardcoded data with React state/props
   - Add event handlers (onClick, onChange, etc.)

### EXAMPLE: Converting Stitch HTML to React Component

**Stitch HTML (`visual_data_table/code.html`):**
```html
<div class="data-row h-[28px] hover:bg-surface-container-high">
  <div class="cell font-mono text-on-surface">42</div>
  <div class="cell font-mono text-on-surface">John Doe</div>
</div>
```

**React Component:**
```tsx
interface DataRowProps {
  row: { id: number; name: string };
  selected: boolean;
  onSelect: () => void;
}

const DataRow: React.FC<DataRowProps> = ({ row, selected, onSelect }) => (
  <div 
    className={cn(
      "h-[28px] flex items-center hover:bg-elevated",
      selected && "bg-[rgba(47,129,247,0.15)] border-l-2 border-accent-blue"
    )}
    onClick={onSelect}
  >
    <div className="px-4 font-data text-primary">{row.id}</div>
    <div className="px-4 font-data text-primary">{row.name}</div>
  </div>
);
```

---

## 📁 WORKSPACE STRUCTURE (Tauri)

```
everydb/
├── Cargo.toml                          # Workspace root
├── GEMINI.md                           # This file
├── README.md
├── LICENSE
├── .github/workflows/
│   ├── ci.yml                          # Build + test
│   └── release.yml                     # Build + package on tag
│
├── stitch_everydb_management_suite/    # READ-ONLY UI reference
│   ├── everydb_core_1/DESIGN.md
│   ├── everydb_core_2/DESIGN.md
│   ├── main_application_shell_gui_first/code.html + screen.png
│   ├── visual_data_table/code.html + screen.png
│   ├── visual_filter_builder/code.html + screen.png
│   └── ... (all other Stitch screens)
│
├── src-tauri/                          # Rust backend (Tauri app)
│   ├── Cargo.toml                      # Tauri dependencies
│   ├── tauri.conf.json                 # Tauri config (window, bundle, etc.)
│   ├── build.rs
│   ├── icons/                          # App icons (PNG, ICO, ICNS)
│   └── src/
│       ├── main.rs                     # Tauri app entry point
│       ├── commands/                   # Tauri command handlers
│       │   ├── mod.rs
│       │   ├── connection.rs           # connect, disconnect, test_connection
│       │   ├── schema.rs               # introspect_schema, get_table_metadata
│       │   ├── query.rs                # execute_query, cancel_query, get_history
│       │   ├── data.rs                 # list_table_data, insert_row, update_row, delete_row
│       │   ├── export.rs               # export_to_csv, export_to_json, export_to_xlsx
│       │   ├── import.rs               # import_from_csv, import_from_excel
│       │   └── settings.rs             # get_settings, save_settings
│       ├── state.rs                    # AppState (driver registry, connections)
│       └── lib.rs                      # Tauri setup, register commands
│
├── crates/
│   ├── core/                           # Shared types, traits, errors
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── driver.rs               # DatabaseDriver trait
│   │       ├── capabilities.rs         # DriverCapabilities flags
│   │       ├── connection.rs           # ConnectionConfig, PoolConfig
│   │       ├── query.rs                # QueryRequest, QueryResult
│   │       ├── schema.rs               # SchemaNode, TableMetadata
│   │       ├── value.rs                # CoreValue enum
│   │       ├── filter.rs               # FilterExpr, SortSpec
│   │       ├── error.rs                # CoreError
│   │       └── user.rs                 # User, Role, Privilege
│   │
│   └── drivers/
│       ├── postgres/                   # PostgreSQL driver
│       ├── mysql/                      # MySQL driver
│       ├── sqlite/                     # SQLite driver
│       ├── mongodb/                    # MongoDB driver
│       └── redis/                      # Redis driver
│
└── src/                                # Frontend (React + TypeScript)
    ├── main.tsx                        # React entry point
    ├── App.tsx                         # Root component
    ├── vite-env.d.ts
    ├── index.css                       # Tailwind imports
    ├── lib/
    │   ├── tauri.ts                    # Tauri command wrappers
    │   ├── types.ts                    # TypeScript types (match Rust types)
    │   └── utils.ts                    # Utility functions (cn, formatters)
    ├── hooks/
    │   ├── useConnections.ts           # TanStack Query for connections
    │   ├── useSchema.ts                # TanStack Query for schema
    │   ├── useTableData.ts             # TanStack Query for table data
    │   └── useQuery.ts                 # TanStack Query for SQL queries
    ├── store/
    │   ├── app.ts                      # Zustand store (active connection, tabs, etc.)
    │   └── settings.ts                 # Zustand store (preferences)
    ├── components/
    │   ├── ui/                         # Primitive components
    │   │   ├── Badge.tsx
    │   │   ├── Button.tsx
    │   │   ├── ContextMenu.tsx
    │   │   ├── TabBar.tsx
    │   │   ├── StatusBar.tsx
    │   │   └── ... (all primitives)
    │   ├── layout/
    │   │   ├── AppShell.tsx            # Three-panel layout
    │   │   ├── Sidebar.tsx             # Left sidebar
    │   │   ├── InspectorPanel.tsx      # Right panel
    │   │   └── Toolbar.tsx             # Top toolbar
    │   ├── grid/
    │   │   ├── DataTable.tsx           # Virtual scrolling grid
    │   │   ├── FilterBuilder.tsx       # Visual filter panel
    │   │   └── CellRenderer.tsx        # Type-specific cell rendering
    │   ├── editor/
    │   │   ├── SqlEditor.tsx           # Monaco editor wrapper
    │   │   ├── QueryResults.tsx        # Results tabs
    │   │   └── ExplainView.tsx         # EXPLAIN plan tree
    │   ├── schema/
    │   │   ├── SchemaTree.tsx          # Lazy-loading tree
    │   │   └── TableInspector.tsx      # Column/index/constraint tabs
    │   ├── mongo/
    │   │   ├── CollectionView.tsx      # Cards/table/JSON view
    │   │   ├── DocumentEditor.tsx      # Tree editor
    │   │   └── PipelineBuilder.tsx     # Aggregation pipeline
    │   ├── redis/
    │   │   ├── KeyspaceView.tsx        # Key list + value viewer
    │   │   ├── ValueEditor.tsx         # Type-specific editors
    │   │   ├── QueueView.tsx           # Queue manager
    │   │   └── PubSubView.tsx          # Pub/Sub monitor
    │   └── dialogs/
    │       ├── ConnectionDialog.tsx    # New/edit connection
    │       ├── SettingsDialog.tsx      # Settings panel
    │       ├── ExportDialog.tsx        # Export data
    │       └── ImportDialog.tsx        # Import data
    ├── views/
    │   ├── TableDataView.tsx           # Data grid + filters
    │   ├── QueryEditorView.tsx         # SQL editor + results
    │   ├── MongoCollectionView.tsx     # MongoDB collection
    │   ├── RedisKeyspaceView.tsx       # Redis keyspace
    │   └── UserManagementView.tsx      # User/role management
    ├── tailwind.config.js              # Tailwind theme (Stitch colors)
    ├── vite.config.ts                  # Vite config
    ├── package.json
    └── tsconfig.json
```

---

## 🔌 TAURI ARCHITECTURE

### Rust Backend (src-tauri/)

**Tauri Commands** replace cxx-qt signals. These are async Rust functions exposed to the frontend.

```rust
// src-tauri/src/commands/connection.rs

use tauri::State;
use crate::state::AppState;
use everydb_core::{ConnectionConfig, CoreError};

#[tauri::command]
pub async fn connect_database(
    state: State<'_, AppState>,
    id: String,
    config: ConnectionConfig,
) -> Result<(), String> {
    let driver = state.get_driver(&config.type_id)?;
    driver.connect(&config, &Default::default()).await
        .map_err(|e| e.to_string())?;
    
    state.add_connection(id.clone(), driver.clone());
    Ok(())
}

#[tauri::command]
pub async fn disconnect_database(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let driver = state.get_driver(&id)?;
    driver.disconnect().await.map_err(|e| e.to_string())?;
    state.remove_connection(&id);
    Ok(())
}

#[tauri::command]
pub async fn test_connection(
    state: State<'_, AppState>,
    config: ConnectionConfig,
) -> Result<String, String> {
    let driver = state.registry.get(&config.type_id)
        .ok_or("Driver not found")?;
    
    driver.connect(&config, &Default::default()).await
        .map_err(|e| e.to_string())?;
    
    let version = driver.server_version().await
        .map_err(|e| e.to_string())?;
    
    driver.disconnect().await.map_err(|e| e.to_string())?;
    Ok(version)
}
```

**AppState** holds the driver registry and active connections:

```rust
// src-tauri/src/state.rs

use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use everydb_core::{DriverRegistry, DatabaseDriver};

pub struct AppState {
    pub registry: DriverRegistry,
    pub connections: Arc<RwLock<HashMap<String, Arc<dyn DatabaseDriver>>>>,
}

impl AppState {
    pub fn new(registry: DriverRegistry) -> Self {
        Self {
            registry,
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn add_connection(&self, id: String, driver: Arc<dyn DatabaseDriver>) {
        self.connections.write().await.insert(id, driver);
    }
    
    pub async fn get_driver(&self, id: &str) -> Result<Arc<dyn DatabaseDriver>, String> {
        self.connections.read().await.get(id).cloned()
            .ok_or_else(|| format!("Connection {} not found", id))
    }
}
```

**Main entry point:**

```rust
// src-tauri/src/main.rs

use tauri::Manager;
use everydb_core::DriverRegistry;
use everydb_driver_postgres::PostgresDriver;
use everydb_driver_mysql::MySQLDriver;
use everydb_driver_mongodb::MongoDBDriver;
use everydb_driver_redis::RedisDriver;
use std::sync::Arc;

mod state;
mod commands;

fn main() {
    // Build driver registry
    let mut registry = DriverRegistry::new();
    registry.register(Arc::new(PostgresDriver::new()));
    registry.register(Arc::new(MySQLDriver::new()));
    registry.register(Arc::new(MongoDBDriver::new()));
    registry.register(Arc::new(RedisDriver::new()));
    
    let state = state::AppState::new(registry);
    
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::connection::connect_database,
            commands::connection::disconnect_database,
            commands::connection::test_connection,
            commands::schema::introspect_schema,
            commands::query::execute_query,
            commands::data::list_table_data,
            commands::data::insert_row,
            commands::data::update_row,
            commands::data::delete_row,
            // ... all other commands
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Frontend (React + TypeScript)

**Tauri Command Wrappers:**

```typescript
// src/lib/tauri.ts

import { invoke } from '@tauri-apps/api/tauri';
import type { ConnectionConfig, QueryResult, SchemaNode } from './types';

export const tauriCommands = {
  connection: {
    connect: (id: string, config: ConnectionConfig) =>
      invoke<void>('connect_database', { id, config }),
    
    disconnect: (id: string) =>
      invoke<void>('disconnect_database', { id }),
    
    test: (config: ConnectionConfig) =>
      invoke<string>('test_connection', { config }),
  },
  
  schema: {
    introspect: (connectionId: string) =>
      invoke<SchemaNode[]>('introspect_schema', { connectionId }),
    
    getTableMetadata: (connectionId: string, schema: string, table: string) =>
      invoke<TableMetadata>('get_table_metadata', { connectionId, schema, table }),
  },
  
  query: {
    execute: (connectionId: string, sql: string) =>
      invoke<QueryResult>('execute_query', { connectionId, sql }),
    
    cancel: (connectionId: string, queryId: string) =>
      invoke<void>('cancel_query', { connectionId, queryId }),
  },
  
  data: {
    list: (connectionId: string, schema: string, table: string, filters: FilterExpr[], limit: number, offset: number) =>
      invoke<QueryResult>('list_table_data', { connectionId, schema, table, filters, limit, offset }),
    
    insert: (connectionId: string, schema: string, table: string, values: Record<string, CoreValue>) =>
      invoke<CoreValue>('insert_row', { connectionId, schema, table, values }),
    
    update: (connectionId: string, schema: string, table: string, pkColumn: string, pkValue: CoreValue, changes: Record<string, CoreValue>) =>
      invoke<number>('update_row', { connectionId, schema, table, pkColumn, pkValue, changes }),
    
    delete: (connectionId: string, schema: string, table: string, pkColumn: string, pkValue: CoreValue) =>
      invoke<number>('delete_row', { connectionId, schema, table, pkColumn, pkValue }),
  },
};
```

**React Hooks with TanStack Query:**

```typescript
// src/hooks/useTableData.ts

import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { tauriCommands } from '@/lib/tauri';
import type { FilterExpr, CoreValue } from '@/lib/types';

export function useTableData(
  connectionId: string,
  schema: string,
  table: string,
  filters: FilterExpr[] = [],
  limit: number = 500,
  offset: number = 0,
) {
  return useQuery({
    queryKey: ['tableData', connectionId, schema, table, filters, limit, offset],
    queryFn: () => tauriCommands.data.list(connectionId, schema, table, filters, limit, offset),
    enabled: !!connectionId && !!schema && !!table,
  });
}

export function useInsertRow(connectionId: string, schema: string, table: string) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (values: Record<string, CoreValue>) =>
      tauriCommands.data.insert(connectionId, schema, table, values),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tableData', connectionId, schema, table] });
    },
  });
}
```

**React Component Example:**

```tsx
// src/components/grid/DataTable.tsx

import React from 'react';
import { useTableData } from '@/hooks/useTableData';
import { useAppStore } from '@/store/app';
import { CellRenderer } from './CellRenderer';

export const DataTable: React.FC = () => {
  const activeConnection = useAppStore((s) => s.activeConnection);
  const activeTable = useAppStore((s) => s.activeTable);
  
  const { data, isLoading, error } = useTableData(
    activeConnection?.id ?? '',
    activeTable?.schema ?? '',
    activeTable?.name ?? '',
  );
  
  if (isLoading) return <div className="flex items-center justify-center h-full">Loading...</div>;
  if (error) return <div className="text-red-500">Error: {error.message}</div>;
  if (!data) return null;
  
  return (
    <div className="flex flex-col h-full">
      {/* Header */}
      <div className="h-[32px] flex bg-panel-bg border-b border-ghost">
        {data.columns.map((col) => (
          <div key={col.name} className="px-4 flex items-center font-ui text-sm text-primary">
            {col.name} <span className="ml-2 font-data text-xs text-muted">{col.type}</span>
          </div>
        ))}
      </div>
      
      {/* Rows */}
      <div className="flex-1 overflow-auto">
        {data.rows.map((row, idx) => (
          <div
            key={idx}
            className="h-[28px] flex items-center hover:bg-elevated border-b border-subtle"
          >
            {data.columns.map((col, colIdx) => (
              <CellRenderer
                key={colIdx}
                value={row[colIdx]}
                type={col.type}
              />
            ))}
          </div>
        ))}
      </div>
    </div>
  );
};
```

---

## 🎨 TAILWIND THEME (Exact Stitch Colors)

**tailwind.config.js:**

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{js,jsx,ts,tsx}'],
  theme: {
    extend: {
      colors: {
        // Surface Layers (from Stitch)
        void: '#0D1117',
        'panel-bg': '#161B22',
        'surface-bg': '#1C2128',
        elevated: '#21262D',
        float: '#31353C',
        deep: '#0A0E14',
        
        // Text
        primary: '#E6EDF3',
        secondary: '#8B949E',
        muted: '#484F58',
        accent: '#ACC7FF',
        
        // Accent Colors (semantic)
        'accent-blue': '#2F81F7',
        'accent-green': '#3FB950',
        'accent-yellow': '#D29922',
        'accent-red': '#F85149',
        'accent-purple': '#A371F7',
        'accent-orange': '#F0883E',
        
        // Database Brands
        'pg-blue': '#336791',
        'mysql-blue': '#00758F',
        'sqlite-blue': '#003B57',
        'mongo-green': '#47A248',
        'redis-red': '#DC382D',
        'oracle-red': '#F80000',
        'mssql-red': '#CC2927',
        
        // Borders
        ghost: 'rgba(48, 54, 61, 0.40)',
        subtle: 'rgba(48, 54, 61, 0.20)',
        solid: '#30363D',
      },
      
      fontFamily: {
        ui: ['Inter', 'sans-serif'],
        data: ['JetBrains Mono', 'monospace'],
      },
      
      fontSize: {
        'ui': '13px',
        'ui-bold': '13px',
        'caps': '11px',
        'data': '12.5px',
        'data-sm': '12px',
      },
      
      spacing: {
        xs: '4px',
        sm: '8px',
        md: '16px',
        lg: '24px',
      },
      
      borderRadius: {
        xs: '2px',
        sm: '4px',
        md: '8px',
        // NEVER larger than 8px
      },
      
      transitionDuration: {
        fast: '100ms',
        normal: '150ms',
      },
    },
  },
  plugins: [],
};
```

**index.css:**

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

/* STRICT RULE: All DB values use font-data, all UI uses font-ui */
.db-value {
  @apply font-data text-primary;
}

/* Row states from Stitch */
.row-normal {
  @apply bg-transparent;
}

.row-hover {
  @apply bg-elevated;
}

.row-selected {
  @apply bg-[rgba(47,129,247,0.15)] border-l-2 border-accent-blue;
}

.row-modified {
  @apply bg-[rgba(212,153,34,0.05)] border-l-2 border-[rgba(212,153,34,0.8)];
}

.row-new {
  @apply bg-[rgba(63,185,80,0.05)] border-l-2 border-[rgba(63,185,80,0.8)];
}

.row-error {
  @apply bg-[rgba(248,81,73,0.05)] border-l-2 border-accent-red;
}
```

---

## 📋 FEATURE IMPLEMENTATION CHECKLIST

This is identical to the Qt version — every feature must be fully implemented.

### 1. Connection Management ✅
- [ ] Save connections to `~/.config/everydb/connections.toml`
- [ ] Encrypt passwords via OS keyring
- [ ] SSH tunnel support (ssh2 crate)
- [ ] SSL/TLS support
- [ ] Connection pooling
- [ ] Test connection (ping + version)
- [ ] Import/export connections

**Tauri Commands:**
```rust
#[tauri::command] async fn save_connection(config: ConnectionConfig) -> Result<(), String>
#[tauri::command] async fn test_connection(config: ConnectionConfig) -> Result<String, String>
#[tauri::command] async fn list_connections() -> Result<Vec<ConnectionConfig>, String>
```

### 2. Schema Browser ✅
- [ ] Introspect full schema (databases → schemas → tables → columns)
- [ ] Lazy loading (only load children when expanded)
- [ ] Context menu per object type
- [ ] Search across all objects
- [ ] FK navigation

**Tauri Commands:**
```rust
#[tauri::command] async fn introspect_schema(connection_id: String) -> Result<Vec<SchemaNode>, String>
#[tauri::command] async fn get_table_metadata(connection_id: String, schema: String, table: String) -> Result<TableMetadata, String>
```

**React Component:**
```tsx
// src/components/schema/SchemaTree.tsx
import { useQuery } from '@tanstack/react-query';
import { tauriCommands } from '@/lib/tauri';

export const SchemaTree = ({ connectionId }: { connectionId: string }) => {
  const { data: schema } = useQuery({
    queryKey: ['schema', connectionId],
    queryFn: () => tauriCommands.schema.introspect(connectionId),
  });
  
  return (
    <div className="w-[280px] bg-panel-bg overflow-auto">
      {/* Render tree recursively */}
    </div>
  );
};
```

### 3. Data Grid ✅ MOST CRITICAL
- [ ] Virtual scrolling (use TanStack Virtual or AG Grid)
- [ ] All row states (hover, selected, modified, new, error)
- [ ] Inline cell editing
- [ ] Type-specific cell rendering
- [ ] Pagination
- [ ] Filters (visual builder)
- [ ] Sorting (multi-column)
- [ ] Export/import

**React Component (Virtual Scrolling Example):**
```tsx
// src/components/grid/DataTable.tsx
import { useVirtualizer } from '@tanstack/react-virtual';

export const DataTable = ({ data }: { data: QueryResult }) => {
  const parentRef = useRef<HTMLDivElement>(null);
  
  const virtualizer = useVirtualizer({
    count: data.rows.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 28, // row height from Stitch
  });
  
  return (
    <div ref={parentRef} className="h-full overflow-auto">
      <div style={{ height: `${virtualizer.getTotalSize()}px`, position: 'relative' }}>
        {virtualizer.getVirtualItems().map((virtualRow) => (
          <div
            key={virtualRow.index}
            className="absolute top-0 left-0 w-full h-[28px]"
            style={{ transform: `translateY(${virtualRow.start}px)` }}
          >
            {/* Render row */}
          </div>
        ))}
      </div>
    </div>
  );
};
```

### 4. SQL Query Editor ✅
- [ ] Monaco Editor with SQL syntax highlighting
- [ ] Auto-completion
- [ ] Execute query (Ctrl+Enter)
- [ ] Query history (stored in SQLite)
- [ ] Bookmarks
- [ ] Transactions (BEGIN/COMMIT/ROLLBACK)

**React Component:**
```tsx
// src/components/editor/SqlEditor.tsx
import Editor from '@monaco-editor/react';

export const SqlEditor = () => {
  const [sql, setSql] = useState('');
  const { mutate: executeQuery } = useMutation({
    mutationFn: (sql: string) => tauriCommands.query.execute(connectionId, sql),
  });
  
  return (
    <Editor
      height="100%"
      language="sql"
      theme="vs-dark"
      value={sql}
      onChange={(value) => setSql(value ?? '')}
      options={{
        fontSize: 13,
        fontFamily: 'JetBrains Mono',
        minimap: { enabled: false },
      }}
    />
  );
};
```

### 5–12. All Other Features

(MongoDB, Redis, EXPLAIN, ER Diagram, User Management, etc.)

All implemented the same way:
- Rust backend: Tauri commands
- React frontend: Components with TanStack Query hooks
- State: Zustand for global state
- Styling: Tailwind classes from Stitch

---

## 🚀 BUILD & PACKAGE

### Development

```bash
# Install frontend dependencies
npm install

# Run dev server (Vite HMR + Tauri)
npm run tauri dev

# This opens the app window with hot reload
# Changes to Rust → restart
# Changes to React → instant HMR
```

### Build

```bash
# Build for production
npm run tauri build

# Output:
# - Linux: src-tauri/target/release/bundle/deb/everydb_1.0.0_amd64.deb
# - Linux: src-tauri/target/release/bundle/rpm/everydb-1.0.0-1.x86_64.rpm
# - Linux: src-tauri/target/release/bundle/appimage/everydb_1.0.0_amd64.AppImage
# - Windows: src-tauri/target/release/bundle/msi/everydb_1.0.0_x64_en-US.msi
# - macOS: src-tauri/target/release/bundle/dmg/everydb_1.0.0_x64.dmg
```

### Tauri Config (tauri.conf.json)

```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devPath": "http://localhost:5173",
    "distDir": "../dist"
  },
  "package": {
    "productName": "EveryDB",
    "version": "1.0.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "fs": {
        "all": true,
        "scope": ["$APPCONFIG/*", "$APPDATA/*", "$HOME/.config/everydb/*"]
      },
      "shell": {
        "all": false,
        "open": true
      }
    },
    "bundle": {
      "active": true,
      "identifier": "com.everydb.app",
      "icon": ["icons/icon.png"],
      "resources": [],
      "externalBin": [],
      "copyright": "",
      "category": "DeveloperTool",
      "shortDescription": "Universal Database IDE",
      "longDescription": "Production-grade database management for PostgreSQL, MySQL, MongoDB, Redis and more",
      "deb": {
        "depends": ["libssl3", "libgtk-3-0"]
      },
      "macOS": {
        "frameworks": [],
        "minimumSystemVersion": "10.13"
      },
      "windows": {
        "certificateThumbprint": null,
        "digestAlgorithm": "sha256",
        "timestampUrl": ""
      }
    },
    "security": {
      "csp": null
    },
    "windows": [
      {
        "title": "EveryDB",
        "width": 1440,
        "height": 900,
        "resizable": true,
        "fullscreen": false
      }
    ]
  }
}
```

---

## 📦 IMPLEMENTATION ROADMAP (24 Weeks)

**Same as before, but with Tauri/React instead of Qt/QML:**

### Phase 1 — Foundation (Weeks 1–4)
```
Week 1: Workspace + Core
├─ Cargo.toml workspace
├─ crates/core complete (driver.rs, capabilities.rs, etc.)
└─ Unit tests: cargo test -p core

Week 2: PostgreSQL Driver
├─ Full driver implementation
└─ Integration tests: cargo test -p postgres

Week 3: MySQL + SQLite Drivers
Week 4: MongoDB + Redis Drivers
```

### Phase 2 — Tauri Setup + Basic UI (Weeks 5–8)
```
Week 5: Tauri Setup
├─ src-tauri/ structure
├─ Tauri commands: connection, schema, query, data
├─ AppState with driver registry
└─ Test: tauri dev launches

Week 6: Frontend Setup
├─ React + TypeScript + Vite
├─ Tailwind config with Stitch colors
├─ TanStack Query setup
├─ Zustand store
└─ Tauri command wrappers

Week 7: Core Components
├─ AppShell (three-panel layout)
├─ Toolbar, StatusBar, Sidebar
├─ Primitive components (Button, Badge, ContextMenu)
└─ ConnectionDialog

Week 8: Schema Tree + Connection
├─ SchemaTree component (lazy loading)
├─ ConnectionCard
├─ Test: connect to PG, schema tree loads
```

### Phase 3 — Data Grid & Query Editor (Weeks 9–12)
```
Week 9–10: DataTable Component
├─ Virtual scrolling (TanStack Virtual)
├─ All row states
├─ Inline editing
├─ Cell renderers

Week 11: FilterBuilder + TableDataView
Week 12: SqlEditor (Monaco) + QueryResults
```

### Phase 4 — MongoDB & Redis (Weeks 13–16)
```
Week 13: MongoDB Views
Week 14: Redis Keyspace
Week 15: Redis Advanced (Queue, Pub/Sub)
Week 16: Testing & Polish
```

### Phase 5 — Advanced Features (Weeks 17–20)
```
Week 17: EXPLAIN View + ER Diagram (Canvas API or library like React Flow)
Week 18: Schema Diff + User Management
Week 19: Import/Export + All Dialogs
Week 20: Testing
```

### Phase 6 — Release (Weeks 21–24)
```
Week 21: Keyboard shortcuts + Packaging
Week 22: Documentation
Week 23: Testing
Week 24: Release (cargo tauri build, publish to GitHub Releases)
```

---

## ✅ ADVANTAGES OF TAURI OVER QT

1. **Stitch HTML directly usable** — No QML translation needed
2. **Smaller bundle size** — ~3MB vs ~30MB for Qt
3. **Web dev ecosystem** — NPM packages, React ecosystem
4. **Hot reload** — Instant feedback during development
5. **Cross-platform** — Windows/macOS/Linux from same codebase
6. **No Qt licensing** — Pure open source (MIT/Apache)
7. **TypeScript** — Type safety across frontend/backend boundary
8. **Easier UI development** — HTML/CSS/React vs QML

---

## 🎯 CRITICAL REMINDERS

1. **EVERY FEATURE MUST BE FULLY IMPLEMENTED**
   - No `todo!()` in production code
   - Integration tests with real databases
   - Manual testing: every button works

2. **STITCH UI IS LAW**
   - Use Tailwind classes from Stitch HTML
   - Extract colors to tailwind.config.js
   - Match pixel-perfect

3. **ARCHITECTURE RULES**
   - `crates/core` has ZERO database dependencies
   - Only `src-tauri/src/main.rs` imports driver crates
   - All DB I/O is async (tokio)
   - Tauri commands are the ONLY backend → frontend bridge

4. **CODE QUALITY**
   ```rust
   #![deny(clippy::all)]
   #![warn(clippy::pedantic)]
   #![forbid(unsafe_code)]
   ```

5. **TESTING**
   ```bash
   cargo test --all          # All unit tests
   cargo test -p postgres    # Integration tests with testcontainers
   npm run tauri dev         # Manual testing
   ```

---

*EveryDB — One IDE for Every Database.*
*Native Performance. Web Technologies UI. Production-Grade.*
*Built with Rust + Tauri. Designed with Stitch.*