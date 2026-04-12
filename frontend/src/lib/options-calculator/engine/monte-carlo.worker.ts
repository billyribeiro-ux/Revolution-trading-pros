// ============================================================
// MONTE CARLO WEB WORKER
// Offloads heavy simulation from the main thread
// ============================================================

import { runMonteCarlo } from './monte-carlo.js';
import type { BSInputs, MonteCarloConfig } from './types.js';

interface WorkerMessage {
	inputs: BSInputs;
	config: MonteCarloConfig;
}

self.onmessage = (event: MessageEvent<WorkerMessage>) => {
	const { inputs, config } = event.data;
	const result = runMonteCarlo(inputs, config);
	self.postMessage(result);
};
