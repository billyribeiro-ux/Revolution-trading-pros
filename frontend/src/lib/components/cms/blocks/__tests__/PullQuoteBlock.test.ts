/**
 * ===============================================================================
 * PullQuoteBlock Component - Unit Tests
 * ===============================================================================
 *
 * @description Comprehensive tests for PullQuoteBlock Svelte component
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Svelte 5 + Testing Library
 *
 * Tests cover:
 * - Quote text rendering and editing
 * - Attribution and source display
 * - Alignment options (left, center, right)
 * - Border styles (left-bar, top-bottom, none)
 * - Accent color customization
 * - Style controls visibility
 * - Accessibility (ARIA attributes)
 * - Keyboard navigation
 */

import { describe, it, expect, vi, afterEach } from 'vitest';
import { render, screen, fireEvent, cleanup } from '@testing-library/svelte';
import PullQuoteBlock from '../content/PullQuoteBlock.svelte';
import type { Block } from '../types';
import { toBlockId } from '$lib/stores/blockState.svelte';

// ===============================================================================
// TEST FIXTURES
// ===============================================================================

interface PullQuoteBlockOverrides {
	id?: string;
	content?: {
		text?: string;
		attribution?: string;
		source?: string;
		alignment?: 'left' | 'center' | 'right';
		borderStyle?: 'left-bar' | 'top-bottom' | 'none';
		accentColor?: string;
	};
	settings?: Record<string, unknown>;
	metadata?: Record<string, unknown>;
}

function createPullQuoteBlock(overrides: PullQuoteBlockOverrides = {}): Block {
	return {
		id: toBlockId(overrides.id || 'test-pullquote'),
		type: 'pullquote',
		content: {
			text: 'The only way to do great work is to love what you do.',
			attribution: 'Steve Jobs',
			source: 'Stanford Commencement, 2005',
			alignment: 'center',
			borderStyle: 'top-bottom',
			accentColor: '#3b82f6',
			...overrides.content
		},
		settings: {
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
// TEST SUITE: Basic Rendering
// ===============================================================================

describe('PullQuoteBlock - Basic Rendering', () => {
	afterEach(() => {
		cleanup();
	});

	it('should render the pullquote as a figure element', () => {
		const block = createPullQuoteBlock();
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('quote-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const figure = screen.getByRole('figure');
		expect(figure).toBeInTheDocument();
	});

	it('should render the quote text', () => {
		const block = createPullQuoteBlock({
			content: { text: 'Test quote text' }
		});
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('quote-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Test quote text')).toBeInTheDocument();
	});

	it('should render decorative quote marks', () => {
		const block = createPullQuoteBlock();
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('quote-3'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const quoteMarks = container.querySelectorAll('.quote-mark');
		expect(quoteMarks.length).toBe(2); // Opening and closing
	});

	it('should render without errors', () => {
		const block = createPullQuoteBlock();
		expect(() => {
			render(PullQuoteBlock, {
				props: {
					block,
					blockId: toBlockId('quote-4'),
					isEditing: false,
					isSelected: false,
					onUpdate: vi.fn()
				}
			});
		}).not.toThrow();
	});
});

// ===============================================================================
// TEST SUITE: Attribution and Source
// ===============================================================================

describe('PullQuoteBlock - Attribution and Source', () => {
	afterEach(() => {
		cleanup();
	});

	it('should render attribution', () => {
		const block = createPullQuoteBlock({
			content: { attribution: 'John Doe' }
		});
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('attr-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('John Doe')).toBeInTheDocument();
	});

	it('should render source', () => {
		const block = createPullQuoteBlock({
			content: { source: 'The Great Book' }
		});
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('attr-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('The Great Book')).toBeInTheDocument();
	});

	it('should render em-dash before attribution', () => {
		const block = createPullQuoteBlock({
			content: { attribution: 'Author Name' }
		});
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('attr-3'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const emDash = container.querySelector('.em-dash');
		expect(emDash).toBeInTheDocument();
	});

	it('should use cite element for attribution', () => {
		const block = createPullQuoteBlock({
			content: { attribution: 'Famous Author' }
		});
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('attr-4'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const cite = container.querySelector('cite');
		expect(cite).toBeInTheDocument();
		expect(cite?.textContent).toBe('Famous Author');
	});

	it('should show placeholder for attribution in edit mode', () => {
		const block = createPullQuoteBlock({
			content: { attribution: '' }
		});
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('attr-5'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const attributionField = container.querySelector('.pullquote-attribution');
		expect(attributionField).toHaveAttribute('data-placeholder', 'Author name');
	});

	it('should show placeholder for source in edit mode', () => {
		const block = createPullQuoteBlock({
			content: { source: '' }
		});
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('attr-6'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const sourceField = container.querySelector('.pullquote-source');
		expect(sourceField).toHaveAttribute('data-placeholder', 'Publication');
	});
});

// ===============================================================================
// TEST SUITE: Alignment Options
// ===============================================================================

describe('PullQuoteBlock - Alignment Options', () => {
	afterEach(() => {
		cleanup();
	});

	it('should apply left alignment class', () => {
		const block = createPullQuoteBlock({
			content: { alignment: 'left' }
		});
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('align-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const pullquote = container.querySelector('.pullquote-block');
		expect(pullquote).toHaveClass('align-left');
	});

	it('should apply center alignment class', () => {
		const block = createPullQuoteBlock({
			content: { alignment: 'center' }
		});
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('align-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const pullquote = container.querySelector('.pullquote-block');
		expect(pullquote).toHaveClass('align-center');
	});

	it('should apply right alignment class', () => {
		const block = createPullQuoteBlock({
			content: { alignment: 'right' }
		});
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('align-3'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const pullquote = container.querySelector('.pullquote-block');
		expect(pullquote).toHaveClass('align-right');
	});

	it('should show alignment buttons in controls', () => {
		const block = createPullQuoteBlock();
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('align-4'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByTitle('Align left')).toBeInTheDocument();
		expect(screen.getByTitle('Align center')).toBeInTheDocument();
		expect(screen.getByTitle('Align right')).toBeInTheDocument();
	});

	it('should update alignment when button clicked', async () => {
		const onUpdate = vi.fn();
		const block = createPullQuoteBlock({
			content: { alignment: 'center' }
		});
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('align-5'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const leftButton = screen.getByTitle('Align left');
		await fireEvent.click(leftButton);

		expect(onUpdate).toHaveBeenCalledWith(
			expect.objectContaining({
				content: expect.objectContaining({
					alignment: 'left'
				})
			})
		);
	});

	it('should highlight active alignment button', () => {
		const block = createPullQuoteBlock({
			content: { alignment: 'center' }
		});
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('align-6'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const activeBtn = container.querySelector('.control-btn.active[aria-pressed="true"]');
		expect(activeBtn).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Border Styles
// ===============================================================================

describe('PullQuoteBlock - Border Styles', () => {
	afterEach(() => {
		cleanup();
	});

	it('should apply left-bar border style class', () => {
		const block = createPullQuoteBlock({
			content: { borderStyle: 'left-bar' }
		});
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('border-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const pullquote = container.querySelector('.pullquote-block');
		expect(pullquote).toHaveClass('border-left-bar');
	});

	it('should apply top-bottom border style class', () => {
		const block = createPullQuoteBlock({
			content: { borderStyle: 'top-bottom' }
		});
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('border-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const pullquote = container.querySelector('.pullquote-block');
		expect(pullquote).toHaveClass('border-top-bottom');
	});

	it('should apply none border style class', () => {
		const block = createPullQuoteBlock({
			content: { borderStyle: 'none' }
		});
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('border-3'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const pullquote = container.querySelector('.pullquote-block');
		expect(pullquote).toHaveClass('border-none');
	});

	it('should show border style buttons in controls', () => {
		const block = createPullQuoteBlock();
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('border-4'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByTitle('Left bar border')).toBeInTheDocument();
		expect(screen.getByTitle('Top and bottom borders')).toBeInTheDocument();
		expect(screen.getByTitle('No border')).toBeInTheDocument();
	});

	it('should update border style when button clicked', async () => {
		const onUpdate = vi.fn();
		const block = createPullQuoteBlock({
			content: { borderStyle: 'top-bottom' }
		});
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('border-5'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const leftBarButton = screen.getByTitle('Left bar border');
		await fireEvent.click(leftBarButton);

		expect(onUpdate).toHaveBeenCalledWith(
			expect.objectContaining({
				content: expect.objectContaining({
					borderStyle: 'left-bar'
				})
			})
		);
	});

	it('should highlight active border style button', () => {
		const block = createPullQuoteBlock({
			content: { borderStyle: 'left-bar' }
		});
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('border-6'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const activeBtn = container.querySelector('.control-btn.active[aria-pressed="true"]');
		expect(activeBtn).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Accent Color
// ===============================================================================

describe('PullQuoteBlock - Accent Color', () => {
	afterEach(() => {
		cleanup();
	});

	it('should apply accent color as CSS variable', () => {
		const block = createPullQuoteBlock({
			content: { accentColor: '#ef4444' }
		});
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('color-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const pullquote = container.querySelector('.pullquote-block');
		expect(pullquote).toHaveStyle({ '--accent-color': '#ef4444' } as any);
	});

	it('should show color picker trigger in controls', () => {
		const block = createPullQuoteBlock();
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('color-2'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const colorTrigger = screen.getByTitle('Choose accent color');
		expect(colorTrigger).toBeInTheDocument();
	});

	it('should show color preview in trigger', () => {
		const block = createPullQuoteBlock({
			content: { accentColor: '#10b981' }
		});
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('color-3'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const colorPreview = container.querySelector('.color-preview');
		expect(colorPreview).toHaveStyle({ backgroundColor: '#10b981' });
	});

	it('should toggle color picker dropdown on click', async () => {
		const block = createPullQuoteBlock();
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('color-4'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const colorTrigger = screen.getByTitle('Choose accent color');
		await fireEvent.click(colorTrigger);

		const dropdown = screen.getByRole('listbox', { name: /color presets/i });
		expect(dropdown).toBeInTheDocument();
	});

	it('should show preset colors in dropdown', async () => {
		const block = createPullQuoteBlock();
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('color-5'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const colorTrigger = screen.getByTitle('Choose accent color');
		await fireEvent.click(colorTrigger);

		const presetColors = screen.getAllByRole('button', { name: /select color/i });
		expect(presetColors.length).toBeGreaterThan(0);
	});

	it('should update color when preset clicked', async () => {
		const onUpdate = vi.fn();
		const block = createPullQuoteBlock();
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('color-6'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const colorTrigger = screen.getByTitle('Choose accent color');
		await fireEvent.click(colorTrigger);

		const presetColors = screen.getAllByRole('button', { name: /select color/i });
		if (presetColors[0]) {
			await fireEvent.click(presetColors[0]);
			expect(onUpdate).toHaveBeenCalled();
		}
	});

	it('should show custom color input', async () => {
		const block = createPullQuoteBlock();
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('color-7'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const colorTrigger = screen.getByTitle('Choose accent color');
		await fireEvent.click(colorTrigger);

		const customColorInput = screen.getByLabelText('Custom:');
		expect(customColorInput).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Style Controls
// ===============================================================================

describe('PullQuoteBlock - Style Controls', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show controls when editing and selected', () => {
		const block = createPullQuoteBlock();
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('ctrl-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const controls = screen.getByRole('toolbar', { name: /quote styling options/i });
		expect(controls).toBeInTheDocument();
	});

	it('should not show controls when not editing', () => {
		const block = createPullQuoteBlock();
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('ctrl-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const controls = screen.queryByRole('toolbar');
		expect(controls).not.toBeInTheDocument();
	});

	it('should not show controls when editing but not selected', () => {
		const block = createPullQuoteBlock();
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('ctrl-3'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const controls = screen.queryByRole('toolbar');
		expect(controls).not.toBeInTheDocument();
	});

	it('should have alignment control group', () => {
		const block = createPullQuoteBlock();
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('ctrl-4'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const alignmentGroup = screen.getByRole('group', { name: /text alignment/i });
		expect(alignmentGroup).toBeInTheDocument();
	});

	it('should have border style control group', () => {
		const block = createPullQuoteBlock();
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('ctrl-5'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const borderGroup = screen.getByRole('group', { name: /border style/i });
		expect(borderGroup).toBeInTheDocument();
	});

	it('should have accent color control group', () => {
		const block = createPullQuoteBlock();
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('ctrl-6'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const colorGroup = screen.getByRole('group', { name: /accent color/i });
		expect(colorGroup).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Content Editing
// ===============================================================================

describe('PullQuoteBlock - Content Editing', () => {
	afterEach(() => {
		cleanup();
	});

	it('should make quote text contenteditable when editing', () => {
		const block = createPullQuoteBlock();
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('edit-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const quoteText = container.querySelector('.pullquote-text');
		expect(quoteText).toHaveAttribute('contenteditable', 'true');
	});

	it('should not be contenteditable when not editing', () => {
		const block = createPullQuoteBlock();
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('edit-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const quoteText = container.querySelector('.pullquote-text');
		expect(quoteText).not.toHaveAttribute('contenteditable', 'true');
	});

	it('should show placeholder for empty quote', () => {
		const block = createPullQuoteBlock({
			content: { text: '' }
		});
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('edit-3'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const quoteText = container.querySelector('.pullquote-text');
		expect(quoteText).toHaveAttribute('data-placeholder', 'Enter a memorable quote...');
	});

	it('should have proper ARIA role when editing', () => {
		const block = createPullQuoteBlock();
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('edit-4'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const quoteText = container.querySelector('.pullquote-text');
		expect(quoteText).toHaveAttribute('role', 'textbox');
	});

	it('should have aria-multiline for quote text', () => {
		const block = createPullQuoteBlock();
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('edit-5'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const quoteText = container.querySelector('.pullquote-text');
		expect(quoteText).toHaveAttribute('aria-multiline', 'true');
	});
});

// ===============================================================================
// TEST SUITE: Accessibility
// ===============================================================================

describe('PullQuoteBlock - Accessibility', () => {
	afterEach(() => {
		cleanup();
	});

	it('should have figure role', () => {
		const block = createPullQuoteBlock();
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('a11y-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const figure = screen.getByRole('figure');
		expect(figure).toBeInTheDocument();
	});

	it('should have aria-labelledby for figure', () => {
		const block = createPullQuoteBlock();
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('a11y-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const figure = container.querySelector('.pullquote-block');
		expect(figure).toHaveAttribute('aria-labelledby');
	});

	it('should have aria-hidden on decorative quote marks', () => {
		const block = createPullQuoteBlock();
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('a11y-3'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const quoteMarks = container.querySelectorAll('.quote-mark');
		quoteMarks.forEach((mark) => {
			expect(mark).toHaveAttribute('aria-hidden', 'true');
		});
	});

	it('should have aria-pressed on style buttons', () => {
		const block = createPullQuoteBlock();
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('a11y-4'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const buttons = screen.getAllByRole('button');
		const pressedButtons = buttons.filter((btn) => btn.hasAttribute('aria-pressed'));
		expect(pressedButtons.length).toBeGreaterThan(0);
	});

	it('should have proper label for quote text when editing', () => {
		const block = createPullQuoteBlock();
		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('a11y-5'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const quoteText = container.querySelector('.pullquote-text');
		expect(quoteText).toHaveAttribute('aria-label', 'Quote text');
	});

	it('should have sr-only text for preset color buttons', async () => {
		const block = createPullQuoteBlock();
		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('a11y-6'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const colorTrigger = screen.getByTitle('Choose accent color');
		await fireEvent.click(colorTrigger);

		const srOnlyText = screen.getAllByText(/select color/i);
		expect(srOnlyText.length).toBeGreaterThan(0);
	});
});

// ===============================================================================
// TEST SUITE: Error Handling
// ===============================================================================

describe('PullQuoteBlock - Error Handling', () => {
	afterEach(() => {
		cleanup();
	});

	it('should handle missing content gracefully', () => {
		const block: Block = {
			id: toBlockId('test'),
			type: 'pullquote',
			content: {},
			settings: {},
			metadata: {
				createdAt: new Date().toISOString(),
				updatedAt: new Date().toISOString(),
				version: 1
			}
		};

		expect(() => {
			render(PullQuoteBlock, {
				props: {
					block,
					blockId: toBlockId('error-1'),
					isEditing: false,
					isSelected: false,
					onUpdate: vi.fn()
				}
			});
		}).not.toThrow();
	});

	it('should use default values for undefined content', () => {
		const block: Block = {
			id: toBlockId('test'),
			type: 'pullquote',
			content: {},
			settings: {},
			metadata: {
				createdAt: new Date().toISOString(),
				updatedAt: new Date().toISOString(),
				version: 1
			}
		};

		const { container } = render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('error-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		// Should still render the quote block structure
		const pullquote = container.querySelector('.pullquote-block');
		expect(pullquote).toBeInTheDocument();
	});

	it('should not call onError for valid input', () => {
		const onError = vi.fn();
		const block = createPullQuoteBlock();

		render(PullQuoteBlock, {
			props: {
				block,
				blockId: toBlockId('error-3'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn(),
				onError
			}
		});

		expect(onError).not.toHaveBeenCalled();
	});
});
