import React, { useState, useEffect, useRef } from 'react';
import EvaluationWorker from './worker?worker';
import { SyntaxToolkit } from './SyntaxToolkit';
import './ReplConsole.css';

interface ReplConsoleProps {
  workerRef: React.MutableRefObject<Worker | null>;
  onCommandExecuted?: (cmd: string) => void;
}

export function ReplConsole({ workerRef, onCommandExecuted }: ReplConsoleProps) {
  const [history, setHistory] = useState<{ type: 'input' | 'output' | 'error', text: string }[]>([
    { type: 'output', text: '=== Monist ITP ===' },
    { type: 'output', text: 'Type "help" for a list of commands, or "theorem <name> <formula>" to start.' }
  ]);
  const [inputValue, setInputValue] = useState('');
  const [proofState, setProofState] = useState<any>(null);
  const [isEvaluating, setIsEvaluating] = useState(false);
  const historyRef = useRef<HTMLDivElement>(null);
  const endOfLogRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (!isEvaluating && inputRef.current) {
      inputRef.current.focus();
    }
  }, [isEvaluating]);

  useEffect(() => {
    if (historyRef.current) {
      historyRef.current.scrollTop = historyRef.current.scrollHeight;
    }
  }, [history, proofState]);

  useEffect(() => {
    const handleMessage = (e: MessageEvent) => {
      setIsEvaluating(false);
      if (e.data.type === 'REPL_UPDATE') {
        if (e.data.success) {
           setProofState(e.data.state);
        } else {
           setHistory(prev => [...prev, { type: 'error', text: e.data.error }]);
        }
      } else if (e.data.type === 'REPL_ERROR') {
         setHistory(prev => [...prev, { type: 'error', text: e.data.error }]);
      }
    };

    workerRef.current?.addEventListener('message', handleMessage);
    return () => workerRef.current?.removeEventListener('message', handleMessage);
  }, [workerRef]);

  const handleCommand = (cmdStr: string) => {
    const parts = cmdStr.trim().split(/\s+/);
    const cmd = parts[0];
    const args = parts.slice(1);

    // Notify tutorial system of every command
    if (onCommandExecuted) {
      onCommandExecuted(cmdStr);
    }

    if (cmd === 'help') {
      setHistory(prev => [...prev, 
        { type: 'output', text: 'Logical Operators Syntax:' },
        { type: 'output', text: '  -> | →                Implication' },
        { type: 'output', text: '  <->                   IFF (If and only if)' },
        { type: 'output', text: '  forall | ∀            Universal Quantifier' },
        { type: 'output', text: '  exists | ∃            Existential Quantifier' },
        { type: 'output', text: '  /\\ | & | ∧            Conjunction (AND)' },
        { type: 'output', text: '  \\/ | ∨                Disjunction (OR)' },
        { type: 'output', text: '  ~ | ¬                 Negation (NOT)' },
        { type: 'output', text: '  e | in | ∈            Set Membership (e.g., x e y)' },
        { type: 'output', text: '  =                     Equality' },
        { type: 'output', text: '  <                     Typestate Inequality (e.g., x < y)' },
        { type: 'output', text: '  {x | P(x)}            Set Comprehension' },
        { type: 'output', text: '' },
        { type: 'output', text: 'Session & Proof Management:' },
        { type: 'output', text: '  theorem <name> <formula>      Start a new proof' },
        { type: 'output', text: '  show_goal                     Show the current goal state' },
        { type: 'output', text: '  qed                           Finish proof' },
        { type: 'output', text: '  abort                         Abort current proof' },
        { type: 'output', text: '' },
        { type: 'output', text: 'Global Commands:' },
        { type: 'output', text: '  eval <formula>                Evaluate a formula' },
        { type: 'output', text: '  check_strat <formula>         Run Bellman-Ford on raw geometry' },
        { type: 'output', text: '  assume <name> <formula>       Add a named axiom' },
        { type: 'output', text: '  deff <name>(<args>) := <form> Define a macro' },
        { type: 'output', text: '' },
        { type: 'output', text: 'Logical Tactics:' },
        { type: 'output', text: '  intro [name]                  Introduce a hypothesis or variable' },
        { type: 'output', text: '  exact <name>                  Close goal if it matches hypothesis' },
        { type: 'output', text: '  apply <name>                  Apply a theorem/hypothesis' },
        { type: 'output', text: '  split                         Split a conjunction goal into two' },
        { type: 'output', text: '  left                          Prove left side of a disjunction' },
        { type: 'output', text: '  right                         Prove right side of a disjunction' },
        { type: 'output', text: '  destruct <name> [n1] [n2]     Break down a hypothesis' },
        { type: 'output', text: '  cut <formula>                 Introduce a formula as a sub-goal' },
        { type: 'output', text: '  have <name> <formula>         Prove a sub-goal and add to context' },
        { type: 'output', text: '  focus_hyp <name>              Pull a hypothesis to top of context' },
        { type: 'output', text: '  defer                         Skip current goal to back of queue' },
        { type: 'output', text: '' },
        { type: 'output', text: 'Topological Tactics:' },
        { type: 'output', text: '  stratify                      Weak stratification topological check' },
        { type: 'output', text: '  refl                          DAG topological equivalence check' },
        { type: 'output', text: '  schonfinkel                   SKI combinator extraction' },
        { type: 'output', text: '  step [formula]                Execute geometric evaluation on target or formula' },
      ]);
      return;
    }

    setIsEvaluating(true);
    
    workerRef.current?.postMessage({
      id: Math.random().toString(),
      type: 'REPL_COMMAND',
      rawCmd: cmdStr,
      cmd,
      args
    });
  };

  const cancelExecution = () => {
    if (workerRef.current) {
        workerRef.current.terminate();
        workerRef.current = new EvaluationWorker();
        
        const handleMessage = (e: MessageEvent) => {
          setIsEvaluating(false);
          if (e.data.type === 'REPL_UPDATE') {
            if (e.data.success) {
               setProofState(e.data.state);
            } else {
               setHistory(prev => [...prev, { type: 'error', text: e.data.error }]);
            }
          } else if (e.data.type === 'REPL_ERROR') {
             setHistory(prev => [...prev, { type: 'error', text: e.data.error }]);
          }
        };
        workerRef.current.addEventListener('message', handleMessage);

        setHistory(prev => [...prev, { type: 'error', text: '[Execution Cancelled by User]' }]);
        setIsEvaluating(false);
    }
  };

  const onSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!inputValue.trim() || isEvaluating) return;
    setHistory(prev => [...prev, { type: 'input', text: `ITP> ${inputValue}` }]);
    handleCommand(inputValue);
    setInputValue('');
  };

  return (
    <div className="ide-grid" style={{ marginBottom: '20px', gridTemplateColumns: '2fr 1fr' }}>
      <div className="editor-sidebar" style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
        <div className="repl-terminal" style={{ height: '300px', position: 'relative' }}>
          <div className="repl-history" ref={historyRef} style={{ paddingRight: '30px' }}>
          {history.map((entry, i) => (
            <div key={i} className={`repl-line repl-${entry.type}`}>
              {entry.text}
            </div>
          ))}
          <div ref={endOfLogRef} />
        </div>
        <button type="button" onClick={() => historyRef.current?.scrollBy({ top: -150, behavior: 'smooth' })} style={{ position: 'absolute', top: '10px', right: '10px', background: 'transparent', border: '1px solid #ccc', color: '#666', width: '24px', height: '24px', display: 'flex', alignItems: 'center', justifyContent: 'center', cursor: 'pointer', fontSize: '12px' }} onMouseEnter={e => { e.currentTarget.style.background = '#f5f5f5'; e.currentTarget.style.color = '#000'; }} onMouseLeave={e => { e.currentTarget.style.background = 'transparent'; e.currentTarget.style.color = '#666'; }}>▲</button>
        <button type="button" onClick={() => historyRef.current?.scrollBy({ top: 150, behavior: 'smooth' })} style={{ position: 'absolute', bottom: '45px', right: '10px', background: 'transparent', border: '1px solid #ccc', color: '#666', width: '24px', height: '24px', display: 'flex', alignItems: 'center', justifyContent: 'center', cursor: 'pointer', fontSize: '12px' }} onMouseEnter={e => { e.currentTarget.style.background = '#f5f5f5'; e.currentTarget.style.color = '#000'; }} onMouseLeave={e => { e.currentTarget.style.background = 'transparent'; e.currentTarget.style.color = '#666'; }}>▼</button>

        <form onSubmit={onSubmit} className="repl-input-form">
          <span className="repl-prompt">ITP&gt;</span>
          <input 
            ref={inputRef}
            type="text" 
            className="repl-input" 
            value={inputValue}
            onChange={(e) => {
              let val = e.target.value;
              val = val.replace(/forall/g, '∀').replace(/exists/g, '∃').replace(/<->/g, '↔').replace(/->/g, '→');
              setInputValue(val);
            }}
            disabled={isEvaluating}
            autoFocus
          />
          <button type="button" onClick={cancelExecution} disabled={!isEvaluating} className="btn-primary rounded-0" style={{ padding: '0 1rem', backgroundColor: isEvaluating ? '#dc3545' : '#6c757d', borderColor: isEvaluating ? '#dc3545' : '#6c757d', color: 'white' }}>Cancel</button>
        </form>
      </div>
      
      <SyntaxToolkit onInsert={(code) => setInputValue(prev => prev + code)} />
      </div>

      <div style={{ display: 'flex', flexDirection: 'column', gap: '1rem', height: '100%' }}>
        <div className="repl-sidebar panel-card" style={{ flex: 1, overflowY: 'auto' }}>
          <h3>Active Proof State</h3>
          <div className="proof-state-content" style={{ border: 'none', padding: 0, background: 'transparent' }}>
            {!proofState || !proofState.goals || proofState.goals.length === 0 ? (
            <div className="no-goals">No active goals. Proof complete!</div>
          ) : (
            <div className="goal-view">
              <div className="goal-context">
                {proofState.goals[0].ctx.map((hyp: any, i: number) => (
                  <div key={i} className="hyp-line">
                    <strong>{hyp[0]}</strong> : [Node {hyp[1]}]
                  </div>
                ))}
              </div>
              <div className="goal-divider">----------------------</div>
              <div className="goal-target">
                [Target Node {proofState.goals[0].target}]
              </div>
              {proofState.goals.length > 1 && (
                <div className="pending-goals">
                  + {proofState.goals.length - 1} pending goal(s)
                </div>
              )}
            </div>
          )}
          </div>
        </div>
      </div>
    </div>
  );
}
