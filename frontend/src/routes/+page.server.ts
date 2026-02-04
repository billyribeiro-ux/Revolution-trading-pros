// ICT11+ PRODUCTION FIX: Hardcode API URL for server-side fetch
// Cloudflare Pages secrets not available via import.meta.env on server
const PRODUCTION_API_URL = 'https://revolution-trading-pros-api.fly.dev';

/**
 * ICT11+ Performance: Simple server load without streaming
 * Streaming was causing hydration issues with __sveltekit variable
 */
export const load = async ({ fetch }: { fetch: typeof globalThis.fetch }) => {
	// Fetch posts with timeout - don't block too long
	const posts = await fetchPosts(fetch);

	return {
		posts
	};
};

async function fetchPosts(fetch: typeof globalThis.fetch) {
	try {
		const controller = new AbortController();
		const timeoutId = setTimeout(() => controller.abort(), 5000); // Increased timeout

		const url = `${PRODUCTION_API_URL}/api/posts?per_page=6`;

		const response = await fetch(url, {
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
