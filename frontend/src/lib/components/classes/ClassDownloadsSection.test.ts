import { afterEach, expect, it, vi } from 'vite-plus/test';
import { cleanup, render, screen } from '@testing-library/svelte';
import ClassDownloadsSection from './ClassDownloadsSection.svelte';
afterEach(() => {
	cleanup();
	vi.unstubAllGlobals();
});
it('renders the real member API nested downloads shape', async () => {
	vi.stubGlobal(
		'fetch',
		vi.fn().mockResolvedValue(
			Response.json({
				success: true,
				data: {
					is_enrolled: true,
					downloads: [
						{
							id: 1,
							title: 'Fixture material',
							file_name: 'Fixture material.pdf',
							file_size_bytes: 1024,
							download_url: '/fixture.pdf',
							file_type: 'pdf',
							created_at: '2026-01-01T00:00:00Z',
							updated_at: '2026-01-01T00:00:00Z'
						}
					]
				}
			})
		)
	);
	render(ClassDownloadsSection, { courseSlug: 'fixture-course' });
	expect(await screen.findByText('Fixture material.pdf')).toBeVisible();
	expect(screen.queryByText('No downloads available')).not.toBeInTheDocument();
});
it('reports a malformed member response instead of claiming there are no files', async () => {
	vi.stubGlobal(
		'fetch',
		vi.fn().mockResolvedValue(Response.json({ success: true, data: { downloads: {} } }))
	);
	render(ClassDownloadsSection, { courseSlug: 'fixture-course' });
	expect(await screen.findByText('Invalid class downloads response')).toBeVisible();
	expect(screen.queryByText('No downloads available')).not.toBeInTheDocument();
});
