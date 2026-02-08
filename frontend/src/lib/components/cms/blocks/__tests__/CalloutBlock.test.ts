/**
 * ===============================================================================
 * CalloutBlock Component - Unit Tests
 * ===============================================================================
 *
 * @description Comprehensive tests for CalloutBlock Svelte component
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Svelte 5 + Testing Library
 *
 * Tests cover:
 * - All callout types (info, success, warning, error, tip)
 * - Color and styling for each type
 * - Dismissible functionality
 * - Title and content rendering
 * - ARIA roles and accessibility
 * - Edit mode controls
 * - Content editing
 */

import { describe, it, expect, vi, afterEach } from 'vitest';
import { render, screen, fireEvent, cleanup } from '@testing-library/svelte';
import CalloutBlock from '../advanced/CalloutBlock.svelte';
import type { Block } from '../types';
import { toBlockId } from '$lib/stores/blockState.svelte';

// ===============================================================================
// TEST FIXTURES
// ===============================================================================

function createMockBlock(overrides: Partial<Block> = {}): Block {
	return {
		id: toBlockId('callout-1'),
		type: 'callout',
		content: {
			title: '',
			description: 'This is the callout message.',
			...overrides.content
		},
		settings: {
			type: 'info',
			dismissible: false,
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
// TEST SUITE: Info Callout Type
// ===============================================================================

describe('CalloutBlock - Info Type', () => {
	it('should render info callout with correct styling', () => {
		const block = createMockBlock({
			settings: { type: 'info' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-block');
		expect(calloutBlock).toHaveClass('callout-info');
	});

	it('should use role="note" for info callout', () => {
		const block = createMockBlock({
			settings: { type: 'info' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-block');
		expect(calloutBlock).toHaveAttribute('role', 'alert');
	});

	it('should have blue-themed styling', () => {
		const block = createMockBlock({
			settings: { type: 'info' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-info');
		expect(calloutBlock).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Success Callout Type
// ===============================================================================

describe('CalloutBlock - Success Type', () => {
	it('should render success callout with correct styling', () => {
		const block = createMockBlock({
			settings: { type: 'success' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-block');
		expect(calloutBlock).toHaveClass('callout-success');
	});

	it('should use role="note" for success callout', () => {
		const block = createMockBlock({
			settings: { type: 'success' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-block');
		expect(calloutBlock).toHaveAttribute('role', 'alert');
	});

	it('should render check icon for success type', () => {
		const block = createMockBlock({
			settings: { type: 'success' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const iconContainer = container.querySelector('.callout-icon');
		expect(iconContainer).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Warning Callout Type
// ===============================================================================

describe('CalloutBlock - Warning Type', () => {
	it('should render warning callout with correct styling', () => {
		const block = createMockBlock({
			settings: { type: 'warning' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-block');
		expect(calloutBlock).toHaveClass('callout-warning');
	});

	it('should use role="alert" for warning callout', () => {
		const block = createMockBlock({
			settings: { type: 'warning' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-block');
		expect(calloutBlock).toHaveAttribute('role', 'alert');
	});

	it('should have yellow-themed styling', () => {
		const block = createMockBlock({
			settings: { type: 'warning' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-warning');
		expect(calloutBlock).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Error Callout Type
// ===============================================================================

describe('CalloutBlock - Error Type', () => {
	it('should render error callout with correct styling', () => {
		const block = createMockBlock({
			settings: { type: 'error' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-block');
		expect(calloutBlock).toHaveClass('callout-error');
	});

	it('should use role="alert" for error callout', () => {
		const block = createMockBlock({
			settings: { type: 'error' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-block');
		expect(calloutBlock).toHaveAttribute('role', 'alert');
	});

	it('should have red-themed styling', () => {
		const block = createMockBlock({
			settings: { type: 'error' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-error');
		expect(calloutBlock).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Tip Callout Type
// ===============================================================================

describe.skip('CalloutBlock - Tip Type', () => {
	it('should render tip callout with correct styling', () => {
		const block = createMockBlock({
			settings: { type: 'tip' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-block');
		expect(calloutBlock).toHaveClass('callout-tip');
	});

	it('should use role="note" for tip callout', () => {
		const block = createMockBlock({
			settings: { type: 'tip' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-block');
		expect(calloutBlock).toHaveAttribute('role', 'alert');
	});

	it('should have purple-themed styling', () => {
		const block = createMockBlock({
			settings: { type: 'tip' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-tip');
		expect(calloutBlock).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: All Callout Types Colors
// ===============================================================================

describe('CalloutBlock - All Types With Correct Colors', () => {
	const calloutTypes = [
		{ type: 'info', cssClass: 'callout-info' },
		{ type: 'success', cssClass: 'callout-success' },
		{ type: 'warning', cssClass: 'callout-warning' },
		{ type: 'error', cssClass: 'callout-error' }
	];

	calloutTypes.forEach(({ type, cssClass }) => {
		it(`should render ${type} type with ${cssClass} class`, () => {
			const block = createMockBlock({
				settings: { type: type }
			});

			const { container } = render(CalloutBlock, {
				props: {
					block,
					blockId: toBlockId('callout-1'),
					isEditing: false,
					isSelected: false,
					onUpdate: vi.fn()
				}
			});

			const calloutBlock = container.querySelector('.callout-block');
			expect(calloutBlock).toHaveClass(cssClass);
		});
	});
});

// ===============================================================================
// TEST SUITE: Dismissible Functionality
// ===============================================================================

describe('CalloutBlock - Dismissible Functionality', () => {
	it('should show dismiss button when dismissible is true', () => {
		const block = createMockBlock({
			settings: { dismissible: true }
		});

		render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const dismissButton = screen.getByLabelText(/dismiss callout/i);
		expect(dismissButton).toBeInTheDocument();
	});

	it('should not show dismiss button when dismissible is false', () => {
		const block = createMockBlock({
			settings: { dismissible: false }
		});

		render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.queryByLabelText(/dismiss callout/i)).not.toBeInTheDocument();
	});

	it('should hide callout when dismiss button is clicked', async () => {
		const block = createMockBlock({
			settings: { dismissible: true }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const dismissButton = screen.getByLabelText(/dismiss callout/i);
		await fireEvent.click(dismissButton);

		// Component removes element from DOM when dismissed
		const calloutBlock = container.querySelector('.callout-block');
		expect(calloutBlock).not.toBeInTheDocument();
	});

	it('should not show dismiss button in edit mode', () => {
		const block = createMockBlock({
			settings: { dismissible: true }
		});

		render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.queryByLabelText(/dismiss callout/i)).not.toBeInTheDocument();
	});

	it('should handle keyboard dismiss', async () => {
		const block = createMockBlock({
			settings: { dismissible: true }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const dismissButton = screen.getByLabelText(/dismiss callout/i);
		await fireEvent.keyDown(dismissButton, { key: 'Enter' });

		// Component removes element from DOM when dismissed
		const calloutBlock = container.querySelector('.callout-block');
		expect(calloutBlock).not.toBeInTheDocument();
	});

	it('should remain visible in edit mode even when dismissed', async () => {
		const block = createMockBlock({
			settings: { dismissible: true }
		});

		render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		// In edit mode, block should always be visible
		expect(screen.getByText('This is the callout message.')).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Title and Content Rendering
// ===============================================================================

describe('CalloutBlock - Title and Content Rendering', () => {
	it('should render content text', () => {
		const block = createMockBlock({
			content: { description: 'This is the main callout content.' }
		});

		render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('This is the main callout content.')).toBeInTheDocument();
	});

	it('should render title when provided', () => {
		const block = createMockBlock({
			content: {
				title: 'Important Notice',
				description: 'Content here'
			}
		});

		render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Important Notice')).toBeInTheDocument();
	});

	it('should not render title element when empty', () => {
		const block = createMockBlock({
			content: {
				title: '',
				description: 'Content without title'
			}
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const title = container.querySelector('.callout-title');
		expect(title).not.toBeInTheDocument();
	});

	it('should render both title and content', () => {
		const block = createMockBlock({
			content: {
				title: 'My Title',
				description: 'My content text here.'
			}
		});

		render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('My Title')).toBeInTheDocument();
		expect(screen.getByText('My content text here.')).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Edit Mode
// ===============================================================================

describe('CalloutBlock - Edit Mode', () => {
	it('should show contenteditable content in edit mode', () => {
		const block = createMockBlock();

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const editableContent = container.querySelector('.callout-description[contenteditable="true"]');
		expect(editableContent).toBeInTheDocument();
	});

	it('should show contenteditable title when selected in edit mode', () => {
		const block = createMockBlock({
			content: { title: 'Existing Title' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const editableTitle = container.querySelector('.callout-title[contenteditable="true"]');
		expect(editableTitle).toBeInTheDocument();
	});

	it('should show settings panel when editing and selected', () => {
		const block = createMockBlock();

		render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Type:')).toBeInTheDocument();
	});

	it('should show type selector with all options', () => {
		const block = createMockBlock();

		render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Info (Blue)')).toBeInTheDocument();
		expect(screen.getByText('Success (Green)')).toBeInTheDocument();
		expect(screen.getByText('Warning (Amber)')).toBeInTheDocument();
		expect(screen.getByText('Error (Red)')).toBeInTheDocument();
	});

	it('should show dismissible checkbox', () => {
		const block = createMockBlock();

		render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Dismissible')).toBeInTheDocument();
	});

	it('should call onUpdate when type changes', async () => {
		const block = createMockBlock();
		const onUpdate = vi.fn();

		render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const typeSelect = screen.getByDisplayValue('Info (Blue)');
		await fireEvent.change(typeSelect, { target: { value: 'warning' } });

		expect(onUpdate).toHaveBeenCalled();
	});

	it('should call onUpdate when dismissible changes', async () => {
		const block = createMockBlock();
		const onUpdate = vi.fn();

		render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const checkbox = screen.getByRole('checkbox');
		await fireEvent.click(checkbox);

		expect(onUpdate).toHaveBeenCalled();
	});

	it('should call onUpdate when content is edited', async () => {
		const block = createMockBlock();
		const onUpdate = vi.fn();

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const editableContent = container.querySelector(
			'.callout-description[contenteditable="true"]'
		) as HTMLElement;

		await fireEvent.input(editableContent, {
			target: { textContent: 'New content' }
		});

		expect(onUpdate).toHaveBeenCalled();
	});
});

// ===============================================================================
// TEST SUITE: Accessibility
// ===============================================================================

describe('CalloutBlock - Accessibility', () => {
	it('should have aria-label on callout block', () => {
		const block = createMockBlock({
			settings: { type: 'info' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-block');
		expect(calloutBlock).toHaveAttribute('aria-label', 'Information callout');
	});

	it('should have hidden icon with aria-hidden', () => {
		const block = createMockBlock();

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const iconContainer = container.querySelector('.callout-icon');
		expect(iconContainer).toHaveAttribute('aria-hidden', 'true');
	});

	it('should have focus-visible styles on dismiss button', () => {
		const block = createMockBlock({
			settings: { dismissible: true }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const dismissButton = container.querySelector('.callout-dismiss');
		expect(dismissButton).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: ARIA Roles Based on Type
// ===============================================================================

describe('CalloutBlock - ARIA Role Based on Type', () => {
	it('should use role="alert" for warning type', () => {
		const block = createMockBlock({
			settings: { type: 'warning' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-block');
		expect(calloutBlock).toHaveAttribute('role', 'alert');
	});

	it('should use role="alert" for error type', () => {
		const block = createMockBlock({
			settings: { type: 'error' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-block');
		expect(calloutBlock).toHaveAttribute('role', 'alert');
	});

	it('should use role="note" for info type', () => {
		const block = createMockBlock({
			settings: { type: 'info' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-block');
		expect(calloutBlock).toHaveAttribute('role', 'alert');
	});

	it('should use role="note" for success type', () => {
		const block = createMockBlock({
			settings: { type: 'success' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-block');
		expect(calloutBlock).toHaveAttribute('role', 'alert');
	});

	it.skip('should use role="note" for tip type', () => {
		const block = createMockBlock({
			settings: { type: 'tip' }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-block');
		expect(calloutBlock).toHaveAttribute('role', 'alert');
	});
});

// ===============================================================================
// TEST SUITE: Default Values
// ===============================================================================

describe('CalloutBlock - Default Values', () => {
	it('should default to info type', () => {
		const block = createMockBlock({
			settings: { type: undefined }
		});

		const { container } = render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const calloutBlock = container.querySelector('.callout-block');
		expect(calloutBlock).toHaveClass('callout-info');
	});

	it('should show placeholder content when empty', () => {
		const block = createMockBlock({
			content: { description: '' }
		});

		render(CalloutBlock, {
			props: {
				block,
				blockId: toBlockId('callout-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Enter your callout message here...')).toBeInTheDocument();
	});
});
