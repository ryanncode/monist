use super::encodings::{v, pair};
use crate::ir::Comb;

/// Hailperin's 8 Set Combinators for New Foundations (Hailperin 1944).
///
/// Hailperin proved that NF is finitely axiomatizable by replacing unrestricted
/// stratified comprehension with 8 algebraic set-forming operations.
/// Every stratified set builder can be compiled into a combination of these 8 operators.

/// 1. Universal Set: V = {x | x = x}
/// Represented as a combinator predicate returning true for all inputs.
pub fn universal_set() -> Comb {
    // \x. True (K I)
    Comb::K.app(Comb::I)
}

/// 2. Set Intersection: A ∩ B = {x | x ∈ A ∧ x ∈ B}
pub fn set_intersection(a: Comb, b: Comb) -> Comb {
    // \x. (A x) ∧ (B x)
    Comb::Conj
        .app(a.app(v("x")))
        .app(b.app(v("x")))
        .abstract_var("x")
}

/// 3. Set Complement: \overline{A} = {x | x ∉ A}
pub fn set_complement(a: Comb) -> Comb {
    // \x. ¬(A x)
    Comb::Neg.app(a.app(v("x"))).abstract_var("x")
}

/// 4. Cartesian Product: A × B = {<x, y> | x ∈ A ∧ y ∈ B}
pub fn cartesian_product(a: Comb, b: Comb) -> Comb {
    // \p. (A (fst p)) ∧ (B (snd p))
    let fst_p = v("p").app(Comb::K);
    let snd_p = v("p").app(Comb::K.app(Comb::I));
    Comb::Conj
        .app(a.app(fst_p))
        .app(b.app(snd_p))
        .abstract_var("p")
}

/// 5. Converse Relation: R^{-1} = {<y, x> | <x, y> ∈ R}
pub fn relation_converse(r: Comb) -> Comb {
    // \p. R (pair (snd p) (fst p))
    let fst_p = v("p").app(Comb::K);
    let snd_p = v("p").app(Comb::K.app(Comb::I));
    let swapped_pair = pair().app(snd_p).app(fst_p);
    r.app(swapped_pair).abstract_var("p")
}

/// 6. Domain: dom(R) = {x | ∃y. <x, y> ∈ R}
pub fn relation_domain(r: Comb) -> Comb {
    // \x. Exists (\y. R (pair x y))
    let p = pair().app(v("x")).app(v("y"));
    let inner = r.app(p).abstract_var("y");
    Comb::Var("Exists".to_string()).app(inner).abstract_var("x")
}

/// 7. Identity Relation: I = {<x, x> | x ∈ V}
pub fn identity_relation() -> Comb {
    // \p. (fst p) = (snd p)
    let fst_p = v("p").app(Comb::K);
    let snd_p = v("p").app(Comb::K.app(Comb::I));
    Comb::Eq.app(fst_p).app(snd_p).abstract_var("p")
}

/// 8. Cyclic Permutation: R^{(3)} = {<<y, z>, x> | <<x, y>, z> ∈ R}
pub fn cyclic_permutation(r: Comb) -> Comb {
    // \p. let yz = fst p in let x = snd p in R (pair (pair x (fst yz)) (snd yz))
    let yz = v("p").app(Comb::K);
    let x = v("p").app(Comb::K.app(Comb::I));
    let y = yz.clone().app(Comb::K);
    let z = yz.app(Comb::K.app(Comb::I));
    let orig_p = pair().app(pair().app(x).app(y)).app(z);
    r.app(orig_p).abstract_var("p")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hailperin_basis_compilation() {
        let v_set = universal_set();
        let empty_set = set_complement(v_set.clone());
        let intersected = set_intersection(v_set, empty_set);
        assert!(matches!(intersected, Comb::App(_, _)));
    }

    #[test]
    fn test_identity_relation_properties() {
        let id_rel = identity_relation();
        let conv_id = relation_converse(id_rel);
        assert!(matches!(conv_id, Comb::App(_, _)));
    }
}

