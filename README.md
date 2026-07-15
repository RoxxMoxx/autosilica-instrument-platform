# AutoSilica Instrument Platform

Production-grade Rust skeleton for the AutoSilica Instrument
Platform: a Clean Architecture, SOLID-principled Cargo workspace with
an Axum REST API, Tokio async runtime, layered configuration,
structured logging, and typed error handling.

> **Scope note:** this is a project skeleton only. No instrument
> communication (serial, TCP, GPIB, SCPI, drivers, etc.) is
> implemented yet. The `domain::repositories::InstrumentRepository`
> port and the `infrastructure` layer are designed so a real driver
> adapter can be added later without touching `domain`, `application`,
> or `api`.

## Architecture

Clean Architecture, dependencies point inward only:

```
app (composition root / binary)
 └─ api            (Axum HTTP layer)
     └─ application (use cases, DTOs)
         └─ domain   (entities, value objects, repository ports)
 └─ infrastructure  (config, logging, repository adapters — implements domain ports)
```

* **`domain`** — pure business logic. No dependency on any other layer.
* **`application`** — use cases orchestrating `domain`. Depends on `domain` only.
* **`infrastructure`** — technical adapters (config loading, tracing setup,
  in-memory repository) implementing `domain` ports. Depends on `domain` only.
* **`api`** — Axum routes/handlers/middleware. Depends on `application` and `domain`.
* **`app`** — binary composition root. Depends on all of the above and wires them together.

This arrangement follows SOLID:
* **S**ingle Responsibility — each crate/module has one reason to change.
* **O**pen/Closed — new repository adapters (e.g. a real database, an
  instrument driver) can be added without modifying `domain` or `application`.
* **L**iskov Substitution — any `InstrumentRepository` implementation is
  interchangeable behind the trait.
* **I**nterface Segregation — narrow, purpose-built traits/DTOs per use case.
* **D**ependency Inversion — `domain` defines ports; `infrastructure` implements them.

## Folder structure

```
.
├── Cargo.toml                # workspace manifest + shared [workspace.dependencies]
├── .env.example               # sample environment variables
├── config/                    # layered TOML configuration
│   ├── default.toml
│   ├── development.toml
│   └── production.toml
├── crates/
│   ├── domain/                # entities, value objects, repository/discovery ports, errors
│   ├── application/           # use cases, DTOs, application errors
│   ├── infrastructure/        # config loading, tracing setup, repository + VISA discovery adapters
│   └── api/                   # Axum routes, handlers, middleware, HTTP error mapping
├── app/                       # composition root binary (`autosilica-server`)
└── tests/                     # workspace-level integration tests
```

## Configuration

Settings are loaded (later overrides earlier) from:

1. `config/default.toml`
2. `config/{APP_ENV}.toml` (optional; `APP_ENV` defaults to `development`)
3. Environment variables prefixed `AUTOSILICA__`, double-underscore separated
   (e.g. `AUTOSILICA__SERVER__PORT=9090`)

Copy `.env.example` to `.env` and adjust as needed; it is loaded automatically at startup.

## Running

```bash
cp .env.example .env
cargo run -p autosilica-server
```

The server starts on the configured `server.host:server.port`
(default `0.0.0.0:8080`) and exposes:

* `GET  /health`
* `GET  /api/v1/instruments`
* `POST /api/v1/instruments`
* `GET  /api/v1/instruments/:id`
* `GET  /api/v1/discovery` — VISA instrument discovery (see below)

## Instrument discovery

`GET /api/v1/discovery` enumerates VISA resources (USB, LAN/TCPIP,
GPIB, Serial/ASRL), opens each, and sends `*IDN?`, returning parsed
vendor/model/serial/firmware:

```json
[
  {
    "id": "dsox3024a-01",
    "resource": "TCPIP0::192.168.1.100::INSTR",
    "vendor": "Keysight Technologies",
    "model": "DSOX3024A",
    "serial": "MY123456",
    "firmware": "02.41"
  }
]
```

Implementation notes:

* The VISA runtime (NI-VISA, Keysight IO Libraries, or any other
  implementation exposing the standard VISA C ABI) is loaded
  **dynamically at runtime** via `libloading` (`crates/infrastructure/src/visa`),
  not linked at compile time. The workspace builds and runs fine on
  machines without any VISA implementation installed — discovery then
  logs a warning once and returns `[]`.
* `domain::discovery::InstrumentDiscoveryPort` is the port; `infrastructure::visa::NiVisaDiscoveryBackend`
  is the adapter. Swapping in a different discovery mechanism later
  requires no changes to `domain`, `application`, or `api`.
* Only resource enumeration and `*IDN?` are implemented. SCPI command
  execution and measurements are explicitly out of scope for this
  module.

## Development

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets
cargo fmt --all
```

## Roadmap (not implemented in this skeleton)

* SCPI command execution and measurements against discovered/registered
  instruments.
* Persistent storage adapter (e.g. Postgres via `sqlx`) implementing
  `InstrumentRepository`.
* Persisting/reconciling discovered instruments into the registered
  `InstrumentRepository`.
* AuthN/AuthZ middleware.
* OpenAPI schema generation.

## Toolchain note

This project was built and tested against **Rust 1.75** (the version
available via `apt` on Ubuntu 24.04 in the environment this skeleton
was generated in). `Cargo.lock` intentionally pins several transitive
dependencies (`indexmap`, `hashbrown`, `uuid`, `pest*`,
`unicode-segmentation`) below versions that require newer Rust
(`edition2024` / raised MSRVs), so `cargo build --workspace` succeeds
out of the box on 1.75+. If you're on a recent stable toolchain (1.85+)
and want the latest dependency versions instead, delete `Cargo.lock`
and run `cargo update` before building.
