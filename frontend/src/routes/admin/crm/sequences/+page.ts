import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'Email Sequences | Admin',
		description:
			'Manage saved email sequences, consented enrollments, scheduling, recorded delivery activity and sequence settings in the administration dashboard.'
	} satisfies SEOInput
});
