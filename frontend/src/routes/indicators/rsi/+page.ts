import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildSoftwareApplication } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'RSI — Relative Strength Index',
		description:
			'Master the RSI indicator to identify overbought/oversold conditions and reversal points with precision.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Indicators', url: `${SITE}/indicators` },
				{ name: 'RSI', url: `${SITE}/indicators/rsi` }
			]),
			buildSoftwareApplication({
				name: 'RSI — Relative Strength Index',
				description:
					'A custom RSI indicator implementation used by Revolution Trading Pros traders for spotting divergence and momentum exhaustion.',
				url: `${SITE}/indicators/rsi`,
				applicationCategory: 'FinanceApplication'
			})
		]
	} satisfies SEOInput
});
