## Overview

This project is a Rust-powered Bitcoin-mock **permissioned** blockchain. It uses the same proof-of-work consensus mechanism, similar mining algorithm, and self-inspired overly-complex (both in time and space) mechanisms for longest-chain validation and UTXO generation.

Peer-to-peer (with one trusted peer) connectivity using raw TCP (for trusted nodes only for now) is included as a feature.

## Getting Started

This project uses the Rust package manager `cargo`. Ensure that you have a version of cargo close to `1.83.0`, which is what this project was built on.

You will need to run two terminal instances of the project, as it is a permissioned network of two nodes.

To run one instance, run the following command in your terminal:

```
cargo run
```

## Things to Note

The `keypairs.bin` file stores the private/public key pairs for two accounts (one with account index of `0 ` and another with `1`). This program is, of course, capable of handling a ledger with more than one account, but, for simplicity and demonstration purposes, only two are stored in disk.
