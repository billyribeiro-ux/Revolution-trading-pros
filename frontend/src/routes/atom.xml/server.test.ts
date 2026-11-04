import { describe, expect, it, vi } from 'vite-plus/test';
import { GET } from './+server';

describe('Atom feed block rendering', () => {
	it('renders valid legacy blocks and safely handles malformed block fields', async () => {
		const fetch = vi.fn<typeof globalThis.fetch>().mockResolvedValue(
			new Response(
				JSON.stringify({
					data: [
						{
							id: 1,
							slug: 'feed-fixture',
							title: 'Feed fixture',
							published_at: '2026-01-01T00:00:00Z',
							updated_at: '2026-01-02T00:00:00Z',
							content_blocks: [
								{
									type: 'paragraph',
									content: '<strong>Visible content</strong><script>alert(1)</script>'
								},
								{ type: 'heading', level: { unexpected: true }, text: 'Bounded heading' },
								{
									type: 'image',
									url: 'https://fixture.example.test/photo.png',
									alt: 'Quoted " alt'
								},
								{ type: 'paragraph', content: { unexpected: true }, text: 'Text fallback' },
								{ type: 'raw', content: { unexpected: true } },
								{ type: 'list', items: ['First item', { unexpected: true }] },
								{ type: 'code', code: '<script>literal</script>' }
							]
						}
					]
				})
			)
		);
		const response = await GET({ fetch } as unknown as Parameters<typeof GET>[0]);
		expect(response.status).toBe(200);
		const xml = await response.text();
		const document = new DOMParser().parseFromString(xml, 'application/xml');
		expect(document.querySelector('parsererror')).toBeNull();
		expect(xml).toContain('<strong>Visible content</strong>');
		expect(xml).toContain('<h2>Bounded heading</h2>');
		expect(xml).toContain('Text fallback');
		const content = new DOMParser().parseFromString(
			document.querySelector('entry content')?.textContent ?? '',
			'text/html'
		);
		expect(content.querySelector('code')?.textContent).toBe('<script>literal</script>');
		expect(content.querySelector('script')).toBeNull();
		expect(xml).not.toContain('[object Object]');
		expect(xml).not.toContain('<script>alert(1)</script>');
	});
});
