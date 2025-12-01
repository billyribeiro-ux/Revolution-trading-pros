<script lang="ts">
	/**
	 * Dashboard Layout
	 * Wraps all dashboard routes with the sidebar navigation
	 * Matches WordPress WooCommerce My Account structure
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { authStore, isAuthenticated, user } from '$lib/stores/auth';
	import { getUserMemberships, type UserMembership } from '$lib/api/user-memberships';
	import DashboardSidebar from '$lib/components/dashboard/DashboardSidebar.svelte';
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	// State
	let memberships = $state<UserMembership[]>([]);
	let isLoading = $state(true);
	let isSidebarOpen = $state(false);
	let isSidebarCollapsed = $state(false);

	// Load memberships on mount
	onMount(async () => {
		// Check authentication
		if (!$isAuthenticated && !$authStore.isInitializing) {
			goto('/login?redirect=' + encodeURIComponent($page.url.pathname), { replaceState: true });
			return;
		}

		try {
			const data = await getUserMemberships();
			memberships = data.memberships;
		} catch (error) {
			console.error('Failed to load memberships:', error);
		} finally {
			isLoading = false;
		}
	});

	// Toggle mobile sidebar
	function toggleSidebar() {
		isSidebarOpen = !isSidebarOpen;
		if (browser) {
			document.documentElement.classList.toggle('html--dashboard-menu-open', isSidebarOpen);
		}
	}

	// Close sidebar when route changes
	$effect(() => {
		if ($page.url.pathname && isSidebarOpen) {
			isSidebarOpen = false;
			if (browser) {
				document.documentElement.classList.remove('html--dashboard-menu-open');
			}
		}
	});
</script>

<svelte:head>
	<title>{$page.data?.title || 'Dashboard'} | Revolution Trading Pros</title>
</svelte:head>

{#if $isAuthenticated || $authStore.isInitializing}
	<div class="woocommerce-account woocommerce-page logged-in">
		<div class="woocommerce">
			<!-- Sidebar Navigation -->
			<nav
				class="woocommerce-MyAccount-navigation dashboard-sidebar"
				class:is-open={isSidebarOpen}
			>
				<DashboardSidebar
					{memberships}
					isCollapsed={isSidebarCollapsed}
				/>
			</nav>

			<!-- Main Content Area -->
			<div class="woocommerce-MyAccount-content">
				{#if isLoading}
					<div class="dashboard-loading">
						<div class="loading-spinner"></div>
						<p>Loading...</p>
					</div>
				{:else}
					{@render children()}
				{/if}
			</div>
		</div>
	</div>

	<!-- Mobile Dashboard Toggle -->
	<div class="csdashboard__toggle">
		<button
			type="button"
			class="dashboard__toggle-button"
			onclick={toggleSidebar}
			aria-label="Toggle dashboard menu"
			aria-expanded={isSidebarOpen}
		>
			<span class="dashboard__toggle-button-icon">
				<span></span>
				<span></span>
				<span></span>
			</span>
			<span class="framework__toggle-button-label">Menu</span>
		</button>
	</div>

	<!-- Overlay for mobile -->
	<button
		class="csdashboard__overlay"
		class:is-visible={isSidebarOpen}
		onclick={toggleSidebar}
		aria-label="Close menu"
	></button>
{:else}
	<div class="dashboard-loading">
		<p>Redirecting to login...</p>
	</div>
{/if}

<style>
	/* WooCommerce Account Structure */
	.woocommerce-account {
		min-height: 100vh;
		background: #f4f4f4;
	}

	.woocommerce-account .woocommerce {
		display: flex;
		margin: 0 auto;
		width: 100%;
	}

	/* Sidebar Navigation */
	.woocommerce-MyAccount-navigation {
		margin-top: 0;
	}

	.woocommerce-MyAccount-navigation.dashboard-sidebar {
		display: flex;
		flex: 0 0 auto;
		flex-flow: row;
		background-color: #0f2d41;
		min-height: calc(100vh - 80px);
		transition: all 0.3s ease-in-out;
	}

	/* Main Content Area */
	.woocommerce-MyAccount-content {
		width: 100%;
		background: #f4f4f4;
		position: relative;
		min-height: calc(100vh - 80px);
	}

	/* Loading State */
	.dashboard-loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 400px;
		color: #666;
	}

	.loading-spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e5e7eb;
		border-top-color: #0984ae;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 16px;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	/* Mobile Dashboard Toggle */
	.csdashboard__toggle {
		background-color: #0d2532;
		bottom: 0;
		height: 50px;
		left: 0;
		line-height: 50px;
		padding: 0;
		position: fixed;
		right: 0;
		z-index: 100010;
		display: none;
	}

	.dashboard__toggle-button {
		appearance: none;
		background: none;
		color: #fff;
		height: 50px;
		overflow: hidden;
		padding: 0 10px 0 50px;
		position: relative;
		border-radius: 10px;
		border: 1px solid #fff;
		cursor: pointer;
		margin: 0 auto;
		display: block;
	}

	.dashboard__toggle-button-icon {
		height: 50px;
		left: 20px;
		position: absolute;
		top: 40%;
		margin-top: -7px;
		width: 50px;
	}

	.dashboard__toggle-button-icon span {
		background-color: #fff;
		border-radius: 0;
		display: block;
		height: 2px;
		left: 0;
		opacity: 1;
		position: absolute;
		transform: rotate(0);
		transform-origin: left center;
		transition: all 0.15s ease-in-out;
		width: 20px;
	}

	.dashboard__toggle-button-icon span:first-child {
		top: 0;
	}

	.dashboard__toggle-button-icon span:nth-child(2) {
		top: 6px;
	}

	.dashboard__toggle-button-icon span:nth-child(3) {
		top: 12px;
	}

	.framework__toggle-button-label {
		font-size: 12px;
		position: relative;
		text-transform: uppercase;
		top: 0;
	}

	/* Overlay */
	.csdashboard__overlay {
		background-color: rgba(0, 0, 0, 0.65);
		bottom: 0;
		left: 0;
		opacity: 0;
		position: fixed;
		right: 0;
		top: 0;
		transition: all 0.3s ease-in-out;
		visibility: hidden;
		z-index: 100009;
		border: none;
		cursor: pointer;
	}

	.csdashboard__overlay.is-visible {
		opacity: 1;
		visibility: visible;
	}

	/* Responsive */
	@media screen and (max-width: 980px) {
		.csdashboard__toggle {
			display: flex;
			align-items: center;
			justify-content: center;
		}

		.woocommerce-MyAccount-navigation.dashboard-sidebar {
			width: 280px;
			bottom: 50px;
			left: 0;
			opacity: 0;
			overflow-x: hidden;
			overflow-y: auto;
			position: fixed;
			top: 0;
			visibility: hidden;
			z-index: 1000011;
		}

		.woocommerce-MyAccount-navigation.dashboard-sidebar.is-open {
			opacity: 1;
			visibility: visible;
		}

		.woocommerce-MyAccount-content {
			width: 100%;
			padding-bottom: 60px;
		}
	}

	/* Global styles for dashboard menu open state */
	:global(.html--dashboard-menu-open) {
		overflow: hidden;
	}

	:global(.html--dashboard-menu-open .csdashboard__overlay) {
		opacity: 1;
		visibility: visible;
	}

	:global(.html--dashboard-menu-open .woocommerce-MyAccount-navigation.dashboard-sidebar) {
		opacity: 1;
		visibility: visible;
	}
</style>
