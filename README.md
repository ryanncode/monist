# Monist

Monist is a high-performance Rust translation of the Lean 4 constraint mapping and untyped combinatory execution logic engine.

## Overview

This workspace serves as the foundational Rust backend for the logic engine, strictly bifurcating mathematical bounding constraints from logical branching, and executing evaluation via a flat topological combinator runtime.

### Crates

* **`monist-core`**: The foundational constraint solver. Handles the AST, DNF expansion, graph constraint generation, and the Bellman-Ford engine for verifying weak stratification boundaries.
* **`monist-comb`**: The execution backend. Implements the flat untyped combinatory logic (S, K, I, U), T-Operator dynamic injection, and prepares the groundwork for topologically-guided lazy reduction.
* **`monist-seq`**: The diagnostic tool suite. Evaluates Sequent Calculus derivations and surfaces extensionality collisions.
* **`monist-parser`**: The parsing environment. Translates string inputs into `monist-core` AST representations.
* **`monist-cli`**: The user interface. Provides a REPL sandbox and diagnostic logging entrypoint.

## Architecture

For a detailed breakdown of the engine components, including constraint matrices and the combinator AST structure, please refer to [ARCHITECTURE.md](ARCHITECTURE.md).

## Building

Ensure you have Rust and Cargo installed, then build the entire workspace from the root:

```bash
cargo build
```

Run the tests across all crates:

```bash
cargo test
```
