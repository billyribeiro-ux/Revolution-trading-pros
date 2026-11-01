import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildSoftwareApplication } from '#lib/seo/schemas.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'RSI — Relative Strength Index',
		description:
			'Master the RSI indicator to identify overbought/oversold conditions and reversal points with precision.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Trading Indicators', url: `${SITE}/indicators` },
				{ name: 'RSI Indicator', url: `${SITE}/indicators/rsi` }
			]),
			buildSoftwareApplication({
				name: 'RSI — Relative Strength Index',
				description:
					'A custom RSI indicator implementation used by Revolution Trading Pros traders for spotting divergence and momentum exhaustion.',
				url: `${SITE}/indicators/rsi`,
				applicationCategory: 'FinanceApplication',
				operatingSystem: 'Web, ThinkorSwim, TradingView',
				image: `${SITE}/revolution-trading-pros.png`,
				offers: {
					price: 0,
					priceCurrency: 'USD',
					url: `${SITE}/indicators/rsi`
				}
			})
		]
	} satisfies SEOInput
});
