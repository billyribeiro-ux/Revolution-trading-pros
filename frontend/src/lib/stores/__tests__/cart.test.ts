/**
 * Cart store + helpers — Unit Tests (R28-D)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * `src/lib/stores/cart.svelte.ts` owns the local-only cart state machine and
 * the price/quantity contract the checkout page depends on. These tests pin
 * invariants that are easy to break in a "just one line" refactor:
 *
 *   1. **Max quantity per line is 1.**  `addItem`, `updateQuantity`, and the
 *      product+interval pairing are all designed around "one of each
 *      membership/course/etc." A regression that lets a user add the same
 *      membership twice would double-charge them at checkout — silent money
 *      bug, fail loud.
 *
 *   2. **Same `id` + DIFFERENT `interval` = two separate line items.**  The
 *      cart treats `{id, interval}` as the composite key — a user can hold
 *      a monthly and a yearly variant simultaneously while we resolve the
 *      conflict before checkout. Losing this property collapses upgrades.
 *
 *   3. **`addItem` returns true on add, false on duplicate.**  The product
 *      pages key their "Added to cart" UI off this boolean. Returning
 *      `void` (the natural "simplification") breaks every Add-to-Cart CTA.
 *
 *   4. **`getTotal` math: `sum(price * quantity)`.**  With `quantity` capped
 *      at 1, the sum equals `sum(price)`. But the formula must stay
 *      explicit so a future `quantity > 1` lift (e.g. for printable
 *      merchandise) keeps the right math.
 *
 *   5. **Coupon apply/remove is line-level, not cart-level.**  `applyCoupon`
 *      writes `couponCode` + `discount` onto every line item; `removeCoupon`
 *      strips them with destructure. A single-line cart-total field would
 *      lose the per-product discount audit trail.
 *
 *   6. **`clearCart` resets to `{ items: [] }`.**  Not "drain in place" —
 *      a fresh reference matters for the `$state` rune to fire its
 *      reactive consumers downstream.
 *
 *   7. **`isUpgrade` priority is monthly(1) < quarterly(2) < yearly(3).**
 *      The conflict UI ("you're upgrading" vs "downgrading") reads this
 *      directly. A swap or a missing interval bucket flips the UX.
 *
 *   8. **`getIntervalDisplayName`**: monthly → "Monthly", quarterly →
 *      "Quarterly", yearly → "Annual" (NOT "Yearly"), anything else passes
 *      through unchanged. The "Annual" mapping is what the marketing
 *      pages use; reverting to "Yearly" silently changes the storefront.
 *
 *   9. **`hasItem` checks `id + interval`; `hasAnyVariant` checks `id` only.**
 *      Confusing the two breaks "is this product in my cart" checks on
 *      product pages.
 */

import { describe, expect, it, beforeEach } from 'vitest';
import {
	cartStore,
	isUpgrade,
	getIntervalDisplayName,
	getCartItemCount,
	getCartTotal,
	getHasCartItems,
	addToCart,
	type CartItem
} from '../cart.svelte';

// ═══════════════════════════════════════════════════════════════════════════
// Test setup — clear cart between cases. Cart state is module-level so
// every test must start clean.
// ═══════════════════════════════════════════════════════════════════════════

beforeEach(() => {
	cartStore.clearCart();
});

// ═══════════════════════════════════════════════════════════════════════════
// isUpgrade — interval priority contract
// ═══════════════════════════════════════════════════════════════════════════

describe('isUpgrade', () => {
	it('happy path: monthly → yearly is an upgrade', () => {
		expect(isUpgrade('monthly', 'yearly')).toBe(true);
	});

	it('happy path: monthly → quarterly is an upgrade', () => {
		expect(isUpgrade('monthly', 'quarterly')).toBe(true);
	});

	it('happy path: quarterly → yearly is an upgrade', () => {
		expect(isUpgrade('quarterly', 'yearly')).toBe(true);
	});

	it('NEGATIVE: yearly → monthly is NOT an upgrade (downgrade)', () => {
		expect(isUpgrade('yearly', 'monthly')).toBe(false);
	});

	it('NEGATIVE: same interval returns false (no change is not an upgrade)', () => {
		expect(isUpgrade('monthly', 'monthly')).toBe(false);
		expect(isUpgrade('quarterly', 'quarterly')).toBe(false);
		expect(isUpgrade('yearly', 'yearly')).toBe(false);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// getIntervalDisplayName — yearly → "Annual" mapping is contract
// ═══════════════════════════════════════════════════════════════════════════

describe('getIntervalDisplayName', () => {
	it('happy path: maps the three canonical intervals', () => {
		expect(getIntervalDisplayName('monthly')).toBe('Monthly');
		expect(getIntervalDisplayName('quarterly')).toBe('Quarterly');
		// "Annual" not "Yearly" — pinned because the storefront copy says Annual.
		expect(getIntervalDisplayName('yearly')).toBe('Annual');
	});

	it('NEGATIVE: passes through unknown intervals unchanged (does not throw, does not map "lifetime")', () => {
		expect(getIntervalDisplayName('lifetime')).toBe('lifetime');
		expect(getIntervalDisplayName('biennial')).toBe('biennial');
		expect(getIntervalDisplayName('')).toBe('');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// cartStore.addItem — return-value AND max-1-per-line contracts
// ═══════════════════════════════════════════════════════════════════════════

describe('cartStore.addItem', () => {
	const sampleMembership: Omit<CartItem, 'quantity'> = {
		id: 'mem-1',
		name: 'Pro Membership',
		price: 99,
		type: 'membership',
		interval: 'monthly'
	};

	it('happy path: adds an item, returns true', () => {
		const ok = cartStore.addItem(sampleMembership);
		expect(ok).toBe(true);
		expect(cartStore.items).toHaveLength(1);
		expect(cartStore.items[0]).toMatchObject({
			id: 'mem-1',
			quantity: 1, // forced to 1
			interval: 'monthly'
		});
	});

	it('NEGATIVE: adding same {id, interval} twice does NOT duplicate, returns false', () => {
		expect(cartStore.addItem(sampleMembership)).toBe(true);
		expect(cartStore.addItem(sampleMembership)).toBe(false);
		expect(cartStore.items).toHaveLength(1);
		// Quantity must NOT increase — this is the silent-money-bug guard.
		expect(cartStore.items[0]?.quantity).toBe(1);
	});

	it('CONTRACT: same id with DIFFERENT interval is a separate line item', () => {
		cartStore.addItem({ ...sampleMembership, interval: 'monthly' });
		cartStore.addItem({ ...sampleMembership, interval: 'yearly', price: 999 });
		expect(cartStore.items).toHaveLength(2);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// cartStore.hasItem / hasAnyVariant
// ═══════════════════════════════════════════════════════════════════════════

describe('cartStore.hasItem / hasAnyVariant', () => {
	beforeEach(() => {
		cartStore.addItem({
			id: 'sub-1',
			name: 'X',
			price: 50,
			type: 'membership',
			interval: 'monthly'
		});
	});

	it('hasItem: exact match on id + interval returns true', () => {
		expect(cartStore.hasItem('sub-1', 'monthly')).toBe(true);
	});

	it('NEGATIVE: hasItem with DIFFERENT interval returns false (composite key)', () => {
		expect(cartStore.hasItem('sub-1', 'yearly')).toBe(false);
	});

	it('hasAnyVariant: matches by id regardless of interval', () => {
		const item = cartStore.hasAnyVariant('sub-1');
		expect(item).toBeDefined();
		expect(item?.id).toBe('sub-1');
	});

	it('NEGATIVE: hasAnyVariant returns undefined when nothing matches', () => {
		expect(cartStore.hasAnyVariant('does-not-exist')).toBeUndefined();
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// cartStore.updateQuantity — max 1 enforced
// ═══════════════════════════════════════════════════════════════════════════

describe('cartStore.updateQuantity', () => {
	beforeEach(() => {
		cartStore.addItem({
			id: 'sub-1',
			name: 'X',
			price: 50,
			type: 'membership',
			interval: 'monthly'
		});
	});

	it('happy path: quantity 1 stays at 1', () => {
		cartStore.updateQuantity('sub-1', 1, 'monthly');
		expect(cartStore.items[0]?.quantity).toBe(1);
	});

	it('CONTRACT: quantity > 1 is clamped to 1 (max-1-per-line)', () => {
		cartStore.updateQuantity('sub-1', 5, 'monthly');
		expect(cartStore.items[0]?.quantity).toBe(1);
	});

	it('NEGATIVE: quantity 0 removes the item', () => {
		cartStore.updateQuantity('sub-1', 0, 'monthly');
		expect(cartStore.items).toHaveLength(0);
	});

	it('NEGATIVE: negative quantity also removes the item', () => {
		cartStore.updateQuantity('sub-1', -3, 'monthly');
		expect(cartStore.items).toHaveLength(0);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// cartStore.removeItem
// ═══════════════════════════════════════════════════════════════════════════

describe('cartStore.removeItem', () => {
	it('removes the matching {id, interval} item, leaves siblings intact', () => {
		cartStore.addItem({
			id: 'sub-1',
			name: 'X',
			price: 50,
			type: 'membership',
			interval: 'monthly'
		});
		cartStore.addItem({
			id: 'sub-1',
			name: 'X-yearly',
			price: 500,
			type: 'membership',
			interval: 'yearly'
		});

		cartStore.removeItem('sub-1', 'monthly');

		expect(cartStore.items).toHaveLength(1);
		expect(cartStore.items[0]?.interval).toBe('yearly');
	});

	it('NEGATIVE: removing a non-existent {id, interval} is a no-op', () => {
		cartStore.addItem({
			id: 'sub-1',
			name: 'X',
			price: 50,
			type: 'membership',
			interval: 'monthly'
		});
		cartStore.removeItem('nope', 'monthly');
		expect(cartStore.items).toHaveLength(1);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// cartStore.applyCoupon / removeCoupon — line-level write
// ═══════════════════════════════════════════════════════════════════════════

describe('cartStore.applyCoupon / removeCoupon', () => {
	beforeEach(() => {
		cartStore.addItem({
			id: 'sub-1',
			name: 'X',
			price: 100,
			type: 'membership',
			interval: 'monthly'
		});
		cartStore.addItem({
			id: 'course-1',
			name: 'Y',
			price: 200,
			type: 'course'
		});
	});

	it('happy path: applyCoupon writes couponCode + discount to every line', () => {
		cartStore.applyCoupon('SAVE10', 10);
		for (const item of cartStore.items) {
			expect(item.couponCode).toBe('SAVE10');
			expect(item.discount).toBe(10);
		}
	});

	it('removeCoupon strips couponCode AND discount from every line', () => {
		cartStore.applyCoupon('SAVE10', 10);
		cartStore.removeCoupon();
		for (const item of cartStore.items) {
			expect(item.couponCode).toBeUndefined();
			expect(item.discount).toBeUndefined();
		}
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// Totals — getItemCount, getTotal, exported getters
// ═══════════════════════════════════════════════════════════════════════════

describe('cart totals', () => {
	it('empty cart: getItemCount=0, getTotal=0, getHasCartItems=false', () => {
		expect(cartStore.getItemCount()).toBe(0);
		expect(cartStore.getTotal()).toBe(0);
		expect(getCartItemCount()).toBe(0);
		expect(getCartTotal()).toBe(0);
		expect(getHasCartItems()).toBe(false);
	});

	it('happy path: getTotal = sum(price * quantity); quantity is always 1', () => {
		cartStore.addItem({ id: 'a', name: 'A', price: 100, type: 'course' });
		cartStore.addItem({ id: 'b', name: 'B', price: 250, type: 'indicator' });
		cartStore.addItem({ id: 'c', name: 'C', price: 49.99, type: 'membership', interval: 'monthly' });

		expect(cartStore.getItemCount()).toBe(3);
		expect(cartStore.getTotal()).toBeCloseTo(399.99, 5);
		expect(getHasCartItems()).toBe(true);
	});

	it('clearCart resets total/count to zero', () => {
		cartStore.addItem({ id: 'a', name: 'A', price: 100, type: 'course' });
		expect(cartStore.getItemCount()).toBe(1);

		cartStore.clearCart();
		expect(cartStore.getItemCount()).toBe(0);
		expect(cartStore.getTotal()).toBe(0);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// addToCart helper — params-bag → cartStore.addItem adapter
// ═══════════════════════════════════════════════════════════════════════════

describe('addToCart helper', () => {
	it('happy path: maps {productId, productName, productType, price} to a cart line', async () => {
		const ok = await addToCart({
			productId: 'p-1',
			productName: 'Pro Membership',
			productType: 'membership',
			price: 99,
			interval: 'monthly'
		});

		expect(ok).toBe(true);
		expect(cartStore.items).toHaveLength(1);
		expect(cartStore.items[0]).toMatchObject({
			id: 'p-1',
			name: 'Pro Membership',
			price: 99,
			type: 'membership',
			interval: 'monthly',
			// Falls back to productId when productSlug is not given — pinned
			// because the checkout page uses productSlug for analytics.
			productSlug: 'p-1'
		});
	});

	it('CONTRACT: explicit productSlug overrides the productId fallback', async () => {
		await addToCart({
			productId: 'p-1',
			productName: 'X',
			productType: 'course',
			price: 49,
			productSlug: 'pro-trader-course'
		});
		expect(cartStore.items[0]?.productSlug).toBe('pro-trader-course');
	});

	it('NEGATIVE: duplicate addToCart returns false', async () => {
		const params = {
			productId: 'p-2',
			productName: 'Y',
			productType: 'indicator' as const,
			price: 19
		};
		expect(await addToCart(params)).toBe(true);
		expect(await addToCart(params)).toBe(false);
		expect(cartStore.items).toHaveLength(1);
	});
});
