import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildWebPage } from '#lib/seo/schemas.js';

import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load = () => {
	const seo: SEOInput = {
		title: 'About Our Trading Desk & Educators',
		description:
			'Join a professional trading floor that genuinely cares. Real trades, real-time voice guidance, and institutional data without the hype. Established 2018.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'About', url: `${SITE}/about` }
			]),
			buildWebPage({
				url: `${SITE}/about`,
				name: 'About Revolution Trading Pros',
				description: 'Professional trading community and mentorship program — est. 2018.',
				dateModified: '2026-05-25',
				pageType: 'AboutPage'
			})
		]
	};
	return { seo };
};
