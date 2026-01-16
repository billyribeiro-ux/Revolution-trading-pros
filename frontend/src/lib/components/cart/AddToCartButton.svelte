<script lang="ts">
	/**
	 * Smart AddToCart Button Component
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Handles cart addition with:
	 * - Ownership checking (already owned products)
	 * - Subscription conflict detection (already subscribed / upgrading)
	 * - User feedback with appropriate messages
	 * - Prevents duplicate purchases for courses and indicators
	 *
	 * @version 1.0.0 (Svelte 5 / December 2025)
	 */

	import { cartStore, type CartItem } from '$lib/stores/cart';
	import { isAuthenticated } from '$lib/stores/auth.svelte';
	import {
		checkProductOwnership,
		type MembershipType,
		type BillingInterval
	} from '$lib/api/user-memberships';
	import IconShoppingCart from '@tabler/icons-svelte/icons/shopping-cart';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconLock from '@tabler/icons-svelte/icons/lock';
	import IconArrowUp from '@tabler/icons-svelte/icons/arrow-up';
	import IconLoader from '@tabler/icons-svelte/icons/loader';

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS
	// ═══════════════════════════════════════════════════════════════════════════

	interface Props {
		productId: string;
		productName: string;
		productSlug: string;
		productType: MembershipType | 'membership' | 'alert-service';
		price: number;
		interval?: BillingInterval;
		thumbnail?: string;
		image?: string;
		description?: string;
		class?: string;
		disabled?: boolean;
	}

	let {
		productId,
		productName,
		productSlug,
		productType,
		price,
		interval,
		thumbnail,
		image,
		description,
		class: className = '',
		disabled = false
	}: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let isChecking = $state(false);
	let isAdding = $state(false);
	let ownershipStatus = $state<{
		owned: boolean;
		conflictType?: 'already_subscribed' | 'upgrade' | 'downgrade' | 'already_owned' | null;
		message?: string;
	} | null>(null);
	let addedToCart = $state(false);
	let showMessage = $state(false);
	let messageTimeout: ReturnType<typeof setTimeout>;

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED
	// ═══════════════════════════════════════════════════════════════════════════

	// Normalize product type
	let normalizedType = $derived.by((): MembershipType => {
		if (productType === 'membership' || productType === 'alert-service') {
			return 'alert-service';
		}
		return productType as MembershipType;
	});

	// Check if item is already in cart (filter out 'lifetime' as cart doesn't support it)
	let isInCart = $derived(cartStore.hasItem(productId, interval === 'lifetime' ? undefined : interval));

	// Determine button state
	let isDisabled = $derived(
		disabled ||
			isChecking ||
			isAdding ||
			isInCart ||
			ownershipStatus?.conflictType === 'already_subscribed' ||
			ownershipStatus?.conflictType === 'already_owned'
	);

	// Button text based on state
	let buttonText = $derived((): string => {
		if (isChecking) return 'Checking...';
		if (isAdding) return 'Adding...';
		if (addedToCart || isInCart) return 'Added to Cart';
		if (ownershipStatus?.conflictType === 'already_owned') return 'Already Owned';
		if (ownershipStatus?.conflictType === 'already_subscribed') return 'Already Subscribed';
		if (ownershipStatus?.conflictType === 'upgrade') return 'Upgrade Now';
		return 'Add to Cart';
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function checkOwnership() {
		if (!$isAuthenticated) return;

		isChecking = true;
		try {
			const result = await checkProductOwnership(
				productId,
				productSlug,
				normalizedType,
				interval
			);
			ownershipStatus = result;
		} catch (error) {
			console.error('[AddToCart] Error checking ownership:', error);
		} finally {
			isChecking = false;
		}
	}

	async function handleAddToCart() {
		// If not authenticated, add to cart anyway (will prompt login at checkout)
		if (!$isAuthenticated) {
			addItemToCart();
			return;
		}

		// Check ownership first if not already checked
		if (!ownershipStatus) {
			await checkOwnership();
		}

		// Handle based on conflict type
		if (ownershipStatus?.conflictType === 'already_owned') {
			showFeedbackMessage();
			return;
		}

		if (ownershipStatus?.conflictType === 'already_subscribed') {
			showFeedbackMessage();
			return;
		}

		// For upgrades, allow adding with the message
		if (ownershipStatus?.conflictType === 'upgrade') {
			addItemToCart();
			return;
		}

		// No conflicts, add to cart
		addItemToCart();
	}

	function addItemToCart() {
		isAdding = true;

		const cartType =
			normalizedType === 'indicator'
				? 'indicator'
				: normalizedType === 'course'
					? 'course'
					: normalizedType === 'trading-room'
						? 'membership'
						: 'alert-service';

		// Filter out 'lifetime' interval as cart store only supports monthly/quarterly/yearly
		const cartItem: Omit<typeof cartStore extends { addItem: (item: infer T) => any } ? T : never, never> = {
			id: productId,
			name: productName,
			price,
			type: cartType,
			...(description && { description }),
			...(thumbnail && { thumbnail }),
			...(image && { image }),
			...(interval && interval !== 'lifetime' && { interval }),
			...(productSlug && { productSlug })
		};
		
		const added = cartStore.addItem(cartItem as Omit<CartItem, 'quantity'>);

		if (added) {
			addedToCart = true;
			showFeedbackMessage();
		}

		isAdding = false;
	}

	function showFeedbackMessage() {
		showMessage = true;
		clearTimeout(messageTimeout);
		messageTimeout = setTimeout(() => {
			showMessage = false;
		}, 4000);
	}

	// Check ownership on mount for authenticated users
	$effect(() => {
		if ($isAuthenticated && !ownershipStatus) {
			checkOwnership();
		}
	});
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     TEMPLATE - Smart AddToCart Button
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="add-to-cart-wrapper {className}">
	<!-- Feedback Message -->
	{#if showMessage && ownershipStatus?.message}
		<div
			class="add-to-cart-message"
			class:upgrade={ownershipStatus.conflictType === 'upgrade'}
			class:blocked={ownershipStatus.conflictType === 'already_owned' ||
				ownershipStatus.conflictType === 'already_subscribed'}
			class:success={addedToCart && !ownershipStatus.conflictType}
		>
			{#if ownershipStatus.conflictType === 'upgrade'}
				<IconArrowUp size={16} />
			{:else if ownershipStatus.conflictType === 'already_owned' || ownershipStatus.conflictType === 'already_subscribed'}
				<IconLock size={16} />
			{:else}
				<IconCheck size={16} />
			{/if}
			<span>{ownershipStatus.message}</span>
		</div>
	{:else if showMessage && addedToCart}
		<div class="add-to-cart-message success">
			<IconCheck size={16} />
			<span>Added to cart!</span>
		</div>
	{/if}

	<!-- Button -->
	<button
		type="button"
		class="add-to-cart-btn"
		class:added={addedToCart || isInCart}
		class:upgrade={ownershipStatus?.conflictType === 'upgrade'}
		class:blocked={ownershipStatus?.conflictType === 'already_owned' ||
			ownershipStatus?.conflictType === 'already_subscribed'}
		onclick={handleAddToCart}
		disabled={isDisabled}
	>
		{#if isChecking || isAdding}
			<IconLoader size={18} class="spin" />
		{:else if addedToCart || isInCart}
			<IconCheck size={18} />
		{:else if ownershipStatus?.conflictType === 'upgrade'}
			<IconArrowUp size={18} />
		{:else if ownershipStatus?.conflictType === 'already_owned' || ownershipStatus?.conflictType === 'already_subscribed'}
			<IconLock size={18} />
		{:else}
			<IconShoppingCart size={18} />
		{/if}
		<span>{buttonText}</span>
	</button>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	.add-to-cart-wrapper {
		position: relative;
	}

	.add-to-cart-message {
		position: absolute;
		bottom: 100%;
		left: 50%;
		transform: translateX(-50%);
		margin-bottom: 8px;
		padding: 10px 16px;
		border-radius: 6px;
		font-size: 13px;
		font-weight: 600;
		white-space: nowrap;
		display: flex;
		align-items: center;
		gap: 8px;
		animation: fadeIn 0.2s ease-out;
		z-index: 10;
	}

	.add-to-cart-message.success {
		background: #10b981;
		color: #fff;
	}

	.add-to-cart-message.upgrade {
		background: #0984ae;
		color: #fff;
	}

	.add-to-cart-message.blocked {
		background: #f59e0b;
		color: #fff;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
			transform: translateX(-50%) translateY(10px);
		}
		to {
			opacity: 1;
			transform: translateX(-50%) translateY(0);
		}
	}

	.add-to-cart-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 14px 28px;
		font-size: 15px;
		font-weight: 700;
		border-radius: 6px;
		border: none;
		cursor: pointer;
		background: #f99e31;
		color: #fff;
		transition: all 0.15s ease-in-out;
		width: 100%;
	}

	.add-to-cart-btn:hover:not(:disabled) {
		background: #f88b09;
		transform: translateY(-1px);
	}

	.add-to-cart-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
		transform: none;
	}

	.add-to-cart-btn.added {
		background: #10b981;
	}

	.add-to-cart-btn.added:hover {
		background: #059669;
	}

	.add-to-cart-btn.upgrade {
		background: #0984ae;
	}

	.add-to-cart-btn.upgrade:hover:not(:disabled) {
		background: #076787;
	}

	.add-to-cart-btn.blocked {
		background: #9ca3af;
	}

	:global(.spin) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
