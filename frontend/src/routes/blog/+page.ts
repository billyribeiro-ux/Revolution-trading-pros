import type { Load } from '@sveltejs/kit';
import { apiFetch, API_ENDPOINTS } from '$lib/api/config';
import type { PaginatedPosts } from '$lib/types/post';
import type { SEOInput } from '$lib/seo/types';
import { breadcrumbSchema } from '$lib/seo/jsonld';
import { logger } from '$lib/utils/logger';

export const prerender = false; // Disable prerendering - dynamic content
// SSR enabled for SEO - blog content should be server-rendered

const SITE_URL = 'https://revolution-trading-pros.pages.dev';

const blogListingSeo: SEOInput = {
	title: 'Trading Blog & Market Insights',
	description:
		'Read the latest trading strategies, market analysis, and educational articles from Revolution Trading Pros — written by professional traders for active traders.',
	canonical: `${SITE_URL}/blog`,
	og: {
		type: 'website',
		title: 'Trading Blog & Market Insights | Revolution Trading Pros',
		description:
			'Professional trading strategies, market analysis, and educational articles from Revolution Trading Pros.'
	},
	jsonld: [
		{
			'@context': 'https://schema.org',
			'@type': 'Blog',
			'@id': `${SITE_URL}/blog#blog`,
			name: 'Revolution Trading Pros Blog',
			url: `${SITE_URL}/blog`,
			description: 'Professional trading strategies, market analysis, and educational articles.',
			inLanguage: 'en-US',
			publisher: {
				'@type': 'Organization',
				'@id': `${SITE_URL}/#organization`,
				name: 'Revolution Trading Pros'
			}
		},
		breadcrumbSchema(
			[
				{ name: 'Home', url: SITE_URL },
				{ name: 'Blog', url: `${SITE_URL}/blog` }
			],
			`${SITE_URL}/blog#breadcrumb`
		)
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
			seo: blogListingSeo
		};
	} catch (error) {
		logger.error('Failed to load posts:', error);
		return {
			posts: [],
			pagination: {
				currentPage: 1,
				lastPage: 1,
				total: 0
			},
			error: error instanceof Error ? error.message : 'Failed to fetch posts',
			seo: blogListingSeo
		};
	}
};
