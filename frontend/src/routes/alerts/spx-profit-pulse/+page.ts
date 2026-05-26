import type { Load } from '@sveltejs/kit';
import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildFAQPage, buildService } from '$lib/seo/schemas';

const SITE = 'https://revolution-trading-pros.pages.dev';

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
			}),
			buildFAQPage([
				{
					q: 'What is SPX 0DTE?',
					a: "SPX 0DTE refers to 'Zero Days to Expiration' options on the S&P 500 index. These contracts expire the same day they are traded, offering high potential returns due to rapid gamma exposure."
				},
				{
					q: 'How fast are the alerts?',
					a: 'Our alerts are sent instantly via SMS text message and Discord webhooks. The average latency is under 5 seconds from the moment our trader executes the trade.'
				},
				{
					q: 'What account size do I need?',
					a: 'Since we trade SPX options, premiums can range from $2.00 to $10.00 ($200-$1,000 per contract). We recommend a starting account of at least $2,000 to manage risk properly.'
				}
			])
		]
	};
	return { seo };
};
