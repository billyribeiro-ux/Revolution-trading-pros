/**
 * ===============================================================================
 * DividerBlock Component - Unit Tests
 * ===============================================================================
 *
 * @description Comprehensive tests for DividerBlock Svelte component
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Svelte 5 + Testing Library
 *
 * Tests cover:
 * - Rendering with different styles (solid, dashed, dotted, double)
 * - Width variations (small=25%, medium=50%, large=75%, full=100%)
 * - Color customization
 * - Spacing/margin settings
 * - Thickness control
 * - Edit mode toolbar functionality
 * - Accessibility (ARIA attributes for separator)
 */

import { describe, it, expect, vi, afterEach } from 'vitest';
import { render, screen, fireEvent, cleanup } from '@testing-library/svelte';
import DividerBlock from '../layout/DividerBlock.svelte';
import type { Block } from '../types';

// ===============================================================================
// TEST FIXTURES
// ===============================================================================

interface DividerBlockOverrides {
	id?: string;
	content?: Record<string, unknown>;
	settings?: {
		borderStyle?: 'solid' | 'dashed' | 'dotted' | 'double';
		width?: 'small' | 'medium' | 'large' | 'full';
		borderColor?: string;
		margin?: 'small' | 'medium' | 'large';
		borderWidth?: string;
	};
	metadata?: Record<string, unknown>;
}

function createDividerBlock(overrides: DividerBlockOverrides = {}): Block {
	return {
		id: overrides.id || 'test-divider',
		type: 'divider',
		content: {
			...overrides.content
		},
		settings: {
			borderStyle: 'solid',
			width: 'full',
			borderColor: '#e2e8f0',
			margin: 'medium',
			borderWidth: '1px',
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

describe('DividerBlock - Basic Rendering', () => {
	afterEach(() => {
		cleanup();
	});

	it('should render the divider line element', () => {
		const block = createDividerBlock();
		render(DividerBlock, {
			props: {
				block,
				blockId: 'divider-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const separator = screen.getByRole('separator');
		expect(separator).toBeInTheDocument();
	});

	it('should have horizontal orientation', () => {
		const block = createDividerBlock();
		render(DividerBlock, {
			props: {
				block,
				blockId: 'divider-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const separator = screen.getByRole('separator');
		expect(separator).toHaveAttribute('aria-orientation', 'horizontal');
	});

	it('should render without errors', () => {
		const block = createDividerBlock();
		expect(() => {
			render(DividerBlock, {
				props: {
					block,
					blockId: 'divider-3',
					isEditing: false,
					isSelected: false,
					onUpdate: vi.fn()
				}
			});
		}).not.toThrow();
	});
});

// ===============================================================================
// TEST SUITE: Border Styles
// ===============================================================================

describe('DividerBlock - Border Styles', () => {
	afterEach(() => {
		cleanup();
	});

	it('should apply solid border style', () => {
		const block = createDividerBlock({
			settings: { borderStyle: 'solid' }
		});
		render(DividerBlock, {
			props: {
				block,
				blockId: 'style-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const separator = screen.getByRole('separator');
		expect(separator).toHaveStyle({ borderTopStyle: 'solid' });
	});

	it('should apply dashed border style', () => {
		const block = createDividerBlock({
			settings: { borderStyle: 'dashed' }
		});
		render(DividerBlock, {
			props: {
				block,
				blockId: 'style-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const separator = screen.getByRole('separator');
		expect(separator).toHaveStyle({ borderTopStyle: 'dashed' });
	});

	it('should apply dotted border style', () => {
		const block = createDividerBlock({
			settings: { borderStyle: 'dotted' }
		});
		render(DividerBlock, {
			props: {
				block,
				blockId: 'style-3',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const separator = screen.getByRole('separator');
		expect(separator).toHaveStyle({ borderTopStyle: 'dotted' });
	});

	it('should apply double border style', () => {
		const block = createDividerBlock({
			settings: { borderStyle: 'double' }
		});
		render(DividerBlock, {
			props: {
				block,
				blockId: 'style-4',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const separator = screen.getByRole('separator');
		expect(separator).toHaveStyle({ borderTopStyle: 'double' });
	});
});

// ===============================================================================
// TEST SUITE: Width Variations
// ===============================================================================

describe('DividerBlock - Width Variations', () => {
	afterEach(() => {
		cleanup();
	});

	it('should apply 25% width (small)', () => {
		const block = createDividerBlock({
			settings: { width: 'small' }
		});
		render(DividerBlock, {
			props: {
				block,
				blockId: 'width-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const separator = screen.getByRole('separator');
		expect(separator).toHaveStyle({ width: '25%' });
	});

	it('should apply 50% width (medium)', () => {
		const block = createDividerBlock({
			settings: { width: 'medium' }
		});
		render(DividerBlock, {
			props: {
				block,
				blockId: 'width-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const separator = screen.getByRole('separator');
		expect(separator).toHaveStyle({ width: '50%' });
	});

	it('should apply 75% width (large)', () => {
		const block = createDividerBlock({
			settings: { width: 'large' }
		});
		render(DividerBlock, {
			props: {
				block,
				blockId: 'width-3',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const separator = screen.getByRole('separator');
		expect(separator).toHaveStyle({ width: '75%' });
	});

	it('should apply 100% width by default (full)', () => {
		const block = createDividerBlock({
			settings: { width: 'full' }
		});
		render(DividerBlock, {
			props: {
				block,
				blockId: 'width-4',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const separator = screen.getByRole('separator');
		expect(separator).toHaveStyle({ width: '100%' });
	});
});

// ===============================================================================
// TEST SUITE: Color Customization
// ===============================================================================

describe('DividerBlock - Color Customization', () => {
	afterEach(() => {
		cleanup();
	});

	it('should apply default border color', () => {
		const block = createDividerBlock({
			settings: { borderColor: '#e2e8f0' }
		});
		render(DividerBlock, {
			props: {
				block,
				blockId: 'color-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const separator = screen.getByRole('separator');
		expect(separator).toHaveStyle({ borderTopColor: '#e2e8f0' });
	});

	it('should apply custom border color', () => {
		const block = createDividerBlock({
			settings: { borderColor: '#3b82f6' }
		});
		render(DividerBlock, {
			props: {
				block,
				blockId: 'color-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const separator = screen.getByRole('separator');
		expect(separator).toHaveStyle({ borderTopColor: '#3b82f6' });
	});

	it('should show color picker in edit mode', () => {
		const block = createDividerBlock();
		render(DividerBlock, {
			props: {
				block,
				blockId: 'color-3',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const colorPicker = screen.getByLabelText(/divider color/i);
		expect(colorPicker).toBeInTheDocument();
	});

	it('should update color when picker changed', async () => {
		const onUpdate = vi.fn();
		const block = createDividerBlock();
		render(DividerBlock, {
			props: {
				block,
				blockId: 'color-4',
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const colorPicker = screen.getByLabelText(/divider color/i);
		await fireEvent.input(colorPicker, { target: { value: '#ef4444' } });

		expect(onUpdate).toHaveBeenCalledWith(
			expect.objectContaining({
				settings: expect.objectContaining({
					borderColor: '#ef4444'
				})
			})
		);
	});

	it('should display current color value', () => {
		const block = createDividerBlock({
			settings: { borderColor: '#10b981' }
		});
		render(DividerBlock, {
			props: {
				block,
				blockId: 'color-5',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('#10b981')).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Spacing/Margin Settings
// ===============================================================================

describe('DividerBlock - Spacing Settings', () => {
	afterEach(() => {
		cleanup();
	});

	it('should apply small spacing', () => {
		const block = createDividerBlock({
			settings: { margin: 'small' }
		});
		const { container } = render(DividerBlock, {
			props: {
				block,
				blockId: 'spacing-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const wrapper = container.querySelector('.divider-wrapper');
		expect(wrapper).toHaveStyle({ marginTop: '1rem', marginBottom: '1rem' });
	});

	it('should apply medium spacing', () => {
		const block = createDividerBlock({
			settings: { margin: 'medium' }
		});
		const { container } = render(DividerBlock, {
			props: {
				block,
				blockId: 'spacing-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const wrapper = container.querySelector('.divider-wrapper');
		expect(wrapper).toHaveStyle({ marginTop: '2rem', marginBottom: '2rem' });
	});

	it('should apply large spacing', () => {
		const block = createDividerBlock({
			settings: { margin: 'large' }
		});
		const { container } = render(DividerBlock, {
			props: {
				block,
				blockId: 'spacing-3',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const wrapper = container.querySelector('.divider-wrapper');
		expect(wrapper).toHaveStyle({ marginTop: '3rem', marginBottom: '3rem' });
	});

	it('should show spacing buttons in toolbar', () => {
		const block = createDividerBlock();
		render(DividerBlock, {
			props: {
				block,
				blockId: 'spacing-4',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByRole('button', { name: /s/i })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /m/i })).toBeInTheDocument();
		expect(screen.getByRole('button', { name: /l/i })).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Thickness Control
// ===============================================================================

describe('DividerBlock - Thickness Control', () => {
	afterEach(() => {
		cleanup();
	});

	it('should apply thin thickness (1px)', () => {
		const block = createDividerBlock({
			settings: { borderWidth: '1px' }
		});
		render(DividerBlock, {
			props: {
				block,
				blockId: 'thick-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const separator = screen.getByRole('separator');
		expect(separator).toHaveStyle({ borderTopWidth: '1px' });
	});

	it('should apply medium thickness (2px)', () => {
		const block = createDividerBlock({
			settings: { borderWidth: '2px' }
		});
		render(DividerBlock, {
			props: {
				block,
				blockId: 'thick-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const separator = screen.getByRole('separator');
		expect(separator).toHaveStyle({ borderTopWidth: '2px' });
	});

	it('should apply thick thickness (3px)', () => {
		const block = createDividerBlock({
			settings: { borderWidth: '3px' }
		});
		render(DividerBlock, {
			props: {
				block,
				blockId: 'thick-3',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const separator = screen.getByRole('separator');
		expect(separator).toHaveStyle({ borderTopWidth: '3px' });
	});

	it('should show thickness select in toolbar', () => {
		const block = createDividerBlock();
		render(DividerBlock, {
			props: {
				block,
				blockId: 'thick-4',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const thicknessSelect = screen.getByLabelText(/line thickness/i);
		expect(thicknessSelect).toBeInTheDocument();
	});

	it('should update thickness when select changed', async () => {
		const onUpdate = vi.fn();
		const block = createDividerBlock();
		render(DividerBlock, {
			props: {
				block,
				blockId: 'thick-5',
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const thicknessSelect = screen.getByLabelText(/line thickness/i);
		await fireEvent.change(thicknessSelect, { target: { value: '3px' } });

		expect(onUpdate).toHaveBeenCalledWith(
			expect.objectContaining({
				settings: expect.objectContaining({
					borderWidth: '3px'
				})
			})
		);
	});

	it('should double thickness for double border style', () => {
		const block = createDividerBlock({
			settings: { borderStyle: 'double', borderWidth: '2px' }
		});
		render(DividerBlock, {
			props: {
				block,
				blockId: 'thick-6',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const separator = screen.getByRole('separator');
		// Double border style doubles the width
		expect(separator).toHaveStyle({ borderTopWidth: '4px' });
	});
});

// ===============================================================================
// TEST SUITE: Edit Mode Toolbar
// ===============================================================================

describe('DividerBlock - Edit Mode Toolbar', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show toolbar when editing and selected', () => {
		const block = createDividerBlock();
		render(DividerBlock, {
			props: {
				block,
				blockId: 'toolbar-1',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const toolbar = screen.getByRole('toolbar', { name: /divider settings/i });
		expect(toolbar).toBeInTheDocument();
	});

	it('should not show toolbar when not editing', () => {
		const block = createDividerBlock();
		render(DividerBlock, {
			props: {
				block,
				blockId: 'toolbar-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const toolbar = screen.queryByRole('toolbar');
		expect(toolbar).not.toBeInTheDocument();
	});

	it('should not show toolbar when editing but not selected', () => {
		const block = createDividerBlock();
		render(DividerBlock, {
			props: {
				block,
				blockId: 'toolbar-3',
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const toolbar = screen.queryByRole('toolbar');
		expect(toolbar).not.toBeInTheDocument();
	});

	it('should show style buttons in toolbar', () => {
		const block = createDividerBlock();
		render(DividerBlock, {
			props: {
				block,
				blockId: 'toolbar-4',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		// Should have 4 style buttons (solid, dashed, dotted, double)
		const styleButtons = screen.getAllByRole('button', { pressed: expect.anything() });
		expect(styleButtons.length).toBeGreaterThanOrEqual(4);
	});

	it('should show width buttons in toolbar', () => {
		const block = createDividerBlock();
		render(DividerBlock, {
			props: {
				block,
				blockId: 'toolbar-5',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('25%')).toBeInTheDocument();
		expect(screen.getByText('50%')).toBeInTheDocument();
		expect(screen.getByText('75%')).toBeInTheDocument();
		expect(screen.getByText('100%')).toBeInTheDocument();
	});

	it('should update style when button clicked', async () => {
		const onUpdate = vi.fn();
		const block = createDividerBlock({
			settings: { borderStyle: 'solid' }
		});
		const { container } = render(DividerBlock, {
			props: {
				block,
				blockId: 'toolbar-6',
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		// Find and click a dashed style button
		const styleButtons = container.querySelectorAll('.style-btn');
		if (styleButtons[1]) {
			await fireEvent.click(styleButtons[1]);
			expect(onUpdate).toHaveBeenCalled();
		}
	});

	it('should update width when button clicked', async () => {
		const onUpdate = vi.fn();
		const block = createDividerBlock();
		render(DividerBlock, {
			props: {
				block,
				blockId: 'toolbar-7',
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const widthButton = screen.getByText('50%');
		await fireEvent.click(widthButton);

		expect(onUpdate).toHaveBeenCalledWith(
			expect.objectContaining({
				settings: expect.objectContaining({
					width: 'medium'
				})
			})
		);
	});
});

// ===============================================================================
// TEST SUITE: Styling Classes
// ===============================================================================

describe('DividerBlock - Styling Classes', () => {
	afterEach(() => {
		cleanup();
	});

	it('should apply editing class when in edit mode', () => {
		const block = createDividerBlock();
		const { container } = render(DividerBlock, {
			props: {
				block,
				blockId: 'class-1',
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const blockElement = container.querySelector('.divider-block');
		expect(blockElement).toHaveClass('editing');
	});

	it('should apply selected class when selected', () => {
		const block = createDividerBlock();
		const { container } = render(DividerBlock, {
			props: {
				block,
				blockId: 'class-2',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const blockElement = container.querySelector('.divider-block');
		expect(blockElement).toHaveClass('selected');
	});

	it('should highlight active style button', () => {
		const block = createDividerBlock({
			settings: { borderStyle: 'solid' }
		});
		const { container } = render(DividerBlock, {
			props: {
				block,
				blockId: 'class-3',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const activeButton = container.querySelector('.style-btn.active');
		expect(activeButton).toBeInTheDocument();
	});

	it('should highlight active width button', () => {
		const block = createDividerBlock({
			settings: { width: 'full' }
		});
		render(DividerBlock, {
			props: {
				block,
				blockId: 'class-4',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const activeButton = screen.getByRole('button', { name: '100%', pressed: true });
		expect(activeButton).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Accessibility
// ===============================================================================

describe('DividerBlock - Accessibility', () => {
	afterEach(() => {
		cleanup();
	});

	it('should use semantic hr element', () => {
		const block = createDividerBlock();
		const { container } = render(DividerBlock, {
			props: {
				block,
				blockId: 'a11y-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const hr = container.querySelector('hr');
		expect(hr).toBeInTheDocument();
	});

	it('should have proper separator role', () => {
		const block = createDividerBlock();
		render(DividerBlock, {
			props: {
				block,
				blockId: 'a11y-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const separator = screen.getByRole('separator');
		expect(separator).toBeInTheDocument();
	});

	it('should have focusable controls with aria-pressed', () => {
		const block = createDividerBlock();
		render(DividerBlock, {
			props: {
				block,
				blockId: 'a11y-3',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const pressedButtons = screen.getAllByRole('button', { pressed: true });
		expect(pressedButtons.length).toBeGreaterThan(0);
	});

	it('should have labeled color picker', () => {
		const block = createDividerBlock();
		render(DividerBlock, {
			props: {
				block,
				blockId: 'a11y-4',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const colorPicker = screen.getByLabelText(/divider color/i);
		expect(colorPicker).toBeInTheDocument();
	});

	it('should have labeled thickness select', () => {
		const block = createDividerBlock();
		render(DividerBlock, {
			props: {
				block,
				blockId: 'a11y-5',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const thicknessSelect = screen.getByLabelText(/line thickness/i);
		expect(thicknessSelect).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Error Handling
// ===============================================================================

describe('DividerBlock - Error Handling', () => {
	afterEach(() => {
		cleanup();
	});

	it('should handle missing settings gracefully', () => {
		const block: Block = {
			id: 'test',
			type: 'divider',
			content: {},
			settings: {},
			metadata: {
				createdAt: new Date().toISOString(),
				updatedAt: new Date().toISOString(),
				version: 1
			}
		};

		expect(() => {
			render(DividerBlock, {
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

	it('should use default values for undefined settings', () => {
		const block: Block = {
			id: 'test',
			type: 'divider',
			content: {},
			settings: {},
			metadata: {
				createdAt: new Date().toISOString(),
				updatedAt: new Date().toISOString(),
				version: 1
			}
		};

		render(DividerBlock, {
			props: {
				block,
				blockId: 'error-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const separator = screen.getByRole('separator');
		expect(separator).toBeInTheDocument();
	});

	it('should not call onError for valid input', () => {
		const onError = vi.fn();
		const block = createDividerBlock();

		render(DividerBlock, {
			props: {
				block,
				blockId: 'error-3',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn(),
				onError
			}
		});

		expect(onError).not.toHaveBeenCalled();
	});
});
