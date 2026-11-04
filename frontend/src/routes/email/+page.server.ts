import type { PageServerLoad } from './$types';
import type { SEOInput } from '#lib/seo/types.js';
import { requireAdmin } from '#lib/server/auth.js';

export const prerender = false;
export const load: PageServerLoad = (event) => {
	requireAdmin(event);
	return {
		seo: {
			title: 'Email Marketing Dashboard | Admin',
			description:
				'Manage email campaigns, templates, sequences and automations from the authenticated email marketing dashboard.',
			robots: { index: false, follow: false }
		} satisfies SEOInput
	};
};
