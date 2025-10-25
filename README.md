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

- The `keypairs.bin` file stores the private/public key pairs for two accounts (one with account index of `0` and another with `1`). This program is, of course, capable of handling a ledger with more than one account, but, for simplicity and demonstration purposes, only two are stored in disk.

- The coinbase reward of each mined block is rewarded to Account 0.

- Avoid inputting the same filename (for storing chain branches) for both node instances.

## Explanation of Various Self-Devised Algorithms

### Chain Identification and Validation

This overly-inefficient alogrithm runs in O(n^2) time complexity. It works as follows: to identity all branches/forks of a chain (which is a vector of blocks, for now), reverse the order of the chain, use a HashMap to record which blocks have been iterated through before (to avoid having overlapping forks), for each block iterated that is not recorded in the HashMap, loop through all of the previous blocks that are chained. This gives all available forks in the chain and a simple check to see which fork has the highest block height yields the valid chain.

### UTXO Computation

From the genesis block onwards, iterate through each block. Within each block, iterate through each transaction. Within each transaction, iterate through each transaction input. For each transaction input, iterate through all UTXO transactions to find the previous transaction referenced by the input, and remove the output referenced. Validating transactions (checking if output amount <= input amount and verifying signature) is done when accepting a block; once the block is accepted, all transactions are trusted to be valid.

### Peer-to-Peer Connectivity Protocol

When a node initiates a TCP stream with another node, the first node sends a `handshake_number`, which determines the purpose of the TCP stream (whether it is propagating new blocks or getting the blockchain for a new node) with a fixed and known byte length.
