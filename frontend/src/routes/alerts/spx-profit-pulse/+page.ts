import type { Load } from '@sveltejs/kit';
import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildProduct } from '#lib/seo/schemas.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load: Load = async () => {
	const seo: SEOInput = {
		title: 'SPX Profit Pulse — 0DTE Options Alerts',
		description:
			'Trade SPX 0DTE options with confidence. Get real-time SMS alerts, precise entries and exits, and institutional risk management.',
		og: {
			type: 'website',
			image: `${SITE}/revolution-trading-pros.png`,
			imageAlt: 'SPX Profit Pulse Alerts'
		},
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Trading Alerts', url: `${SITE}/alerts` },
				{ name: 'SPX Profit Pulse', url: `${SITE}/alerts/spx-profit-pulse` }
			]),
			buildProduct({
				name: 'SPX Profit Pulse — 0DTE Options Alerts',
				description:
					'Professional SPX 0DTE options alerts delivered via SMS and Discord with real-time entries, exits, and risk management.',
				url: `${SITE}/alerts/spx-profit-pulse`,
				image: `${SITE}/revolution-trading-pros.png`,
				offers: { price: 97, priceCurrency: 'USD' }
			})
		]
	};
	return { seo };
};
