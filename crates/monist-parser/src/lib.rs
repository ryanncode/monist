pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
    use super::parser::Parser;
    use monist_core::ast::{Atomic, Formula, FormulaArena, Var};

    #[test]
    fn test_parse_equality() {
        let mut arena = FormulaArena::new();
        let mut parser = Parser::new("x = y", &mut arena);
        let id = parser.parse_formula();
        assert_eq!(
            arena.get(id),
            Some(&Formula::Atom(Atomic::Eq(
                Var::Free("x".to_string()),
                Var::Free("y".to_string())
            )))
        );
    }

    #[test]
    fn test_parse_logic() {
        let mut arena = FormulaArena::new();
        let mut parser = Parser::new("x = y /\\ y in z", &mut arena);
        let id = parser.parse_formula();
        
        let left = Formula::Atom(Atomic::Eq(
            Var::Free("x".to_string()),
            Var::Free("y".to_string())
        ));
        let right = Formula::Atom(Atomic::Mem(
            Var::Free("y".to_string()),
            Var::Free("z".to_string())
        ));
        
        // We know they are added sequentially in this simple test
        assert_eq!(arena.get(0), Some(&left));
        assert_eq!(arena.get(1), Some(&right));
        assert_eq!(arena.get(id), Some(&Formula::Conj(0, 1)));
    }

    #[test]
    fn test_forall() {
        let mut arena = FormulaArena::new();
        let mut parser = Parser::new("forall x . x = y", &mut arena);
        let id = parser.parse_formula();
        
        let inner = Formula::Atom(Atomic::Eq(
            Var::Bound(0),
            Var::Free("y".to_string())
        ));
        
        assert_eq!(arena.get(0), Some(&inner));
        assert_eq!(arena.get(id), Some(&Formula::Univ(0, "x".to_string(), 0)));
    }
}
