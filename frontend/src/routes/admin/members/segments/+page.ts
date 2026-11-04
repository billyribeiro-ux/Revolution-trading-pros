import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'Member Segments | Admin',
		description:
			'Manage member segments, saved member filters and tags. Review matching members and recorded segment activity.'
	} satisfies SEOInput
});
