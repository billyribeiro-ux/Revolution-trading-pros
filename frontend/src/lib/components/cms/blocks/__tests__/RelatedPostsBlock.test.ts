/**
 * ===============================================================================
 * RelatedPostsBlock Component - Unit Tests
 * ===============================================================================
 *
 * @description Comprehensive tests for RelatedPostsBlock Svelte component
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Svelte 5 + Testing Library
 *
 * Tests cover:
 * - Grid layout rendering
 * - List layout rendering
 * - Edit mode placeholders
 * - Post count rendering (2, 3, 4 posts)
 * - Category and excerpt display settings
 * - Title editing
 * - Accessibility
 * - Responsive behavior classes
 */

import { describe, it, expect, vi, afterEach } from 'vitest';
import { render, screen, fireEvent, cleanup } from '@testing-library/svelte';
import RelatedPostsBlock from '../advanced/RelatedPostsBlock.svelte';
import type { Block } from '../types';
import { toBlockId } from '$lib/stores/blockState.svelte';

// ===============================================================================
// TEST FIXTURES
// ===============================================================================

function createMockBlock(overrides: Partial<Block> = {}): Block {
	return {
		id: toBlockId('related-1'),
		type: 'relatedPosts',
		content: {
			relatedPostsTitle: 'Related Articles',
			relatedPostsLayout: 'grid',
			relatedPostsCount: 3,
			...overrides.content
		},
		settings: {
			relatedPostsShowExcerpt: true,
			relatedPostsShowCategory: true,
			...overrides.settings
		},
		metadata: {
			createdAt: new Date().toISOString(),
			updatedAt: new Date().toISOString(),
			version: 1,
			...overrides.metadata
		}
	};
}

// ===============================================================================
// MOCK SETUP
// ===============================================================================

afterEach(() => {
	cleanup();
	vi.clearAllMocks();
});

// ===============================================================================
// TEST SUITE: Grid Layout
// ===============================================================================

describe('RelatedPostsBlock - Grid Layout', () => {
	it('should render grid layout container', () => {
		const block = createMockBlock({
			content: { relatedPostsLayout: 'grid' }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const postsContainer = container.querySelector('.related-posts-container');
		expect(postsContainer).toHaveClass('layout-grid');
	});

	it('should apply correct column class for grid', () => {
		const block = createMockBlock({
			content: { relatedPostsLayout: 'grid', relatedPostsCount: 3 }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const postsContainer = container.querySelector('.related-posts-container');
		expect(postsContainer).toHaveClass('cols-3');
	});

	it('should render posts in grid cards', () => {
		const block = createMockBlock({
			content: { relatedPostsLayout: 'grid', relatedPostsCount: 3 }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const postCards = container.querySelectorAll('.post-card');
		expect(postCards.length).toBe(3);
	});
});

// ===============================================================================
// TEST SUITE: List Layout
// ===============================================================================

describe('RelatedPostsBlock - List Layout', () => {
	it('should render list layout container', () => {
		const block = createMockBlock({
			content: { relatedPostsLayout: 'list' }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const postsContainer = container.querySelector('.related-posts-container');
		expect(postsContainer).toHaveClass('layout-list');
	});

	it('should render posts as list items', () => {
		const block = createMockBlock({
			content: { relatedPostsLayout: 'list', relatedPostsCount: 3 }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const postCards = container.querySelectorAll('.post-card');
		expect(postCards.length).toBe(3);
	});

	it('should have horizontal card layout in list mode', () => {
		const block = createMockBlock({
			content: { relatedPostsLayout: 'list' }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const postsContainer = container.querySelector('.layout-list');
		expect(postsContainer).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Edit Mode Placeholders
// ===============================================================================

describe('RelatedPostsBlock - Edit Mode Placeholders', () => {
	it('should show image placeholders in edit mode', () => {
		const block = createMockBlock();

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const placeholders = container.querySelectorAll('.image-placeholder');
		expect(placeholders.length).toBeGreaterThan(0);
	});

	it('should show "Post Image" text in placeholders', () => {
		const block = createMockBlock();

		render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const placeholderTexts = screen.getAllByText('Post Image');
		expect(placeholderTexts.length).toBeGreaterThan(0);
	});

	it('should show layout toggle in edit mode when selected', () => {
		const block = createMockBlock();

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const layoutToggle = container.querySelector('.layout-toggle');
		expect(layoutToggle).toBeInTheDocument();
	});

	it('should not show layout toggle when not selected', () => {
		const block = createMockBlock();

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const layoutToggle = container.querySelector('.layout-toggle');
		expect(layoutToggle).not.toBeInTheDocument();
	});

	it('should prevent link navigation in edit mode', async () => {
		const block = createMockBlock();

		render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		// In edit mode, links should not be anchor elements
		const link = screen.queryByRole('link');
		expect(link).not.toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Post Count
// ===============================================================================

describe('RelatedPostsBlock - Post Count', () => {
	it('should render 2 posts when count is 2', () => {
		const block = createMockBlock({
			content: { relatedPostsCount: 2 }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const postCards = container.querySelectorAll('.post-card');
		expect(postCards.length).toBe(2);
	});

	it('should render 3 posts when count is 3', () => {
		const block = createMockBlock({
			content: { relatedPostsCount: 3 }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const postCards = container.querySelectorAll('.post-card');
		expect(postCards.length).toBe(3);
	});

	it('should render 4 posts when count is 4', () => {
		const block = createMockBlock({
			content: { relatedPostsCount: 4 }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const postCards = container.querySelectorAll('.post-card');
		expect(postCards.length).toBe(4);
	});

	it('should apply cols-2 class for 2 posts', () => {
		const block = createMockBlock({
			content: { relatedPostsCount: 2, relatedPostsLayout: 'grid' }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const postsContainer = container.querySelector('.related-posts-container');
		expect(postsContainer).toHaveClass('cols-2');
	});

	it('should apply cols-4 class for 4 posts', () => {
		const block = createMockBlock({
			content: { relatedPostsCount: 4, relatedPostsLayout: 'grid' }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const postsContainer = container.querySelector('.related-posts-container');
		expect(postsContainer).toHaveClass('cols-4');
	});

	it('should cap at 4 posts maximum', () => {
		const block = createMockBlock({
			content: { relatedPostsCount: 10 }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const postCards = container.querySelectorAll('.post-card');
		expect(postCards.length).toBe(4);
	});

	it('should require minimum of 2 posts', () => {
		const block = createMockBlock({
			content: { relatedPostsCount: 1 }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const postCards = container.querySelectorAll('.post-card');
		expect(postCards.length).toBe(2);
	});
});

// ===============================================================================
// TEST SUITE: Category Display
// ===============================================================================

describe('RelatedPostsBlock - Category Display', () => {
	it('should show category when setting is enabled', () => {
		const block = createMockBlock({
			settings: { relatedPostsShowCategory: true }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const categories = container.querySelectorAll('.post-category');
		expect(categories.length).toBeGreaterThan(0);
	});

	it('should hide category when setting is disabled', () => {
		const block = createMockBlock({
			settings: { relatedPostsShowCategory: false }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const categories = container.querySelectorAll('.post-category');
		expect(categories.length).toBe(0);
	});

	it('should display category badge with correct styling', () => {
		const block = createMockBlock({
			settings: { relatedPostsShowCategory: true }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const category = container.querySelector('.post-category');
		expect(category).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Excerpt Display
// ===============================================================================

describe('RelatedPostsBlock - Excerpt Display', () => {
	it('should show excerpt when setting is enabled', () => {
		const block = createMockBlock({
			settings: { relatedPostsShowExcerpt: true }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const excerpts = container.querySelectorAll('.post-excerpt');
		expect(excerpts.length).toBeGreaterThan(0);
	});

	it('should hide excerpt when setting is disabled', () => {
		const block = createMockBlock({
			settings: { relatedPostsShowExcerpt: false }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const excerpts = container.querySelectorAll('.post-excerpt');
		expect(excerpts.length).toBe(0);
	});
});

// ===============================================================================
// TEST SUITE: Title
// ===============================================================================

describe('RelatedPostsBlock - Title', () => {
	it('should render section title', () => {
		const block = createMockBlock({
			content: { relatedPostsTitle: 'Related Articles' }
		});

		render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Related Articles')).toBeInTheDocument();
	});

	it('should make title editable in edit mode', () => {
		const block = createMockBlock();

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const title = container.querySelector('.related-posts-title[contenteditable="true"]');
		expect(title).toBeInTheDocument();
	});

	it('should call onUpdate when title changes', async () => {
		const block = createMockBlock();
		const onUpdate = vi.fn();

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const title = container.querySelector(
			'.related-posts-title[contenteditable="true"]'
		) as HTMLElement;
		await fireEvent.input(title, { target: { textContent: 'New Title' } });

		expect(onUpdate).toHaveBeenCalled();
	});
});

// ===============================================================================
// TEST SUITE: Settings Panel
// ===============================================================================

describe('RelatedPostsBlock - Settings Panel', () => {
	it('should show settings panel when editing and selected', () => {
		const block = createMockBlock();

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const settings = container.querySelector('.related-posts-settings');
		expect(settings).toBeInTheDocument();
	});

	it('should show post count selector', () => {
		const block = createMockBlock();

		render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Number of Posts:')).toBeInTheDocument();
	});

	it('should show category toggle', () => {
		const block = createMockBlock();

		render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Show Category')).toBeInTheDocument();
	});

	it('should show excerpt toggle', () => {
		const block = createMockBlock();

		render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Show Excerpt')).toBeInTheDocument();
	});

	it('should show settings note about content population', () => {
		const block = createMockBlock();

		render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText(/posts will be automatically populated/i)).toBeInTheDocument();
	});

	it('should call onUpdate when post count changes', async () => {
		const block = createMockBlock();
		const onUpdate = vi.fn();

		render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const countSelect = screen.getByDisplayValue('3 Posts');
		await fireEvent.change(countSelect, { target: { value: '4' } });

		expect(onUpdate).toHaveBeenCalled();
	});

	it('should call onUpdate when category toggle changes', async () => {
		const block = createMockBlock();
		const onUpdate = vi.fn();

		render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const checkboxes = screen.getAllByRole('checkbox');
		await fireEvent.click(checkboxes[0]);

		expect(onUpdate).toHaveBeenCalled();
	});
});

// ===============================================================================
// TEST SUITE: Layout Toggle
// ===============================================================================

describe('RelatedPostsBlock - Layout Toggle', () => {
	it('should show grid button in layout toggle', () => {
		const block = createMockBlock();

		render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const gridButton = screen.getByLabelText('Grid layout');
		expect(gridButton).toBeInTheDocument();
	});

	it('should show list button in layout toggle', () => {
		const block = createMockBlock();

		render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const listButton = screen.getByLabelText('List layout');
		expect(listButton).toBeInTheDocument();
	});

	it('should highlight active layout button', () => {
		const block = createMockBlock({
			content: { relatedPostsLayout: 'grid' }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const activeButton = container.querySelector('.toggle-btn.active');
		expect(activeButton).toBeInTheDocument();
	});

	it('should call onUpdate when layout changes', async () => {
		const block = createMockBlock({
			content: { relatedPostsLayout: 'grid' }
		});
		const onUpdate = vi.fn();

		render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const listButton = screen.getByLabelText('List layout');
		await fireEvent.click(listButton);

		expect(onUpdate).toHaveBeenCalledWith(
			expect.objectContaining({
				content: expect.objectContaining({
					relatedPostsLayout: 'list'
				})
			})
		);
	});
});

// ===============================================================================
// TEST SUITE: Accessibility
// ===============================================================================

describe('RelatedPostsBlock - Accessibility', () => {
	it('should have section role', () => {
		const block = createMockBlock();

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const section = container.querySelector('section');
		expect(section).toHaveAttribute('role', 'region');
	});

	it('should have aria-label on section', () => {
		const block = createMockBlock();

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const section = container.querySelector('section');
		expect(section).toHaveAttribute('aria-label', 'Related articles');
	});

	it('should have list role on posts container', () => {
		const block = createMockBlock();

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const postsContainer = container.querySelector('.related-posts-container');
		expect(postsContainer).toHaveAttribute('role', 'list');
	});

	it('should have listitem role on post cards', () => {
		const block = createMockBlock();

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const postCards = container.querySelectorAll('.post-card');
		postCards.forEach((card) => {
			expect(card).toHaveAttribute('role', 'listitem');
		});
	});

	it('should have aria-label on posts list', () => {
		const block = createMockBlock();

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const postsContainer = container.querySelector('.related-posts-container');
		expect(postsContainer).toHaveAttribute('aria-label', 'Related posts list');
	});

	it('should have loading="lazy" on images', () => {
		const block = createMockBlock();

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const images = container.querySelectorAll('.post-image img');
		images.forEach((img) => {
			expect(img).toHaveAttribute('loading', 'lazy');
		});
	});

	it('should have alt text on images', () => {
		const block = createMockBlock();

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const images = container.querySelectorAll('.post-image img');
		images.forEach((img) => {
			expect(img).toHaveAttribute('alt');
		});
	});

	it('should hide decorative icons', () => {
		const block = createMockBlock();

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const icons = container.querySelectorAll('[aria-hidden="true"]');
		expect(icons.length).toBeGreaterThan(0);
	});
});

// ===============================================================================
// TEST SUITE: Post Card Content
// ===============================================================================

describe('RelatedPostsBlock - Post Card Content', () => {
	it('should render post titles', () => {
		const block = createMockBlock();

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const titles = container.querySelectorAll('.post-title');
		expect(titles.length).toBeGreaterThan(0);
	});

	it('should render "Read more" links', () => {
		const block = createMockBlock();

		render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const readMoreLinks = screen.getAllByText('Read more');
		expect(readMoreLinks.length).toBeGreaterThan(0);
	});

	it('should render post images in view mode', () => {
		const block = createMockBlock();

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		// Images may not load due to placeholder paths, but image elements should exist
		const postImages = container.querySelectorAll('.post-image');
		expect(postImages.length).toBeGreaterThan(0);
	});

	it('should have clickable card links in view mode', () => {
		const block = createMockBlock();

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const links = container.querySelectorAll('a.post-card-inner');
		expect(links.length).toBeGreaterThan(0);
	});
});

// ===============================================================================
// TEST SUITE: Default Values
// ===============================================================================

describe('RelatedPostsBlock - Default Values', () => {
	it('should default to grid layout', () => {
		const block = createMockBlock({
			content: { relatedPostsLayout: undefined }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const postsContainer = container.querySelector('.related-posts-container');
		expect(postsContainer).toHaveClass('layout-grid');
	});

	it('should default to 3 posts', () => {
		const block = createMockBlock({
			content: { relatedPostsCount: undefined }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const postCards = container.querySelectorAll('.post-card');
		expect(postCards.length).toBe(3);
	});

	it('should default title to "Related Articles"', () => {
		const block = createMockBlock({
			content: { relatedPostsTitle: undefined }
		});

		render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Related Articles')).toBeInTheDocument();
	});

	it('should show excerpt by default', () => {
		const block = createMockBlock({
			settings: { relatedPostsShowExcerpt: undefined }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const excerpts = container.querySelectorAll('.post-excerpt');
		expect(excerpts.length).toBeGreaterThan(0);
	});

	it('should show category by default', () => {
		const block = createMockBlock({
			settings: { relatedPostsShowCategory: undefined }
		});

		const { container } = render(RelatedPostsBlock, {
			props: {
				block,
				blockId: toBlockId('related-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const categories = container.querySelectorAll('.post-category');
		expect(categories.length).toBeGreaterThan(0);
	});
});
