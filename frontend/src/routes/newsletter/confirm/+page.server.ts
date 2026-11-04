import type { PageServerLoad } from './$types';
import type { SEOInput } from '#lib/seo/types.js';
import { newsletterCallback } from '#lib/server/newsletter-callback.js';

export const prerender = false;
export const csr = false;
export const load: PageServerLoad = async (event) => ({
	...(await newsletterCallback(event, 'confirm')),
	seo: {
		title: 'Newsletter Confirmation',
		description:
			'Confirm the newsletter subscription associated with your email link and review the recorded subscription result.',
		canonical: '/newsletter/confirm',
		robots: { index: false, follow: false }
	} satisfies SEOInput
});
