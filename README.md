# SNMP Simulator

[![Build](https://github.com/sonalake/snmp-sim-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/sonalake/snmp-sim-rust/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/sonalake/snmp-sim-rust/branch/main/graph/badge.svg?token=23507AD585)](https://codecov.io/gh/sonalake/snmp-sim-rust)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

# Pre-requisite

To only compile the application manually you'll need to install:

- [rust](https://www.rust-lang.org/tools/install)
- [cargo-make](https://github.com/sagiegurari/cargo-make#installation)
- [sqlx-cli](https://crates.io/crates/sqlx-cli)
- [sea-orm-cli](https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/)
- [openapi-generator](https://openapi-generator.tech/docs/installation/)
- [gsed (Only required for Mac, other OS's use sed)](https://formulae.brew.sh/formula/gnu-sed)

To use the automated build/test tools, we recommend that you install:

- [Brew](https://brew.sh/)

Once Brew is installed, you can install both  `docker` and GNU `make`:

```bash
brew install docker make
```

Since macOS provides an older version of GNU `make` (v3.81 dated April 1, 2006),
we recommend installing and using `gmake`, which provides numerous features and
performance enhancements.

To use containerization platform, we recommend that you instasll:

- [Docker](https://docs.docker.com/engine/install/)
- [Docker compose](https://docs.docker.com/compose/install/)


For testing purposes, we recommend that you install:

- [Net-SNMP package](http://www.net-snmp.org/)

# How to build

## Build with `cargo`

```bash
cargo build --release
```

Build the application and its container-image by using `make` (or `gmake`):

```bash
make
```

The deafult is to compile the application in debug-mode. If you want to build a
release-mode binary, then set the `RELEASE_OR_DEBUG` flag to `release`:

```bash
make RELEASE_OR_DEBUG=release
```

## Build with `cargo-make`

```bash
cargo make build
```

## Build with `docker`
```bash
docker build -t snmp-sim .
```

```bash
docker run -p 127.0.0.1:8161:8161/udp --name snmp-sim -t snmp-sim &
```

```bash
docker stop snmp-sim
```

## Build with `docker-compose`

```bash
docker-compose build
```

```bash
docker-compose up -d
```

```bash
docker-compose down
```


# How to run the tests using `cargo`

The following command will invoke doc, unit-tests and integration-tests execution:

```bash
cargo test
```

or by using cargo-make

```bash
cargo make test
```

# Development

Implementation of the SNMP Simulator service is relying on the [Actix](https://github.com/actix/actix-web) web frammework.

The service implements an HTTP REST API and exposes it to access the simulator functional operations.

[YAML](https://yaml.org/spec/1.2.2/) files are used to store the static service configuration. The configuration files can be extended via merging configuration parameters from environment variables.

The SNMP Simulator uses [SQLite](https://sqlite.org/index.html) database as a persistent storage of runtime configuration.

## Configuration

The static service configuration is expected at `./configuration/base.yaml` mandatory file. The base configuration can be extended or overriden by optional configuration file expected at `./configuration/local.yaml`.

An example of base.yaml configuration:
```yaml
application:
  host: 0.0.0.0
  port: 8080
  uri_prefix: "mngmt/v1"
  level: "error"

database:
  connection_uri: "sqlite://~/.snmp-sim/snmp-sim.db"
```
An example of local.yaml configuration:
```yaml
application:
  host: localhost
  port: 8180
  uri_prefix: ""
  level: "trace"
```

### Configuration from Environment Variable

In addition to static configuration files, environment variables can be used to define the service configuration. This is useful with automated CI or cloud environments.

Setting the `APP__APPLICATION__PORT=5001` environment variable overrides the `application.port` static configuration file  content.

## Database

SNMP Simulator is relying on [SeaORM](https://github.com/SeaQL/sea-orm) relational, async and dynamic ORM crate which provides abstraction over common operations against an SQLite database.
[SQLx](https://github.com/launchbadge/sqlx) crate is used as SeaORM's underlying driver.

### Create and Run Migrations

```bash
sqlx migrate add <name>
```

Creates a new file in `migrations/<timestamp>-<name>.sql`. Add your database schema changes to this new file.
The SNMP simulator executes the migrations scripts as part of startup procedure.
The SQLite database is created, if not exists. The database path and filename can be configured by the configuration file.

You can run the database migrations scripts manually by:

```bash
sqlx migrate run
```

Every script is executed in the database only once, even if the migration is invoked multiple times.

### Update Database Entity Files

SeaORM can discover all tables in a database and generate a corresponding SeaORM entity files  for each table.

Running the following command, the database entities implementations stored in `./src/data_access/entity` folder are auto-generated, so never modify the content of that folder, since it will be overwritten.

```bash
cargo make db-entity
```

## OpenAPI Specification

The HTTP RestAPI specification is generated from the server implementation by running a build script.
The `generate_spec` binary is built together with the `snmp-sim` crate and binary. The `generate_spec` binary exports the current openapi specification to the output file. It needs to be invoked manually to generate the actual HTTP RestAPI openapi specification.

## HTTP RestAPI Rust Client

The rust client implementation is auto-generated from the openapi specification by running a build script.

```bash
cd clients/rust
cargo build
```

First the cargo builds and runs the build script that invokes the `generate_spec` binary to create the current openapi specification stored at [openapi.json](docs/openapi.json)

Then the rust client code is generated by invoking the [openapi-generator](https://openapi-generator.tech/) using the `openapi.json` directly from the build script at compile time.

The version of the generated rust client crate is based on the version specified in `./clients/rust/Cargo.toml`. Follow the [SemVer](https://doc.rust-lang.org/cargo/reference/semver.html) compatibility quidelines, whenever the HTTP RestAPI specification is changed.

## Swagger UI

The HTTP RestAPI openapi spectification is exposed by Swagger UI by browsing the service `/swagger` path, e.g. `http://localhost:8180/swagger`

# SNMP Simulator CLI

See [README](./snmp-sim-cli/README.md) for more details.

# Testing using `snmpget`

The `snmp-sim` can be also tested by an external tool `snmpget`:

```bash
snmpget -v1 -c public localhost:8161 .1.3.6.1.4.1.11.2.14.11.5.1.1.2
snmpget -v2c -c public localhost:8161 .1.3.6.1.4.1.11.2.14.11.5.1.1.2
snmpget -v3 -c public localhost:8161 .1.3.6.1.4.1.11.2.14.11.5.1.1.2
```

## License

This SNMP Simulator CLI tool is licensed under the [APACHE-2.0](https://www.apache.org/licenses/LICENSE-2.0) license.

## Contributing

Want to contribute? Great 🎉

There are many ways to give back to the project, whether it be writing new code, fixing bugs, or just reporting errors. All forms of contributions are encouraged!

For instructions on how to contribute, see our [Guide to contributing](https://github.com/sonalake/snmp-sim-rust/blob/main/CONTRIBUTING.md).
