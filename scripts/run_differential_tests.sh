#!/bin/bash
set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
MONIST_ROOT="$DIR/.."

# Locate nf-sketches/parse-strat
LEAN_DIR=""
if [ -d "$MONIST_ROOT/../nf-sketches/parse-strat" ]; then
    LEAN_DIR="$MONIST_ROOT/../nf-sketches/parse-strat"
elif [ -d "$MONIST_ROOT/../../nf-sketches/parse-strat" ]; then
    LEAN_DIR="$MONIST_ROOT/../../nf-sketches/parse-strat"
elif [ -d "/home/gemini/Data/Code/nf-sketches/parse-strat" ]; then
    LEAN_DIR="/home/gemini/Data/Code/nf-sketches/parse-strat"
fi

if [ -z "$LEAN_DIR" ]; then
    echo "Error: nf-sketches/parse-strat repository not found."
    echo "Please set NF_SKETCHES_DIR or clone nf-sketches as a sibling directory."
    exit 1
fi

tests=(
    "strongly_cantorian"
    "incomparable_cardinals"
    "sc_daemon_proof"
    "ai_alignment_playbook"
    "smt_k_iteration"
    "extensionality_collision"
    "specker_refutation"
    "russell"
    "test_specker"
    "burali_forti"
    "transfinite"
    "frege_russell_numerals"
    "hailperin_finite_axioms"
    "lawvere_pseudo_elephant"
    "logit_space_sic_filter"
)

echo "================================================================================"
echo "    Running Monist -> Lean 4 SMT Differential Equivalence Verification Suite    "
echo "================================================================================"

pass_count=0
total_count=${#tests[@]}

for test_name in "${tests[@]}"; do
    printf "[RUN ] %-30s ... " "$test_name"
    
    # Run binary and extract all SMT blocks
    output=$(cargo run --manifest-path "$MONIST_ROOT/Cargo.toml" -p monist-examples --bin "$test_name" 2>/dev/null || true)
    
    # Check if SMT block exists
    if ! echo "$output" | grep -q "; === BEGIN STRATIFICATION WITNESS ==="; then
        printf "\033[33mNO SMT WITNESS\033[0m\n"
        continue
    fi
    
    # Stream SMT block directly to Lean 4 SMT ingest
    lean_result=$(echo "$output" | awk '/; === BEGIN STRATIFICATION WITNESS ===/{flag=1} flag{print} /; === END STRATIFICATION WITNESS ===/{flag=0; exit}' | lake exe --dir "$LEAN_DIR" parse-strat --ingest-smt 2>/dev/null || true)
    
    if echo "$lean_result" | grep -q "Equivalence Check: Both Lean and Rust agree on"; then
        if echo "$lean_result" | grep -q "agree on SUCCESS"; then
            printf "\033[32mPASS (Agreed on SUCCESS)\033[0m\n"
        else
            printf "\033[32mPASS (Agreed on FAILURE)\033[0m\n"
        fi
        pass_count=$((pass_count + 1))
    else
        printf "\033[31mFAIL (Mismatch or Ingest Error)\033[0m\n"
    fi
done

echo "================================================================================"
printf "Differential Verification Result: %d/%d test suites mutually verified.\n" "$pass_count" "$total_count"
echo "================================================================================"

