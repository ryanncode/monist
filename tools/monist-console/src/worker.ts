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
      if (data.cmd === 'theorem') {
         replSession!.start_proof(data.args[0], data.args.slice(1).join(" "));
      } else if (data.cmd === 'deff') {
         const rawCmd = data.rawCmd as string;
         const eqIdx = rawCmd.indexOf(':=');
         if (eqIdx === -1) throw new Error("Usage: deff <name>(<args>) := <formula>");
         const sigStr = rawCmd.substring(4, eqIdx).replace(/\s+/g, '');
         const formulaStr = rawCmd.substring(eqIdx + 2).trim();
         const openParen = sigStr.indexOf('(');
         const closeParen = sigStr.indexOf(')');
         let name = "";
         let params: string[] = [];
         if (openParen !== -1 && closeParen !== -1) {
             name = sigStr.substring(0, openParen);
             const paramsStr = sigStr.substring(openParen + 1, closeParen);
             if (paramsStr.length > 0) {
                 params = paramsStr.split(',');
             }
         } else {
             name = sigStr;
         }
         replSession!.define_macro(name, params, formulaStr);
      } else {
         replSession!.execute_tactic(data.cmd, data.args);
      }
      const state = replSession!.get_state_json();
      self.postMessage({ id: data.id, success: true, type: 'REPL_UPDATE', state });
    } catch (err: any) {
      self.postMessage({ id: data.id, success: false, type: 'REPL_ERROR', error: err.toString() });
    }
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
