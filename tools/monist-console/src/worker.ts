import init, { evaluate_formula, init_panic_hook } from 'monist-wasm';

let initialized = false;

self.onmessage = async (e: MessageEvent) => {
  const { id, query } = e.data;
  
  if (!initialized) {
    try {
      await init();
      init_panic_hook();
      initialized = true;
    } catch (err: any) {
      self.postMessage({ id, success: false, error: "Worker failed to initialize WASM: " + err.toString() });
      return;
    }
  }

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
