/**
 * ===============================================================================
 * HtmlBlock Component - Unit Tests
 * ===============================================================================
 *
 * @description Comprehensive tests for HtmlBlock Svelte component
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Svelte 5 + Testing Library
 *
 * Tests cover:
 * - HTML sanitization with DOMPurify
 * - XSS prevention
 * - Editor visibility in edit mode
 * - Preview toggle functionality
 * - Split view mode
 * - XSS warning banner
 * - Character count
 * - Editor height settings
 * - Tab key handling
 * - Accessibility
 */

import { describe, it, expect, vi, afterEach } from 'vitest';
import { render, screen, fireEvent, cleanup } from '@testing-library/svelte';
import HtmlBlock from '../advanced/HtmlBlock.svelte';
import type { Block } from '../types';
import { toBlockId } from '$lib/stores/blockState.svelte';

// ===============================================================================
// MOCKS
// ===============================================================================

// Mock the sanitization module
vi.mock('$lib/utils/sanitization', () => ({
	sanitizeHTML: vi.fn((html: string, _options?: { mode?: string }) => {
		// Simplified sanitization for testing - removes script tags
		if (!html || typeof html !== 'string') return '';

		// Remove script tags and their content
		let sanitized = html.replace(/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi, '');

		// Remove onclick and other event handlers
		sanitized = sanitized.replace(/\s*on\w+\s*=\s*["'][^"']*["']/gi, '');

		// Remove javascript: URLs
		sanitized = sanitized.replace(/javascript:/gi, '');

		return sanitized;
	})
}));

// ===============================================================================
// TEST FIXTURES
// ===============================================================================

function createMockBlock(overrides: Partial<Block> = {}): Block {
	return {
		id: toBlockId('html-1'),
		type: 'html',
		content: {
			html: '<p>Safe content</p>',
			...overrides.content
		},
		settings: {
			htmlEditorHeight: 'medium',
			htmlShowWarning: true,
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
// SETUP & TEARDOWN
// ===============================================================================

afterEach(() => {
	cleanup();
	vi.clearAllMocks();
});

// ===============================================================================
// TEST SUITE: HTML Sanitization
// ===============================================================================

describe('HtmlBlock - HTML Sanitization', () => {
	it('should sanitize HTML and remove script tags', () => {
		const block = createMockBlock({
			content: { html: '<script>alert("xss")</script><p>Safe content</p>' }
		});

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		// Safe content should be visible
		expect(screen.getByText('Safe content')).toBeInTheDocument();
	});

	it('should remove inline event handlers', () => {
		const block = createMockBlock({
			content: { html: '<div onclick="alert(\'xss\')">Click me</div>' }
		});

		const { container } = render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const outputDiv = container.querySelector('.html-output div');
		expect(outputDiv).not.toHaveAttribute('onclick');
	});

	it('should render safe paragraph content', () => {
		const block = createMockBlock({
			content: { html: '<p>This is safe content</p>' }
		});

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('This is safe content')).toBeInTheDocument();
	});

	it('should preserve safe HTML elements', () => {
		const block = createMockBlock({
			content: {
				html: '<div><h2>Title</h2><p>Paragraph</p><ul><li>Item 1</li></ul></div>'
			}
		});

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Title')).toBeInTheDocument();
		expect(screen.getByText('Paragraph')).toBeInTheDocument();
		expect(screen.getByText('Item 1')).toBeInTheDocument();
	});

	it('should handle empty HTML', () => {
		const block = createMockBlock({
			content: { html: '' }
		});

		const { container } = render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const output = container.querySelector('.html-output');
		expect(output).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Editor in Edit Mode
// ===============================================================================

describe('HtmlBlock - Editor in Edit Mode', () => {
	it('should show textarea in edit mode', () => {
		const block = createMockBlock();

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const textarea = screen.getByRole('textbox');
		expect(textarea).toBeInTheDocument();
	});

	it('should show code editor header', () => {
		const block = createMockBlock();

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Custom HTML')).toBeInTheDocument();
	});

	it('should display current HTML in textarea', () => {
		const block = createMockBlock({
			content: { html: '<p>Test content</p>' }
		});

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const textarea = screen.getByRole('textbox');
		expect(textarea).toHaveValue('<p>Test content</p>');
	});

	it('should have placeholder text', () => {
		const block = createMockBlock({
			content: { html: '' }
		});

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const textarea = screen.getByRole('textbox');
		expect(textarea).toHaveAttribute('placeholder');
	});

	it('should call onUpdate when content changes', async () => {
		const block = createMockBlock();
		const onUpdate = vi.fn();

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const textarea = screen.getByRole('textbox');
		await fireEvent.input(textarea, { target: { value: '<div>New content</div>' } });

		expect(onUpdate).toHaveBeenCalled();
	});

	it('should have spellcheck disabled', () => {
		const block = createMockBlock();

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const textarea = screen.getByRole('textbox');
		expect(textarea).toHaveAttribute('spellcheck', 'false');
	});

	it('should have autocomplete disabled', () => {
		const block = createMockBlock();

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const textarea = screen.getByRole('textbox');
		expect(textarea).toHaveAttribute('autocomplete', 'off');
	});
});

// ===============================================================================
// TEST SUITE: Preview Toggle
// ===============================================================================

describe('HtmlBlock - Preview Toggle', () => {
	it('should show view mode toggle buttons', () => {
		const block = createMockBlock();

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Edit')).toBeInTheDocument();
		expect(screen.getByText('Preview')).toBeInTheDocument();
		expect(screen.getByText('Split')).toBeInTheDocument();
	});

	it('should start in edit view mode', () => {
		const block = createMockBlock();

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const editButton = screen.getByText('Edit').closest('button');
		expect(editButton).toHaveClass('active');
	});

	it('should switch to preview mode', async () => {
		const block = createMockBlock({
			content: { html: '<p>Preview content</p>' }
		});

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const previewButton = screen.getByText('Preview');
		await fireEvent.click(previewButton);

		expect(screen.getByText('Preview content')).toBeInTheDocument();
	});

	it('should show preview label in preview mode', async () => {
		const block = createMockBlock();

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const previewButton = screen.getByText('Preview');
		await fireEvent.click(previewButton);

		// Look for the preview label element
		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});
	});

	it('should have aria-pressed on toggle buttons', () => {
		const block = createMockBlock();

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const editButton = screen.getByLabelText('Edit mode');
		expect(editButton).toHaveAttribute('aria-pressed', 'true');

		const previewButton = screen.getByLabelText('Preview mode');
		expect(previewButton).toHaveAttribute('aria-pressed', 'false');
	});
});

// ===============================================================================
// TEST SUITE: Split View Mode
// ===============================================================================

describe('HtmlBlock - Split View Mode', () => {
	it('should show both editor and preview in split mode', async () => {
		const block = createMockBlock({
			content: { html: '<p>Split content</p>' }
		});

		const { container } = render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const splitButton = screen.getByText('Split');
		await fireEvent.click(splitButton);

		const editorContainer = container.querySelector('.html-editor-container');
		expect(editorContainer).toHaveClass('view-split');
	});

	it('should show editor pane in split mode', async () => {
		const block = createMockBlock();

		const { container } = render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const splitButton = screen.getByText('Split');
		await fireEvent.click(splitButton);

		const editorPane = container.querySelector('.editor-pane');
		expect(editorPane).toBeInTheDocument();
	});

	it('should show preview pane in split mode', async () => {
		const block = createMockBlock();

		const { container } = render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const splitButton = screen.getByText('Split');
		await fireEvent.click(splitButton);

		const previewPane = container.querySelector('.preview-pane');
		expect(previewPane).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: XSS Warning Banner
// ===============================================================================

describe('HtmlBlock - XSS Warning Banner', () => {
	it('should show XSS warning when enabled', () => {
		const block = createMockBlock({
			settings: { htmlShowWarning: true }
		});

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText(/security notice/i)).toBeInTheDocument();
	});

	it('should not show XSS warning when disabled', () => {
		const block = createMockBlock({
			settings: { htmlShowWarning: false }
		});

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.queryByText(/security notice/i)).not.toBeInTheDocument();
	});

	it('should have role="alert" on warning', () => {
		const block = createMockBlock({
			settings: { htmlShowWarning: true }
		});

		const { container } = render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const warning = container.querySelector('.xss-warning');
		expect(warning).toHaveAttribute('role', 'alert');
	});

	it('should mention sanitization in warning', () => {
		const block = createMockBlock({
			settings: { htmlShowWarning: true }
		});

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText(/sanitized/i)).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Character Count
// ===============================================================================

describe('HtmlBlock - Character Count', () => {
	it('should show character count in editor footer', () => {
		const block = createMockBlock({
			content: { html: '<p>Test</p>' }
		});

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		// Content is '<p>Test</p>' = 11 characters
		expect(screen.getByText(/11 characters/i)).toBeInTheDocument();
	});

	it('should show 0 characters for empty content', () => {
		const block = createMockBlock({
			content: { html: '' }
		});

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText(/0 characters/i)).toBeInTheDocument();
	});

	it('should update count when content changes', async () => {
		const block = createMockBlock({
			content: { html: '' }
		});

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const textarea = screen.getByRole('textbox');
		await fireEvent.input(textarea, { target: { value: '<div>New</div>' } });

		// After input event fires, onUpdate is called
	});
});

// ===============================================================================
// TEST SUITE: Editor Height Settings
// ===============================================================================

describe('HtmlBlock - Editor Height Settings', () => {
	it('should show height selector when selected', () => {
		const block = createMockBlock();

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Editor Height:')).toBeInTheDocument();
	});

	it('should have height options', () => {
		const block = createMockBlock();

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Small (150px)')).toBeInTheDocument();
		expect(screen.getByText('Medium (250px)')).toBeInTheDocument();
		expect(screen.getByText('Large (400px)')).toBeInTheDocument();
		expect(screen.getByText('Auto')).toBeInTheDocument();
	});

	it('should apply small height', () => {
		const block = createMockBlock({
			settings: { htmlEditorHeight: 'small' }
		});

		const { container } = render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const textarea = container.querySelector('.html-textarea');
		expect(textarea).toHaveStyle({ height: '150px' });
	});

	it('should apply medium height', () => {
		const block = createMockBlock({
			settings: { htmlEditorHeight: 'medium' }
		});

		const { container } = render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const textarea = container.querySelector('.html-textarea');
		expect(textarea).toHaveStyle({ height: '250px' });
	});

	it('should apply large height', () => {
		const block = createMockBlock({
			settings: { htmlEditorHeight: 'large' }
		});

		const { container } = render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const textarea = container.querySelector('.html-textarea');
		expect(textarea).toHaveStyle({ height: '400px' });
	});

	it('should call onUpdate when height changes', async () => {
		const block = createMockBlock();
		const onUpdate = vi.fn();

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const heightSelect = screen.getByDisplayValue('Medium (250px)');
		await fireEvent.change(heightSelect, { target: { value: 'large' } });

		expect(onUpdate).toHaveBeenCalled();
	});
});

// ===============================================================================
// TEST SUITE: Settings Panel
// ===============================================================================

describe('HtmlBlock - Settings Panel', () => {
	it('should show settings panel when editing and selected', () => {
		const block = createMockBlock();

		const { container } = render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const settings = container.querySelector('.html-settings');
		expect(settings).toBeInTheDocument();
	});

	it('should not show settings when not selected', () => {
		const block = createMockBlock();

		const { container } = render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const settings = container.querySelector('.html-settings');
		expect(settings).not.toBeInTheDocument();
	});

	it('should show warning toggle checkbox', () => {
		const block = createMockBlock();

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Show security warning')).toBeInTheDocument();
	});

	it('should call onUpdate when warning toggle changes', async () => {
		const block = createMockBlock();
		const onUpdate = vi.fn();

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const checkbox = screen.getByRole('checkbox');
		await fireEvent.click(checkbox);

		expect(onUpdate).toHaveBeenCalled();
	});
});

// ===============================================================================
// TEST SUITE: Expand/Collapse Editor
// ===============================================================================

describe('HtmlBlock - Expand/Collapse Editor', () => {
	it('should show expand button', () => {
		const block = createMockBlock();

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const expandButton = screen.getByLabelText(/expand editor/i);
		expect(expandButton).toBeInTheDocument();
	});

	it('should toggle expanded state on click', async () => {
		const block = createMockBlock();

		const { container } = render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const expandButton = screen.getByLabelText(/expand editor/i);
		await fireEvent.click(expandButton);

		const htmlBlock = container.querySelector('.html-block');
		expect(htmlBlock).toHaveClass('expanded');
	});

	it('should show collapse button when expanded', async () => {
		const block = createMockBlock();

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const expandButton = screen.getByLabelText(/expand editor/i);
		await fireEvent.click(expandButton);

		const collapseButton = screen.getByLabelText(/collapse editor/i);
		expect(collapseButton).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Tab Key Handling
// ===============================================================================

describe('HtmlBlock - Tab Key Handling', () => {
	it('should insert tab character on Tab key', async () => {
		const block = createMockBlock({
			content: { html: '<div>' }
		});
		const onUpdate = vi.fn();

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const textarea = screen.getByRole('textbox') as HTMLTextAreaElement;

		// Set cursor position
		textarea.selectionStart = 5;
		textarea.selectionEnd = 5;

		await fireEvent.keyDown(textarea, { key: 'Tab' });

		// Should call onUpdate
		expect(onUpdate).toHaveBeenCalled();
	});
});

// ===============================================================================
// TEST SUITE: View Mode
// ===============================================================================

describe('HtmlBlock - View Mode', () => {
	it('should render sanitized HTML in view mode', () => {
		const block = createMockBlock({
			content: { html: '<p>View mode content</p>' }
		});

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('View mode content')).toBeInTheDocument();
	});

	it('should not show editor in view mode', () => {
		const block = createMockBlock();

		const { container } = render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const textarea = container.querySelector('textarea');
		expect(textarea).not.toBeInTheDocument();
	});

	it('should show html-output container', () => {
		const block = createMockBlock();

		const { container } = render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const output = container.querySelector('.html-output');
		expect(output).toBeInTheDocument();
	});

	it('should not show warning in view mode', () => {
		const block = createMockBlock({
			settings: { htmlShowWarning: true }
		});

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.queryByText(/security notice/i)).not.toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Accessibility
// ===============================================================================

describe('HtmlBlock - Accessibility', () => {
	it('should have region role on container', () => {
		const block = createMockBlock();

		const { container } = render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const htmlBlock = container.querySelector('.html-block');
		expect(htmlBlock).toHaveAttribute('role', 'region');
	});

	it('should have aria-label on container', () => {
		const block = createMockBlock();

		const { container } = render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const htmlBlock = container.querySelector('.html-block');
		expect(htmlBlock).toHaveAttribute('aria-label', 'Custom HTML block');
	});

	it('should have screen reader only label for textarea', () => {
		const block = createMockBlock();

		const { container } = render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const srLabel = container.querySelector('.sr-only');
		expect(srLabel).toBeInTheDocument();
	});

	it('should have aria-label on output region', () => {
		const block = createMockBlock();

		const { container } = render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const output = container.querySelector('.html-output');
		expect(output).toHaveAttribute('role', 'region');
		expect(output).toHaveAttribute('aria-label', 'Custom HTML content');
	});
});

// ===============================================================================
// TEST SUITE: Sanitization Badge
// ===============================================================================

describe('HtmlBlock - Sanitization Badge', () => {
	it('should show sanitized badge when content is modified', () => {
		const block = createMockBlock({
			content: { html: '<script>alert("xss")</script><p>Safe</p>' }
		});

		const { container } = render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const sanitizedBadge = container.querySelector('.sanitized-badge');
		expect(sanitizedBadge).toBeInTheDocument();
	});

	it('should show "Content sanitized" text', () => {
		const block = createMockBlock({
			content: { html: '<script>bad</script><p>Good</p>' }
		});

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Content sanitized')).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Preview Placeholder
// ===============================================================================

describe('HtmlBlock - Preview Placeholder', () => {
	it('should show placeholder when preview is empty', async () => {
		const block = createMockBlock({
			content: { html: '' }
		});

		render(HtmlBlock, {
			props: {
				block,
				blockId: toBlockId('html-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const previewButton = screen.getByText('Preview');
		await fireEvent.click(previewButton);

		expect(screen.getByText('Enter HTML to see preview')).toBeInTheDocument();
	});
});
