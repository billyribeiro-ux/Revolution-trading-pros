import type { Load } from '@sveltejs/kit';
import { apiFetch, API_ENDPOINTS } from '$lib/api/config';
import type { PaginatedPosts } from '$lib/types/post';
import type { SEOInput } from '$lib/seo/types';
import { buildBreadcrumb } from '$lib/seo/schemas';

const SITE = 'https://revolutiontradingpros.com';

export const prerender = false; // Disable prerendering - dynamic content
// SSR enabled for SEO - blog content should be server-rendered

const blogSeo: SEOInput = {
	title: 'Blog — Trading Insights & Tutorials',
	description:
		'Expert trading insights, tutorials, and market analysis from Revolution Trading Pros. Learn day trading, swing trading, options strategies, and more.',
	jsonld: [
		buildBreadcrumb([
			{ name: 'Home', url: `${SITE}/` },
			{ name: 'Blog', url: `${SITE}/blog` }
		]),
		{
			'@context': 'https://schema.org',
			'@type': 'Blog',
			'@id': `${SITE}/blog#blog`,
			name: 'Revolution Trading Pros Blog',
			description: 'Expert trading insights, tutorials, and market analysis',
			url: `${SITE}/blog`,
			publisher: {
				'@type': 'Organization',
				name: 'Revolution Trading Pros',
				logo: {
					'@type': 'ImageObject',
					url: `${SITE}/icon-512.png`
				}
			}
		}
	]
};

export const load: Load = async ({ fetch: svelteKitFetch }) => {
	try {
		// Pass SvelteKit's fetch to apiFetch for proper SSR support
		const posts = await apiFetch<PaginatedPosts>(API_ENDPOINTS.posts.list, {
			fetch: svelteKitFetch
		});

		return {
			posts: posts.data,
			pagination: {
				currentPage: posts.current_page,
				lastPage: posts.last_page,
				total: posts.total
			},
			seo: blogSeo
		};
	} catch (error) {
		console.error('Failed to load posts:', error);
		return {
			posts: [],
			pagination: {
				currentPage: 1,
				lastPage: 1,
				total: 0
			},
			error: error instanceof Error ? error.message : 'Failed to fetch posts',
			seo: blogSeo
		};
	}
};
