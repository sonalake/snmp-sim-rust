# snmp-sim-rust
SNMP Simulator

[![Build](https://github.com/sonalake/snmp-sim-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/sonalake/snmp-sim-rust/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/sonalake/snmp-sim-rust/branch/main/graph/badge.svg?token=23507AD585)](https://codecov.io/gh/sonalake/snmp-sim-rust)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

# Pre-requisite

To only compile the application manually you'll need to install:

- [Rust](https://www.rust-lang.org/tools/install)
- [cargo-make](https://github.com/sagiegurari/cargo-make#installation)

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

## Using `cargo`

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


## Using `docker`
```bash
docker build -t snmp-sim .
```

```bash
docker run -p 127.0.0.1:8161:8161/udp --name snmp-sim -t snmp-sim &
```

```bash
docker stop snmp-sim
```

## Using `docker-compose`

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