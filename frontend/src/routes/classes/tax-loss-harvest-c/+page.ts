import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildCourse, buildVideoObject } from '#lib/seo/schemas.js';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Tax Loss Harvest Strategy Class',
		description:
			'Learn institutional tax-loss harvesting strategies for active traders and investors to optimize portfolio tax efficiency at year-end.',
		og: {
			type: 'article'
		},
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Classes', url: `${SITE}/classes` },
				{ name: 'Tax Loss Harvest', url: `${SITE}/classes/tax-loss-harvest-c` }
			]),
			buildCourse({
				url: `${SITE}/classes/tax-loss-harvest-c`,
				name: 'Tax Loss Harvest Strategy Class',
				description:
					'Institutional tax-loss harvesting strategies for active traders and options investors.',
				provider: { name: 'Revolution Trading Pros', url: SITE },
				hasCourseInstance: {
					courseMode: 'Online'
				}
			}),
			buildVideoObject({
				name: 'Tax Loss Harvest Strategy Class',
				description:
					'Recorded multi-session workshop on tax-loss harvesting strategies for active traders.',
				thumbnailUrl: `${SITE}/revolution-trading-pros.png`,
				uploadDate: '2025-12-31T00:00:00Z',
				contentUrl:
					'https://cloud-streaming.s3.amazonaws.com/chatrecordings/STWebinars/6941b999d4e2e95d75e7ddfb_Tax_Loss_Harvest_2025_JC_12-31-2025__03.00.276_PM.mp4',
				creator: {
					name: 'Revolution Trading Pros',
					url: SITE,
					type: 'Organization'
				}
			})
		]
	} satisfies SEOInput
});
