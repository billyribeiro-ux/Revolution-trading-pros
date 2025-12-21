<script lang="ts">
	/**
	 * Dashboard - Billing Address Page - Simpler Trading EXACT
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /dashboard/account/edit-address
	 * Shows billing and shipping addresses with edit links
	 *
	 * @version 1.0.0 (Simpler Trading Exact / December 2025)
	 */

	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { authStore, isAuthenticated, user } from '$lib/stores/auth';
	import { IconUser } from '$lib/icons';
	import Footer from '$lib/components/sections/Footer.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (browser && !$isAuthenticated && !$authStore.isInitializing) {
			goto('/login?redirect=/dashboard/account/edit-address', { replaceState: true });
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// DATA (would come from API)
	// ═══════════════════════════════════════════════════════════════════════════

	interface Address {
		firstName: string;
		lastName: string;
		email: string;
		phone?: string;
		address1: string;
		address2?: string;
		city: string;
		state: string;
		postcode: string;
		country: string;
	}

	// Sample billing address data
	const billingAddress: Address = {
		firstName: 'Zack',
		lastName: 'Stambowski',
		email: 'welberribeirodrums@gmail.com',
		phone: '801-721-0940',
		address1: '2417 S KIHEI RD',
		address2: '',
		city: 'KIHEI',
		state: 'HI',
		postcode: '96753-8624',
		country: 'US'
	};

	// Shipping address (empty in this example)
	const shippingAddress: Address | null = null;
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>Billing Address | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEADER
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="dashboard__header">
	<h1 class="dashboard__page-title">My Account</h1>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     MAIN CONTENT
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- Billing Address Card -->
		<div class="address-card">
			<div class="address-content">
				<div class="address-avatar">
					<IconUser size={32} />
				</div>
				<div class="address-details">
					{#if billingAddress}
						<p class="address-name">
							<strong>{billingAddress.firstName} {billingAddress.lastName}</strong>
						</p>
						<p class="address-email">{billingAddress.email}</p>
						<div class="address-lines">
							<p>{billingAddress.address1}</p>
							{#if billingAddress.address2}
								<p>{billingAddress.address2}</p>
							{/if}
							<p>{billingAddress.city}, {billingAddress.state} {billingAddress.postcode}</p>
						</div>
					{:else}
						<p class="address-empty">No billing address set.</p>
					{/if}
				</div>
			</div>
			<a href="/dashboard/account/edit-address/billing" class="address-edit-link">Edit</a>
		</div>

		<!-- Shipping Address Card -->
		<div class="address-card">
			<div class="address-content">
				<div class="address-avatar">
					<IconUser size={32} />
				</div>
				<div class="address-details">
					{#if shippingAddress}
						<p class="address-name">
							<strong>{shippingAddress.firstName} {shippingAddress.lastName}</strong>
						</p>
						{#if shippingAddress.email}
							<p class="address-email">{shippingAddress.email}</p>
						{/if}
						<div class="address-lines">
							<p>{shippingAddress.address1}</p>
							{#if shippingAddress.address2}
								<p>{shippingAddress.address2}</p>
							{/if}
							<p>{shippingAddress.city}, {shippingAddress.state} {shippingAddress.postcode}</p>
						</div>
					{:else}
						<p class="address-empty"></p>
					{/if}
				</div>
			</div>
			<a href="/dashboard/account/edit-address/shipping" class="address-edit-link">Edit</a>
		</div>
	</div>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     FOOTER
     ═══════════════════════════════════════════════════════════════════════════ -->

<Footer />

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - Simpler Trading EXACT
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background: #fff;
		border-bottom: 1px solid #e9ebed;
		padding: 20px 30px;
	}

	.dashboard__page-title {
		color: #333;
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 36px;
		font-weight: 700;
		margin: 0;
		line-height: 1.2;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		padding: 30px;
		background: #fff;
		min-height: 400px;
	}

	.dashboard__content-main {
		max-width: 600px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ADDRESS CARD - Simpler Trading EXACT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.address-card {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: 20px 24px;
		background: #fff;
		border: 1px solid #e9ebed;
		border-radius: 4px;
		margin-bottom: 16px;
	}

	.address-content {
		display: flex;
		gap: 16px;
	}

	.address-avatar {
		width: 48px;
		height: 48px;
		background: #e9ebed;
		border-radius: 4px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #999;
		flex-shrink: 0;
	}

	.address-details {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.address-name {
		margin: 0;
		font-size: 14px;
		color: #333;
	}

	.address-name strong {
		font-weight: 600;
	}

	.address-email {
		margin: 0;
		font-size: 14px;
		color: #333;
	}

	.address-lines {
		margin-top: 8px;
	}

	.address-lines p {
		margin: 0;
		font-size: 14px;
		color: #333;
		line-height: 1.5;
	}

	.address-empty {
		margin: 0;
		font-size: 14px;
		color: #999;
		font-style: italic;
	}

	.address-edit-link {
		color: #1e73be;
		font-size: 14px;
		text-decoration: none;
		flex-shrink: 0;
	}

	.address-edit-link:hover {
		text-decoration: underline;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 768px) {
		.dashboard__header {
			padding: 16px 20px;
		}

		.dashboard__page-title {
			font-size: 28px;
		}

		.dashboard__content {
			padding: 20px;
		}

		.address-card {
			padding: 16px 20px;
		}

		.address-content {
			gap: 12px;
		}

		.address-avatar {
			width: 40px;
			height: 40px;
		}
	}

	@media screen and (max-width: 480px) {
		.address-card {
			flex-direction: column;
			gap: 16px;
		}

		.address-edit-link {
			align-self: flex-end;
		}
	}
</style>
