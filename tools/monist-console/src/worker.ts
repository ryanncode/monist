import init, { evaluate_formula, init_panic_hook, ReplWasmSession } from 'monist-wasm';

let initialized = false;
let replSession: ReplWasmSession | null = null;

self.onmessage = async (e: MessageEvent) => {
  const data = e.data;
  
  if (!initialized) {
    try {
      await init();
      init_panic_hook();
      replSession = new ReplWasmSession();
      initialized = true;
    } catch (err: any) {
      self.postMessage({ id: data.id, success: false, error: "Worker failed to initialize WASM: " + err.toString() });
      return;
    }
  }

  // Handle Stateful REPL commands
  if (data.type === 'REPL_COMMAND') {
    try {
      const line = (data.rawCmd || data.line || '').trim();
      const resp = replSession!.process_repl_line(line);
      self.postMessage({
        id: data.id,
        type: 'REPL_UPDATE',
        success: resp.success,
        output_lines: resp.output_lines || [],
        state: resp.state,
        error: resp.error
      });
    } catch (err: any) {
      self.postMessage({ id: data.id, success: false, type: 'REPL_ERROR', error: err.toString() });
    }
    return;
  }

  if (data.type === 'REPL_RESET') {
    replSession = new ReplWasmSession();
    self.postMessage({ id: data.id, type: 'REPL_UPDATE', success: true, output_lines: ['REPL session reset.'], state: null });
    return;
  }

  // Handle Stateless Bounds Checker
  const { id, query } = data;
  try {
    const res = evaluate_formula(query);
    self.postMessage({
      id,
      success: true,
      data: {
        is_stratified: res.is_stratified,
        max_k_iterations: res.max_k_iterations,
        mcm: res.mcm,
        smt_witness: res.smt_witness
      }
    });
  } catch (err: any) {
    self.postMessage({
      id,
      success: false,
      error: err.toString()
    });
  }
};

