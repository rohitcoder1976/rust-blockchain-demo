## Overview

This project is a Rust-powered Bitcoin-mock blockchain. It uses the same proof-of-work consensus mechanism, similar mining algorithm, and self-inspired overly-complex (both in time and space) mechanisms for longest-chain validation and UTXO generation.

Peer-to-peer (with one trusted peer) connectivity using raw TCP (for trusted nodes only for now) is included as a feature.

## Getting Started

This project uses the Rust package manager `cargo`. Ensure that you have a version of cargo close to `1.83.0`, which is what this project was built on.

To run the project, run this command:

```
cargo run
```
