import type { PageServerLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';
import { env } from '$env/dynamic/private';

// API base resolves from runtime env (reads .env.local in dev, secrets in prod)
// with a Fly fallback so prerender / Cloudflare deploys still work without env vars.
const API_BASE_URL =
	env.API_BASE_URL || env.BACKEND_URL || 'https://revolution-trading-pros-api.fly.dev';

/**
 * ICT11+ Performance: Simple server load without streaming
 * Streaming was causing hydration issues with __sveltekit variable
 */
export const load: PageServerLoad = async ({ fetch }) => {
	// Fetch posts with timeout - don't block too long
	const posts = await fetchPosts(fetch);

	const seo: SEOInput = {
		title: null,
		titleTemplate: null,
		og: {
			type: 'website'
		}
	};

	return {
		posts,
		seo
	};
};

async function fetchPosts(fetch: typeof globalThis.fetch) {
	try {
		const controller = new AbortController();
		const timeoutId = setTimeout(() => controller.abort(), 5000); // Increased timeout

		const url = `${API_BASE_URL}/api/posts?per_page=6`;
		console.log('[SSR] Fetching posts from:', url);

		const response = await fetch(url, {
			signal: controller.signal,
			headers: {
				Accept: 'application/json'
			}
		});

		clearTimeout(timeoutId);

		if (!response.ok) {
			console.log('[SSR] Posts fetch failed:', response.status);
			return [];
		}

		const data = await response.json();
		console.log('[SSR] Posts fetched:', data.data?.length || 0);
		return data.data || [];
	} catch (e) {
		console.log('[SSR] Posts fetch error:', e);
		return [];
	}
}
