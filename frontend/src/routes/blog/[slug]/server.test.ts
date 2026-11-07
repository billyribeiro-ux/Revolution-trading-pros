// @vitest-environment node
import { expect, it, vi } from 'vite-plus/test';

const readPublicPost = vi.hoisted(() => vi.fn());
vi.mock('#lib/server/public-posts.js', () => ({ readPublicPost }));

import { load } from './+page.server';
import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

it('uses an absolute image URL in article metadata when delivery is same origin', async () => {
	readPublicPost.mockResolvedValue({
		title: 'A practical trading article',
		slug: 'practical-trading',
		excerpt: 'A useful and sufficiently detailed article description for search results.',
		featured_image: '/api/media/image/01234567-89ab-cdef-0123-456789abcdef',
		published_at: '2026-01-01T00:00:00Z',
		author: { name: 'Billy Ribeiro' }
	});
	const result = await load({ params: { slug: 'practical-trading' } } as Parameters<
		typeof load
	>[0]);
	if (!result || typeof result !== 'object' || !('seo' in result)) {
		throw new Error('Expected article page data');
	}
	const seo = result.seo;
	if (!seo) throw new Error('Expected article SEO data');
	const image = `${PRODUCTION_ORIGIN}/api/media/image/01234567-89ab-cdef-0123-456789abcdef`;
	expect(seo.og?.image).toBe(image);
	expect(JSON.stringify(seo.jsonld)).toContain(image);
});
