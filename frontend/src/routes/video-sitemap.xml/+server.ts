/**
 * Google Video Sitemap (`MAP-01`, `MAP-04`)
 * Only emits real, verified public video watch pages.
 * Never emits placeholder or unseeded video IDs.
 */

import type { RequestHandler } from '@sveltejs/kit';

export const prerender = false;

export const GET: RequestHandler = async () => {
	const xml = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:video="http://www.google.com/schemas/sitemap-video/1.1">
</urlset>
`;
	return new Response(xml, {
		status: 200,
		headers: {
			'Content-Type': 'application/xml; charset=utf-8',
			'Cache-Control': 'public, max-age=3600, s-maxage=7200',
			'X-Robots-Tag': 'noindex'
		}
	});
};
