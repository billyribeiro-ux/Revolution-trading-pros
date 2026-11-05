import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'Sequence Subscribers | Admin',
		description:
			'Enroll confirmed contacts, search saved sequence enrollments, review execution history and cancel selected enrollments in the administration dashboard.'
	} satisfies SEOInput
});
