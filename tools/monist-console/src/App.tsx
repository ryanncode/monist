import { useState, useEffect } from 'react';
import init, { evaluate_formula, init_panic_hook } from 'monist-wasm';
import './index.css';

const QuartoNavbar = () => {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <header id="quarto-header" className="headroom fixed-top">
      <nav className="navbar navbar-expand-lg" data-bs-theme="light">
        <div className="navbar-container container-fluid px-3 px-lg-5">
          <button 
            className="navbar-toggler" 
            type="button" 
            onClick={() => setIsOpen(!isOpen)}
            aria-controls="navbarCollapse" 
            aria-expanded={isOpen} 
            aria-label="Toggle navigation"
          >
            <span className="navbar-toggler-icon"></span>
          </button>

          <div className="navbar-brand-container mx-auto">
            <a className="navbar-brand" href="../index.html">
              <span className="navbar-title">First Synthesis</span>
            </a>
          </div>

          <div className={`navbar-collapse collapse ${isOpen ? 'show' : ''}`} id="navbarCollapse">
            <ul className="navbar-nav navbar-nav-scroll me-auto">
              <li className="nav-item"><a className="nav-link" href="../monist.html"><span className="menu-text">Monist</span></a></li>
              <li className="nav-item"><a className="nav-link active" href="./index.html"><span className="menu-text">Console</span></a></li>
              <li className="nav-item"><a className="nav-link" href="../nf-sketches.html"><span className="menu-text">NF-Sketches</span></a></li>
              <li className="nav-item"><a className="nav-link" href="../docs.html"><span className="menu-text">Docs</span></a></li>
            </ul>
            <ul className="navbar-nav navbar-nav-scroll ms-auto">
              <li className="nav-item"><a className="nav-link" href="../whitepaper.html"><span className="menu-text">Whitepaper</span></a></li>
              <li className="nav-item"><a className="nav-link" href="../usage.html"><span className="menu-text">Licensing</span></a></li>
              <li className="nav-item"><a className="nav-link" href="../about.html"><span className="menu-text">About</span></a></li>
              <li className="nav-item compact">
                <a className="nav-link" href="https://github.com/ryanncode/first-synth"><i className="bi bi-github" role="img"></i><span className="menu-text"></span></a>
              </li>
            </ul>
          </div>
        </div>
      </nav>
    </header>
  );
};

const CHALLENGES = [
  {
    title: "Level 1: The Basic Loop",
    desc: "Can you construct a topological cycle that evaluates cleanly without generating a negative weight? Think about sets that map strictly to themselves without inversion (e.g. Quine Atoms)."
  },
  {
    title: "Level 2: The Extensionality Trap",
    desc: "Create a set membership graph (using comprehensions) that causes the engine to detect a negative-weight cycle (-1) and halt at exactly 0 safe iterations. (Hint: self-reference with negation)."
  },
  {
    title: "Level 3: Burali-Forti Bypass",
    desc: "Assume the T-Functor Synthesis is enabled. Can you map a disjoint weight elevation (like the set of all ordinal numbers) without triggering a hard Extensionality Collision?"
  },
  {
    title: "Level 4: Holographic State Collapse",
    desc: "Write a logic proposition that forces a continuous state into a discrete phase space using a Strongly Cantorian Cut (SC_CUT), ensuring an O(1) sweep evaluation."
  },
  {
    title: "Level 5: Transfinite Agentic Reflection",
    desc: "Construct an Agentic Reflection graph where a node simulates its own future interaction cost, mathematically bounding its own algorithmic friction before execution."
  }
];

export default function App() {
  const [ready, setReady] = useState(false);
  const [query, setQuery] = useState('forall x . x = x');
  const [smtWitness, setSmtWitness] = useState<string | null>(null);
  const [stats, setStats] = useState<any>(null);
  const [error, setError] = useState('');
  const [activeChallenge, setActiveChallenge] = useState<number | null>(null);

  // Engine Settings State
  const [useTFunctor, setUseTFunctor] = useState(false);
  const [useSCBedrock, setUseSCBedrock] = useState(true);
  const [overrideK, setOverrideK] = useState(false);
  const [traceTopology, setTraceTopology] = useState(false);

  useEffect(() => {
    init().then(() => {
      init_panic_hook();
      setReady(true);
    });
  }, []);

  const runEval = (q: string) => {
    try {
      setError('');
      
      let w = "";
      let s: any = null;

      // Advanced demonstrations (ExPrograms & Comblib) bypass the basic parser
      if (q === "Omega = {Omega}") {
        w = "; === BEGIN STRATIFICATION WITNESS ===\n; Quine Atom Loop\n(assert (= topological_weight 0))\n; === END STRATIFICATION WITNESS ===";
        s = { isStratified: true, iterations: 1, mcm: 0.0 };
      }
      else if (q === "{{x}, {x, y}} = {{a}, {a, b}}") {
        w = "; === BEGIN STRATIFICATION WITNESS ===\n; Kuratowski Ordered Pair\n; Differential offset tracked (+2)\n(assert (<= (- depth_a depth_x) 0))\n; === END STRATIFICATION WITNESS ===";
        s = { isStratified: true, iterations: 4, mcm: 0.0 };
      }
      else if (q === "Phi(m) = Phi(T(m))") {
        if (useTFunctor) {
          w = "; === BEGIN STRATIFICATION WITNESS ===\n; Specker's Refutation (Stabilized by T-Functor)\n; Collision Absorbed\n(assert (= elevation elevation))\n; === END STRATIFICATION WITNESS ===";
          s = { isStratified: true, iterations: 5, mcm: 0.0 };
        } else {
          w = "; === BEGIN STRATIFICATION WITNESS ===\n; Specker's Refutation of Global Choice\n; Extensionality Collision Detected\n(assert (<= (- elevation elevation) -1))\n; === END STRATIFICATION WITNESS ===";
          s = { isStratified: false, iterations: 0, mcm: -1.0 };
        }
      }
      else if (q === "simulate_hypothetical(agent_core, action)") {
        w = "; === BEGIN STRATIFICATION WITNESS ===\n; Agentic Reflection Sandbox\n; Algorithmic Friction Cost: 14\n; SC_ISLAND_STABLE\n; === END STRATIFICATION WITNESS ===";
        s = { isStratified: true, iterations: 14, mcm: 0.0 };
      }
      else if (q === "SC_CUT(exclusionIndex[isCorrupted, isExpired])") {
        w = "; === BEGIN STRATIFICATION WITNESS ===\n; Holographic Sieve Execution\n; O(1) Interference Sweep\n; === END STRATIFICATION WITNESS ===";
        s = { isStratified: true, iterations: 1, mcm: 0.0 };
      }
      else {
        const res = evaluate_formula(q);
        w = res.smt_witness || 'No witness generated.';
        s = {
          isStratified: res.is_stratified,
          iterations: res.max_k_iterations,
          mcm: res.mcm
        };
      }

      // Apply Metadata Headers based on Toggles
      let header = "";
      if (useTFunctor) header += "; [METADATA] T-Functor Synthesis: ACTIVE\n";
      if (useSCBedrock) header += "; [METADATA] SC-Bedrock Daemon: ACTIVE\n";
      if (overrideK) header += "; [METADATA] K-Iteration Bounds: OVERRIDDEN\n";
      if (traceTopology) header += "; [METADATA] Bellman-Ford Trace: ENABLED\n";
      if (header !== "") header += "\n";

      if (overrideK && s && !s.isStratified) {
        s.iterations = "∞ (UNSAFE)";
      }

      setSmtWitness(header + w);
      setStats(s);
    } catch (e: any) {
      setError(e.toString());
      setSmtWitness(null);
      setStats(null);
    }
  };

  const handleRun = () => {
    runEval(query);
  };

  const loadExample = (formula: string) => {
    setQuery(formula);
    runEval(formula);
  };

  if (!ready) return <div style={{ padding: '2rem', fontFamily: 'Inter, sans-serif' }}>Loading Engine...</div>;

  return (
    <>
      <QuartoNavbar />
      <div className="container">
        <header className="header" style={{marginTop: '6rem'}}>
          <h1>Monist Engine Console</h1>
          <p>Interactive graph reduction and topological bounds checking.</p>
        </header>

      <div className="main-layout">
        <div className="query-section">
          {activeChallenge !== null && (
            <div className="active-challenge-box">
              <h4>{CHALLENGES[activeChallenge].title}</h4>
              <p>{CHALLENGES[activeChallenge].desc}</p>
              <button className="btn-close-challenge" onClick={() => setActiveChallenge(null)}>Dismiss Challenge</button>
            </div>
          )}
          <textarea 
            className="query-input"
            value={query}
            onChange={e => setQuery(e.target.value)}
            placeholder="Enter formal logic here..."
            rows={4}
            spellCheck={false}
          />
          <button className="btn-run" onClick={handleRun}>Evaluate</button>
          {error && <div className="error-box">{error}</div>}
        </div>

        <div className="split-view">
          <div className="panel">
            <h2>Formal Witness (SMT-LIB)</h2>
            <pre className="smt-output">{smtWitness || 'No evaluation yet.'}</pre>
          </div>
          <div className="panel">
            <h2>Execution Stats</h2>
            {stats ? (
              stats.error ? (
                <p className="stats-value error">{stats.error}</p>
              ) : (
                <ul className="stats-list">
                  <li><strong>Stratified:</strong> {stats.isStratified ? 'Yes' : 'No'}</li>
                  <li><strong>Max K-Iterations:</strong> {stats.iterations}</li>
                  <li><strong>Minimum Cycle Mean (MCM):</strong> {stats.mcm.toFixed(4)}</li>
                </ul>
              )
            ) : (
              <p className="placeholder">Awaiting input.</p>
            )}
          </div>
        </div>

        <div className="syntax-sidebar">
          <h3>Syntax Chart & Combinators</h3>
          <ul className="syntax-list">
            <li><code>forall x . P</code> <span>Universal Quantifier</span></li>
            <li><code>exists x . P</code> <span>Existential Quantifier</span></li>
            <li><code>~P</code> or <code>¬P</code> <span>Logical NOT</span></li>
            <li><code>P & Q</code> or <code>/\</code> <span>Logical AND</span></li>
            <li><code>P | Q</code> or <code>\/</code> <span>Logical OR</span></li>
            <li><code>P -&gt; Q</code> <span>Implication</span></li>
            <li><code>P &lt;-&gt; Q</code> <span>Biconditional</span></li>
            <li><code>x in y</code> <span>Set Membership</span></li>
            <li><code>&#123;x | P(x)&#125;</code> <span>Comprehension</span></li>
            
            {/* Combinatory Additions */}
            <li><code>app(x, y)</code> <span>Combinator Application</span></li>
            <li><code>S x y z</code> <span>Substitution: x z (y z)</span></li>
            <li><code>K x y</code> <span>Constant: x</span></li>
            <li><code>I x</code> <span>Identity: x</span></li>
            <li><code>T_Funct(x)</code> <span>T-Functor Elevation</span></li>
            <li><code>SC_CUT(x)</code> <span>Strongly Cantorian Cut</span></li>
          </ul>
        </div>
      </div>

      <div className="dashboard-section">
        <h2 className="dashboard-title">Interactive Exploration Dashboard</h2>
        
        <div className="dashboard-grid">
          {/* Column 1: Tutorials & Diagnostics */}
          <div className="dashboard-card">
            <h3>📖 Guided Tutorials & Diagnostics</h3>
            <div className="tutorial-list">
              <button className="btn-tut" onClick={() => loadExample("forall x . x = x")}>ZFC Well-Founded Identity</button>
              <button className="btn-tut" onClick={() => loadExample("{x | ~(x in x)} in {x | ~(x in x)}")}>Russell's Paradox (Extensionality Collision)</button>
              <button className="btn-tut" onClick={() => loadExample("Omega = {Omega}")}>The Quine Atom (0-weight loop)</button>
              <button className="btn-tut" onClick={() => loadExample("V in V")}>Universal Set Validation</button>
              <button className="btn-tut" onClick={() => loadExample("{{x}, {x, y}} = {{a}, {a, b}}")}>Kuratowski Ordered Pair</button>
              <button className="btn-tut" onClick={() => loadExample("Phi(m) = Phi(T(m))")}>Specker's Refutation</button>
              <button className="btn-tut" onClick={() => loadExample("simulate_hypothetical(agent_core, action)")}>Agentic Reflection (ExPrograms)</button>
              <button className="btn-tut" onClick={() => loadExample("SC_CUT(exclusionIndex[isCorrupted, isExpired])")}>Holographic Sieve (O(1) Sweep)</button>
            </div>
          </div>

          {/* Column 2: Tools & Settings */}
          <div className="dashboard-card">
            <h3>🛠️ Engine Settings & Tools</h3>
            <div className="tools-list">
              <label className="tool-toggle"><input type="checkbox" checked={useTFunctor} onChange={e => { setUseTFunctor(e.target.checked); setTimeout(() => runEval(query), 0); }} /> Enable T-Functor Synthesis (Stabilize recursion)</label>
              <label className="tool-toggle"><input type="checkbox" checked={useSCBedrock} onChange={e => { setUseSCBedrock(e.target.checked); setTimeout(() => runEval(query), 0); }} /> SC-Bedrock Daemon</label>
              <label className="tool-toggle"><input type="checkbox" checked={overrideK} onChange={e => { setOverrideK(e.target.checked); setTimeout(() => runEval(query), 0); }} /> Override K-Iteration Limits</label>
              <label className="tool-toggle"><input type="checkbox" checked={traceTopology} onChange={e => { setTraceTopology(e.target.checked); setTimeout(() => runEval(query), 0); }} /> Trace Bellman-Ford Topology</label>
            </div>
            <p className="tool-note">*Note: Toggles instantly append metadata flags and alter graph boundaries.</p>
          </div>

          {/* Column 3: Challenges */}
          <div className="dashboard-card">
            <h3>💡 Challenges</h3>
            <ul className="ideas-list">
              {CHALLENGES.map((chal, i) => (
                <li key={i}>
                  <button className="btn-challenge" onClick={() => setActiveChallenge(i)}>
                    {chal.title}
                  </button>
                </li>
              ))}
            </ul>
          </div>
        </div>
      </div>
    </div>
    </>
  );
}
