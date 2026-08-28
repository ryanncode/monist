# <div align="center">The Monist Engine</div>

_A bare-metal, GPU-accelerated logic engine utilizing `O(V+E)` topological bounds checking for lock-free interaction net reduction._

<div align="center">
  <br>
  <a href="https://ryanncode.github.io/monist/"><strong>Read the Docs</strong></a>
  &nbsp;&nbsp;&nbsp;&nbsp; | &nbsp;&nbsp;&nbsp;&nbsp;
  <a href="https://firstsynth.dev/"><strong>First Synth</strong></a>
  &nbsp;&nbsp;&nbsp;&nbsp; | &nbsp;&nbsp;&nbsp;&nbsp;
  <a href="https://firstsynth.dev/console/"><strong>Web Console</strong></a>
  <br><br>
</div>

---

## Architecture & Syntactic Monism

The Monist Engine is a deterministic logic evaluator and hardware compilation pipeline designed for non-well-founded set theories, specifically Quine's New Foundations (NF).

Mainstream type systems and formal proof assistants (ZFC, Homotopy Type Theory, Coq, Lean) enforce logical consistency through infinite hierarchical universe towers (`U0:U1:U2...`) and acyclic data dependencies. These invariants forbid self-membership, rejecting self-referential graph structures and cyclic feedback networks at the compilation boundary.

Monist implements **Syntactic Monism**, shifting consistency enforcement entirely from ontological hierarchies onto graph topology:

* **Topological Stratification**: The engine translates first-order propositions into difference-constraint matrices. Algorithms for shortest-path routing (Bellman-Ford) and cycle means (Karp MCM) detect logical paradoxes as negative-weight cycles in `O(V+E)` time.
* **Native Cyclic Graphs**: Propositions containing cyclic self-reference, including the Universal Set (`V ∈ V`), evaluate without infinite recursion or stack exhaustion.
* **GPU Interaction Combinators**: Validated logic graphs compile into variable-free 2-Symmetric Interaction Combinators (2-SIC), executing as parallel graph rewrites across lock-free GPU compute shaders.

**The Dual-Verification Pipeline:** Monist pairs bare-metal execution with formal mathematical verification through a synchronized differential testing pipeline with its companion lab in Lean 4 ([NF Sketches](https://github.com/ryanncode/nf-sketches)). The Rust solver exports SMT-LIB witnesses capturing difference constraints, which the Lean 4 kernel independently ingests and checks against formal soundness proofs.

---

## Setup and Building

This repository is organized as a unified Cargo workspace containing all core libraries, WebGPU shaders, Python bindings, and CLI tools.

### Quick Build & Test

To build the entire workspace and run the full unit and integration test suite:

```bash
# Build all crates, CLI binaries, and examples
cargo build --release

# Execute test suite across all crates
cargo test --workspace
```

### The Interactive REPL

Launch the tactical theorem-proving environment:

```bash
cargo run -p monist-cli -- repl
```

Inside the REPL, you can declare axioms (`assume`), set proof goals (`theorem`), and deploy 19 interactive tactics (`intro`, `destruct`, `rw`, `simp`, `elevate`, `collapse_loop`, `sc_cut`). For full details on tactic syntax and proof management, see the [CLI User Guide](docs/02-cli-guide.qmd).

### Hello World: Your First Evaluation

Evaluate foundational propositions and paradoxical self-references directly from the command line:

```bash
# 1. Standard tautology evaluates and stratifies
cargo run -p monist-cli -- eval "forall x. x = x"

cargo run -p monist-cli -- eval "V in V"

# 3. Russell's Paradox triggers an Extensionality Collision and halts safely
cargo run -p monist-cli -- eval "{x | ~(x in x)} in {x | ~(x in x)}"
```

### Python Machine Learning Bridge (`monist-python`)

For **Semantic Self-Verification (SSV)**, neural networks and neuro-symbolic agent loops query the Monist Oracle via zero-cost PyO3 bindings to audit reasoning graphs and intercept cyclic hallucinations:

```python
import monist_engine as monist

# Initialize the topological bounds engine
oracle = monist.Engine(enable_t_functor=True)

# Evaluate an Extensionality Collision (Russell's Paradox)
res = oracle.evaluate("{x | ~(x in x)} in {x | ~(x in x)}")
print(res.is_stratified)     # False
print(res.collision_weight)  # -1.0
```

---

## Core Crates

The workspace partitions execution across dedicated crates, separating logical syntax from CPU constraint geometry, GPU shaders, formal verification, and developer tooling.

### 1. `monist-core`: The Oracle Layer
The CPU-bound geometry solver. It translates formulas into algebraic constraint matrices, executes Kosaraju and Tarjan SCC algorithms to single-pass flatten 0-weight semantic equality rings, and deploys Bellman-Ford traversal with Karp's Minimum Cycle Mean (MCM) integration to detect negative-weight paradox loops.

### 2. `monist-comb`: The Interaction Net Backend
An untyped combinator execution environment (`S, K, I`). It compiles validated logic graphs into nameless de Bruijn combinators bounded by Okasaki `Susp` structures, evaluating 2-Symmetric Interaction Combinators (2-SIC) natively through lock-free WebGPU WGSL compute shaders.

### 3. `monist-seq`: Sequent Calculus & ITP Engine
Implements an interactive natural deduction engine and sequent calculus evaluator. It features a complete 19-tactic ecosystem (including `intro`, `destruct`, `rw`, `simp`, `elevate`, `collapse_loop`, `sc_cut`) driving automated goal management and proof session serialization.

### 4. `monist-verify`: The Verification Gateway
Manages cross-language differential equivalence testing against the Lean 4 formal lab. It automatically extracts SMT-LIB stratification witnesses for exported graph topologies, enabling independent mathematical audit of runtime execution.

### 5. `monist-wasm` & `monist-console`: WebAssembly & Interactive Infoview
Compiles the topological solver and proof session to WebAssembly, powering the in-browser interactive Infoview console with real-time syntax tokenization, interactive hypothesis chips, and multi-goal proof tabs.

### 6. `monist-python`: Neuro-Symbolic ML Bridge
Provides zero-cost PyO3 bindings for Python pipelines (PyTorch, JAX, LLMs). This enables Semantic Self-Verification (SSV), allowing neural network inference loops to query the Monist Oracle in `O(1)` time to intercept hallucinations and cyclic reasoning loops.

### 7. `monist-examples` & `monist-cli`: Tooling & Mathematical Diagnostics
Hosts the interactive command-line interface alongside automated diagnostic suites executing canonical set-theoretic paradoxes (Specker's Refutation of Global Choice, Extensionality Collisions, and Choice-Free Transfinite Arithmetic).

---

## Documentation & Formal Theory

Detailed technical guides, benchmarks, and mathematical foundations are available in [`docs/`](docs/):

* [System Architecture & Compilation Pipeline](docs/01-architecture.qmd)
* [CLI User Guide & Tactic Cheatsheet](docs/02-cli-guide.qmd)
* [Building Primitives & Canonical Examples](docs/03-examples-and-primitives.qmd)
* [Interaction Nets & GPU Compute Engine](docs/04-interaction-net.qmd)
* [Holographic Co-Processor & Continuous VSA](docs/05-holographic.qmd)
* [Performance Benchmarks Matrix](docs/06-benchmarks.qmd)
* [Lean 4 Formal Verification Integration](docs/07-proofs.qmd)
* [Advanced Synthetic Applications & Frontiers](docs/08-advanced-synthetic-applications.qmd)
* [Theoretical Foundations](docs/09-theoretical-foundations.qmd)
* [Mathematical Philosophy](docs/10-mathematical-philosophy.qmd)

---

## License

This project is licensed under the GNU Affero General Public License v3 (AGPLv3) - see the [LICENSE](LICENSE) file for details.
