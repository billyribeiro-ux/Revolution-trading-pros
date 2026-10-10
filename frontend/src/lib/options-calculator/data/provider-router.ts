// ============================================================
// PROVIDER ROUTER — Automatic Failover Chain
// Tries providers in priority order. If one fails, seamlessly
// falls back to the next. Zero downtime for the end user.
// ============================================================

import { on } from 'svelte/events';
import type { MarketDataProvider, RequestOptions } from './provider-interface.js';
import {
	AllProvidersFailedError,
	AuthenticationError,
	CapabilityNotSupportedError,
	InvalidResponseError,
	RateLimitExceededError
} from './provider-interface.js';
import type { DataProviderName, ProviderStatus } from './types.js';

export interface RouterConfig {
	providers: MarketDataProvider[];
	maxRetries: number;
	timeoutMs: number;
	onFallback?: (from: DataProviderName, to: DataProviderName, error: Error) => void;
	onAllFailed?: (
		capability: string,
		errors: { provider: DataProviderName; error: Error }[]
	) => void;
}

/** Errors a retry against the same provider cannot fix. */
function isRetryable(error: Error): boolean {
	return !(
		error instanceof CapabilityNotSupportedError ||
		error instanceof RateLimitExceededError ||
		error instanceof AuthenticationError ||
		error instanceof InvalidResponseError
	);
}

/** Expected gaps (unsupported capability, rate limit) fall through silently. */
function isReportable(error: Error): boolean {
	return !(error instanceof CapabilityNotSupportedError || error instanceof RateLimitExceededError);
}

/**
 * Create a provider router that automatically fails over between providers.
 */
export function createProviderRouter(config: RouterConfig) {
	const { providers, maxRetries, timeoutMs, onFallback, onAllFailed } = config;

	const lastSuccessful = new Map<string, DataProviderName>();

	/**
	 * Run one attempt with its own AbortController. The attempt is aborted —
	 * cancelling the underlying fetch — when it times out or when the caller's
	 * signal aborts; the race also settles if a provider ignores its signal.
	 * The timer and the listener are released however the attempt ends.
	 */
	async function attempt<T>(
		provider: MarketDataProvider,
		fn: (provider: MarketDataProvider, signal: AbortSignal) => Promise<T>,
		signal: AbortSignal | undefined
	): Promise<T> {
		const controller = new AbortController();
		let timer: ReturnType<typeof setTimeout> | undefined;
		let stopListening: (() => void) | undefined;

		const cancelled = new Promise<never>((_, reject) => {
			const fail = (reason: unknown) => {
				controller.abort(reason);
				reject(reason);
			};
			timer = setTimeout(() => fail(new Error(`Timeout after ${timeoutMs}ms`)), timeoutMs);
			if (signal) stopListening = on(signal, 'abort', () => fail(signal.reason));
		});

		try {
			return await Promise.race([fn(provider, controller.signal), cancelled]);
		} finally {
			clearTimeout(timer);
			stopListening?.();
		}
	}

	async function route<T>(
		capability: string,
		fn: (provider: MarketDataProvider, signal: AbortSignal) => Promise<T>,
		{ signal }: RequestOptions = {}
	): Promise<T> {
		const errors: { provider: DataProviderName; error: Error }[] = [];

		const preferred = lastSuccessful.get(capability);
		const sortedProviders = preferred
			? [
					providers.find((p) => p.name === preferred),
					...providers.filter((p) => p.name !== preferred)
				].filter((p): p is MarketDataProvider => p !== undefined)
			: providers;

		for (const [index, provider] of sortedProviders.entries()) {
			for (let attemptNo = 0; attemptNo <= maxRetries; attemptNo++) {
				try {
					signal?.throwIfAborted();
					const result = await attempt(provider, fn, signal);
					// A provider that ignored the abort may still resolve: drop that result.
					signal?.throwIfAborted();

					lastSuccessful.set(capability, provider.name);
					return result;
				} catch (error) {
					// The caller gave up (superseded or torn down): no retry, no
					// fallback, and nothing to report — rethrow its abort reason.
					signal?.throwIfAborted();

					const err = error instanceof Error ? error : new Error(String(error));

					if (isRetryable(err) && attemptNo < maxRetries) continue;

					errors.push({ provider: provider.name, error: err });
					const nextProvider = sortedProviders[index + 1];
					if (nextProvider && isReportable(err)) {
						onFallback?.(provider.name, nextProvider.name, err);
					}
					break;
				}
			}
		}

		onAllFailed?.(capability, errors);
		throw new AllProvidersFailedError(capability, errors);
	}

	async function getAllStatuses(options?: RequestOptions): Promise<ProviderStatus[]> {
		return Promise.all(providers.map((p) => p.getStatus(options)));
	}

	function getActiveProvider(capability: string): DataProviderName | undefined {
		return lastSuccessful.get(capability);
	}

	return { route, getAllStatuses, getActiveProvider, providers };
}

export type ProviderRouter = ReturnType<typeof createProviderRouter>;
