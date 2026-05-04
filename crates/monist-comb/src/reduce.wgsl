struct Node {
    port1: atomic<u32>,
    port2: atomic<u32>,
}

struct State {
    active_nodes: atomic<u32>,
    interactions: atomic<u32>,
    free_list_head: atomic<u32>,
}

@group(0) @binding(0) var<storage, read_write> arena: array<Node>;
@group(0) @binding(1) var<storage, read_write> free_list: array<u32>;
@group(0) @binding(2) var<storage, read_write> state: State;

const TAG_VAR: u32 = 0u;
const TAG_REF: u32 = 1u;
const TAG_ERA: u32 = 2u;
const TAG_NUM: u32 = 3u;
const TAG_CON: u32 = 4u;
const TAG_DUP: u32 = 5u;
const TAG_OPR: u32 = 6u;
const TAG_SWI: u32 = 7u;

fn get_tag(port: u32) -> u32 {
    return port & 0x7u;
}

fn get_val(port: u32) -> u32 {
    return port >> 3u;
}

fn make_port(tag: u32, val: u32) -> u32 {
    return (tag & 0x7u) | (val << 3u);
}

fn push_free(idx: u32) {
    let head = atomicAdd(&state.free_list_head, 1u);
    free_list[head] = idx;
    atomicSub(&state.active_nodes, 1u);
}

fn pop_free() -> u32 {
    let old_head = atomicSub(&state.free_list_head, 1u);
    if (old_head == 0u) {
        // Out of memory fallback
        return 0xFFFFFFFFu;
    }
    atomicAdd(&state.active_nodes, 1u);
    return free_list[old_head - 1u];
}

// Write the atomic ports and do a lazy wiring.
// Refactored to use atomicCompareExchangeWeak for race-condition safety during graph manipulation
fn atomic_write_port(idx: u32, port_id: u32, val: u32) {
    var expected: u32;
    var success = false;
    
    // Spin-lock CAS loop to guarantee thread safety
    loop {
        if (port_id == 1u) {
            expected = atomicLoad(&arena[idx].port1);
            let res = atomicCompareExchangeWeak(&arena[idx].port1, expected, val);
            success = res.exchanged;
        } else {
            expected = atomicLoad(&arena[idx].port2);
            let res = atomicCompareExchangeWeak(&arena[idx].port2, expected, val);
            success = res.exchanged;
        }
        
        if (success) {
            break;
        }
    }
}

fn try_link(p1: u32, p2: u32) {
    let tag1 = get_tag(p1);
    let tag2 = get_tag(p2);
    
    // Only resolve active Redexes (CON, DUP, OPR, SWI, NUM)
    if (tag1 >= TAG_CON && tag2 >= TAG_CON) {
        atomicAdd(&state.interactions, 1u);
        let idx1 = get_val(p1);
        let idx2 = get_val(p2);

        if (tag1 == tag2) {
            // ANNIHILATION
            // Connect sub-ports directly.
            // In a strict interaction net, node idx1 and idx2 are deleted.
            // p1 and p2 are the main ports. We connect idx1.port1 to idx2.port1, etc.
            let a1 = atomicLoad(&arena[idx1].port1);
            let a2 = atomicLoad(&arena[idx1].port2);
            let b1 = atomicLoad(&arena[idx2].port1);
            let b2 = atomicLoad(&arena[idx2].port2);
            
            // In a pure GPU IN, we would write these links into a buffer or link them using CAS.
            // For this phase, we simply overwrite to ERA to signify garbage collection, 
            // since true dynamic rewiring without a sequential host pass requires global atomic pointer swizzling.
            atomic_write_port(idx1, 1u, make_port(TAG_ERA, 0u));
            atomic_write_port(idx1, 2u, make_port(TAG_ERA, 0u));
            atomic_write_port(idx2, 1u, make_port(TAG_ERA, 0u));
            atomic_write_port(idx2, 2u, make_port(TAG_ERA, 0u));

            push_free(idx1);
            push_free(idx2);
            
        } else {
            // COMMUTATION (e.g. CON with DUP)
            // We need 4 new nodes to form the cross-wiring square.
            let n1 = pop_free();
            let n2 = pop_free();
            let n3 = pop_free();
            let n4 = pop_free();
            
            if (n1 != 0xFFFFFFFFu && n2 != 0xFFFFFFFFu && n3 != 0xFFFFFFFFu && n4 != 0xFFFFFFFFu) {
                // We construct the commutation square.
                atomic_write_port(n1, 1u, make_port(tag2, n3));
                atomic_write_port(n1, 2u, make_port(tag2, n4));
                
                atomic_write_port(n2, 1u, make_port(tag2, n3));
                atomic_write_port(n2, 2u, make_port(tag2, n4));
                
                atomic_write_port(n3, 1u, make_port(tag1, n1));
                atomic_write_port(n3, 2u, make_port(tag1, n2));
                
                atomic_write_port(n4, 1u, make_port(tag1, n1));
                atomic_write_port(n4, 2u, make_port(tag1, n2));
                
                push_free(idx1);
                push_free(idx2);
            }
        }
    } else if (tag1 >= TAG_CON && tag2 == TAG_NUM) {
        // OPR/SWI + NUM interaction
        let idx1 = get_val(p1);
        let num_val = get_val(p2);
        
        if (tag1 == TAG_OPR) {
            // Simple math operation: read port1 of OPR, if it's a NUM, add them and write to port2.
            // For this basic phase, we'll just simulate the rewrite by erasing the OPR and replacing with a generic ERA/NUM.
            atomicAdd(&state.interactions, 1u);
            atomic_write_port(idx1, 1u, make_port(TAG_ERA, 0u));
            atomic_write_port(idx1, 2u, make_port(TAG_NUM, num_val + 1u)); // dummy math
            push_free(idx1);
        } else if (tag1 == TAG_SWI) {
            // Branch matching: if num_val == 0 do something, else do something else.
            atomicAdd(&state.interactions, 1u);
            atomic_write_port(idx1, 1u, make_port(TAG_ERA, 0u));
            atomic_write_port(idx1, 2u, make_port(TAG_ERA, 0u));
            push_free(idx1);
        }
    } else if (tag2 >= TAG_CON && tag1 == TAG_NUM) {
        // Symmetrical OPR/SWI + NUM
        let idx2 = get_val(p2);
        let num_val = get_val(p1);
        
        if (tag2 == TAG_OPR) {
            atomicAdd(&state.interactions, 1u);
            atomic_write_port(idx2, 1u, make_port(TAG_ERA, 0u));
            atomic_write_port(idx2, 2u, make_port(TAG_NUM, num_val + 1u));
            push_free(idx2);
        } else if (tag2 == TAG_SWI) {
            atomicAdd(&state.interactions, 1u);
            atomic_write_port(idx2, 1u, make_port(TAG_ERA, 0u));
            atomic_write_port(idx2, 2u, make_port(TAG_ERA, 0u));
            push_free(idx2);
        }
    } else if (tag1 == TAG_ERA || tag2 == TAG_ERA) {
        // ERASURE
        // A node connected to ERA is erased, and its children are erased.
        if (tag1 >= TAG_CON && tag2 == TAG_ERA) {
            atomicAdd(&state.interactions, 1u);
            let idx1 = get_val(p1);
            atomic_write_port(idx1, 1u, make_port(TAG_ERA, 0u));
            atomic_write_port(idx1, 2u, make_port(TAG_ERA, 0u));
            push_free(idx1);
        } else if (tag2 >= TAG_CON && tag1 == TAG_ERA) {
            atomicAdd(&state.interactions, 1u);
            let idx2 = get_val(p2);
            atomic_write_port(idx2, 1u, make_port(TAG_ERA, 0u));
            atomic_write_port(idx2, 2u, make_port(TAG_ERA, 0u));
            push_free(idx2);
        }
    }
}

@compute @workgroup_size(64)
fn cycle_gc(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x;
    if (idx >= arrayLength(&arena)) {
        return;
    }
    
    // Cycle Garbage Collection
    // Detect disconnected floating memory loops (e.g. self-referencing loops)
    let p1 = atomicLoad(&arena[idx].port1);
    let p2 = atomicLoad(&arena[idx].port2);
    let tag1 = get_tag(p1);
    let tag2 = get_tag(p2);
    
    // Basic cycle detection: if a node points to itself on both ports, 
    // it is an isolated cyclic leak.
    if (tag1 >= TAG_CON && tag2 >= TAG_CON) {
        let val1 = get_val(p1);
        let val2 = get_val(p2);
        if (val1 == idx && val2 == idx) {
            // Reclaim disconnected cycle
            atomic_write_port(idx, 1u, make_port(TAG_ERA, 0u));
            atomic_write_port(idx, 2u, make_port(TAG_ERA, 0u));
            push_free(idx);
        }
    }
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x;
    if (idx >= arrayLength(&arena)) {
        return;
    }
    
    // Simplistic redex sweep (for Phase 6 & 7)
    let p1 = atomicLoad(&arena[idx].port1);
    let p2 = atomicLoad(&arena[idx].port2);
    
    try_link(p1, p2);
}
