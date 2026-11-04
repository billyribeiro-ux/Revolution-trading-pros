import type { SEOInput } from '#lib/seo/types.js';
export const prerender = false;
export const load = () => ({
	seo: {
		title: 'CRM Webhooks',
		description: 'Manage CRM event subscriptions, signed notifications and delivery history.',
		robots: { index: false, follow: false }
	} satisfies SEOInput
});
