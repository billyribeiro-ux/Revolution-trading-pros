import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'Recorded Member Profile | Admin',
		description:
			'Review recorded member account facts, membership history, orders, email receipts, activity and internal notes.'
	} satisfies SEOInput
});
