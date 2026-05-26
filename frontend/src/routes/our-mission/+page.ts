import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb, buildFAQPage, buildWebPage } from '$lib/seo/schemas';

const SITE = 'https://revolution-trading-pros.pages.dev';

export const prerender = true;

export const load = () => {
	const seo: SEOInput = {
		title: 'Our Mission — The Institutional Bridge',
		description:
			'Dismantling the retail trader stereotype. We bridge the gap between gambling and institutional risk management through data, discipline, and transparency. Join the 1% of traders who treat this as a business.',
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
			buildFAQPage([
				{
					q: "What is the 'Retail Trap' in trading?",
					a: 'The Retail Trap refers to the statistical probability of failure for non-professional traders (often cited as the 90/90/90 rule). This failure is driven by a lack of risk management, emotional trading, and reliance on lagging indicators.'
				},
				{
					q: 'How is Revolution Trading Pros different from other groups?',
					a: "We focus on 'Auction Market Theory' and 'Volume Profiling' rather than subjective chart patterns. We treat trading as a business of probability (Expectancy) rather than a game of prediction."
				}
			]),
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
