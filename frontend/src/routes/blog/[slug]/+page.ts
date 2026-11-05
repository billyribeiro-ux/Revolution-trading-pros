import type { Load } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';
import { apiFetch, API_ENDPOINTS } from '#lib/api/config.js';
import type { Post } from '#lib/types/post.js';
import type { SEOInput } from '#lib/seo/types.js';
import { articleSchema } from '#lib/seo/jsonld.js';
import { authors } from '#lib/authors/authors.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';
export const prerender = false;
export const load: Load = async ({ params, fetch }) => {
	let post: Post;
	try {
		post = await apiFetch<Post>(API_ENDPOINTS.posts.single(params.slug ?? ''), { fetch });
	} catch (cause) {
		if (cause instanceof Error && 'status' in cause && cause.status === 404)
			error(404, 'Blog post not found');
		error(503, 'Blog post is temporarily unavailable');
	}
	const path = `/blog/${post.slug}`,
		postUrl = `${PRODUCTION_ORIGIN}${path}`;
	const author = authors.find((author) => author.name === post.author?.name) ?? null;
	const published =
		post.published_at &&
		Number.isFinite(
			Date.parse(`${post.published_at}${/Z$|[+-]\d\d:\d\d$/.test(post.published_at) ? '' : 'Z'}`)
		)
			? new Date(
					`${post.published_at}${/Z$|[+-]\d\d:\d\d$/.test(post.published_at) ? '' : 'Z'}`
				).toISOString()
			: null;
	const description = post.excerpt || post.meta_description || '';
	const indexable =
		post.indexable !== false &&
		!!author &&
		!!published &&
		Date.parse(published) <= Date.now() &&
		!!post.featured_image &&
		!!post.excerpt &&
		post.excerpt.length >= 50;
	const seo: SEOInput = {
		title: post.title,
		description,
		canonical: post.canonical_url || path,
		robots: { index: indexable, follow: true },
		og: {
			type: 'article',
			image: post.featured_image,
			imageAlt: post.title,
			article: {
				...(published ? { publishedTime: published } : {}),
				...(author ? { author: `${PRODUCTION_ORIGIN}/authors/${author.slug}` } : {})
			}
		},
		jsonld:
			author && published
				? [
						articleSchema({
							headline: post.title,
							url: postUrl,
							datePublished: published,
							authorName: author.name,
							authorUrl: `${PRODUCTION_ORIGIN}/authors/${author.slug}`,
							publisherName: 'Revolution Trading Pros',
							publisherLogo: `${PRODUCTION_ORIGIN}/icon-512.png`,
							type: 'BlogPosting',
							description,
							...(post.featured_image ? { image: post.featured_image } : {}),
							id: `${postUrl}#article`
						})
					]
				: []
	};
	return { post: { ...post, published_at: published ?? '' }, authorProfile: author, seo };
};
