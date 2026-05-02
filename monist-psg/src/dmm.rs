/// Represents a closure or lifetime identifier used in DMM.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LifetimeId(pub usize);

/// Deterministic Memory Management (DMM) as a coeffect discipline.
/// Replaces traditional binary escape analysis with a discriminated union
/// describing exact memory allocation locations and bounded lifetimes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryCoeffect {
    /// Allocated to the stack (`memref.alloca`), strictly bounded by lexical scope.
    StackScoped,
    /// Arena allocated, bound directly to the lifetime of closure `t`.
    ClosureCapture(LifetimeId),
    /// Arena allocated in the caller's scope, allowing values to be returned.
    ReturnEscape,
    /// Arena allocated in the parameter's origin scope, escaping via a reference.
    ByRefEscape,
}

/// Represents the memory management interaction model used by the developer/compiler.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DmmModel {
    /// Explicit target allocation defined by the developer.
    Push,
    /// Scoped inference to bound lifetimes automatically.
    Bounded,
    /// Full usage inference across the entire program.
    Poll,
}

/// An environment mapping variables or nodes to their inferred memory coeffects.
#[derive(Debug, Default)]
pub struct DmmEnvironment {
    allocations: std::collections::HashMap<usize, MemoryCoeffect>,
    model: DmmModel,
}

impl DmmEnvironment {
    pub fn new(model: DmmModel) -> Self {
        Self {
            allocations: std::collections::HashMap::new(),
            model,
        }
    }

    /// Assigns a memory coeffect to a specific node/variable ID.
    pub fn assign_coeffect(&mut self, id: usize, coeffect: MemoryCoeffect) {
        self.allocations.insert(id, coeffect);
    }

    /// Retrieves the memory coeffect for a specific ID, if tracked.
    pub fn get_coeffect(&self, id: usize) -> Option<&MemoryCoeffect> {
        self.allocations.get(&id)
    }

    /// Retrieves the current DMM interaction model.
    pub fn get_model(&self) -> &DmmModel {
        &self.model
    }
}

impl Default for DmmModel {
    fn default() -> Self {
        Self::Bounded
    }
}
