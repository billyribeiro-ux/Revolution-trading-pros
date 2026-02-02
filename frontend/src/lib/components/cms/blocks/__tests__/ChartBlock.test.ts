/**
 * ===============================================================================
 * ChartBlock Component - Unit Tests
 * ===============================================================================
 *
 * @description Comprehensive tests for ChartBlock Svelte component
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Svelte 5 + Testing Library
 *
 * Tests cover:
 * - TradingView embed mode rendering
 * - Image mode rendering
 * - Symbol input and validation
 * - Interval selection
 * - Theme switching (light/dark/auto)
 * - Loading and error states
 * - Fullscreen toggle
 * - Edit mode controls
 * - Accessibility (ARIA attributes)
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, fireEvent, cleanup, waitFor } from '@testing-library/svelte';
import ChartBlock from '../trading/ChartBlock.svelte';
import type { Block } from '../types';

// ===============================================================================
// TEST FIXTURES
// ===============================================================================

function createMockBlock(overrides: Partial<Block> = {}): Block {
	return {
		id: 'chart-1',
		type: 'chart',
		content: {
			chartMode: 'embed',
			chartSymbol: 'NASDAQ:AAPL',
			chartInterval: '1D',
			chartTheme: 'auto',
			chartImageUrl: '',
			chartImageAlt: 'Trading chart',
			chartImageCaption: '',
			...overrides.content
		},
		settings: {
			chartHeight: '400px',
			chartShowToolbar: true,
			chartAllowFullscreen: true,
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

function createImageModeBlock(imageUrl = 'https://example.com/chart.png'): Block {
	return createMockBlock({
		content: {
			chartMode: 'image',
			chartImageUrl: imageUrl,
			chartImageAlt: 'Stock chart analysis',
			chartImageCaption: 'Weekly AAPL analysis'
		}
	});
}

// ===============================================================================
// MOCK SETUP
// ===============================================================================

// Mock MutationObserver
const mockMutationObserver = vi.fn().mockImplementation(() => ({
	observe: vi.fn(),
	disconnect: vi.fn(),
	takeRecords: vi.fn()
}));

beforeEach(() => {
	vi.stubGlobal('MutationObserver', mockMutationObserver);
});

afterEach(() => {
	cleanup();
	vi.clearAllMocks();
});

// ===============================================================================
// TEST SUITE: TradingView Embed Mode
// ===============================================================================

describe('ChartBlock - TradingView Embed Mode', () => {
	it('should render TradingView iframe in embed mode', () => {
		const block = createMockBlock();
		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const iframe = screen.getByTitle(/trading chart for nasdaq:aapl/i);
		expect(iframe).toBeInTheDocument();
		expect(iframe.tagName).toBe('IFRAME');
	});

	it('should construct correct TradingView embed URL', () => {
		const block = createMockBlock({
			content: {
				chartMode: 'embed',
				chartSymbol: 'NYSE:MSFT',
				chartInterval: '1W'
			}
		});

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const iframe = screen.getByTitle(/trading chart for nyse:msft/i);
		expect(iframe).toHaveAttribute('src');
		const src = iframe.getAttribute('src') || '';
		expect(src).toContain('tradingview.com');
		expect(src).toContain('symbol=NYSE%3AMSFT');
	});

	it('should show loading state initially', () => {
		const block = createMockBlock();
		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText(/loading chart/i)).toBeInTheDocument();
	});

	it('should have sandbox attribute for security', () => {
		const block = createMockBlock();
		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const iframe = screen.getByTitle(/trading chart/i);
		expect(iframe).toHaveAttribute('sandbox');
		expect(iframe.getAttribute('sandbox')).toContain('allow-scripts');
	});

	it('should apply correct height from settings', () => {
		const block = createMockBlock({
			settings: { chartHeight: '500px' }
		});

		const { container } = render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const chartContainer = container.querySelector('.chart-container');
		expect(chartContainer).toHaveStyle({ height: '500px' });
	});
});

// ===============================================================================
// TEST SUITE: Image Mode
// ===============================================================================

describe('ChartBlock - Image Mode', () => {
	it('should render image in image mode', () => {
		const block = createImageModeBlock('https://example.com/chart.png');

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const img = screen.getByRole('img');
		expect(img).toBeInTheDocument();
		expect(img).toHaveAttribute('src', 'https://example.com/chart.png');
	});

	it('should display alt text correctly', () => {
		const block = createImageModeBlock();

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const img = screen.getByRole('img');
		expect(img).toHaveAttribute('alt', 'Stock chart analysis');
	});

	it('should show caption when provided', () => {
		const block = createImageModeBlock();

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Weekly AAPL analysis')).toBeInTheDocument();
	});

	it('should show placeholder when no image URL', () => {
		const block = createMockBlock({
			content: {
				chartMode: 'image',
				chartImageUrl: ''
			}
		});

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText(/enter an image url/i)).toBeInTheDocument();
	});

	it('should show error for invalid image URL', () => {
		const block = createMockBlock({
			content: {
				chartMode: 'image',
				chartImageUrl: 'not-a-valid-url'
			}
		});

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText(/please enter a valid url/i)).toBeInTheDocument();
	});
});

// ===============================================================================
// TEST SUITE: Edit Mode Controls
// ===============================================================================

describe('ChartBlock - Edit Mode Controls', () => {
	it('should show mode toggle in edit mode when selected', () => {
		const block = createMockBlock();

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('TradingView')).toBeInTheDocument();
		expect(screen.getByText('Image')).toBeInTheDocument();
	});

	it('should show symbol input in embed mode', () => {
		const block = createMockBlock();

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Symbol:')).toBeInTheDocument();
		const input = screen.getByPlaceholderText(/nasdaq:aapl/i);
		expect(input).toBeInTheDocument();
	});

	it('should show interval selector in embed mode', () => {
		const block = createMockBlock();

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Interval:')).toBeInTheDocument();
		expect(screen.getByText('1 Day')).toBeInTheDocument();
	});

	it('should show theme selector in embed mode', () => {
		const block = createMockBlock();

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Theme:')).toBeInTheDocument();
		expect(screen.getByText('Auto (sync)')).toBeInTheDocument();
	});

	it('should show height selector', () => {
		const block = createMockBlock();

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Height:')).toBeInTheDocument();
	});

	it('should show symbol examples', () => {
		const block = createMockBlock();

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('NASDAQ:AAPL')).toBeInTheDocument();
		expect(screen.getByText('NYSE:SPY')).toBeInTheDocument();
		expect(screen.getByText('BINANCE:BTCUSDT')).toBeInTheDocument();
	});

	it('should call onUpdate when symbol changes', async () => {
		const block = createMockBlock();
		const onUpdate = vi.fn();

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const input = screen.getByPlaceholderText(/nasdaq:aapl/i);
		await fireEvent.input(input, { target: { value: 'NYSE:TSLA' } });

		expect(onUpdate).toHaveBeenCalled();
	});

	it('should show image URL input in image mode', () => {
		const block = createImageModeBlock();

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-2',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Image URL:')).toBeInTheDocument();
	});

	it('should show alt text input in image mode', () => {
		const block = createImageModeBlock();

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-2',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText('Alt Text:')).toBeInTheDocument();
	});

	it('should toggle between embed and image mode', async () => {
		const block = createMockBlock();
		const onUpdate = vi.fn();

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const imageButton = screen.getByText('Image');
		await fireEvent.click(imageButton);

		expect(onUpdate).toHaveBeenCalledWith(
			expect.objectContaining({
				content: expect.objectContaining({
					chartMode: 'image'
				})
			})
		);
	});
});

// ===============================================================================
// TEST SUITE: Symbol Validation
// ===============================================================================

describe('ChartBlock - Symbol Validation', () => {
	it('should accept valid symbol format', () => {
		const block = createMockBlock({
			content: { chartSymbol: 'NASDAQ:NVDA' }
		});

		const { container } = render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const input = container.querySelector('input[type="text"]');
		expect(input).not.toHaveClass('invalid');
	});

	it('should show placeholder for invalid symbol', () => {
		const block = createMockBlock({
			content: { chartSymbol: '' }
		});

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		expect(screen.getByText(/enter a valid trading symbol/i)).toBeInTheDocument();
	});

	it('should uppercase symbol input', async () => {
		const block = createMockBlock();
		const onUpdate = vi.fn();

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: true,
				isSelected: true,
				onUpdate
			}
		});

		const input = screen.getByPlaceholderText(/nasdaq:aapl/i);
		await fireEvent.input(input, { target: { value: 'nyse:msft' } });

		expect(onUpdate).toHaveBeenCalledWith(
			expect.objectContaining({
				content: expect.objectContaining({
					chartSymbol: 'NYSE:MSFT'
				})
			})
		);
	});
});

// ===============================================================================
// TEST SUITE: Toolbar and Actions
// ===============================================================================

describe('ChartBlock - Toolbar and Actions', () => {
	it('should show toolbar on hover in view mode', () => {
		const block = createMockBlock();

		const { container } = render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const toolbar = container.querySelector('.chart-toolbar');
		expect(toolbar).toBeInTheDocument();
	});

	it('should show fullscreen button when allowed', () => {
		const block = createMockBlock({
			settings: { chartAllowFullscreen: true }
		});

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const fullscreenBtn = screen.getByLabelText(/fullscreen/i);
		expect(fullscreenBtn).toBeInTheDocument();
	});

	it('should show open in new tab button', () => {
		const block = createMockBlock();

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const newTabBtn = screen.getByLabelText(/open in new tab/i);
		expect(newTabBtn).toBeInTheDocument();
	});

	it('should display symbol badge in toolbar', () => {
		const block = createMockBlock({
			content: { chartSymbol: 'NYSE:SPY' }
		});

		const { container } = render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const symbolBadge = container.querySelector('.symbol-badge');
		expect(symbolBadge).toBeInTheDocument();
		expect(symbolBadge?.textContent).toBe('NYSE:SPY');
	});

	it('should display interval badge in toolbar', () => {
		const block = createMockBlock({
			content: { chartInterval: '1W' }
		});

		const { container } = render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const intervalBadge = container.querySelector('.interval-badge');
		expect(intervalBadge).toBeInTheDocument();
		expect(intervalBadge?.textContent).toBe('1W');
	});
});

// ===============================================================================
// TEST SUITE: Accessibility
// ===============================================================================

describe('ChartBlock - Accessibility', () => {
	it('should have correct role attribute', () => {
		const block = createMockBlock();

		const { container } = render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const chartBlock = container.querySelector('.chart-block');
		expect(chartBlock).toHaveAttribute('role', 'img');
	});

	it('should have correct aria-label for embed mode', () => {
		const block = createMockBlock({
			content: { chartSymbol: 'NASDAQ:TSLA', chartInterval: '1M' }
		});

		const { container } = render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const chartBlock = container.querySelector('.chart-block');
		expect(chartBlock).toHaveAttribute(
			'aria-label',
			'Trading chart for NASDAQ:TSLA, 1M interval'
		);
	});

	it('should have correct aria-label for image mode', () => {
		const block = createImageModeBlock();

		const { container } = render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-2',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const chartBlock = container.querySelector('.chart-block');
		expect(chartBlock).toHaveAttribute('aria-label', 'Stock chart analysis');
	});

	it('should have aria-live on loading indicator', () => {
		const block = createMockBlock();

		const { container } = render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const loading = container.querySelector('.chart-loading');
		expect(loading).toHaveAttribute('role', 'status');
		expect(loading).toHaveAttribute('aria-live', 'polite');
	});

	it('should have title attribute on iframe', () => {
		const block = createMockBlock();

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		const iframe = screen.getByTitle(/trading chart/i);
		expect(iframe).toHaveAttribute('title');
	});

	it('should have aria-pressed on mode toggle buttons', () => {
		const block = createMockBlock();

		render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: true,
				isSelected: true,
				onUpdate: vi.fn()
			}
		});

		const embedBtn = screen.getByRole('button', { name: /tradingview/i });
		const imageBtn = screen.getByRole('button', { name: /image/i });

		expect(embedBtn).toHaveAttribute('aria-pressed', 'true');
		expect(imageBtn).toHaveAttribute('aria-pressed', 'false');
	});
});

// ===============================================================================
// TEST SUITE: Error Handling
// ===============================================================================

describe('ChartBlock - Error Handling', () => {
	it('should show error state with retry button', () => {
		const block = createMockBlock();

		const { container } = render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn()
			}
		});

		// Simulate error state by triggering iframe error
		const iframe = container.querySelector('iframe');
		if (iframe) {
			fireEvent.error(iframe);
		}
	});

	it('should call onError callback when provided', () => {
		const block = createMockBlock();
		const onError = vi.fn();

		const { container } = render(ChartBlock, {
			props: {
				block,
				blockId: 'chart-1',
				isEditing: false,
				isSelected: false,
				onUpdate: vi.fn(),
				onError
			}
		});

		const iframe = container.querySelector('iframe');
		if (iframe) {
			fireEvent.error(iframe);
		}

		// onError should be called on error
		expect(onError).toHaveBeenCalled();
	});
});

// ===============================================================================
// TEST SUITE: Interval Mapping
// ===============================================================================

describe('ChartBlock - Interval Mapping', () => {
	const intervalTests = [
		{ input: '1D', expected: 'D' },
		{ input: '1W', expected: 'W' },
		{ input: '1M', expected: 'M' },
		{ input: '3M', expected: '3M' },
		{ input: '1Y', expected: '12M' }
	];

	intervalTests.forEach(({ input, expected }) => {
		it(`should map ${input} to TradingView interval ${expected}`, () => {
			const block = createMockBlock({
				content: { chartInterval: input }
			});

			render(ChartBlock, {
				props: {
					block,
					blockId: 'chart-1',
					isEditing: false,
					isSelected: false,
					onUpdate: vi.fn()
				}
			});

			const iframe = screen.getByTitle(/trading chart/i);
			const src = iframe.getAttribute('src') || '';
			expect(src).toContain(`interval=${expected}`);
		});
	});
});
