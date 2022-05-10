# snmp-sim-cli
SNMP Simulator Managemet CLI

## Overview

This tool allows you to remotelly control an instance of SNMP Simulator through HTTP Rest API.

## Usage

All commands require that a SNMP Simulator HTTP Rest API url is provided. This can be done either setting the URL command line option

```shell
SNMP_SIM_URL=http://localhost:8180 snmp-sim-cli agents
```

or by setting SNMP_SIM_URL environment variable

```shell
snmp-sim-cli http://localhost:8180 agents
```

### Getting help

Use `-h` or `--help` flag on any CLI command or subcommand for help.

```shell
snmp-sim-cli 0.1.0
SNMP Simulator Management CLI

USAGE:
    snmp-sim-cli <URL> <SUBCOMMAND>

ARGS:
    <URL>    [env: SNMP_SIM_URL=]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    agent      Manage SNMP Agents
    agents     List SNMP Agents
    device     Manage Devices
    devices    List Devices
    help       Print this message or the help of the given subcommand(s)
```

#### Manage an Agent

```shell
snmp-sim-cli-agent
Manage SNMP Agents

USAGE:
    snmp-sim-cli <URL> agent <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    add       Create a new instance of SNMP Agent
    get       Get Agent by ID
    help      Print this message or the help of the given subcommand(s)
    ls        List SNMP Agents
    rm        Remove Agent by ID
    update    Update an existing instance of SNMP Agent
```

#### Manage a Device

```shell
snmp-sim-cli-device
Manage Devices

USAGE:
    snmp-sim-cli <URL> device <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    add       Create a new instance of SNMP Device
    get       Get Device by ID
    help      Print this message or the help of the given subcommand(s)
    ls        List SNMP Devices
    rm        Remove Device by ID
    start     Start a Device by ID
    stop      Stop a Device by ID
    update    Update an existing instance of SNMP Device
```

## License

This SNMP Simulator CLI tool is licensed under the [APACHE-2.0](https://www.apache.org/licenses/LICENSE-2.0) license.

## Contributing

Want to contribute? Great 🎉

There are many ways to give back to the project, whether it be writing new code, fixing bugs, or just reporting errors. All forms of contributions are encouraged!

For instructions on how to contribute, see our [Guide to contributing](https://github.com/sonalake/snmp-sim-rust/blob/main/CONTRIBUTING.md).