import React, { useState, useEffect, useRef } from 'react';
import EvaluationWorker from './worker?worker';
import { SyntaxToolkit } from './SyntaxToolkit';
import './ReplConsole.css';

interface ReplConsoleProps {
  workerRef: React.MutableRefObject<Worker | null>;
  onCommandExecuted?: (cmd: string) => void;
}

interface FormattedHypothesis {
  name: string;
  formula: string;
  raw_node: number;
}

interface FormattedGoal {
  ctx: FormattedHypothesis[];
  target: string;
  raw_target_node: number;
}

interface FormattedProofState {
  goals: FormattedGoal[];
  is_stratified: boolean;
  mcm: number;
}

const STARTER_PROOFS = [
  { name: 'Modus Ponens', cmd: 'theorem ModusPonens ((P -> Q) /\\ P) -> Q' },
  { name: 'Identity', cmd: 'theorem Identity P -> P' },
  { name: 'De Morgan (LR)', cmd: 'theorem DeMorganLR ~(A \\/ B) -> (~A /\\ ~B)' },
  { name: 'Quine Atom Flatness', cmd: 'theorem QuineAtom Omega in Omega' },
  { name: 'Russell Collision', cmd: 'theorem Russell {x | ~(x in x)} in {x | ~(x in x)}' },
  { name: 'Subset Transitivity', cmd: 'theorem SubsetTrans ((A subset B) /\\ (B subset C)) -> (A subset C)' },
];

const QUICK_TACTICS = [
  'intro', 'split', 'apply', 'exact', 'destruct', 'stratify', 'refl', 'simp', 'rw', 'have', 'cut', 'elevate', 'sc_cut', 'qed'
];

export function ReplConsole({ workerRef, onCommandExecuted }: ReplConsoleProps) {
  const [history, setHistory] = useState<{ type: 'input' | 'output' | 'error', text: string }[]>([
    { type: 'output', text: '=== Monist Interactive Theorem Prover (ITP) ===' },
    { type: 'output', text: 'Type "help" for syntax reference, or click a 1-Click Starter Proof above.' }
  ]);
  const [inputValue, setInputValue] = useState('');
  const [proofState, setProofState] = useState<FormattedProofState | null>(null);
  const [activeGoalIndex, setActiveGoalIndex] = useState(0);
  const [isEvaluating, setIsEvaluating] = useState(false);
  const historyRef = useRef<HTMLDivElement>(null);
  const endOfLogRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (historyRef.current) {
      historyRef.current.scrollTop = historyRef.current.scrollHeight;
    }
  }, [history, proofState]);

  useEffect(() => {
    const handleMessage = (e: MessageEvent) => {
      setIsEvaluating(false);
      if (e.data.type === 'REPL_UPDATE') {
        if (e.data.output_lines && e.data.output_lines.length > 0) {
          const newLines = e.data.output_lines.map((line: string) => ({ type: 'output' as const, text: line }));
          setHistory(prev => [...prev, ...newLines]);
        }
        if (e.data.error) {
          setHistory(prev => [...prev, { type: 'error', text: e.data.error }]);
        }
        setProofState(e.data.state || null);
        setActiveGoalIndex(0);
      } else if (e.data.type === 'REPL_ERROR') {
        setHistory(prev => [...prev, { type: 'error', text: e.data.error }]);
      }
    };

    workerRef.current?.addEventListener('message', handleMessage);
    return () => workerRef.current?.removeEventListener('message', handleMessage);
  }, [workerRef]);

  const executeCommandLine = (cmdStr: string) => {
    const trimmed = cmdStr.trim();
    if (!trimmed) return;

    if (onCommandExecuted) {
      onCommandExecuted(trimmed);
    }

    if (trimmed === 'help') {
      setHistory(prev => [...prev, 
        { type: 'output', text: 'Logical Operators Syntax:' },
        { type: 'output', text: '  -> | →                Implication' },
        { type: 'output', text: '  <-> | ↔               IFF (If and only if)' },
        { type: 'output', text: '  forall | ∀            Universal Quantifier' },
        { type: 'output', text: '  exists | ∃            Existential Quantifier' },
        { type: 'output', text: '  /\\ | & | ∧            Conjunction (AND)' },
        { type: 'output', text: '  \\/ | ∨                Disjunction (OR)' },
        { type: 'output', text: '  ~ | ¬                 Negation (NOT)' },
        { type: 'output', text: '  e | in | ∈            Set Membership' },
        { type: 'output', text: '  =                     Equality' },
        { type: 'output', text: '  {x | P(x)}            Set Comprehension' },
        { type: 'output', text: '' },
        { type: 'output', text: 'Commands & Proof Management:' },
        { type: 'output', text: '  theorem <name> <formula>      Start a new proof' },
        { type: 'output', text: '  assume <name> <formula>       Add a named axiom' },
        { type: 'output', text: '  deff <name>(<args>) := <form> Define a macro' },
        { type: 'output', text: '  show_goal                     Show the current goal state' },
        { type: 'output', text: '  qed                           Finish and commit proof' },
        { type: 'output', text: '  abort                         Abort current proof' },
        { type: 'output', text: '  eval <formula>                Evaluate a formula' },
        { type: 'output', text: '' },
        { type: 'output', text: 'Deduction & Topological Tactics:' },
        { type: 'output', text: '  intro [name]                  Introduce hypothesis' },
        { type: 'output', text: '  exact <name>                  Close matching goal' },
        { type: 'output', text: '  apply <name>                  Apply implication hypothesis' },
        { type: 'output', text: '  split                         Split conjunction into subgoals' },
        { type: 'output', text: '  left / right                  Disjunction left/right branch' },
        { type: 'output', text: '  destruct <name> [n1] [n2]     Break down conjunction' },
        { type: 'output', text: '  rw <name>                     Equality rewrite substitution' },
        { type: 'output', text: '  simp                          DNF and push-negation simplify' },
        { type: 'output', text: '  cut <formula>                 Introduce sub-goal cut' },
        { type: 'output', text: '  have <name> <formula>         Prove sub-goal to context' },
        { type: 'output', text: '  stratify                      Verify weak stratification' },
        { type: 'output', text: '  refl                          Topological equivalence check' },
        { type: 'output', text: '  elevate [name]                Apply T-functor shift (x ↦ x_ι)' },
        { type: 'output', text: '  sc_cut <var>                  Strongly Cantorian bedrock cut' },
        { type: 'output', text: '  collapse_loop                 Contract 0-weight SCC cycles' },
        { type: 'output', text: '  defer                         Cycle active goal to end' },
      ]);
      return;
    }

    setIsEvaluating(true);
    workerRef.current?.postMessage({
      id: Math.random().toString(),
      type: 'REPL_COMMAND',
      rawCmd: trimmed,
      line: trimmed,
    });
  };

  const cancelExecution = () => {
    if (workerRef.current) {
      workerRef.current.terminate();
      workerRef.current = new EvaluationWorker();
      
      const handleMessage = (e: MessageEvent) => {
        setIsEvaluating(false);
        if (e.data.type === 'REPL_UPDATE') {
          if (e.data.output_lines && e.data.output_lines.length > 0) {
            const newLines = e.data.output_lines.map((line: string) => ({ type: 'output' as const, text: line }));
            setHistory(prev => [...prev, ...newLines]);
          }
          if (e.data.error) {
            setHistory(prev => [...prev, { type: 'error', text: e.data.error }]);
          }
          setProofState(e.data.state || null);
        } else if (e.data.type === 'REPL_ERROR') {
          setHistory(prev => [...prev, { type: 'error', text: e.data.error }]);
        }
      };
      workerRef.current.addEventListener('message', handleMessage);

      setHistory(prev => [...prev, { type: 'error', text: '[Execution Cancelled by User]' }]);
      setIsEvaluating(false);
    }
  };

  const resetSession = () => {
    workerRef.current?.postMessage({
      id: Math.random().toString(),
      type: 'REPL_RESET'
    });
    setHistory([
      { type: 'output', text: '=== Monist Interactive Theorem Prover (ITP) ===' },
      { type: 'output', text: 'Type "help" for syntax reference, or click a 1-Click Starter Proof above.' },
      { type: 'output', text: 'Session reset to default.' }
    ]);
    setProofState(null);
    setActiveGoalIndex(0);
    setInputValue('');
  };

  const onSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!inputValue.trim() || isEvaluating) return;
    setHistory(prev => [...prev, { type: 'input', text: `ITP> ${inputValue}` }]);
    executeCommandLine(inputValue);
    setInputValue('');
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    let val = e.target.value;
    val = val
      .replace(/forall/g, '∀')
      .replace(/exists/g, '∃')
      .replace(/<->/g, '↔')
      .replace(/->/g, '→')
      .replace(/\\\/|\/\\/g, (match) => match === '/\\' ? '∧' : '∨')
      .replace(/~/g, '¬')
      .replace(/\s+in\s+/g, ' ∈ ')
      .replace(/\s+subset\s+/g, ' ⊆ ');
    setInputValue(val);
  };

  const insertTactic = (tactic: string) => {
    setInputValue(prev => (prev ? `${prev} ${tactic}` : tactic));
    inputRef.current?.focus({ preventScroll: true });
  };

  const runQuickAction = (cmd: string) => {
    setHistory(prev => [...prev, { type: 'input', text: `ITP> ${cmd}` }]);
    executeCommandLine(cmd);
  };

  const renderFormulaTokens = (formula: string) => {
    const parts = formula.split(/([∀∃∧∨→↔¬∈⊆=()\[\]{}]+|\s+)/g);
    return (
      <span>
        {parts.map((p, idx) => {
          if (!p) return null;
          if (['∧', '∨', '→', '↔'].includes(p)) {
            return <span key={idx} className="tok-connective">{p}</span>;
          }
          if (['∀', '∃', '∈', '⊆'].includes(p)) {
            return <span key={idx} className="tok-quantifier">{p}</span>;
          }
          if (['¬', '~'].includes(p)) {
            return <span key={idx} className="tok-neg">{p}</span>;
          }
          return <span key={idx} className="tok-var">{p}</span>;
        })}
      </span>
    );
  };

  const activeGoal = proofState?.goals && proofState.goals.length > 0 ? proofState.goals[0] : null;

  return (
    <div className="repl-ide-grid">
      {/* Column 1: Terminal & Controls */}
      <div className="editor-sidebar" style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
        
        {/* 1-Click Starter Proofs */}
        <div className="starter-bar">
          <span className="starter-title">Starter Proofs:</span>
          {STARTER_PROOFS.map((st, i) => (
            <button
              key={i}
              type="button"
              className="starter-chip"
              onClick={() => runQuickAction(st.cmd)}
            >
              {st.name}
            </button>
          ))}
        </div>

        {/* Terminal Window */}
        <div className="repl-terminal">
          <div className="repl-history" ref={historyRef}>
            {history.map((entry, i) => (
              <div key={i} className={`repl-line repl-${entry.type}`}>
                {entry.text}
              </div>
            ))}
            <div ref={endOfLogRef} />
          </div>

          {/* Top-Right Scroll Button */}
          <button
            type="button"
            onClick={() => historyRef.current?.scrollBy({ top: -150, behavior: "smooth" })}
            style={{ position: "absolute", top: "15px", right: "12px", background: "transparent", border: "1px solid #ccc", color: "#666", width: "22px", height: "22px", display: "flex", alignItems: "center", justifyContent: "center", cursor: "pointer", fontSize: "11px", borderRadius: "2px", zIndex: 10 }}
            onMouseEnter={e => { e.currentTarget.style.background = "#000"; e.currentTarget.style.color = "#fff"; }}
            onMouseLeave={e => { e.currentTarget.style.background = "transparent"; e.currentTarget.style.color = "#666"; }}
            title="Scroll Up"
          >
            ▲
          </button>

          {/* Bottom-Right Scroll Button */}
          <button
            type="button"
            onClick={() => historyRef.current?.scrollBy({ top: 150, behavior: "smooth" })}
            style={{ position: "absolute", bottom: "15px", right: "12px", background: "transparent", border: "1px solid #ccc", color: "#666", width: "22px", height: "22px", display: "flex", alignItems: "center", justifyContent: "center", cursor: "pointer", fontSize: "11px", borderRadius: "2px", zIndex: 10 }}
            onMouseEnter={e => { e.currentTarget.style.background = "#000"; e.currentTarget.style.color = "#fff"; }}
            onMouseLeave={e => { e.currentTarget.style.background = "transparent"; e.currentTarget.style.color = "#666"; }}
            title="Scroll Down"
          >
            ▼
          </button>

          {/* Quick-Tactic Palette */}
          <div className="tactic-palette">
            {QUICK_TACTICS.map((tac, i) => (
              <button
                key={i}
                type="button"
                className="tactic-chip"
                onClick={() => insertTactic(tac)}
              >
                {tac}
              </button>
            ))}
          </div>

          {/* Prompt Form */}
          <form onSubmit={onSubmit} className="repl-input-form">
            <span className="repl-prompt">ITP&gt;</span>
            <input 
              ref={inputRef}
              type="text" 
              className="repl-input-field" 
              value={inputValue}
              onChange={handleInputChange}
              placeholder="Enter tactic or command (e.g. intro H, apply H, qed)..."
              disabled={isEvaluating}
            />
            {isEvaluating ? (
              <button 
                type="button" 
                onClick={cancelExecution} 
                className="btn-primary rounded-0" 
                style={{ padding: '0.4rem 0.8rem', backgroundColor: '#dc3545', borderColor: '#dc3545', color: 'white', minWidth: '70px' }}
              >
                Cancel
              </button>
            ) : (
              <button 
                type="button" 
                onClick={resetSession} 
                className="btn-secondary rounded-0" 
                style={{ padding: '0.4rem 0.8rem', backgroundColor: '#f0f0f0', borderColor: '#ccc', color: '#333', minWidth: '70px' }}
                title="Reset proof state and session back to default"
              >
                Reset
              </button>
            )}
          </form>
        </div>

        <SyntaxToolkit onInsert={(code) => setInputValue(prev => prev + code)} />
      </div>

      {/* Column 2: Advanced Proof State Infoview */}
      <div className="proof-sidebar">
        <div className="repl-sidebar panel-card" style={{ flex: 1, overflowY: 'auto' }}>
          
          {/* Infoview Header */}
          <div className="infoview-header">
            <h3>Active Proof State</h3>
            {proofState && (
              <span className={`strat-badge ${proofState.is_stratified ? 'stratified' : 'collision'}`}>
                {proofState.is_stratified ? `● STRATIFIED (MCM ${proofState.mcm.toFixed(2)})` : `▲ COLLISION (MCM ${proofState.mcm.toFixed(2)})`}
              </span>
            )}
          </div>

          {/* Proof State Content */}
          <div className="proof-state-box">
            {!proofState || !proofState.goals || proofState.goals.length === 0 ? (
              <div style={{ color: '#666', fontStyle: 'italic', padding: '10px 0' }}>
                No active proof. Type <code>theorem &lt;name&gt; &lt;formula&gt;</code> or click a Starter Proof above.
              </div>
            ) : (
              <div>
                {/* Multi-Goal Navigation Tabs */}
                {proofState.goals.length > 1 && (
                  <div className="goal-tabs">
                    {proofState.goals.map((_, idx) => (
                      <button
                        key={idx}
                        type="button"
                        className={`goal-tab ${idx === activeGoalIndex ? 'active' : ''}`}
                        onClick={() => {
                          if (idx > 0) {
                            runQuickAction('defer');
                          }
                        }}
                      >
                        Goal {idx + 1}{idx === 0 ? ' (Active)' : ''}
                      </button>
                    ))}
                  </div>
                )}

                {/* Hypotheses Context */}
                {activeGoal && (
                  <>
                    <div className="context-section-title">Context &amp; Hypotheses ({activeGoal.ctx.length})</div>
                    {activeGoal.ctx.length === 0 ? (
                      <div style={{ color: '#999', fontSize: '0.8rem', fontStyle: 'italic', marginBottom: '8px' }}>
                        (Empty context)
                      </div>
                    ) : (
                      activeGoal.ctx.map((hyp, i) => {
                        const isVar = hyp.name.length === 1 && !hyp.formula.includes('→') && !hyp.formula.includes('∧');
                        const hasImpl = hyp.formula.includes('→');
                        const hasConj = hyp.formula.includes('∧');
                        const hasEq = hyp.formula.includes('=');
                        const matchesTarget = hyp.formula === activeGoal.target;

                        return (
                          <div key={i} className="hyp-row">
                            <div className="hyp-left">
                              <span className={isVar ? 'tag-var' : 'tag-hyp'}>{isVar ? 'VAR' : 'HYP'}</span>
                              <span className="hyp-name">{hyp.name}</span> : <span>{renderFormulaTokens(hyp.formula)}</span>
                            </div>
                            <div className="hyp-actions">
                              {matchesTarget && (
                                <button type="button" className="hyp-action-btn" onClick={() => runQuickAction(`exact ${hyp.name}`)}>
                                  Exact
                                </button>
                              )}
                              {hasImpl && (
                                <button type="button" className="hyp-action-btn" onClick={() => runQuickAction(`apply ${hyp.name}`)}>
                                  Apply
                                </button>
                              )}
                              {hasConj && (
                                <button type="button" className="hyp-action-btn" onClick={() => runQuickAction(`destruct ${hyp.name} ${hyp.name}_l ${hyp.name}_r`)}>
                                  Destruct
                                </button>
                              )}
                              {hasEq && (
                                <button type="button" className="hyp-action-btn" onClick={() => runQuickAction(`rw ${hyp.name}`)}>
                                  Rewrite
                                </button>
                              )}
                            </div>
                          </div>
                        );
                      })
                    )}

                    {/* Turnstile Divider & Target */}
                    <div className="goal-divider-line">
                      <span className="turnstile">⊢ Target</span>
                    </div>

                    <div className="target-row">
                      {renderFormulaTokens(activeGoal.target)}
                    </div>
                  </>
                )}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

