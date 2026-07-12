import { useState, useEffect } from 'react';
import init, { evaluate_formula, init_panic_hook } from 'monist-wasm';
import './index.css';

export default function App() {
  const [ready, setReady] = useState(false);
  const [query, setQuery] = useState('forall x . x = x');
  const [smtWitness, setSmtWitness] = useState('');
  const [error, setError] = useState('');
  const [stats, setStats] = useState({ isStratified: false, iterations: 0, mcm: 0 });

  useEffect(() => {
    init().then(() => {
      init_panic_hook();
      setReady(true);
    });
  }, []);

  const handleEvaluate = () => {
    try {
      setError('');
      const result = evaluate_formula(query);
      setSmtWitness(result.smt_witness);
      setStats({
        isStratified: result.is_stratified,
        iterations: result.max_k_iterations,
        mcm: result.mcm,
      });
    } catch (e: any) {
      setError(e.toString());
      setSmtWitness('');
    }
  };

  if (!ready) return <div style={{ padding: '2rem', fontFamily: 'Inter, sans-serif' }}>Loading Engine...</div>;

  return (
    <div className="container">
      <header className="header">
        <h1>Monist Engine Console</h1>
        <p>Interactive graph reduction and topological bounds checking.</p>
      </header>

      <div className="query-section">
        <textarea 
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          className="query-input"
          rows={3}
          spellCheck={false}
        />
        <button onClick={handleEvaluate} className="evaluate-btn">Evaluate Topology</button>
        {error && <div className="error-box">{error}</div>}
      </div>

      <div className="split-view">
        <div className="panel">
          <h2>Formal Witness (SMT-LIB)</h2>
          <pre className="smt-output">{smtWitness || 'No evaluation yet.'}</pre>
        </div>
        <div className="panel">
          <h2>Execution Stats</h2>
          {smtWitness ? (
            <ul className="stats-list">
              <li><strong>Stratified:</strong> {stats.isStratified ? 'Yes' : 'No'}</li>
              <li><strong>Max K-Iterations:</strong> {stats.iterations}</li>
              <li><strong>Minimum Cycle Mean (MCM):</strong> {stats.mcm.toFixed(4)}</li>
            </ul>
          ) : (
            <p className="placeholder">Awaiting input.</p>
          )}
        </div>
      </div>
    </div>
  );
}
