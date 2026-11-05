import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'Sequence Analytics | Admin',
		description:
			'Review recorded email sequence enrollments, provider receipts, delivery events, engagement and queue status in the administration dashboard.'
	} satisfies SEOInput
});
