import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb, buildCourse, buildVideoObject } from '#lib/seo/schemas.js';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = true;

export const load = () => ({
	seo: {
		title: 'Quickstart To Precision Trading',
		description:
			'Master precision entry and exit timing, chart setups, and institutional execution techniques in the Quickstart To Precision Trading class.',
		og: {
			type: 'article'
		},
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Classes', url: `${SITE}/classes` },
				{
					name: 'Quickstart To Precision Trading',
					url: `${SITE}/classes/quickstart-precision-trading-c`
				}
			]),
			buildCourse({
				url: `${SITE}/classes/quickstart-precision-trading-c`,
				name: 'Quickstart To Precision Trading',
				description:
					'Master precision entry and exit timing, chart setups, and institutional execution techniques.',
				provider: { name: 'Revolution Trading Pros', url: SITE },
				hasCourseInstance: {
					courseMode: 'Online'
				}
			}),
			buildVideoObject({
				name: 'Quickstart To Precision Trading',
				description:
					'Full class recording covering precision trading entries, risk management, and chart setups.',
				thumbnailUrl: `${SITE}/revolution-trading-pros.png`,
				uploadDate: '2025-12-01T00:00:00Z',
				contentUrl:
					'https://s3.amazonaws.com/simpler-cdn/dev/wp-content/uploads/2021/08/02153240/Quickstart-To-Precision-Trading-with-TG-Watkins.mp4',
				creator: {
					name: 'Revolution Trading Pros',
					url: SITE,
					type: 'Organization'
				}
			})
		]
	} satisfies SEOInput
});
