[![ci](https://github.com/OkieOth/mongoschema/actions/workflows/test.yml/badge.svg)](https://github.com/OkieOth/mongoschema/actions/workflows/test.yml)
![WIP](https://img.shields.io/badge/work%20in%20progress-red)

# TL;DR;

A tool to guess the schema behind collection data of mongodb


# Usage

## Create a binary

```bash
cargo build release

cd target/release
```

## Query existing databases

```bash
# prints available commands
./mongoschema

# show available commandline switches
./mongoschema dbs --help

# list existing databases
./mongoschema dbs \
    --conn-str mongodb://admin:secretpassword@localhost:27017/admin \
    --verbose
```