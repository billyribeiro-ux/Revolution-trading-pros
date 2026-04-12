import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';

export type ActiveProvider = 'polygon' | 'tradier' | 'theta_data' | 'yahoo';

interface CalculatorConfig {
	activeProvider: ActiveProvider;
	updatedAt: string;
}

// In-memory storage with env-var fallback (matches existing connections pattern)
let calculatorConfig: CalculatorConfig = {
	activeProvider: (env.DEFAULT_MARKET_PROVIDER as ActiveProvider) || 'polygon',
	updatedAt: new Date().toISOString()
};

const PROVIDER_META: Record<
	ActiveProvider,
	{ name: string; envKey: string; requiresKey: boolean }
> = {
	polygon: { name: 'Polygon.io', envKey: 'POLYGON_API_KEY', requiresKey: true },
	tradier: { name: 'Tradier', envKey: 'TRADIER_ACCESS_TOKEN', requiresKey: true },
	theta_data: { name: 'ThetaData', envKey: 'THETA_DATA_API_KEY', requiresKey: true },
	yahoo: { name: 'Yahoo Finance', envKey: '', requiresKey: false }
};

/** Exported so other server modules can read the active provider */
export function getActiveProvider(): ActiveProvider {
	return calculatorConfig.activeProvider;
}

/** Exported so other server modules can update the active provider */
export function setActiveProvider(provider: ActiveProvider): void {
	calculatorConfig = {
		activeProvider: provider,
		updatedAt: new Date().toISOString()
	};
}

export const GET: RequestHandler = async () => {
	const providerStatuses = Object.entries(PROVIDER_META).map(([key, meta]) => {
		const hasKey = meta.requiresKey ? !!(env as Record<string, string | undefined>)[meta.envKey] : true;
		return {
			provider: key,
			name: meta.name,
			isActive: key === calculatorConfig.activeProvider,
			isConfigured: hasKey,
			requiresKey: meta.requiresKey
		};
	});

	return json({
		success: true,
		activeProvider: calculatorConfig.activeProvider,
		updatedAt: calculatorConfig.updatedAt,
		providers: providerStatuses
	});
};

export const PUT: RequestHandler = async ({ request }) => {
	const body = await request.json();
	const { active_provider } = body;

	if (!active_provider || !PROVIDER_META[active_provider as ActiveProvider]) {
		return error(400, `Invalid provider. Must be one of: ${Object.keys(PROVIDER_META).join(', ')}`);
	}

	calculatorConfig = {
		activeProvider: active_provider as ActiveProvider,
		updatedAt: new Date().toISOString()
	};

	return json({
		success: true,
		activeProvider: calculatorConfig.activeProvider,
		updatedAt: calculatorConfig.updatedAt
	});
};
