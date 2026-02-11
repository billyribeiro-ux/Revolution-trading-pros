// ============================================================
// PROVIDER ROUTER — Automatic Failover Chain
// Tries providers in priority order. If one fails, seamlessly
// falls back to the next. Zero downtime for the end user.
// ============================================================

import type { MarketDataProvider } from './provider-interface.js';
import { AllProvidersFailedError } from './provider-interface.js';
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

/**
 * Create a provider router that automatically fails over between providers.
 */
export function createProviderRouter(config: RouterConfig) {
	const { providers, maxRetries, timeoutMs, onFallback, onAllFailed } = config;

	const lastSuccessful = new Map<string, DataProviderName>();

	async function route<T>(
		capability: string,
		fn: (provider: MarketDataProvider) => Promise<T>
	): Promise<T> {
		const errors: { provider: DataProviderName; error: Error }[] = [];

		const preferred = lastSuccessful.get(capability);
		const sortedProviders = preferred
			? [
					providers.find((p) => p.name === preferred),
					...providers.filter((p) => p.name !== preferred)
				].filter((p): p is MarketDataProvider => p !== undefined)
			: providers;

		for (const provider of sortedProviders) {
			for (let attempt = 0; attempt <= maxRetries; attempt++) {
				try {
					const result = await Promise.race([
						fn(provider),
						new Promise<never>((_, reject) =>
							setTimeout(() => reject(new Error(`Timeout after ${timeoutMs}ms`)), timeoutMs)
						)
					]);

					lastSuccessful.set(capability, provider.name);
					return result;
				} catch (error) {
					const err = error instanceof Error ? error : new Error(String(error));

					if (err.name === 'CapabilityNotSupportedError') {
						errors.push({ provider: provider.name, error: err });
						break;
					}

					if (err.name === 'RateLimitExceededError') {
						errors.push({ provider: provider.name, error: err });
						break;
					}

					if (attempt === maxRetries) {
						errors.push({ provider: provider.name, error: err });
						const nextProvider = sortedProviders[sortedProviders.indexOf(provider) + 1];
						if (nextProvider && onFallback) {
							onFallback(provider.name, nextProvider.name, err);
						}
					}
				}
			}
		}

		onAllFailed?.(capability, errors);
		throw new AllProvidersFailedError(capability, errors);
	}

	async function getAllStatuses(): Promise<ProviderStatus[]> {
		return Promise.all(providers.map((p) => p.getStatus()));
	}

	function getActiveProvider(capability: string): DataProviderName | undefined {
		return lastSuccessful.get(capability);
	}

	return { route, getAllStatuses, getActiveProvider, providers };
}

export type ProviderRouter = ReturnType<typeof createProviderRouter>;
