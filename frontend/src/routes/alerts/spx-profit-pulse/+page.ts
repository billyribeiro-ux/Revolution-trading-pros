import type { Load } from '@sveltejs/kit';
import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildService } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

// Critical: Render this page as static HTML at build time.
export const prerender = true;
export const trailingSlash = 'always';

export const load: Load = async () => {
	const seo: SEOInput = {
		title: 'SPX Profit Pulse — #1 0DTE Options Alerts',
		description:
			'Trade SPX 0DTE options with confidence. Get real-time SMS alerts, precise entries/exits, and professional risk management. Join 1,000+ traders.',
		og: {
			type: 'website',
			image: `${SITE}/images/og-spx-pulse.jpg`,
			imageAlt: 'SPX Profit Pulse Alerts'
		},
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Alerts', url: `${SITE}/alerts` },
				{ name: 'SPX Profit Pulse', url: `${SITE}/alerts/spx-profit-pulse` }
			]),
			buildService({
				name: 'SPX Profit Pulse — 0DTE Options Alerts',
				description:
					'Professional SPX 0DTE options alerts delivered via SMS and Discord. Real-time entries, exits, and risk management.',
				url: `${SITE}/alerts/spx-profit-pulse`,
				provider: { name: 'Revolution Trading Pros', url: SITE },
				serviceType: 'Trading Alerts'
			})
		]
	};
	return { seo };
};
