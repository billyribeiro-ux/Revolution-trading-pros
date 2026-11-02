import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildProduct } from '#lib/seo/schemas.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load = () => {
	const seo: SEOInput = {
		title: '1-on-1 Institutional Trading Mentorship',
		description:
			'High-velocity forensic deconstruction of your trading business. Designed for portfolio managers and proprietary traders deploying capital.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: '1-on-1 Mentorship', url: `${SITE}/mentorship` }
			]),
			buildProduct({
				name: 'Institutional Strategy Audit & 1-on-1 Mentorship',
				description:
					"Forensic deconstruction of a trader's system, risk framework, and execution edge — strictly for portfolio managers and proprietary traders.",
				url: `${SITE}/mentorship`,
				image: `${SITE}/revolution-trading-pros.png`,
				offers: { price: 25000, priceCurrency: 'USD' }
			})
		]
	};
	return { seo };
};
