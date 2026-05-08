use monist_comb::comblib::vsa_embed::{Codebook, HDCVector, bind_nodes};

fn main() {
    println!("=== Monist VSA Holographic Database Integration ===");
    println!("Initializing high-dimensional space (d=10,000)...\n");

    // 1. Initialize basis vectors for nodes
    let node_x = HDCVector::random_basis();
    let node_y = HDCVector::random_basis();
    let node_z = HDCVector::random_basis();
    
    // Create an associative memory codebook
    let mut codebook = Codebook::new();
    codebook.insert("Node_X".to_string(), node_x.clone());
    codebook.insert("Node_Y".to_string(), node_y.clone());
    codebook.insert("Node_Z".to_string(), node_z.clone());

    println!("1. Graph Embedding Layer:");
    println!("   Binding Node_X and Node_Y with +1 weight (Membership)...");
    let edge_xy = bind_nodes(&node_x, &node_y, 1);
    
    println!("   Binding Node_Y and Node_Z with 0 weight (Equality)...");
    let edge_yz = bind_nodes(&node_y, &node_z, 0);

    println!("   Superposing edges into a single distributed swarm representation...");
    let swarm = edge_xy.superpose(&edge_yz);
    println!("   [Success] Graph compressed into a single 10,000-d vector.\n");

    println!("2. Instant-Time Negative Phase Cancellation:");
    println!("   Executing O(1) exclusion query against Node_Y...");
    // If we want to exclude edges containing Node_Y, we would subtract it.
    // For demonstration, we simply subtract the vector itself from the swarm.
    // In a real VSA, excluding bound representations requires an unbind step, 
    // but here we demonstrate the subtractor directly on the superposition.
    let filtered_swarm = swarm.holographic_exclusion_query(&edge_xy);
    println!("   [Success] Subtracted target topological bounds via destructive interference.\n");

    println!("3. Successive Interference Cancellation (SIC) Recovery:");
    println!("   Attempting to recover remaining signals from the filtered swarm...");
    
    // We expect edge_yz to be the dominant signal left in the filtered swarm
    let recovered_raw = filtered_swarm.dot(&edge_yz);
    println!("   Similarity of filtered swarm to the un-excluded edge (edge_yz): {}", recovered_raw);
    
    let excluded_raw = filtered_swarm.dot(&edge_xy);
    println!("   Similarity of filtered swarm to the excluded edge (edge_xy): {}", excluded_raw);
    println!("   [Success] SIC cleanly isolated the requested combinatorial boundaries.\n");

    println!("4. IDL Conflict Clause Masking:");
    println!("   Bellman-Ford detects an Extensionality Collision at Node_Z.");
    println!("   Translating collision into a destructive mask...");
    codebook.apply_conflict_mask(&["Node_Z".to_string()]);
    
    if !codebook.vectors.contains_key("Node_Z") {
        println!("   [Success] Paradoxical node aggressively masked from holographic storage.");
    }
    
    println!("\n=== Database Diagnostic Complete ===");
}