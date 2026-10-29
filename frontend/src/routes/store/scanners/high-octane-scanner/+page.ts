import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildSoftwareApplication } from '#lib/seo/schemas.js';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'High Octane Scanner — Real-Time Options & Stock Scanner',
		description:
			'Professional-grade options scanner with real-time alerts and advanced filtering algorithms. Find high-probability setups in milliseconds.',
		og: {
			type: 'website',
			image: `${SITE}/images/scanners/high-octane-scanner-hero.jpg`,
			imageAlt: 'High Octane Scanner — Professional Options & Stock Scanner'
		},
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Scanners', url: `${SITE}/store/scanners` },
				{
					name: 'High Octane Scanner',
					url: `${SITE}/store/scanners/high-octane-scanner`
				}
			]),
			buildSoftwareApplication({
				name: 'High Octane Scanner',
				description:
					'Professional-grade options scanner with real-time alerts and advanced filtering algorithms.',
				url: `${SITE}/store/scanners/high-octane-scanner`,
				applicationCategory: 'FinanceApplication',
				operatingSystem: 'Web',
				image: `${SITE}/images/scanners/high-octane-scanner-hero.jpg`,
				offers: {
					price: 119,
					priceCurrency: 'USD',
					url: `${SITE}/store/scanners/high-octane-scanner`
				},
				aggregateRating: {
					ratingValue: 4.8,
					ratingCount: 127
				}
			})
		]
	} satisfies SEOInput
});
