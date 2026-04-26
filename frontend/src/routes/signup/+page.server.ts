import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

/**
 * FIX-2026-04-26: 301 permanent redirect from /signup to /register.
 *
 * /register is the canonical, branded signup flow (1237 LOC, linked from
 * /login and from courses/options-trading, courses/swing-trading-pro,
 * verify-email/[id]/[hash]). /signup (296 LOC, plain Tailwind) was a
 * duplicate that risked drift. This page.server.ts issues an HTTP 301 so
 * external links + bookmarks resolve to the canonical page.
 *
 * The /signup/+page.svelte is intentionally left on disk — the SvelteKit
 * router prefers +page.server.ts's load() throw over rendering the .svelte
 * file. If the redirect ever needs reversal, delete this file.
 */
export const load: PageServerLoad = async ({ url }) => {
	const target = `/register${url.search}`;
	throw redirect(301, target);
};
