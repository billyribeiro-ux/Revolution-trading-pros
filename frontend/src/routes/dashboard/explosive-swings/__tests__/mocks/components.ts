/**
 * ===============================================================================
 * Component Mocks for Isolated Testing
 * ===============================================================================
 *
 * @description Mock implementations of child components for unit testing
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Vitest January 2026 Patterns
 *
 * This module provides:
 * - Stub components for isolated testing
 * - Mock event emitters for component interactions
 * - Type-safe component mock factories
 */

import { vi } from 'vitest';

// ===============================================================================
// MOCK COMPONENT FACTORY
// ===============================================================================

/**
 * Creates a mock Svelte component that renders a simple div
 * and captures props for assertion
 */
export function createMockComponent(name: string) {
	const capturedProps: unknown[] = [];
	const capturedEvents: Array<{ event: string; detail: unknown }> = [];

	const MockComponent = vi.fn().mockImplementation((options: { props?: Record<string, unknown> }) => {
		if (options?.props) {
			capturedProps.push(options.props);
		}

		return {
			$set: vi.fn((newProps: Record<string, unknown>) => {
				capturedProps.push(newProps);
			}),
			$destroy: vi.fn(),
			$on: vi.fn((event: string, callback: (e: CustomEvent) => void) => {
				// Store event listener reference
				return () => {};
			})
		};
	});

	return {
		MockComponent,
		capturedProps,
		capturedEvents,
		getLastProps: () => capturedProps[capturedProps.length - 1],
		clearCaptured: () => {
			capturedProps.length = 0;
			capturedEvents.length = 0;
		}
	};
}

// ===============================================================================
// SPECIFIC COMPONENT MOCKS
// ===============================================================================

/**
 * Mock WeeklyHero component
 */
export const mockWeeklyHero = createMockComponent('WeeklyHero');

/**
 * Mock AlertsFeed component
 */
export const mockAlertsFeed = createMockComponent('AlertsFeed');

/**
 * Mock AlertCard component
 */
export const mockAlertCard = createMockComponent('AlertCard');

/**
 * Mock PerformanceSummary component
 */
export const mockPerformanceSummary = createMockComponent('PerformanceSummary');

/**
 * Mock ActivePositionCard component
 */
export const mockActivePositionCard = createMockComponent('ActivePositionCard');

/**
 * Mock ClosedTradesPills component
 */
export const mockClosedTradesPills = createMockComponent('ClosedTradesPills');

/**
 * Mock QuickStatsBar component
 */
export const mockQuickStatsBar = createMockComponent('QuickStatsBar');

/**
 * Mock TradePlanTable component
 */
export const mockTradePlanTable = createMockComponent('TradePlanTable');

/**
 * Mock VideoPlayer component
 */
export const mockVideoPlayer = createMockComponent('VideoPlayer');

/**
 * Mock Pagination component
 */
export const mockPagination = createMockComponent('Pagination');

/**
 * Mock FilterBar component
 */
export const mockFilterBar = createMockComponent('FilterBar');

// ===============================================================================
// MODAL COMPONENT MOCKS
// ===============================================================================

/**
 * Mock AlertModal component with form handling
 */
export function createMockAlertModal() {
	const onSave = vi.fn();
	const onClose = vi.fn();

	return {
		...createMockComponent('AlertModal'),
		onSave,
		onClose,
		simulateSubmit: (data: Record<string, unknown>) => {
			onSave(data);
		},
		simulateClose: () => {
			onClose();
		}
	};
}

/**
 * Mock TradeEntryModal component
 */
export function createMockTradeEntryModal() {
	const onSave = vi.fn();
	const onClose = vi.fn();

	return {
		...createMockComponent('TradeEntryModal'),
		onSave,
		onClose,
		simulateSubmit: (data: Record<string, unknown>) => {
			onSave(data);
		},
		simulateClose: () => {
			onClose();
		}
	};
}

/**
 * Mock ClosePositionModal component
 */
export function createMockClosePositionModal() {
	const onConfirm = vi.fn();
	const onCancel = vi.fn();

	return {
		...createMockComponent('ClosePositionModal'),
		onConfirm,
		onCancel,
		simulateConfirm: (exitPrice: number) => {
			onConfirm({ exitPrice, exitDate: new Date().toISOString() });
		},
		simulateCancel: () => {
			onCancel();
		}
	};
}

/**
 * Mock VideoUploadModal component
 */
export function createMockVideoUploadModal() {
	const onUpload = vi.fn();
	const onClose = vi.fn();
	const uploadProgress = { current: 0 };

	return {
		...createMockComponent('VideoUploadModal'),
		onUpload,
		onClose,
		uploadProgress,
		simulateUpload: (file: File) => {
			onUpload(file);
		},
		simulateProgress: (percent: number) => {
			uploadProgress.current = percent;
		},
		simulateClose: () => {
			onClose();
		}
	};
}

// ===============================================================================
// UI COMPONENT MOCKS
// ===============================================================================

/**
 * Mock Loading spinner component
 */
export const mockLoadingSpinner = createMockComponent('LoadingSpinner');

/**
 * Mock Error boundary component
 */
export function createMockErrorBoundary() {
	const onError = vi.fn();
	const onRetry = vi.fn();

	return {
		...createMockComponent('ErrorBoundary'),
		onError,
		onRetry,
		simulateError: (error: Error) => {
			onError(error);
		},
		simulateRetry: () => {
			onRetry();
		}
	};
}

/**
 * Mock Toast notification component
 */
export function createMockToast() {
	const toasts: Array<{ id: string; message: string; type: string }> = [];

	return {
		toasts,
		show: vi.fn((message: string, type = 'info') => {
			const id = Math.random().toString(36).slice(2);
			toasts.push({ id, message, type });
			return id;
		}),
		dismiss: vi.fn((id: string) => {
			const index = toasts.findIndex((t) => t.id === id);
			if (index !== -1) {
				toasts.splice(index, 1);
			}
		}),
		clear: vi.fn(() => {
			toasts.length = 0;
		})
	};
}

// ===============================================================================
// ICON COMPONENT MOCKS
// ===============================================================================

/**
 * Mock FontAwesome icons - returns a simple span with icon name
 */
export function createMockIcon(iconName: string) {
	const mock = createMockComponent(`Icon-${iconName}`);
	return {
		...mock,
		render: () => `<span data-testid="icon-${iconName}"></span>`
	};
}

/**
 * Mock Heroicons - returns a simple span with icon name
 */
export function createMockHeroIcon(iconName: string) {
	const mock = createMockComponent(`HeroIcon-${iconName}`);
	return {
		...mock,
		render: () => `<span data-testid="heroicon-${iconName}"></span>`
	};
}

// ===============================================================================
// CHART COMPONENT MOCKS
// ===============================================================================

/**
 * Mock performance chart component
 */
export function createMockPerformanceChart() {
	const chartData: Array<{ x: Date; y: number }> = [];
	const updateData = vi.fn((data: typeof chartData) => {
		chartData.length = 0;
		chartData.push(...data);
	});

	return {
		...createMockComponent('PerformanceChart'),
		chartData,
		updateData,
		simulateHover: vi.fn(),
		simulateClick: vi.fn()
	};
}

/**
 * Mock price chart component (lightweight-charts)
 */
export function createMockPriceChart() {
	return {
		...createMockComponent('PriceChart'),
		setData: vi.fn(),
		update: vi.fn(),
		resize: vi.fn(),
		remove: vi.fn()
	};
}

// ===============================================================================
// FORM COMPONENT MOCKS
// ===============================================================================

/**
 * Mock form input with validation
 */
export function createMockFormInput(name: string) {
	let value = '';
	let error: string | null = null;
	const onChange = vi.fn((newValue: string) => {
		value = newValue;
	});
	const onBlur = vi.fn();

	return {
		name,
		get value() {
			return value;
		},
		set value(v: string) {
			value = v;
			onChange(v);
		},
		get error() {
			return error;
		},
		setError: (e: string | null) => {
			error = e;
		},
		onChange,
		onBlur,
		validate: vi.fn(() => error === null)
	};
}

/**
 * Mock form select with options
 */
export function createMockFormSelect<T extends string>(name: string, options: T[]) {
	let value: T = options[0];
	const onChange = vi.fn((newValue: T) => {
		value = newValue;
	});

	return {
		name,
		options,
		get value() {
			return value;
		},
		set value(v: T) {
			value = v;
			onChange(v);
		},
		onChange
	};
}

// ===============================================================================
// RESET UTILITIES
// ===============================================================================

/**
 * Reset all component mocks to their initial state
 */
export function resetAllComponentMocks() {
	mockWeeklyHero.clearCaptured();
	mockAlertsFeed.clearCaptured();
	mockAlertCard.clearCaptured();
	mockPerformanceSummary.clearCaptured();
	mockActivePositionCard.clearCaptured();
	mockClosedTradesPills.clearCaptured();
	mockQuickStatsBar.clearCaptured();
	mockTradePlanTable.clearCaptured();
	mockVideoPlayer.clearCaptured();
	mockPagination.clearCaptured();
	mockFilterBar.clearCaptured();
	mockLoadingSpinner.clearCaptured();
}
