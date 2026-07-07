# `monist-seq`

**Sequent Calculus Operations**

`monist-seq` bridges the gap between raw textual syntax and structural constraints. It evaluates sequent calculus mappings, converting human-readable proof tactics (like `intro` or `destruct`) into concrete, cut-free relational structures ready for topological bounding.

## Architecture Pipeline

1. [`monist-parser`](../monist-parser/README.md): Emits the initial raw syntax tree.
2. **`monist-seq`** 📍 *You are here*
3. [`monist-core`](../monist-core/README.md): Receives the sequent maps and converts them into a topographical constraint matrix.

## Navigation
- ⬅️ **Previous:** [`monist-parser`](../monist-parser/README.md)
- ➡️ **Next:** [`monist-core`](../monist-core/README.md)
- 🏠 **Workspace Root:** [Return to `monist` root](../../README.md)
