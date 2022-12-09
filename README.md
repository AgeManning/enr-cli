enr-cli
============

[![Build Status]][Build Link] [![Doc Status]][Doc Link] [![Crates
Status]][Crates Link]

[Build Status]: https://github.com/AgeManning/enr-cli/workflows/build/badge.svg?branch=master
[Build Link]: https://github.com/AgeManning/enr-cli/actions
[Doc Status]: https://docs.rs/enr-cli/badge.svg
[Doc Link]: https://docs.rs/enr-cli
[Crates Status]: https://img.shields.io/crates/v/enr-cli.svg
[Crates Link]: https://crates.io/crates/enr-cli

[Documentation at docs.rs](https://docs.rs/enr-cli)

## Overview

This is a simple CLI utility for reading base64 encoded ENRs as well as building new ones.

## Install

This can be installed via cargo:

```bash
$ cargo install enr-cli
```

## Usage

`enr-cli` has two primary subcommands, `read` and `build`. The `read` subcommand takes a base64 encoded ENR and prints out the common parameters. The `build` subcommand takes a private key and a set of parameters and builds a new ENR. If no private key or private key file path is specified, a random one will be generated. If the private key is invalid or the private key file cannot be parsed, an error will be returned. The build subcommand also allows you to specify ENR parameters such as IP, TCP port, UDP port, and Eth2 fork digest.

_The following is the help text for the `enr-cli` command, and can be accessed via `enr-cli --help`_

```bash
Simple utility to read Ethereum Node Records (ENR)

Usage: enr-cli [OPTIONS] [COMMAND]

Commands:
  build
          Builds an ENR
  read
          Reads an ENR
  help
          Print this message or the help of the given subcommand(s)

Options:
  -v, --log-level <LOG_LEVEL>
          Sets the logging verbosity level.

          [default: info]

          Possible values:
          - trace: Trace level
          - debug: Debug level
          - info:  Info level
          - warn:  Warn level
          - error: Error level

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```

**Read Subcommand**

_The following is the help text for the `read` subcommand, and can be accessed via `enr-cli read --help`_

```bash
Read Command Options

Reads a base64 ENR and prints common parameters.

Usage: enr-cli read <ENR>

Arguments:
  <ENR>
          The base64-encoded ENR.

Options:
  -h, --help
          Print help information (use `-h` for a summary)
```

**Build Subcommand**

_The following is the help text for the `build` subcommand, and can be accessed via `enr-cli build --help`_

```bash
Build Command Options

Builds a base64 ENR from a private key and parameters.

Usage: enr-cli build [OPTIONS]

Options:
  -k, --private-key <PRIVATE_KEY>
          A hex encoded private key to use for signing. If this or --key-file is not specified a random one will be generated.

  -j, --key-file <KEY_FILE>
          Path to a key file that stores raw bytes of an ENR key. Example for lighthouse is in ~/.lighthouse/mainnet/beacon/network/key.dat.

  -i, --ip <IP>
          Set an ip address

  -s, --seq-no <SEQ>
          Set a sequence number

  -p, --tcp-port <TCP_PORT>
          Set an tcp port

  -u, --udp-port <UDP_PORT>
          Set an udp port

  -f, --eth2 <ETH2>
          Set an eth2 fork field. Takes the raw SSZ bytes input

  -h, --help
          Print help information (use `-h` for a summary)
```

## Examples


#### Read Subcommand

```bash
$ enr-cli read enr:-Ku4QJsxkOibTc9FXfBWYmcdMAGwH4bnOOFb4BlTHfMdx_f0WN-u4IUqZcQVP9iuEyoxipFs7-Qd_rH_0HfyOQitc7IBh2F0dG5ldHOIAAAAAAAAAACEZXRoMpD1pf1CAAAAAP__________gmlkgnY0gmlwhLAJM9iJc2VjcDI1NmsxoQL2RyM26TKZzqnUsyycHQB4jnyg6Wi79rwLXtaZXty06YN1ZHCCW8w

2022-12-09T17:22:40.263Z INFO  [enr_cli::common::output] ENR Read:
2022-12-09T17:22:40.263Z INFO  [enr_cli::common::output] Sequence No:1662390331644
2022-12-09T17:22:40.263Z INFO  [enr_cli::common::output] NodeId: e733f215211b0817b50f09230a3e4fc84de4fbdc8af0b8da68317dfc7ef825f0
2022-12-09T17:22:40.263Z INFO  [enr_cli::common::output] EnodeId: enode://e733f215211b0817b50f09230a3e4fc84de4fbdc8af0b8da68317dfc7ef825f0@95.216.242.53:13000?discport=12000
2022-12-09T17:22:40.263Z INFO  [enr_cli::common::output] Libp2p PeerId: 16Uiu2HAmC6YniAnDnBBYXDDvBtFtKq9q5RpLLJhb3LFNRpHmyn98
2022-12-09T17:22:40.263Z INFO  [enr_cli::common::output] IP:95.216.242.53
2022-12-09T17:22:40.263Z INFO  [enr_cli::common::output] TCP Port:13000
2022-12-09T17:22:40.263Z INFO  [enr_cli::common::output] UDP Port:12000
2022-12-09T17:22:40.263Z INFO  [enr_cli::common::output] Eth2 Field:
	Fork digest: 4a26c58b
	Next fork version: 02000000
	Next fork epoch: 18446744073709551615
	SSZ Bytes: 4a26c58b02000000ffffffffffffffff
2022-12-09T17:22:40.263Z INFO  [enr_cli::common::output] Known multiaddrs:
2022-12-09T17:22:40.263Z INFO  [enr_cli::common::output] /ip4/95.216.242.53/udp/12000
2022-12-09T17:22:40.263Z INFO  [enr_cli::common::output] /ip4/95.216.242.53/tcp/13000
```

#### Build Subcommand

```bash
2022-12-09T17:23:13.521Z INFO  [enr_cli::services::build::runner] Built ENR: enr:-HW4QEpkuoRGsoMd92WpEtVLnRm3msDgnswF6urU_f-Udf5KDw5hbA6g52Jejqv03Kgr8DSeOe7PxQOBzvoWFDQdzt0BgmlkgnY0iXNlY3AyNTZrMaEDmKFQ0Dkl1BLnAux-7NU7IPcVGaGoW7fEcc6nFCwYjfk
2022-12-09T17:23:13.521Z INFO  [enr_cli::services::build::runner]
2022-12-09T17:23:13.521Z INFO  [enr_cli::common::output] ENR Read:
2022-12-09T17:23:13.521Z INFO  [enr_cli::common::output] Sequence No:1
2022-12-09T17:23:13.521Z INFO  [enr_cli::common::output] NodeId: e04d4199344e74586a04770194cbaf360dee56b03c4724b09a8ad6af2e5c8c4a
2022-12-09T17:23:13.521Z INFO  [enr_cli::common::output] EnodeId: enode://e04d4199344e74586a04770194cbaf360dee56b03c4724b09a8ad6af2e5c8c4a
2022-12-09T17:23:13.522Z INFO  [enr_cli::common::output] Libp2p PeerId: 16Uiu2HAmNvnzKZ5VszDerQ4EPpojUNmykrHJ7ZZk17qCZxFFwBSQ
```
