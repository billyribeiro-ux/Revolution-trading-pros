/**
 * Resources Page Configuration - Apple ICT 7
 *
 * SSG Configuration:
 * - prerender: true (static page for SEO)
 * - ssr: true (server-side rendering)
 *
 * @version 1.0.0
 */

import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

// Enable static site generation for SEO
export const prerender = true;

// Enable server-side rendering
export const ssr = true;

export const load = () => ({
	seo: {
		title: 'Free Trading Resources & Education Tools',
		description:
			'Free trading resources, guides, calculators, and tools. Start your trading education with our comprehensive resource library. Position size calculators, trading journal templates, and more.',
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Resources', url: `${SITE}/resources` }
			]),
			{
				'@context': 'https://schema.org',
				'@type': 'CollectionPage',
				'@id': `${SITE}/resources/#collection`,
				name: 'Free Trading Resources',
				description:
					'Free trading resources, guides, calculators, and educational tools for traders.',
				url: `${SITE}/resources`,
				mainEntity: {
					'@type': 'ItemList',
					itemListElement: [
						{
							'@type': 'ListItem',
							position: 1,
							name: 'Beginner Trading Guides',
							description: 'Complete guides for new traders including account setup and basics.'
						},
						{
							'@type': 'ListItem',
							position: 2,
							name: 'Free Trading Tools',
							description:
								'Position size calculator, risk/reward calculator, and trading journal templates.'
						},
						{
							'@type': 'ListItem',
							position: 3,
							name: 'Video Tutorials',
							description: 'Platform setup, chart reading, and live trade walkthroughs.'
						},
						{
							'@type': 'ListItem',
							position: 4,
							name: 'Market Analysis',
							description: 'Weekly recaps, economic calendar, and key levels to watch.'
						},
						{
							'@type': 'ListItem',
							position: 5,
							name: 'Trading Psychology',
							description: 'Managing emotions, developing discipline, and building confidence.'
						}
					]
				}
			}
		]
	} satisfies SEOInput
});
