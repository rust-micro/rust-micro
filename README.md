# rust-micro

Rust Micro provides the core requirements for distributed systems development including RPC and Event driven communication. The Rust Micro philosophy is sane defaults with a pluggable architecture. We provide defaults to get you started quickly but everything can be easily swapped out.

## Inspirations
- [go-micro](https://github.com/go-micro)

## Requirements

- Docker
- Rust > 1.60 with Cargo > 1.60

## Installation

```bash
$ cargo install rust-micro
$ cargo micro check # checks the requirements
```

## Usage

```bash
$ cargo micro new my-project
$ cd my-project
$ cargo micro run # sets up all needed services and starts all bins in cargo.toml
```


