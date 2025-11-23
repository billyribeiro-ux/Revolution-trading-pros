import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';

export interface CartItem {
	id: string;
	name: string;
	description?: string;
	price: number;
	type: 'membership' | 'course' | 'alert-service';
	image?: string;
	quantity: number;
	interval?: 'monthly' | 'quarterly' | 'yearly'; // For subscriptions
	couponCode?: string; // Applied coupon code
	discount?: number; // Discount amount
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
		 * Add item to cart
		 */
		addItem: (item: Omit<CartItem, 'quantity'>) => {
			update((state) => {
				// Check if item already exists
				const existingIndex = state.items.findIndex(
					(i) => i.id === item.id && i.interval === item.interval
				);

				if (existingIndex >= 0) {
					// Increase quantity for existing item
					state.items[existingIndex].quantity += 1;
				} else {
					// Add new item with quantity 1
					state.items.push({ ...item, quantity: 1 });
				}

				return state;
			});
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
		 * Update item quantity
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
						item.quantity = quantity;
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
