<script lang="ts">
	/**
	 * Account Layout - Secondary Navigation
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * 100% PIXEL-PERFECT match to SimplerMyOrders/SimplerAccountDetails reference
	 * Secondary nav with account tabs
	 *
	 * @version 2.0.0 - 100% Pixel Perfect
	 */
	import { page } from '$app/stores';
	import type { Snippet } from 'svelte';

	// Account navigation tabs - exact from reference
	const accountTabs = [
		{ href: '/dashboard/account/orders', label: 'My Orders', slug: 'orders' },
		{ href: '/dashboard/account/subscriptions', label: 'My Subscriptions', slug: 'subscriptions' },
		{ href: '/dashboard/account/coupons', label: 'Coupons', slug: 'coupons' },
		{ href: '/dashboard/account/billing-address', label: 'Billing Address', slug: 'billing-address' },
		{ href: '/dashboard/account/payment-methods', label: 'Payment Methods', slug: 'payment-methods' },
		{ href: '/dashboard/account/edit-account', label: 'Account Details', slug: 'edit-account' }
	];

	let { children }: { children: Snippet } = $props();
</script>

<!-- Secondary Navigation - Exact from SimplerMyOrders reference line 2895-2919 -->
<nav class="dashboard__nav-secondary">
	<ul>
		{#each accountTabs as tab}
			<li class="woocommerce-MyAccount-navigation-link" class:is-active={$page.url.pathname.includes(tab.slug)}>
				<a class="no-icon" href={tab.href}>{tab.label}</a>
			</li>
		{/each}
		<li class="woocommerce-MyAccount-navigation-link woocommerce-MyAccount-navigation-link--customer-logout">
			<a class="no-icon" href="/logout">Log out</a>
		</li>
	</ul>
</nav>

<!-- Page Content -->
{@render children()}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   100% PIXEL-PERFECT STYLES - Matching SimplerMyOrders reference
	   Secondary Navigation Tabs
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-secondary {
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
		margin-bottom: 30px;
	}

	.dashboard__nav-secondary ul {
		display: flex;
		flex-wrap: wrap;
		list-style: none;
		margin: 0;
		padding: 0;
		gap: 0;
	}

	.woocommerce-MyAccount-navigation-link {
		position: relative;
	}

	.woocommerce-MyAccount-navigation-link a {
		display: block;
		padding: 15px 20px;
		font-size: 14px;
		font-weight: 600;
		color: #666;
		text-decoration: none;
		transition: all 0.15s ease;
		border-bottom: 3px solid transparent;
		font-family: 'Open Sans', sans-serif;
	}

	.woocommerce-MyAccount-navigation-link a:hover {
		color: #0984ae;
	}

	.woocommerce-MyAccount-navigation-link.is-active a {
		color: #0984ae;
		border-bottom-color: #0984ae;
	}

	.woocommerce-MyAccount-navigation-link--customer-logout a {
		color: #999;
	}

	.woocommerce-MyAccount-navigation-link--customer-logout a:hover {
		color: #dc3545;
	}

	@media (max-width: 768px) {
		.dashboard__nav-secondary ul {
			flex-direction: column;
		}

		.woocommerce-MyAccount-navigation-link a {
			padding: 12px 15px;
			border-bottom: none;
			border-left: 3px solid transparent;
		}

		.woocommerce-MyAccount-navigation-link.is-active a {
			border-bottom: none;
			border-left-color: #0984ae;
			background: #f9f9f9;
		}
	}
</style>
