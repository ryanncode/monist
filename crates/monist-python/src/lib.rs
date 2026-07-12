use pyo3::prelude::*;
use monist_parser::parser::Parser;
use monist_core::ast::FormulaArena;
use monist_core::graph::GraphArena;
use monist_core::eval::ExecutionLimits;

#[pyclass]
pub struct PyEvaluationResult {
    #[pyo3(get)]
    pub is_stratified: bool,
    #[pyo3(get)]
    pub max_k_iterations: usize,
    #[pyo3(get)]
    pub mcm: f64,
}

#[pyfunction]
fn evaluate_formula(input: &str) -> PyResult<PyEvaluationResult> {
    let mut arena = FormulaArena::new();
    
    let parse_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut parser = Parser::new(input, &mut arena);
        parser.parse_formula()
    }));

    let id = match parse_result {
        Ok(idx) => idx,
        Err(_) => return Err(pyo3::exceptions::PyValueError::new_err("Failed to parse formula")),
    };

    if arena.get(id).is_none() {
        return Err(pyo3::exceptions::PyValueError::new_err("Failed to parse formula"));
    }

    let constraints = monist_core::graph::extract_constraints_aux(&arena, id, 0, false);
    let graph = GraphArena::from_constraints(&constraints);

    let limits = ExecutionLimits::compute_for_graph(&graph)
        .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Graph is empty or unevaluable"))?;

    Ok(PyEvaluationResult {
        is_stratified: limits.mcm >= 0.0,
        max_k_iterations: limits.max_k_iterations,
        mcm: limits.mcm,
    })
}

#[pymodule]
fn monist_python(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(evaluate_formula, m)?)?;
    m.add_class::<PyEvaluationResult>()?;
    Ok(())
}
