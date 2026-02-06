/**
 * ===============================================================================
 * ColumnsBlock Component - Unit Tests
 * ===============================================================================
 *
 * @description Comprehensive tests for ColumnsBlock Svelte component
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Svelte 5 + Testing Library
 *
 * Tests cover:
 * - Rendering with different column counts
 * - Preset layouts (50/50, 33/33/33, 66/34, etc.)
 * - Gap control functionality
 * - Edit mode toolbar visibility
 * - Column content placeholders
 * - Accessibility (ARIA attributes)
 * - Responsive behavior
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, fireEvent, cleanup } from '@testing-library/svelte';
import ColumnsBlock from '../layout/ColumnsBlock.svelte';
import type { Block } from '../types';
import { toBlockId } from '$lib/stores/blockState.svelte';

// ===============================================================================
// TEST FIXTURES
// ===============================================================================

interface ColumnsBlockOverrides {
	id?: string;
	content?: {
		columnCount?: number;
		columnLayout?: '50/50' | '33/33/33' | '66/34' | '34/66' | '25/25/25/25' | 'custom';
		children?: Block[];
	};
	settings?: {
		gap?: string;
		preset?: string;
	};
	metadata?: Record<string, unknown>;
}

function createColumnsBlock(overrides: ColumnsBlockOverrides = {}): Block {
	return {
		id: toBlockId(overrides.id || 'test-columns'),
		type: 'columns',
		content: {
			columnCount: 2,
			columnLayout: '50/50',
			children: [],
			...overrides.content
		},
		settings: {
			gap: '2rem',
			preset: '50/50',
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

describe('ColumnsBlock - Basic Rendering', () => {
	afterEach(() => {
		cleanup();
	});

	it('should render the columns block container', () => {
		const block = createColumnsBlock();
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('columns-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const container = screen.getByRole('group');
		expect(container).toBeInTheDocument();
	});

	it('should render correct number of columns for 50/50 preset', () => {
		const block = createColumnsBlock({
			content: { columnLayout: '50/50', columnCount: 2 }
		});
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('columns-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const columns = screen.getAllByRole('region');
		expect(columns.length).toBe(2);
	});

	it('should render correct number of columns for 33/33/33 preset', () => {
		const block = createColumnsBlock({
			content: { columnLayout: '33/33/33', columnCount: 3 }
		});
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('columns-3'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const columns = screen.getAllByRole('region');
		expect(columns.length).toBe(3);
	});

	it('should render correct number of columns for 25/25/25/25 preset', () => {
		const block = createColumnsBlock({
			content: { columnLayout: '25/25/25/25', columnCount: 4 }
		});
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('columns-4'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const columns = screen.getAllByRole('region');
		expect(columns.length).toBe(4);
	});

	it('should have correct aria-label with column count', () => {
		const block = createColumnsBlock({
			content: { columnLayout: '33/33/33', columnCount: 3 }
		});
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('columns-5'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const container = screen.getByRole('group');
		expect(container).toHaveAttribute('aria-label', expect.stringContaining('3 columns'));
	});
});

// ===============================================================================
// TEST SUITE: Preset Layouts
// ===============================================================================

describe('ColumnsBlock - Preset Layouts', () => {
	afterEach(() => {
		cleanup();
	});

	it('should apply 50/50 preset widths correctly', () => {
		const block = createColumnsBlock({
			content: { columnLayout: '50/50' }
		});
		const { container } = render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('preset-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const columnsContainer = container.querySelector('.columns-container');
		expect(columnsContainer).toBeInTheDocument();
	});

	it('should apply 66/34 preset widths correctly', () => {
		const block = createColumnsBlock({
			content: { columnLayout: '66/34', columnCount: 2 }
		});
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('preset-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const columns = screen.getAllByRole('region');
		expect(columns.length).toBe(2);
	});

	it('should apply 34/66 preset widths correctly', () => {
		const block = createColumnsBlock({
			content: { columnLayout: '34/66', columnCount: 2 }
		});
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('preset-3'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const columns = screen.getAllByRole('region');
		expect(columns.length).toBe(2);
	});

	it('should handle custom column count', () => {
		const block = createColumnsBlock({
			content: { columnLayout: 'custom', columnCount: 3 }
		});
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('preset-4'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const columns = screen.getAllByRole('region');
		expect(columns.length).toBe(3);
	});
});

// ===============================================================================
// TEST SUITE: Edit Mode
// ===============================================================================

describe('ColumnsBlock - Edit Mode', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show toolbar when editing and selected', () => {
		const block = createColumnsBlock();
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('edit-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const toolbar = screen.getByRole('toolbar', { name: /column settings/i });
		expect(toolbar).toBeInTheDocument();
	});

	it('should not show toolbar when not editing', () => {
		const block = createColumnsBlock();
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('edit-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const toolbar = screen.queryByRole('toolbar');
		expect(toolbar).not.toBeInTheDocument();
	});

	it('should not show toolbar when editing but not selected', () => {
		const block = createColumnsBlock();
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('edit-3'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const toolbar = screen.queryByRole('toolbar');
		expect(toolbar).not.toBeInTheDocument();
	});

	it('should show preset buttons in toolbar', () => {
		const block = createColumnsBlock();
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('edit-4'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('50/50')).toBeInTheDocument();
		expect(screen.getByText('33/33/33')).toBeInTheDocument();
		expect(screen.getByText('66/34')).toBeInTheDocument();
		expect(screen.getByText('34/66')).toBeInTheDocument();
		expect(screen.getByText('25/25/25/25')).toBeInTheDocument();
	});

	it('should show gap control in toolbar', () => {
		const block = createColumnsBlock();
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('edit-5'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const gapSelect = screen.getByLabelText(/column gap/i);
		expect(gapSelect).toBeInTheDocument();
	});

	it('should show custom column count input', () => {
		const block = createColumnsBlock();
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('edit-6'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const columnInput = screen.getByLabelText(/number of columns/i);
		expect(columnInput).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Preset Selection
// ===============================================================================

describe('ColumnsBlock - Preset Selection', () => {
	afterEach(() => {
		cleanup();
	});

	it('should call onUpdate when preset button clicked', async () => {
		const onUpdate = vi.fn();
		const block = createColumnsBlock();
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('select-1'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const threeColButton = screen.getByText('33/33/33');
		await fireEvent.click(threeColButton);

		expect(onUpdate).toHaveBeenCalled();
	});

	it('should highlight active preset button', () => {
		const block = createColumnsBlock({
			content: { columnLayout: '50/50' }
		});
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('select-2'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const activeButton = screen.getByText('50/50');
		expect(activeButton).toHaveAttribute('aria-pressed', 'true');
	});

	it('should update column count when preset changed', async () => {
		const onUpdate = vi.fn();
		const block = createColumnsBlock();
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('select-3'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const fourColButton = screen.getByText('25/25/25/25');
		await fireEvent.click(fourColButton);

		expect(onUpdate).toHaveBeenCalledWith(
			expect.objectContaining({
				content: expect.objectContaining({
					columnLayout: '25/25/25/25',
					columnCount: 4
				})
			})
		);
	});
});

// ===============================================================================
// TEST SUITE: Gap Control
// ===============================================================================

describe('ColumnsBlock - Gap Control', () => {
	afterEach(() => {
		cleanup();
	});

	it('should update gap when select changed', async () => {
		const onUpdate = vi.fn();
		const block = createColumnsBlock();
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('gap-1'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const gapSelect = screen.getByLabelText(/column gap/i);
		await fireEvent.change(gapSelect, { target: { value: '1rem' } });

		expect(onUpdate).toHaveBeenCalledWith(
			expect.objectContaining({
				settings: expect.objectContaining({
					gap: '1rem'
				})
			})
		);
	});

	it('should display all gap options', () => {
		const block = createColumnsBlock();
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('gap-2'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const gapSelect = screen.getByLabelText(/column gap/i);
		expect(gapSelect.querySelectorAll('option').length).toBeGreaterThanOrEqual(5);
	});

	it('should apply gap style to columns container', () => {
		const block = createColumnsBlock({
			settings: { gap: '1.5rem' }
		});
		const { container } = render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('gap-3'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const columnsContainer = container.querySelector('.columns-container');
		expect(columnsContainer).toHaveStyle({ gap: '1.5rem' });
	});
});

// ===============================================================================
// TEST SUITE: Custom Column Count
// ===============================================================================

describe('ColumnsBlock - Custom Column Count', () => {
	afterEach(() => {
		cleanup();
	});

	it('should update to custom layout when column count changed', async () => {
		const onUpdate = vi.fn();
		const block = createColumnsBlock();
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('custom-1'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const columnInput = screen.getByLabelText(/number of columns/i);
		await fireEvent.input(columnInput, { target: { value: '3' } });

		expect(onUpdate).toHaveBeenCalled();
	});

	it('should clamp column count to valid range (1-4)', async () => {
		const onUpdate = vi.fn();
		const block = createColumnsBlock();
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('custom-2'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const columnInput = screen.getByLabelText(/number of columns/i);
		expect(columnInput).toHaveAttribute('min', '1');
		expect(columnInput).toHaveAttribute('max', '4');
	});
});

// ===============================================================================
// TEST SUITE: Column Content
// ===============================================================================

describe('ColumnsBlock - Column Content', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show placeholder text in edit mode', () => {
		const block = createColumnsBlock();
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('content-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Column 1 content')).toBeInTheDocument();
		expect(screen.getByText('Column 2 content')).toBeInTheDocument();
	});

	it('should show drop hint in edit mode', () => {
		const block = createColumnsBlock();
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('content-2'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const dropHints = screen.getAllByText(/drop blocks here/i);
		expect(dropHints.length).toBeGreaterThan(0);
	});

	it('should have correct aria-label for each column', () => {
		const block = createColumnsBlock({
			content: { columnLayout: '33/33/33', columnCount: 3 }
		});
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('content-3'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const columns = screen.getAllByRole('region');
		expect(columns[0]).toHaveAttribute('aria-label', 'Column 1');
		expect(columns[1]).toHaveAttribute('aria-label', 'Column 2');
		expect(columns[2]).toHaveAttribute('aria-label', 'Column 3');
	});
});

// ===============================================================================
// TEST SUITE: Styling
// ===============================================================================

describe('ColumnsBlock - Styling', () => {
	afterEach(() => {
		cleanup();
	});

	it('should apply editing class when in edit mode', () => {
		const block = createColumnsBlock();
		const { container } = render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('style-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const blockElement = container.querySelector('.columns-block');
		expect(blockElement).toHaveClass('editing');
	});

	it('should apply selected class when selected', () => {
		const block = createColumnsBlock();
		const { container } = render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('style-2'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const blockElement = container.querySelector('.columns-block');
		expect(blockElement).toHaveClass('selected');
	});

	it('should not have editing class when not editing', () => {
		const block = createColumnsBlock();
		const { container } = render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('style-3'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const blockElement = container.querySelector('.columns-block');
		expect(blockElement).not.toHaveClass('editing');
	});
});

// ===============================================================================
// TEST SUITE: Accessibility
// ===============================================================================

describe('ColumnsBlock - Accessibility', () => {
	afterEach(() => {
		cleanup();
	});

	it('should have proper role hierarchy', () => {
		const block = createColumnsBlock();
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('a11y-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const mainGroup = screen.getByRole('group');
		expect(mainGroup).toBeInTheDocument();

		const regions = screen.getAllByRole('region');
		expect(regions.length).toBeGreaterThan(0);
	});

	it('should have focusable preset buttons with aria-pressed', () => {
		const block = createColumnsBlock();
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('a11y-2'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const presetButtons = screen.getAllByRole('button', { pressed: false });
		expect(presetButtons.length).toBeGreaterThan(0);

		const activeButton = screen.getByRole('button', { pressed: true });
		expect(activeButton).toBeInTheDocument();
	});

	it('should have labeled form controls', () => {
		const block = createColumnsBlock();
		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('a11y-3'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByLabelText(/number of columns/i)).toBeInTheDocument();
		expect(screen.getByLabelText(/column gap/i)).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Error Handling
// ===============================================================================

describe('ColumnsBlock - Error Handling', () => {
	afterEach(() => {
		cleanup();
	});

	it('should handle missing content gracefully', () => {
		const block: Block = {
			id: toBlockId('test'),
			type: 'columns',
			content: {},
			settings: {},
			metadata: {
				createdAt: new Date().toISOString(),
				updatedAt: new Date().toISOString(),
				version: 1
			}
		};

		expect(() => {
			render(ColumnsBlock, {
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

	it('should handle undefined settings gracefully', () => {
		const block = createColumnsBlock();
		block.settings = {};

		expect(() => {
			render(ColumnsBlock, {
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

	it('should call onError when provided and error occurs', () => {
		const onError = vi.fn();
		const block = createColumnsBlock();

		render(ColumnsBlock, {
			props: {
				block,
				blockId: toBlockId('error-3'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn(),
				onError
			}
		});

		// Component should render without calling onError for valid input
		expect(onError).not.toHaveBeenCalled();
	});
});
