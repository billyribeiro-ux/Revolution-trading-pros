import type { Load } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';
import { apiFetch, API_ENDPOINTS } from '$lib/api/config';
import type { Post } from '$lib/types/post';
import type { SEOInput } from '$lib/seo/types';
import { articleSchema, breadcrumbSchema } from '$lib/seo/jsonld';

const SITE_URL = 'https://revolution-trading-pros.pages.dev';

export const prerender = false; // Disable prerendering - dynamic content
// SSR enabled for SEO - blog content should be server-rendered

export const load: Load = async ({ params, url }) => {
	let post: Post;

	try {
		post = await apiFetch<Post>(API_ENDPOINTS.posts.single(params.slug ?? ''));
	} catch (err) {
		console.error('Failed to load post:', err);
		error(404, {
        			message: 'Blog post not found'
        		});
	}

	const postUrl = `${SITE_URL}${url.pathname}`;
	const postTitle = post.title ?? params.slug ?? 'Blog Post';
	const postDescription =
		post.excerpt ?? post.meta_description ?? `Read ${postTitle} on Revolution Trading Pros`;
	const postImage = post.featured_image ?? `${SITE_URL}/og-default.png`;
	const authorName = post.author?.name ?? 'Revolution Trading Pros';
	const publishedDate = post.published_at ?? new Date().toISOString();

	const seo: SEOInput = {
		title: postTitle,
		description: postDescription,
		canonical: postUrl,
		og: {
			title: postTitle,
			description: postDescription,
			type: 'article',
			image: postImage,
			imageAlt: postTitle,
			article: {
				publishedTime: publishedDate,
				modifiedTime: publishedDate,
				author: authorName,
				section: 'Trading'
			}
		},
		twitter: {
			title: postTitle,
			description: postDescription,
			image: postImage,
			imageAlt: postTitle
		},
		jsonld: [
			articleSchema({
				headline: postTitle,
				url: postUrl,
				datePublished: publishedDate,
				dateModified: publishedDate,
				authorName,
				publisherName: 'Revolution Trading Pros',
				publisherLogo: `${SITE_URL}/logo.png`,
				type: 'BlogPosting',
				description: postDescription,
				image: postImage,
				articleSection: 'Trading',
				id: `${postUrl}#article`
			}),
			breadcrumbSchema(
				[
					{ name: 'Home', url: SITE_URL },
					{ name: 'Blog', url: `${SITE_URL}/blog` },
					{ name: postTitle, url: postUrl }
				],
				`${postUrl}#breadcrumb`
			)
		]
	};

	return {
		post,
		seo
	};
};
