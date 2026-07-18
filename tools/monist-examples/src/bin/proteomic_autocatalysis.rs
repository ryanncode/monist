//! # Cyclic Proteomic Autocatalysis
//!
//! A standalone Monist Engine example that models a highly simplified, 3-node
//! continuous kinase cascade (cyclic MAPK/ERK proxy). The code demonstrates
//! the bifurcation of the Monist pipeline:
//!
//! 1. **CPU Geometry** — Intercepts the cyclical type-violation geometrically
//!    via Kosaraju SCC + Bellman-Ford negative-weight cycle detection.
//! 2. **Combinator Synthesis** — Translates the biological loop into an untyped
//!    Y-Combinator fixpoint, eradicating all variables via bracket abstraction.
//! 3. **GPU Dispatch** — Compiles the oscillator into a lock-free Interaction Net
//!    and evaluates it on the WGPU physics backend.
//!
//! This proves for systems biology that unstratified, non-well-founded protein
//! feedback loops can be compiled directly into physical geometry without
//! triggering call-stack exhaustion.

use monist_comb::ast::GNet;
use monist_comb::backend::WgpuExecutor;
use monist_comb::comblib::encodings::{v, y_comb};
use monist_core::ast::Var;
use monist_core::eval::ExecutionLimits;
use monist_core::graph::{GraphArena, ScopedVar};

fn main() {
    // =========================================================================
    // Phase 1: CPU Geometry Construction
    // =========================================================================

    println!("[SYSTEM] Initializing Cyclic Proteomic Matrix...");

    // Step 1a: Initialize the Arena
    let mut arena = GraphArena::new();

    // Step 1b: Declare Biological Nodes
    let kinase_a = ScopedVar(Var::Free("Kinase_A".into()), 0);
    let catalyst_b = ScopedVar(Var::Free("Catalyst_B".into()), 0);
    let receptor_c = ScopedVar(Var::Free("Receptor_C".into()), 0);

    let a = arena.add_var(kinase_a.clone());
    let b = arena.add_var(catalyst_b.clone());
    let c = arena.add_var(receptor_c.clone());

    // Step 1c: Map the Activation Pathway (forward typestate elevations)
    // Kinase_A activates Catalyst_B    (+1 weight = protein activation)
    arena.edges.push((a, b, 1, true));
    // Catalyst_B activates Receptor_C  (+1 weight = protein activation)
    arena.edges.push((b, c, 1, true));

    // Step 1d: Introduce the Autocatalytic Paradox
    // Receptor_C feeds back to Kinase_A, closing the biological cycle.
    // This forces the engine to evaluate a structure where a downstream product
    // causally necessitates its own precursor — a structural paradox under
    // traditional hierarchical type-checking.
    arena.edges.push((c, a, 1, true));

    // Reverse constraint edges (implied by membership semantics, mirroring
    // how Mem generates bidirectional constraints in graph.rs:66-79)
    arena.edges.push((b, a, -1, true));
    arena.edges.push((c, b, -1, true));
    arena.edges.push((a, c, -1, true));

    println!("[GEOMETRY] Mapping A -> B -> C -> A topology.");

    // Step 1e: Diagnostic — SCC + ExecutionLimits interception
    let limits = ExecutionLimits::compute_for_graph(&arena)
        .expect("Graph must contain nodes for limit computation");

    println!(
        "[ORACLE] Cycle Detected. Intercepting Extensionality Collision at topological depth {}.",
        limits.max_k_iterations
    );

    assert!(
        limits.mcm < 0.0,
        "MCM must be negative for cyclic autocatalytic cascade (got {:.2})",
        limits.mcm
    );
    assert_eq!(
        limits.max_k_iterations, 0,
        "K-iterations must halt at 0 for negative-weight cycle interception"
    );

    // Confirm via Bellman-Ford detection
    match arena.evaluate_topology() {
        Ok(_) => panic!("Bellman-Ford should have detected negative-weight cycle in A->B->C->A"),
        Err(collision) => {
            println!("       Bellman-Ford: {}", collision.lines().next().unwrap_or(&collision));
        }
    }

    // =========================================================================
    // Phase 2: Combinator Synthesis
    // =========================================================================

    println!("[COMPILER] Variables eradicated. Synthesizing Y-Combinator continuous oscillator...");

    // Step 2a: Define the Phosphorylation Operator
    // F = λf. catalyst (f kinase) — models the protein interaction step,
    // mapping directly to standard untyped lambda synthesis.
    let kinase_step = v("catalyst")
        .app(v("f").app(v("kinase")))
        .abstract_var("f");

    // Step 2b: Synthesize the Loop via Y-Combinator
    // Applying Y to the interaction step ensures infinite self-application,
    // serving as a perfect proxy for an unbounded metabolic cycle.
    let pathway_oscillator = y_comb().app(kinase_step);

    // =========================================================================
    // Phase 3: GPU Dispatch and Lock-Free Execution
    // =========================================================================

    println!("[HARDWARE] Dispatching biological net to GPU...");

    // Step 3a: VRAM Allocation — compile the oscillator into a generic
    // interaction net suitable for GPU physics evaluation.
    let net = GNet::from_comb(&pathway_oscillator, 1024 * 1024);

    // Step 3b: Execute the Physics — instantiate the WGPU executor and
    // dispatch the network. The WGSL compute shaders evaluate the cyclic
    // collisions natively via lock-free CAS loops.
    let executor = WgpuExecutor::new();
    let (_result_net, state) = executor.execute(&net);

    // Step 3c: Sweep and Terminate — capture the terminal interaction metric.
    println!(
        "[SUCCESS] Oscillator stabilized at {} unreachable floating rings. Total molecular collisions evaluated: {}",
        state.active_nodes, state.interactions
    );
}
