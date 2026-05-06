#!/bin/bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$DIR/../../nf-sketches/parse-strat"

tests=("strongly_cantorian" "incomparable_cardinals" "sc_daemon_proof" "ai_alignment_playbook" "smt_k_iteration" "extensionality_collision" "specker_refutation" "russell" "test_specker" "burali_forti")

for test_name in "${tests[@]}"; do
    echo "========================================"
    echo "Piping test: $test_name"
    cargo run --manifest-path "$DIR/../Cargo.toml" --bin $test_name > output.log 2>&1
    
    # Extract each SMT block into a separate file and run parse-strat
    awk '
    /; === BEGIN STRATIFICATION WITNESS ===/ {
        file = "out_" ++count ".smt";
        flag = 1;
    }
    flag {
        print > file;
    }
    /; === END STRATIFICATION WITNESS ===/ {
        flag = 0;
    }
    ' output.log

    for f in out_*.smt; do
        if [ -f "$f" ]; then
            echo "-> Parsing $f"
            lake exe parse-strat --ingest-smt < "$f"
            rm "$f"
        fi
    done
    rm -f output.log
done
