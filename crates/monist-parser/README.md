# `monist-parser`

**The Syntax Layer**

`monist-parser` is the first entry point into the Monist Engine. It converts raw text, ASCII constraints, and high-level declarative axioms into the engine's internal structural syntax. It strips away complex natural deduction syntax and isolates pure atomic set-theoretic relations (equality and membership) for downstream geometric bounding.

## Architecture Pipeline

1. **`monist-parser`** 📍 *You are here*
2. [`monist-seq`](../monist-seq/README.md): Receives the raw syntax and maps structural logical rules.
3. [`monist-core`](../monist-core/README.md): Receives the structured logic and converts it into a topographical matrix.

## Navigation
- ➡️ **Next:** [`monist-seq`](../monist-seq/README.md)
- 🏠 **Workspace Root:** [Return to `monist` root](../../README.md)
