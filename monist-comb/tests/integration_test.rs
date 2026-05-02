use monist_comb::ast::GNet;
use monist_comb::backend::BendExecutor;
use monist_comb::translate::BendTranslator;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_translation_and_execution() {
        let gnet = GNet::new();
        // Here we would manually construct a small graph
        // representing a combinatorial test case (e.g. `I x = x`).
        
        let translator = BendTranslator::new(&gnet);
        let bend_code = translator.translate();
        
        // Ensure the generated code looks correct natively.
        assert!(bend_code.contains("def main():"));
        
        let temp_dir = env::temp_dir().join("monist_test_exec");
        let _executor = BendExecutor::new(temp_dir);
        
        // Normally we would run this, but for CI/CD environments without Bend installed,
        // we might mock this or skip it.
        // let result = _executor.compile_and_run_cuda("test_transl", &bend_code).unwrap();
        // assert!(result.status.success());
    }
}
