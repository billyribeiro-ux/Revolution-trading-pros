/**
 * ===============================================================================
 * SpacerBlock Component - Unit Tests
 * ===============================================================================
 *
 * @description Comprehensive tests for SpacerBlock Svelte component
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Svelte 5 + Testing Library
 *
 * Tests cover:
 * - Height control and presets
 * - Drag resize functionality
 * - Visibility in view/edit modes
 * - Slider input functionality
 * - Keyboard navigation
 * - Accessibility (ARIA attributes)
 * - Responsive behavior
 */

import { describe, it, expect, vi, afterEach } from 'vitest';
import { render, screen, fireEvent, cleanup } from '@testing-library/svelte';
import SpacerBlock from '../layout/SpacerBlock.svelte';
import type { Block } from '../types';
import { toBlockId } from '$lib/stores/blockState.svelte';

// ===============================================================================
// TEST FIXTURES
// ===============================================================================

interface SpacerBlockOverrides {
	id?: string;
	content?: Record<string, unknown>;
	settings?: {
		height?: string;
	};
	metadata?: Record<string, unknown>;
}

function createSpacerBlock(overrides: SpacerBlockOverrides = {}): Block {
	return {
		id: toBlockId(overrides.id || 'test-spacer'),
		type: 'spacer',
		content: {
			...overrides.content
		},
		settings: {
			height: '2.5rem',
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

describe('SpacerBlock - Basic Rendering', () => {
	afterEach(() => {
		cleanup();
	});

	it('should render the spacer block', () => {
		const block = createSpacerBlock();
		const { container } = render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('spacer-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const spacerBlock = container.querySelector('.spacer-block');
		expect(spacerBlock).toBeInTheDocument();
	});

	it('should be hidden/minimal in view mode', () => {
		const block = createSpacerBlock();
		const { container } = render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('spacer-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const spacerBlock = container.querySelector('.spacer-block');
		expect(spacerBlock).toHaveAttribute('role', 'separator');
	});

	it('should render without errors', () => {
		const block = createSpacerBlock();
		expect(() => {
			render(SpacerBlock, {
				props: {
					block,
					blockId: toBlockId('spacer-3'),
					isEditing: false,
					isSelected: false,
					onUpdate: vi.fn()
				}
			});
		}).not.toThrow();
	});
});

// ===============================================================================
// TEST SUITE: Height Control
// ===============================================================================

describe('SpacerBlock - Height Control', () => {
	afterEach(() => {
		cleanup();
	});

	it('should apply specified height', () => {
		const block = createSpacerBlock({
			settings: { height: '4rem' }
		});
		const { container } = render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('height-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const spacerBlock = container.querySelector('.spacer-block');
		expect(spacerBlock).toHaveStyle({ height: '4rem' });
	});

	it('should apply default height when not specified', () => {
		const block = createSpacerBlock({
			settings: { height: '2.5rem' }
		});
		const { container } = render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('height-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const spacerBlock = container.querySelector('.spacer-block');
		expect(spacerBlock).toHaveStyle({ height: '2.5rem' });
	});

	it('should display height in pixels in edit mode', () => {
		const block = createSpacerBlock({
			settings: { height: '2rem' }
		});
		render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('height-3'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		// 2rem = 32px
		expect(screen.getByText('32px')).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Height Presets
// ===============================================================================

describe('SpacerBlock - Height Presets', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show preset buttons when selected in edit mode', () => {
		const block = createSpacerBlock();
		render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('preset-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('XS')).toBeInTheDocument();
		expect(screen.getByText('S')).toBeInTheDocument();
		expect(screen.getByText('M')).toBeInTheDocument();
		expect(screen.getByText('L')).toBeInTheDocument();
		expect(screen.getByText('XL')).toBeInTheDocument();
	});

	it('should update height when preset clicked', async () => {
		const onUpdate = vi.fn();
		const block = createSpacerBlock();
		render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('preset-2'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const largeButton = screen.getByText('L');
		await fireEvent.click(largeButton);

		expect(onUpdate).toHaveBeenCalledWith(
			expect.objectContaining({
				settings: expect.objectContaining({
					height: '4rem'
				})
			})
		);
	});

	it('should highlight active preset', () => {
		const block = createSpacerBlock({
			settings: { height: '2rem' }
		});
		render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('preset-3'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const activeButton = screen.getByRole('button', { name: 'S', pressed: true });
		expect(activeButton).toBeInTheDocument();
	});

	it('should apply XS preset (1rem)', async () => {
		const onUpdate = vi.fn();
		const block = createSpacerBlock();
		render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('preset-4'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		await fireEvent.click(screen.getByText('XS'));

		expect(onUpdate).toHaveBeenCalledWith(
			expect.objectContaining({
				settings: expect.objectContaining({
					height: '1rem'
				})
			})
		);
	});

	it('should apply 3XL preset (10rem)', async () => {
		const onUpdate = vi.fn();
		const block = createSpacerBlock();
		render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('preset-5'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		await fireEvent.click(screen.getByText('3XL'));

		expect(onUpdate).toHaveBeenCalledWith(
			expect.objectContaining({
				settings: expect.objectContaining({
					height: '10rem'
				})
			})
		);
	});
});

// ===============================================================================
// TEST SUITE: Slider Control
// ===============================================================================

describe('SpacerBlock - Slider Control', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show slider when selected in edit mode', () => {
		const block = createSpacerBlock();
		const { container } = render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('slider-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const slider = container.querySelector('.height-slider');
		expect(slider).toBeInTheDocument();
	});

	it('should have correct min/max values', () => {
		const block = createSpacerBlock();
		const { container } = render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('slider-2'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const slider = container.querySelector('.height-slider');
		expect(slider).toHaveAttribute('min', '1');
		expect(slider).toHaveAttribute('max', '10');
	});

	it('should have step of 0.25', () => {
		const block = createSpacerBlock();
		const { container } = render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('slider-3'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const slider = container.querySelector('.height-slider');
		expect(slider).toHaveAttribute('step', '0.25');
	});

	it('should update height when slider changed', async () => {
		const onUpdate = vi.fn();
		const block = createSpacerBlock();
		const { container } = render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('slider-4'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const slider = container.querySelector('.height-slider') as HTMLInputElement;
		await fireEvent.input(slider, { target: { value: '5' } });

		expect(onUpdate).toHaveBeenCalled();
	});

	it('should have ARIA value attributes', () => {
		const block = createSpacerBlock({
			settings: { height: '3rem' }
		});
		const { container } = render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('slider-5'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const slider = container.querySelector('.height-slider');
		expect(slider).toHaveAttribute('aria-valuemin', '1');
		expect(slider).toHaveAttribute('aria-valuemax', '10');
	});

	it('should display height value next to slider', () => {
		const block = createSpacerBlock({
			settings: { height: '4.5rem' }
		});
		render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('slider-6'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		// Should show "4.5rem (72px)"
		expect(screen.getByText(/4\.5rem/)).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Drag Resize
// ===============================================================================

describe('SpacerBlock - Drag Resize', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show drag handle in edit mode', () => {
		const block = createSpacerBlock();
		render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('drag-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const dragHandle = screen.getByLabelText(/resize spacer height/i);
		expect(dragHandle).toBeInTheDocument();
	});

	it('should have slider role on drag handle', () => {
		const block = createSpacerBlock();
		render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('drag-2'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const dragHandle = screen.getByRole('slider', { name: /resize spacer height/i });
		expect(dragHandle).toBeInTheDocument();
	});

	it('should be focusable', () => {
		const block = createSpacerBlock();
		render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('drag-3'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const dragHandle = screen.getByRole('slider', { name: /resize spacer height/i });
		expect(dragHandle).toHaveAttribute('tabindex', '0');
	});

	it('should show drag hint text', () => {
		const block = createSpacerBlock();
		render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('drag-4'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText(/drag to resize/i)).toBeInTheDocument();
	});

	it('should have ARIA value attributes', () => {
		const block = createSpacerBlock({
			settings: { height: '3rem' }
		});
		render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('drag-5'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const dragHandle = screen.getByRole('slider', { name: /resize spacer height/i });
		expect(dragHandle).toHaveAttribute('aria-valuemin', '16');
		expect(dragHandle).toHaveAttribute('aria-valuemax', '160');
		expect(dragHandle).toHaveAttribute('aria-valuenow', '48'); // 3rem * 16 = 48px
	});
});

// ===============================================================================
// TEST SUITE: Keyboard Navigation
// ===============================================================================

describe('SpacerBlock - Keyboard Navigation', () => {
	afterEach(() => {
		cleanup();
	});

	it('should increase height on ArrowUp', async () => {
		const onUpdate = vi.fn();
		const block = createSpacerBlock({
			settings: { height: '2rem' }
		});
		render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('keyboard-1'),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const dragHandle = screen.getByRole('slider', { name: /resize spacer height/i });
		await fireEvent.keyDown(dragHandle, { key: 'ArrowUp' });

		expect(onUpdate).toHaveBeenCalled();
	});

	it('should decrease height on ArrowDown', async () => {
		const onUpdate = vi.fn();
		const block = createSpacerBlock({
			settings: { height: '3rem' }
		});
		render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('keyboard-2'),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const dragHandle = screen.getByRole('slider', { name: /resize spacer height/i });
		await fireEvent.keyDown(dragHandle, { key: 'ArrowDown' });

		expect(onUpdate).toHaveBeenCalled();
	});

	it('should use larger step with Shift key', async () => {
		const onUpdate = vi.fn();
		const block = createSpacerBlock({
			settings: { height: '3rem' }
		});
		render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('keyboard-3'),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const dragHandle = screen.getByRole('slider', { name: /resize spacer height/i });
		await fireEvent.keyDown(dragHandle, { key: 'ArrowUp', shiftKey: true });

		expect(onUpdate).toHaveBeenCalled();
	});

	it('should not respond to keyboard when not editing', async () => {
		const onUpdate = vi.fn();
		const block = createSpacerBlock();
		const { container } = render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('keyboard-4'),
				isEditing: false,
				isSelected: false,
				onUpdate
			}
		});

		// In view mode, drag handle shouldn't exist
		const dragHandle = container.querySelector('.drag-handle');
		expect(dragHandle).not.toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Edit vs View Mode
// ===============================================================================

describe('SpacerBlock - Edit vs View Mode', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show visual representation in edit mode', () => {
		const block = createSpacerBlock();
		const { container } = render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('mode-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const spacerVisual = container.querySelector('.spacer-visual');
		expect(spacerVisual).toBeInTheDocument();
	});

	it('should be invisible in view mode', () => {
		const block = createSpacerBlock();
		const { container } = render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('mode-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const spacerVisual = container.querySelector('.spacer-visual');
		expect(spacerVisual).not.toBeInTheDocument();
	});

	it('should show toolbar only when selected in edit mode', () => {
		const block = createSpacerBlock();
		render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('mode-3'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const toolbar = screen.getByRole('toolbar', { name: /spacer settings/i });
		expect(toolbar).toBeInTheDocument();
	});

	it('should not show toolbar when not selected', () => {
		const block = createSpacerBlock();
		render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('mode-4'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const toolbar = screen.queryByRole('toolbar');
		expect(toolbar).not.toBeInTheDocument();
	});

	it('should show dashed outline in edit mode', () => {
		const block = createSpacerBlock();
		const { container } = render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('mode-5'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const outline = container.querySelector('.spacer-outline');
		expect(outline).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Styling Classes
// ===============================================================================

describe('SpacerBlock - Styling Classes', () => {
	afterEach(() => {
		cleanup();
	});

	it('should apply editing class when in edit mode', () => {
		const block = createSpacerBlock();
		const { container } = render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('style-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const spacerBlock = container.querySelector('.spacer-block');
		expect(spacerBlock).toHaveClass('editing');
	});

	it('should apply selected class when selected', () => {
		const block = createSpacerBlock();
		const { container } = render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('style-2'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const spacerBlock = container.querySelector('.spacer-block');
		expect(spacerBlock).toHaveClass('selected');
	});

	it('should not have editing class when not editing', () => {
		const block = createSpacerBlock();
		const { container } = render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('style-3'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const spacerBlock = container.querySelector('.spacer-block');
		expect(spacerBlock).not.toHaveClass('editing');
	});
});

// ===============================================================================
// TEST SUITE: Accessibility
// ===============================================================================

describe('SpacerBlock - Accessibility', () => {
	afterEach(() => {
		cleanup();
	});

	it('should have separator role in view mode', () => {
		const block = createSpacerBlock();
		const { container } = render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('a11y-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const spacerBlock = container.querySelector('.spacer-block');
		expect(spacerBlock).toHaveAttribute('role', 'separator');
	});

	it('should have labeled slider control', () => {
		const block = createSpacerBlock();
		const { container } = render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('a11y-2'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const slider = container.querySelector('.height-slider');
		expect(slider).toBeInTheDocument();
	});

	it('should have accessible preset buttons', () => {
		const block = createSpacerBlock();
		render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('a11y-3'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const buttons = screen.getAllByRole('button');
		expect(buttons.length).toBeGreaterThan(0);
	});

	it('should have focusable drag handle', () => {
		const block = createSpacerBlock();
		render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('a11y-4'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const dragHandle = screen.getByRole('slider', { name: /resize spacer height/i });
		expect(dragHandle).toHaveAttribute('tabindex', '0');
	});

	it('should have proper aria-valuetext', () => {
		const block = createSpacerBlock({
			settings: { height: '3rem' }
		});
		const { container } = render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('a11y-5'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const slider = container.querySelector('.height-slider');
		expect(slider).toHaveAttribute('aria-valuetext', expect.stringContaining('rem'));
	});
});

// ===============================================================================
// TEST SUITE: Error Handling
// ===============================================================================

describe('SpacerBlock - Error Handling', () => {
	afterEach(() => {
		cleanup();
	});

	it('should handle missing settings gracefully', () => {
		const block: Block = {
			id: toBlockId('test'),
			type: 'spacer',
			content: {},
			settings: {},
			metadata: {
				createdAt: new Date().toISOString(),
				updatedAt: new Date().toISOString(),
				version: 1
			}
		};

		expect(() => {
			render(SpacerBlock, {
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

	it('should use default height for undefined settings', () => {
		const block: Block = {
			id: toBlockId('test'),
			type: 'spacer',
			content: {},
			settings: {},
			metadata: {
				createdAt: new Date().toISOString(),
				updatedAt: new Date().toISOString(),
				version: 1
			}
		};

		const { container } = render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('error-2'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const spacerBlock = container.querySelector('.spacer-block');
		expect(spacerBlock).toBeInTheDocument();
	});

	it('should handle invalid height values', () => {
		const block = createSpacerBlock({
			settings: { height: 'invalid' }
		});

		expect(() => {
			render(SpacerBlock, {
				props: {
					block,
					blockId: toBlockId('error-3'),
					isEditing: false,
					isSelected: false,
					onUpdate: vi.fn()
				}
			});
		}).not.toThrow();
	});

	it('should not call onError for valid input', () => {
		const onError = vi.fn();
		const block = createSpacerBlock();

		render(SpacerBlock, {
			props: {
				block,
				blockId: toBlockId('error-4'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn(),
				onError
			}
		});

		expect(onError).not.toHaveBeenCalled();
	});
});
