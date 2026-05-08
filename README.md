# <div align="center">The Monist Engine</div>

<div align="center">
  <br>
  <a href="https://ryanncode.github.io/monist/"><strong>📚 Read the Docs</strong></a>
  &nbsp;&nbsp;&nbsp;&nbsp; | &nbsp;&nbsp;&nbsp;&nbsp;
  <a href="https://firstsynth.dev/"><strong>🌐 First Synth</strong></a>
  &nbsp;&nbsp;&nbsp;&nbsp; | &nbsp;&nbsp;&nbsp;&nbsp;
  <a href="https://ryanncode.github.io/monist/specs/monist_core/"><strong>⚙️ API Specs</strong></a>
  <br><br>
</div>

This logic engine is a high-performance evaluator and bare-metal compilation pipeline designed exclusively to compute Quine's New Foundations (NF). By abandoning traditional hierarchical type-checking in favor of geometric shortest-path routing (Bellman-Ford), the project successfully compiles unstratified, self-referential paradoxes into executable Interaction Nets. 

It mathematically computes the precise topological weight of systemic ambiguity, catching extensionality collisions before offloading purely untyped combinator nodes to the GPU.

---

## Scalable Oversight & Neuro-Symbolic Integration

The frontier of machine learning is overwhelmingly probabilistic. Neural architectures generate outputs that resist strict, structural verification. The Monist Engine provides the exact opposite: a deterministic execution layer capable of safely evaluating infinitely recursive or cyclical structures. 

As the industry shifts its focus from experimental models toward scalable oversight, organizations require mathematical guarantees over black-box systems. The Monist Engine serves as the deterministic bedrock necessary to audit, govern, and formally verify probabilistic agents. Through Semantic Self-Verification (SSV) and active retrieve-generate-verify feedback loops, it structurally grounds LLMs to solve high-difficulty mathematical abstractions without hallucinatory semantic drift.

---

## Capabilities & Architecture

The standard computing industry enforces memory safety via strict Directed Acyclic Graphs (DAGs)—evidenced by Rust's borrow checker or Lean's dependent type hierarchy. If you attempt to feed a structurally dense, cyclic self-reference like the Universal Set ($V \in V$) into standard frameworks, they immediately crash, resulting in either a syntax error or a VRAM exhaustion (warp divergence).

The Monist Engine subverts this by bifurcating the computational stack:

1. **The Oracle Layer (CPU/Geometry):** The frontend parser transforms standard first-order logic into an algebraic system of constraints. Instead of rejecting cyclic graphs, the `monist-core` executes Kosaraju's SCC algorithm to flatten 0-weight semantic cycles, and deploys the Bellman-Ford algorithm to map the thermodynamic weight between nodes. If a paradox forms, the engine dynamically calculates the $K$-Iteration depth boundary and intercepts the recursion mathematically.
2. **The T-Functor Synthesis:** When dense impredicative recursion forms (like the Burali-Forti sequence), the engine dynamically synthesizes and injects a `T-operator` ($x \mapsto \iota"x$), acting as a topological stabilizer that absorbs the structural friction, preserving the weak stratification boundary.
3. **The Interaction Net Backend (GPU/Physics):** Stripped of all types and hierarchical scaffolding, the validated logic is compiled into pure $S, K, I$ combinators (`monist-comb`). We bypass traditional substitution environments, feeding these localized atomic nodes directly to autonomous WGSL compute shaders. Operating entirely in WGPU without CPU synchronization overhead, the hardware natively folds self-reference topologically, scaling across millions of lock-free collisions per second.
4. **Holographic Fast-Failing Oracle:** The runtime natively supports Vector Symbolic Architectures (VSA/HDC) to embed discrete graph logic into a continuous $10,000$-dimensional phase space. Enterprise systems (such as legacy SIEMs or computational biology pipelines) can use the Monist Engine as a massively parallel data sieve. By utilizing $O(1)$ destructive interference and a GPU-accelerated Successive Interference Cancellation (SIC) bridge, it violently drops valid data and snaps unresolvable anomalies back into discrete variables in milliseconds, shielding exact traditional databases from combinatorial explosion.

---

## Build & Run

Ensure you have Rust and Cargo installed, alongside a WGPU-compatible graphics backend (Vulkan, Metal, DirectX 12, or WebGPU) for executing the engine natively on the GPU.

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

Inside the REPL, you can `assume Extensionality`, run backwards-reasoning tactics (`intro`, `apply`, `destruct`), or run a live, color-coded diagnostic visualizer using `step "x in x"`. For full details on available commands, axioms, and visualization options, see the [CLI Guide](docs/02-cli-guide.qmd).

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

### SMT-LIB Differential to Lean Pipeline

We have built a fully synchronized Differential Equivalence Testing pipeline linking the Rust implementation to our Lean formalization ([nf-sketches/parse-strat](https://github.com/ryanncode/nf-sketches/tree/main/parse-strat)). When a `monist-examples` mathematical test executes, the `monist-core` engine seamlessly generates an SMT-LIB v2 witness of the topological graph, explicitly capturing evaluation limits, bounds, and Extensionality Collisions. 

This witness can then be natively piped into the Lean interpreter (`lake exe parse-strat --ingest-smt`), which uses its own completely independent topological Bellman-Ford implementation to trace the exact same SMT constraints. By proving 1-to-1 equivalence between Lean and Rust, we guarantee that the engine handles paradoxical scopes and Comprehension boundaries flawlessly.

* **`scripts/run_differential_tests.sh`**: Iterates through all automated mathematical refutations, extracting SMT-LIB blocks and piping them seamlessly into the Lean `parse-strat` interpreter.
* **Comprehension Bounds**: The engine uses the `in_comp` topological boundary flag to distinguish between unstratifiable Comprehensions (like Russell's Paradox) which trigger `Extensionality Collision`, and raw unstratified logical queries which are mathematically neutralized and safely evaluated via the SC-Bedrock daemon.

To manually pipe a diagnostic test directly into the Lean 4 interpreter:

```bash
cd tools/monist-examples
cargo run --bin specker_refutation | awk '/; === BEGIN STRATIFICATION WITNESS ===/{flag=1; print; next} /; === END STRATIFICATION WITNESS ===/{print; flag=0} flag' > out.smt
cd ../../../nf-sketches/parse-strat
lake exe parse-strat --ingest-smt < ../../monist/tools/monist-examples/out.smt
```

Or run the full automated cross-language verification script from the root:

```bash
./scripts/run_differential_tests.sh
```

### Mathematical Diagnostics

* **Specker's Refutation of Global Choice (`specker_refutation.rs`)**: Mechanically proves that bridging disjoint integer weight elevations (${ \Phi(m) }$ vs ${ \Phi(T(m)) }$) without a $T$-operator creates a negative-weight cycle, validating the absolute halting limit.
* **The Extensionality Collision (`extensionality_collision.rs`)**: Evaluates the Kuratowski ordered pair vs the Quine ordered pair, proving the engine tracks dense structural depth offsets (+2 vs 0) without triggering a false paradox halt.
* **Russell's Paradox (`russell.rs`)**: Computes $R \in R$, dynamically intercepting the unstratified graph prior to call-stack exhaustion via the $K$-Iteration bound.

To execute a diagnostic:

```bash
cargo run -p monist-examples --bin specker_refutation
```

For bare-metal throughput execution tests bypassing the CLI:

```bash
# Run the lock-free OpenCL and discrete GPU bounds benchmarks
cargo bench -p monist-bench-gpu
```

For an in-depth breakdown of lock-free atomic throughput and discrete execution bounds, see the [Reproducible Performance Benchmarks Matrix](docs/06-benchmarks.qmd).

## Formal Theory Integration

The mechanical systems defined in this codebase strictly adhere to the formal axioms outlined in the Lean proof architecture. For the formal verification of the Bellman-Ford geometric matrices, see [NF Sketches - AUDIT](https://github.com/ryanncode/nf-sketches/blob/main/AUDIT.md). For detailed instructions on running these proofs and understanding the theoretical bounds of our non-well-founded set implementation, see our [Lean Proofs Repository Integration](docs/07-proofs.qmd) document. To dive deeper into the theoretical origins of this architecture, see [Theoretical Foundations](docs/09-theoretical-foundations.qmd) and [Mathematical Philosophy](docs/10-mathematical-philosophy.qmd).
