import { env } from '$env/dynamic/private';
import {
	getActiveProvider,
	type ActiveProvider
} from '../../routes/api/admin/options-calculator/+server';

interface ProviderCredentials {
	provider: ActiveProvider | 'fred';
	apiKey: string;
	isConfigured: boolean;
}

const ENV_KEY_MAP: Record<string, string> = {
	polygon: 'POLYGON_API_KEY',
	tradier: 'TRADIER_ACCESS_TOKEN',
	theta_data: 'THETA_DATA_API_KEY',
	yahoo: '',
	fred: 'FRED_API_KEY'
};

const VALID_PROVIDERS = new Set(Object.keys(ENV_KEY_MAP));

/**
 * Resolve the active market data provider and its credentials.
 *
 * Priority:
 * 1. Explicit `providerOverride` param (from ?provider= query string)
 * 2. Admin-configured active provider (in-memory)
 * 3. Falls back to environment variables for the API key
 */
export function resolveProviderCredentials(providerOverride?: string): ProviderCredentials {
	// Validate provider override is a known provider
	const provider = (providerOverride && VALID_PROVIDERS.has(providerOverride))
		? providerOverride
		: getActiveProvider();

	if (provider === 'yahoo') {
		return { provider, apiKey: '', isConfigured: true };
	}

	const envKey = ENV_KEY_MAP[provider] ?? '';
	const apiKey = envKey ? ((env as Record<string, string | undefined>)[envKey] ?? '') : '';

	return {
		provider: provider as ActiveProvider | 'fred',
		apiKey,
		isConfigured: !!apiKey
	};
}
