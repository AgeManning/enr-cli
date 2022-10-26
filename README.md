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

This is a simple CLI utility for reading base64 encoded ENRs as well as
building new ones.

Features may be added in the future.

## Install

This can be installed via cargo:

```bash
$ cargo install enr-cli
```

## Usage

```bash
Sigma Prime <contact@sigmaprime.io>
Simple CLI for reading and building ENRs.

Usage: enr-cli [COMMAND]

Commands:
  read   Reads and ENR
  build  Builds an ENR
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## Example

```bash
$ enr-cli read enr:-Ku4QJsxkOibTc9FXfBWYmcdMAGwH4bnOOFb4BlTHfMdx_f0WN-u4IUqZcQVP9iuEyoxipFs7-Qd_rH_0HfyOQitc7IBh2F0dG5ldHOIAAAAAAAAAACEZXRoMpD1pf1CAAAAAP__________gmlkgnY0gmlwhLAJM9iJc2VjcDI1NmsxoQL2RyM26TKZzqnUsyycHQB4jnyg6Wi79rwLXtaZXty06YN1ZHCCW8w

ENR Read:
Sequence No:1
NodeId: 3ab5eb24b287e4fc130fe25ed7424626fd9b53c5068b9778f42d1c7bb0831447
EnodeId: enode://3ab5eb24b287e4fc130fe25ed7424626fd9b53c5068b9778f42d1c7bb0831447@176.9.51.216:?discport=23500
Libp2p PeerId: 16Uiu2HAmC13Brucnz5qR8caKi8qKK6766PFoxsF5MzK2RvbTyBRr
IP:176.9.51.216
UDP Port:23500
Eth2 Field:
	Fork digest: f5a5fd42
	Next fork version: 00000000
	Next fork epoch: 18446744073709551615
	SSZ Bytes: f5a5fd4200000000ffffffffffffffff
Known multiaddrs:
/ip4/176.9.51.216/udp/23500
```
