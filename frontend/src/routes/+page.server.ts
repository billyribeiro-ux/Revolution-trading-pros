import type { PageServerLoad } from './$types';
import type { SEOInput } from '$lib/seo/types';
import { env } from '$env/dynamic/private';

// API base resolves from runtime env (reads .env.local in dev, secrets in prod)
// with a Fly fallback so prerender / Cloudflare deploys still work without env vars.
const API_BASE_URL = env.API_BASE_URL || env.BACKEND_URL || 'http://localhost:8080';

/**
 * ICT11+ Performance: Simple server load without streaming
 * Streaming was causing hydration issues with __sveltekit variable
 */
export const load: PageServerLoad = async ({ fetch }) => {
	// Fetch posts with timeout - don't block too long
	const posts = await fetchPosts(fetch);

	const seo: SEOInput = {
		// Use the site default title (already templated as
		// "Revolution Trading Pros — Professional Trading Education & Tools")
		title: null,
		titleTemplate: null,
		og: { type: 'website' },
		jsonld: [
			{
				'@context': 'https://schema.org',
				'@type': 'FinancialService',
				'@id': 'https://revolution-trading-pros.pages.dev/#financialservice',
				name: 'Revolution Trading Pros',
				description:
					'Professional trading education, live trading rooms, custom indicators, and alert services for active traders.',
				url: 'https://revolution-trading-pros.pages.dev',
				serviceType: 'Trading Education',
				areaServed: 'Worldwide'
			}
		]
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

		const response = await fetch(`${API_BASE_URL}/api/posts?per_page=6`, {
			signal: controller.signal,
			headers: {
				Accept: 'application/json'
			}
		});

		clearTimeout(timeoutId);

		if (!response.ok) {
			return [];
		}

		const data = await response.json();
		return data.data || [];
	} catch {
		return [];
	}
}
