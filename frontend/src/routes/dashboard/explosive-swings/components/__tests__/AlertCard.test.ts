/**
 * ===============================================================================
 * AlertCard Component - Unit Tests
 * ===============================================================================
 *
 * @description Comprehensive tests for AlertCard Svelte component
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Svelte 5 + Testing Library
 *
 * Tests cover:
 * - Rendering with different alert types (ENTRY, EXIT, UPDATE)
 * - TOS string display and copy functionality
 * - New alert indicator
 * - Admin actions visibility
 * - Notes expansion
 * - Price information display
 * - Accessibility (ARIA attributes)
 */

import { describe, it, expect, vi, afterEach } from 'vitest';
import { render, screen, fireEvent, cleanup } from '@testing-library/svelte';
import AlertCard from '../AlertCard.svelte';
import type { Alert } from '../../types';

// ===============================================================================
// TEST FIXTURES
// ===============================================================================

function createMockAlert(overrides: Partial<Alert> = {}): Alert {
	return {
		id: 1,
		type: 'ENTRY',
		ticker: 'NVDA',
		title: 'Opening NVDA Swing Position',
		time: 'Today at 10:32 AM',
		message: 'Entering NVDA at $142.50. First target $148, stop at $136.',
		isNew: false,
		notes: 'Strong momentum into earnings. RSI at 62.',
		tosString: undefined,
		entryPrice: undefined,
		targetPrice: undefined,
		stopPrice: undefined,
		resultPercent: undefined,
		...overrides
	};
}

// ===============================================================================
// TEST SUITE: Basic Rendering
// ===============================================================================

describe('AlertCard - Basic Rendering', () => {
	afterEach(() => {
		cleanup();
	});

	it('should render alert with correct ticker', () => {
		const alert = createMockAlert({ ticker: 'TSLA' });
		render(AlertCard, { props: { alert } });

		expect(screen.getByText('TSLA')).toBeInTheDocument();
	});

	it('should render alert title', () => {
		const alert = createMockAlert({ title: 'Test Alert Title' });
		render(AlertCard, { props: { alert } });

		expect(screen.getByText('Test Alert Title')).toBeInTheDocument();
	});

	it('should render alert message', () => {
		const alert = createMockAlert({ message: 'This is the alert message' });
		render(AlertCard, { props: { alert } });

		expect(screen.getByText('This is the alert message')).toBeInTheDocument();
	});

	it('should render alert timestamp', () => {
		const alert = createMockAlert({ time: 'Yesterday at 3:45 PM' });
		render(AlertCard, { props: { alert } });

		expect(screen.getByText('Yesterday at 3:45 PM')).toBeInTheDocument();
	});

	it('should have correct aria-label for accessibility', () => {
		const alert = createMockAlert({ ticker: 'NVDA', type: 'ENTRY' });
		render(AlertCard, { props: { alert } });

		const article = screen.getByRole('article');
		expect(article).toHaveAttribute('aria-label', 'NVDA ENTRY alert');
	});
});

// ===============================================================================
// TEST SUITE: Alert Types
// ===============================================================================

describe('AlertCard - Alert Types', () => {
	afterEach(() => {
		cleanup();
	});

	describe('ENTRY alerts', () => {
		it('should display ENTRY badge', () => {
			const alert = createMockAlert({ type: 'ENTRY' });
			render(AlertCard, { props: { alert } });

			expect(screen.getByText('ENTRY')).toBeInTheDocument();
		});

		it('should apply entry styling (teal border)', () => {
			const alert = createMockAlert({ type: 'ENTRY' });
			const { container } = render(AlertCard, { props: { alert } });

			const article = container.querySelector('article');
			expect(article?.classList.toString()).toContain('border-l-teal');
		});

		it('should show price info when entry/target/stop prices provided', () => {
			const alert = createMockAlert({
				type: 'ENTRY',
				entryPrice: 142.5,
				targetPrice: 148.0,
				stopPrice: 136.0
			});
			render(AlertCard, { props: { alert } });

			expect(screen.getByText('Entry')).toBeInTheDocument();
			expect(screen.getByText('Target')).toBeInTheDocument();
			expect(screen.getByText('Stop')).toBeInTheDocument();
		});

		it('should show "View Trade Plan" button when callback provided', () => {
			const alert = createMockAlert({ type: 'ENTRY' });
			const onViewTradePlan = vi.fn();
			render(AlertCard, { props: { alert, onViewTradePlan } });

			expect(screen.getByText('View Trade Plan')).toBeInTheDocument();
		});
	});

	describe('UPDATE alerts', () => {
		it('should display UPDATE badge', () => {
			const alert = createMockAlert({ type: 'UPDATE' });
			render(AlertCard, { props: { alert } });

			expect(screen.getByText('UPDATE')).toBeInTheDocument();
		});

		it('should apply update styling (amber border)', () => {
			const alert = createMockAlert({ type: 'UPDATE' });
			const { container } = render(AlertCard, { props: { alert } });

			const article = container.querySelector('article');
			expect(article?.classList.toString()).toContain('border-l-amber');
		});
	});

	describe('EXIT alerts', () => {
		it('should display EXIT badge', () => {
			const alert = createMockAlert({ type: 'EXIT' });
			render(AlertCard, { props: { alert } });

			expect(screen.getByText('EXIT')).toBeInTheDocument();
		});

		it('should show positive result with green styling', () => {
			const alert = createMockAlert({
				type: 'EXIT',
				resultPercent: 8.2
			});
			const { container } = render(AlertCard, { props: { alert } });

			expect(screen.getByText('+8.2%')).toBeInTheDocument();
			expect(screen.getByText('Profit')).toBeInTheDocument();

			const article = container.querySelector('article');
			expect(article?.classList.toString()).toContain('border-l-emerald');
		});

		it('should show negative result with red styling', () => {
			const alert = createMockAlert({
				type: 'EXIT',
				resultPercent: -2.1
			});
			const { container } = render(AlertCard, { props: { alert } });

			expect(screen.getByText('-2.1%')).toBeInTheDocument();
			expect(screen.getByText('Loss')).toBeInTheDocument();

			const article = container.querySelector('article');
			expect(article?.classList.toString()).toContain('border-l-red');
		});

		it('should handle zero percent result as profit', () => {
			const alert = createMockAlert({
				type: 'EXIT',
				resultPercent: 0
			});
			render(AlertCard, { props: { alert } });

			expect(screen.getByText('+0.0%')).toBeInTheDocument();
			expect(screen.getByText('Profit')).toBeInTheDocument();
		});
	});
});

// ===============================================================================
// TEST SUITE: New Alert Indicator
// ===============================================================================

describe('AlertCard - New Alert Indicator', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show NEW badge when isNew is true', () => {
		const alert = createMockAlert({ isNew: true });
		render(AlertCard, { props: { alert } });

		expect(screen.getByText('NEW')).toBeInTheDocument();
	});

	it('should not show NEW badge when isNew is false', () => {
		const alert = createMockAlert({ isNew: false });
		render(AlertCard, { props: { alert } });

		expect(screen.queryByText('NEW')).not.toBeInTheDocument();
	});

	it('should apply special styling for new alerts', () => {
		const alert = createMockAlert({ isNew: true });
		const { container } = render(AlertCard, { props: { alert } });

		const article = container.querySelector('article');
		expect(article?.classList.toString()).toContain('is-new');
	});

	it('should include pulse dot in NEW badge', () => {
		const alert = createMockAlert({ isNew: true });
		const { container } = render(AlertCard, { props: { alert } });

		const pulseDot = container.querySelector('.pulse-dot');
		expect(pulseDot).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: TOS String Display
// ===============================================================================

describe('AlertCard - TOS String Display', () => {
	afterEach(() => {
		cleanup();
	});

	it('should display TOS string when provided', () => {
		const alert = createMockAlert({ tosString: 'TOS:NVDA:145C:01/24' });
		render(AlertCard, { props: { alert } });

		expect(screen.getByText('TOS:NVDA:145C:01/24')).toBeInTheDocument();
	});

	it('should not show TOS container when tosString is undefined', () => {
		const alert = createMockAlert({ tosString: undefined });
		const { container } = render(AlertCard, { props: { alert } });

		const tosContainer = container.querySelector('.tos-container');
		expect(tosContainer).not.toBeInTheDocument();
	});

	it('should show copy button for TOS string', () => {
		const alert = createMockAlert({ tosString: 'TOS:NVDA:145C:01/24' });
		render(AlertCard, { props: { alert } });

		const tosContainer = screen.getByText('TOS:NVDA:145C:01/24').closest('.tos-container');
		const copyButton = tosContainer?.querySelector('button');
		expect(copyButton).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Notes Expansion
// ===============================================================================

describe('AlertCard - Notes Expansion', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show notes toggle button', () => {
		const alert = createMockAlert({ notes: 'Some trade notes' });
		render(AlertCard, { props: { alert } });

		expect(screen.getByText('Notes')).toBeInTheDocument();
	});

	it('should not show expanded notes by default', () => {
		const alert = createMockAlert({ notes: 'Some trade notes' });
		render(AlertCard, { props: { alert, isNotesExpanded: false } });

		// Notes panel should not be visible
		const notesPanel = screen.queryByText('Trade Notes');
		expect(notesPanel).not.toBeInTheDocument();
	});

	it('should show notes panel when isNotesExpanded is true', () => {
		const alert = createMockAlert({ notes: 'Some detailed trade notes here' });
		render(AlertCard, { props: { alert, isNotesExpanded: true } });

		expect(screen.getByText('Trade Notes')).toBeInTheDocument();
		expect(screen.getByText('Some detailed trade notes here')).toBeInTheDocument();
	});

	it('should call onToggleNotes when toggle button clicked', async () => {
		const alert = createMockAlert({ notes: 'Notes' });
		const onToggleNotes = vi.fn();
		render(AlertCard, { props: { alert, onToggleNotes } });

		const toggleButton = screen.getByRole('button', { name: /notes/i });
		await fireEvent.click(toggleButton);

		expect(onToggleNotes).toHaveBeenCalledWith(alert.id);
	});

	it('should have correct aria-expanded attribute', () => {
		const alert = createMockAlert({ notes: 'Notes' });
		render(AlertCard, { props: { alert, isNotesExpanded: true } });

		const toggleButton = screen.getByRole('button', { name: /notes/i });
		expect(toggleButton).toHaveAttribute('aria-expanded', 'true');
	});
});

// ===============================================================================
// TEST SUITE: Copy Functionality
// ===============================================================================

describe('AlertCard - Copy Functionality', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show copy button', () => {
		const alert = createMockAlert();
		render(AlertCard, { props: { alert } });

		expect(screen.getByText('Copy')).toBeInTheDocument();
	});

	it('should call onCopy when copy button clicked', async () => {
		const alert = createMockAlert();
		const onCopy = vi.fn();
		render(AlertCard, { props: { alert, onCopy } });

		const copyButton = screen.getByText('Copy');
		await fireEvent.click(copyButton);

		expect(onCopy).toHaveBeenCalledWith(alert);
	});

	it('should show "Copied!" when isCopied is true', () => {
		const alert = createMockAlert();
		render(AlertCard, { props: { alert, isCopied: true } });

		expect(screen.getByText('Copied!')).toBeInTheDocument();
	});

	it('should apply copied styling when isCopied is true', () => {
		const alert = createMockAlert();
		const { container } = render(AlertCard, { props: { alert, isCopied: true } });

		const copyButton = container.querySelector('.copy-btn.copied');
		expect(copyButton).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Admin Actions
// ===============================================================================

describe('AlertCard - Admin Actions', () => {
	afterEach(() => {
		cleanup();
	});

	it('should not show admin buttons when isAdmin is false', () => {
		const alert = createMockAlert();
		const onEdit = vi.fn();
		const onDelete = vi.fn();
		render(AlertCard, { props: { alert, isAdmin: false, onEdit, onDelete } });

		expect(screen.queryByText('Edit')).not.toBeInTheDocument();
		expect(screen.queryByText('Delete')).not.toBeInTheDocument();
	});

	it('should show admin buttons when isAdmin is true', () => {
		const alert = createMockAlert();
		const onEdit = vi.fn();
		const onDelete = vi.fn();
		render(AlertCard, { props: { alert, isAdmin: true, onEdit, onDelete } });

		expect(screen.getByText('Edit')).toBeInTheDocument();
		expect(screen.getByText('Delete')).toBeInTheDocument();
	});

	it('should call onEdit when Edit button clicked', async () => {
		const alert = createMockAlert();
		const onEdit = vi.fn();
		render(AlertCard, { props: { alert, isAdmin: true, onEdit } });

		const editButton = screen.getByText('Edit');
		await fireEvent.click(editButton);

		expect(onEdit).toHaveBeenCalledWith(alert);
	});

	it('should call onDelete when Delete button clicked', async () => {
		const alert = createMockAlert();
		const onDelete = vi.fn();
		render(AlertCard, { props: { alert, isAdmin: true, onDelete } });

		const deleteButton = screen.getByText('Delete');
		await fireEvent.click(deleteButton);

		expect(onDelete).toHaveBeenCalledWith(alert.id);
	});

	it('should not show Edit button if onEdit not provided', () => {
		const alert = createMockAlert();
		render(AlertCard, { props: { alert, isAdmin: true } });

		expect(screen.queryByText('Edit')).not.toBeInTheDocument();
	});

	it('should not show Delete button if onDelete not provided', () => {
		const alert = createMockAlert();
		render(AlertCard, { props: { alert, isAdmin: true } });

		expect(screen.queryByText('Delete')).not.toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Price Information Display
// ===============================================================================

describe('AlertCard - Price Information Display', () => {
	afterEach(() => {
		cleanup();
	});

	it('should format entry price correctly', () => {
		const alert = createMockAlert({
			type: 'ENTRY',
			entryPrice: 142.5
		});
		render(AlertCard, { props: { alert } });

		expect(screen.getByText('$142.50')).toBeInTheDocument();
	});

	it('should format target price with target styling', () => {
		const alert = createMockAlert({
			type: 'ENTRY',
			targetPrice: 148.0
		});
		const { container } = render(AlertCard, { props: { alert } });

		const targetValue = container.querySelector('.price-value.target');
		expect(targetValue).toBeInTheDocument();
	});

	it('should format stop price with stop styling', () => {
		const alert = createMockAlert({
			type: 'ENTRY',
			stopPrice: 136.0
		});
		const { container } = render(AlertCard, { props: { alert } });

		const stopValue = container.querySelector('.price-value.stop');
		expect(stopValue).toBeInTheDocument();
	});

	it('should not show price info for non-ENTRY alerts', () => {
		const alert = createMockAlert({
			type: 'UPDATE',
			entryPrice: 142.5,
			targetPrice: 148.0,
			stopPrice: 136.0
		});
		const { container } = render(AlertCard, { props: { alert } });

		const priceInfo = container.querySelector('.price-info');
		expect(priceInfo).not.toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: View Trade Plan Button
// ===============================================================================

describe('AlertCard - View Trade Plan Button', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show "View Trade Plan" for ENTRY alerts with callback', () => {
		const alert = createMockAlert({ type: 'ENTRY' });
		const onViewTradePlan = vi.fn();
		render(AlertCard, { props: { alert, onViewTradePlan } });

		expect(screen.getByText('View Trade Plan')).toBeInTheDocument();
	});

	it('should not show "View Trade Plan" for non-ENTRY alerts', () => {
		const alert = createMockAlert({ type: 'EXIT' });
		const onViewTradePlan = vi.fn();
		render(AlertCard, { props: { alert, onViewTradePlan } });

		expect(screen.queryByText('View Trade Plan')).not.toBeInTheDocument();
	});

	it('should call onViewTradePlan when clicked', async () => {
		const alert = createMockAlert({ type: 'ENTRY' });
		const onViewTradePlan = vi.fn();
		render(AlertCard, { props: { alert, onViewTradePlan } });

		const button = screen.getByText('View Trade Plan');
		await fireEvent.click(button);

		expect(onViewTradePlan).toHaveBeenCalledWith(alert);
	});
});

// ===============================================================================
// TEST SUITE: Index Prop
// ===============================================================================

describe('AlertCard - Index Prop', () => {
	afterEach(() => {
		cleanup();
	});

	it('should accept index prop for animation timing', () => {
		const alert = createMockAlert();
		// Index prop is used for staggered animations
		// Component should render without errors
		expect(() => {
			render(AlertCard, { props: { alert, index: 5 } });
		}).not.toThrow();
	});

	it('should default index to 0', () => {
		const alert = createMockAlert();
		expect(() => {
			render(AlertCard, { props: { alert } });
		}).not.toThrow();
	});
});
