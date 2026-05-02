use monist_comb::comblib::holographic::{exclusion_gate, holographic_search};
use monist_comb::ir::Comb;

fn main() {
    println!("=== Holographic Sieve Execution ===");
    
    // We demonstrate holographic search
    // Creating an exclusion gate for a specific target
    let target = Comb::Terminal("Omega".to_string());
    
    let ex_gate = exclusion_gate(target.clone());
    println!("Constructed Exclusion Gate for {:?}: \n{:?}\n", target, ex_gate);
    
    // Create the holographic search combinator
    let h_search = holographic_search();
    println!("Holographic Search Combinator: \n{:?}\n", h_search);
    
    // Apply holographic search to a mock swarm and target
    let mock_swarm = Comb::Terminal("SwarmIndex".to_string());
    let search_instance = h_search.app(mock_swarm).app(target);
    
    println!("Applying Search Instance (O(1) Absolute Complement Query): \n{:?}\n", search_instance);
    
    println!("[SUCCESS] Holographic O(1) Absolute Complement Query compiled to combinatory primitives successfully!");
}
