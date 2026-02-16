/**
 * Consent Banner A/B Testing Module
 *
 * Test different banner designs to optimize consent rates:
 * - Different layouts (bottom bar, modal, corner)
 * - Different copy (friendly, formal, minimal)
 * - Different button styles
 * - Different color schemes
 *
 * @module consent/ab-testing
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import { writable, get } from 'svelte/store';
import { logger } from '$lib/utils/logger';

/**
 * Banner variant types
 */
export type BannerLayout = 'bottom-bar' | 'modal' | 'corner-popup' | 'top-bar' | 'full-screen';
export type BannerTone = 'friendly' | 'formal' | 'minimal' | 'urgent';
export type ButtonStyle = 'filled' | 'outline' | 'gradient' | 'rounded';
export type ColorScheme = 'dark' | 'light' | 'brand' | 'neutral';

/**
 * A/B test variant configuration
 */
export interface BannerVariant {
	id: string;
	name: string;
	layout: BannerLayout;
	tone: BannerTone;
	buttonStyle: ButtonStyle;
	colorScheme: ColorScheme;
	weight: number; // Probability weight (higher = more likely)
	customCopy?: {
		title?: string;
		description?: string;
		acceptButton?: string;
		rejectButton?: string;
	};
	customStyles?: Record<string, string>;
}

/**
 * A/B test result
 */
export interface ABTestResult {
	variantId: string;
	action: 'accept_all' | 'reject_all' | 'customize' | 'dismiss' | 'ignore';
	timeToDecision: number; // ms from banner shown to decision
	timestamp: string;
	sessionId: string;
}

/**
 * A/B test analytics
 */
export interface ABTestAnalytics {
	variantId: string;
	impressions: number;
	acceptRate: number;
	rejectRate: number;
	customizeRate: number;
	avgTimeToDecision: number;
}

/**
 * Default variants for testing
 */
export const DEFAULT_VARIANTS: BannerVariant[] = [
	{
		id: 'control',
		name: 'Control (Default)',
		layout: 'bottom-bar',
		tone: 'friendly',
		buttonStyle: 'gradient',
		colorScheme: 'dark',
		weight: 50
	},
	{
		id: 'minimal',
		name: 'Minimal Design',
		layout: 'corner-popup',
		tone: 'minimal',
		buttonStyle: 'outline',
		colorScheme: 'dark',
		weight: 25,
		customCopy: {
			title: 'Cookies',
			description: 'We use cookies to improve your experience.',
			acceptButton: 'OK',
			rejectButton: 'No'
		}
	},
	{
		id: 'formal',
		name: 'Formal/Legal',
		layout: 'bottom-bar',
		tone: 'formal',
		buttonStyle: 'filled',
		colorScheme: 'neutral',
		weight: 25,
		customCopy: {
			title: 'Cookie Notice',
			description:
				'This website uses cookies and similar technologies to enhance your browsing experience, analyze traffic, and provide personalized content. By continuing to use this site, you consent to our use of cookies in accordance with our Privacy Policy.'
		}
	}
];

/**
 * Current variant store
 */
export const currentVariant = writable<BannerVariant | null>(null);

/**
 * Test results store
 */
const testResults = writable<ABTestResult[]>([]);

/**
 * Storage key for variant assignment
 */
const VARIANT_STORAGE_KEY = 'rtp_ab_variant';
const RESULTS_STORAGE_KEY = 'rtp_ab_results';

/**
 * Banner shown timestamp (for time-to-decision calculation)
 */
let bannerShownAt: number | null = null;

/**
 * Generate session ID
 */
function getSessionId(): string {
	if (!browser) return '';

	let sessionId = sessionStorage.getItem('rtp_ab_session');
	if (!sessionId) {
		sessionId = `ab_${Date.now().toString(36)}_${Math.random().toString(36).substring(2, 8)}`;
		sessionStorage.setItem('rtp_ab_session', sessionId);
	}
	return sessionId;
}

/**
 * Select a variant based on weights
 */
function selectVariant(variants: BannerVariant[]): BannerVariant {
	const totalWeight = variants.reduce((sum, v) => sum + v.weight, 0);
	let random = Math.random() * totalWeight;

	for (const variant of variants) {
		random -= variant.weight;
		if (random <= 0) {
			return variant;
		}
	}

	const firstVariant = variants[0];
	if (!firstVariant) throw new Error('No variants provided');
	return firstVariant;
}

/**
 * Initialize A/B testing
 */
export function initializeABTest(
	variants: BannerVariant[] = DEFAULT_VARIANTS,
	forceVariant?: string
): BannerVariant {
	if (!browser) {
		const firstVariant = variants[0];
		if (!firstVariant) throw new Error('No variants provided');
		return firstVariant;
	}

	// Check for forced variant (for testing)
	if (forceVariant) {
		const forced = variants.find((v) => v.id === forceVariant);
		if (forced) {
			currentVariant.set(forced);
			return forced;
		}
	}

	// Check for existing assignment
	const storedVariantId = localStorage.getItem(VARIANT_STORAGE_KEY);
	if (storedVariantId) {
		const stored = variants.find((v) => v.id === storedVariantId);
		if (stored) {
			currentVariant.set(stored);
			return stored;
		}
	}

	// Assign new variant
	const variant = selectVariant(variants);
	localStorage.setItem(VARIANT_STORAGE_KEY, variant.id);
	currentVariant.set(variant);

	logger.debug('[ABTest] Assigned variant:', variant.id);

	return variant;
}

/**
 * Record banner impression
 */
export function recordImpression(): void {
	if (!browser) return;

	bannerShownAt = Date.now();

	// Load existing results
	loadResults();

	logger.debug('[ABTest] Banner impression recorded');
}

/**
 * Record user decision
 */
export function recordDecision(action: ABTestResult['action']): void {
	if (!browser) return;

	const variant = get(currentVariant);
	if (!variant) return;

	const result: ABTestResult = {
		variantId: variant.id,
		action,
		timeToDecision: bannerShownAt ? Date.now() - bannerShownAt : 0,
		timestamp: new Date().toISOString(),
		sessionId: getSessionId()
	};

	testResults.update((results) => [...results, result]);

	// Save to localStorage
	saveResults();

	logger.debug('[ABTest] Decision recorded:', result);
}

/**
 * Load results from storage
 */
function loadResults(): void {
	if (!browser) return;

	try {
		const stored = localStorage.getItem(RESULTS_STORAGE_KEY);
		if (stored) {
			testResults.set(JSON.parse(stored));
		}
	} catch (e) {
		logger.debug('[ABTest] Failed to load results:', e);
	}
}

/**
 * Save results to storage
 */
function saveResults(): void {
	if (!browser) return;

	const results = get(testResults);
	localStorage.setItem(RESULTS_STORAGE_KEY, JSON.stringify(results));
}

/**
 * Get analytics for all variants
 */
export function getABTestAnalytics(): ABTestAnalytics[] {
	const results = get(testResults);
	const variantMap = new Map<string, ABTestResult[]>();

	// Group by variant
	for (const result of results) {
		const existing = variantMap.get(result.variantId) || [];
		existing.push(result);
		variantMap.set(result.variantId, existing);
	}

	// Calculate analytics
	const analytics: ABTestAnalytics[] = [];

	for (const [variantId, variantResults] of variantMap) {
		const impressions = variantResults.length;
		const accepts = variantResults.filter((r) => r.action === 'accept_all').length;
		const rejects = variantResults.filter((r) => r.action === 'reject_all').length;
		const customizes = variantResults.filter((r) => r.action === 'customize').length;
		const totalTimeToDecision = variantResults.reduce((sum, r) => sum + r.timeToDecision, 0);

		analytics.push({
			variantId,
			impressions,
			acceptRate: impressions > 0 ? accepts / impressions : 0,
			rejectRate: impressions > 0 ? rejects / impressions : 0,
			customizeRate: impressions > 0 ? customizes / impressions : 0,
			avgTimeToDecision: impressions > 0 ? totalTimeToDecision / impressions : 0
		});
	}

	return analytics;
}

/**
 * Get best performing variant
 */
export function getBestVariant(
	metric: 'acceptRate' | 'avgTimeToDecision' = 'acceptRate'
): string | null {
	const analytics = getABTestAnalytics();

	if (analytics.length === 0) return null;

	if (metric === 'acceptRate') {
		// Higher is better
		return analytics.reduce((best, current) =>
			current.acceptRate > best.acceptRate ? current : best
		).variantId;
	} else {
		// Lower is better (faster decision)
		return analytics.reduce((best, current) =>
			current.avgTimeToDecision < best.avgTimeToDecision ? current : best
		).variantId;
	}
}

/**
 * Clear A/B test data
 */
export function clearABTestData(): void {
	if (!browser) return;

	localStorage.removeItem(VARIANT_STORAGE_KEY);
	localStorage.removeItem(RESULTS_STORAGE_KEY);
	sessionStorage.removeItem('rtp_ab_session');
	currentVariant.set(null);
	testResults.set([]);
}

/**
 * Export A/B test data for analysis
 */
export function exportABTestData(): string {
	return JSON.stringify(
		{
			currentVariant: get(currentVariant),
			results: get(testResults),
			analytics: getABTestAnalytics(),
			exportedAt: new Date().toISOString()
		},
		null,
		2
	);
}

/**
 * Get variant-specific styles
 */
export function getVariantStyles(variant: BannerVariant): Record<string, string> {
	const baseStyles: Record<string, string> = {};

	// Layout styles
	switch (variant.layout) {
		case 'bottom-bar':
			baseStyles['--banner-position'] = 'fixed';
			baseStyles['--banner-bottom'] = '0';
			baseStyles['--banner-left'] = '0';
			baseStyles['--banner-right'] = '0';
			baseStyles['--banner-border-radius'] = '0';
			break;
		case 'top-bar':
			baseStyles['--banner-position'] = 'fixed';
			baseStyles['--banner-top'] = '0';
			baseStyles['--banner-left'] = '0';
			baseStyles['--banner-right'] = '0';
			baseStyles['--banner-border-radius'] = '0';
			break;
		case 'corner-popup':
			baseStyles['--banner-position'] = 'fixed';
			baseStyles['--banner-bottom'] = '1rem';
			baseStyles['--banner-right'] = '1rem';
			baseStyles['--banner-max-width'] = '400px';
			baseStyles['--banner-border-radius'] = '12px';
			break;
		case 'modal':
			baseStyles['--banner-position'] = 'fixed';
			baseStyles['--banner-inset'] = '0';
			baseStyles['--banner-display'] = 'flex';
			baseStyles['--banner-align-items'] = 'center';
			baseStyles['--banner-justify-content'] = 'center';
			break;
		case 'full-screen':
			baseStyles['--banner-position'] = 'fixed';
			baseStyles['--banner-inset'] = '0';
			break;
	}

	// Color scheme styles
	switch (variant.colorScheme) {
		case 'dark':
			baseStyles['--banner-bg'] = 'rgba(10, 16, 28, 0.98)';
			baseStyles['--banner-text'] = '#e2e8f0';
			break;
		case 'light':
			baseStyles['--banner-bg'] = 'rgba(255, 255, 255, 0.98)';
			baseStyles['--banner-text'] = '#1e293b';
			break;
		case 'brand':
			baseStyles['--banner-bg'] = 'linear-gradient(135deg, #0ea5e9, #06b6d4)';
			baseStyles['--banner-text'] = '#ffffff';
			break;
		case 'neutral':
			baseStyles['--banner-bg'] = 'rgba(51, 65, 85, 0.98)';
			baseStyles['--banner-text'] = '#f1f5f9';
			break;
	}

	// Button style
	switch (variant.buttonStyle) {
		case 'gradient':
			baseStyles['--button-bg'] = 'linear-gradient(135deg, #0ea5e9, #06b6d4)';
			break;
		case 'filled':
			baseStyles['--button-bg'] = '#0ea5e9';
			break;
		case 'outline':
			baseStyles['--button-bg'] = 'transparent';
			baseStyles['--button-border'] = '2px solid #0ea5e9';
			break;
		case 'rounded':
			baseStyles['--button-border-radius'] = '9999px';
			break;
	}

	// Apply custom styles
	if (variant.customStyles) {
		Object.assign(baseStyles, variant.customStyles);
	}

	return baseStyles;
}
