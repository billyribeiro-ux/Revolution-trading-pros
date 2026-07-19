import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildSoftwareApplication } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'MACD — Moving Average Convergence Divergence',
		description:
			'Master the MACD indicator to identify trend changes and momentum shifts with this powerful trend-following tool.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Indicators', url: `${SITE}/indicators` },
				{ name: 'MACD', url: `${SITE}/indicators/macd` }
			]),
			buildSoftwareApplication({
				name: 'MACD — Moving Average Convergence Divergence',
				description:
					'A custom MACD indicator implementation used by Revolution Trading Pros traders to filter trends and confirm momentum.',
				url: `${SITE}/indicators/macd`,
				applicationCategory: 'FinanceApplication'
			})
		]
	} satisfies SEOInput
});
