import type { SEOInput } from '#lib/seo/types.js';
export const load = () => ({
	seo: {
		title: 'Email Campaigns | Admin',
		description:
			'Manage saved campaign drafts, choose templates and member audiences, and review recorded delivery activity before submitting a broadcast.'
	} satisfies SEOInput
});
