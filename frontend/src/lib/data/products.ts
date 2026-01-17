/**
 * Product Catalog
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Central product database for checkout routing.
 * Maps URL slugs to cart items for the dynamic checkout flow.
 *
 * @version 1.0.0 (December 2025)
 */

import type { CartItem } from '$lib/stores/cart.svelte';

export interface Product {
	id: string;
	slug: string;
	name: string;
	description: string;
	price: number;
	type: 'membership' | 'course' | 'alert-service';
	interval?: 'monthly' | 'quarterly' | 'yearly';
	image?: string;
	features?: string[];
	category: 'trading-room' | 'alert' | 'course' | 'indicator';
}

/**
 * All available products indexed by checkout slug
 */
export const products: Record<string, Product> = {
	// ═══════════════════════════════════════════════════════════════════════════
	// EXPLOSIVE SWINGS ALERT SERVICE
	// ═══════════════════════════════════════════════════════════════════════════
	'monthly-swings': {
		id: 'explosive-swings-monthly',
		slug: 'monthly-swings',
		name: 'Explosive Swings - Monthly',
		description: 'Premium multi-day swing trading alerts. 2-4 alerts per week.',
		price: 97,
		type: 'alert-service',
		interval: 'monthly',
		category: 'alert',
		features: [
			'2-4 Premium Swings / Week',
			'Instant SMS & Email Alerts',
			'Private Discord Community',
			'Detailed Technical Analysis'
		]
	},
	'annual-swings': {
		id: 'explosive-swings-annual',
		slug: 'annual-swings',
		name: 'Explosive Swings - Annual',
		description: 'Premium multi-day swing trading alerts. Save $237/year!',
		price: 927,
		type: 'alert-service',
		interval: 'yearly',
		category: 'alert',
		features: [
			'2-4 Premium Swings / Week',
			'Instant SMS & Email Alerts',
			'Private Discord Community',
			'Detailed Technical Analysis',
			'Strategy Video Library'
		]
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// SPX PROFIT PULSE ALERT SERVICE
	// ═══════════════════════════════════════════════════════════════════════════
	monthly: {
		id: 'spx-profit-pulse-monthly',
		slug: 'monthly',
		name: 'SPX Profit Pulse - Monthly',
		description: 'Daily SPX/SPY options alerts with precise entries and exits.',
		price: 147,
		type: 'alert-service',
		interval: 'monthly',
		category: 'alert',
		features: [
			'Daily SPX/SPY Alerts',
			'Real-time Entry/Exit Signals',
			'Private Discord Access',
			'Risk Management Guidance'
		]
	},
	quarterly: {
		id: 'spx-profit-pulse-quarterly',
		slug: 'quarterly',
		name: 'SPX Profit Pulse - Quarterly',
		description: 'Daily SPX/SPY options alerts. Save with quarterly billing.',
		price: 397,
		type: 'alert-service',
		interval: 'quarterly',
		category: 'alert',
		features: [
			'Daily SPX/SPY Alerts',
			'Real-time Entry/Exit Signals',
			'Private Discord Access',
			'Risk Management Guidance'
		]
	},
	annual: {
		id: 'spx-profit-pulse-annual',
		slug: 'annual',
		name: 'SPX Profit Pulse - Annual',
		description: 'Daily SPX/SPY options alerts. Best value - save $367/year!',
		price: 1397,
		type: 'alert-service',
		interval: 'yearly',
		category: 'alert',
		features: [
			'Daily SPX/SPY Alerts',
			'Real-time Entry/Exit Signals',
			'Private Discord Access',
			'Risk Management Guidance',
			'1-on-1 Strategy Session'
		]
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// LIVE TRADING ROOMS
	// ═══════════════════════════════════════════════════════════════════════════
	'monthly-room': {
		id: 'trading-room-monthly',
		slug: 'monthly-room',
		name: 'Live Trading Room - Monthly',
		description: 'Full access to live trading sessions with professional traders.',
		price: 197,
		type: 'membership',
		interval: 'monthly',
		category: 'trading-room',
		features: [
			'Live Trading Sessions (M-F)',
			'Real-time Trade Alerts',
			'Screen Share Access',
			'Community Chat'
		]
	},
	'quarterly-room': {
		id: 'trading-room-quarterly',
		slug: 'quarterly-room',
		name: 'Live Trading Room - Quarterly',
		description: 'Full access to live trading sessions. Save with quarterly billing.',
		price: 497,
		type: 'membership',
		interval: 'quarterly',
		category: 'trading-room',
		features: [
			'Live Trading Sessions (M-F)',
			'Real-time Trade Alerts',
			'Screen Share Access',
			'Community Chat',
			'Trade Replay Archive'
		]
	},
	'annual-room': {
		id: 'trading-room-annual',
		slug: 'annual-room',
		name: 'Live Trading Room - Annual',
		description: 'Full access to live trading sessions. Best value!',
		price: 1497,
		type: 'membership',
		interval: 'yearly',
		category: 'trading-room',
		features: [
			'Live Trading Sessions (M-F)',
			'Real-time Trade Alerts',
			'Screen Share Access',
			'Community Chat',
			'Trade Replay Archive',
			'1-on-1 Coaching Session'
		]
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// DAY TRADING ROOM
	// ═══════════════════════════════════════════════════════════════════════════
	'day-trading-monthly': {
		id: 'day-trading-room-monthly',
		slug: 'day-trading-monthly',
		name: 'Day Trading Room - Monthly',
		description: 'Live day trading sessions focusing on intraday opportunities.',
		price: 247,
		type: 'membership',
		interval: 'monthly',
		category: 'trading-room',
		features: [
			'Live Day Trading Sessions',
			'Intraday Alerts',
			'Market Open Analysis',
			'Private Discord'
		]
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// SWING TRADING ROOM
	// ═══════════════════════════════════════════════════════════════════════════
	'swing-trading-monthly': {
		id: 'swing-trading-room-monthly',
		slug: 'swing-trading-monthly',
		name: 'Swing Trading Room - Monthly',
		description: 'Live swing trading sessions for multi-day opportunities.',
		price: 197,
		type: 'membership',
		interval: 'monthly',
		category: 'trading-room',
		features: [
			'Weekly Swing Trade Ideas',
			'Technical Analysis Sessions',
			'Position Management',
			'Private Discord'
		]
	},

	// ═══════════════════════════════════════════════════════════════════════════
	// SMALL ACCOUNTS ROOM
	// ═══════════════════════════════════════════════════════════════════════════
	'small-accounts-monthly': {
		id: 'small-accounts-room-monthly',
		slug: 'small-accounts-monthly',
		name: 'Small Accounts Room - Monthly',
		description: 'Specialized strategies for growing small trading accounts.',
		price: 147,
		type: 'membership',
		interval: 'monthly',
		category: 'trading-room',
		features: [
			'Small Account Strategies',
			'Risk-Focused Trades',
			'Live Trading Sessions',
			'Account Growth Plans'
		]
	}
};

/**
 * Get product by checkout slug
 */
export function getProductBySlug(slug: string): Product | undefined {
	return products[slug];
}

/**
 * Convert product to cart item
 */
export function productToCartItem(product: Product): Omit<CartItem, 'quantity'> {
	return {
		id: product.id,
		name: product.name,
		description: product.description,
		price: product.price,
		type: product.type,
		interval: product.interval,
		image: product.image
	};
}

/**
 * Get all products by category
 */
export function getProductsByCategory(category: Product['category']): Product[] {
	return Object.values(products).filter((p) => p.category === category);
}
