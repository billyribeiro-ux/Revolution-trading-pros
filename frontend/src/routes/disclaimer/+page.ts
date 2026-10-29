import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildWebPage } from '#lib/seo/schemas.js';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Risk Disclaimer',
		description:
			'Important risk disclaimer and financial disclosure information for Revolution Trading Pros members and visitors.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Risk Disclaimer', url: `${SITE}/disclaimer` }
			]),
			buildWebPage({
				url: `${SITE}/disclaimer`,
				name: 'Risk Disclaimer — Revolution Trading Pros',
				description:
					'Important risk disclaimer and financial disclosure information for Revolution Trading Pros members and visitors.',
				dateModified: '2026-06-01'
			})
		]
	} satisfies SEOInput
});
