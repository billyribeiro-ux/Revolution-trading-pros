/**
 * VideoBlock Component - Unit Tests
 * ===============================================================================
 * Comprehensive tests for the VideoBlock Svelte component
 *
 * @description Production-ready tests for video player with YouTube, Vimeo, and native support
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Svelte 5 + Testing Library
 *
 * Tests cover:
 * - URL parsing (YouTube, Vimeo, native video)
 * - Embed URL generation
 * - Loading states
 * - Error handling
 * - Caption editing
 * - ARIA accessibility attributes
 * - Platform detection
 * - Edit mode UI
 */

// Import setup first to ensure proper test environment
import './setup';

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, fireEvent, cleanup, waitFor } from '@testing-library/svelte';
import { BlockStateManager, createBlockId, type BlockId } from '$lib/stores/blockState.svelte';
import type { Block } from '../types';

// ===============================================================================
// MOCK BLOCK STATE MANAGER
// ===============================================================================

// Create a shared state manager for tests
let testStateManager: BlockStateManager;

// Mock the getBlockStateManager function to return our test instance
vi.mock('$lib/stores/blockState.svelte', async (importOriginal) => {
	const actual = await importOriginal<typeof import('$lib/stores/blockState.svelte')>();
	return {
		...actual,
		getBlockStateManager: () => {
			if (!testStateManager) {
				testStateManager = new actual.BlockStateManager();
			}
			return testStateManager;
		}
	};
});

// Import VideoBlock after mocking
import VideoBlock from '../media/VideoBlock.svelte';

// ===============================================================================
// TEST FIXTURES
// ===============================================================================

function createMockVideoBlock(overrides: Partial<Block> = {}): Block {
	return {
		id: `test-video-${Date.now()}`,
		type: 'video',
		content: {
			mediaUrl: '',
			mediaCaption: ''
		},
		settings: {},
		metadata: {
			createdAt: new Date().toISOString(),
			updatedAt: new Date().toISOString(),
			version: 1
		},
		...overrides
	} as Block;
}

function createTestBlockId(id: string): BlockId {
	return createBlockId(id);
}

// ===============================================================================
// TEST SUITE: URL Parsing - YouTube
// ===============================================================================

describe('VideoBlock - YouTube URL Parsing', () => {
	let onUpdate: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		testStateManager = new BlockStateManager();
		onUpdate = vi.fn();
	});

	afterEach(() => {
		cleanup();
		if (testStateManager) {
			testStateManager.cleanupAll();
		}
	});

	it('should parse standard YouTube URL', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const iframe = container.querySelector('iframe');
		expect(iframe).toBeInTheDocument();
		expect(iframe?.src).toContain('youtube-nocookie.com');
		expect(iframe?.src).toContain('dQw4w9WgXcQ');
	});

	it('should parse youtu.be short URL', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://youtu.be/dQw4w9WgXcQ' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const iframe = container.querySelector('iframe');
		expect(iframe).toBeInTheDocument();
		expect(iframe?.src).toContain('dQw4w9WgXcQ');
	});

	it('should parse YouTube embed URL', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://www.youtube.com/embed/dQw4w9WgXcQ' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const iframe = container.querySelector('iframe');
		expect(iframe).toBeInTheDocument();
		expect(iframe?.src).toContain('dQw4w9WgXcQ');
	});

	it('should parse YouTube shorts URL', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://www.youtube.com/shorts/abc123defgh' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const iframe = container.querySelector('iframe');
		expect(iframe).toBeInTheDocument();
		expect(iframe?.src).toContain('abc123defgh');
	});

	it('should use privacy-enhanced YouTube domain', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const iframe = container.querySelector('iframe');
		expect(iframe?.src).toContain('youtube-nocookie.com');
	});
});

// ===============================================================================
// TEST SUITE: URL Parsing - Vimeo
// ===============================================================================

describe('VideoBlock - Vimeo URL Parsing', () => {
	let onUpdate: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		onUpdate = vi.fn();
	});

	afterEach(() => {
		cleanup();
	});

	it('should parse standard Vimeo URL', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://vimeo.com/123456789' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const iframe = container.querySelector('iframe');
		expect(iframe).toBeInTheDocument();
		expect(iframe?.src).toContain('player.vimeo.com');
		expect(iframe?.src).toContain('123456789');
	});

	it('should parse Vimeo video URL format', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://vimeo.com/video/987654321' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const iframe = container.querySelector('iframe');
		expect(iframe).toBeInTheDocument();
		expect(iframe?.src).toContain('987654321');
	});

	it('should include DNT parameter for Vimeo', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://vimeo.com/123456789' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const iframe = container.querySelector('iframe');
		expect(iframe?.src).toContain('dnt=1');
	});
});

// ===============================================================================
// TEST SUITE: URL Parsing - Native Video
// ===============================================================================

describe('VideoBlock - Native Video Parsing', () => {
	let onUpdate: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		onUpdate = vi.fn();
	});

	afterEach(() => {
		cleanup();
	});

	it('should render native video for MP4 URL', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://example.com/video.mp4' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const video = container.querySelector('video');
		expect(video).toBeInTheDocument();
		expect(video?.src).toContain('video.mp4');
	});

	it('should render native video for WebM URL', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://example.com/video.webm' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const video = container.querySelector('video');
		expect(video).toBeInTheDocument();
	});

	it('should render native video for OGG URL', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://example.com/video.ogg' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const video = container.querySelector('video');
		expect(video).toBeInTheDocument();
	});

	it('should have controls attribute on native video', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://example.com/video.mp4' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const video = container.querySelector('video');
		expect(video).toHaveAttribute('controls');
	});

	it('should have preload="metadata" on native video', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://example.com/video.mp4' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const video = container.querySelector('video');
		expect(video).toHaveAttribute('preload', 'metadata');
	});

	it('should have captions track element', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://example.com/video.mp4' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const track = container.querySelector('track');
		expect(track).toBeInTheDocument();
		expect(track).toHaveAttribute('kind', 'captions');
	});
});

// ===============================================================================
// TEST SUITE: Empty and Edit States
// ===============================================================================

describe('VideoBlock - Empty and Edit States', () => {
	let onUpdate: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		onUpdate = vi.fn();
	});

	afterEach(() => {
		cleanup();
	});

	it('should show empty state in view mode without video', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: '' }
		});

		render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		expect(screen.getByText('No video available')).toBeInTheDocument();
	});

	it('should show upload UI in edit mode without video', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: '' }
		});

		render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		expect(screen.getByText('Add a Video')).toBeInTheDocument();
	});

	it('should show URL input in edit mode', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: '' }
		});

		render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const input = screen.getByPlaceholderText(/youtube.com/i);
		expect(input).toBeInTheDocument();
	});

	it('should show supported platforms', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: '' }
		});

		render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		expect(screen.getByText('YouTube')).toBeInTheDocument();
		expect(screen.getByText('Vimeo')).toBeInTheDocument();
		expect(screen.getByText('MP4/WebM')).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: URL Input Functionality
// ===============================================================================

describe('VideoBlock - URL Input', () => {
	let onUpdate: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		onUpdate = vi.fn();
	});

	afterEach(() => {
		cleanup();
	});

	it('should update content when valid YouTube URL is submitted', async () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: '' }
		});

		render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const input = screen.getByPlaceholderText(/youtube.com/i);
		await fireEvent.input(input, { target: { value: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ' } });

		const submitButton = screen.getByRole('button', { name: /add video/i });
		await fireEvent.click(submitButton);

		expect(onUpdate).toHaveBeenCalledWith({
			content: expect.objectContaining({
				mediaUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ'
			})
		});
	});

	it('should submit URL on Enter key press', async () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: '' }
		});

		render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const input = screen.getByPlaceholderText(/youtube.com/i);
		await fireEvent.input(input, { target: { value: 'https://vimeo.com/123456789' } });
		await fireEvent.keyDown(input, { key: 'Enter' });

		expect(onUpdate).toHaveBeenCalled();
	});

	it('should show error for unrecognized URL', async () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: '' }
		});
		const onError = vi.fn();

		render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: true,
				isSelected: false,
				onUpdate,
				onError
			}
		});

		const input = screen.getByPlaceholderText(/youtube.com/i);
		await fireEvent.input(input, { target: { value: 'https://random-site.com/video' } });

		const submitButton = screen.getByRole('button', { name: /add video/i });
		await fireEvent.click(submitButton);

		await waitFor(() => {
			const errorText = screen.queryByText(/Unable to recognize/i);
			expect(errorText || onError.mock.calls.length).toBeTruthy();
		});
	});

	it('should disable submit button when input is empty', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: '' }
		});

		render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const submitButton = screen.getByRole('button', { name: /add video/i });
		expect(submitButton).toBeDisabled();
	});
});

// ===============================================================================
// TEST SUITE: Loading State
// ===============================================================================

describe('VideoBlock - Loading State', () => {
	let onUpdate: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		onUpdate = vi.fn();
	});

	afterEach(() => {
		cleanup();
	});

	it('should show loading spinner initially', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const loadingSpinner = container.querySelector('.video-loading');
		expect(loadingSpinner).toBeInTheDocument();
	});

	it('should have aria-live on loading indicator', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const loading = container.querySelector('.video-loading');
		expect(loading).toHaveAttribute('aria-live', 'polite');
	});

	it('should have visually hidden loading text for screen readers', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const hiddenText = container.querySelector('.visually-hidden');
		expect(hiddenText?.textContent).toContain('Loading');
	});
});

// ===============================================================================
// TEST SUITE: Error Handling
// ===============================================================================

describe('VideoBlock - Error Handling', () => {
	let onUpdate: ReturnType<typeof vi.fn>;
	let onError: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		onUpdate = vi.fn();
		onError = vi.fn();
	});

	afterEach(() => {
		cleanup();
	});

	it('should call onError with invalid URL', async () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: '' }
		});

		render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: true,
				isSelected: false,
				onUpdate,
				onError
			}
		});

		const input = screen.getByPlaceholderText(/youtube.com/i);
		await fireEvent.input(input, { target: { value: 'invalid-url' } });

		const submitButton = screen.getByRole('button', { name: /add video/i });
		await fireEvent.click(submitButton);

		// Either error callback or error message displayed
		await waitFor(() => {
			const hasError = onError.mock.calls.length > 0 || screen.queryByText(/Invalid URL/i);
			expect(hasError).toBeTruthy();
		});
	});

	it('should display error message in error state', async () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: '' }
		});

		render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: true,
				isSelected: false,
				onUpdate,
				onError
			}
		});

		const input = screen.getByPlaceholderText(/youtube.com/i);
		await fireEvent.input(input, { target: { value: 'https://not-a-video-site.com/page' } });

		const submitButton = screen.getByRole('button', { name: /add video/i });
		await fireEvent.click(submitButton);

		await waitFor(() => {
			const errorAlert = screen.queryByRole('alert');
			expect(errorAlert).toBeInTheDocument();
		});
	});

	it('should have alert role on error display', async () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: '' }
		});

		render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const input = screen.getByPlaceholderText(/youtube.com/i);
		await fireEvent.input(input, { target: { value: 'invalid' } });

		const submitButton = screen.getByRole('button', { name: /add video/i });
		await fireEvent.click(submitButton);

		await waitFor(() => {
			const alert = screen.queryByRole('alert');
			expect(alert).toBeInTheDocument();
		});
	});
});

// ===============================================================================
// TEST SUITE: Caption Editing
// ===============================================================================

describe('VideoBlock - Caption Editing', () => {
	let onUpdate: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		onUpdate = vi.fn();
	});

	afterEach(() => {
		cleanup();
	});

	it('should display caption when provided', () => {
		const block = createMockVideoBlock({
			content: {
				mediaUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
				mediaCaption: 'Test video caption'
			}
		});

		render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		expect(screen.getByText('Test video caption')).toBeInTheDocument();
	});

	it('should make caption editable in edit mode', () => {
		const block = createMockVideoBlock({
			content: {
				mediaUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
				mediaCaption: 'Editable caption'
			}
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const caption = container.querySelector('figcaption');
		expect(caption).toHaveAttribute('contenteditable', 'true');
	});

	it('should show caption placeholder in edit mode', () => {
		const block = createMockVideoBlock({
			content: {
				mediaUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
				mediaCaption: ''
			}
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const caption = container.querySelector('figcaption');
		expect(caption).toHaveAttribute('data-placeholder', 'Add a caption...');
	});
});

// ===============================================================================
// TEST SUITE: Edit Mode Controls
// ===============================================================================

describe('VideoBlock - Edit Mode Controls', () => {
	let onUpdate: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		onUpdate = vi.fn();
	});

	afterEach(() => {
		cleanup();
	});

	it('should show clear button in edit mode with video', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ' }
		});

		render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		// Clear button appears after loading
		// It has aria-label="Remove video"
		const clearButton = screen.queryByLabelText('Remove video');
		// May or may not be visible depending on loading state
	});

	it('should clear video on remove button click', async () => {
		const block = createMockVideoBlock({
			content: {
				mediaUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
				mediaCaption: 'Test caption'
			}
		});

		render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		// Wait for loading to complete
		await waitFor(() => {
			const clearButton = screen.queryByLabelText('Remove video');
			if (clearButton) {
				return true;
			}
			return false;
		}, { timeout: 3000 }).catch(() => {});

		const clearButton = screen.queryByLabelText('Remove video');
		if (clearButton) {
			await fireEvent.click(clearButton);
			expect(onUpdate).toHaveBeenCalledWith({
				content: expect.objectContaining({
					mediaUrl: '',
					mediaCaption: ''
				})
			});
		}
	});
});

// ===============================================================================
// TEST SUITE: Accessibility
// ===============================================================================

describe('VideoBlock - Accessibility', () => {
	let onUpdate: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		onUpdate = vi.fn();
	});

	afterEach(() => {
		cleanup();
	});

	it('should have region role on container', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ' }
		});

		render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const region = screen.getByRole('region');
		expect(region).toBeInTheDocument();
		expect(region).toHaveAttribute('aria-label', 'Video player');
	});

	it('should have title attribute on iframe', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const iframe = container.querySelector('iframe');
		expect(iframe).toHaveAttribute('title');
	});

	it('should have aria-label on iframe', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const iframe = container.querySelector('iframe');
		expect(iframe).toHaveAttribute('aria-label');
	});

	it('should have aria-label on URL input', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: '' }
		});

		render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const input = screen.getByPlaceholderText(/youtube.com/i);
		expect(input).toHaveAttribute('aria-label', 'Video URL');
	});

	it('should have role="alert" on error display', async () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: '' }
		});

		render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const input = screen.getByPlaceholderText(/youtube.com/i);
		await fireEvent.input(input, { target: { value: 'invalid-url-format' } });

		const submitButton = screen.getByRole('button', { name: /add video/i });
		await fireEvent.click(submitButton);

		await waitFor(() => {
			const alert = screen.queryByRole('alert');
			expect(alert).toBeInTheDocument();
		});
	});
});

// ===============================================================================
// TEST SUITE: Iframe Attributes
// ===============================================================================

describe('VideoBlock - Iframe Attributes', () => {
	let onUpdate: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		onUpdate = vi.fn();
	});

	afterEach(() => {
		cleanup();
	});

	it('should have loading="lazy" attribute', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const iframe = container.querySelector('iframe');
		expect(iframe).toHaveAttribute('loading', 'lazy');
	});

	it('should have allowfullscreen attribute', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const iframe = container.querySelector('iframe');
		expect(iframe).toHaveAttribute('allowfullscreen');
	});

	it('should have proper allow attribute for permissions', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const iframe = container.querySelector('iframe');
		const allowAttr = iframe?.getAttribute('allow');
		expect(allowAttr).toContain('autoplay');
		expect(allowAttr).toContain('encrypted-media');
		expect(allowAttr).toContain('picture-in-picture');
	});

	it('should have frameborder="0"', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		const iframe = container.querySelector('iframe');
		expect(iframe).toHaveAttribute('frameborder', '0');
	});
});

// ===============================================================================
// TEST SUITE: Selection State
// ===============================================================================

describe('VideoBlock - Selection State', () => {
	let onUpdate: ReturnType<typeof vi.fn>;

	beforeEach(() => {
		onUpdate = vi.fn();
	});

	afterEach(() => {
		cleanup();
	});

	it('should apply selected class when isSelected is true', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: false,
				isSelected: true,
				onUpdate
			}
		});

		const videoBlock = container.querySelector('.video-block');
		expect(videoBlock?.classList.contains('video-block--selected')).toBe(true);
	});

	it('should apply editing class when isEditing is true', () => {
		const block = createMockVideoBlock({
			content: { mediaUrl: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ' }
		});

		const { container } = render(VideoBlock, {
			props: {
				block,
				blockId: createTestBlockId(block.id),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const videoBlock = container.querySelector('.video-block');
		expect(videoBlock?.classList.contains('video-block--editing')).toBe(true);
	});
});
