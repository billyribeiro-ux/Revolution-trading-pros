/**
 * ===============================================================================
 * RiskDisclaimerBlock Component - Unit Tests
 * ===============================================================================
 *
 * @description Comprehensive tests for RiskDisclaimerBlock Svelte component
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Svelte 5 + Testing Library
 *
 * Tests cover:
 * - Warning, danger, and info style rendering
 * - Expandable text functionality
 * - Acknowledgment checkbox
 * - Preset disclaimers
 * - ARIA roles and accessibility
 * - Edit mode controls
 * - Session storage for acknowledgment
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, fireEvent, cleanup } from '@testing-library/svelte';
import RiskDisclaimerBlock from '../trading/RiskDisclaimerBlock.svelte';
import type { Block } from '../types';

// ===============================================================================
// TEST FIXTURES
// ===============================================================================

function createMockBlock(overrides: Partial<Block> = {}): Block {
	return {
		id: 'disclaimer-1',
		type: 'riskDisclaimer',
		content: {
			disclaimerText: 'Trading involves substantial risk of loss.',
			disclaimerStyle: 'warning',
			disclaimerPreset: 'general',
			disclaimerExpandedText: 'The high degree of leverage can work against you.',
			disclaimerRequireAck: false,
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
// MOCK SETUP
// ===============================================================================

const mockSessionStorage: Record<string, string> = {};

beforeEach(() => {
	// Mock sessionStorage
	vi.spyOn(Storage.prototype, 'getItem').mockImplementation(
		(key) => mockSessionStorage[key] || null
	);
	vi.spyOn(Storage.prototype, 'setItem').mockImplementation((key, value) => {
		mockSessionStorage[key] = value;
	});
	vi.spyOn(Storage.prototype, 'removeItem').mockImplementation((key) => {
		delete mockSessionStorage[key];
	});
});

afterEach(() => {
	cleanup();
	vi.clearAllMocks();
	Object.keys(mockSessionStorage).forEach((key) => delete mockSessionStorage[key]);
});

// ===============================================================================
// TEST SUITE: Warning Style
// ===============================================================================

describe('RiskDisclaimerBlock - Warning Style', () => {
	it('should render warning style with correct background', () => {
		const block = createMockBlock({
			content: { disclaimerStyle: 'warning' }
		});

		const { container } = render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const disclaimerBlock = container.querySelector('.risk-disclaimer-block');
		expect(disclaimerBlock).toHaveClass('style-warning');
	});

	it('should render warning icon for warning style', () => {
		const block = createMockBlock({
			content: { disclaimerStyle: 'warning' }
		});

		const { container } = render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const iconContainer = container.querySelector('.disclaimer-icon');
		expect(iconContainer).toBeInTheDocument();
	});

	it('should use role="note" for warning style', () => {
		const block = createMockBlock({
			content: { disclaimerStyle: 'warning' }
		});

		const { container } = render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const disclaimerBlock = container.querySelector('.risk-disclaimer-block');
		expect(disclaimerBlock).toHaveAttribute('role', 'note');
	});
});

// ===============================================================================
// TEST SUITE: Danger Style
// ===============================================================================

describe('RiskDisclaimerBlock - Danger Style', () => {
	it('should render danger style with red background', () => {
		const block = createMockBlock({
			content: { disclaimerStyle: 'danger' }
		});

		const { container } = render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const disclaimerBlock = container.querySelector('.risk-disclaimer-block');
		expect(disclaimerBlock).toHaveClass('style-danger');
	});

	it('should use role="alert" for danger style', () => {
		const block = createMockBlock({
			content: { disclaimerStyle: 'danger' }
		});

		const { container } = render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const disclaimerBlock = container.querySelector('.risk-disclaimer-block');
		expect(disclaimerBlock).toHaveAttribute('role', 'alert');
	});

	it('should render danger icon for danger style', () => {
		const block = createMockBlock({
			content: { disclaimerStyle: 'danger' }
		});

		const { container } = render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const iconContainer = container.querySelector('.disclaimer-icon');
		expect(iconContainer).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Info Style
// ===============================================================================

describe('RiskDisclaimerBlock - Info Style', () => {
	it('should render info style with blue background', () => {
		const block = createMockBlock({
			content: { disclaimerStyle: 'info' }
		});

		const { container } = render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const disclaimerBlock = container.querySelector('.risk-disclaimer-block');
		expect(disclaimerBlock).toHaveClass('style-info');
	});

	it('should use role="note" for info style', () => {
		const block = createMockBlock({
			content: { disclaimerStyle: 'info' }
		});

		const { container } = render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const disclaimerBlock = container.querySelector('.risk-disclaimer-block');
		expect(disclaimerBlock).toHaveAttribute('role', 'note');
	});
});

// ===============================================================================
// TEST SUITE: Expandable Content
// ===============================================================================

describe('RiskDisclaimerBlock - Expandable Content', () => {
	it('should show "Read more" button when expanded text exists', () => {
		const block = createMockBlock({
			content: {
				disclaimerExpandedText: 'Additional detailed information here.'
			}
		});

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Read more')).toBeInTheDocument();
	});

	it('should not show expand button when no expanded text', () => {
		const block = createMockBlock({
			content: {
				disclaimerExpandedText: ''
			}
		});

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.queryByText('Read more')).not.toBeInTheDocument();
	});

	it('should expand content on click', async () => {
		const block = createMockBlock({
			content: {
				disclaimerExpandedText: 'This is the expanded detailed text.'
			}
		});

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const expandButton = screen.getByText('Read more');
		await fireEvent.click(expandButton);

		expect(screen.getByText('Hide details')).toBeInTheDocument();
		expect(screen.getByText('This is the expanded detailed text.')).toBeInTheDocument();
	});

	it('should collapse content on second click', async () => {
		const block = createMockBlock({
			content: {
				disclaimerExpandedText: 'Expanded content text.'
			}
		});

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const expandButton = screen.getByText('Read more');
		await fireEvent.click(expandButton);

		const hideButton = screen.getByText('Hide details');
		await fireEvent.click(hideButton);

		expect(screen.getByText('Read more')).toBeInTheDocument();
	});

	it('should have correct aria-expanded attribute', async () => {
		const block = createMockBlock({
			content: {
				disclaimerExpandedText: 'Expanded content.'
			}
		});

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const expandButton = screen.getByRole('button', { name: /read more/i });
		expect(expandButton).toHaveAttribute('aria-expanded', 'false');

		await fireEvent.click(expandButton);

		const hideButton = screen.getByRole('button', { name: /hide details/i });
		expect(hideButton).toHaveAttribute('aria-expanded', 'true');
	});

	it('should handle keyboard navigation', async () => {
		const block = createMockBlock({
			content: {
				disclaimerExpandedText: 'Expanded text content.'
			}
		});

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const expandButton = screen.getByRole('button', { name: /read more/i });
		await fireEvent.keyDown(expandButton, { key: 'Enter' });

		expect(screen.getByText('Hide details')).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Acknowledgment Checkbox
// ===============================================================================

describe('RiskDisclaimerBlock - Acknowledgment Checkbox', () => {
	it('should show acknowledgment checkbox when required', () => {
		const block = createMockBlock({
			content: { disclaimerRequireAck: true }
		});

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('I understand and acknowledge the risks involved')).toBeInTheDocument();
	});

	it('should not show acknowledgment checkbox when not required', () => {
		const block = createMockBlock({
			content: { disclaimerRequireAck: false }
		});

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(
			screen.queryByText('I understand and acknowledge the risks involved')
		).not.toBeInTheDocument();
	});

	it('should toggle acknowledgment state on click', async () => {
		const block = createMockBlock({
			content: { disclaimerRequireAck: true }
		});

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const checkbox = screen.getByRole('checkbox');
		expect(checkbox).toHaveAttribute('aria-checked', 'false');

		await fireEvent.click(checkbox);

		expect(checkbox).toHaveAttribute('aria-checked', 'true');
		expect(screen.getByText('Risk acknowledged')).toBeInTheDocument();
	});

	it('should save acknowledgment to sessionStorage', async () => {
		const block = createMockBlock({
			content: { disclaimerRequireAck: true }
		});

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const checkbox = screen.getByRole('checkbox');
		await fireEvent.click(checkbox);

		expect(sessionStorage.setItem).toHaveBeenCalledWith('risk-ack-disclaimer-1', 'true');
	});

	it('should not show checkbox in edit mode', () => {
		const block = createMockBlock({
			content: { disclaimerRequireAck: true }
		});

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		// Should show preview text instead of actual checkbox
		expect(screen.getByText(/checkbox shown in preview/i)).toBeInTheDocument();
	});

	it('should show confirmation message when acknowledged', async () => {
		const block = createMockBlock({
			content: { disclaimerRequireAck: true }
		});

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const checkbox = screen.getByRole('checkbox');
		await fireEvent.click(checkbox);

		expect(screen.getByText('Risk acknowledged')).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Preset Disclaimers
// ===============================================================================

describe('RiskDisclaimerBlock - Preset Disclaimers', () => {
	it('should show general preset text', () => {
		const block = createMockBlock({
			content: {
				disclaimerPreset: 'general',
				disclaimerText: '' // Empty to use preset
			}
		});

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText(/trading involves substantial risk of loss/i)).toBeInTheDocument();
	});

	it('should show trading preset text', () => {
		const block = createMockBlock({
			content: {
				disclaimerPreset: 'trading',
				disclaimerText: ''
			}
		});

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(
			screen.getByText(/past performance is not indicative of future results/i)
		).toBeInTheDocument();
	});

	it('should show investment preset text', () => {
		const block = createMockBlock({
			content: {
				disclaimerPreset: 'investment',
				disclaimerText: ''
			}
		});

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText(/does not constitute financial advice/i)).toBeInTheDocument();
	});

	it('should use custom text when provided', () => {
		const block = createMockBlock({
			content: {
				disclaimerPreset: 'custom',
				disclaimerText: 'This is my custom disclaimer text.'
			}
		});

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('This is my custom disclaimer text.')).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Edit Mode
// ===============================================================================

describe('RiskDisclaimerBlock - Edit Mode', () => {
	it('should show settings panel when editing and selected', () => {
		const block = createMockBlock();

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Disclaimer Settings')).toBeInTheDocument();
	});

	it('should show preset selector', () => {
		const block = createMockBlock();

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Preset:')).toBeInTheDocument();
		expect(screen.getByText('General Risk')).toBeInTheDocument();
	});

	it('should show style selector', () => {
		const block = createMockBlock();

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Style:')).toBeInTheDocument();
	});

	it('should show acknowledgment toggle', () => {
		const block = createMockBlock();

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Require acknowledgment')).toBeInTheDocument();
	});

	it('should show text areas for custom text', () => {
		const block = createMockBlock();

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Main Warning Text:')).toBeInTheDocument();
		expect(screen.getByText('Expanded Details (optional):')).toBeInTheDocument();
	});

	it('should call onUpdate when preset changes', async () => {
		const block = createMockBlock();
		const onUpdate = vi.fn();

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const presetSelect = screen.getByDisplayValue('General Risk');
		await fireEvent.change(presetSelect, { target: { value: 'trading' } });

		expect(onUpdate).toHaveBeenCalled();
	});

	it('should show style badge in edit mode', () => {
		const block = createMockBlock({
			content: { disclaimerStyle: 'danger' }
		});

		const { container } = render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: true,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const badge = container.querySelector('.style-badge');
		expect(badge).toBeInTheDocument();
		expect(badge?.textContent).toContain('Danger');
	});
});

// ===============================================================================
// TEST SUITE: Accessibility
// ===============================================================================

describe('RiskDisclaimerBlock - Accessibility', () => {
	it('should have aria-labelledby pointing to content', () => {
		const block = createMockBlock();

		const { container } = render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const disclaimerBlock = container.querySelector('.risk-disclaimer-block');
		expect(disclaimerBlock).toHaveAttribute('aria-labelledby');
	});

	it('should have hidden icon with aria-hidden', () => {
		const block = createMockBlock();

		const { container } = render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const iconContainer = container.querySelector('.disclaimer-icon');
		expect(iconContainer).toHaveAttribute('aria-hidden', 'true');
	});

	it('should have aria-controls on expand button', () => {
		const block = createMockBlock({
			content: {
				disclaimerExpandedText: 'Some expanded text.'
			}
		});

		render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const expandButton = screen.getByRole('button', { name: /read more/i });
		expect(expandButton).toHaveAttribute('aria-controls');
	});

	it('should have unique IDs for content and expanded sections', () => {
		const block = createMockBlock({
			content: {
				disclaimerExpandedText: 'Expanded content.'
			}
		});

		const { container } = render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-test',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const contentId = container.querySelector('#disclaimer-content-disclaimer-test');
		const expandedId = container.querySelector('#disclaimer-expanded-disclaimer-test');

		expect(contentId).toBeInTheDocument();
		expect(expandedId).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: ARIA Roles Based on Style
// ===============================================================================

describe('RiskDisclaimerBlock - ARIA Role Based on Style', () => {
	it('should use role="alert" for danger style', () => {
		const block = createMockBlock({
			content: { disclaimerStyle: 'danger' }
		});

		const { container } = render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const disclaimerBlock = container.querySelector('.risk-disclaimer-block');
		expect(disclaimerBlock).toHaveAttribute('role', 'alert');
	});

	it('should use role="note" for warning style', () => {
		const block = createMockBlock({
			content: { disclaimerStyle: 'warning' }
		});

		const { container } = render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const disclaimerBlock = container.querySelector('.risk-disclaimer-block');
		expect(disclaimerBlock).toHaveAttribute('role', 'note');
	});

	it('should use role="note" for info style', () => {
		const block = createMockBlock({
			content: { disclaimerStyle: 'info' }
		});

		const { container } = render(RiskDisclaimerBlock, {
			props: {
				block,
				blockId: 'disclaimer-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const disclaimerBlock = container.querySelector('.risk-disclaimer-block');
		expect(disclaimerBlock).toHaveAttribute('role', 'note');
	});
});
