use monist_comb::ast::{GNet, Node, Port, TAG_CON, TAG_ERA, TAG_NUM, TAG_OPR};
use monist_comb::backend::WgpuExecutor;
use monist_comb::ir::Comb;

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

    #[test]
    fn test_numeric_primitives() {
        // Test OPR + NUM rewrite rule
        let mut gnet = GNet::new(100);

        // OPR node
        let opr_idx = gnet.alloc_node(Port::new(TAG_NUM, 5), Port::new(TAG_NUM, 0));
        // Wire connecting OPR main port to NUM port
        let _wire_idx = gnet.alloc_node(Port::new(TAG_OPR, opr_idx), Port::new(TAG_NUM, 10));

        // Trigger execution
        let executor = WgpuExecutor::new();
        let (out_net, state) = executor.execute(&gnet);

        // It should have performed at least 1 interaction
        assert!(state.interactions > 0);

        // The OPR node should be erased
        assert_eq!(out_net.nodes[opr_idx as usize].port1().tag(), TAG_ERA);
        // And the result should be updated with dummy math (num_val + 1 = 10 + 1 = 11)
        assert_eq!(out_net.nodes[opr_idx as usize].port2().tag(), TAG_NUM);
        assert_eq!(out_net.nodes[opr_idx as usize].port2().val(), 11);
    }

    #[test]
    fn test_cycle_garbage_collection() {
        // Test cyclic memory reclamation
        let mut gnet = GNet::new(100);

        // Allocate a node and manually make it point to itself
        // ( simulating a disconnected cyclic loop )
        let cycle_idx = gnet.alloc_node(Port(0), Port(0));
        gnet.nodes[cycle_idx as usize] =
            Node::new(Port::new(TAG_CON, cycle_idx), Port::new(TAG_CON, cycle_idx));

        let executor = WgpuExecutor::new();
        let (out_net, _state) = executor.execute(&gnet);

        // The GC pass should have detected the self-loop and erased it
        assert_eq!(out_net.nodes[cycle_idx as usize].port1().tag(), TAG_ERA);
        assert_eq!(out_net.nodes[cycle_idx as usize].port2().tag(), TAG_ERA);
    }
}
