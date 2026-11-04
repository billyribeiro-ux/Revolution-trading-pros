/** Bounded, idempotent delivery for existing browser metric producers. */
export async function deliverPerformanceObservation(
	endpoint: string,
	observation: Record<string, unknown>,
	headers: Record<string, string> = {},
	signal?: AbortSignal
): Promise<void> {
	const body = JSON.stringify({
		...observation,
		id: observation.id || crypto.randomUUID(),
		timestamp: observation.timestamp ?? Date.now()
	});
	let failure: Error = new Error('Performance observation was not acknowledged');
	for (let attempt = 0; attempt < 3; attempt++) {
		if (signal?.aborted) throw new DOMException('Metric collection stopped', 'AbortError');
		let retry = true;
		try {
			const response = await fetch(endpoint, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json', ...headers },
				body,
				keepalive: true,
				signal: signal
					? AbortSignal.any([signal, AbortSignal.timeout(12_000)])
					: AbortSignal.timeout(12_000)
			});
			if (!response.ok) {
				retry = response.status >= 500 || response.status === 408 || response.status === 429;
				throw new Error(`Metric storage returned HTTP ${response.status}`);
			}
			const receipt: unknown = await response.json();
			if (
				!receipt ||
				typeof receipt !== 'object' ||
				!('success' in receipt) ||
				receipt.success !== true ||
				!('received' in receipt) ||
				receipt.received !== 1 ||
				!('persisted' in receipt) ||
				(receipt.persisted !== 0 && receipt.persisted !== 1)
			) {
				throw new Error('Metric storage did not return a durable receipt');
			}
			return;
		} catch (cause) {
			failure = cause instanceof Error ? cause : new Error('Metric storage request failed');
			if (signal?.aborted) throw new DOMException('Metric collection stopped', 'AbortError');
			if (!retry) throw failure;
		}
		if (attempt < 2)
			await new Promise<void>((resolve, reject) => {
				if (signal?.aborted) {
					reject(new DOMException('Metric collection stopped', 'AbortError'));
					return;
				}
				const abort = () => {
					clearTimeout(timer);
					signal?.removeEventListener('abort', abort);
					reject(new DOMException('Metric collection stopped', 'AbortError'));
				};
				const timer = setTimeout(
					() => {
						signal?.removeEventListener('abort', abort);
						resolve();
					},
					250 * 2 ** attempt
				);
				signal?.addEventListener('abort', abort, { once: true });
			});
	}
	throw failure;
}
