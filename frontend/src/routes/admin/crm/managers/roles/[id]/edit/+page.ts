import type { SEOInput } from '#lib/seo/types.js';
export const prerender = false;
export const load = () => ({
	seo: {
		title: 'CRM Security Management | Admin',
		description:
			'Manage explicit CRM role permissions and current account assignments with versioned changes and global administrator security controls.',
		robots: { index: false, follow: false }
	} satisfies SEOInput
});
