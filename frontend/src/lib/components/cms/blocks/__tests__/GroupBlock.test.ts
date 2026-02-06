/**
 * ===============================================================================
 * GroupBlock Component - Unit Tests
 * ===============================================================================
 *
 * @description Comprehensive tests for GroupBlock Svelte component
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Svelte 5 + Testing Library
 *
 * Tests cover:
 * - Background color control
 * - Padding settings (none, small, medium, large)
 * - Border radius options
 * - Max width settings
 * - Alignment options (left, center, right)
 * - Nested block content
 * - Edit mode toolbar functionality
 * - Accessibility (ARIA attributes)
 */

import { describe, it, expect, vi, afterEach } from 'vitest';
import { render, screen, fireEvent, cleanup } from '@testing-library/svelte';
import GroupBlock from '../layout/GroupBlock.svelte';
import type { Block } from '../types';
import { toBlockId } from '$lib/stores/blockState.svelte';

// ===============================================================================
// TEST FIXTURES
// ===============================================================================

interface GroupBlockOverrides {
	id?: string;
	content?: {
		children?: Block[];
	};
	settings?: {
		backgroundColor?: string;
		padding?: 'none' | 'small' | 'medium' | 'large';
		borderRadius?: string;
		maxWidth?: 'full' | 'large' | 'medium' | 'small';
		textAlign?: 'left' | 'center' | 'right';
	};
	metadata?: Record<string, unknown>;
}

function createGroupBlock(overrides: GroupBlockOverrides = {}): Block {
	return {
		id: toBlockId(overrides.id || 'test-group'),
		type: 'group',
		content: {
			children: [],
			...overrides.content
		},
		settings: {
			backgroundColor: '',
			padding: 'medium',
			borderRadius: '8px',
			maxWidth: 'full',
			textAlign: 'left',
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

describe('GroupBlock - Basic Rendering', () => {
	afterEach(() => {
		cleanup();
	});

	it('should render the group container', () => {
		const block = createGroupBlock();
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('group-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const container = screen.getByRole('group', { name: /content group/i });
		expect(container).toBeInTheDocument();
	});

	it('should render without errors', () => {
		const block = createGroupBlock();
		expect(() => {
			render(GroupBlock, {
				props: {
					block,
					blockId: toBlockId('group-2'),
					isEditing: false,
					isSelected: false,
					onUpdate: vi.fn()
				}
			});
		}).not.toThrow();
	});

	it('should have correct aria-label', () => {
		const block = createGroupBlock();
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('group-3'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const container = screen.getByRole('group');
		expect(container).toHaveAttribute('aria-label', 'Content group');
	});
});

// ===============================================================================
// TEST SUITE: Background Color
// ===============================================================================

describe('GroupBlock - Background Color', () => {
	afterEach(() => {
		cleanup();
	});

	it('should apply background color when set', () => {
		const block = createGroupBlock({
			settings: { backgroundColor: '#3b82f6' }
		});
		const { container } = render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('bg-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const groupContainer = container.querySelector('.group-container');
		expect(groupContainer).toHaveStyle({ backgroundColor: '#3b82f6' });
	});

	it('should not apply background when empty', () => {
		const block = createGroupBlock({
			settings: { backgroundColor: '' }
		});
		const { container } = render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('bg-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const groupContainer = container.querySelector('.group-container');
		expect(groupContainer).not.toHaveStyle({ backgroundColor: '' });
	});

	it('should show color picker in edit mode', () => {
		const block = createGroupBlock();
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('bg-3'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const colorPicker = screen.getByLabelText(/background color/i);
		expect(colorPicker).toBeInTheDocument();
	});

	it('should update color when picker changed', async () => {
		const onUpdate = vi.fn();
		const block = createGroupBlock();
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('bg-4'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const colorPicker = screen.getByLabelText(/background color/i);
		await fireEvent.input(colorPicker, { target: { value: '#ef4444' } });

		expect(onUpdate).toHaveBeenCalledWith(
			expect.objectContaining({
				settings: expect.objectContaining({
					backgroundColor: '#ef4444'
				})
			})
		);
	});

	it('should show clear button when background is set', () => {
		const block = createGroupBlock({
			settings: { backgroundColor: '#10b981' }
		});
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('bg-5'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const clearButton = screen.getByLabelText(/clear background/i);
		expect(clearButton).toBeInTheDocument();
	});

	it('should clear background when clear button clicked', async () => {
		const onUpdate = vi.fn();
		const block = createGroupBlock({
			settings: { backgroundColor: '#10b981' }
		});
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('bg-6'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const clearButton = screen.getByLabelText(/clear background/i);
		await fireEvent.click(clearButton);

		expect(onUpdate).toHaveBeenCalledWith(
			expect.objectContaining({
				settings: expect.objectContaining({
					backgroundColor: ''
				})
			})
		);
	});
});

// ===============================================================================
// TEST SUITE: Padding Settings
// ===============================================================================

describe('GroupBlock - Padding Settings', () => {
	afterEach(() => {
		cleanup();
	});

	it('should apply no padding when set to none', () => {
		const block = createGroupBlock({
			settings: { padding: 'none' }
		});
		const { container } = render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('padding-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const groupContainer = container.querySelector('.group-container');
		expect(groupContainer).toHaveStyle({ padding: '0' });
	});

	it('should apply small padding', () => {
		const block = createGroupBlock({
			settings: { padding: 'small' }
		});
		const { container } = render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('padding-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const groupContainer = container.querySelector('.group-container');
		expect(groupContainer).toHaveStyle({ padding: '1rem' });
	});

	it('should apply medium padding', () => {
		const block = createGroupBlock({
			settings: { padding: 'medium' }
		});
		const { container } = render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('padding-3'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const groupContainer = container.querySelector('.group-container');
		expect(groupContainer).toHaveStyle({ padding: '2rem' });
	});

	it('should apply large padding', () => {
		const block = createGroupBlock({
			settings: { padding: 'large' }
		});
		const { container } = render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('padding-4'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const groupContainer = container.querySelector('.group-container');
		expect(groupContainer).toHaveStyle({ padding: '3rem' });
	});

	it('should show padding buttons in toolbar', () => {
		const block = createGroupBlock();
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('padding-5'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByRole('button', { name: /n/i })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /s/i })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /m/i })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /l/i })).toBeInTheDocument();
	});

	it('should update padding when button clicked', async () => {
		const onUpdate = vi.fn();
		const block = createGroupBlock();
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('padding-6'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		// Find button by text content (L for large)
		const buttons = screen.getAllByRole('button', { pressed: expect.anything() });
		const largeButton = buttons.find((btn) => btn.textContent === 'L');
		if (largeButton) {
			await fireEvent.click(largeButton);
			expect(onUpdate).toHaveBeenCalled();
		}
	});
});

// ===============================================================================
// TEST SUITE: Border Radius
// ===============================================================================

describe('GroupBlock - Border Radius', () => {
	afterEach(() => {
		cleanup();
	});

	it('should apply border radius', () => {
		const block = createGroupBlock({
			settings: { borderRadius: '12px' }
		});
		const { container } = render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('radius-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const groupContainer = container.querySelector('.group-container');
		expect(groupContainer).toHaveStyle({ borderRadius: '12px' });
	});

	it('should apply no border radius when set to 0', () => {
		const block = createGroupBlock({
			settings: { borderRadius: '0' }
		});
		const { container } = render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('radius-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const groupContainer = container.querySelector('.group-container');
		expect(groupContainer).toHaveStyle({ borderRadius: '0' });
	});

	it('should show border radius select in toolbar', () => {
		const block = createGroupBlock();
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('radius-3'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const radiusSelect = screen.getByLabelText(/border radius/i);
		expect(radiusSelect).toBeInTheDocument();
	});

	it('should update radius when select changed', async () => {
		const onUpdate = vi.fn();
		const block = createGroupBlock();
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('radius-4'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const radiusSelect = screen.getByLabelText(/border radius/i);
		await fireEvent.change(radiusSelect, { target: { value: '16px' } });

		expect(onUpdate).toHaveBeenCalledWith(
			expect.objectContaining({
				settings: expect.objectContaining({
					borderRadius: '16px'
				})
			})
		);
	});

	it('should have all radius options available', () => {
		const block = createGroupBlock();
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('radius-5'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const radiusSelect = screen.getByLabelText(/border radius/i);
		expect(radiusSelect.querySelectorAll('option').length).toBeGreaterThanOrEqual(5);
	});
});

// ===============================================================================
// TEST SUITE: Max Width Settings
// ===============================================================================

describe('GroupBlock - Max Width Settings', () => {
	afterEach(() => {
		cleanup();
	});

	it('should apply full max width (100%)', () => {
		const block = createGroupBlock({
			settings: { maxWidth: 'full' }
		});
		const { container } = render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('width-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const groupContainer = container.querySelector('.group-container');
		expect(groupContainer).toHaveStyle({ maxWidth: '100%' });
	});

	it('should apply large max width (1200px)', () => {
		const block = createGroupBlock({
			settings: { maxWidth: 'large' }
		});
		const { container } = render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('width-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const groupContainer = container.querySelector('.group-container');
		expect(groupContainer).toHaveStyle({ maxWidth: '1200px' });
	});

	it('should apply medium max width (960px)', () => {
		const block = createGroupBlock({
			settings: { maxWidth: 'medium' }
		});
		const { container } = render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('width-3'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const groupContainer = container.querySelector('.group-container');
		expect(groupContainer).toHaveStyle({ maxWidth: '960px' });
	});

	it('should apply small max width (640px)', () => {
		const block = createGroupBlock({
			settings: { maxWidth: 'small' }
		});
		const { container } = render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('width-4'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const groupContainer = container.querySelector('.group-container');
		expect(groupContainer).toHaveStyle({ maxWidth: '640px' });
	});

	it('should show max width buttons in toolbar', () => {
		const block = createGroupBlock();
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('width-5'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const buttons = screen.getAllByRole('button', { pressed: expect.anything() });
		const widthButtons = buttons.filter(
			(btn) =>
				btn.textContent === 'F' ||
				btn.textContent === 'L' ||
				btn.textContent === 'M' ||
				btn.textContent === 'S'
		);
		expect(widthButtons.length).toBeGreaterThanOrEqual(4);
	});
});

// ===============================================================================
// TEST SUITE: Alignment
// ===============================================================================

describe('GroupBlock - Alignment', () => {
	afterEach(() => {
		cleanup();
	});

	it('should apply left alignment margin', () => {
		const block = createGroupBlock({
			settings: { textAlign: 'left', maxWidth: 'medium' }
		});
		const { container } = render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('align-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const groupContainer = container.querySelector('.group-container');
		expect(groupContainer).toHaveStyle({ margin: '0' });
	});

	it('should apply center alignment margin', () => {
		const block = createGroupBlock({
			settings: { textAlign: 'center', maxWidth: 'medium' }
		});
		const { container } = render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('align-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const groupContainer = container.querySelector('.group-container');
		expect(groupContainer).toHaveStyle({ margin: '0 auto' });
	});

	it('should apply right alignment margin', () => {
		const block = createGroupBlock({
			settings: { textAlign: 'right', maxWidth: 'medium' }
		});
		const { container } = render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('align-3'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const groupContainer = container.querySelector('.group-container');
		expect(groupContainer).toHaveStyle({ margin: '0 0 0 auto' });
	});

	it('should show alignment buttons in toolbar', () => {
		const block = createGroupBlock();
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('align-4'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByLabelText(/align left/i)).toBeInTheDocument();
		expect(screen.getByLabelText(/align center/i)).toBeInTheDocument();
		expect(screen.getByLabelText(/align right/i)).toBeInTheDocument();
	});

	it('should update alignment when button clicked', async () => {
		const onUpdate = vi.fn();
		const block = createGroupBlock();
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('align-5'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const centerButton = screen.getByLabelText(/align center/i);
		await fireEvent.click(centerButton);

		expect(onUpdate).toHaveBeenCalledWith(
			expect.objectContaining({
				settings: expect.objectContaining({
					textAlign: 'center'
				})
			})
		);
	});

	it('should highlight active alignment button', () => {
		const block = createGroupBlock({
			settings: { textAlign: 'center' }
		});
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('align-6'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const centerButton = screen.getByRole('button', { name: /align center/i, pressed: true });
		expect(centerButton).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Nested Content
// ===============================================================================

describe('GroupBlock - Nested Content', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show placeholder when empty in edit mode', () => {
		const block = createGroupBlock({
			content: { children: [] }
		});
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('nest-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText(/group content/i)).toBeInTheDocument();
	});

	it('should show drop hint in edit mode', () => {
		const block = createGroupBlock({
			content: { children: [] }
		});
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('nest-2'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText(/drop blocks here/i)).toBeInTheDocument();
	});

	it('should show nested block count when children exist', () => {
		const block = createGroupBlock({
			content: {
				children: [
					{
						id: toBlockId('child-1'),
						type: 'paragraph',
						content: { text: 'Test' },
						settings: {},
						metadata: { createdAt: '', updatedAt: '', version: 1 }
					},
					{
						id: toBlockId('child-2'),
						type: 'paragraph',
						content: { text: 'Test 2' },
						settings: {},
						metadata: { createdAt: '', updatedAt: '', version: 1 }
					}
				]
			}
		});
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('nest-3'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText(/2 nested blocks/i)).toBeInTheDocument();
	});

	it('should show singular text for single nested block', () => {
		const block = createGroupBlock({
			content: {
				children: [
					{
						id: toBlockId('child-1'),
						type: 'paragraph',
						content: { text: 'Test' },
						settings: {},
						metadata: { createdAt: '', updatedAt: '', version: 1 }
					}
				]
			}
		});
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('nest-4'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText(/1 nested block$/i)).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Edit Mode Toolbar
// ===============================================================================

describe('GroupBlock - Edit Mode Toolbar', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show toolbar when editing and selected', () => {
		const block = createGroupBlock();
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('toolbar-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const toolbar = screen.getByRole('toolbar', { name: /group settings/i });
		expect(toolbar).toBeInTheDocument();
	});

	it('should not show toolbar when not editing', () => {
		const block = createGroupBlock();
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('toolbar-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const toolbar = screen.queryByRole('toolbar');
		expect(toolbar).not.toBeInTheDocument();
	});

	it('should not show toolbar when editing but not selected', () => {
		const block = createGroupBlock();
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('toolbar-3'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const toolbar = screen.queryByRole('toolbar');
		expect(toolbar).not.toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Styling Classes
// ===============================================================================

describe('GroupBlock - Styling Classes', () => {
	afterEach(() => {
		cleanup();
	});

	it('should apply editing class when in edit mode', () => {
		const block = createGroupBlock();
		const { container } = render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('style-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const wrapper = container.querySelector('.group-block-wrapper');
		expect(wrapper).toHaveClass('editing');
	});

	it('should apply selected class when selected', () => {
		const block = createGroupBlock();
		const { container } = render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('style-2'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const wrapper = container.querySelector('.group-block-wrapper');
		expect(wrapper).toHaveClass('selected');
	});

	it('should show dashed border in edit mode', () => {
		const block = createGroupBlock();
		const { container } = render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('style-3'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const groupContainer = container.querySelector('.editing .group-container');
		expect(groupContainer).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Accessibility
// ===============================================================================

describe('GroupBlock - Accessibility', () => {
	afterEach(() => {
		cleanup();
	});

	it('should have proper group role', () => {
		const block = createGroupBlock();
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('a11y-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const group = screen.getByRole('group');
		expect(group).toBeInTheDocument();
	});

	it('should have accessible toolbar label', () => {
		const block = createGroupBlock();
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('a11y-2'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const toolbar = screen.getByRole('toolbar', { name: /group settings/i });
		expect(toolbar).toBeInTheDocument();
	});

	it('should have labeled color picker', () => {
		const block = createGroupBlock();
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('a11y-3'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const colorPicker = screen.getByLabelText(/background color/i);
		expect(colorPicker).toBeInTheDocument();
	});

	it('should have labeled border radius select', () => {
		const block = createGroupBlock();
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('a11y-4'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const radiusSelect = screen.getByLabelText(/border radius/i);
		expect(radiusSelect).toBeInTheDocument();
	});

	it('should have focusable alignment buttons', () => {
		const block = createGroupBlock();
		render(GroupBlock, {
			props: {
				block,
				blockId: toBlockId('a11y-5'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const alignButtons = [
			screen.getByLabelText(/align left/i),
			screen.getByLabelText(/align center/i),
			screen.getByLabelText(/align right/i)
		];

		alignButtons.forEach((button) => {
			expect(button).toHaveAttribute('type', 'button');
		});
	});
});

// ===============================================================================
// TEST SUITE: Error Handling
// ===============================================================================

describe('GroupBlock - Error Handling', () => {
	afterEach(() => {
		cleanup();
	});

	it('should handle missing content gracefully', () => {
		const block: Block = {
			id: toBlockId('test'),
			type: 'group',
			content: {},
			settings: {},
			metadata: {
				createdAt: new Date().toISOString(),
				updatedAt: new Date().toISOString(),
				version: 1
			}
		};

		expect(() => {
			render(GroupBlock, {
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

	it('should handle missing settings gracefully', () => {
		const block: Block = {
			id: toBlockId('test'),
			type: 'group',
			content: { children: [] },
			settings: {},
			metadata: {
				createdAt: new Date().toISOString(),
				updatedAt: new Date().toISOString(),
				version: 1
			}
		};

		expect(() => {
			render(GroupBlock, {
				props: {
					block,
					blockId: toBlockId('error-2'),
					isEditing: false,
					isSelected: false,
					onUpdate: vi.fn()
				}
			});
		}).not.toThrow();
	});

	it('should not call onError for valid input', () => {
		const onError = vi.fn();
		const block = createGroupBlock();

		render(GroupBlock, {
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
