/**
 * Revolution Trading Pros - Test Fixtures
 * Apple ICT 11+ Principal Engineer Grade
 *
 * Shared test utilities and fixtures
 */

import { test as base, expect } from '@playwright/test';

// Custom test fixture with common utilities
export const test = base.extend({
	// Add any custom fixtures here
});

// Re-export expect for convenience
export { expect };

// Common test data
export const testData = {
	validEmail: 'test@example.com',
	validPassword: 'TestPassword123!',
	invalidEmail: 'invalid-email',
	shortPassword: '123'
};

// Page routes
export const routes = {
	home: '/',
	shop: '/shop',
	cart: '/cart',
	checkout: '/checkout',
	login: '/login',
	register: '/register',
	account: '/my-account',
	pricing: '/pricing',
	dashboard: '/dashboard'
};

// Common selectors
export const selectors = {
	// Navigation
	mainNav: 'nav, header, [role="navigation"]',
	mobileMenu: '[data-mobile-menu], .mobile-menu, button[aria-label*="menu"]',

	// Forms
	emailInput: 'input[type="email"], input[name="email"]',
	passwordInput: 'input[type="password"]',
	submitButton: 'button[type="submit"], input[type="submit"]',

	// Cart
	addToCart: 'button:has-text("Add to Cart"), [data-add-to-cart]',
	cartIcon: '[data-cart-icon], a[href*="cart"]',
	cartCount: '[data-cart-count], .cart-count',

	// Products
	productCard: '[data-product], .product-card, .product',
	productPrice: '[data-price], .price',
	productTitle: '[data-product-title], .product-title'
};

// Helper functions
export async function waitForNetworkIdle(page: import('@playwright/test').Page): Promise<void> {
	await page.waitForLoadState('networkidle');
}

export async function clearLocalStorage(page: import('@playwright/test').Page): Promise<void> {
	await page.evaluate(() => localStorage.clear());
}

export async function clearSessionStorage(page: import('@playwright/test').Page): Promise<void> {
	await page.evaluate(() => sessionStorage.clear());
}

export async function getAuthToken(page: import('@playwright/test').Page): Promise<string | null> {
	return page.evaluate(() => {
		return localStorage.getItem('auth_token') || localStorage.getItem('token') || null;
	});
}
