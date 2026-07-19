import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'ETF Stock List',
		description:
			'Explore our complete ETF stock list with detailed information on ETFs and their underlying stocks. Perfect for traders and investors looking to diversify.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Resources', url: `${SITE}/resources` },
				{ name: 'ETF Stocks List', url: `${SITE}/resources/etf-stocks-list` }
			])
		]
	} satisfies SEOInput
});
