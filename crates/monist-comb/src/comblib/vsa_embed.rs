use rand_distr::{Normal, Distribution};

pub const D: usize = 10000;

/// A High-Dimensional Computing (HDC) Vector used in the Holographic Co-processor.
/// Provides base operations like superposition and binding in $D$-dimensional continuous space.
#[derive(Debug, Clone)]
pub struct HDCVector {
    pub values: Vec<f32>,
}

impl HDCVector {
    pub fn new() -> Self {
        Self { values: vec![0.0; D] }
    }

    /// Generates a random basis vector from a normal distribution N(0, 1/D)
    pub fn random_basis() -> Self {
        let mut rng = rand::rng();
        let normal = Normal::new(0.0, 1.0 / (D as f32).sqrt()).unwrap();
        
        let mut values = Vec::with_capacity(D);
        for _ in 0..D {
            values.push(normal.sample(&mut rng));
        }
        Self { values }
    }

    /// Superposition: Pointwise addition
    pub fn superpose(&self, other: &Self) -> Self {
        let mut result = Vec::with_capacity(D);
        for (a, b) in self.values.iter().zip(other.values.iter()) {
            result.push(a + b);
        }
        Self { values: result }
    }

    /// Binding: Circular convolution (implemented naively or via FFT; naively for now)
    pub fn bind(&self, other: &Self) -> Self {
        let mut result = vec![0.0; D];
        for i in 0..D {
            for j in 0..D {
                result[i] += self.values[j] * other.values[(i + D - j) % D];
            }
        }
        Self { values: result }
    }

    /// Cyclic permutation ($\Pi$) to represent +1 weight edges (Membership)
    pub fn permute(&self, shift: usize) -> Self {
        let mut result = vec![0.0; D];
        for i in 0..D {
            result[i] = self.values[(i + D - (shift % D)) % D];
        }
        Self { values: result }
    }

    /// Instant-Time Negative Phase Cancellation (HRR Subtractor)
    /// Subtracts the target vector point-wise from the current superposed state.
    pub fn holographic_exclusion_query(&self, target: &Self) -> Self {
        let mut result = Vec::with_capacity(D);
        for (a, b) in self.values.iter().zip(target.values.iter()) {
            result.push(a - b);
        }
        Self { values: result }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.values.iter().zip(other.values.iter()).map(|(a, b)| a * b).sum()
    }
}

/// bind_nodes()
/// Takes two vectors (representing nodes) and a weight (0 or 1).
/// If weight is 0 (Equality), returns self bound to other.
/// If weight is 1 (Membership), returns self bound to permutation of other.
pub fn bind_nodes(node_a: &HDCVector, node_b: &HDCVector, weight: i32) -> HDCVector {
    if weight == 0 {
        node_a.bind(node_b)
    } else if weight == 1 {
        // Membership: shift by 1 to represent hierarchical structure
        node_a.bind(&node_b.permute(1))
    } else {
        // Inverse or other weights
        node_a.bind(&node_b.permute(weight.rem_euclid(D as i32) as usize))
    }
}

use std::collections::HashMap;

/// Successive Interference Cancellation (SIC) Recovery Bridge
/// Associative Memory Codebook for recovering discrete combinators from superposed vector fields.
pub struct Codebook {
    pub vectors: HashMap<String, HDCVector>,
}

impl Codebook {
    pub fn new() -> Self {
        Self {
            vectors: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, vec: HDCVector) {
        self.vectors.insert(key, vec);
    }

    /// Receives the noisy superposed vector and extracts bound nodes via SIC
    pub fn recover_discrete_combinators(&self, mut superposed: HDCVector, threshold: f32) -> Vec<String> {
        let mut recovered = Vec::new();
        
        loop {
            let mut best_match = None;
            let mut highest_sim = threshold;

            // Identify nearest neighbor using normalized dot product
            for (key, vec) in &self.vectors {
                // The expected dot product of a basis vector with itself is 1.0
                // Since the variance of components is 1/D.
                let sim = superposed.dot(vec);
                if sim > highest_sim {
                    highest_sim = sim;
                    best_match = Some((key.clone(), vec.clone()));
                }
            }

            if let Some((key, vec)) = best_match {
                recovered.push(key);
                // SIC: Subtract its signal from the superposed vector
                superposed = superposed.holographic_exclusion_query(&vec);
            } else {
                // Signal-to-noise ratio dropped below threshold
                break;
            }
        }

        recovered
    }

    /// Integrate IDL Conflict Clauses
    /// Translates conflict clauses (negative-weight cycles from Bellman-Ford)
    /// into destructive interference masks to aggressively remove them from storage.
    pub fn apply_conflict_mask(&mut self, conflict_clause_nodes: &[String]) {
        let mut mask = HDCVector::new();
        
        // Build the interference mask by superposing the offending nodes
        for node in conflict_clause_nodes {
            if let Some(vec) = self.vectors.get(node) {
                mask = mask.superpose(vec);
            }
        }

        // Apply inverse phase operation against the primary holographic storage array
        // (For the associative memory codebook, we effectively nullify these nodes)
        for node in conflict_clause_nodes {
            self.vectors.remove(node);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vsa_embedding_and_sic_recovery() {
        let node_a = HDCVector::random_basis();
        let node_b = HDCVector::random_basis();
        let node_c = HDCVector::random_basis();

        let mut codebook = Codebook::new();
        codebook.insert("NodeA".to_string(), node_a.clone());
        codebook.insert("NodeB".to_string(), node_b.clone());
        codebook.insert("NodeC".to_string(), node_c.clone());

        // Superpose A and B
        let superposed = node_a.superpose(&node_b);

        // Recover with SIC
        // The threshold is slightly below 1.0 (approx normal dot self is 1.0)
        let recovered = codebook.recover_discrete_combinators(superposed, 0.4);

        assert!(recovered.contains(&"NodeA".to_string()));
        assert!(recovered.contains(&"NodeB".to_string()));
        assert!(!recovered.contains(&"NodeC".to_string()));
    }

    #[test]
    fn test_holographic_exclusion() {
        let node_a = HDCVector::random_basis();
        let node_b = HDCVector::random_basis();

        let superposed = node_a.superpose(&node_b);

        // Exclude A
        let excluded = superposed.holographic_exclusion_query(&node_a);

        let mut codebook = Codebook::new();
        codebook.insert("NodeA".to_string(), node_a.clone());
        codebook.insert("NodeB".to_string(), node_b.clone());

        let recovered = codebook.recover_discrete_combinators(excluded, 0.4);

        assert!(!recovered.contains(&"NodeA".to_string()));
        assert!(recovered.contains(&"NodeB".to_string()));
    }

    #[test]
    fn test_conflict_clause_masking() {
        let node_x = HDCVector::random_basis();
        let mut codebook = Codebook::new();
        codebook.insert("ParadoxicalNode".to_string(), node_x.clone());
        
        let conflict_nodes = vec!["ParadoxicalNode".to_string()];
        codebook.apply_conflict_mask(&conflict_nodes);
        
        assert!(!codebook.vectors.contains_key("ParadoxicalNode"));
    }
}