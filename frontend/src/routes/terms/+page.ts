import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildWebPage } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Terms of Service',
		description:
			'Terms of service for Revolution Trading Pros — your rights and responsibilities when using the platform.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Terms of Service', url: `${SITE}/terms` }
			]),
			buildWebPage({
				url: `${SITE}/terms`,
				name: 'Terms of Service',
				description: 'Terms of service for Revolution Trading Pros.',
				dateModified: '2026-05-25'
			})
		]
	} satisfies SEOInput
});
