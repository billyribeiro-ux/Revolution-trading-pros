import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildWebPage } from '#lib/seo/schemas.js';

import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = true;

export const load = () => {
	const seo: SEOInput = {
		title: 'Our Mission — The Institutional Bridge',
		description:
			'We bridge the gap between retail trading and institutional risk management through data, discipline, and transparency.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Our Mission', url: `${SITE}/our-mission` }
			]),
			buildWebPage({
				url: `${SITE}/our-mission`,
				name: 'Our Mission',
				description:
					'The Institutional Bridge — bridging retail traders to professional risk management via Auction Market Theory and Volume Profiling.',
				dateModified: '2026-05-25',
				pageType: 'AboutPage'
			}),
			{
				'@context': 'https://schema.org',
				'@type': 'Course',
				name: 'Institutional Trading Mastery',
				description:
					'A comprehensive curriculum covering Auction Market Theory, Volume Profiling, and Institutional Risk Management.',
				provider: { '@type': 'Organization', name: 'Revolution Trading Pros', url: SITE },
				hasCourseInstance: {
					'@type': 'CourseInstance',
					courseMode: 'online',
					courseWorkload: 'P12W'
				}
			}
		]
	};
	return { seo };
};
