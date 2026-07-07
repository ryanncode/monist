# `monist-psg`

**Phase Space Geometry Layer**

`monist-psg` computes the spatial boundary check limits derived from the topologies flattened by `monist-core`. By treating evaluated matrices as continuous multidimensional fields, it guarantees that no operation will scale into unallocated phase space memory during WebGPU parallel execution. 

## Architecture Pipeline

1. [`monist-core`](../monist-core/README.md): Provides the flattened matrix and topological constraints.
2. **`monist-psg`** 📍 *You are here*
3. [`monist-verify`](../monist-verify/README.md): Asserts the phase space boundaries before passing the DAG to the physical execution engine.

## Navigation
- ⬅️ **Previous:** [`monist-core`](../monist-core/README.md)
- ➡️ **Next:** [`monist-verify`](../monist-verify/README.md)
- 🏠 **Workspace Root:** [Return to `monist` root](../../README.md)
