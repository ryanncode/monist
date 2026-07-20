use monist_core::ast::FormulaArena;
use monist_core::budget::ResourceBudget;
use monist_parser::parser::Parser;

#[test]
#[should_panic(expected = "Nesting Limit Exceeded")]
fn test_extreme_nesting_panic() {
    let mut arena = FormulaArena::new();
    let budget = ResourceBudget {
        max_bytes: 1_000_000,
        max_depth: 500,
        ..ResourceBudget::default()
    };

    let mut formula = String::new();
    for _ in 0..100 { // 100 * 7 = 700 depth > 500
        formula.push('(');
    }
    formula.push_str("x = x");
    for _ in 0..100 {
        formula.push(')');
    }

    let mut parser = Parser::new(&formula, &mut arena, budget);
    parser.parse_formula();
}

#[test]
#[should_panic(expected = "AST Node Limit Exceeded")]
fn test_macro_bomb_panic() {
    let mut arena = FormulaArena::new();
    let budget = ResourceBudget {
        max_bytes: 1_000_000,
        max_ast_nodes: 5_000,
        ..ResourceBudget::default()
    };
    
    let mut formula = String::from("x = x");
    for _ in 0..6_000 {
        formula.push_str(" \\/ x = x");
    }

    let mut parser = Parser::new(&formula, &mut arena, budget);
    parser.parse_formula();
}

#[test]
fn test_large_but_safe_formula() {
    let mut arena = FormulaArena::new();
    let budget = ResourceBudget {
        max_bytes: 1_000_000,
        max_depth: 1_000,
        max_ast_nodes: 10_000,
        ..ResourceBudget::default()
    };
    
    let mut formula = String::new();
    for _ in 0..100 { // 100 * 7 = 700 depth < 1000
        formula.push('(');
    }
    formula.push_str("x = x");
    for _ in 0..100 {
        formula.push(')');
    }

    let mut parser = Parser::new(&formula, &mut arena, budget);
    parser.parse_formula(); // should not panic
}
