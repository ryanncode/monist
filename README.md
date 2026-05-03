# The Monist Engine

This logic engine is a high-performance evaluator and bare-metal compilation pipeline designed exclusively to compute Quine's New Foundations (NF). By abandoning traditional hierarchical type-checking in favor of geometric shortest-path routing (Bellman-Ford), the project successfully compiles unstratified, self-referential paradoxes into executable Interaction Nets. 

It mathematically computes the precise topological weight of systemic ambiguity, catching extensionality collisions before offloading purely untyped combinator nodes to the GPU.

---

## Scalable Oversight

The frontier of machine learning is overwhelmingly probabilistic. Neural architectures generate outputs that resist strict, structural verification. The Monist Engine provides the exact opposite: a deterministic execution layer capable of safely evaluating infinitely recursive or cyclical structures. 

As the industry shifts its focus from experimental models toward scalable oversight and governance, organizations require mathematical guarantees over black-box systems. The Monist Engine's non-well-founded set evaluator serves as the deterministic bedrock necessary to audit, govern, and formally verify probabilistic agents, delivering absolute structural certainty.

---

## Capabilities & Architecture

The standard computing industry enforces memory safety via strict Directed Acyclic Graphs (DAGs)—evidenced by Rust’s borrow checker or Lean 4’s dependent type hierarchy. If you attempt to feed a structurally dense, cyclic self-reference like the Universal Set ($V \in V$) into standard frameworks, they immediately crash, resulting in either a syntax error or a VRAM exhaustion (warp divergence).

The Monist Engine subverts this by bifurcating the computational stack:

1.  **The Oracle Layer (CPU/Geometry):** The frontend parser transforms standard first-order logic into an algebraic system of constraints. Instead of rejecting cyclic graphs, the `monist-core` executes Kosaraju's SCC algorithm to flatten 0-weight semantic cycles, and deploys the Bellman-Ford algorithm to map the thermodynamic weight between nodes. If a paradox forms, the engine dynamically calculates the $K$-Iteration depth boundary and intercepts the recursion mathematically.
2.  **The T-Functor Synthesis:** When dense impredicative recursion forms (like the Burali-Forti sequence), the engine dynamically synthesizes and injects a `T-operator` ($x \mapsto \iota"x$), acting as a topological stabilizer that absorbs the structural friction, preserving the weak stratification boundary.
3.  **The Interaction Net Backend (GPU/Physics):** Stripped of all types and hierarchical scaffolding, the validated logic is compiled into pure $S, K, I$ combinators (`monist-comb`). We bypass traditional substitution environments, feeding these localized atomic nodes directly to HVM2/Bend-style Interaction Nets. The hardware natively folds self-reference topologically, scaling across millions of lock-free collisions per second.

---

## Build & Run

Ensure you have Rust and Cargo installed, alongside an OpenCL-compatible driver if targeting discrete GPU execution.

```bash
# Clone the repository
git clone https://github.com/your-org/monist.git
cd monist

# Build the entire workspace (Crates, Tools, and Benches)
cargo build --release
```

### The Interactive REPL
To start the tactical theorem-proving environment:
```bash
cargo run -p monist-cli -- repl
```
Inside the REPL, you can `assume Extensionality`, run backwards-reasoning tactics (`intro`, `apply`, `destruct`), or run a live, color-coded diagnostic visualizer using `step "x in x"`.

### Visual Demonstrations
The Engine includes visual terminal simulations illustrating its unique performance boundaries:
```bash
# Demonstrate O(1) Exclusion-First Routing for Holographic Queries
cargo run -p monist-cli -- demo holographic

# Demonstrate the Principle of Least Syntactic Action for AI Agentic boundaries
cargo run -p monist-cli -- demo agentic
```

---

## Benchmarks & Mathematical Diagnostics

The engine's validity is proven via a suite of automated diagnostic refutations located in `tools/monist-examples/src/bin/`. These execute the core paradoxes of modern set theory, outputting the mathematically verified topological boundaries and generating standard `SMT-LIB v2` witnesses for third-party prover ingestion.

*   **Specker's Refutation of Global Choice (`specker_refutation.rs`)**: Mechanically proves that bridging disjoint integer weight elevations ($\Phi(m)$ vs $\Phi(T(m))$) without a $T$-operator creates a negative-weight cycle, validating the absolute halting limit.
*   **The Extensionality Collision (`extensionality_collision.rs`)**: Evaluates the Kuratowski ordered pair vs the Quine ordered pair, proving the engine tracks dense structural depth offsets (+2 vs 0) without triggering a false paradox halt.
*   **Russell's Paradox (`russell.rs`)**: Computes $R \in R$, dynamically intercepting the unstratified graph prior to call-stack exhaustion via the $K$-Iteration bound.

To execute a diagnostic:
```bash
cargo run -p monist-examples --bin specker_refutation
```

For bare-metal throughput execution tests bypassing the CLI:
```bash
# Run the lock-free OpenCL bridge benchmark
cargo bench -p monist-bench-gpu
```

For an in-depth breakdown of lock-free atomic throughput and comparisons against state-of-the-art runtimes like HVM2/Bend, see the [Reproducible Performance Benchmarks Matrix](docs/BENCHMARKS.md).

## Formal Theory Integration

The mechanical systems defined in this codebase strictly adhere to the formal axioms outlined in the Lean 4 proof architecture. For the formal verification of the Bellman-Ford geometric matrices, see [NF Sketches - AUDIT](https://github.com/ryanncode/nf-sketches/blob/main/AUDIT.md). For detailed instructions on running these proofs and understanding the theoretical bounds of our non-well-founded set implementation, see our [Lean 4 Proofs Repository Integration](docs/PROOFS.md) document.
