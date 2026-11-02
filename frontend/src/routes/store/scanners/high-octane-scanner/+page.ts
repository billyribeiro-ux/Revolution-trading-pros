import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildSoftwareApplication } from '#lib/seo/schemas.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'High Octane Scanner — Real-Time Options',
		description:
			'Professional-grade options scanner with real-time alerts and advanced filtering algorithms. Find high-probability setups in milliseconds.',
		og: {
			type: 'website',
			image: `${SITE}/revolution-trading-pros.png`,
			imageAlt: 'High Octane Scanner — Professional Options & Stock Scanner'
		},
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Market Scanners', url: `${SITE}/store/scanners` },
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
				image: `${SITE}/revolution-trading-pros.png`,
				offers: {
					price: 119,
					priceCurrency: 'USD',
					url: `${SITE}/store/scanners/high-octane-scanner`
				}
			})
		]
	} satisfies SEOInput
});
