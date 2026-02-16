import { cartStore, type CartItem } from '$lib/stores/cart.svelte';
import { addToCart as addToCartApi } from '$lib/api/cart';
import { goto } from '$app/navigation';
import { base } from '$app/paths';
import { logger } from '$lib/utils/logger';

/**
 * Add a product/membership to cart and optionally navigate to cart page
 *
 * @param item - The item to add to cart
 * @param navigateToCart - Whether to navigate to cart page after adding (default: false)
 */
export async function addItemToCart(
	item: Omit<CartItem, 'quantity'>,
	navigateToCart: boolean = false
): Promise<void> {
	// Add to local cart store
	cartStore.addItem(item);

	// Sync with backend if user is authenticated
	try {
		await addToCartApi(item);
	} catch (error) {
		logger.error('Failed to sync cart with backend:', error);
		// Continue anyway - local cart works
	}

	// Navigate to cart if requested
	if (navigateToCart) {
		// eslint-disable-next-line svelte/no-navigation-without-resolve -- base path is prepended
		await goto(`${base}/cart`);
	}
}

/**
 * Quick add membership to cart
 *
 * @param membershipId - The membership ID
 * @param name - Membership name
 * @param description - Membership description
 * @param price - Monthly or yearly price
 * @param interval - 'monthly', 'quarterly', or 'yearly'
 * @param navigateToCart - Whether to navigate to cart (default: true for "Sign Up Now")
 */
export async function addMembershipToCart(
	membershipId: string,
	name: string,
	description: string,
	price: number,
	interval: 'monthly' | 'quarterly' | 'yearly',
	navigateToCart: boolean = true
): Promise<void> {
	await addItemToCart(
		{
			id: membershipId,
			name,
			description,
			price,
			type: 'membership',
			interval
		},
		navigateToCart
	);
}

/**
 * Quick add course to cart
 */
export async function addCourseToCart(
	courseId: string,
	name: string,
	description: string,
	price: number,
	navigateToCart: boolean = false
): Promise<void> {
	await addItemToCart(
		{
			id: courseId,
			name,
			description,
			price,
			type: 'course'
		},
		navigateToCart
	);
}

/**
 * Quick add alert service to cart
 */
export async function addAlertServiceToCart(
	serviceId: string,
	name: string,
	description: string,
	price: number,
	interval: 'monthly' | 'quarterly' | 'yearly',
	navigateToCart: boolean = false
): Promise<void> {
	await addItemToCart(
		{
			id: serviceId,
			name,
			description,
			price,
			type: 'alert-service',
			interval
		},
		navigateToCart
	);
}
