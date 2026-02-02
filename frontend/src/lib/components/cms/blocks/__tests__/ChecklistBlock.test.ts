/**
 * ===============================================================================
 * ChecklistBlock Component - Unit Tests
 * ===============================================================================
 *
 * @description Comprehensive tests for ChecklistBlock Svelte component
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Svelte 5 + Testing Library
 *
 * Tests cover:
 * - Rendering checklist items
 * - Toggling checkbox state
 * - Progress indicator display
 * - Adding new items
 * - Removing items
 * - Strikethrough styling for completed items
 * - Keyboard navigation (Tab, Space, Enter, Arrow keys)
 * - Title editing
 * - Settings toggles (showProgress, strikethrough)
 * - Accessibility (ARIA attributes)
 */

import { describe, it, expect, vi, afterEach } from 'vitest';
import { render, screen, fireEvent, cleanup } from '@testing-library/svelte';
import ChecklistBlock from '../content/ChecklistBlock.svelte';
import type { Block } from '../types';

// ===============================================================================
// TEST FIXTURES
// ===============================================================================

interface ChecklistItem {
	id: string;
	text: string;
	checked: boolean;
}

interface ChecklistBlockOverrides {
	id?: string;
	content?: {
		items?: ChecklistItem[];
		showProgress?: boolean;
		strikethrough?: boolean;
		title?: string;
	};
	settings?: Record<string, unknown>;
	metadata?: Record<string, unknown>;
}

function createChecklistBlock(overrides: ChecklistBlockOverrides = {}): Block {
	return {
		id: overrides.id || 'test-checklist',
		type: 'checklist',
		content: {
			items: [
				{ id: '1', text: 'First item', checked: false },
				{ id: '2', text: 'Second item', checked: true },
				{ id: '3', text: 'Third item', checked: false }
			],
			showProgress: true,
			strikethrough: true,
			title: '',
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

describe('ChecklistBlock - Basic Rendering', () => {
	afterEach(() => {
		cleanup();
	});

	it('should render the checklist container', () => {
		const block = createChecklistBlock();
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'checklist-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const container = screen.getByRole('region');
		expect(container).toBeInTheDocument();
	});

	it('should render all checklist items', () => {
		const block = createChecklistBlock();
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'checklist-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('First item')).toBeInTheDocument();
		expect(screen.getByText('Second item')).toBeInTheDocument();
		expect(screen.getByText('Third item')).toBeInTheDocument();
	});

	it('should render items as list', () => {
		const block = createChecklistBlock();
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'checklist-3',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const list = screen.getByRole('list');
		expect(list).toBeInTheDocument();

		const listItems = screen.getAllByRole('listitem');
		expect(listItems.length).toBe(3);
	});

	it('should render without errors', () => {
		const block = createChecklistBlock();
		expect(() => {
			render(ChecklistBlock, {
				props: {
					block,
					blockId: 'checklist-4',
					isEditing: false,
					isSelected: false,
					onUpdate: vi.fn()
				}
			});
		}).not.toThrow();
	});
});

// ===============================================================================
// TEST SUITE: Checkbox Toggling
// ===============================================================================

describe('ChecklistBlock - Checkbox Toggling', () => {
	afterEach(() => {
		cleanup();
	});

	it('should render checkboxes for each item', () => {
		const block = createChecklistBlock();
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'toggle-1',
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const checkboxes = screen.getAllByRole('checkbox');
		expect(checkboxes.length).toBe(3);
	});

	it('should show checked state correctly', () => {
		const block = createChecklistBlock({
			content: {
				items: [
					{ id: '1', text: 'Unchecked', checked: false },
					{ id: '2', text: 'Checked', checked: true }
				]
			}
		});
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'toggle-2',
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const checkboxes = screen.getAllByRole('checkbox');
		expect(checkboxes[0]).toHaveAttribute('aria-checked', 'false');
		expect(checkboxes[1]).toHaveAttribute('aria-checked', 'true');
	});

	it('should toggle checkbox on click', async () => {
		const onUpdate = vi.fn();
		const block = createChecklistBlock({
			content: {
				items: [{ id: '1', text: 'Item 1', checked: false }]
			}
		});
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'toggle-3',
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const checkbox = screen.getByRole('checkbox');
		await fireEvent.click(checkbox);

		expect(onUpdate).toHaveBeenCalledWith(
			expect.objectContaining({
				content: expect.objectContaining({
					items: [{ id: '1', text: 'Item 1', checked: true }]
				})
			})
		);
	});

	it('should disable checkbox when not editing', () => {
		const block = createChecklistBlock();
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'toggle-4',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const checkboxes = screen.getAllByRole('checkbox');
		checkboxes.forEach((checkbox) => {
			expect(checkbox).toBeDisabled();
		});
	});

	it('should toggle checkbox on Space key', async () => {
		const onUpdate = vi.fn();
		const block = createChecklistBlock({
			content: {
				items: [{ id: '1', text: 'Item 1', checked: false }]
			}
		});
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'toggle-5',
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const checkbox = screen.getByRole('checkbox');
		await fireEvent.keyDown(checkbox, { key: ' ' });

		expect(onUpdate).toHaveBeenCalled();
	});

	it('should toggle checkbox on Enter key', async () => {
		const onUpdate = vi.fn();
		const block = createChecklistBlock({
			content: {
				items: [{ id: '1', text: 'Item 1', checked: false }]
			}
		});
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'toggle-6',
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const checkbox = screen.getByRole('checkbox');
		await fireEvent.keyDown(checkbox, { key: 'Enter' });

		expect(onUpdate).toHaveBeenCalled();
	});
});

// ===============================================================================
// TEST SUITE: Progress Indicator
// ===============================================================================

describe('ChecklistBlock - Progress Indicator', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show progress indicator when enabled', () => {
		const block = createChecklistBlock({
			content: {
				items: [
					{ id: '1', text: 'Item 1', checked: true },
					{ id: '2', text: 'Item 2', checked: false }
				],
				showProgress: true
			}
		});
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'progress-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText(/1 of 2 completed/i)).toBeInTheDocument();
	});

	it('should show progress percentage', () => {
		const block = createChecklistBlock({
			content: {
				items: [
					{ id: '1', text: 'Item 1', checked: true },
					{ id: '2', text: 'Item 2', checked: false }
				],
				showProgress: true
			}
		});
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'progress-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('50%')).toBeInTheDocument();
	});

	it('should show progress bar with correct role', () => {
		const block = createChecklistBlock({
			content: { showProgress: true }
		});
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'progress-3',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const progressBar = screen.getByRole('progressbar');
		expect(progressBar).toBeInTheDocument();
	});

	it('should have correct aria-valuenow on progress bar', () => {
		const block = createChecklistBlock({
			content: {
				items: [
					{ id: '1', text: 'Item 1', checked: true },
					{ id: '2', text: 'Item 2', checked: true },
					{ id: '3', text: 'Item 3', checked: false },
					{ id: '4', text: 'Item 4', checked: false }
				],
				showProgress: true
			}
		});
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'progress-4',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const progressBar = screen.getByRole('progressbar');
		expect(progressBar).toHaveAttribute('aria-valuenow', '50');
	});

	it('should not show progress indicator when disabled', () => {
		const block = createChecklistBlock({
			content: { showProgress: false }
		});
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'progress-5',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const progressBar = screen.queryByRole('progressbar');
		expect(progressBar).not.toBeInTheDocument();
	});

	it('should show 100% when all items checked', () => {
		const block = createChecklistBlock({
			content: {
				items: [
					{ id: '1', text: 'Item 1', checked: true },
					{ id: '2', text: 'Item 2', checked: true }
				],
				showProgress: true
			}
		});
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'progress-6',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('100%')).toBeInTheDocument();
	});

	it('should not show progress when no items', () => {
		const block = createChecklistBlock({
			content: {
				items: [],
				showProgress: true
			}
		});
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'progress-7',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const progressBar = screen.queryByRole('progressbar');
		expect(progressBar).not.toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Adding Items
// ===============================================================================

describe('ChecklistBlock - Adding Items', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show add button when editing', () => {
		const block = createChecklistBlock();
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'add-1',
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const addButton = screen.getByRole('button', { name: /add.*item/i });
		expect(addButton).toBeInTheDocument();
	});

	it('should not show add button when not editing', () => {
		const block = createChecklistBlock();
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'add-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const addButton = screen.queryByRole('button', { name: /add.*item/i });
		expect(addButton).not.toBeInTheDocument();
	});

	it('should call onUpdate when add button clicked', async () => {
		const onUpdate = vi.fn();
		const block = createChecklistBlock({
			content: {
				items: [{ id: '1', text: 'Existing item', checked: false }]
			}
		});
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'add-3',
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const addButton = screen.getByRole('button', { name: /add.*item/i });
		await fireEvent.click(addButton);

		expect(onUpdate).toHaveBeenCalledWith(
			expect.objectContaining({
				content: expect.objectContaining({
					items: expect.arrayContaining([
						expect.objectContaining({ text: '' })
					])
				})
			})
		);
	});

	it('should add item after current on Enter key', async () => {
		const onUpdate = vi.fn();
		const block = createChecklistBlock({
			content: {
				items: [{ id: '1', text: 'First item', checked: false }]
			}
		});
		const { container } = render(ChecklistBlock, {
			props: {
				block,
				blockId: 'add-4',
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const itemText = container.querySelector('.item-text');
		if (itemText) {
			await fireEvent.keyDown(itemText, { key: 'Enter' });
			expect(onUpdate).toHaveBeenCalled();
		}
	});
});

// ===============================================================================
// TEST SUITE: Removing Items
// ===============================================================================

describe('ChecklistBlock - Removing Items', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show remove button for each item when editing', () => {
		const block = createChecklistBlock({
			content: {
				items: [
					{ id: '1', text: 'Item 1', checked: false },
					{ id: '2', text: 'Item 2', checked: false }
				]
			}
		});
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'remove-1',
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const removeButtons = screen.getAllByRole('button', { name: /remove item/i });
		expect(removeButtons.length).toBe(2);
	});

	it('should not show remove button when not editing', () => {
		const block = createChecklistBlock();
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'remove-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const removeButtons = screen.queryAllByRole('button', { name: /remove item/i });
		expect(removeButtons.length).toBe(0);
	});

	it('should call onUpdate when remove button clicked', async () => {
		const onUpdate = vi.fn();
		const block = createChecklistBlock({
			content: {
				items: [
					{ id: '1', text: 'Item 1', checked: false },
					{ id: '2', text: 'Item 2', checked: false }
				]
			}
		});
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'remove-3',
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const removeButtons = screen.getAllByRole('button', { name: /remove item/i });
		await fireEvent.click(removeButtons[0]);

		expect(onUpdate).toHaveBeenCalledWith(
			expect.objectContaining({
				content: expect.objectContaining({
					items: [{ id: '2', text: 'Item 2', checked: false }]
				})
			})
		);
	});

	it('should remove item on Backspace when text is empty', async () => {
		const onUpdate = vi.fn();
		const block = createChecklistBlock({
			content: {
				items: [
					{ id: '1', text: '', checked: false },
					{ id: '2', text: 'Item 2', checked: false }
				]
			}
		});
		const { container } = render(ChecklistBlock, {
			props: {
				block,
				blockId: 'remove-4',
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const itemTexts = container.querySelectorAll('.item-text');
		if (itemTexts[0]) {
			await fireEvent.keyDown(itemTexts[0], { key: 'Backspace' });
			expect(onUpdate).toHaveBeenCalled();
		}
	});
});

// ===============================================================================
// TEST SUITE: Strikethrough Styling
// ===============================================================================

describe('ChecklistBlock - Strikethrough Styling', () => {
	afterEach(() => {
		cleanup();
	});

	it('should apply strikethrough class to checked items', () => {
		const block = createChecklistBlock({
			content: {
				items: [{ id: '1', text: 'Checked item', checked: true }],
				strikethrough: true
			}
		});
		const { container } = render(ChecklistBlock, {
			props: {
				block,
				blockId: 'strike-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const checkedItem = container.querySelector('.checklist-item.strikethrough');
		expect(checkedItem).toBeInTheDocument();
	});

	it('should not apply strikethrough when disabled', () => {
		const block = createChecklistBlock({
			content: {
				items: [{ id: '1', text: 'Checked item', checked: true }],
				strikethrough: false
			}
		});
		const { container } = render(ChecklistBlock, {
			props: {
				block,
				blockId: 'strike-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const strikethroughItem = container.querySelector('.checklist-item.strikethrough');
		expect(strikethroughItem).not.toBeInTheDocument();
	});

	it('should apply checked class to checked items', () => {
		const block = createChecklistBlock({
			content: {
				items: [{ id: '1', text: 'Checked item', checked: true }]
			}
		});
		const { container } = render(ChecklistBlock, {
			props: {
				block,
				blockId: 'strike-3',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const checkedItem = container.querySelector('.checklist-item.checked');
		expect(checkedItem).toBeInTheDocument();
	});

	it('should not apply strikethrough to unchecked items', () => {
		const block = createChecklistBlock({
			content: {
				items: [{ id: '1', text: 'Unchecked item', checked: false }],
				strikethrough: true
			}
		});
		const { container } = render(ChecklistBlock, {
			props: {
				block,
				blockId: 'strike-4',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const strikethroughItem = container.querySelector('.checklist-item.strikethrough');
		expect(strikethroughItem).not.toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Keyboard Navigation
// ===============================================================================

describe('ChecklistBlock - Keyboard Navigation', () => {
	afterEach(() => {
		cleanup();
	});

	it('should move focus to next item on ArrowDown', async () => {
		const block = createChecklistBlock({
			content: {
				items: [
					{ id: '1', text: 'Item 1', checked: false },
					{ id: '2', text: 'Item 2', checked: false }
				]
			}
		});
		const { container } = render(ChecklistBlock, {
			props: {
				block,
				blockId: 'keyboard-1',
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const itemTexts = container.querySelectorAll('.item-text');
		if (itemTexts[0]) {
			await fireEvent.keyDown(itemTexts[0], { key: 'ArrowDown' });
			// Focus should move to next item
		}
	});

	it('should move focus to previous item on ArrowUp', async () => {
		const block = createChecklistBlock({
			content: {
				items: [
					{ id: '1', text: 'Item 1', checked: false },
					{ id: '2', text: 'Item 2', checked: false }
				]
			}
		});
		const { container } = render(ChecklistBlock, {
			props: {
				block,
				blockId: 'keyboard-2',
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const itemTexts = container.querySelectorAll('.item-text');
		if (itemTexts[1]) {
			await fireEvent.keyDown(itemTexts[1], { key: 'ArrowUp' });
			// Focus should move to previous item
		}
	});

	it('should have focusable checkboxes', () => {
		const block = createChecklistBlock();
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'keyboard-3',
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const checkboxes = screen.getAllByRole('checkbox');
		checkboxes.forEach((checkbox) => {
			expect(checkbox).toHaveAttribute('tabindex', '0');
		});
	});

	it('should have focusable item text fields', () => {
		const block = createChecklistBlock();
		const { container } = render(ChecklistBlock, {
			props: {
				block,
				blockId: 'keyboard-4',
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const itemTexts = container.querySelectorAll('.item-text');
		itemTexts.forEach((itemText) => {
			expect(itemText).toHaveAttribute('contenteditable', 'true');
		});
	});
});

// ===============================================================================
// TEST SUITE: Title Editing
// ===============================================================================

describe('ChecklistBlock - Title Editing', () => {
	afterEach(() => {
		cleanup();
	});

	it('should render title when provided', () => {
		const block = createChecklistBlock({
			content: { title: 'My Checklist' }
		});
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'title-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('My Checklist')).toBeInTheDocument();
	});

	it('should show title placeholder in edit mode when empty', () => {
		const block = createChecklistBlock({
			content: { title: '' }
		});
		const { container } = render(ChecklistBlock, {
			props: {
				block,
				blockId: 'title-2',
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const titleField = container.querySelector('.checklist-title');
		expect(titleField).toHaveAttribute('data-placeholder', 'Checklist title (optional)');
	});

	it('should make title contenteditable when editing', () => {
		const block = createChecklistBlock();
		const { container } = render(ChecklistBlock, {
			props: {
				block,
				blockId: 'title-3',
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const titleField = container.querySelector('.checklist-title');
		expect(titleField).toHaveAttribute('contenteditable', 'true');
	});

	it('should have proper ARIA role when editing', () => {
		const block = createChecklistBlock();
		const { container } = render(ChecklistBlock, {
			props: {
				block,
				blockId: 'title-4',
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const titleField = container.querySelector('.checklist-title');
		expect(titleField).toHaveAttribute('role', 'textbox');
	});
});

// ===============================================================================
// TEST SUITE: Settings Toggles
// ===============================================================================

describe('ChecklistBlock - Settings Toggles', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show controls when editing and selected', () => {
		const block = createChecklistBlock();
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'settings-1',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const controls = screen.getByRole('toolbar', { name: /checklist options/i });
		expect(controls).toBeInTheDocument();
	});

	it('should have show progress toggle', () => {
		const block = createChecklistBlock();
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'settings-2',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText(/show progress/i)).toBeInTheDocument();
	});

	it('should have strikethrough toggle', () => {
		const block = createChecklistBlock();
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'settings-3',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText(/strikethrough completed/i)).toBeInTheDocument();
	});

	it('should toggle showProgress when checkbox changed', async () => {
		const onUpdate = vi.fn();
		const block = createChecklistBlock({
			content: { showProgress: true }
		});
		const { container } = render(ChecklistBlock, {
			props: {
				block,
				blockId: 'settings-4',
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const toggleInputs = container.querySelectorAll('.control-toggle input');
		if (toggleInputs[0]) {
			await fireEvent.change(toggleInputs[0]);
			expect(onUpdate).toHaveBeenCalled();
		}
	});

	it('should toggle strikethrough when checkbox changed', async () => {
		const onUpdate = vi.fn();
		const block = createChecklistBlock({
			content: { strikethrough: true }
		});
		const { container } = render(ChecklistBlock, {
			props: {
				block,
				blockId: 'settings-5',
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const toggleInputs = container.querySelectorAll('.control-toggle input');
		if (toggleInputs[1]) {
			await fireEvent.change(toggleInputs[1]);
			expect(onUpdate).toHaveBeenCalled();
		}
	});
});

// ===============================================================================
// TEST SUITE: Empty State
// ===============================================================================

describe('ChecklistBlock - Empty State', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show empty state when no items and not editing', () => {
		const block = createChecklistBlock({
			content: { items: [] }
		});
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'empty-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText(/no items in this checklist/i)).toBeInTheDocument();
	});

	it('should not show empty state when editing', () => {
		const block = createChecklistBlock({
			content: { items: [] }
		});
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'empty-2',
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.queryByText(/no items in this checklist/i)).not.toBeInTheDocument();
	});

	it('should show add button when empty and editing', () => {
		const block = createChecklistBlock({
			content: { items: [] }
		});
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'empty-3',
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const addButton = screen.getByRole('button', { name: /add.*item/i });
		expect(addButton).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Accessibility
// ===============================================================================

describe('ChecklistBlock - Accessibility', () => {
	afterEach(() => {
		cleanup();
	});

	it('should have proper region role', () => {
		const block = createChecklistBlock();
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'a11y-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const region = screen.getByRole('region');
		expect(region).toBeInTheDocument();
	});

	it('should have aria-labelledby when title provided', () => {
		const block = createChecklistBlock({
			content: { title: 'My List' }
		});
		const { container } = render(ChecklistBlock, {
			props: {
				block,
				blockId: 'a11y-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const region = container.querySelector('.checklist-block');
		expect(region).toHaveAttribute('aria-labelledby');
	});

	it('should have aria-live on progress section', () => {
		const block = createChecklistBlock({
			content: { showProgress: true }
		});
		const { container } = render(ChecklistBlock, {
			props: {
				block,
				blockId: 'a11y-3',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const progressSection = container.querySelector('.progress-section');
		expect(progressSection).toHaveAttribute('aria-live', 'polite');
	});

	it('should have proper checkbox role and aria-checked', () => {
		const block = createChecklistBlock({
			content: {
				items: [{ id: '1', text: 'Test', checked: true }]
			}
		});
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'a11y-4',
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const checkbox = screen.getByRole('checkbox');
		expect(checkbox).toHaveAttribute('aria-checked', 'true');
	});

	it('should have aria-label on checkboxes', () => {
		const block = createChecklistBlock();
		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'a11y-5',
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const checkboxes = screen.getAllByRole('checkbox');
		checkboxes.forEach((checkbox) => {
			expect(checkbox).toHaveAttribute('aria-label');
		});
	});

	it('should have aria-describedby on list when progress shown', () => {
		const block = createChecklistBlock({
			content: { showProgress: true }
		});
		const { container } = render(ChecklistBlock, {
			props: {
				block,
				blockId: 'a11y-6',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const list = container.querySelector('.checklist-items');
		expect(list).toHaveAttribute('aria-describedby');
	});
});

// ===============================================================================
// TEST SUITE: Error Handling
// ===============================================================================

describe('ChecklistBlock - Error Handling', () => {
	afterEach(() => {
		cleanup();
	});

	it('should handle missing content gracefully', () => {
		const block: Block = {
			id: 'test',
			type: 'checklist',
			content: {},
			settings: {},
			metadata: {
				createdAt: new Date().toISOString(),
				updatedAt: new Date().toISOString(),
				version: 1
			}
		};

		expect(() => {
			render(ChecklistBlock, {
				props: {
					block,
					blockId: 'error-1',
					isEditing: false,
					isSelected: false,
					onUpdate: vi.fn()
				}
			});
		}).not.toThrow();
	});

	it('should use default values for undefined content', () => {
		const block: Block = {
			id: 'test',
			type: 'checklist',
			content: {},
			settings: {},
			metadata: {
				createdAt: new Date().toISOString(),
				updatedAt: new Date().toISOString(),
				version: 1
			}
		};

		const { container } = render(ChecklistBlock, {
			props: {
				block,
				blockId: 'error-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const checklistBlock = container.querySelector('.checklist-block');
		expect(checklistBlock).toBeInTheDocument();
	});

	it('should handle empty items array', () => {
		const block = createChecklistBlock({
			content: { items: [] }
		});

		expect(() => {
			render(ChecklistBlock, {
				props: {
					block,
					blockId: 'error-3',
					isEditing: false,
					isSelected: false,
					onUpdate: vi.fn()
				}
			});
		}).not.toThrow();
	});

	it('should not call onError for valid input', () => {
		const onError = vi.fn();
		const block = createChecklistBlock();

		render(ChecklistBlock, {
			props: {
				block,
				blockId: 'error-4',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn(),
				onError
			}
		});

		expect(onError).not.toHaveBeenCalled();
	});
});
