import type { PageLoad } from './$types';
import type { SEOInput } from '#lib/seo/types.js';
import { buildBreadcrumb } from '#lib/seo/schemas.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

const SITE = PRODUCTION_ORIGIN;

export const prerender = false;

export const load: PageLoad = ({ data, url }) => {
	const pageParam = Number(url.searchParams.get('page') ?? '1');
	const pageNum = Number.isFinite(pageParam) && pageParam > 1 ? Math.floor(pageParam) : 1;
	const canonicalUrl = pageNum > 1 ? `${SITE}/blog?page=${pageNum}` : `${SITE}/blog`;

	const blogSeo: SEOInput = {
		title: pageNum > 1 ? `Trading Blog — Page ${pageNum}` : 'Trading Blog — Insights & Tutorials',
		description:
			'Expert trading insights, tutorials, and market analysis from Revolution Trading Pros. Learn day trading, swing trading, options strategies, and more.',
		canonical: canonicalUrl,
		jsonld: [
			buildBreadcrumb([
				{ name: 'Home', url: `${SITE}/` },
				{ name: 'Trading Blog', url: `${SITE}/blog` }
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

	return {
		posts: data?.posts ?? [],
		pagination: data?.pagination ?? {
			currentPage: pageNum,
			lastPage: 1,
			total: 0
		},
		seo: blogSeo
	};
};
