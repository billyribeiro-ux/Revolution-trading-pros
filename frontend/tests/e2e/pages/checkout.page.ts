/**
 * Revolution Trading Pros - Checkout Page Object
 *
 * Page object for checkout flow with:
 * - Cart management
 * - Billing form
 * - Payment integration (Stripe)
 * - Order confirmation
 */

import { type Page, type Locator, expect } from '@playwright/test';
import { BasePage } from './base.page';
import { TEST_CARDS, TEST_ADDRESSES } from '../helpers/test-data.helper';

export class CheckoutPage extends BasePage {
	// Cart elements
	readonly cartItems: Locator;
	readonly cartTotal: Locator;
	readonly cartSubtotal: Locator;
	readonly emptyCartMessage: Locator;
	readonly removeItemButton: Locator;
	readonly couponInput: Locator;
	readonly applyCouponButton: Locator;

	// Billing form
	readonly emailInput: Locator;
	readonly firstNameInput: Locator;
	readonly lastNameInput: Locator;
	readonly addressInput: Locator;
	readonly cityInput: Locator;
	readonly stateInput: Locator;
	readonly zipInput: Locator;
	readonly countrySelect: Locator;
	readonly phoneInput: Locator;

	// Payment elements (Stripe)
	readonly stripeCardElement: Locator;
	readonly stripeCardNumber: Locator;
	readonly stripeExpiry: Locator;
	readonly stripeCvc: Locator;

	// Actions
	readonly proceedToCheckoutButton: Locator;
	readonly placeOrderButton: Locator;
	readonly backToCartButton: Locator;

	// Messages
	readonly errorMessage: Locator;
	readonly successMessage: Locator;
	readonly processingMessage: Locator;

	// Order summary
	readonly orderSummary: Locator;
	readonly orderNumber: Locator;

	constructor(page: Page) {
		super(page);

		// Cart
		this.cartItems = page.locator('[data-testid="cart-item"], .cart-item, .line-item');
		this.cartTotal = page.locator('[data-testid="cart-total"], .cart-total, .total-price');
		this.cartSubtotal = page.locator('[data-testid="cart-subtotal"], .subtotal');
		this.emptyCartMessage = page.locator('[data-testid="empty-cart"], .empty-cart');
		this.removeItemButton = page.locator('[data-testid="remove-item"], .remove-item');
		this.couponInput = page.locator('input[name="coupon"], [data-testid="coupon-input"]');
		this.applyCouponButton = page.locator('button:has-text("Apply"), [data-testid="apply-coupon"]');

		// Billing form
		this.emailInput = page.locator('input[name="email"], input[type="email"]').first();
		this.firstNameInput = page.locator('input[name="firstName"], input[name="first_name"]');
		this.lastNameInput = page.locator('input[name="lastName"], input[name="last_name"]');
		this.addressInput = page.locator('input[name="address"], input[name="address1"]');
		this.cityInput = page.locator('input[name="city"]');
		this.stateInput = page.locator('input[name="state"], select[name="state"]');
		this.zipInput = page.locator('input[name="zip"], input[name="postal_code"]');
		this.countrySelect = page.locator('select[name="country"]');
		this.phoneInput = page.locator('input[name="phone"]');

		// Stripe elements (inside iframes)
		this.stripeCardElement = page.locator('[data-testid="stripe-card"], .StripeElement');
		this.stripeCardNumber = page.frameLocator('iframe[name*="card-number"]').locator('input[name="cardnumber"]');
		this.stripeExpiry = page.frameLocator('iframe[name*="card-expiry"]').locator('input[name="exp-date"]');
		this.stripeCvc = page.frameLocator('iframe[name*="card-cvc"]').locator('input[name="cvc"]');

		// Actions
		this.proceedToCheckoutButton = page.locator(
			'button:has-text("Checkout"), button:has-text("Proceed"), [data-testid="proceed-checkout"]'
		);
		this.placeOrderButton = page.locator(
			'button:has-text("Place Order"), button:has-text("Pay"), button:has-text("Complete"), [data-testid="place-order"]'
		);
		this.backToCartButton = page.locator('a:has-text("Back"), button:has-text("Back")');

		// Messages
		this.errorMessage = page.locator('[data-testid="checkout-error"], .checkout-error, .error-message');
		this.successMessage = page.locator('[data-testid="checkout-success"], .checkout-success');
		this.processingMessage = page.locator('.processing, .loading, :has-text("Processing")');

		// Order summary
		this.orderSummary = page.locator('[data-testid="order-summary"], .order-summary');
		this.orderNumber = page.locator('[data-testid="order-number"], .order-number');
	}

	get path(): string {
		return '/checkout';
	}

	/**
	 * Navigates to cart page
	 */
	async gotoCart(): Promise<void> {
		await this.page.goto('/cart');
		await this.waitForPageReady();
	}

	/**
	 * Checks if cart is empty
	 */
	async isCartEmpty(): Promise<boolean> {
		return await this.emptyCartMessage.isVisible();
	}

	/**
	 * Gets the number of items in cart
	 */
	async getCartItemCount(): Promise<number> {
		return await this.cartItems.count();
	}

	/**
	 * Gets the cart total text
	 */
	async getCartTotal(): Promise<string | null> {
		try {
			return await this.cartTotal.textContent();
		} catch {
			return null;
		}
	}

	/**
	 * Removes an item from cart by index
	 */
	async removeItem(index: number = 0): Promise<void> {
		await this.removeItemButton.nth(index).click();
	}

	/**
	 * Applies a coupon code
	 */
	async applyCoupon(code: string): Promise<void> {
		await this.couponInput.fill(code);
		await this.applyCouponButton.click();
		await this.page.waitForTimeout(1000);
	}

	/**
	 * Fills the billing form with test address
	 */
	async fillBillingForm(options?: {
		email?: string;
		firstName?: string;
		lastName?: string;
		address?: typeof TEST_ADDRESSES.us;
		phone?: string;
	}): Promise<void> {
		const address = options?.address || TEST_ADDRESSES.us;

		if (await this.emailInput.isVisible()) {
			await this.emailInput.fill(options?.email || 'test@example.com');
		}

		if (await this.firstNameInput.isVisible()) {
			await this.firstNameInput.fill(options?.firstName || 'Test');
		}

		if (await this.lastNameInput.isVisible()) {
			await this.lastNameInput.fill(options?.lastName || 'User');
		}

		if (await this.addressInput.isVisible()) {
			await this.addressInput.fill(address.line1);
		}

		if (await this.cityInput.isVisible()) {
			await this.cityInput.fill(address.city);
		}

		if (await this.stateInput.isVisible()) {
			const isSelect = await this.stateInput.evaluate((el) => el.tagName.toLowerCase() === 'select');
			if (isSelect) {
				await this.stateInput.selectOption(address.state);
			} else {
				await this.stateInput.fill(address.state);
			}
		}

		if (await this.zipInput.isVisible()) {
			await this.zipInput.fill(address.zip);
		}

		if (await this.countrySelect.isVisible()) {
			await this.countrySelect.selectOption(address.country);
		}

		if (options?.phone && await this.phoneInput.isVisible()) {
			await this.phoneInput.fill(options.phone);
		}
	}

	/**
	 * Fills Stripe card details (for test mode)
	 */
	async fillStripeCard(
		card: typeof TEST_CARDS.success = TEST_CARDS.success
	): Promise<void> {
		// Stripe uses iframes, need to handle carefully
		try {
			// Wait for Stripe to load
			await this.page.waitForSelector('iframe[name*="card"]', { timeout: 10000 });

			// Fill card number
			const cardNumberFrame = this.page.frameLocator('iframe[name*="card-number"]').first();
			await cardNumberFrame.locator('input').fill(card.number);

			// Fill expiry
			const expiryFrame = this.page.frameLocator('iframe[name*="card-expiry"]').first();
			await expiryFrame.locator('input').fill(`${card.expMonth}/${card.expYear.slice(-2)}`);

			// Fill CVC
			const cvcFrame = this.page.frameLocator('iframe[name*="card-cvc"]').first();
			await cvcFrame.locator('input').fill(card.cvc);

			// Fill zip if there's a separate field
			const zipFrame = this.page.frameLocator('iframe[name*="postal"]').first();
			if (await zipFrame.locator('input').isVisible().catch(() => false)) {
				await zipFrame.locator('input').fill(card.zip);
			}
		} catch (error) {
			console.log('Stripe iframe handling:', error);
			// Try alternative approach with single card element
			const cardInput = this.page.locator('[data-stripe="card"], #card-element input');
			if (await cardInput.isVisible()) {
				await cardInput.fill(card.number);
			}
		}
	}

	/**
	 * Proceeds from cart to checkout
	 */
	async proceedToCheckout(): Promise<void> {
		await this.proceedToCheckoutButton.click();
		await this.page.waitForURL('**/checkout**');
	}

	/**
	 * Places the order
	 */
	async placeOrder(): Promise<void> {
		await this.placeOrderButton.click();
	}

	/**
	 * Waits for order to complete
	 */
	async waitForOrderCompletion(timeout: number = 30000): Promise<void> {
		// Wait for either success page or thank you message
		await this.page.waitForURL(
			(url) =>
				url.pathname.includes('thank-you') ||
				url.pathname.includes('success') ||
				url.pathname.includes('confirmation'),
			{ timeout }
		);
	}

	/**
	 * Gets the order number after successful checkout
	 */
	async getOrderNumber(): Promise<string | null> {
		try {
			await this.orderNumber.waitFor({ state: 'visible', timeout: 5000 });
			return await this.orderNumber.textContent();
		} catch {
			return null;
		}
	}

	/**
	 * Verifies checkout error is displayed
	 */
	async verifyErrorDisplayed(expectedText?: string): Promise<void> {
		await expect(this.errorMessage).toBeVisible();
		if (expectedText) {
			await expect(this.errorMessage).toContainText(expectedText);
		}
	}

	/**
	 * Complete checkout flow with test card
	 */
	async completeCheckoutWithTestCard(options?: {
		card?: typeof TEST_CARDS.success;
		billingInfo?: Parameters<typeof this.fillBillingForm>[0];
	}): Promise<void> {
		// Fill billing if needed
		await this.fillBillingForm(options?.billingInfo);

		// Fill payment
		await this.fillStripeCard(options?.card || TEST_CARDS.success);

		// Place order
		await this.placeOrder();

		// Wait for completion
		await this.waitForOrderCompletion();
	}

	/**
	 * Verifies order summary shows correct items
	 */
	async verifyOrderSummary(expectedItems: Array<{ name: string; price?: string }>): Promise<void> {
		await expect(this.orderSummary).toBeVisible();

		for (const item of expectedItems) {
			await expect(this.orderSummary.getByText(item.name)).toBeVisible();
			if (item.price) {
				await expect(this.orderSummary.getByText(item.price)).toBeVisible();
			}
		}
	}
}
