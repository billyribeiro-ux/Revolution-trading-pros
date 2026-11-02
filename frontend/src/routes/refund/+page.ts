import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildWebPage } from '#lib/seo/schemas.js';

import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Refund Policy',
		description:
			'Revolution Trading Pros 7-day money-back guarantee, refund, and subscription cancellation policy.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Refund Policy', url: `${SITE}/refund` }
			]),
			buildWebPage({
				url: `${SITE}/refund`,
				name: 'Refund Policy — Revolution Trading Pros',
				description:
					'Revolution Trading Pros refund and cancellation policy. Understand your rights and our commitment to your satisfaction.',
				dateModified: '2026-06-01'
			})
		]
	} satisfies SEOInput
});
