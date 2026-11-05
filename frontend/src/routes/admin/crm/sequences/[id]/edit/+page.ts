import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'Edit Email Sequence | Admin',
		description:
			'Edit and save a paused email sequence, review its original content, and manage sender settings, message order and scheduling in the administration dashboard.'
	} satisfies SEOInput
});
