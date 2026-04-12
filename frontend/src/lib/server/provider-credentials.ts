import { env } from '$env/dynamic/private';
import {
	getActiveProvider,
	type ActiveProvider
} from '../../routes/api/admin/options-calculator/+server';

interface ProviderCredentials {
	provider: ActiveProvider;
	apiKey: string;
	isConfigured: boolean;
}

const ENV_KEY_MAP: Record<ActiveProvider, string> = {
	polygon: 'POLYGON_API_KEY',
	tradier: 'TRADIER_ACCESS_TOKEN',
	theta_data: 'THETA_DATA_API_KEY',
	yahoo: ''
};

/**
 * Resolve the active market data provider and its credentials.
 *
 * Priority:
 * 1. Explicit `providerOverride` param (from ?provider= query string)
 * 2. Admin-configured active provider (in-memory)
 * 3. Falls back to environment variables for the API key
 */
export function resolveProviderCredentials(providerOverride?: string): ProviderCredentials {
	const provider = (providerOverride as ActiveProvider) || getActiveProvider();

	if (provider === 'yahoo') {
		return { provider, apiKey: '', isConfigured: true };
	}

	const envKey = ENV_KEY_MAP[provider] ?? '';
	const apiKey = envKey ? ((env as Record<string, string | undefined>)[envKey] ?? '') : '';

	return {
		provider,
		apiKey,
		isConfigured: !!apiKey
	};
}
