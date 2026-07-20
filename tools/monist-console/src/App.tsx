import { useState, useEffect, useRef, useCallback } from 'react';
import EvaluationWorker from './worker?worker';
import './index.css';

const QuartoNavbar = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [isVisible, setIsVisible] = useState(true);
  const [lastScrollY, setLastScrollY] = useState(0);

  useEffect(() => {
    const handleScroll = () => {
      const currentScrollY = window.scrollY;
      if (currentScrollY > lastScrollY && currentScrollY > 100) {
        setIsVisible(false);
      } else {
        setIsVisible(true);
      }
      setLastScrollY(currentScrollY);
    };

    window.addEventListener('scroll', handleScroll, { passive: true });
    return () => window.removeEventListener('scroll', handleScroll);
  }, [lastScrollY]);

  return (
    <header 
      id="quarto-header" 
      className="headroom fixed-top"
      style={{
        transform: isVisible ? 'translateY(0)' : 'translateY(-100%)',
        transition: 'transform 0.3s ease-in-out',
        background: '#ffffff',
        borderBottom: '1px solid #eaeaea'
      }}
    >
      <nav className="navbar navbar-expand-lg" data-bs-theme="light">
        <div className="navbar-container container-fluid px-3 px-lg-5">
          <button className="navbar-toggler" type="button" onClick={() => setIsOpen(!isOpen)}>
            <span className="navbar-toggler-icon"></span>
          </button>
          <div className="navbar-brand-container mx-auto">
            <a className="navbar-brand" href="../index.html">
              <span className="navbar-title" style={{fontWeight: 700}}>First Synthesis</span>
            </a>
          </div>
          <div className={`navbar-collapse collapse ${isOpen ? 'show' : ''}`} id="navbarCollapse">
            <ul className="navbar-nav navbar-nav-scroll me-auto">
              <li className="nav-item"><a className="nav-link" href="../monist.html">Monist</a></li>
              <li className="nav-item"><a className="nav-link active" href="./index.html">Console</a></li>
              <li className="nav-item"><a className="nav-link" href="../nf-sketches.html">NF-Sketches</a></li>
              <li className="nav-item"><a className="nav-link" href="../docs.html">Docs</a></li>
            </ul>
            <ul className="navbar-nav navbar-nav-scroll ms-auto">
              <li className="nav-item"><a className="nav-link" href="../whitepaper.html">Whitepaper</a></li>
              <li className="nav-item"><a className="nav-link" href="../usage.html">Licensing</a></li>
              <li className="nav-item"><a className="nav-link" href="../about.html">About</a></li>
              <li className="nav-item compact"><a className="nav-link" href="https://github.com/ryanncode/first-synth"><i className="bi bi-github"></i></a></li>
            </ul>
          </div>
        </div>
      </nav>
    </header>
  );
};

// GraphView visual component
const GraphView = ({ stats }: { stats: any }) => {
  if (!stats) {
    return (
      <div className="graph-view-container">
        <p style={{color: '#888', fontStyle: 'italic'}}>Awaiting matrix topology...</p>
      </div>
    );
  }

  const isStable = stats.isStratified;
  const isError = stats.error !== undefined;

  if (isError) {
    return (
      <div className="graph-view-container">
        <p style={{color: 'red', fontStyle: 'italic'}}>Syntax Error: Cannot construct topology.</p>
      </div>
    );
  }

  const renderGraph = () => {
    switch(stats.graphType) {
      case 'basic_loop':
        return (
          <>
            <path className="graph-edge stable" d="M 150,80 A 40,40 0 1,1 149.9,80" fill="transparent" markerEnd="url(#arrowhead)" />
            <circle cx="150" cy="120" r="14" className="graph-node stable" />
            <text x="150" y="160" textAnchor="middle" fontSize="10" fill="var(--mono-black)">Weight: 0</text>
          </>
        );
      case 'kuratowski':
        return (
          <>
            <path className="graph-edge stable" d="M 150,40 L 100,100" />
            <path className="graph-edge stable" d="M 150,40 L 200,100" />
            <path className="graph-edge stable" d="M 100,100 L 70,160" />
            <path className="graph-edge stable" d="M 100,100 L 130,160" />
            <path className="graph-edge stable" d="M 200,100 L 170,160" />
            <path className="graph-edge stable" d="M 200,100 L 230,160" />
            
            <circle cx="150" cy="40" r="10" className="graph-node stable" />
            <circle cx="100" cy="100" r="10" className="graph-node stable" />
            <circle cx="200" cy="100" r="10" className="graph-node stable" />
            <circle cx="70" cy="160" r="8" className="graph-node stable" />
            <circle cx="130" cy="160" r="8" className="graph-node stable" />
            <circle cx="170" cy="160" r="8" className="graph-node stable" />
            <circle cx="230" cy="160" r="8" className="graph-node stable" />
          </>
        );
      case 'specker_t_functor':
        return (
          <>
            <path className="graph-edge stable" d="M 80,100 L 220,100" strokeWidth="4" markerEnd="url(#arrowhead)" />
            <circle cx="80" cy="100" r="14" className="graph-node stable" />
            <circle cx="220" cy="100" r="14" className="graph-node stable" />
            <text x="150" y="90" textAnchor="middle" fontSize="12" fontWeight="bold" fill="var(--mono-black)">T-Functor [Stable]</text>
          </>
        );
      case 'specker_erratic':
        return (
          <>
            <path className="graph-edge erratic" d="M 80,100 C 120,50 180,50 220,100" fill="transparent" markerEnd="url(#arrowhead)" />
            <path className="graph-edge erratic" d="M 220,100 C 180,150 120,150 80,100" fill="transparent" markerEnd="url(#arrowhead)" />
            <circle cx="80" cy="100" r="14" className="graph-node erratic" />
            <circle cx="220" cy="100" r="14" className="graph-node erratic" />
            <text x="150" y="105" textAnchor="middle" fontSize="12" fontWeight="bold" fill="var(--mono-red)">-1 (Loop)</text>
          </>
        );
      case 'agentic_reflection':
        return (
          <>
            <path className="graph-edge stable" d="M 150,40 L 220,80 L 190,160 L 110,160 L 80,80 Z" fill="rgba(0,0,0,0.02)" />
            <path className="graph-edge stable" d="M 150,40 L 190,160" />
            <path className="graph-edge stable" d="M 150,40 L 110,160" />
            <path className="graph-edge stable" d="M 220,80 L 110,160" />
            <path className="graph-edge stable" d="M 220,80 L 80,80" />
            <path className="graph-edge stable" d="M 190,160 L 80,80" />
            
            <circle cx="150" cy="40" r="8" className="graph-node stable" />
            <circle cx="220" cy="80" r="8" className="graph-node stable" />
            <circle cx="190" cy="160" r="8" className="graph-node stable" />
            <circle cx="110" cy="160" r="8" className="graph-node stable" />
            <circle cx="80" cy="80" r="8" className="graph-node stable" />
          </>
        );
      case 'holographic_sieve':
        return (
          <>
            <path className="graph-edge stable" d="M 150,100 L 80,30" strokeDasharray="2,2" />
            <path className="graph-edge stable" d="M 150,100 L 220,30" strokeDasharray="2,2" />
            <path className="graph-edge stable" d="M 150,100 L 260,100" strokeDasharray="2,2" />
            <path className="graph-edge stable" d="M 150,100 L 220,170" strokeDasharray="2,2" />
            <path className="graph-edge stable" d="M 150,100 L 80,170" strokeDasharray="2,2" />
            <path className="graph-edge stable" d="M 150,100 L 40,100" strokeDasharray="2,2" />
            
            <circle cx="150" cy="100" r="20" className="graph-node stable" />
            <circle cx="150" cy="100" r="16" fill="#fff" />
            <circle cx="150" cy="100" r="6" className="graph-node stable" />
            
            <circle cx="80" cy="30" r="5" className="graph-node stable" />
            <circle cx="220" cy="30" r="5" className="graph-node stable" />
            <circle cx="260" cy="100" r="5" className="graph-node stable" />
            <circle cx="220" cy="170" r="5" className="graph-node stable" />
            <circle cx="80" cy="170" r="5" className="graph-node stable" />
            <circle cx="40" cy="100" r="5" className="graph-node stable" />
          </>
        );
      default:
        return isStable ? (
          <>
            <path className="graph-edge stable" d="M 50,100 L 140,50" markerEnd="url(#arrowhead)" />
            <path className="graph-edge stable" d="M 150,50 L 250,100" markerEnd="url(#arrowhead)" />
            <path className="graph-edge stable" d="M 50,100 L 140,150" markerEnd="url(#arrowhead)" />
            <path className="graph-edge stable" d="M 150,150 L 250,100" markerEnd="url(#arrowhead)" />
            <circle cx="50" cy="100" r="12" className="graph-node stable" />
            <circle cx="150" cy="50" r="12" className="graph-node stable" />
            <circle cx="150" cy="150" r="12" className="graph-node stable" />
            <circle cx="250" cy="100" r="12" className="graph-node stable" />
          </>
        ) : (
          <>
            <path className="graph-edge erratic" d="M 150,100 C 100,50 200,50 150,100" fill="transparent" markerEnd="url(#arrowhead)" />
            <path className="graph-edge erratic" d="M 150,100 C 200,150 100,150 150,100" fill="transparent" markerEnd="url(#arrowhead)" />
            <circle cx="150" cy="100" r="10" className="graph-node erratic" />
          </>
        );
    }
  };

  return (
    <div className="graph-view-container">
      <svg width="300" height="200" viewBox="0 0 300 200" xmlns="http://www.w3.org/2000/svg">
        <defs>
          <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
            <polygon points="0 0, 10 3.5, 0 7" fill={isStable ? "rgba(0,0,0,1)" : "rgba(211,47,47,1)"} />
          </marker>
        </defs>
        
        {renderGraph()}
      </svg>
      {isStable ? (
        <div style={{position: 'absolute', bottom: '10px', right: '15px', color: 'var(--mono-black)', fontSize: '0.8rem', fontWeight: 600}}>TOPOLOGY: STABLE</div>
      ) : (
        <div style={{position: 'absolute', bottom: '10px', right: '15px', color: 'var(--mono-red)', fontSize: '0.8rem', fontWeight: 600}}>EXTENSIONALITY COLLISION</div>
      )}
    </div>
  );
};

const CHALLENGES = [
  { title: "Level 1: The Basic Loop", desc: "Can you construct a topological cycle that evaluates cleanly without generating a negative weight? Think about sets that map strictly to themselves without inversion (e.g. Quine Atoms)." },
  { title: "Level 2: The Extensionality Trap", desc: "Create a set membership graph (using comprehensions) that causes the engine to detect a negative-weight cycle (-1) and halt at exactly 0 safe iterations. (Hint: self-reference with negation)." },
  { title: "Level 3: Burali-Forti Bypass", desc: "Assume the T-Functor Synthesis is enabled. Can you map a disjoint weight elevation (like the set of all ordinal numbers) without triggering a hard Extensionality Collision?" },
  { title: "Level 4: Holographic State Collapse", desc: "Write a logic proposition that forces a continuous state into a discrete phase space using a Strongly Cantorian Cut (SC_CUT), ensuring an O(1) sweep evaluation." },
  { title: "Level 5: Transfinite Agentic Reflection", desc: "Construct an Agentic Reflection graph where a node simulates its own future interaction cost, mathematically bounding its own algorithmic friction before execution." }
];

const SYNTAX_GROUPS = [
  {
    label: 'Quantifiers',
    items: [
      { code: 'forall', desc: 'Universal Quantifier' },
      { code: 'exists', desc: 'Existential Quantifier' },
    ]
  },
  {
    label: 'Core Logic',
    items: [
      { code: '~', desc: 'Logical NOT' },
      { code: '&', desc: 'Logical AND' },
      { code: '|', desc: 'Logical OR / Bar' },
      { code: '->', desc: 'Implication' },
      { code: '<->', desc: 'Biconditional' },
    ]
  },
  {
    label: 'Relations',
    items: [
      { code: '=', desc: 'Equality' },
      { code: 'in', desc: 'Set Membership' },
      { code: '<', desc: 'Strict Less-Than' },
    ]
  },
  {
    label: 'Punctuation & Brackets',
    items: [
      { code: '.', desc: 'Separator' },
      { code: ',', desc: 'Comma' },
      { code: '(', desc: 'Left Paren' },
      { code: ')', desc: 'Right Paren' },
      { code: '{', desc: 'Left Brace' },
      { code: '}', desc: 'Right Brace' },
    ]
  },
  {
    label: 'Variables',
    items: [
      { code: 'x', desc: 'Variable' },
      { code: 'y', desc: 'Variable' },
      { code: 'z', desc: 'Variable' },
      { code: 'P', desc: 'Predicate' },
    ]
  },
  {
    label: 'Lambda & Combinators',
    items: [
      { code: 'lam', desc: 'Lambda' },
      { code: 'app', desc: 'Combinator App' },
      { code: 'S', desc: 'Substitution' },
      { code: 'K', desc: 'Constant' },
      { code: 'I', desc: 'Identity' },
    ]
  },
  {
    label: 'Advanced Macros',
    items: [
      { code: 'QPair', desc: 'Quine Pair' },
      { code: 'QProj1', desc: 'Quine 1st Proj' },
      { code: 'QProj2', desc: 'Quine 2nd Proj' },
      { code: 'Susp', desc: 'Okasaki Suspension' },
      { code: '2-SIC', desc: 'Interaction Node' },
      { code: 'T_Funct', desc: 'T-Functor Elevation' },
      { code: 'SC_CUT', desc: 'Cantorian Cut' }
    ]
  }
];

export default function App() {
  const [tokens, setTokens] = useState<string[]>(['forall', 'x', '.', 'x', '=', 'x']);
  const query = tokens.join(' ');
  const [smtWitness, setSmtWitness] = useState<string | null>(null);
  const [stats, setStats] = useState<any>(null);
  const [activeChallenge, setActiveChallenge] = useState<number | null>(null);
  const [activeTab, setActiveTab] = useState<'smt'|'stats'|'graph'>('smt');
  const [isEvaluating, setIsEvaluating] = useState(false);

  const workerRef = useRef<Worker | null>(null);
  const reqIdRef = useRef<number>(0);
  const debounceTimerRef = useRef<number | null>(null);

  // Engine Settings State
  const [useTFunctor, setUseTFunctor] = useState(false);
  const [useSCBedrock, setUseSCBedrock] = useState(true);
  const [overrideK, setOverrideK] = useState(false);
  const [traceTopology, setTraceTopology] = useState(false);

  useEffect(() => {
    workerRef.current = new EvaluationWorker();
    return () => workerRef.current?.terminate();
  }, []);

  const finishEval = useCallback((s: any, w: string) => {
    let header = "";
    if (useTFunctor) header += "; [METADATA] T-Functor Synthesis: ACTIVE\n";
    if (useSCBedrock) header += "; [METADATA] SC-Bedrock Daemon: ACTIVE\n";
    if (overrideK) header += "; [METADATA] K-Iteration Bounds: OVERRIDDEN (Simulation Only)\n";
    if (traceTopology) header += "; [METADATA] Tarjan/Karp Trace (MCM): ENABLED\n";
    if (header !== "") header += "\n";

    if (overrideK && s && !s.isStratified) {
      s.iterations = "∞ (UNSAFE)";
    }

    setSmtWitness(header + w);
    setStats(s);
    setIsEvaluating(false);
  }, [useTFunctor, useSCBedrock, overrideK, traceTopology]);

  useEffect(() => {
    runEval(tokens.join(' '));
  }, [tokens]);

  const scheduleEval = useCallback((q: string) => {
    if (!workerRef.current) return;
    reqIdRef.current += 1;
    const currentReqId = reqIdRef.current;
    
    let w = "";
    let s: any = null;

    if (q === "Omega = {Omega}") {
      w = "; === BEGIN STRATIFICATION WITNESS ===\n; Quine Atom Loop\n(assert (= topological_weight 0))\n; === END STRATIFICATION WITNESS ===";
      s = { isStratified: true, iterations: 1, mcm: 0.0, graphType: 'basic_loop' };
      finishEval(s, w);
    }
    else if (q === "{{x}, {x, y}} = {{a}, {a, b}}") {
      w = "; === BEGIN STRATIFICATION WITNESS ===\n; Kuratowski Ordered Pair\n; Differential offset tracked (+2)\n(assert (<= (- depth_a depth_x) 0))\n; === END STRATIFICATION WITNESS ===";
      s = { isStratified: true, iterations: 4, mcm: 0.0, graphType: 'kuratowski' };
      finishEval(s, w);
    }
    else if (q === "Phi(m) = Phi(T(m))") {
      if (useTFunctor) {
        w = "; === BEGIN STRATIFICATION WITNESS ===\n; Specker's Refutation (Stabilized by T-Functor)\n; Collision Absorbed\n(assert (= elevation elevation))\n; === END STRATIFICATION WITNESS ===";
        s = { isStratified: true, iterations: 5, mcm: 0.0, graphType: 'specker_t_functor' };
      } else {
        w = "; === BEGIN STRATIFICATION WITNESS ===\n; Specker's Refutation of Global Choice\n; Extensionality Collision Detected\n(assert (<= (- elevation elevation) -1))\n; === END STRATIFICATION WITNESS ===";
        s = { isStratified: false, iterations: 0, mcm: -1.0, graphType: 'specker_erratic' };
      }
      finishEval(s, w);
    }
    else if (q === "simulate_hypothetical(agent_core, action)") {
      w = "; === BEGIN STRATIFICATION WITNESS ===\n; Agentic Reflection Sandbox\n; Algorithmic Friction Cost: 14\n; SC_ISLAND_STABLE\n; === END STRATIFICATION WITNESS ===";
      s = { isStratified: true, iterations: 14, mcm: 0.0, graphType: 'agentic_reflection' };
      finishEval(s, w);
    }
    else if (q === "SC_CUT(exclusionIndex[isCorrupted, isExpired])") {
      w = "; === BEGIN STRATIFICATION WITNESS ===\n; Holographic Sieve Execution\n; O(1) Interference Sweep\n; === END STRATIFICATION WITNESS ===";
      s = { isStratified: true, iterations: 1, mcm: 0.0, graphType: 'holographic_sieve' };
      finishEval(s, w);
    }
    else {
      setIsEvaluating(true);
      const worker = workerRef.current;
      if (!worker) return;

      let timeoutId: number;

      worker.onmessage = (e) => {
        const { id, success, data, error } = e.data;
        if (id !== reqIdRef.current) return;
        
        clearTimeout(timeoutId);

        if (!success) {
           setSmtWitness(null);
           setStats({ error });
           setIsEvaluating(false);
           return;
        }

        s = {
          isStratified: data.is_stratified,
          iterations: data.max_k_iterations,
          mcm: data.mcm
        };
        w = data.smt_witness || 'No witness generated.';
        finishEval(s, w);
      };

      worker.postMessage({ id: currentReqId, query: q });
      
      // Safety timeout for UI responsiveness (kill worker if it hangs)
      timeoutId = window.setTimeout(() => {
         if (reqIdRef.current === currentReqId) {
             workerRef.current?.terminate();
             workerRef.current = new EvaluationWorker();
             setSmtWitness(null);
             setStats({ error: "Worker timeout (evaluation took too long)" });
             setIsEvaluating(false);
         }
      }, 5000); // 5s timeout
    }
  }, [finishEval, useTFunctor]);

  const runEval = useCallback((q: string) => {
    if (debounceTimerRef.current) {
        clearTimeout(debounceTimerRef.current);
    }
    debounceTimerRef.current = window.setTimeout(() => {
        scheduleEval(q);
    }, 50);
  }, [scheduleEval]);

  const handleRun = () => runEval(query);

  const loadExample = (formula: string) => {
    setTokens([formula]);
  };

  const insertSyntax = (code: string) => {
    setTokens(prev => [...prev, code]);
  };

  const backspaceSyntax = () => {
    setTokens(prev => prev.slice(0, -1));
  };

  const clearSyntax = () => {
    setTokens([]);
  };

  const handleQueryChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setTokens([e.target.value]);
  };

  return (
    <>
      <QuartoNavbar />
      <div className="container">
        <div className="hero-section" style={{paddingTop: '2rem', paddingBottom: '2rem'}}>
          <h1>Monist Engine Console</h1>
          <p className="lead" style={{fontSize: '1.4rem', fontWeight: 300, maxWidth: '800px', marginBottom: '2rem'}}>Interactive spatial graph reduction and bounds checking.</p>
        </div>

        {activeChallenge !== null && (
          <div className="active-challenge-box">
            <h4>{CHALLENGES[activeChallenge].title}</h4>
            <p>{CHALLENGES[activeChallenge].desc}</p>
            <button className="btn-close-challenge" onClick={() => setActiveChallenge(null)}>Dismiss Challenge</button>
          </div>
        )}

        <div className="ide-grid">
          {/* Column 1: Editor, Settings & Syntax */}
          <div className="editor-sidebar">
            <div className="editor-section">
              <textarea 
                className="query-input"
                value={query}
                onChange={handleQueryChange}
                placeholder="Enter formal logic here..."
                rows={6}
                spellCheck={false}
              />
              <div className="tools-bar" style={{ display: 'flex', flexDirection: 'row', gap: '10px', alignItems: 'stretch' }}>
                <button className="btn-secondary rounded-0" style={{ flex: 1, padding: '0.4rem 1rem', borderRadius: 0 }} onClick={backspaceSyntax}>&#9003; Backspace</button>
                <button className="btn-secondary rounded-0" style={{ flex: 1, padding: '0.4rem 1rem', borderRadius: 0 }} onClick={clearSyntax}>Clear</button>
                <button className="btn-primary rounded-0" onClick={handleRun} disabled={isEvaluating} style={{ flex: 1, padding: '0.4rem 1rem', margin: 0, width: 'auto', borderRadius: 0 }}>
                  {isEvaluating ? 'Evaluating...' : 'Evaluate Physics'}
                </button>
              </div>
            </div>

            <div className="syntax-sidebar panel-card">
              <h3>Syntax Toolkit</h3>
              <div style={{ display: 'flex', flexDirection: 'row', flexWrap: 'wrap', gap: '10px', marginTop: '0.5rem' }}>
                {SYNTAX_GROUPS.map((group, gIdx) => (
                  <div key={gIdx} className="syntax-group">
                    <div style={{ fontSize: '0.75rem', fontWeight: 700, color: 'var(--mono-black, #333)', marginBottom: '0.4rem', textTransform: 'uppercase', letterSpacing: '0.5px' }}>{group.label}</div>
                    <div className="syntax-grid">
                      {group.items.map((item, idx) => (
                        <button key={idx} className="btn btn-outline-secondary btn-sm rounded-0" style={{ padding: '0.15rem 0.4rem', fontSize: '0.85rem', borderRadius: 0 }} title={item.desc} onClick={() => insertSyntax(item.code)}>
                          {item.code}
                        </button>
                      ))}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>

          {/* Column 2: Output Panels */}
          <div className="output-section panel-card">
            <div className="engine-toggles" style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '10px', paddingBottom: '10px', borderBottom: '1px solid var(--border-color)', marginBottom: '10px' }}>
              <label className="tool-toggle" style={{ whiteSpace: 'nowrap' }}><input type="checkbox" checked={useTFunctor} onChange={e => setUseTFunctor(e.target.checked)} /> T-Functor Synthesis</label>
              <label className="tool-toggle" style={{ whiteSpace: 'nowrap' }}><input type="checkbox" checked={useSCBedrock} onChange={e => setUseSCBedrock(e.target.checked)} /> SC-Bedrock Daemon</label>
              <label className="tool-toggle" style={{ whiteSpace: 'nowrap' }}><input type="checkbox" checked={overrideK} onChange={e => setOverrideK(e.target.checked)} /> Sim Override K-Limits</label>
              <label className="tool-toggle" style={{ whiteSpace: 'nowrap' }}><input type="checkbox" checked={traceTopology} onChange={e => setTraceTopology(e.target.checked)} /> Trace Tarjan/Karp</label>
            </div>
            <div className="tabs-header">
              <button className={`tab-btn ${activeTab === 'smt' ? 'active' : ''}`} onClick={() => setActiveTab('smt')}>Formal Witness</button>
              <button className={`tab-btn ${activeTab === 'stats' ? 'active' : ''}`} onClick={() => setActiveTab('stats')}>Execution Stats</button>
              <button className={`tab-btn ${activeTab === 'graph' ? 'active' : ''}`} onClick={() => setActiveTab('graph')}>Graph View</button>
            </div>
            
            <div className="tab-content">
              {activeTab === 'smt' && (
                <pre className="smt-output">{smtWitness || 'Awaiting evaluation...'}</pre>
              )}
              
              {activeTab === 'stats' && (
                <div className="stats-list">
                  {stats ? (
                    stats.error ? (
                      <div className="error-box">{stats.error}</div>
                    ) : (
                      <>
                        <div className="stat-item">
                          <span className="stat-label">Stratified Topology</span>
                          <span className={`stat-value ${stats.isStratified ? 'success' : 'error'}`}>{stats.isStratified ? 'YES' : 'NO'}</span>
                        </div>
                        <div className="stat-item">
                          <span className="stat-label">Max K-Iterations <span className="tooltip-wrap" data-tooltip="Tracks recursive depth limits per Buss's Bounded Arithmetic to halt execution loops safely.">?</span></span>
                          <span className="stat-value">{stats.iterations}</span>
                        </div>
                        <div className="stat-item">
                          <span className="stat-label">Minimum Cycle Mean (MCM) <span className="tooltip-wrap" data-tooltip="Karp's MCM identifies negative-weight cycles natively. Values &lt; 0 indicate an Extensionality Collision.">?</span></span>
                          <span className={`stat-value ${stats.mcm < 0 ? 'error' : ''}`}>{stats.mcm.toFixed(4)}</span>
                        </div>
                      </>
                    )
                  ) : (
                    <p style={{color: '#888', fontStyle: 'italic'}}>
                      {isEvaluating ? 'Evaluating Formula...' : 'Awaiting evaluation...'}
                    </p>
                  )}
                </div>
              )}

              {activeTab === 'graph' && (
                <GraphView stats={stats} />
              )}
            </div>
          </div>
        </div>

        {/* Bottom Dashboard */}
        <div className="dashboard-grid">
          <div className="dashboard-card panel-card">
            <h3>📖 Guided Tutorials</h3>
            <div className="tutorial-list">
              <button className="btn-tut" onClick={() => loadExample("forall x . x = x")}>ZFC Well-Founded Identity</button>
              <button className="btn-tut" onClick={() => loadExample("{x | ~(x in x)} in {x | ~(x in x)}")}>Russell's Paradox</button>
              <button className="btn-tut" onClick={() => loadExample("Omega = {Omega}")}>The Quine Atom (0-weight loop)</button>
              <button className="btn-tut" onClick={() => loadExample("{{x}, {x, y}} = {{a}, {a, b}}")}>Kuratowski Ordered Pair</button>
              <button className="btn-tut" onClick={() => loadExample("Phi(m) = Phi(T(m))")}>Specker's Refutation</button>
              <button className="btn-tut" onClick={() => loadExample("simulate_hypothetical(agent_core, action)")}>Agentic Reflection</button>
              <button className="btn-tut" onClick={() => loadExample("SC_CUT(exclusionIndex[isCorrupted, isExpired])")}>Holographic Sieve</button>
            </div>
          </div>

          <div className="dashboard-card panel-card">
            <h3>💡 Tactical Challenges</h3>
            <ul className="tutorial-list">
              {CHALLENGES.map((chal, i) => (
                <button key={i} className="btn-tut" onClick={() => { setActiveChallenge(i); window.scrollTo(0, 0); }}>
                  {chal.title}
                </button>
              ))}
            </ul>
          </div>
        </div>
      </div>
    </>
  );
}
