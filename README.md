# <div align="center">The Monist Engine</div>

<div align="center">
  <br>
  <a href="https://ryanncode.github.io/monist/"><strong>📚 Read the Docs</strong></a>
  &nbsp;&nbsp;&nbsp;&nbsp; | &nbsp;&nbsp;&nbsp;&nbsp;
  <a href="https://firstsynth.dev/"><strong>🌐 First Synth</strong></a>
  &nbsp;&nbsp;&nbsp;&nbsp; | &nbsp;&nbsp;&nbsp;&nbsp;
  <a href="https://firstsynth.dev/console/"><strong>🖥️ Web Console</strong></a>
  <br><br>
</div>

> **TL;DR:** A bare-metal, GPU-accelerated logic engine that safely evaluates self-referential paradoxes and cyclic graphs without crashing. It serves as a deterministic topological foundry for transfinite combinatorial computing, quantum-logical physics simulations, custom hardware synthesis, and formal AI verification.

_For the mathematically verified Lean 4 companion project, see the [NF Sketches](https://github.com/ryanncode/nf-sketches) repository._

This logic engine is a high-performance evaluator and bare-metal compilation pipeline designed exclusively to compute Quine's New Foundations (NF). By abandoning traditional hierarchical type-checking in favor of geometric shortest-path routing (Bellman-Ford), the project successfully compiles unstratified, self-referential paradoxes into executable Interaction Nets.

It mathematically computes the precise topological weight of systemic ambiguity, catching extensionality collisions before offloading purely untyped combinator nodes to the GPU.

---

## Transfinite Computing & Speculative Frontiers

The standard computing industry enforces memory safety via strict Directed Acyclic Graphs (DAGs)—evidenced by Rust's borrow checker or Lean's dependent type hierarchy. This limits computation to well-founded, hierarchical structures. The Monist Engine abandons this completely, providing a deterministic execution layer capable of safely evaluating infinitely recursive or cyclical topological structures natively on the GPU.

This unlocks a highly speculative frontier where abstract combinatorial rewriting meets physical constraints, enabling:

- **Transfinite Combinatorial Computing:** Native computation of transfinite cardinals without bottlenecking on the Axiom of Choice, utilizing Ramsey-theoretic bounds to calculate fixed points in self-referential systems.
- **Hardware-Logic Co-Evolution:** Synthesizing custom Interaction Net FPGAs where logical reduction is performed via physical signal collisions, transforming processor spatial routing into execution logic without fetch-execute cycles.
- **Quantum-Topos Logic Synthesis:** Treating logic as a geometric object to model quantum vacuums as "saturated computational boundaries," calculating measurable energy shifts based on the algorithmic friction of re-leveling variables.
- **Scalable AI Oversight:** Providing mathematical guarantees over probabilistic black-box models. Organizations can use Monist as a deterministic bedrock to audit, govern, and formally verify neural network outputs, checking for hallucination via structural self-verification.

---

## Quickstart: The Paradox Engine

Traditional type systems crash when fed cyclic self-reference. Monist executes it natively.

```bash
# 1. A standard proposition is evaluated normally
$ cargo run -p monist-cli -- eval "forall x. x = x"
> Stratification successful.

# 2. Evaluating the Universal Set (V in V)
$ cargo run -p monist-cli -- eval "V in V"
> Stratification successful.
> Neutralized SC defining self-loop on V_0

# 3. Evaluating Russell's Paradox
$ cargo run -p monist-cli -- eval "{x | ~(x in x)} in {x | ~(x in x)}"
> Error: Extensionality Collision: Negative-weight cycle detected (μ* = -1.0000)!
> Engine halted safely (K_ITERATION_HALT)
> Topological Trace: b0_1 -> b0_1 (-1) = -1
```

---

## Python Bindings (`monist-python`)

To integrate Monist topological bounding directly into machine learning pipelines (such as PyTorch, JAX, or LLM inference loops), we provide zero-cost PyO3 bindings via the `monist-python` package. This unlocks **Semantic Self-Verification (SSV)**, allowing LLMs to rapidly query the Monist Oracle in parallel, instantly checking for paradoxical loops, hallucinated cycles, or geometric extensionality violations in $O(1)$ time.

```python
import monist_engine as monist

# Initialize the topological bounds checker
oracle = monist.Engine(enable_t_functor=True)

# Evaluate an Extensionality Collision (Russell's Paradox)
res = oracle.evaluate("{x | ~(x in x)} in {x | ~(x in x)}")
print(res.is_stratified) # False
print(res.collision_weight) # -1.0
```

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

## Capabilities & Architecture

If you attempt to feed a structurally dense, cyclic self-reference like the Universal Set ($V \in V$) into standard frameworks, they immediately crash, resulting in either a syntax error or a VRAM exhaustion (warp divergence).

The Monist Engine subverts this by bifurcating the computational stack:

1. **The Oracle Layer (CPU/Geometry):** The frontend parser transforms standard first-order logic into an algebraic system of constraints. Instead of rejecting cyclic graphs, the `monist-core` executes Tarjan's SCC algorithm to single-pass flatten 0-weight semantic cycles, and deploys the Bellman-Ford algorithm with Karp's Minimum Cycle Mean (MCM) integration to explicitly map the thermodynamic weight and path sequences between nodes. If a paradox forms, the engine mathematically calculates Extensionality Collisions, dynamically bounding the execution via K-Iteration limits.
2. **The T-Functor Synthesis**: When dense impredicative recursion forms, the engine dynamically synthesizes and injects a `T-operator` ($x \mapsto \iota"x$), acting as a topological stabilizer that absorbs the structural friction, preserving the weak stratification boundary.
3. **Variable Annihilation and Lock-Free Atomics:** Traditional software engineering binds logic to named variables. Monist eliminates this overhead entirely. The `monist-comb` compiler destroys alphanumeric variables, replacing them with nameless de Bruijn levels, `Susp` lazy thunks (Okasaki), and 2-Symmetric Interaction Combinator (2-SIC) topological rewrite rules. GPU threads evaluate Annihilations and Commutations natively utilizing Interaction Monoid structure and Girard's Execution Formula to bypass cycle exhaustion.
4. **Holographic Fast-Failing Oracle:** The runtime natively supports Vector Symbolic Architectures (VSA/HDC) to embed discrete graph logic into a continuous $10,000$-dimensional phase space. Enterprise systems (such as legacy SIEMs or computational biology pipelines) can use the Monist Engine as a massively parallel data sieve. By utilizing $O(1)$ destructive interference and a GPU-accelerated Successive Interference Cancellation (SIC) bridge, it violently drops valid data and snaps unresolvable anomalies back into discrete variables in milliseconds, shielding exact traditional databases from combinatorial explosion.
5. **Topological Recursion Cost:** When the GPU executes a logic network, it outputs a precise, reproducible integer: the exact count of graph-rewrite collisions required to normalize a feedback topology into a stable fixpoint. This deterministic observable functions as a discrete topological analogue of free energy expenditure, providing an exact, distribution-free count of erasures versus rearrangements.

---

## Benchmarks & Mathematical Diagnostics

The engine's validity is proven via a suite of automated diagnostic refutations located in `tools/monist-examples/src/bin/`. These execute the core paradoxes of modern set theory, outputting the mathematically verified topological boundaries and generating standard `SMT-LIB` witnesses for third-party prover ingestion.

### SMT-LIB Differential to Lean Pipeline

We have built a fully synchronized Differential Equivalence Testing pipeline linking the Rust implementation to our Lean formalization ([nf-sketches/parse-strat](https://github.com/ryanncode/nf-sketches/tree/main/parse-strat)). When a `monist-examples` mathematical test executes, the `monist-core` engine seamlessly generates an SMT-LIB witness of the topological graph, explicitly capturing evaluation limits, bounds, and Extensionality Collisions.

This witness can then be natively piped into the Lean interpreter (`lake exe parse-strat --ingest-smt`), which uses its own completely independent topological Bellman-Ford implementation to trace the exact same SMT constraints. By proving 1-to-1 equivalence between Lean and Rust, we guarantee that the engine handles paradoxical scopes and Comprehension boundaries flawlessly.

- **`scripts/run_differential_tests.sh`**: Iterates through all automated mathematical refutations, extracting SMT-LIB blocks and piping them seamlessly into the Lean `parse-strat` interpreter.
- **Comprehension Bounds**: The engine uses the `in_comp` topological boundary flag to distinguish between unstratifiable Comprehensions (like Russell's Paradox) which trigger `Extensionality Collision`, and raw unstratified logical queries which are mathematically neutralized and safely evaluated via the SC-Bedrock daemon.

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

- **Specker's Refutation of Global Choice (`specker_refutation.rs`)**: Mechanically proves that bridging disjoint integer weight elevations (${ \Phi(m) }$ vs ${ \Phi(T(m)) }$) without a $T$-operator creates a negative-weight cycle, validating the absolute halting limit.
- **The Extensionality Collision (`extensionality_collision.rs`)**: Evaluates the Kuratowski ordered pair vs the Quine ordered pair, proving the engine tracks dense structural depth offsets (+2 vs 0) without triggering a false paradox halt.
- **Russell's Paradox (`russell.rs`)**: Computes $R \in R$, dynamically intercepting the unstratified graph prior to call-stack exhaustion via the $K$-Iteration bound.

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

---

## API Documentation

For a complete technical breakdown of the Rust workspace and core engine functions, see the [Monist Core API Specs](https://ryanncode.github.io/monist/specs/monist_core/).

## Repository Structure

To help orient yourself within the codebase, here is how the architecture maps to the crates:

- `crates/monist-parser` - **The Syntax Layer**: Parses raw text and ASCII constraints into standard logical syntax.
- `crates/monist-core` - **The Oracle Layer**: The frontend AST, constraint algebra, and the CPU-bound geometric routing algorithms (Kosaraju SCC & Bellman-Ford) that detect extensionality collisions.
- `crates/monist-comb` - **The Interaction Net Backend**: The purely untyped combinator nodes ($S, K, I$) and the WebGPU (`wgpu`) compute shaders that evaluate the logic dynamically.
- `crates/monist-seq` - **Sequent Operations**: Sequent calculus evaluation mapping structural rules to combinatory embeddings.
- `crates/monist-category` - **Categorical Structures**: The T-functor and relative adjunction definitions.
- `crates/monist-psg` - **Phase Space Geometry**: Data structures for topological graphing and boundary checks.
- `crates/monist-verify` - **Verification Layer**: Validates differential equivalence and Bellman-Ford limits prior to evaluation.
- `crates/monist-macros` - **Engine Macros**: Syntactic procedural macros for seamless Rust testing and syntax.
- `bindings/monist-python` - **The Python ML Bridge**: Zero-cost PyO3 bindings for direct integration into PyTorch, JAX, and Neuro-symbolic LLM verification loops.
- `tools/monist-cli` - **The Interactive REPL**: The tactical theorem-proving interface and diagnostic visualizer.
- `tools/monist-examples` - **Mathematical Diagnostics**: The automated mathematical refutations that prove the engine's theoretical boundaries.

---

## License

This project is licensed under the GNU Affero General Public License v3 (AGPLv3) - see the [LICENSE](LICENSE) file for details.
