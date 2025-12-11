/**
 * Revolution Trading Pros - Test Data Helper
 *
 * Provides utilities for managing test data:
 * - Unique identifier generation
 * - Mock data factories
 * - Data cleanup utilities
 * - Environment-aware data selection
 *
 * Netflix L11+ Pattern: Isolated, deterministic test data
 */

import { v4 as uuidv4 } from 'uuid';

/** Generates a unique ID for test isolation */
export function generateUniqueId(prefix: string = 'test'): string {
	return `${prefix}_${Date.now()}_${Math.random().toString(36).substring(2, 8)}`;
}

/** Generates a unique email for test user */
export function generateTestEmail(prefix: string = 'e2e'): string {
	return `${prefix}+${generateUniqueId()}@test.revolutiontradingpros.com`;
}

/** Generates a unique username */
export function generateTestUsername(prefix: string = 'e2e_user'): string {
	return `${prefix}_${Date.now()}`;
}

/** Standard test passwords (meeting common requirements) */
export const TEST_PASSWORDS = {
	valid: 'TestPassword123!',
	weak: 'password',
	noNumbers: 'TestPassword!',
	noSpecial: 'TestPassword123',
	tooShort: 'Test1!'
} as const;

/**
 * Test user factory
 */
export interface TestUser {
	email: string;
	password: string;
	name: string;
	firstName: string;
	lastName: string;
}

export function createTestUser(overrides: Partial<TestUser> = {}): TestUser {
	const id = generateUniqueId();
	const firstName = overrides.firstName || `Test`;
	const lastName = overrides.lastName || `User${id.substring(0, 6)}`;

	return {
		email: overrides.email || generateTestEmail(),
		password: overrides.password || TEST_PASSWORDS.valid,
		name: overrides.name || `${firstName} ${lastName}`,
		firstName,
		lastName
	};
}

/**
 * Test product factory
 */
export interface TestProduct {
	name: string;
	slug: string;
	price: number;
	description: string;
	type: 'course' | 'indicator' | 'subscription' | 'membership';
}

export function createTestProduct(overrides: Partial<TestProduct> = {}): TestProduct {
	const id = generateUniqueId('product');
	return {
		name: overrides.name || `Test Product ${id}`,
		slug: overrides.slug || `test-product-${id}`,
		price: overrides.price || 99.99,
		description: overrides.description || 'Test product description',
		type: overrides.type || 'course'
	};
}

/**
 * Test trading room factory
 */
export interface TestTradingRoom {
	name: string;
	slug: string;
	description: string;
	isActive: boolean;
}

export function createTestTradingRoom(overrides: Partial<TestTradingRoom> = {}): TestTradingRoom {
	const id = generateUniqueId('room');
	return {
		name: overrides.name || `Test Trading Room ${id}`,
		slug: overrides.slug || `test-room-${id}`,
		description: overrides.description || 'Test trading room description',
		isActive: overrides.isActive ?? true
	};
}

/**
 * Known test data for specific tests
 * These must exist in the test environment
 */
export const KNOWN_TEST_DATA = {
	tradingRooms: {
		dayTrading: {
			slug: 'day-trading',
			name: 'Day Trading Room'
		},
		swingTrading: {
			slug: 'swing-trading',
			name: 'Swing Trading Room'
		},
		smallAccounts: {
			slug: 'small-accounts',
			name: 'Small Accounts Room'
		}
	},
	courses: {
		dayTradingMasterclass: {
			slug: 'day-trading-masterclass',
			name: 'Day Trading Masterclass'
		},
		swingTradingPro: {
			slug: 'swing-trading-pro',
			name: 'Swing Trading Pro'
		},
		optionsTrading: {
			slug: 'options-trading',
			name: 'Options Trading'
		}
	},
	alerts: {
		spxProfitPulse: {
			slug: 'spx-profit-pulse',
			name: 'SPX Profit Pulse'
		},
		explosiveSwings: {
			slug: 'explosive-swings',
			name: 'Explosive Swings'
		}
	},
	indicators: {
		rsi: {
			slug: 'rsi',
			name: 'RSI Indicator'
		},
		macd: {
			slug: 'macd',
			name: 'MACD Indicator'
		}
	}
} as const;

/**
 * Test credit card data (Stripe test mode)
 */
export const TEST_CARDS = {
	success: {
		number: '4242424242424242',
		expMonth: '12',
		expYear: '2030',
		cvc: '123',
		zip: '10001'
	},
	declined: {
		number: '4000000000000002',
		expMonth: '12',
		expYear: '2030',
		cvc: '123',
		zip: '10001'
	},
	insufficientFunds: {
		number: '4000000000009995',
		expMonth: '12',
		expYear: '2030',
		cvc: '123',
		zip: '10001'
	},
	requires3ds: {
		number: '4000002500003155',
		expMonth: '12',
		expYear: '2030',
		cvc: '123',
		zip: '10001'
	}
} as const;

/**
 * Test addresses
 */
export const TEST_ADDRESSES = {
	us: {
		line1: '123 Test Street',
		line2: 'Suite 100',
		city: 'New York',
		state: 'NY',
		zip: '10001',
		country: 'US'
	},
	ca: {
		line1: '456 Test Avenue',
		line2: '',
		city: 'Toronto',
		state: 'ON',
		zip: 'M5V 3A8',
		country: 'CA'
	}
} as const;

/**
 * Generates random test data for stress testing
 */
export function generateRandomTestData(count: number): TestUser[] {
	return Array.from({ length: count }, () => createTestUser());
}

/**
 * Delay utility for rate limiting
 */
export function delay(ms: number): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

/**
 * Retry utility for flaky operations
 */
export async function retry<T>(
	fn: () => Promise<T>,
	options: { retries?: number; delay?: number; onRetry?: (attempt: number, error: unknown) => void } = {}
): Promise<T> {
	const { retries = 3, delay: delayMs = 1000, onRetry } = options;

	let lastError: unknown;

	for (let attempt = 1; attempt <= retries; attempt++) {
		try {
			return await fn();
		} catch (error) {
			lastError = error;
			if (attempt < retries) {
				onRetry?.(attempt, error);
				await delay(delayMs * attempt);
			}
		}
	}

	throw lastError;
}
