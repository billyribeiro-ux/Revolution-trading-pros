import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Stock Indexes List',
		description:
			'The S&P 500 is a prominent stock index that tracks the 500 largest companies listed on major U.S. exchanges. Learn about major stock indexes and how to trade them.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Resources', url: `${SITE}/resources` },
				{ name: 'Stock Indexes List', url: `${SITE}/resources/stock-indexes-list` }
			])
		]
	} satisfies SEOInput
});
