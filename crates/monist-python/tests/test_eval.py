import pytest
from monist_python import evaluate_formula

def test_russells_paradox():
    """
    Tests Extensionality Collision (Russell's Paradox).
    The engine should parse {x | ~(x in x)} in {x | ~(x in x)}, assign depths,
    and detect the negative-weight cycle via Bellman-Ford.
    """
    result = evaluate_formula("{x | ~(x in x)} in {x | ~(x in x)}")
    
    # It must not be stratified
    assert not result.is_stratified
    
    # Execution halts at 0 safe iterations due to contradiction
    assert result.max_k_iterations == 0
    
    # Minimum Cycle Mean must be exactly -1.0
    assert result.mcm == -1.0

def test_stratified_identity():
    """
    Tests a basic Stratified Identity: forall x . x = x
    This should be cleanly evaluated and stratified.
    """
    result = evaluate_formula("forall x . x = x")
    
    # It must be perfectly stratified (ZFC-compliant logic)
    assert result.is_stratified
    
    # Safe iterations limit should be calculated (> 0)
    assert result.max_k_iterations > 0
    
    # MCM should be positive or zero
    assert result.mcm >= 0.0

def test_parse_error():
    """
    Tests that invalid syntax raises an exception cleanly.
    """
    with pytest.raises(ValueError, match="Failed to parse formula"):
        evaluate_formula("in in in = = = = x x x x =")
