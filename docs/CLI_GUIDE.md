# Monist Engine CLI User Guide

Welcome to the Monist Engine CLI! This guide will help you navigate the unified interactive REPL (Read-Eval-Print Loop) environment, which provides a modern, stateful experience for mathematical verification and Natural Deduction tactical proving.

## Getting Started

To start the interactive session, simply run the CLI without any arguments:

```bash
cargo run -p monist-cli -- repl
```

You will be greeted with a `monist>` prompt, ready to accept your commands.

---

## Interactive Tutorial: Proving Strong Cantorian Preservation

The best way to understand the REPL is through a canonical, continuous example. Let's walk through how a user would interactively prove that the Quine pair preserves Strongly Cantorian (SC) sets.

### 1. Defining Axioms

First, we need to declare the foundational rules we are working with using `assume <name> <formula>`.

```text
monist> assume SC_Def forall x. SC(x) <-> (x = T(x))
Assumed: SC_Def

monist> assume Quine_Flatness forall x y. typestate(Q(x,y)) == max(typestate(x), typestate(y))
Assumed: Quine_Flatness
```

*Note: The system globally registers these named axioms for later retrieval or constraint evaluation.*

### 2. Setting a Goal

We declare the theorem we want to prove using `theorem <name> <formula>`. This transitions the REPL into tactical proof mode.

```text
monist> theorem SC_Preservation forall a b. (SC(a) /\ SC(b)) -> SC(Q(a,b))
[Goal Set] 1 unproven target.
```

### 3. Tactical Natural Deduction

We can use `show_goal` at any time to inspect the local Context (hypotheses) and Target. Let's break down the logic using standard Natural Deduction tactics.

```text
monist> intro a
monist> intro b
monist> intro H_SC
monist> destruct H_SC H1 H2
monist> show_goal

--- Context ---
H1: SC(a)
H2: SC(b)
--- Target ---
SC(Q(a,b))
```

*The `intro` command brought our variables and implication premise into the context, and `destruct` cleanly split the `H_SC` conjunction into `H1` and `H2`.*

### 4. Rewriting

We use `rewrite <name>` to perform symbolic substitution based on equalities.

```text
monist> rewrite SC_Def
[Goal Rewritten] Target 1 is now: Q(a,b) = T(Q(a,b))

monist> rewrite H1
monist> rewrite H2
[Goal Rewritten] Target 1 is now: Q(T(a), T(b)) = T(Q(a,b))
```

### 5. Diagnostics and Stratification

At this stage, we might want to hand off the topological heavy lifting to the internal geometric constraint engine. We can use the `eval` command to test if the formula successfully stratifies across the T-boundary!

```text
monist> eval ((((((a = Ta /\ b = Tb) /\ Qab = a) /\ Qab = b) /\ QTaTb = Ta) /\ QTaTb = Tb) /\ QTaTb = TQab) /\ Qab = TQab
Stratification successful. Witness: {a@0 : 0, b@0 : 0, Ta@0 : 0, Tb@0 : 0, QTaTb@0 : 0, Qab@0 : 0, TQab@0 : 0}
```

*The `eval` and `step` commands bypass the tactical proof state, instantly extracting topological constraints and running the Bellman-Ford cycle detection algorithm to verify mathematical consistency.*

### 6. Managing the Session

You can save your progress mid-proof and load it back later:

```text
monist> save_session my_proof.json
Session saved to my_proof.json.

monist> qed
Proof accepted.
```

---

## Unified Command Reference

### Core Proof Environment

* `help`: Show the help message.
* `exit` / `quit`: Exit the REPL.
* `save_session <file.json>`: Save the current session (axioms and active goals) to a JSON file.
* `load_session <file.json>`: Load a previously saved session.

### Axioms & Proofs

* `assume <name> <formula>`: Register a named axiom into the global environment.
* `theorem <name> <formula>`: Start a new proof with a target goal and enter tactical mode.
* `deff <name> := <formula>`: Define a macro with Kosaraju SCC pre-flattening, collapsing internal 0-weight edges into an optimized DAG to minimize topological evaluation overhead.
* `show_goal`: Print the active target goal and its local context/hypotheses (with De Bruijn scope tags intentionally hidden to minimize UI clutter).
* `qed`: Conclude an interactive proof if all goals are solved.
* `abort`: Cancel the current interactive proof.

### Interactive Tactics

* `intro [name]`: Introduce a premise or a universally quantified variable into the local context.
* `exact <name>`: Close the current goal if it exactly matches the specified hypothesis.
* `apply <name>`: Apply backward reasoning using an implication hypothesis.
* `split`: Split a conjunction (`/\`) goal into two separate sub-goals.
* `left` / `right`: Choose a side to prove for a disjunction (`\/`) goal.
* `destruct <name> [n1] [n2]`: Break down a hypothesis (like a conjunction or disjunction) into smaller pieces in the context.
* `rewrite <name>`: Substitute variables inside the goal based on an equality hypothesis.
* `cut <formula>`: Weaponized Cut Tactic. Introduces a highly saturated formula as a sub-goal, allowing you to explicitly trigger Extensionality bounds.
* `focus_hyp <name>`: Pull a specific hypothesis to the top of your context array for easier visibility.
* `defer`: Skip the current active goal, sending it to the back of the `ProofState` queue.

### Diagnostic Evaluation

* `check_strat <formula>`: An alias/sandbox command to run the AST parser and Bellman-Ford algorithm on raw geometry instantly before proofs.
* `eval <formula>`: Immediately test a formula for stratification loops and geometric friction without entering the tactical proof state.
* `step <formula>`: Process a logical formula step-by-step, providing immediate, color-coded feedback on topological friction and geometric bounds.

---

## Understanding the Engine: Constraints and Tactics

To effectively use this dual-engine architecture, it is crucial to understand what the underlying Rust graph engine can "see," and how to guide your tactical proofs to produce that specific geometry.

### Geometric Constraints
The Bellman-Ford topological engine (`eval` / `step`) is incredibly fast, but its vocabulary is strictly limited to foundational set-theoretic boundaries. It translates formulas into a geometric Directed Acyclic Graph (DAG) using **only** the following atomic constraints:

* **Equality (`x = y`)**: Generates a **`0` weight** constraint. The engine mathematically locks the two variables at the exact same typestate level.
* **Membership (`x e y`)**: Generates a **`+1` weight** constraint. The set `y` must exist at a typestate level strictly higher than its element `x`.
* **Function Application (`z = u(v)`)**: Generates a **`+1` weight** constraint. The function `u` must be typed one level higher than the argument `v`.
* **Lambda Abstraction (`z = \lambda x. t`)**: Generates a **`+1` weight** constraint. The abstracted function body sits one level higher than the variable it binds.
* **Quine Pairs (`Q(a,b)`)**: Generates **`0` weight** constraints. Unlike standard Kuratowski pairs (which force a `+2` type shift), Quine pairs are geometrically "flat." 

### The Tactical Workflow
When staring at a high-level mathematical theorem, your goal as the user is to act as the "compiler," using tactics to translate human semantics down into the raw spatial constraints listed above.

1. **Strip the Logical Scaffolding**: The geometric engine evaluates spatial structures, not conditional hypotheticals. Use `intro` to strip away `forall` quantifiers and pull `If` premises down into your local Context as usable facts. Use `destruct` to break complex conjunctions (`/\`) into isolated facts.
2. **Unfold Semantic Definitions**: The graph engine cannot read abstract properties like "Strongly Cantorian" or "Ordinal." Use `rewrite` (alongside your global `assume` axioms) to unfold these abstractions into raw set theory (e.g., rewriting `SC(x)` to `x = T(x)`).
3. **Isolate the Friction**: Use `rewrite` to substitute variables across your equalities until the core components of your target goal are expressed purely in terms of raw variables connected by `=` or `e`. You want to find the exact boundary where the theorem forces a variable to cross a typestate level.
4. **The Topological Handoff**: Once your `ProofState` has been entirely stripped of implications, quantifiers, and abstract names, you gather the surviving structural equations and feed them into `eval`. The graph engine will instantly compute if that raw geometry can safely exist in finite computational space, or if it collapses into a negative-weight cycle (a paradox).

---

## One-Shot Commands

You can still use the CLI in traditional one-shot mode for scripting or quick checks outside the REPL.

### `verify <formula>`
Quickly evaluates a formula.

```bash
cargo run -p monist-cli -- verify "x = y"
```

### `export-smt <formula>`
Generates a `StratificationWitness` trace in standard SMT-LIB v2 format for external theorem provers (like Z3 or CVC5).

```bash

cargo run -p monist-cli -- export-smt "x in y /\ y in z /\ z in x" > trace.smt2