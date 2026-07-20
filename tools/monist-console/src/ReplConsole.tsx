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
        { type: 'output', text: 'Commands:' },
        { type: 'output', text: '  theorem <name> <formula> - Start a new proof' },
        { type: 'output', text: '  deff <name>(<args>) := <formula> - Define a macro' },
        { type: 'output', text: '  intro [name] - Introduce hypothesis' },
        { type: 'output', text: '  apply <name> - Apply hypothesis' },
        { type: 'output', text: '  exact <name> - Close goal' },
        { type: 'output', text: '  destruct <name> [n1] [n2] - Destruct hypothesis' },
        { type: 'output', text: '  split / left / right - Manage goals' },
        { type: 'output', text: '' },
        { type: 'output', text: 'Formula Syntax:' },
        { type: 'output', text: '  forall x . P(x)   Universal Quantifier' },
        { type: 'output', text: '  exists x . P(x)   Existential Quantifier' },
        { type: 'output', text: '  /\\                Logical AND (or &)' },
        { type: 'output', text: '  \\/                Logical OR (or | for bar)' },
        { type: 'output', text: '  ->                Implication' },
        { type: 'output', text: '  <->               Biconditional' },
        { type: 'output', text: '  ~ or ¬            Logical NOT' },
        { type: 'output', text: '  =                 Equality' },
        { type: 'output', text: '  in or e           Set Membership' },
        { type: 'output', text: '  { x | P(x) }      Comprehension' },
        { type: 'output', text: '  <                 Strict Less-Than' },
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
