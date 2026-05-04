use monist_comb::ir::Comb;
use monist_comb::backend::WgpuExecutor;
use monist_comb::ast::GNet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation_and_execution() {
        let comb = Comb::I;
        
        // Convert to GNet
        let gnet = GNet::from_comb(&comb, 1024);
        
        let executor = WgpuExecutor::new();
        let (_, state) = executor.execute(&gnet);
        
        // Assert execution happened and memory was active
        assert!(state.active_nodes > 0);
    }
}
