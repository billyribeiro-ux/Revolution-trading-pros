// ============================================================
// MONTE CARLO WEB WORKER
// Runs the GBM simulation off the main thread so the UI stays
// responsive while thousands of paths are generated.
// ============================================================

import { runMonteCarlo } from './monte-carlo.js';
import type { BSInputs, MonteCarloConfig, MonteCarloResult } from './types.js';

export interface MonteCarloWorkerRequest {
	inputs: BSInputs;
	config: MonteCarloConfig;
}

export type MonteCarloWorkerResponse =
	{ ok: true; result: MonteCarloResult } | { ok: false; error: string };

self.onmessage = (event: MessageEvent<MonteCarloWorkerRequest>) => {
	let response: MonteCarloWorkerResponse;
	try {
		const { inputs, config } = event.data;
		response = { ok: true, result: runMonteCarlo(inputs, config) };
	} catch (err) {
		response = { ok: false, error: err instanceof Error ? err.message : String(err) };
	}
	self.postMessage(response);
};
