import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildWebPage } from '#lib/seo/schemas.js';

const SITE = 'https://revolutiontradingpros.com';

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
