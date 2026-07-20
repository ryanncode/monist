import React from 'react';

export const SYNTAX_GROUPS = [
  {
    label: 'Quantifiers',
    items: [
      { code: '∀', desc: 'Universal Quantifier' },
      { code: '∃', desc: 'Existential Quantifier' },
    ]
  },
  {
    label: 'Core Logic',
    items: [
      { code: '~', desc: 'Logical NOT' },
      { code: '&', desc: 'Logical AND' },
      { code: '|', desc: 'Logical OR / Bar' },
      { code: '→', desc: 'Implication' },
      { code: '↔', desc: 'Biconditional' },
    ]
  },
  {
    label: 'Relations',
    items: [
      { code: '=', desc: 'Equality' },
      { code: '∈', desc: 'Set Membership' },
      { code: '<', desc: 'Strict Less-Than' },
    ]
  },
  {
    label: 'Punctuation',
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
    label: 'Combinators',
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

interface SyntaxToolkitProps {
  onInsert: (code: string) => void;
}

export const SyntaxToolkit: React.FC<SyntaxToolkitProps> = ({ onInsert }) => {
  return (
    <div className="syntax-sidebar panel-card">
      <h3>Syntax Toolkit</h3>
      <div style={{ display: 'flex', flexDirection: 'row', flexWrap: 'wrap', gap: '10px', marginTop: '0.5rem' }}>
        {SYNTAX_GROUPS.map((group, gIdx) => (
          <div key={gIdx} className="syntax-group">
            <div style={{ fontSize: '0.75rem', fontWeight: 700, color: 'var(--mono-black, #333)', marginBottom: '0.4rem', textTransform: 'uppercase', letterSpacing: '0.5px' }}>{group.label}</div>
            <div className="syntax-grid">
              {group.items.map((item, idx) => (
                <button 
                  key={idx} 
                  className="btn btn-outline-secondary btn-sm rounded-0 syntax-btn" 
                  style={{ padding: '0.15rem 0.4rem', fontSize: '1rem', borderRadius: 0 }} 
                  title={item.desc} 
                  onClick={() => onInsert(item.code)}
                >
                  <code>{item.code}</code>
                </button>
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
