use crate::ir::Comb;

/// A Frege-Russell numeral `N` is the set of all sets of size `N`.
/// We represent a set as a list, so `Num0` is the set of all empty lists.
pub fn num0() -> Comb {
    // isNil is a function that returns True if the list is empty, False otherwise.
    // In standard Scott encoding for lists:
    // isNil = \l. l True (\h t. False)
    // Here we can just mock it or rely on existing list checks if any.
    // We'll simulate `mockIsSizeZero` from the lean file:
    Comb::app(
        Comb::app(Comb::S, Comb::I),
        Comb::app(Comb::K, crate::comblib::encodings::true_comb()),
    )
}

/// To check if a list has size 1:
/// `isSizeOne = \s. and (not (isNil s)) (isNil (tail s))`
pub fn num1() -> Comb {
    // mockIsSizeOne from the lean file
    // app (app S I) (app K fls)
    Comb::app(
        Comb::app(Comb::S, Comb::I),
        Comb::app(Comb::K, crate::comblib::encodings::false_comb()),
    )
}
