import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { readPublicPost } from '#lib/server/public-posts.js';
import type { Post } from '#lib/types/post.js';
import type { SEOInput } from '#lib/seo/types.js';
import { articleSchema } from '#lib/seo/jsonld.js';
import { authors } from '#lib/authors/authors.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';
export const prerender = false;
export const load: PageServerLoad = async ({ params }) => {
	let post: Post;
	try {
		post = await readPublicPost(params.slug ?? '');
	} catch (cause) {
		if (cause instanceof Error && 'status' in cause && cause.status === 404)
			error(404, 'Blog post not found');
		error(503, 'Blog post is temporarily unavailable');
	}
	const path = `/blog/${post.slug}`,
		postUrl = `${PRODUCTION_ORIGIN}${path}`;
	const featuredImage = (() => {
		if (!post.featured_image) return undefined;
		try {
			const image = new URL(post.featured_image, PRODUCTION_ORIGIN);
			return ['https:', 'http:'].includes(image.protocol) ? image.href : undefined;
		} catch {
			return undefined;
		}
	})();
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
		!!featuredImage &&
		!!post.excerpt &&
		post.excerpt.length >= 50;
	const seo: SEOInput = {
		title: post.title,
		description,
		canonical: post.canonical_url || path,
		robots: { index: indexable, follow: true },
		og: {
			type: 'article',
			image: featuredImage,
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
							...(featuredImage ? { image: featuredImage } : {}),
							id: `${postUrl}#article`
						})
					]
				: []
	};
	return { post: { ...post, published_at: published ?? '' }, authorProfile: author, seo };
};
