use crate::ir::Comb;

// helper function for generating variables
pub fn v(name: &str) -> Comb {
    Comb::Var(name.to_string())
}

// Booleans
pub fn true_comb() -> Comb {
    Comb::K
}

pub fn false_comb() -> Comb {
    Comb::K.app(Comb::I)
}

pub fn and_comb() -> Comb {
    // \p q. p q p
    v("p")
        .app(v("q"))
        .app(v("p"))
        .abstract_var("q")
        .abstract_var("p")
}

pub fn or_comb() -> Comb {
    // \p q. p p q
    v("p")
        .app(v("p"))
        .app(v("q"))
        .abstract_var("q")
        .abstract_var("p")
}

pub fn not_comb() -> Comb {
    // \p. p False True
    v("p").app(false_comb()).app(true_comb()).abstract_var("p")
}

// Numerals
pub fn zero() -> Comb {
    // \f x. x -> mathematically equivalent to False
    false_comb()
}

pub fn succ() -> Comb {
    // \n f x. f (n f x)
    v("f")
        .app(v("n").app(v("f")).app(v("x")))
        .abstract_var("x")
        .abstract_var("f")
        .abstract_var("n")
}

// Inductive Lists geometries
pub fn cons() -> Comb {
    // \h t z. z h t
    v("z")
        .app(v("h"))
        .app(v("t"))
        .abstract_var("z")
        .abstract_var("t")
        .abstract_var("h")
}

pub fn head() -> Comb {
    // \l. l True
    v("l").app(true_comb()).abstract_var("l")
}

pub fn tail() -> Comb {
    // \l. l False
    v("l").app(false_comb()).abstract_var("l")
}

// Y Combinator
pub fn y_comb() -> Comb {
    // \f. (\x. f (x x)) (\x. f (x x))
    let inner = v("f").app(v("x").app(v("x"))).abstract_var("x");
    inner.clone().app(inner).abstract_var("f")
}
