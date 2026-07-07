# `monist-core`

**The Oracle Layer**

`monist-core` is the central CPU geometry solver of the Monist Engine. It completely destroys Abstract Syntax Trees (ASTs) by converting logical propositions into a directed acyclic graph (DAG) adjacency matrix. 

It executes **Kosaraju's SCC algorithm** to flatten 0-weight semantic equality rings, and then deploys the **Bellman-Ford algorithm** to calculate topological integer bounds. This effectively identifies cyclic self-referential paradoxes (as negative-weight cycles) *before* physical execution, providing structural guarantees for unstratified logic.

## Architecture Pipeline

1. [`monist-seq`](../monist-seq/README.md): Provides the evaluated structural logical maps.
2. **`monist-core`** 📍 *You are here*
3. [`monist-psg`](../monist-psg/README.md): Analyzes the generated topology to establish Phase Space bounds.
4. [`monist-verify`](../monist-verify/README.md): Exports these topological bounds to the Lean 4 formal lab.
5. [`monist-comb`](../monist-comb/README.md): Receives the verified graph and executes it on the GPU.

## Navigation
- ⬅️ **Previous:** [`monist-seq`](../monist-seq/README.md)
- ➡️ **Next:** [`monist-psg`](../monist-psg/README.md) | [`monist-verify`](../monist-verify/README.md) | [`monist-comb`](../monist-comb/README.md)
- 🏠 **Workspace Root:** [Return to `monist` root](../../README.md)
