import React, { useState, useEffect } from 'react';

export interface TutorialStep {
  title: string;
  description: React.ReactNode;
  expectedCommand?: RegExp;
  expectedStatsCondition?: (stats: any) => boolean;
  onSuccessMessage: string;
}

export const TUTORIAL_QUINE_ATOM: TutorialStep[] = [
  {
    title: "1. The Concept of a Quine Atom",
    description: (
      <>
        <p>In standard ZFC set theory, the Axiom of Foundation forbids a set from containing itself. But in Quine's New Foundations (NF), self-referential sets can exist if they can be stratified!</p>
        <p>A <strong>Quine Atom</strong> is a set that contains only itself: <code>Ω = {"{Ω}"}</code>. Paste the following into the editor and click <strong>Evaluate</strong>:</p>
        <pre><code>Omega = {"{Omega}"}</code></pre>
      </>
    ),
    expectedCommand: /^Omega\s*=\s*\{Omega\}$/,
    expectedStatsCondition: (stats) => stats?.isStratified === true && stats?.graphType === 'basic_loop',
    onSuccessMessage: "Perfect! The engine resolved the self-reference. In a graph topology, a Quine Atom is just a 0-weight loop on a single node."
  },
  {
    title: "2. Breaking Stratification",
    description: (
      <>
        <p>Now let's try something that violates stratification. In NF, you cannot form a set of all sets that do not contain themselves (Russell's Paradox).</p>
        <p>Evaluate:</p>
        <pre><code>{"{x | ~(x ∈ x)} ∈ {x | ~(x ∈ x)}"}</code></pre>
        <p><em>(Hint: You can use the ∈ symbol from the Syntax Toolkit!)</em></p>
      </>
    ),
    expectedCommand: /\{.*\|\s*~\(.*\s*∈\s*.*\)\}\s*∈\s*\{.*\|\s*~\(.*\s*∈\s*.*\)\}/,
    expectedStatsCondition: (stats) => stats?.isStratified === false && stats?.mcm < 0,
    onSuccessMessage: "Great job! The engine immediately detected a negative-weight cycle (MCM < 0), meaning it's mathematically impossible to stratify."
  },
  {
    title: "3. Wrapping Up",
    description: (
      <>
        <p>You've successfully seen how Monist models both stable self-reference and unstable paradoxes natively on a graph!</p>
      </>
    ),
    onSuccessMessage: ""
  }
];

export const TUTORIAL_ITP_BASICS: TutorialStep[] = [
  {
    title: "1. Getting Help",
    description: (
      <>
        <p>Welcome to the Interactive Theorem Prover! This tutorial will walk you through the basics of proving theorems using tactics.</p>
        <p>Start by typing <code>help</code> to see the available commands:</p>
        <pre><code>help</code></pre>
      </>
    ),
    expectedCommand: /^help$/,
    onSuccessMessage: "You can see all available tactics and formula syntax. Let's use them!"
  },
  {
    title: "2. Starting a Proof",
    description: (
      <>
        <p>To prove a theorem, use the <code>theorem</code> command followed by a name and a formula.</p>
        <p>Let's prove a simple identity — that P implies P. Type:</p>
        <pre><code>theorem id P -&gt; P</code></pre>
      </>
    ),
    expectedCommand: /^theorem\s+\w+\s+/,
    onSuccessMessage: "A proof goal has been created. Look at the Active Proof State panel on the right — it shows your current goal."
  },
  {
    title: "3. Introducing a Hypothesis",
    description: (
      <>
        <p>The goal is <code>P → P</code>. To prove an implication, we introduce the antecedent as a hypothesis using <code>intro</code>.</p>
        <p>Type:</p>
        <pre><code>intro h</code></pre>
        <p>This moves <code>P</code> from the goal into the context as hypothesis <code>h</code>.</p>
      </>
    ),
    expectedCommand: /^intro\s+/,
    onSuccessMessage: "Now you have hypothesis h in your context, and the remaining goal is just P."
  },
  {
    title: "4. Closing the Goal",
    description: (
      <>
        <p>The goal is now just <code>P</code>, and we have <code>h : P</code> in our context. We can close this goal exactly with our hypothesis.</p>
        <p>Type:</p>
        <pre><code>exact h</code></pre>
      </>
    ),
    expectedCommand: /^exact\s+/,
    onSuccessMessage: "Proof complete! You've proven your first theorem using the ITP."
  },
  {
    title: "5. What's Next",
    description: (
      <>
        <p>Congratulations! You've completed your first interactive proof. Here's a summary of what you learned:</p>
        <ul>
          <li><code>theorem name formula</code> — start a new proof</li>
          <li><code>intro name</code> — introduce a hypothesis</li>
          <li><code>exact name</code> — close a goal with a hypothesis</li>
        </ul>
        <p>Try other tactics like <code>apply</code>, <code>split</code>, <code>left</code>, <code>right</code>, and <code>destruct</code> on your own!</p>
      </>
    ),
    onSuccessMessage: ""
  }
];

interface GuidedTutorialBoxProps {
  lastCommand: string;
  lastStats: any;
  tutorial: TutorialStep[];
}

export const GuidedTutorialBox: React.FC<GuidedTutorialBoxProps> = ({ lastCommand, lastStats, tutorial }) => {
  const [currentStep, setCurrentStep] = useState(0);
  const [stepCompleted, setStepCompleted] = useState(false);

  useEffect(() => {
    setCurrentStep(0);
    setStepCompleted(false);
  }, [tutorial]);

  useEffect(() => {
    if (stepCompleted || currentStep >= tutorial.length) return;
    
    const step = tutorial[currentStep];
    let isComplete = false;
    
    // Auto-complete if there are no conditions
    if (!step.expectedCommand && !step.expectedStatsCondition) {
      isComplete = true;
    }
    
    // Both conditions must pass when both are specified
    if (step.expectedCommand && step.expectedStatsCondition) {
      if (step.expectedCommand.test(lastCommand) && step.expectedStatsCondition(lastStats)) {
        isComplete = true;
      }
    } else if (step.expectedCommand && step.expectedCommand.test(lastCommand)) {
      isComplete = true;
    } else if (step.expectedStatsCondition && step.expectedStatsCondition(lastStats)) {
      isComplete = true;
    }
    
    if (isComplete) {
      setStepCompleted(true);
    }
  }, [lastCommand, lastStats, currentStep, stepCompleted, tutorial]);

  const handleNext = () => {
    setCurrentStep(prev => prev + 1);
    setStepCompleted(false);
  };
  
  if (currentStep >= tutorial.length) {
    return (
      <div className="active-challenge-box" style={{ marginTop: '20px', border: '1px solid var(--mono-black)', background: 'var(--mono-white)', width: '100%', boxSizing: 'border-box' }}>
        <h4 style={{ color: 'var(--mono-green)' }}>Tutorial Complete!</h4>
        <p>You have finished this interactive guided tutorial. Choose another from the dashboard below.</p>
        <button className="btn-primary rounded-0" onClick={() => { setCurrentStep(0); setStepCompleted(false); }} style={{ width: 'fit-content', padding: '0.5rem 1rem' }}>Restart Tutorial</button>
      </div>
    );
  }

  const step = tutorial[currentStep];

  return (
    <div className="active-challenge-box" style={{ marginTop: '20px', border: '1px solid var(--mono-black)', background: 'var(--mono-white)', width: '100%', boxSizing: 'border-box' }}>
      <h4>Interactive Tutorial: {step.title}</h4>
      <div style={{ padding: '10px 0' }}>
        {step.description}
      </div>
      
      {stepCompleted ? (
        <div style={{ marginTop: '15px', padding: '15px', background: 'rgba(46, 125, 50, 0.1)', border: '1px solid var(--mono-green)' }}>
          <strong style={{ color: 'var(--mono-green)' }}>Success!</strong> {step.onSuccessMessage}
          <div style={{ marginTop: '15px' }}>
            <button className="btn-primary rounded-0" onClick={handleNext} style={{ padding: '0.5rem 1.5rem' }}>Next Step →</button>
          </div>
        </div>
      ) : (
        <div style={{ marginTop: '15px', padding: '10px', fontStyle: 'italic', color: '#666', borderLeft: '3px solid var(--mono-black)' }}>
          Awaiting input... Evaluate the required formula to continue.
        </div>
      )}
    </div>
  );
};

export const TUTORIAL_ITP_SUBSET_TRANS: TutorialStep[] = [
  {
    title: "1. Transitivity of Subset",
    description: (
      <>
        <p>This theorem tests the fundamental natural deduction pipeline, handling nested universal quantifiers, implications, and backward reasoning.</p>
        <p>Let's start by stating the theorem. Type:</p>
        <pre><code>theorem Subset_Trans forall A B C z. (((z e A -&gt; z e B) /\ (z e B -&gt; z e C)) -&gt; (z e A -&gt; z e C))</code></pre>
      </>
    ),
    expectedCommand: /^theorem\s+Subset_Trans/,
    onSuccessMessage: "Great! The theorem is initialized."
  },
  {
    title: "2. Introducing Variables and Premises",
    description: (
      <>
        <p>Use the <code>intro</code> tactic to strip the universal quantifiers and the implication premise into your context.</p>
        <p>Type <code>intro A</code>, <code>intro B</code>, <code>intro C</code>, <code>intro z</code>, and finally to bring the main premise into context, type:</p>
        <pre><code>intro H</code></pre>
      </>
    ),
    expectedCommand: /^intro\s+H$/,
    onSuccessMessage: "Variables and the premise are now in context."
  },
  {
    title: "3. Destructing the Conjunction",
    description: (
      <>
        <p>Our hypothesis <code>H</code> is a conjunction of two implications. We need to break it apart.</p>
        <p>Type:</p>
        <pre><code>destruct H H1 H2</code></pre>
      </>
    ),
    expectedCommand: /^destruct\s+H\s+H1\s+H2$/,
    onSuccessMessage: "The premise is split into H1 and H2."
  },
  {
    title: "4. Preparing the Final Goal",
    description: (
      <>
        <p>Our goal is now <code>z e A -&gt; z e C</code>. Introduce the final hypothesis.</p>
        <p>Type:</p>
        <pre><code>intro Hz</code></pre>
      </>
    ),
    expectedCommand: /^intro\s+Hz$/,
    onSuccessMessage: "Goal is now z e C."
  },
  {
    title: "5. Backward Reasoning",
    description: (
      <>
        <p>Now we use <code>apply</code> to reason backwards. <code>H2</code> says <code>z e B -&gt; z e C</code>. By applying it, our goal becomes proving <code>z e B</code>.</p>
        <pre><code>apply H2</code></pre>
      </>
    ),
    expectedCommand: /^apply\s+H2$/,
    onSuccessMessage: "Goal is now z e B."
  },
  {
    title: "6. Closing the Proof",
    description: (
      <>
        <p>Apply <code>H1</code> to change the goal to <code>z e A</code>, and then close it exactly with <code>Hz</code>.</p>
        <pre><code>apply H1</code></pre>
        <p>Then type: <code>exact Hz</code>.</p>
      </>
    ),
    expectedCommand: /^exact\s+Hz$/,
    onSuccessMessage: "Proof complete! You have proven the transitivity of the subset relation."
  }
];

export const TUTORIAL_ITP_DISTRIBUTIVITY: TutorialStep[] = [
  {
    title: "1. Distributivity of Intersection Over Union",
    description: (
      <>
        <p>This benchmark tests Disjunctive Normal Form (DNF) reduction and branching tactics.</p>
        <p>Type:</p>
        <pre><code>theorem Distributivity_LR forall x A B C. ((x e A /\ (x e B \/ x e C)) -&gt; ((x e A /\ x e B) \/ (x e A /\ x e C)))</code></pre>
      </>
    ),
    expectedCommand: /^theorem\s+Distributivity_LR/,
    onSuccessMessage: "Theorem created."
  },
  {
    title: "2. Destructing Nested Hypotheses",
    description: (
      <>
        <p>After introducing variables (<code>intro x</code>, <code>intro A</code>, <code>intro B</code>, <code>intro C</code>) and the premise (<code>intro H</code>), destruct the premise to isolate <code>x e A</code> and the disjunction.</p>
        <p>Introduce everything, then type:</p>
        <pre><code>destruct H HA HBC</code></pre>
      </>
    ),
    expectedCommand: /^destruct\s+H\s+HA\s+HBC$/,
    onSuccessMessage: "You now have HA and the disjunction HBC in context."
  },
  {
    title: "3. Branching the Proof State",
    description: (
      <>
        <p>A <code>destruct</code> on a disjunction splits the proof state into two distinct sub-goals.</p>
        <p>Type:</p>
        <pre><code>destruct HBC HB HC</code></pre>
      </>
    ),
    expectedCommand: /^destruct\s+HBC\s+HB\s+HC$/,
    onSuccessMessage: "The proof state has branched!"
  },
  {
    title: "4. Solving the Left Branch",
    description: (
      <>
        <p>Use the <code>left</code> tactic to choose the left side of the OR goal. Then use <code>split</code> to break the AND goal into two.</p>
        <pre><code>left</code></pre>
        <p>Then <code>split</code>, and close the goals with <code>exact HA</code> and <code>exact HB</code>.</p>
      </>
    ),
    expectedCommand: /^exact\s+HB$/,
    onSuccessMessage: "First branch closed."
  },
  {
    title: "5. Solving the Right Branch",
    description: (
      <>
        <p>Now solve the second branch. Choose <code>right</code>, <code>split</code>, and close with <code>exact HA</code> and <code>exact HC</code>.</p>
        <pre><code>exact HC</code></pre>
      </>
    ),
    expectedCommand: /^exact\s+HC$/,
    onSuccessMessage: "Proof complete! You successfully navigated a branched proof state."
  }
];

export const TUTORIAL_ITP_STRATEGIC_CUT: TutorialStep[] = [
  {
    title: "1. The Strategic Cut",
    description: (
      <>
        <p>The <code>cut</code> tactic injects new geometry dynamically into the graph. Let's bound a union.</p>
        <p>Type:</p>
        <pre><code>theorem Union_Addition forall x A B. (x e A -&gt; (x e A \/ x e B))</code></pre>
      </>
    ),
    expectedCommand: /^theorem\s+Union_Addition/,
    onSuccessMessage: "Theorem created."
  },
  {
    title: "2. Injecting the Topology",
    description: (
      <>
        <p>Introduce the variables and premise (<code>intro x</code>, <code>intro A</code>, <code>intro B</code>, <code>intro H</code>). Then, inject an intermediate topological bound using <code>cut</code>.</p>
        <p>Type:</p>
        <pre><code>cut (x e A \/ x e B)</code></pre>
      </>
    ),
    expectedCommand: /^cut\s+/,
    onSuccessMessage: "Cut successfully processed without a +1 type shift violation!"
  },
  {
    title: "3. Closing the Goals",
    description: (
      <>
        <p>The tactic splits the proof. Solve the first branch with <code>left</code> and <code>exact H</code>, then the cut formula is added to your hypotheses to easily close the remaining goal.</p>
        <p>Once you finish the last <code>exact H</code>, the proof will be complete.</p>
      </>
    ),
    expectedCommand: /^exact\s+H$/,
    onSuccessMessage: "Proof complete! You verified the engine safely integrated the cut geometry."
  }
];
