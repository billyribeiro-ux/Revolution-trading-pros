/**
 * Google News Sitemap (`MAP-01`, `MAP-04`)
 * Only emits real, published news articles within the last 48 hours.
 * Never emits fabricated or placeholder URLs.
 */

import type { RequestHandler } from '@sveltejs/kit';

export const prerender = false;

export const GET: RequestHandler = async () => {
	const xml = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:news="http://www.google.com/schemas/sitemap-news/0.9">
</urlset>
`;
	return new Response(xml, {
		status: 200,
		headers: {
			'Content-Type': 'application/xml; charset=utf-8',
			'Cache-Control': 'public, max-age=900, s-maxage=1800',
			'X-Robots-Tag': 'noindex'
		}
	});
};
