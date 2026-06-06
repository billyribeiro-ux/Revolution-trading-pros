<script lang="ts">
	import { authStore, isAuthenticated } from '$lib/stores/auth.svelte';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import apiClient, { type Membership, type Product } from '$lib/api/client.svelte';

	let memberships = $state<Membership[]>([]);
	let products = $state<Product[]>([]);
	let loadingMemberships = $state(true);
	let loadingProducts = $state(true);

	onMount(async () => {
		// Auth guard - redirect if not authenticated (user interaction: page load)
		if (!$isAuthenticated && !$authStore.isLoading && !$authStore.isInitializing) {
			goto('/login?redirect=/account', { replaceState: true });
			return;
		}

		if ($authStore.user) {
			try {
				memberships = await apiClient.getMyMemberships();
			} catch (error) {
				console.error('Failed to load memberships:', error);
			} finally {
				loadingMemberships = false;
			}

			try {
				products = await apiClient.getMyProducts();
			} catch (error) {
				console.error('Failed to load products:', error);
			} finally {
				loadingProducts = false;
			}
		}
	});

	async function handleLogout() {
		await authStore.logout();
		// Use replaceState so back button doesn't return to protected page
		goto('/', { replaceState: true });
	}
</script>

<svelte:head>
	<title>My Account - Revolution Trading Pros</title>
	<meta name="description" content="Manage your Revolution Trading Pros account" />
</svelte:head>

<!-- SSR-safe: Always render container to prevent hydration mismatch.
     Show loading state until auth is initialized, then show content or redirect. -->
{#if $authStore.isInitializing || $authStore.isLoading}
	<!-- Loading skeleton for consistent SSR/CSR output -->
	<div class="account-page">
		<div class="account-container">
			<div class="account-card account-card--loading">
				<div class="skeleton skeleton--title"></div>
				<div class="skeleton skeleton--text"></div>
			</div>
		</div>
	</div>
{:else if $authStore.user}
	<!-- ICT11+ Fix: removed fixed viewport minimum height; parent flex container handles height. -->
	<div class="account-page">
		<div class="account-container">
			<!-- Header -->
			<div class="account-card account-header-card">
				<div class="account-header">
					<div>
						<h1 class="account-title">My Account</h1>
						<p class="account-subtitle">Welcome back, {$authStore.user?.name || 'User'}!</p>
					</div>
					<button onclick={handleLogout} class="outline-button"> Sign Out </button>
				</div>
			</div>

			<div class="account-layout">
				<!-- Left Column: Profile Info -->
				<div class="account-sidebar">
					<!-- Profile Card -->
					<div class="account-card">
						<h2 class="section-title">Profile Information</h2>
						<div class="detail-list">
							<div class="detail-item">
								<span class="detail-label">Full Name</span>
								<p class="detail-value">{$authStore.user?.name || ''}</p>
							</div>
							<div class="detail-item">
								<span class="detail-label">Email Address</span>
								<p class="detail-value">{$authStore.user?.email || ''}</p>
							</div>
							<div class="detail-item">
								<span class="detail-label">Account Status</span>
								<p class="detail-value">
									<span class="status-badge">
										<svg
											aria-hidden="true"
											class="status-icon"
											fill="currentColor"
											viewBox="0 0 20 20"
										>
											<path
												fill-rule="evenodd"
												d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
												clip-rule="evenodd"
											/>
										</svg>
										{#if $authStore.user?.role === 'developer'}
											Developer Access (Full)
										{:else if $authStore.user?.role === 'super-admin'}
											Superadmin (Full Access)
										{:else}
											Verified
										{/if}
									</span>
								</p>
							</div>
							<div class="detail-item">
								<span class="detail-label">Member Since</span>
								<p class="detail-value">
									{new Date($authStore.user?.created_at || Date.now()).toLocaleDateString('en-US', {
										month: 'long',
										year: 'numeric'
									})}
								</p>
							</div>
						</div>
						<button class="secondary-button"> Edit Profile </button>
					</div>

					<!-- Quick Actions -->
					<div class="account-card">
						<h3 class="section-title section-title--small">Quick Actions</h3>
						<div class="quick-actions">
							<a href="/live-trading-rooms/day-trading" class="quick-action">
								<div class="quick-action-content">
									<svg
										class="quick-action-icon"
										fill="none"
										stroke="currentColor"
										viewBox="0 0 24 24"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"
										/>
									</svg>
									Join Live Room
								</div>
							</a>
							<a href="/indicators" class="quick-action">
								<div class="quick-action-content">
									<svg
										class="quick-action-icon"
										fill="none"
										stroke="currentColor"
										viewBox="0 0 24 24"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4h16v12a1 1 0 01-1 1H5a1 1 0 01-1-1V4z"
										/>
									</svg>
									Browse Indicators
								</div>
							</a>
							<a href="/courses" class="quick-action">
								<div class="quick-action-content">
									<svg
										class="quick-action-icon"
										fill="none"
										stroke="currentColor"
										viewBox="0 0 24 24"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
										/>
									</svg>
									View Courses
								</div>
							</a>
						</div>
					</div>
				</div>

				<!-- Right Column: Memberships & Products -->
				<div class="account-main">
					<!-- Active Memberships -->
					<div class="account-card">
						<h2 class="section-title">Active Memberships</h2>
						{#if loadingMemberships}
							<div class="loading-panel">
								<svg
									class="loading-spinner"
									xmlns="http://www.w3.org/2000/svg"
									fill="none"
									viewBox="0 0 24 24"
								>
									<circle
										class="spinner-track"
										cx="12"
										cy="12"
										r="10"
										stroke="currentColor"
										stroke-width="4"
									></circle>
									<path
										class="spinner-head"
										fill="currentColor"
										d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
									></path>
								</svg>
							</div>
						{:else if memberships.length === 0}
							<div class="empty-state">
								<svg class="empty-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"
									/>
								</svg>
								<h3 class="empty-title">No Active Memberships</h3>
								<p class="empty-copy">Unlock premium features with a membership plan</p>
								<a href="/live-trading-rooms/day-trading" class="primary-button">
									View Membership Plans
								</a>
							</div>
						{:else}
							<div class="membership-list">
								{#each memberships as membership (membership.id)}
									<div class="membership-card">
										<div class="membership-row">
											<div>
												<h3 class="item-title">{membership.plan.name}</h3>
												<p class="item-meta">
													Started: {new Date(membership.starts_at).toLocaleDateString()}
												</p>
												{#if membership.expires_at}
													<p class="item-meta">
														Expires: {new Date(membership.expires_at).toLocaleDateString()}
													</p>
												{/if}
											</div>
											<span class="status-badge">
												{membership.status}
											</span>
										</div>
									</div>
								{/each}
							</div>
						{/if}
					</div>

					<!-- My Products -->
					<div class="account-card">
						<h2 class="section-title">My Products</h2>
						{#if loadingProducts}
							<div class="loading-panel">
								<svg
									class="loading-spinner"
									xmlns="http://www.w3.org/2000/svg"
									fill="none"
									viewBox="0 0 24 24"
								>
									<circle
										class="spinner-track"
										cx="12"
										cy="12"
										r="10"
										stroke="currentColor"
										stroke-width="4"
									></circle>
									<path
										class="spinner-head"
										fill="currentColor"
										d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
									></path>
								</svg>
							</div>
						{:else if products.length === 0}
							<div class="empty-state">
								<svg class="empty-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z"
									/>
								</svg>
								<h3 class="empty-title">No Products Yet</h3>
								<p class="empty-copy">Browse our courses and indicators to get started</p>
								<div class="empty-actions">
									<a href="/courses" class="primary-button"> Browse Courses </a>
									<a href="/indicators" class="outline-button outline-button--strong">
										View Indicators
									</a>
								</div>
							</div>
						{:else}
							<div class="product-grid">
								{#each products as product (product.id)}
									<div class="product-card">
										<div class="product-header">
											<h3 class="item-title">{product.name}</h3>
											<span class="product-type">
												{product.type}
											</span>
										</div>
										<p class="product-description">{product.description}</p>
										<a
											href={product.type === 'indicator'
												? `/indicators/${product.slug}`
												: `/courses/${product.slug}`}
											class="product-link"
										>
											View Details
										</a>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	.account-page {
		background: var(--rtp-bg, #05070d);
		padding: 3rem 1rem;
	}

	.account-container {
		width: min(100%, 80rem);
		margin: 0 auto;
	}

	.account-card {
		border: 1px solid var(--rtp-border, rgba(255, 255, 255, 0.12));
		border-radius: 1rem;
		background: var(--rtp-surface, #101826);
		padding: 1.5rem;
		box-shadow: 0 16px 36px rgba(0, 0, 0, 0.22);
	}

	.account-card--loading {
		margin-bottom: 2rem;
		padding: 2rem;
		animation: pulse 1.6s ease-in-out infinite;
	}

	.account-header-card {
		margin-bottom: 2rem;
		padding: 2rem;
	}

	.account-header {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		justify-content: space-between;
		gap: 1rem;
	}

	.account-title,
	.section-title,
	.empty-title,
	.item-title {
		color: var(--rtp-text, #f8fafc);
		font-family: var(--font-heading, inherit);
		font-weight: 700;
	}

	.account-title {
		margin: 0 0 0.5rem;
		font-size: 1.875rem;
		line-height: 2.25rem;
	}

	.account-subtitle,
	.detail-label,
	.item-meta,
	.empty-copy,
	.product-description {
		color: var(--rtp-muted, #94a3b8);
	}

	.account-layout {
		display: grid;
		grid-template-columns: 1fr;
		gap: 2rem;
	}

	.account-sidebar,
	.account-main {
		display: grid;
		align-content: start;
		gap: 1.5rem;
	}

	.section-title {
		margin: 0 0 1rem;
		font-size: 1.25rem;
		line-height: 1.75rem;
	}

	.section-title--small {
		font-size: 1.125rem;
		line-height: 1.75rem;
	}

	.detail-list {
		display: grid;
		gap: 1rem;
	}

	.detail-label {
		font-size: 0.875rem;
		font-weight: 600;
	}

	.detail-value {
		margin: 0.25rem 0 0;
		color: var(--rtp-text, #f8fafc);
	}

	.status-badge {
		display: inline-flex;
		align-items: center;
		border-radius: 999px;
		background: #dcfce7;
		color: #166534;
		padding: 0.25rem 0.75rem;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.status-icon {
		width: 1rem;
		height: 1rem;
		margin-right: 0.25rem;
	}

	.outline-button,
	.secondary-button,
	.primary-button,
	.product-link {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		border-radius: 0.5rem;
		font-weight: 600;
		text-decoration: none;
		transition:
			background-color 160ms ease,
			border-color 160ms ease,
			box-shadow 160ms ease,
			color 160ms ease,
			transform 160ms ease;
	}

	.outline-button {
		border: 2px solid var(--rtp-primary, #e6b800);
		background: transparent;
		color: var(--rtp-primary, #e6b800);
		padding: 0.5rem 1.5rem;
	}

	.outline-button:hover,
	.outline-button:focus-visible {
		background: var(--rtp-primary, #e6b800);
		color: #ffffff;
	}

	.outline-button--strong {
		padding: 0.75rem 1.5rem;
		font-weight: 700;
	}

	.secondary-button {
		width: 100%;
		margin-top: 1.5rem;
		border: 1px solid var(--rtp-border, rgba(255, 255, 255, 0.12));
		background: transparent;
		color: var(--rtp-text, #f8fafc);
		padding: 0.5rem 1rem;
	}

	.secondary-button:hover,
	.secondary-button:focus-visible,
	.quick-action:hover,
	.quick-action:focus-visible {
		background: var(--rtp-bg, #05070d);
	}

	.quick-actions,
	.membership-list {
		display: grid;
		gap: 0.5rem;
	}

	.membership-list {
		gap: 1rem;
	}

	.quick-action {
		display: block;
		border-radius: 0.5rem;
		color: var(--rtp-text, #f8fafc);
		padding: 0.75rem 1rem;
		text-decoration: none;
		transition: background-color 160ms ease;
	}

	.quick-action-content {
		display: flex;
		align-items: center;
	}

	.quick-action-icon {
		width: 1.25rem;
		height: 1.25rem;
		margin-right: 0.75rem;
		color: var(--rtp-primary, #e6b800);
	}

	.loading-panel {
		display: flex;
		justify-content: center;
		padding: 2rem 0;
	}

	.loading-spinner {
		width: 2rem;
		height: 2rem;
		color: var(--rtp-primary, #e6b800);
		animation: spin 800ms linear infinite;
	}

	.spinner-track {
		opacity: 0.25;
	}

	.spinner-head {
		opacity: 0.75;
	}

	.empty-state {
		padding: 3rem 0;
		text-align: center;
	}

	.empty-icon {
		width: 4rem;
		height: 4rem;
		margin: 0 auto 1rem;
		color: var(--rtp-muted, #94a3b8);
	}

	.empty-title {
		margin: 0 0 0.5rem;
		font-size: 1.125rem;
		line-height: 1.75rem;
	}

	.empty-copy,
	.product-description {
		margin: 0 0 1rem;
	}

	.empty-actions {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 1rem;
	}

	.primary-button {
		border: 0;
		background: linear-gradient(90deg, var(--rtp-primary, #e6b800), var(--rtp-blue, #2563eb));
		color: #ffffff;
		padding: 0.75rem 1.5rem;
		font-weight: 700;
	}

	.primary-button:hover,
	.primary-button:focus-visible {
		box-shadow: 0 14px 28px rgba(37, 99, 235, 0.22);
		transform: translateY(-1px);
	}

	.membership-card,
	.product-card {
		border: 1px solid var(--rtp-border, rgba(255, 255, 255, 0.12));
		border-radius: 0.5rem;
		padding: 1rem;
	}

	.membership-row {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 1rem;
	}

	.item-title {
		margin: 0;
		font-size: 1.125rem;
		line-height: 1.75rem;
	}

	.item-meta {
		margin: 0.25rem 0 0;
		font-size: 0.875rem;
	}

	.product-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1rem;
	}

	.product-card {
		transition:
			box-shadow 160ms ease,
			transform 160ms ease;
	}

	.product-card:hover {
		box-shadow: 0 14px 30px rgba(0, 0, 0, 0.2);
		transform: translateY(-1px);
	}

	.product-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 0.5rem;
	}

	.product-type {
		border-radius: 0.25rem;
		background: var(--rtp-primarySoft, rgba(230, 184, 0, 0.16));
		color: var(--rtp-primary, #e6b800);
		padding: 0.25rem 0.5rem;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.product-description {
		font-size: 0.875rem;
	}

	.product-link {
		background: var(--rtp-primary, #e6b800);
		color: #ffffff;
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
	}

	.product-link:hover,
	.product-link:focus-visible {
		background: var(--rtp-blue, #2563eb);
	}

	.skeleton {
		border-radius: 0.25rem;
		background: #e5e7eb;
	}

	.skeleton--title {
		width: 12rem;
		height: 2rem;
		margin-bottom: 1rem;
	}

	.skeleton--text {
		width: 8rem;
		height: 1rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@keyframes pulse {
		50% {
			opacity: 0.6;
		}
	}

	@media (min-width: 640px) {
		.product-grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}

	@media (min-width: 768px) {
		.account-header {
			flex-direction: row;
			align-items: center;
		}
	}

	@media (min-width: 1024px) {
		.account-layout {
			grid-template-columns: minmax(0, 1fr) minmax(0, 2fr);
		}
	}
</style>
