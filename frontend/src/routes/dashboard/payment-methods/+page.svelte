<script lang="ts">
	/**
	 * Dashboard - Payment Methods Page
	 * WordPress Revolution Trading Exact Match
	 */
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { isAuthenticated, authStore } from '$lib/stores/auth';
	import IconCreditCard from '@tabler/icons-svelte/icons/credit-card';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconBrandPaypal from '@tabler/icons-svelte/icons/brand-paypal';

	onMount(() => {
		if (!$isAuthenticated && !$authStore.isInitializing) {
			goto('/login?redirect=/dashboard/payment-methods', { replaceState: true });
		}
	});

	interface PaymentMethod {
		id: string;
		type: 'card' | 'paypal';
		brand?: string;
		last4?: string;
		expMonth?: number;
		expYear?: number;
		email?: string;
		isDefault: boolean;
	}

	let paymentMethods: PaymentMethod[] = $state([
		{ id: 'pm_1', type: 'card', brand: 'Visa', last4: '4242', expMonth: 12, expYear: 2027, isDefault: true },
		{ id: 'pm_2', type: 'paypal', email: 'john@example.com', isDefault: false }
	]);

	let showAddForm = $state(false);

	function setDefault(id: string) {
		paymentMethods = paymentMethods.map(pm => ({ ...pm, isDefault: pm.id === id }));
	}

	function removeMethod(id: string) {
		paymentMethods = paymentMethods.filter(pm => pm.id !== id);
	}

	function getCardIcon(brand?: string) {
		return IconCreditCard;
	}
</script>

<svelte:head>
	<title>Payment Methods | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

{#if $isAuthenticated}
	<header class="dashboard__header">
		<h1 class="dashboard__page-title">Payment Methods</h1>
		<p class="dashboard__page-subtitle">Manage your saved payment methods</p>
	</header>

	<div class="dashboard__content">
		<div class="dashboard__content-main">
			<!-- Payment Methods List -->
			<div class="payment-methods-list">
				{#each paymentMethods as method (method.id)}
					<div class="payment-card" class:is-default={method.isDefault}>
						<div class="payment-card__icon">
							{#if method.type === 'card'}
								<IconCreditCard size={32} />
							{:else}
								<IconBrandPaypal size={32} />
							{/if}
						</div>
						<div class="payment-card__info">
							{#if method.type === 'card'}
								<p class="payment-card__title">{method.brand} ending in {method.last4}</p>
								<p class="payment-card__detail">Expires {method.expMonth}/{method.expYear}</p>
							{:else}
								<p class="payment-card__title">PayPal</p>
								<p class="payment-card__detail">{method.email || ''}</p>
							{/if}
							{#if method.isDefault}
								<span class="default-badge">
									<IconCheck size={12} />
									Default
								</span>
							{/if}
						</div>
						<div class="payment-card__actions">
							{#if !method.isDefault}
								<button class="action-btn" onclick={() => setDefault(method.id)}>Set as Default</button>
							{/if}
							<button class="action-btn action-btn--danger" onclick={() => removeMethod(method.id)}>
								<IconTrash size={16} />
							</button>
						</div>
					</div>
				{/each}
			</div>

			<!-- Add New Button -->
			<button class="add-method-btn" onclick={() => showAddForm = true}>
				<IconPlus size={20} />
				Add Payment Method
			</button>

			{#if showAddForm}
				<!-- Add Form Modal -->
				<div
					class="add-form-overlay"
					role="button"
					tabindex="0"
					onclick={() => showAddForm = false}
					onkeydown={(e) => e.key === 'Escape' && (showAddForm = false)}
					aria-label="Close modal"
				>
					<div class="add-form" role="dialog" aria-modal="true" tabindex="-1" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
						<h3>Add Payment Method</h3>
						<div class="payment-options">
							<button class="payment-option">
								<IconCreditCard size={24} />
								<span>Credit/Debit Card</span>
							</button>
							<button class="payment-option">
								<IconBrandPaypal size={24} />
								<span>PayPal</span>
							</button>
						</div>
						<p class="add-form__note">You'll be redirected to securely add your payment method.</p>
						<div class="add-form__actions">
							<button class="cancel-btn" onclick={() => showAddForm = false}>Cancel</button>
						</div>
					</div>
				</div>
			{/if}
		</div>
	</div>
{:else}
	<div class="loading-state"><p>Redirecting to login...</p></div>
{/if}

<style>
	.dashboard__header { background: #fff; padding: 30px; border-bottom: 1px solid #ededed; }
	.dashboard__page-title { font-size: 32px; font-weight: 700; color: #333; margin: 0 0 8px; font-family: 'Open Sans Condensed', 'Open Sans', sans-serif; }
	.dashboard__page-subtitle { font-size: 15px; color: #666; margin: 0; }
	.dashboard__content { padding: 30px; background: #fff; min-height: 400px; }
	.dashboard__content-main { max-width: 700px; }

	.payment-methods-list { display: flex; flex-direction: column; gap: 16px; margin-bottom: 24px; }

	.payment-card { display: flex; align-items: center; gap: 16px; padding: 20px; border: 1px solid #ededed; border-radius: 8px; background: #fff; transition: all 0.15s; }
	.payment-card.is-default { border-color: #0984ae; background: #f0f9ff; }
	.payment-card__icon { color: #666; flex-shrink: 0; }
	.payment-card.is-default .payment-card__icon { color: #0984ae; }
	.payment-card__info { flex: 1; }
	.payment-card__title { margin: 0 0 4px; font-weight: 600; color: #333; }
	.payment-card__detail { margin: 0; font-size: 13px; color: #666; }
	.default-badge { display: inline-flex; align-items: center; gap: 4px; margin-top: 8px; padding: 4px 8px; background: #dcfce7; color: #166534; font-size: 11px; font-weight: 600; border-radius: 4px; text-transform: uppercase; }
	.payment-card__actions { display: flex; gap: 8px; }

	.action-btn { padding: 8px 14px; background: #f4f4f4; border: 1px solid #dbdbdb; border-radius: 6px; font-size: 13px; color: #333; cursor: pointer; transition: all 0.15s; }
	.action-btn:hover { background: #eee; }
	.action-btn--danger { padding: 8px; color: #ef4444; }
	.action-btn--danger:hover { background: #fef2f2; border-color: #ef4444; }

	.add-method-btn { display: inline-flex; align-items: center; gap: 8px; padding: 12px 20px; background: #0984ae; border: none; border-radius: 6px; color: #fff; font-weight: 600; cursor: pointer; transition: background 0.15s; }
	.add-method-btn:hover { background: #076787; }

	.add-form-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 1000; }
	.add-form { background: #fff; padding: 30px; border-radius: 12px; max-width: 400px; width: 90%; }
	.add-form h3 { margin: 0 0 20px; font-size: 20px; color: #333; }
	.payment-options { display: flex; flex-direction: column; gap: 12px; margin-bottom: 20px; }
	.payment-option { display: flex; align-items: center; gap: 12px; padding: 16px; border: 1px solid #ededed; border-radius: 8px; background: #fff; cursor: pointer; transition: all 0.15s; }
	.payment-option:hover { border-color: #0984ae; background: #f0f9ff; }
	.payment-option span { font-weight: 500; color: #333; }
	.add-form__note { font-size: 13px; color: #666; margin-bottom: 20px; }
	.add-form__actions { display: flex; justify-content: flex-end; }
	.cancel-btn { padding: 10px 20px; background: #f4f4f4; border: 1px solid #dbdbdb; border-radius: 6px; color: #333; cursor: pointer; }

	.loading-state { display: flex; align-items: center; justify-content: center; min-height: 300px; color: #666; }

	@media (max-width: 768px) {
		.dashboard__header { padding: 20px; }
		.dashboard__page-title { font-size: 26px; }
		.dashboard__content { padding: 20px; }
		.payment-card { flex-wrap: wrap; }
		.payment-card__actions { width: 100%; justify-content: flex-end; margin-top: 12px; padding-top: 12px; border-top: 1px solid #ededed; }
	}
</style>
