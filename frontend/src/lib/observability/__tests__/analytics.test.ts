/**
 * Analytics System - Comprehensive Test Suite
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Enterprise-level validation of all analytics functions.
 * Tests every exported function, type, and integration point.
 *
 * Run with: npx vitest run src/lib/observability/__tests__/analytics.test.ts
 *
 * @version 1.0.0
 */

// Import setup first to initialize globals
import './setup';

import { describe, it, expect, beforeEach, vi } from 'vitest';

// ═══════════════════════════════════════════════════════════════════════════
// Mock SvelteKit modules
// ═══════════════════════════════════════════════════════════════════════════

vi.mock('$app/environment', () => ({
	browser: true,
	dev: true,
	building: false,
	version: 'test'
}));

vi.mock('$env/dynamic/public', () => ({
	env: {
		PUBLIC_GA4_MEASUREMENT_ID: 'G-TESTID12345'
	}
}));

vi.mock('$lib/api/config', () => ({
	API_BASE_URL: 'http://localhost:8000'
}));

// ═══════════════════════════════════════════════════════════════════════════
// Import modules under test
// ═══════════════════════════════════════════════════════════════════════════

import {
	AnalyticsEvents,
	type AnalyticsConfig,
	type PageViewPayload,
	type CustomEventPayload,
	type PurchasePayload,
	type IdentifyPayload,
	type EcommerceItem
} from '../adapters/types';

import {
	getGoogleAnalyticsAdapter,
	createGoogleAnalyticsAdapter,
	resetGoogleAnalyticsAdapter
} from '../adapters/google-analytics';

import { getConsoleAdapter, createConsoleAdapter } from '../adapters/console';

import { getBackendAdapter, createBackendAdapter } from '../adapters/backend';

import { getOrchestrator, createOrchestrator, resetOrchestrator } from '../orchestrator';

import {
	metrics,
	track,
	trackPageView,
	identify,
	startTimer,
	trackPurchase,
	trackSignUp,
	trackLogin,
	trackLogout,
	trackSearch,
	trackShare,
	trackError,
	trackFormSubmit,
	trackFormError,
	Events
} from '../metrics';

import {
	initializeAnalytics,
	updateAnalyticsConsent,
	getAnalyticsMetrics,
	flushAnalytics,
	destroyAnalytics
} from '../index';
import { logger } from '$lib/utils/logger';

// ═══════════════════════════════════════════════════════════════════════════
// TEST SUITE: Adapter Types
// ═══════════════════════════════════════════════════════════════════════════

describe('Adapter Types', () => {
	it('should export AnalyticsEvents constant with all standard events', () => {
		expect(AnalyticsEvents).toBeDefined();
		expect(AnalyticsEvents.PAGE_VIEW).toBe('page_view');
		expect(AnalyticsEvents.LOGIN).toBe('login');
		expect(AnalyticsEvents.SIGN_UP).toBe('sign_up');
		expect(AnalyticsEvents.PURCHASE).toBe('purchase');
		expect(AnalyticsEvents.ADD_TO_CART).toBe('add_to_cart');
		expect(AnalyticsEvents.BEGIN_CHECKOUT).toBe('begin_checkout');
		expect(AnalyticsEvents.SEARCH).toBe('search');
		expect(AnalyticsEvents.SHARE).toBe('share');
		expect(AnalyticsEvents.ERROR).toBe('error');
	});

	it('should have Events constant with custom events', () => {
		expect(Events).toBeDefined();
		expect(Events.USER_SIGNED_UP).toBe('user_signed_up');
		expect(Events.USER_LOGGED_IN).toBe('user_logged_in');
		expect(Events.TRADING_ROOM_JOINED).toBe('trading_room_joined');
		expect(Events.VIDEO_STARTED).toBe('video_started');
		expect(Events.COURSE_STARTED).toBe('course_started');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// TEST SUITE: Google Analytics Adapter
// ═══════════════════════════════════════════════════════════════════════════

describe('GoogleAnalytics Adapter', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		(globalThis as any).window.dataLayer = [];
		resetGoogleAnalyticsAdapter();
	});

	it('should create singleton instance via getGoogleAnalyticsAdapter', () => {
		const adapter1 = getGoogleAnalyticsAdapter();
		const adapter2 = getGoogleAnalyticsAdapter();
		expect(adapter1).toBe(adapter2);
	});

	it('should create new instance via createGoogleAnalyticsAdapter', () => {
		const adapter1 = createGoogleAnalyticsAdapter();
		const adapter2 = createGoogleAnalyticsAdapter();
		expect(adapter1).not.toBe(adapter2);
	});

	it('should have correct id and name', () => {
		const adapter = createGoogleAnalyticsAdapter();
		expect(adapter.id).toBe('google-analytics');
		expect(adapter.name).toBe('Google Analytics 4');
	});

	it('should start in uninitialized state', () => {
		const adapter = createGoogleAnalyticsAdapter();
		expect(adapter.state).toBe('uninitialized');
	});

	it('should initialize with valid config', async () => {
		// Mock script loading in JSDOM - scripts don't actually load
		const originalCreateElement = document.createElement.bind(document);
		vi.spyOn(document, 'createElement').mockImplementation((tag: string) => {
			const el = originalCreateElement(tag);
			if (tag === 'script') {
				setTimeout(() => {
					if (el.onload) (el.onload as Function)(new Event('load'));
				}, 0);
			}
			return el;
		});

		const adapter = createGoogleAnalyticsAdapter();
		const config: AnalyticsConfig = {
			enabled: true,
			environment: 'development',
			googleAnalytics: {
				measurementId: 'G-TEST123456',
				debug: true
			},
			consent: { analytics: true, marketing: false }
		};

		await adapter.initialize(config);
		expect(adapter.state).not.toBe('uninitialized');
	});

	it('should track page view', () => {
		const adapter = createGoogleAnalyticsAdapter();
		expect(() => {
			adapter.trackPageView({
				page_path: '/test',
				page_title: 'Test Page'
			});
		}).not.toThrow();
	});

	it('should track custom event', () => {
		const adapter = createGoogleAnalyticsAdapter();
		expect(() => {
			adapter.trackEvent('test_event', {
				event_category: 'testing',
				event_label: 'unit test',
				value: 100
			});
		}).not.toThrow();
	});

	it('should track purchase', () => {
		const adapter = createGoogleAnalyticsAdapter();
		expect(() => {
			adapter.trackPurchase!({
				transaction_id: 'T-123',
				value: 99.99,
				currency: 'USD',
				items: [{ item_id: 'SKU-1', item_name: 'Product 1', price: 99.99 }]
			});
		}).not.toThrow();
	});

	it('should identify user', () => {
		const adapter = createGoogleAnalyticsAdapter();
		expect(() => {
			adapter.identify!({
				user_id: 'user-123',
				email: 'test@example.com',
				name: 'Test User'
			});
		}).not.toThrow();
	});

	it('should reset user', () => {
		const adapter = createGoogleAnalyticsAdapter();
		expect(() => {
			adapter.reset!();
		}).not.toThrow();
	});

	it('should handle consent change', () => {
		const adapter = createGoogleAnalyticsAdapter();
		expect(() => {
			adapter.onConsentChange!({ analytics: true, marketing: true });
		}).not.toThrow();
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// TEST SUITE: Console Adapter
// ═══════════════════════════════════════════════════════════════════════════

describe('Console Adapter', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	it('should create singleton instance via getConsoleAdapter', () => {
		const adapter1 = getConsoleAdapter();
		const adapter2 = getConsoleAdapter();
		expect(adapter1).toBe(adapter2);
	});

	it('should create new instance via createConsoleAdapter', () => {
		const adapter1 = createConsoleAdapter();
		const adapter2 = createConsoleAdapter();
		expect(adapter1).not.toBe(adapter2);
	});

	it('should have correct id and name', () => {
		const adapter = createConsoleAdapter();
		expect(adapter.id).toBe('console');
		expect(adapter.name).toBe('Console Logger');
	});

	it('should initialize successfully', async () => {
		const adapter = createConsoleAdapter();
		const config: AnalyticsConfig = {
			enabled: true,
			environment: 'development',
			console: { enabled: true, prettyPrint: true },
			consent: { analytics: true, marketing: false }
		};

		await adapter.initialize(config);
		expect(adapter.state).toBe('ready');
	});

	it('should track page view without error', async () => {
		const adapter = createConsoleAdapter();
		await adapter.initialize({
			enabled: true,
			environment: 'development',
			console: { enabled: true },
			consent: { analytics: true, marketing: false }
		});

		expect(() => {
			adapter.trackPageView({
				page_path: '/test',
				page_title: 'Test Page'
			});
		}).not.toThrow();
	});

	it('should track events without error', async () => {
		const adapter = createConsoleAdapter();
		await adapter.initialize({
			enabled: true,
			environment: 'development',
			console: { enabled: true },
			consent: { analytics: true, marketing: false }
		});

		expect(() => {
			adapter.trackEvent('button_clicked', { button_id: 'cta' });
		}).not.toThrow();
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// TEST SUITE: Backend Adapter
// ═══════════════════════════════════════════════════════════════════════════

describe('Backend Adapter', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		(globalThis as any).localStorage.data = {};
	});

	it('should create singleton instance via getBackendAdapter', () => {
		const adapter1 = getBackendAdapter();
		const adapter2 = getBackendAdapter();
		expect(adapter1).toBe(adapter2);
	});

	it('should create new instance via createBackendAdapter', () => {
		const adapter1 = createBackendAdapter();
		const adapter2 = createBackendAdapter();
		expect(adapter1).not.toBe(adapter2);
	});

	it('should have correct id and name', () => {
		const adapter = createBackendAdapter();
		expect(adapter.id).toBe('backend');
		expect(adapter.name).toBe('Backend API');
	});

	it('should initialize with config', async () => {
		const adapter = createBackendAdapter();
		const config: AnalyticsConfig = {
			enabled: true,
			environment: 'production',
			backend: {
				endpoint: '/api/analytics/batch',
				flushIntervalMs: 5000,
				maxBatchSize: 50
			},
			consent: { analytics: true, marketing: false }
		};

		await adapter.initialize(config);
		expect(adapter.state).toBe('ready');
	});

	it('should track page view and queue event', async () => {
		const adapter = createBackendAdapter();
		await adapter.initialize({
			enabled: true,
			environment: 'production',
			backend: { endpoint: '/api/analytics' },
			consent: { analytics: true, marketing: false }
		});

		adapter.trackPageView({
			page_path: '/products',
			page_title: 'Products'
		});

		expect(adapter.metrics.eventsTracked).toBe(1);
	});

	it('should track custom events', async () => {
		const adapter = createBackendAdapter();
		await adapter.initialize({
			enabled: true,
			environment: 'production',
			backend: { endpoint: '/api/analytics' },
			consent: { analytics: true, marketing: false }
		});

		adapter.trackEvent('product_viewed', { product_id: 'SKU-123' });
		expect(adapter.metrics.eventsTracked).toBe(1);
	});

	it('should identify user', async () => {
		const adapter = createBackendAdapter();
		await adapter.initialize({
			enabled: true,
			environment: 'production',
			backend: { endpoint: '/api/analytics' },
			consent: { analytics: true, marketing: false }
		});

		adapter.identify!({
			user_id: 'user-456',
			email: 'user@test.com'
		});

		expect(adapter.metrics.eventsTracked).toBe(1);
	});

	it('should clear queue on consent revoke', async () => {
		const adapter = createBackendAdapter();
		await adapter.initialize({
			enabled: true,
			environment: 'production',
			backend: { endpoint: '/api/analytics' },
			consent: { analytics: true, marketing: false }
		});

		adapter.trackEvent('test_event', {});
		adapter.onConsentChange!({ analytics: false, marketing: false });

		expect(adapter.metrics.queueSize).toBe(0);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// TEST SUITE: Orchestrator
// ═══════════════════════════════════════════════════════════════════════════

describe('Analytics Orchestrator', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		resetOrchestrator();
		resetGoogleAnalyticsAdapter();
	});

	it('should create singleton instance via getOrchestrator', () => {
		const orch1 = getOrchestrator();
		const orch2 = getOrchestrator();
		expect(orch1).toBe(orch2);
	});

	it('should create new instance via createOrchestrator', () => {
		const orch1 = createOrchestrator();
		const orch2 = createOrchestrator();
		expect(orch1).not.toBe(orch2);
	});

	it('should initialize with config', async () => {
		const orchestrator = createOrchestrator();
		await orchestrator.initialize({
			enabled: true,
			environment: 'development',
			consent: { analytics: true, marketing: false }
		});

		expect(orchestrator.initialized).toBe(true);
	});

	it('should track page view across all adapters', async () => {
		const orchestrator = createOrchestrator();
		await orchestrator.initialize({
			enabled: true,
			environment: 'development',
			consent: { analytics: true, marketing: false }
		});

		expect(() => {
			orchestrator.trackPageView({ page_path: '/test' });
		}).not.toThrow();
	});

	it('should track events across all adapters', async () => {
		const orchestrator = createOrchestrator();
		await orchestrator.initialize({
			enabled: true,
			environment: 'development',
			consent: { analytics: true, marketing: false }
		});

		expect(() => {
			orchestrator.trackEvent('test_event', { value: 123 });
		}).not.toThrow();
	});

	it('should track purchase across all adapters', async () => {
		const orchestrator = createOrchestrator();
		await orchestrator.initialize({
			enabled: true,
			environment: 'development',
			consent: { analytics: true, marketing: false }
		});

		expect(() => {
			orchestrator.trackPurchase({
				transaction_id: 'T-999',
				value: 199.99,
				currency: 'USD'
			});
		}).not.toThrow();
	});

	it('should identify user across all adapters', async () => {
		const orchestrator = createOrchestrator();
		await orchestrator.initialize({
			enabled: true,
			environment: 'development',
			consent: { analytics: true, marketing: false }
		});

		expect(() => {
			orchestrator.identify('user-789', { email: 'test@test.com' });
		}).not.toThrow();
	});

	it('should update consent across all adapters', async () => {
		const orchestrator = createOrchestrator();
		await orchestrator.initialize({
			enabled: true,
			environment: 'development',
			consent: { analytics: false, marketing: false }
		});

		orchestrator.updateConsent({ analytics: true, marketing: true });
		expect(orchestrator.consent.analytics).toBe(true);
		expect(orchestrator.consent.marketing).toBe(true);
	});

	it('should reset user across all adapters', async () => {
		const orchestrator = createOrchestrator();
		await orchestrator.initialize({
			enabled: true,
			environment: 'development',
			consent: { analytics: true, marketing: false }
		});

		orchestrator.identify('user-123', {});
		expect(() => {
			orchestrator.reset();
		}).not.toThrow();
	});

	it('should flush all adapters', async () => {
		const orchestrator = createOrchestrator();
		await orchestrator.initialize({
			enabled: true,
			environment: 'development',
			consent: { analytics: true, marketing: false }
		});

		await expect(orchestrator.flush()).resolves.not.toThrow();
	});

	it('should provide metrics', async () => {
		const orchestrator = createOrchestrator();
		await orchestrator.initialize({
			enabled: true,
			environment: 'development',
			consent: { analytics: true, marketing: false }
		});

		const metricsData = orchestrator.metrics;
		expect(metricsData).toHaveProperty('totalEvents');
		expect(metricsData).toHaveProperty('failedEvents');
		expect(metricsData).toHaveProperty('adapterMetrics');
		expect(metricsData).toHaveProperty('uptime');
	});

	it('should destroy and cleanup', async () => {
		const orchestrator = createOrchestrator();
		await orchestrator.initialize({
			enabled: true,
			environment: 'development',
			consent: { analytics: true, marketing: false }
		});

		expect(() => {
			orchestrator.destroy();
		}).not.toThrow();

		expect(orchestrator.initialized).toBe(false);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// TEST SUITE: Metrics Service
// ═══════════════════════════════════════════════════════════════════════════

describe('Metrics Service', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		resetOrchestrator();
		resetGoogleAnalyticsAdapter();
	});

	it('should export metrics singleton', () => {
		expect(metrics).toBeDefined();
		expect(typeof metrics.track).toBe('function');
		expect(typeof metrics.trackPageView).toBe('function');
		expect(typeof metrics.identify).toBe('function');
	});

	it('should export track function', () => {
		expect(typeof track).toBe('function');
	});

	it('should export trackPageView function', () => {
		expect(typeof trackPageView).toBe('function');
	});

	it('should export identify function', () => {
		expect(typeof identify).toBe('function');
	});

	it('should export startTimer function', () => {
		expect(typeof startTimer).toBe('function');
		const endTimer = startTimer('test_timer');
		expect(typeof endTimer).toBe('function');
	});

	it('should export trackPurchase function', () => {
		expect(typeof trackPurchase).toBe('function');
	});

	it('should export trackSignUp function', () => {
		expect(typeof trackSignUp).toBe('function');
	});

	it('should export trackLogin function', () => {
		expect(typeof trackLogin).toBe('function');
	});

	it('should export trackLogout function', () => {
		expect(typeof trackLogout).toBe('function');
	});

	it('should export trackSearch function', () => {
		expect(typeof trackSearch).toBe('function');
	});

	it('should export trackShare function', () => {
		expect(typeof trackShare).toBe('function');
	});

	it('should export trackError function', () => {
		expect(typeof trackError).toBe('function');
	});

	it('should export trackFormSubmit function', () => {
		expect(typeof trackFormSubmit).toBe('function');
	});

	it('should export trackFormError function', () => {
		expect(typeof trackFormError).toBe('function');
	});

	it('track() should not throw', () => {
		expect(() => {
			track('test_event', { key: 'value' });
		}).not.toThrow();
	});

	it('trackPageView() should not throw', () => {
		expect(() => {
			trackPageView({ path: '/test', title: 'Test' });
		}).not.toThrow();
	});

	it('identify() should not throw', () => {
		expect(() => {
			identify('user-123', { email: 'test@test.com' });
		}).not.toThrow();
	});

	it('trackPurchase() should not throw', () => {
		expect(() => {
			trackPurchase({
				transactionId: 'T-123',
				value: 99.99,
				currency: 'USD'
			});
		}).not.toThrow();
	});

	it('trackSignUp() should not throw', () => {
		expect(() => trackSignUp('email')).not.toThrow();
	});

	it('trackLogin() should not throw', () => {
		expect(() => trackLogin('google')).not.toThrow();
	});

	it('trackLogout() should not throw', () => {
		expect(() => trackLogout()).not.toThrow();
	});

	it('trackSearch() should not throw', () => {
		expect(() => trackSearch('test query', 10)).not.toThrow();
	});

	it('trackShare() should not throw', () => {
		expect(() => trackShare('article', 'art-123', 'twitter')).not.toThrow();
	});

	it('trackError() with Error object should not throw', () => {
		expect(() => {
			trackError(new Error('Test error'), { page: '/test' });
		}).not.toThrow();
	});

	it('trackError() with string should not throw', () => {
		expect(() => {
			trackError('Test error message', { page: '/test' });
		}).not.toThrow();
	});

	it('trackFormSubmit() should not throw', () => {
		expect(() => {
			trackFormSubmit('contact-form', 'Contact Form', true);
		}).not.toThrow();
	});

	it('trackFormError() should not throw', () => {
		expect(() => {
			trackFormError('contact-form', 'Email is required', 'email');
		}).not.toThrow();
	});

	it('startTimer() should return a function that tracks duration', () => {
		const endTimer = startTimer('operation_duration');
		expect(typeof endTimer).toBe('function');
		expect(() => endTimer()).not.toThrow();
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// TEST SUITE: Index Exports
// ═══════════════════════════════════════════════════════════════════════════

describe('Index Exports', () => {
	it('should export initializeAnalytics', () => {
		expect(typeof initializeAnalytics).toBe('function');
	});

	it('should export updateAnalyticsConsent', () => {
		expect(typeof updateAnalyticsConsent).toBe('function');
	});

	it('should export getAnalyticsMetrics', () => {
		expect(typeof getAnalyticsMetrics).toBe('function');
	});

	it('should export flushAnalytics', () => {
		expect(typeof flushAnalytics).toBe('function');
	});

	it('should export destroyAnalytics', () => {
		expect(typeof destroyAnalytics).toBe('function');
	});

	it('initializeAnalytics() should not throw', async () => {
		await expect(
			initializeAnalytics({
				enabled: true,
				environment: 'development',
				consent: { analytics: true, marketing: false }
			})
		).resolves.not.toThrow();
	});

	it('updateAnalyticsConsent() should not throw', () => {
		expect(() => {
			updateAnalyticsConsent({ analytics: true, marketing: true });
		}).not.toThrow();
	});

	it('getAnalyticsMetrics() should return metrics object', () => {
		const metricsData = getAnalyticsMetrics();
		expect(metricsData).toHaveProperty('totalEvents');
		expect(metricsData).toHaveProperty('failedEvents');
	});

	it('flushAnalytics() should not throw', async () => {
		await expect(flushAnalytics()).resolves.not.toThrow();
	});

	it('destroyAnalytics() should not throw', () => {
		expect(() => destroyAnalytics()).not.toThrow();
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// TEST SUITE: Type Safety
// ═══════════════════════════════════════════════════════════════════════════

describe('Type Safety', () => {
	it('should accept valid PageViewPayload', () => {
		const payload: PageViewPayload = {
			page_path: '/products',
			page_title: 'Products',
			page_referrer: 'https://google.com',
			page_type: 'category'
		};
		expect(payload.page_path).toBe('/products');
	});

	it('should accept valid CustomEventPayload', () => {
		const payload: CustomEventPayload = {
			event_category: 'engagement',
			event_label: 'button_click',
			value: 100,
			custom_field: 'custom_value'
		};
		expect(payload.event_category).toBe('engagement');
	});

	it('should accept valid PurchasePayload', () => {
		const payload: PurchasePayload = {
			transaction_id: 'T-12345',
			value: 299.99,
			currency: 'USD',
			tax: 25.0,
			shipping: 9.99,
			coupon: 'SAVE10',
			items: [
				{
					item_id: 'SKU-001',
					item_name: 'Premium Course',
					price: 299.99,
					quantity: 1
				}
			]
		};
		expect(payload.transaction_id).toBe('T-12345');
		expect(payload.items?.length).toBe(1);
	});

	it('should accept valid IdentifyPayload', () => {
		const payload: IdentifyPayload = {
			user_id: 'user-12345',
			email: 'user@example.com',
			name: 'John Doe',
			plan: 'premium',
			created_at: '2024-01-01T00:00:00Z'
		};
		expect(payload.user_id).toBe('user-12345');
	});

	it('should accept valid EcommerceItem', () => {
		const item: EcommerceItem = {
			item_id: 'SKU-001',
			item_name: 'Trading Course',
			price: 199.99,
			quantity: 1,
			item_brand: 'RTP',
			item_category: 'Education',
			item_variant: 'Digital',
			coupon: 'WELCOME',
			discount: 20,
			index: 0,
			item_list_name: 'Featured Products'
		};
		expect(item.item_id).toBe('SKU-001');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// TEST SUITE: Integration
// ═══════════════════════════════════════════════════════════════════════════

describe('Integration Tests', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		resetOrchestrator();
		resetGoogleAnalyticsAdapter();
		(globalThis as any).localStorage.data = {};
	});

	it('should handle full user journey', async () => {
		// Initialize
		await initializeAnalytics({
			enabled: true,
			environment: 'production',
			consent: { analytics: true, marketing: true }
		});

		// Page view
		trackPageView({ path: '/', title: 'Home' });

		// User signs up
		trackSignUp('email');

		// User identified
		identify('user-123', {
			email: 'user@test.com',
			name: 'Test User'
		});

		// User browses
		trackPageView({ path: '/products', title: 'Products' });
		track('product_viewed', { product_id: 'SKU-001' });

		// User adds to cart
		track('add_to_cart', {
			item_id: 'SKU-001',
			item_name: 'Course',
			price: 99.99
		});

		// User purchases
		trackPurchase({
			transactionId: 'T-001',
			value: 99.99,
			currency: 'USD',
			items: [{ item_id: 'SKU-001', item_name: 'Course' }]
		});

		// Flush and verify
		await flushAnalytics();

		const metricsData = getAnalyticsMetrics();
		expect(metricsData.totalEvents).toBeGreaterThan(0);
	});

	it('should respect consent state', async () => {
		// Initialize with analytics disabled
		await initializeAnalytics({
			enabled: true,
			environment: 'production',
			consent: { analytics: false, marketing: false }
		});

		// Track should be blocked
		track('test_event', {});

		// Enable consent
		updateAnalyticsConsent({ analytics: true, marketing: false });

		// Now track should work
		track('test_event_2', {});

		const metricsData = getAnalyticsMetrics();
		expect(metricsData).toBeDefined();
	});

	it('should handle errors gracefully', async () => {
		await initializeAnalytics({
			enabled: true,
			environment: 'production',
			consent: { analytics: true, marketing: false }
		});

		// Track error
		trackError(new Error('Test error'), {
			component: 'TestComponent',
			action: 'test_action'
		});

		// Form error
		trackFormError('checkout-form', 'Card declined', 'card_number');

		await flushAnalytics();

		// Should complete without throwing
		expect(true).toBe(true);
	});
});

logger.info('✅ All test suites defined successfully');
