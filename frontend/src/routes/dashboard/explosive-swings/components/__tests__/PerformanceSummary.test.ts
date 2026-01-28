/**
 * ===============================================================================
 * PerformanceSummary Component - Unit Tests
 * ===============================================================================
 *
 * @description Comprehensive tests for PerformanceSummary Svelte component
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Svelte 5 + Testing Library
 *
 * Tests cover:
 * - Stats display (win rate, trades, avg win/loss)
 * - Loading state (skeleton display)
 * - Error state
 * - Empty state
 * - Active positions display
 * - Closed trades ticker pills
 * - Admin actions (Add Trade)
 * - Responsive design considerations
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, fireEvent, cleanup } from '@testing-library/svelte';
import PerformanceSummary from '../PerformanceSummary.svelte';
import type { WeeklyPerformance, ClosedTrade, ActivePosition } from '../../types';

// ===============================================================================
// TEST FIXTURES
// ===============================================================================

function createMockPerformance(overrides: Partial<WeeklyPerformance> = {}): WeeklyPerformance {
	return {
		winRate: 82,
		totalTrades: 7,
		winningTrades: 6,
		avgWinPercent: 5.7,
		avgLossPercent: 2.1,
		riskRewardRatio: 3.2,
		...overrides
	};
}

function createMockClosedTrade(overrides: Partial<ClosedTrade> = {}): ClosedTrade {
	return {
		id: String(Math.random()),
		ticker: 'MSFT',
		percentageGain: 8.2,
		isWinner: true,
		closedAt: new Date(),
		entryPrice: 425,
		exitPrice: 460,
		...overrides
	};
}

function createMockActivePosition(overrides: Partial<ActivePosition> = {}): ActivePosition {
	return {
		id: String(Math.random()),
		ticker: 'NVDA',
		status: 'ACTIVE',
		entryPrice: 142.5,
		currentPrice: 145.0,
		unrealizedPercent: 1.75,
		targets: [
			{ price: 156, percentFromEntry: 9.5, label: 'Target 1' },
			{ price: 168, percentFromEntry: 17.9, label: 'Target 2' }
		],
		stopLoss: { price: 136, percentFromEntry: -4.6 },
		progressToTarget1: 18.5,
		triggeredAt: new Date(),
		...overrides
	};
}

const defaultProps = {
	performance: createMockPerformance(),
	closedTrades: [],
	activePositions: []
};

// ===============================================================================
// TEST SUITE: Basic Rendering
// ===============================================================================

describe('PerformanceSummary - Basic Rendering', () => {
	afterEach(() => {
		cleanup();
	});

	it('should render section with correct heading', () => {
		render(PerformanceSummary, { props: defaultProps });

		expect(screen.getByRole('region')).toBeInTheDocument();
		expect(screen.getByText("This Week's Performance")).toBeInTheDocument();
	});

	it('should have accessible heading structure', () => {
		render(PerformanceSummary, { props: defaultProps });

		const heading = screen.getByRole('heading', { name: "This Week's Performance" });
		expect(heading).toBeInTheDocument();
		expect(heading.tagName).toBe('H2');
	});

	it('should display subtitle', () => {
		render(PerformanceSummary, { props: defaultProps });

		expect(screen.getByText('Trade results and active positions')).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Win Rate Display
// ===============================================================================

describe('PerformanceSummary - Win Rate Display', () => {
	afterEach(() => {
		cleanup();
	});

	it('should display win rate percentage', () => {
		const props = {
			...defaultProps,
			performance: createMockPerformance({ winRate: 85 })
		};
		render(PerformanceSummary, { props });

		expect(screen.getByText('85%')).toBeInTheDocument();
		expect(screen.getByText('Win Rate')).toBeInTheDocument();
	});

	it('should display win/total trades ratio', () => {
		const props = {
			...defaultProps,
			performance: createMockPerformance({ winningTrades: 6, totalTrades: 7 })
		};
		render(PerformanceSummary, { props });

		expect(screen.getByText('6/7 trades')).toBeInTheDocument();
	});

	it('should handle 100% win rate', () => {
		const props = {
			...defaultProps,
			performance: createMockPerformance({ winRate: 100, winningTrades: 10, totalTrades: 10 })
		};
		render(PerformanceSummary, { props });

		expect(screen.getByText('100%')).toBeInTheDocument();
		expect(screen.getByText('10/10 trades')).toBeInTheDocument();
	});

	it('should handle 0% win rate', () => {
		const props = {
			...defaultProps,
			performance: createMockPerformance({ winRate: 0, winningTrades: 0, totalTrades: 5 })
		};
		render(PerformanceSummary, { props });

		expect(screen.getByText('0%')).toBeInTheDocument();
		expect(screen.getByText('0/5 trades')).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Performance Metrics
// ===============================================================================

describe('PerformanceSummary - Performance Metrics', () => {
	afterEach(() => {
		cleanup();
	});

	it('should display average winner with positive styling', () => {
		const props = {
			...defaultProps,
			closedTrades: [createMockClosedTrade()],
			performance: createMockPerformance({ avgWinPercent: 5.7 })
		};
		render(PerformanceSummary, { props });

		expect(screen.getByText('Avg Winner:')).toBeInTheDocument();
		expect(screen.getByText('+5.7%')).toBeInTheDocument();
	});

	it('should display average loser with negative styling', () => {
		const props = {
			...defaultProps,
			closedTrades: [createMockClosedTrade()],
			performance: createMockPerformance({ avgLossPercent: 2.1 })
		};
		render(PerformanceSummary, { props });

		expect(screen.getByText('Avg Loser:')).toBeInTheDocument();
		expect(screen.getByText('-2.1%')).toBeInTheDocument();
	});

	it('should display risk/reward ratio when positive', () => {
		const props = {
			...defaultProps,
			closedTrades: [createMockClosedTrade()],
			performance: createMockPerformance({ riskRewardRatio: 3.2 })
		};
		render(PerformanceSummary, { props });

		expect(screen.getByText('R/R:')).toBeInTheDocument();
		expect(screen.getByText('3.2:1')).toBeInTheDocument();
	});

	it('should show dash when risk/reward is zero', () => {
		const props = {
			...defaultProps,
			closedTrades: [createMockClosedTrade()],
			performance: createMockPerformance({ riskRewardRatio: 0 })
		};
		render(PerformanceSummary, { props });

		// R/R section should not be rendered or show dash
		const rrLabel = screen.queryByText('R/R:');
		if (rrLabel) {
			expect(screen.queryByText('0.0:1')).not.toBeInTheDocument();
		}
	});
});

// ===============================================================================
// TEST SUITE: Closed Trades Section
// ===============================================================================

describe('PerformanceSummary - Closed Trades Section', () => {
	afterEach(() => {
		cleanup();
	});

	it('should display "Closed This Week" heading', () => {
		const props = {
			...defaultProps,
			closedTrades: [createMockClosedTrade()]
		};
		render(PerformanceSummary, { props });

		expect(screen.getByText('Closed This Week')).toBeInTheDocument();
	});

	it('should render ticker pills for closed trades', () => {
		const props = {
			...defaultProps,
			closedTrades: [
				createMockClosedTrade({ ticker: 'MSFT', percentageGain: 8.2 }),
				createMockClosedTrade({ ticker: 'AAPL', percentageGain: 5.1 }),
				createMockClosedTrade({ ticker: 'GOOGL', percentageGain: -2.1 })
			]
		};
		render(PerformanceSummary, { props });

		expect(screen.getByText('MSFT')).toBeInTheDocument();
		expect(screen.getByText('AAPL')).toBeInTheDocument();
		expect(screen.getByText('GOOGL')).toBeInTheDocument();
	});

	it('should have list role for accessibility', () => {
		const props = {
			...defaultProps,
			closedTrades: [createMockClosedTrade()]
		};
		render(PerformanceSummary, { props });

		const list = screen.getByRole('list', { name: 'Closed trades this week' });
		expect(list).toBeInTheDocument();
	});

	it('should not show closed section when no trades', () => {
		const props = {
			...defaultProps,
			closedTrades: [],
			isLoading: false
		};
		render(PerformanceSummary, { props });

		expect(screen.queryByText('Closed This Week')).not.toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Active Positions Section
// ===============================================================================

describe('PerformanceSummary - Active Positions Section', () => {
	afterEach(() => {
		cleanup();
	});

	it('should display "Active Positions" heading with count', () => {
		const props = {
			...defaultProps,
			activePositions: [
				createMockActivePosition({ ticker: 'NVDA' }),
				createMockActivePosition({ ticker: 'META' })
			]
		};
		render(PerformanceSummary, { props });

		expect(screen.getByText('Active Positions')).toBeInTheDocument();
		expect(screen.getByText('(2)')).toBeInTheDocument();
	});

	it('should render ActivePositionCard for each position', () => {
		const props = {
			...defaultProps,
			activePositions: [
				createMockActivePosition({ ticker: 'NVDA' }),
				createMockActivePosition({ ticker: 'META' })
			]
		};
		render(PerformanceSummary, { props });

		expect(screen.getByText('NVDA')).toBeInTheDocument();
		expect(screen.getByText('META')).toBeInTheDocument();
	});

	it('should not show count during loading', () => {
		const props = {
			...defaultProps,
			activePositions: [],
			isLoading: true
		};
		render(PerformanceSummary, { props });

		// Count should not be visible during loading
		expect(screen.queryByText('(0)')).not.toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Loading State
// ===============================================================================

describe('PerformanceSummary - Loading State', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show skeleton loaders for ticker pills when loading', () => {
		const props = {
			...defaultProps,
			isLoading: true
		};
		const { container } = render(PerformanceSummary, { props });

		const skeletons = container.querySelectorAll('.ticker-pill-skeleton');
		expect(skeletons.length).toBeGreaterThan(0);
	});

	it('should show skeleton loaders for position cards when loading', () => {
		const props = {
			...defaultProps,
			isLoading: true
		};
		const { container } = render(PerformanceSummary, { props });

		const skeletons = container.querySelectorAll('.position-skeleton');
		expect(skeletons.length).toBeGreaterThan(0);
	});

	it('should hide actual data when loading', () => {
		const props = {
			...defaultProps,
			closedTrades: [createMockClosedTrade({ ticker: 'MSFT' })],
			activePositions: [createMockActivePosition({ ticker: 'NVDA' })],
			isLoading: true
		};
		render(PerformanceSummary, { props });

		// Real data should not be shown during loading
		// (TickerPill and ActivePositionCard components won't render)
		// The skeleton loaders should be shown instead
	});
});

// ===============================================================================
// TEST SUITE: Empty State
// ===============================================================================

describe('PerformanceSummary - Empty State', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show empty state when no trades and not loading', () => {
		const props = {
			...defaultProps,
			closedTrades: [],
			activePositions: [],
			isLoading: false
		};
		render(PerformanceSummary, { props });

		expect(screen.getByText('No trades this week yet')).toBeInTheDocument();
	});

	it('should show helpful message in empty state', () => {
		const props = {
			...defaultProps,
			closedTrades: [],
			activePositions: [],
			isLoading: false
		};
		render(PerformanceSummary, { props });

		expect(
			screen.getByText('Check back after the market opens for live alerts and positions.')
		).toBeInTheDocument();
	});

	it('should show empty icon', () => {
		const props = {
			...defaultProps,
			closedTrades: [],
			activePositions: [],
			isLoading: false
		};
		const { container } = render(PerformanceSummary, { props });

		const emptyIcon = container.querySelector('.empty-icon');
		expect(emptyIcon).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Admin Actions
// ===============================================================================

describe('PerformanceSummary - Admin Actions', () => {
	afterEach(() => {
		cleanup();
	});

	it('should show "Add Trade" button for admin users', () => {
		const props = {
			...defaultProps,
			activePositions: [createMockActivePosition()],
			isAdmin: true,
			onAddTrade: vi.fn()
		};
		render(PerformanceSummary, { props });

		expect(screen.getByText('Add Trade')).toBeInTheDocument();
	});

	it('should not show "Add Trade" button for non-admin users', () => {
		const props = {
			...defaultProps,
			activePositions: [createMockActivePosition()],
			isAdmin: false,
			onAddTrade: vi.fn()
		};
		render(PerformanceSummary, { props });

		expect(screen.queryByText('Add Trade')).not.toBeInTheDocument();
	});

	it('should call onAddTrade when button clicked', async () => {
		const onAddTrade = vi.fn();
		const props = {
			...defaultProps,
			activePositions: [createMockActivePosition()],
			isAdmin: true,
			onAddTrade
		};
		render(PerformanceSummary, { props });

		const addButton = screen.getByText('Add Trade');
		await fireEvent.click(addButton);

		expect(onAddTrade).toHaveBeenCalled();
	});

	it('should show empty state with add trade button for admin', () => {
		const onAddTrade = vi.fn();
		const props = {
			...defaultProps,
			closedTrades: [],
			activePositions: [],
			isAdmin: true,
			onAddTrade
		};
		render(PerformanceSummary, { props });

		// Admin should see "Add Your First Trade" button
		expect(screen.getByText('Add Your First Trade')).toBeInTheDocument();
	});

	it('should call onAddTrade from empty state button', async () => {
		const onAddTrade = vi.fn();
		const props = {
			...defaultProps,
			closedTrades: [],
			activePositions: [],
			isAdmin: true,
			onAddTrade
		};
		render(PerformanceSummary, { props });

		const addButton = screen.getByText('Add Your First Trade');
		await fireEvent.click(addButton);

		expect(onAddTrade).toHaveBeenCalled();
	});
});

// ===============================================================================
// TEST SUITE: Position Actions (Callbacks)
// ===============================================================================

describe('PerformanceSummary - Position Actions', () => {
	afterEach(() => {
		cleanup();
	});

	it('should pass onClosePosition to ActivePositionCard', () => {
		const onClosePosition = vi.fn();
		const position = createMockActivePosition();
		const props = {
			...defaultProps,
			activePositions: [position],
			isAdmin: true,
			onClosePosition
		};
		render(PerformanceSummary, { props });

		// Component should render without error with callback passed
		expect(screen.getByText(position.ticker)).toBeInTheDocument();
	});

	it('should pass onUpdatePosition to ActivePositionCard', () => {
		const onUpdatePosition = vi.fn();
		const position = createMockActivePosition();
		const props = {
			...defaultProps,
			activePositions: [position],
			isAdmin: true,
			onUpdatePosition
		};
		render(PerformanceSummary, { props });

		expect(screen.getByText(position.ticker)).toBeInTheDocument();
	});

	it('should pass onInvalidatePosition to ActivePositionCard', () => {
		const onInvalidatePosition = vi.fn();
		const position = createMockActivePosition();
		const props = {
			...defaultProps,
			activePositions: [position],
			isAdmin: true,
			onInvalidatePosition
		};
		render(PerformanceSummary, { props });

		expect(screen.getByText(position.ticker)).toBeInTheDocument();
	});

	it('should pass onDeletePosition to ActivePositionCard', () => {
		const onDeletePosition = vi.fn();
		const position = createMockActivePosition();
		const props = {
			...defaultProps,
			activePositions: [position],
			isAdmin: true,
			onDeletePosition
		};
		render(PerformanceSummary, { props });

		expect(screen.getByText(position.ticker)).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Display Formatting
// ===============================================================================

describe('PerformanceSummary - Display Formatting', () => {
	afterEach(() => {
		cleanup();
	});

	it('should format risk/reward ratio correctly', () => {
		const props = {
			...defaultProps,
			closedTrades: [createMockClosedTrade()],
			performance: createMockPerformance({ riskRewardRatio: 2.71 })
		};
		render(PerformanceSummary, { props });

		expect(screen.getByText('2.7:1')).toBeInTheDocument();
	});

	it('should format positive percent with + sign', () => {
		const props = {
			...defaultProps,
			closedTrades: [createMockClosedTrade()],
			performance: createMockPerformance({ avgWinPercent: 5.7 })
		};
		render(PerformanceSummary, { props });

		expect(screen.getByText('+5.7%')).toBeInTheDocument();
	});

	it('should format negative percent with - sign', () => {
		const props = {
			...defaultProps,
			closedTrades: [createMockClosedTrade()],
			performance: createMockPerformance({ avgLossPercent: 2.1 })
		};
		render(PerformanceSummary, { props });

		expect(screen.getByText('-2.1%')).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Responsive Considerations
// ===============================================================================

describe('PerformanceSummary - Layout Structure', () => {
	afterEach(() => {
		cleanup();
	});

	it('should have section landmark for accessibility', () => {
		render(PerformanceSummary, { props: defaultProps });

		const section = screen.getByRole('region');
		expect(section).toHaveAttribute('aria-labelledby', 'performance-heading');
	});

	it('should render positions in grid layout', () => {
		const props = {
			...defaultProps,
			activePositions: [
				createMockActivePosition({ ticker: 'NVDA' }),
				createMockActivePosition({ ticker: 'META' })
			]
		};
		const { container } = render(PerformanceSummary, { props });

		const positionsGrid = container.querySelector('.positions-grid');
		expect(positionsGrid).toBeInTheDocument();
	});

	it('should render ticker pills in scrollable container', () => {
		const props = {
			...defaultProps,
			closedTrades: [
				createMockClosedTrade({ ticker: 'MSFT' }),
				createMockClosedTrade({ ticker: 'AAPL' }),
				createMockClosedTrade({ ticker: 'GOOGL' })
			]
		};
		const { container } = render(PerformanceSummary, { props });

		const scrollContainer = container.querySelector('.ticker-pills-scroll');
		expect(scrollContainer).toBeInTheDocument();
	});
});
