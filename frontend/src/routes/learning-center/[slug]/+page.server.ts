import type { PageServerLoad } from './$types';
import type { SEOInput } from '#lib/seo/types.js';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';
import { buildVideoObject } from '#lib/seo/schemas.js';
import { readPublicVideo } from '#lib/server/seo-publications.js';
export const prerender = false;
export const load: PageServerLoad = async ({ params }) => {
	const video = await readPublicVideo(params.slug),
		authorUrl = `${PRODUCTION_ORIGIN}/authors/${video.author_slug}`;
	return {
		video,
		seo: {
			title: video.title,
			description: video.description.slice(0, 160),
			canonical: video.path,
			og: { type: 'video.other', image: video.thumbnail_url, imageAlt: video.title },
			jsonld: [
				buildVideoObject({
					name: video.title,
					description: video.description,
					thumbnailUrl: video.thumbnail_url,
					uploadDate: video.published_at,
					duration: `PT${video.duration}S`,
					...(video.playback_kind === 'video'
						? { contentUrl: video.playback_url }
						: { embedUrl: video.playback_url }),
					author: { name: video.author_name, url: authorUrl },
					creator: { name: video.author_name, url: authorUrl }
				})
			]
		} satisfies SEOInput
	};
};
