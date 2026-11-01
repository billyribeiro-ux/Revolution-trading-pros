import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildWebPage } from '#lib/seo/schemas.js';

import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Contact Support & Trading Desk',
		description:
			'Reach the Revolution Trading Pros team — sales, support, and partnership enquiries.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Contact', url: `${SITE}/contact` }
			]),
			buildWebPage({
				url: `${SITE}/contact`,
				name: 'Contact Revolution Trading Pros',
				description: 'Get in touch with sales, support, and partnerships.',
				dateModified: '2026-05-25',
				pageType: 'ContactPage'
			}),
			{
				'@context': 'https://schema.org',
				'@type': 'Organization',
				'@id': `${SITE}/#organization`,
				name: 'Revolution Trading Pros',
				url: SITE,
				// TODO: replace with verified contact details before launch
				contactPoint: {
					'@type': 'ContactPoint',
					contactType: 'customer support',
					email: 'support@revolutiontradingpros.com',
					availableLanguage: ['en']
				}
			}
		]
	} satisfies SEOInput
});
