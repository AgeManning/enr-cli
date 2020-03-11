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

This is a simple CLI utility for reading base64 encoded ENRs.

Features may be added in the future.

## Install

This can be installed via cargo:

```bash
$ cargo install enr-cli
```

## Usage

```bash
Sigma Prime <contact@sigmaprime.io>
Simple CLI for reading and modifying ENRs.

USAGE:
    enr-cli --enr <BASE64-ENR>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --enr <BASE64-ENR>    Reads a base64 ENR and prints common parameters.
```

## Example

```bash
$ enr-cli -e -Iu4QM-YJF2RRpMcZkFiWzMf2kRd1A5F1GIekPa4Sfi_v0DCLTDBfOMTMMWJhhawr1YLUPb5008CpnBKrgjY3sstjfgCgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQP8u1uyQFyJYuQUTyA1raXKhSw1HhhxNUQ2VE52LNHWMIN0Y3CCIyiDdWRwgiMo 
ENR Read
Sequence No: 2
Node ID: 0x2ba1..acde
IP: 127.0.0.1
TCP port: 9000
UDP port: 9000
```
