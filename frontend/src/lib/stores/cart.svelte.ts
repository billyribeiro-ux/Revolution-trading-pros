/**
 * Cart Store - Svelte 5 Runes Migration
 * ═══════════════════════════════════════════════════════════════════════════
 */

import { browser } from '$app/environment';

export interface CartItem {
	id: string;
	name: string;
	description?: string;
	price: number;
	type:
		| 'membership'
		| 'course'
		| 'alert-service'
		| 'indicator'
		| 'trading-room'
		| 'weekly-watchlist'
		| 'premium-report';
	image?: string;
	thumbnail?: string; // Product thumbnail URL
	quantity: number;
	interval?: 'monthly' | 'quarterly' | 'yearly' | 'lifetime'; // For subscriptions
	couponCode?: string; // Applied coupon code
	discount?: number; // Discount amount
	productSlug?: string; // Unique product identifier
}

/**
 * Subscription conflict types for user feedback
 */
export type SubscriptionConflictType =
	| 'already_subscribed' // User has exact same subscription
	| 'upgrade' // User is upgrading (e.g., monthly to quarterly/yearly)
	| 'downgrade' // User is downgrading (e.g., yearly to monthly)
	| 'already_owned' // User already owns this product (course/indicator)
	| null; // No conflict

export interface SubscriptionConflict {
	type: SubscriptionConflictType;
	currentInterval?: 'monthly' | 'quarterly' | 'yearly';
	newInterval?: 'monthly' | 'quarterly' | 'yearly';
	productName: string;
	message: string;
}

/**
 * Interval priority for upgrade/downgrade detection
 */
const INTERVAL_PRIORITY: Record<string, number> = {
	monthly: 1,
	quarterly: 2,
	yearly: 3
};

/**
 * Check if changing from one interval to another is an upgrade
 */
export function isUpgrade(
	currentInterval: 'monthly' | 'quarterly' | 'yearly',
	newInterval: 'monthly' | 'quarterly' | 'yearly'
): boolean {
	return (INTERVAL_PRIORITY[newInterval] ?? 0) > (INTERVAL_PRIORITY[currentInterval] ?? 0);
}

/**
 * Get a human-readable interval label
 */
export function getIntervalDisplayName(interval: string): string {
	switch (interval) {
		case 'monthly':
			return 'Monthly';
		case 'quarterly':
			return 'Quarterly';
		case 'yearly':
			return 'Annual';
		default:
			return interval;
	}
}

interface CartState {
	items: CartItem[];
}

const CART_STORAGE_KEY = 'revolution_cart';

// Load cart from localStorage
function loadCart(): CartState {
	if (!browser) {
		return { items: [] };
	}

	try {
		const stored = localStorage.getItem(CART_STORAGE_KEY);
		if (stored) {
			return JSON.parse(stored);
		}
	} catch (error) {
		console.error('Error loading cart from localStorage:', error);
	}

	return { items: [] };
}

// ═══════════════════════════════════════════════════════════════════════════
// Svelte 5 Runes State
// ═══════════════════════════════════════════════════════════════════════════

let cartState = $state<CartState>(loadCart());

// Save to localStorage whenever cart changes (skip initial run to prevent hydration issues)
if (browser) {
	$effect.root(() => {
		let isFirstRun = true;
		$effect(() => {
			// Read the state to create dependency
			const state = cartState;
			// Skip initial run to prevent infinite loop during hydration
			if (isFirstRun) {
				isFirstRun = false;
				return;
			}
			try {
				localStorage.setItem(CART_STORAGE_KEY, JSON.stringify(state));
			} catch (error) {
				console.error('Error saving cart to localStorage:', error);
			}
		});
	});
}

// ═══════════════════════════════════════════════════════════════════════════
// Cart Store API
// ═══════════════════════════════════════════════════════════════════════════

export const cartStore = {
	get state() {
		return cartState;
	},

	get items() {
		return cartState.items;
	},

	/**
	 * Add item to cart - Limited to quantity of 1 per product/service
	 * Returns true if item was added, false if already in cart
	 */
	addItem(item: Omit<CartItem, 'quantity'>): boolean {
		// Check if exact item already exists (same ID and interval)
		const existingIndex = cartState.items.findIndex(
			(i) => i.id === item.id && i.interval === item.interval
		);

		if (existingIndex >= 0) {
			// Item already in cart - DO NOT increase quantity (max 1 per item)
			return false;
		}

		// Add new item with quantity 1 (always 1, never more)
		cartState.items = [...cartState.items, { ...item, quantity: 1 }];
		return true;
	},

	/**
	 * Check if an item is already in the cart
	 */
	hasItem(itemId: string, interval?: 'monthly' | 'quarterly' | 'yearly'): boolean {
		return cartState.items.some((i) => i.id === itemId && i.interval === interval);
	},

	/**
	 * Check if user has any subscription variant of a product in cart
	 */
	hasAnyVariant(itemId: string): CartItem | undefined {
		return cartState.items.find((i) => i.id === itemId);
	},

	/**
	 * Remove item from cart
	 */
	removeItem(itemId: string, interval?: 'monthly' | 'quarterly' | 'yearly' | 'lifetime') {
		cartState.items = cartState.items.filter(
			(item) => !(item.id === itemId && item.interval === interval)
		);
	},

	/**
	 * Update item quantity - Enforces max quantity of 1
	 */
	updateQuantity(itemId: string, quantity: number, interval?: 'monthly' | 'quarterly' | 'yearly') {
		const itemIndex = cartState.items.findIndex((i) => i.id === itemId && i.interval === interval);

		if (itemIndex >= 0) {
			if (quantity <= 0) {
				// Remove item if quantity is 0 or less
				cartState.items = cartState.items.filter(
					(i) => !(i.id === itemId && i.interval === interval)
				);
			} else {
				// Enforce max quantity of 1
				cartState.items = cartState.items.map((item, index) =>
					index === itemIndex ? { ...item, quantity: Math.min(quantity, 1) } : item
				);
			}
		}
	},

	/**
	 * Apply coupon to cart
	 */
	applyCoupon(couponCode: string, discount: number) {
		cartState.items = cartState.items.map((item) => ({
			...item,
			couponCode,
			discount
		}));
	},

	/**
	 * Remove coupon from cart
	 */
	removeCoupon() {
		cartState.items = cartState.items.map((item) => {
			const { couponCode: _couponCode, discount: _discount, ...rest } = item;
			return rest as CartItem;
		});
	},

	/**
	 * Clear entire cart
	 */
	clearCart() {
		cartState = { items: [] };
	},

	/**
	 * Get item count
	 */
	getItemCount(): number {
		return cartState.items.reduce((total, item) => total + item.quantity, 0);
	},

	/**
	 * Get cart total
	 */
	getTotal(): number {
		return cartState.items.reduce((total, item) => total + item.price * item.quantity, 0);
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// Getter Functions (Svelte 5 - cannot export $derived from modules)
// ═══════════════════════════════════════════════════════════════════════════
export function getCartItemCount() {
	return cartState.items.reduce((total, item) => total + item.quantity, 0);
}

export function getCartTotal() {
	return cartState.items.reduce((total, item) => total + item.price * item.quantity, 0);
}

export function getHasCartItems() {
	return cartState.items.length > 0;
}

/**
 * Helper function to add a product to cart
 * Simplified API for adding products from product pages
 */
export async function addToCart(params: {
	productId: string;
	productName: string;
	productType: CartItem['type'];
	price: number;
	interval?: CartItem['interval'];
	quantity?: number;
	description?: string;
	image?: string;
	productSlug?: string;
}): Promise<boolean> {
	const item: Omit<CartItem, 'quantity'> = {
		id: params.productId,
		name: params.productName,
		type: params.productType,
		price: params.price,
		interval: params.interval,
		description: params.description,
		image: params.image,
		productSlug: params.productSlug || params.productId
	};

	return cartStore.addItem(item);
}
