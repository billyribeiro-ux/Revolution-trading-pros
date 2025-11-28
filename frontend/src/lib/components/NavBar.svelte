<script lang="ts">
	/**
	 * NavBar - Desktop Only (Clean Version)
	 * SvelteKit 5 | Svelte 5 Runes | Zero CLS
	 */
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import type { ComponentType } from 'svelte';
	import {
		IconUser,
		IconLogout,
		IconShoppingCart,
		IconBook,
		IconChartLine,
		IconSettings,
		IconChevronDown
	} from '@tabler/icons-svelte';
	import { authStore, user, isAuthenticated } from '$lib/stores/auth';
	import { cartItemCount, hasCartItems } from '$lib/stores/cart';
	import { logout as logoutApi } from '$lib/api/auth';

	// Props
	interface Props {
		isAdmin?: boolean;
	}
	let { isAdmin = false }: Props = $props();

	// Types
	interface SubMenuItem {
		href: string;
		label: string;
	}

	interface NavItem {
		id: string;
		label: string;
		href?: string;
		submenu?: SubMenuItem[];
	}

	interface UserMenuItem {
		href: string;
		label: string;
		icon: ComponentType;
	}

	// Dashboard menu items (shown when authenticated)
	const dashboardItems: SubMenuItem[] = [
		{ href: '/dashboard', label: 'Dashboard Home' },
		{ href: '/dashboard/courses', label: 'My Courses' },
		{ href: '/dashboard/indicators', label: 'My Indicators' },
		{ href: '/dashboard/account', label: 'My Account' }
	];

	// Navigation Config
	const navItems: NavItem[] = [
		{
			id: 'live',
			label: 'Live Trading Rooms',
			submenu: [
				{ href: '/live-trading-rooms/day-trading', label: 'Day Trading Room' },
				{ href: '/live-trading-rooms/swing-trading', label: 'Swing Trading Room' },
				{ href: '/live-trading-rooms/small-accounts', label: 'Small Accounts Room' }
			]
		},
		{
			id: 'alerts',
			label: 'Alert Services',
			submenu: [
				{ href: '/alerts/spx-profit-pulse', label: 'SPX Profit Pulse' },
				{ href: '/alerts/explosive-swings', label: 'Explosive Swings' }
			]
		},
		{ id: 'mentorship', label: 'Mentorship', href: '/mentorship' },
		{
			id: 'store',
			label: 'Store',
			submenu: [
				{ href: '/courses', label: 'Courses' },
				{ href: '/indicators', label: 'Indicators' }
			]
		},
		{ id: 'mission', label: 'Our Mission', href: '/our-mission' },
		{ id: 'about', label: 'About', href: '/about' },
		{ id: 'blog', label: 'Blogs', href: '/blog' },
		{
			id: 'resources',
			label: 'Resources',
			submenu: [
				{ href: '/resources/etf-stocks-list', label: 'ETF Stocks List' },
				{ href: '/resources/stock-indexes-list', label: 'Stock Indexes List' }
			]
		}
	];

	const userMenuItems: UserMenuItem[] = [
		{ href: '/dashboard/courses', label: 'My Courses', icon: IconBook },
		{ href: '/dashboard/indicators', label: 'My Indicators', icon: IconChartLine },
		{ href: '/dashboard/account', label: 'My Account', icon: IconSettings }
	];

	// State
	let activeDropdown = $state<string | null>(null);
	let isUserMenuOpen = $state(false);

	// Derived
	const currentPath = $derived($page.url.pathname);

	function isActiveRoute(item: NavItem): boolean {
		if (item.href && currentPath === item.href) return true;
		if (item.submenu) {
			return item.submenu.some(sub => currentPath === sub.href || currentPath.startsWith(sub.href + '/'));
		}
		return false;
	}

	// Handlers
	function handleDropdownEnter(id: string) {
		activeDropdown = id;
	}

	function handleDropdownLeave() {
		activeDropdown = null;
	}

	async function handleLogout() {
		isUserMenuOpen = false;
		try {
			await logoutApi();
		} catch (e) {
			console.error('Logout error:', e);
		}
		authStore.clearAuth();
		await goto('/login');
	}

	// Close menus on click outside
	function handleClickOutside(event: MouseEvent) {
		const target = event.target as HTMLElement;
		if (isUserMenuOpen && !target.closest('[data-user-menu]')) {
			isUserMenuOpen = false;
		}
		if (activeDropdown && !target.closest('.nav-item')) {
			activeDropdown = null;
		}
	}

	onMount(() => {
		document.addEventListener('click', handleClickOutside);
		return () => document.removeEventListener('click', handleClickOutside);
	});
</script>

<header class="header">
	<div class="container">
		<!-- LOGO -->
		<a href="/" class="logo">
			<img
				src="/revolution-trading-pros.png"
				alt="Revolution Trading Pros"
				width="200"
				height="50"
			/>
		</a>

		<!-- NAV -->
		<nav class="nav">
			{#each navItems as item (item.id)}
				{#if item.submenu}
					<!-- svelte-ignore a11y_no_static_element_interactions -->
					<div
						class="nav-item"
						onmouseenter={() => handleDropdownEnter(item.id)}
						onmouseleave={handleDropdownLeave}
					>
						<button
							class="nav-link"
							class:active={isActiveRoute(item)}
							aria-expanded={activeDropdown === item.id}
						>
							{item.label}
							<IconChevronDown size={14} />
						</button>
						{#if activeDropdown === item.id}
							<div class="dropdown">
								{#each item.submenu as sub (sub.href)}
									<a href={sub.href} class="dropdown-item" onclick={() => activeDropdown = null}>
										{sub.label}
									</a>
								{/each}
							</div>
						{/if}
					</div>
				{:else}
					<a href={item.href} class="nav-link" class:active={currentPath === item.href}>
						{item.label}
					</a>
				{/if}
			{/each}

			<!-- Dashboard dropdown (when logged in) - right after Resources -->
			{#if $isAuthenticated}
				<!-- svelte-ignore a11y_no_static_element_interactions -->
				<div
					class="nav-item"
					onmouseenter={() => handleDropdownEnter('dashboard')}
					onmouseleave={handleDropdownLeave}
				>
					<button
						class="nav-link"
						class:active={currentPath.startsWith('/dashboard')}
						aria-expanded={activeDropdown === 'dashboard'}
					>
						Dashboard
						<IconChevronDown size={14} />
					</button>
					{#if activeDropdown === 'dashboard'}
						<div class="dropdown">
							{#each dashboardItems as item (item.href)}
								<a href={item.href} class="dropdown-item" onclick={() => activeDropdown = null}>
									{item.label}
								</a>
							{/each}
							<div class="dropdown-divider"></div>
							<button class="dropdown-item logout" onclick={handleLogout}>
								Logout
							</button>
						</div>
					{/if}
				</div>
			{/if}
		</nav>

		<!-- ACTIONS -->
		<div class="actions">
			{#if $hasCartItems}
				<a href="/cart" class="cart-btn" aria-label="Cart ({$cartItemCount} items)">
					<IconShoppingCart size={22} />
					{#if $cartItemCount > 0}
						<span class="badge">{$cartItemCount}</span>
					{/if}
				</a>
			{/if}

			{#if !$isAuthenticated}
				<a href="/login" class="login-btn">Login</a>
				<a href="/get-started" class="cta-btn">Get Started</a>
			{/if}
		</div>
	</div>
</header>

<style>
	/* Header */
	.header {
		position: sticky;
		top: 0;
		z-index: 1000; /* Below admin toolbar (10100) */
		height: 72px;
		background: #151F31;
		border-bottom: 1px solid rgba(255,255,255,0.1);
	}

	/* When admin toolbar is present, adjust sticky position */
	:global(.has-admin-toolbar) .header {
		top: 0; /* Stays at top of its container (which has padding-top) */
	}

	/* Container */
	.container {
		display: flex;
		align-items: center;
		height: 100%;
		max-width: 1440px;
		margin: 0 auto;
		padding: 0 24px;
		gap: 32px;
	}

	/* Logo */
	.logo {
		flex-shrink: 0;
		display: flex;
		align-items: center;
	}

	.logo img {
		width: 200px;
		height: 50px;
		object-fit: contain;
	}

	/* Nav */
	.nav {
		display: flex;
		align-items: center;
		gap: 4px;
		flex: 1;
	}

	.nav-item {
		position: relative;
	}

	.nav-link {
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 12px 14px;
		color: rgba(255,255,255,0.7);
		font-weight: 500;
		font-size: 14px;
		text-decoration: none;
		background: none;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		transition: color 0.15s, background 0.15s;
	}

	.nav-link:hover,
	.nav-link.active {
		color: #facc15;
		background: rgba(255,255,255,0.05);
	}

	/* Dropdown */
	.dropdown {
		position: absolute;
		top: 100%;
		left: 0;
		min-width: 220px;
		padding: 8px;
		background: #151F31;
		border: 1px solid rgba(255,255,255,0.1);
		border-radius: 12px;
		box-shadow: 0 10px 40px rgba(0,0,0,0.4);
	}

	.dropdown-item {
		display: block;
		padding: 12px 16px;
		color: rgba(255,255,255,0.7);
		font-size: 14px;
		text-decoration: none;
		border-radius: 8px;
		transition: background 0.15s, color 0.15s;
	}

	.dropdown-item:hover {
		background: rgba(250,204,21,0.1);
		color: #facc15;
	}

	.dropdown-item.logout {
		width: 100%;
		color: #f87171;
		border: none;
		cursor: pointer;
	}

	.dropdown-item.logout:hover {
		background: rgba(248,113,113,0.1);
		color: #f87171;
	}

	.dropdown-divider {
		height: 1px;
		margin: 8px 0;
		background: rgba(255,255,255,0.1);
	}

	/* Actions */
	.actions {
		display: flex;
		align-items: center;
		gap: 12px;
		margin-left: auto;
	}

	.cart-btn {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		color: #a78bfa;
		background: rgba(139,92,246,0.1);
		border-radius: 10px;
		text-decoration: none;
	}

	.badge {
		position: absolute;
		top: -4px;
		right: -4px;
		min-width: 20px;
		height: 20px;
		padding: 0 6px;
		background: #facc15;
		color: #151F31;
		font-size: 11px;
		font-weight: 700;
		line-height: 20px;
		text-align: center;
		border-radius: 10px;
	}

	.login-btn {
		display: flex;
		align-items: center;
		height: 44px;
		padding: 0 20px;
		color: white;
		font-weight: 600;
		font-size: 14px;
		text-decoration: none;
		background: transparent;
		border: 1px solid rgba(255,255,255,0.25);
		border-radius: 10px;
		transition: background 0.15s, border-color 0.15s;
	}

	.login-btn:hover {
		background: rgba(255,255,255,0.05);
		border-color: rgba(255,255,255,0.4);
	}

	.cta-btn {
		display: flex;
		align-items: center;
		height: 44px;
		padding: 0 24px;
		color: #0a101c;
		font-weight: 700;
		font-size: 14px;
		text-decoration: none;
		background: linear-gradient(135deg, #facc15, #eab308);
		border-radius: 10px;
		box-shadow: 0 4px 12px rgba(250,204,21,0.3);
		transition: transform 0.15s, box-shadow 0.15s;
	}

	.cta-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 6px 20px rgba(250,204,21,0.4);
	}

	</style>
