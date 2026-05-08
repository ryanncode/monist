use monist_comb::comblib::vsa_embed::{Codebook, HDCVector};

fn main() {
    println!("=== Holographic Sieve Execution ===");

    // Initialize the VSA environment
    let mut codebook = Codebook::new();

    // The Sieve generates massive amounts of data
    let omega_node = HDCVector::random_basis();
    let alpha_node = HDCVector::random_basis();
    let beta_node = HDCVector::random_basis();
    
    codebook.insert("Omega".to_string(), omega_node.clone());
    codebook.insert("Alpha".to_string(), alpha_node.clone());
    codebook.insert("Beta".to_string(), beta_node.clone());

    let mut swarm = omega_node.superpose(&alpha_node).superpose(&beta_node);

    println!("Constructed Holographic Swarm (Omega, Alpha, Beta) mapped to 10,000-d phase space.\n");

    // Creating an exclusion gate for a specific target
    println!("Applying Search Instance (O(1) Absolute Complement Query to remove Omega)...\n");
    let excluded_swarm = swarm.holographic_exclusion_query(&omega_node);

    println!("Attempting SIC Recovery on the sieved space...");
    let remaining_nodes = codebook.recover_discrete_combinators(excluded_swarm, 0.4);

    println!("Nodes remaining in the swarm after O(1) exclusion gate:");
    for node in remaining_nodes {
        println!(" - {}", node);
    }

    println!("\n[SUCCESS] Holographic O(1) Absolute Complement Query computed using continuous VSA physics!");
}
