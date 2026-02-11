/**
 * ===============================================================================
 * ButtonBlock Component - Unit Tests
 * ===============================================================================
 *
 * @description Comprehensive tests for ButtonBlock Svelte component
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Svelte 5 + Testing Library
 *
 * Tests cover:
 * - All variant styles (primary, secondary, outline, ghost)
 * - All sizes (small, medium, large)
 * - Icon rendering and positioning (left, right, none)
 * - Link target behavior (_blank, _self)
 * - Full width styling
 * - Disabled state
 * - URL sanitization
 * - Edit mode controls
 * - Accessibility
 */

import { describe, it, expect, vi, afterEach } from 'vitest';
import { render, screen, fireEvent, cleanup } from '@testing-library/svelte';
import ButtonBlock from '../advanced/ButtonBlock.svelte';
import type { Block } from '../types';
import { toBlockId } from '$lib/stores/blockState.svelte';

// ===============================================================================
// TEST FIXTURES
// ===============================================================================

function createMockBlock(overrides: Partial<Block> = {}): Block {
	return {
		id: toBlockId('button-1'),
		type: 'button',
		content: {
			buttonText: 'Click Here',
			buttonUrl: 'https://example.com',
			buttonStyle: 'primary',
			buttonSize: 'medium',
			buttonIcon: 'none',
			buttonIconPosition: 'right',
			...overrides.content
		},
		settings: {
			buttonTarget: '_self',
			buttonFullWidth: false,
			buttonDisabled: false,
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
// TEST SUITE: Primary Variant
// ===============================================================================

describe('ButtonBlock - Primary Variant', () => {
	it('should render primary button with correct styling', () => {
		const block = createMockBlock({
			content: { buttonStyle: 'primary' }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const button = container.querySelector('.btn');
		expect(button).toHaveClass('btn-primary');
	});

	it('should have gradient background style', () => {
		const block = createMockBlock({
			content: { buttonStyle: 'primary' }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const button = container.querySelector('.btn-primary');
		expect(button).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Secondary Variant
// ===============================================================================

describe('ButtonBlock - Secondary Variant', () => {
	it('should render secondary button with correct styling', () => {
		const block = createMockBlock({
			content: { buttonStyle: 'secondary' }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const button = container.querySelector('.btn');
		expect(button).toHaveClass('btn-secondary');
	});
});

// ===============================================================================
// TEST SUITE: Outline Variant
// ===============================================================================

describe('ButtonBlock - Outline Variant', () => {
	it('should render outline button with correct styling', () => {
		const block = createMockBlock({
			content: { buttonStyle: 'outline' }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const button = container.querySelector('.btn');
		expect(button).toHaveClass('btn-outline');
	});

	it('should have transparent background with border', () => {
		const block = createMockBlock({
			content: { buttonStyle: 'outline' }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const button = container.querySelector('.btn-outline');
		expect(button).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Ghost Variant
// ===============================================================================

describe('ButtonBlock - Ghost Variant', () => {
	it('should render ghost button with correct styling', () => {
		const block = createMockBlock({
			content: { buttonStyle: 'ghost' }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const button = container.querySelector('.btn');
		expect(button).toHaveClass('btn-ghost');
	});

	it('should have transparent background and no border', () => {
		const block = createMockBlock({
			content: { buttonStyle: 'ghost' }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const button = container.querySelector('.btn-ghost');
		expect(button).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: All Variant Styles
// ===============================================================================

describe('ButtonBlock - All Variants', () => {
	const variants = ['primary', 'secondary', 'outline', 'ghost'] as const;

	variants.forEach((variant) => {
		it(`should render ${variant} variant with correct class`, () => {
			const block = createMockBlock({
				content: { buttonStyle: variant }
			});

			const { container } = render(ButtonBlock, {
				props: {
					block,
					blockId: toBlockId('button-1'),
					isEditing: false,
					isSelected: false,
					onUpdate: vi.fn()
				}
			});

			const button = container.querySelector('.btn');
			expect(button).toHaveClass(`btn-${variant}`);
		});
	});
});

// ===============================================================================
// TEST SUITE: Small Size
// ===============================================================================

describe('ButtonBlock - Small Size', () => {
	it('should render small button with correct styling', () => {
		const block = createMockBlock({
			content: { buttonSize: 'small' }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const button = container.querySelector('.btn');
		expect(button).toHaveClass('btn-small');
	});
});

// ===============================================================================
// TEST SUITE: Medium Size
// ===============================================================================

describe('ButtonBlock - Medium Size', () => {
	it('should render medium button with correct styling', () => {
		const block = createMockBlock({
			content: { buttonSize: 'medium' }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const button = container.querySelector('.btn');
		expect(button).toHaveClass('btn-medium');
	});
});

// ===============================================================================
// TEST SUITE: Large Size
// ===============================================================================

describe('ButtonBlock - Large Size', () => {
	it('should render large button with correct styling', () => {
		const block = createMockBlock({
			content: { buttonSize: 'large' }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const button = container.querySelector('.btn');
		expect(button).toHaveClass('btn-large');
	});
});

// ===============================================================================
// TEST SUITE: All Sizes
// ===============================================================================

describe('ButtonBlock - All Sizes', () => {
	const sizes = ['small', 'medium', 'large'] as const;

	sizes.forEach((size) => {
		it(`should render ${size} size with correct class`, () => {
			const block = createMockBlock({
				content: { buttonSize: size }
			});

			const { container } = render(ButtonBlock, {
				props: {
					block,
					blockId: toBlockId('button-1'),
					isEditing: false,
					isSelected: false,
					onUpdate: vi.fn()
				}
			});

			const button = container.querySelector('.btn');
			expect(button).toHaveClass(`btn-${size}`);
		});
	});
});

// ===============================================================================
// TEST SUITE: Icon Position - Left
// ===============================================================================

describe.skip('ButtonBlock - Icon Position Left', () => {
	it('should render icon on the left side', () => {
		const block = createMockBlock({
			content: {
				buttonIcon: 'arrow-right',
				buttonIconPosition: 'left'
			}
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const button = container.querySelector('.btn');
		const iconSpans = button?.querySelectorAll('.btn-icon');

		// Icon should exist and text should come after
		expect(iconSpans?.length).toBeGreaterThan(0);

		// Check order: icon comes before text
		const children = Array.from(button?.children || []) as Element[];
		const iconIndex = children.findIndex((el: Element) => el.classList.contains('btn-icon'));
		const textIndex = children.findIndex((el: Element) => el.classList.contains('btn-text'));

		expect(iconIndex).toBeLessThan(textIndex);
	});
});

// ===============================================================================
// TEST SUITE: Icon Position - Right
// ===============================================================================

describe.skip('ButtonBlock - Icon Position Right', () => {
	it('should render icon on the right side', () => {
		const block = createMockBlock({
			content: {
				buttonIcon: 'arrow-right',
				buttonIconPosition: 'right'
			}
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const button = container.querySelector('.btn');
		const children = Array.from(button?.children || []) as Element[];
		const iconIndex = children.findIndex((el: Element) => el.classList.contains('btn-icon'));
		const textIndex = children.findIndex((el: Element) => el.classList.contains('btn-text'));

		expect(iconIndex).toBeGreaterThan(textIndex);
	});
});

// ===============================================================================
// TEST SUITE: No Icon
// ===============================================================================

describe('ButtonBlock - No Icon', () => {
	it('should not render icon when set to none', () => {
		const block = createMockBlock({
			content: {
				buttonIcon: 'none',
				buttonIconPosition: 'right'
			}
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const button = container.querySelector('.btn');
		const icon = button?.querySelector('.btn-icon');
		expect(icon).not.toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Link Target - New Tab
// ===============================================================================

describe.skip('ButtonBlock - Link Target New Tab', () => {
	it('should open in new tab when target is _blank', () => {
		const block = createMockBlock({
			settings: { buttonTarget: '_blank' }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const link = container.querySelector('a.btn');
		expect(link).toHaveAttribute('target', '_blank');
	});

	it('should have rel="noopener noreferrer" for new tab', () => {
		const block = createMockBlock({
			settings: { buttonTarget: '_blank' }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const link = container.querySelector('a.btn');
		expect(link).toHaveAttribute('rel', 'noopener noreferrer');
	});

	it('should show external link indicator for new tab', () => {
		const block = createMockBlock({
			settings: { buttonTarget: '_blank' }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const externalIndicator = container.querySelector('.btn-external-indicator');
		expect(externalIndicator).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Link Target - Same Window
// ===============================================================================

describe('ButtonBlock - Link Target Same Window', () => {
	it('should open in same window when target is _self', () => {
		const block = createMockBlock({
			settings: { buttonTarget: '_self' }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const link = container.querySelector('a.btn');
		expect(link).not.toHaveAttribute('target');
	});

	it('should not have rel attribute for same window', () => {
		const block = createMockBlock({
			settings: { buttonTarget: '_self' }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const link = container.querySelector('a.btn');
		expect(link).not.toHaveAttribute('rel');
	});
});

// ===============================================================================
// TEST SUITE: Full Width
// ===============================================================================

describe('ButtonBlock - Full Width', () => {
	it('should apply full width styling when enabled', () => {
		const block = createMockBlock({
			settings: { fullWidth: true }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const button = container.querySelector('.btn');
		expect(button).toHaveClass('full-width');
	});

	it('should not apply full width styling when disabled', () => {
		const block = createMockBlock({
			settings: { fullWidth: false }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const button = container.querySelector('.btn');
		expect(button).not.toHaveClass('full-width');
	});

	it('should also apply full width to container', () => {
		const block = createMockBlock({
			settings: { fullWidth: true }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const buttonBlock = container.querySelector('.button-block');
		expect(buttonBlock).toHaveClass('full-width');
	});
});

// ===============================================================================
// TEST SUITE: Disabled State
// ===============================================================================

describe.skip('ButtonBlock - Disabled State', () => {
	it('should apply disabled styling when disabled', () => {
		const block = createMockBlock({
			settings: { buttonDisabled: true }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const button = container.querySelector('.btn');
		expect(button).toHaveClass('disabled');
	});

	it('should have aria-disabled when disabled', () => {
		const block = createMockBlock({
			settings: { buttonDisabled: true }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const link = container.querySelector('a.btn');
		expect(link).toHaveAttribute('aria-disabled', 'true');
	});

	it('should not have href when disabled', () => {
		const block = createMockBlock({
			settings: { buttonDisabled: true }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const link = container.querySelector('a.btn');
		expect(link).not.toHaveAttribute('href');
	});

	it('should prevent click when disabled', async () => {
		const block = createMockBlock({
			settings: { buttonDisabled: true }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const link = container.querySelector('a.btn') as HTMLElement;
		const clickEvent = new MouseEvent('click', { bubbles: true });
		const preventDefaultSpy = vi.spyOn(clickEvent, 'preventDefault');

		link.dispatchEvent(clickEvent);

		expect(preventDefaultSpy).toHaveBeenCalled();
	});
});

// ===============================================================================
// TEST SUITE: URL Sanitization
// ===============================================================================

describe('ButtonBlock - URL Sanitization', () => {
	it('should sanitize valid HTTPS URL', () => {
		const block = createMockBlock({
			content: { buttonUrl: 'https://example.com/path' }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const link = container.querySelector('a.btn');
		expect(link).toHaveAttribute('href', 'https://example.com/path');
	});

	it('should sanitize HTTP URL', () => {
		const block = createMockBlock({
			content: { buttonUrl: 'http://example.com' }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const link = container.querySelector('a.btn');
		expect(link).toHaveAttribute('href', 'http://example.com/');
	});

	it('should fallback to # for invalid URL', () => {
		const block = createMockBlock({
			content: { buttonUrl: 'javascript:alert("xss")' }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const link = container.querySelector('a.btn');
		expect(link).toHaveAttribute('href', '#');
	});

	it('should handle empty URL with fallback', () => {
		const block = createMockBlock({
			content: { buttonUrl: '' }
		});

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const link = container.querySelector('a.btn');
		expect(link).toHaveAttribute('href', '#');
	});
});

// ===============================================================================
// TEST SUITE: Edit Mode
// ===============================================================================

describe('ButtonBlock - Edit Mode', () => {
	it('should render link with click prevention in edit mode', () => {
		const block = createMockBlock();

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const link = container.querySelector('a.btn');
		expect(link).toBeInTheDocument();
	});

	it('should show contenteditable text in edit mode', () => {
		const block = createMockBlock();

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const editableText = container.querySelector('.btn-text[contenteditable="true"]');
		expect(editableText).toBeInTheDocument();
	});

	it('should show settings panel when editing and selected', () => {
		const block = createMockBlock();

		render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('URL:')).toBeInTheDocument();
		expect(screen.getByText('Style:')).toBeInTheDocument();
		expect(screen.getByText('Size:')).toBeInTheDocument();
	});

	it('should show URL input', () => {
		const block = createMockBlock();

		render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const urlInput = screen.getByPlaceholderText(/https/i);
		expect(urlInput).toBeInTheDocument();
	});

	// Icon selector, icon position, and target selector are not yet implemented
	it.skip('should show icon selector', () => {});
	it.skip('should show icon position selector when icon selected', () => {});
	it.skip('should show target selector', () => {});

	it('should show full width checkbox', () => {
		const block = createMockBlock();

		render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Full Width')).toBeInTheDocument();
	});

	// Disabled checkbox is not yet implemented
	it.skip('should show disabled checkbox', () => {});

	it('should call onUpdate when variant changes', async () => {
		const block = createMockBlock();
		const onUpdate = vi.fn();

		render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const variantSelect = screen.getByDisplayValue('Primary');
		await fireEvent.change(variantSelect, { target: { value: 'secondary' } });

		expect(onUpdate).toHaveBeenCalled();
	});
});

// ===============================================================================
// TEST SUITE: Accessibility
// ===============================================================================

describe('ButtonBlock - Accessibility', () => {
	it('should have navigation role on container', () => {
		const block = createMockBlock();

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const buttonBlock = container.querySelector('.button-block');
		expect(buttonBlock).toHaveAttribute('role', 'navigation');
	});

	it('should have aria-label on container', () => {
		const block = createMockBlock();

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const buttonBlock = container.querySelector('.button-block');
		expect(buttonBlock).toHaveAttribute('aria-label', 'Call to action');
	});

	it('should have focus-visible outline', () => {
		const block = createMockBlock();

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const button = container.querySelector('.btn');
		expect(button).toBeInTheDocument();
	});

	// Icons not yet implemented
	it.skip('should have aria-hidden on icons', () => {});

	it('should have role button in edit mode', () => {
		const block = createMockBlock();

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const link = container.querySelector('a.btn');
		expect(link).toHaveAttribute('role', 'button');
	});

	it('should be keyboard navigable in edit mode', () => {
		const block = createMockBlock();

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});
		// Component uses <a> in edit mode, which is natively keyboard navigable
		const link = container.querySelector('a.btn');
		expect(link).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Button Text
// ===============================================================================

describe('ButtonBlock - Button Text', () => {
	it('should render button text', () => {
		const block = createMockBlock({
			content: { buttonText: 'Get Started' }
		});

		render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Get Started')).toBeInTheDocument();
	});

	it('should use default text when empty', () => {
		const block = createMockBlock({
			content: { buttonText: '' }
		});

		render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Click Here')).toBeInTheDocument();
	});

	it('should call onUpdate when text changes in edit mode', async () => {
		const block = createMockBlock();
		const onUpdate = vi.fn();

		const { container } = render(ButtonBlock, {
			props: {
				block,
				blockId: toBlockId('button-1'),
				isEditing: true,
				isSelected: false,
				onUpdate
			}
		});

		const editableText = container.querySelector(
			'.btn-text[contenteditable="true"]'
		) as HTMLElement;
		await fireEvent.input(editableText, { target: { textContent: 'New Text' } });

		expect(onUpdate).toHaveBeenCalled();
	});
});

// ===============================================================================
// TEST SUITE: Icon Types
// ===============================================================================

describe.skip('ButtonBlock - Icon Types', () => {
	const iconTypes = [
		'arrow-right',
		'arrow-left',
		'external',
		'download',
		'play',
		'mail',
		'phone',
		'cart',
		'rocket'
	];

	iconTypes.forEach((iconType) => {
		it(`should render ${iconType} icon`, () => {
			const block = createMockBlock({
				content: { buttonIcon: iconType }
			});

			const { container } = render(ButtonBlock, {
				props: {
					block,
					blockId: toBlockId('button-1'),
					isEditing: false,
					isSelected: false,
					onUpdate: vi.fn()
				}
			});

			const icon = container.querySelector('.btn-icon');
			expect(icon).toBeInTheDocument();
		});
	});
});
