import { writable, derived, get } from 'svelte/store';
import { browser } from '$app/environment';

export interface CartItem {
	id: string;
	name: string;
	description?: string;
	price: number;
	type: 'membership' | 'course' | 'alert-service' | 'indicator';
	image?: string;
	thumbnail?: string; // Product thumbnail URL
	quantity: number;
	interval?: 'monthly' | 'quarterly' | 'yearly'; // For subscriptions
	couponCode?: string; // Applied coupon code
	discount?: number; // Discount amount
	productSlug?: string; // Unique product identifier
}

/**
 * Subscription conflict types for user feedback
 */
export type SubscriptionConflictType =
	| 'already_subscribed'  // User has exact same subscription
	| 'upgrade'             // User is upgrading (e.g., monthly to quarterly/yearly)
	| 'downgrade'           // User is downgrading (e.g., yearly to monthly)
	| 'already_owned'       // User already owns this product (course/indicator)
	| null;                 // No conflict

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
	return INTERVAL_PRIORITY[newInterval] > INTERVAL_PRIORITY[currentInterval];
}

/**
 * Get a human-readable interval label
 */
export function getIntervalDisplayName(interval: string): string {
	switch (interval) {
		case 'monthly': return 'Monthly';
		case 'quarterly': return 'Quarterly';
		case 'yearly': return 'Annual';
		default: return interval;
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

// Create the cart store
function createCartStore() {
	const { subscribe, set, update } = writable<CartState>(loadCart());

	// Save to localStorage whenever cart changes
	if (browser) {
		subscribe((state) => {
			try {
				localStorage.setItem(CART_STORAGE_KEY, JSON.stringify(state));
			} catch (error) {
				console.error('Error saving cart to localStorage:', error);
			}
		});
	}

	return {
		subscribe,

		/**
		 * Add item to cart - Limited to quantity of 1 per product/service
		 * Returns true if item was added, false if already in cart
		 */
		addItem: (item: Omit<CartItem, 'quantity'>): boolean => {
			let added = false;
			update((state) => {
				// Check if exact item already exists (same ID and interval)
				const existingIndex = state.items.findIndex(
					(i) => i.id === item.id && i.interval === item.interval
				);

				if (existingIndex >= 0) {
					// Item already in cart - DO NOT increase quantity (max 1 per item)
					added = false;
				} else {
					// Add new item with quantity 1 (always 1, never more)
					state.items.push({ ...item, quantity: 1 });
					added = true;
				}

				return state;
			});
			return added;
		},

		/**
		 * Check if an item is already in the cart
		 */
		hasItem: (itemId: string, interval?: 'monthly' | 'quarterly' | 'yearly'): boolean => {
			const state = get({ subscribe });
			return state.items.some((i) => i.id === itemId && i.interval === interval);
		},

		/**
		 * Check if user has any subscription variant of a product in cart
		 */
		hasAnyVariant: (itemId: string): CartItem | undefined => {
			const state = get({ subscribe });
			return state.items.find((i) => i.id === itemId);
		},

		/**
		 * Remove item from cart
		 */
		removeItem: (itemId: string, interval?: 'monthly' | 'quarterly' | 'yearly') => {
			update((state) => {
				state.items = state.items.filter(
					(item) => !(item.id === itemId && item.interval === interval)
				);
				return state;
			});
		},

		/**
		 * Update item quantity - Enforces max quantity of 1
		 */
		updateQuantity: (
			itemId: string,
			quantity: number,
			interval?: 'monthly' | 'quarterly' | 'yearly'
		) => {
			update((state) => {
				const item = state.items.find((i) => i.id === itemId && i.interval === interval);

				if (item) {
					if (quantity <= 0) {
						// Remove item if quantity is 0 or less
						state.items = state.items.filter((i) => !(i.id === itemId && i.interval === interval));
					} else {
						// Enforce max quantity of 1
						item.quantity = Math.min(quantity, 1);
					}
				}

				return state;
			});
		},

		/**
		 * Apply coupon to cart
		 */
		applyCoupon: (couponCode: string, discount: number) => {
			update((state) => {
				state.items = state.items.map((item) => ({
					...item,
					couponCode,
					discount
				}));
				return state;
			});
		},

		/**
		 * Remove coupon from cart
		 */
		removeCoupon: () => {
			update((state) => {
				state.items = state.items.map((item) => {
					const { couponCode, discount, ...rest } = item;
					return rest as CartItem;
				});
				return state;
			});
		},

		/**
		 * Clear entire cart
		 */
		clearCart: () => {
			set({ items: [] });
		},

		/**
		 * Get item count
		 */
		getItemCount: (state: CartState): number => {
			return state.items.reduce((total, item) => total + item.quantity, 0);
		},

		/**
		 * Get cart total
		 */
		getTotal: (state: CartState): number => {
			return state.items.reduce((total, item) => total + item.price * item.quantity, 0);
		}
	};
}

export const cartStore = createCartStore();

// Derived stores for computed values
export const cartItemCount = derived(cartStore, ($cart) =>
	$cart.items.reduce((total, item) => total + item.quantity, 0)
);

export const cartTotal = derived(cartStore, ($cart) =>
	$cart.items.reduce((total, item) => total + item.price * item.quantity, 0)
);

export const hasCartItems = derived(cartItemCount, ($count) => $count > 0);
