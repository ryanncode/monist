# Monist

Monist is a high-performance Rust translation of the Lean 4 constraint mapping and untyped combinatory execution logic engine. It is designed to abandon hierarchical type universes and execute Quine's monist ontology (New Foundations) natively.

## Overview

This workspace serves as the foundational Rust backend for the logic engine. It strictly bifurcates mathematical bounding constraints from logical branching, evaluates topology via Bellman-Ford and Kosaraju SCC, and executes via a flat topological combinator runtime on massively parallel hardware (HVM2).

### Crates

* **`monist-core`**: The foundational constraint solver. Handles the bifurcated AST, DNF expansion, graph constraint generation via contiguous integer memory arenas, Kosaraju SCC 0-weight cycle collapse, Bellman-Ford negative-weight cycle (Extensionality Collision) detection, and Minimum Cycle Mean (MCM) execution limit calculations.
* **`monist-comb`**: The execution backend and combinatory compiler. 
  * Implements Bracket Abstraction to translate logical ASTs into unrolled `S`, `K`, `I`, `B`, and `C` functional combinators.
  * Dynamically computes Algorithmic T-Weaking to mechanically synthesize and inject topological friction via the $T$-operator (`Comb::T`).
  * Enforces Topologically-Guided Call-by-Need bounds to safely halt infinite recursions (e.g., $V \in V$) inside `Comb::Limit`.
  * Integrates the native `.bend` compilation pipeline and hardware backend handoff (`hvm gen-cu`).
  * Contains `CombLib`, implementing Transfinite Macro-Primitives such as choice-free Cardinal Arithmetic, Quine atoms ($\Omega = \{\Omega\}$), and Holographic Data Indexing.
* **`monist-category`**: The Categorical Semantics layer. Implements the Stratified Pseudo Elephant (SPE) architecture containing $T$-Relative Adjunctions, Strongly Cantorian (SC) Retractions via Knaster-Tarski stability, and Stratified Yoneda Lemma traversals across non-well-founded constraints.
* **`monist-seq`**: The diagnostic tool suite. Evaluates Sequent Calculus derivations and surfaces extensionality collisions.
* **`monist-parser`**: The parsing environment. Translates string inputs into `monist-core` AST representations.
* **`monist-cli`**: The user interface. Provides a REPL sandbox and diagnostic logging entrypoint.

## Architecture

The engine architecture relies on pre-computing topological routes and limits before converting variables to physical graph bindings. For detailed execution semantics and structural forms, refer to [ARCHITECTURE.md](ARCHITECTURE.md), `COMB_CAT_PLAN.md`, and the theoretical documentation in `test/`.

## Building

Ensure you have Rust, Cargo, and the [HVM2 toolchain](https://github.com/HigherOrderCO/hvm) installed. Build the entire workspace from the root:

```bash
cargo build
```

Run the unit and integration tests across all crates:

```bash
cargo test
```
