import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildService } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

// +page.ts
export const prerender = true;

export const load = () => {
	const seo: SEOInput = {
		title: 'Institutional Strategy Audit — $25,000 Consultation',
		description:
			'A high-velocity, forensic deconstruction of your trading business. Designed strictly for Portfolio Managers and Proprietary Traders deploying 7-8 figure capital.',
		robots: { index: false, follow: false },
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Mentorship', url: `${SITE}/mentorship` }
			]),
			buildService({
				name: 'Institutional Strategy Audit',
				description:
					"Forensic deconstruction of a trader's system, risk framework, and execution edge — strictly for portfolio managers and proprietary traders.",
				url: `${SITE}/mentorship`,
				provider: { name: 'Revolution Trading Pros', url: SITE },
				serviceType: 'Trading Mentorship',
				areaServed: 'Worldwide'
			})
		]
	};
	return { seo };
};
