# Monist CLI User Guide

Welcome to the Monist Engine CLI! This guide will help you navigate the new interactive REPL (Read-Eval-Print Loop) environment, which is designed to provide a modern, stateful experience for mathematical verification.

## Getting Started

To start the interactive session, simply run the CLI without any arguments:

```bash
cargo run -p monist-cli -- repl
```

You will be greeted with a `monist>` prompt, ready to accept your commands.

## Core Commands

### 1. `assume <axiom>`
Use this command to register a structural axiom into the current session's state.

**Example:**

```
monist> assume Extensionality
```

This registers "Extensionality" as a working premise for future validations.

### 2. `step <formula>`
This is the core diagnostic visualizer. It processes a logical formula step-by-step, providing immediate, color-coded feedback on topological friction and geometric bounds. It extracts constraints, collapses zero-weight SCCs, and executes the Bellman-Ford algorithm to detect negative-weight cycles.

**Example:**

```
monist> step x in x
```

*Notice how negative edge weights (like `-1`) are highlighted in **red** to immediately draw attention to unstratified boundaries!*

### 3. `save_session <file.json>`
Saves your current working state (including any assumed axioms and graph components) to a JSON file on disk. Perfect for long-running verification projects.

**Example:**

```
monist> save_session my_work.json
```

### 4. `load_session <file.json>`
Restores a previously saved session state from disk.

**Example:**

```
monist> load_session my_work.json
```

### 5. `exit` or `quit`
Terminates the REPL session.

## One-Shot Commands

You can still use the CLI in traditional one-shot mode for scripting or quick checks.

### `verify <formula>`
Quickly evaluates a formula without entering the REPL.

**Example:**

```bash
cargo run -p monist-cli -- verify "x = y"
```

### `export-smt <formula>`
Generates a `StratificationWitness` trace in standard SMT-LIB v2 format. This allows external theorem provers (like Z3 or CVC5) to cross-verify the topological boundaries calculated by the Monist Engine.

**Example:**

```bash
cargo run -p monist-cli -- export-smt "x in y /\ y in z /\ z in x" > trace.smt2