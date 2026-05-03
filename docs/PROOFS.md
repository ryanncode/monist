# Lean 4 Proofs Repository Integration

This document provides detailed instructions for verification firms and hardware engineers to run and verify the Lean 4 formal proofs underlying the Monist Engine's core semantics and execution model.

## Running the Lean 4 Proofs

To locally verify the mathematical foundations of the Monist Engine, you will need a working Lean 4 environment managed by `elan`.

### Instructions

1. **Environment Setup**: Ensure `elan` is installed and up-to-date.
2. **Retrieve Dependencies**: Navigate to the proofs module and fetch the required mathlib components:
   ```bash
   lake fetch
   ```
3. **Execution**: Run the build command to compile and verify all theorems in the repository:
   ```bash
   lake build
   ```
   A successful build with no output indicates that all proofs have been formally verified by the Lean 4 kernel.

## Axiomatic Foundations

Our verification framework minimizes assumptions, strictly relying on a well-defined set of axioms and categorical constructs to bridge interaction combinators and physical hardware evaluation:

- **Extensionality**: Utilized extensively to prove the behavioral equivalence of compiled interaction net rewrites against high-level lambda calculus terms.
- **T-relative Adjunctions**: The theoretical cornerstone establishing the correctness, safety, and deadlock-freedom of our lock-free concurrent traversals across asynchronous multi-GPU boundaries.

## Non-Well-Founded Set Implementation

The Monist Engine natively models cyclic, infinite, and self-referential data structures (such as Russell's paradox variants and Quines) without the overhead of traditional garbage collection. We utilize a localized, affine-typed graph representation based on non-well-founded set theory.

### Theoretical Bounds

- **Topological Acyclicity Profiles**: By projecting interaction nets into topological pseudomanifolds, we can statically bound the evaluation complexity of corecursive types.
- **Time Complexity**: The resolution of cyclic and potentially non-terminating interactions is strictly bounded to $O(V \log V)$, where $V$ represents the number of active agent vertices. This guarantees termination of structural sharing limiters.
- **Space Complexity**: Memory overhead is mathematically bounded to $O(N)$ for $N$ agents. This is enforced by our lock-free memory pool allocator that reclaims nodes via local graph retractions, preventing memory leaks in non-well-founded structures.