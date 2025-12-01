<script lang="ts">
	/**
	 * NavBar - Self-Contained Navigation Component
	 * All-in-one: Desktop nav + Mobile menu with snippets
	 */
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import {
		IconShoppingCart,
		IconMenu2,
		IconX,
		IconChevronDown
	} from '@tabler/icons-svelte';
	import { authStore, isAuthenticated, user } from '$lib/stores/auth';
	import { cartItemCount, hasCartItems } from '$lib/stores/cart';
	import { logout as logoutApi } from '$lib/api/auth';

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

	const dashboardItems: SubMenuItem[] = [
		{ href: '/dashboard', label: 'Dashboard Home' },
		{ href: '/dashboard/courses', label: 'My Courses' },
		{ href: '/dashboard/indicators', label: 'My Indicators' },
		{ href: '/dashboard/account', label: 'My Account' }
	];

	// State
	let mobileMenuOpen = $state(false);
	let activeDropdown = $state<string | null>(null);
	let mobileExpandedSection = $state<string | null>(null);
	let isScrolled = $state(false);

	// Derived
	const currentPath = $derived($page.url.pathname);

	// Handlers
	function toggleMobileMenu() {
		mobileMenuOpen = !mobileMenuOpen;
		if (mobileMenuOpen) {
			document.body.style.overflow = 'hidden';
		} else {
			document.body.style.overflow = '';
			mobileExpandedSection = null;
		}
	}

	function closeMobileMenu() {
		mobileMenuOpen = false;
		document.body.style.overflow = '';
		mobileExpandedSection = null;
	}

	function toggleDropdown(id: string) {
		activeDropdown = activeDropdown === id ? null : id;
	}

	function toggleMobileSection(id: string) {
		mobileExpandedSection = mobileExpandedSection === id ? null : id;
	}

	function handleNavClick(href: string) {
		closeMobileMenu();
		activeDropdown = null;
		goto(href);
	}

	async function handleLogout() {
		closeMobileMenu();
		activeDropdown = null;
		try {
			await logoutApi();
		} catch (e) {
			console.error('Logout error:', e);
		}
		authStore.clearAuth();
		goto('/login');
	}

	function handleClickOutside(event: MouseEvent) {
		const target = event.target as HTMLElement;
		if (activeDropdown && !target.closest('[data-dropdown]')) {
			activeDropdown = null;
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			if (mobileMenuOpen) closeMobileMenu();
			if (activeDropdown) activeDropdown = null;
		}
	}

	function handleScroll() {
		isScrolled = window.scrollY > 20;
	}

	onMount(() => {
		document.addEventListener('click', handleClickOutside);
		document.addEventListener('keydown', handleKeydown);
		window.addEventListener('scroll', handleScroll, { passive: true });
		handleScroll();
		
		return () => {
			document.removeEventListener('click', handleClickOutside);
			document.removeEventListener('keydown', handleKeydown);
			window.removeEventListener('scroll', handleScroll);
			document.body.style.overflow = '';
		};
	});
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     MOBILE MENU - Direct render, no snippets
     ═══════════════════════════════════════════════════════════════════════════ -->
{#if mobileMenuOpen}
	<!-- Backdrop -->
	<button 
		class="mobile-backdrop" 
		onclick={closeMobileMenu}
		aria-label="Close menu"
		type="button"
	></button>

	<!-- Panel -->
	<div class="mobile-panel" role="dialog" aria-modal="true" aria-label="Navigation menu">
		<!-- Header -->
		<div class="mobile-header">
			<span class="mobile-title">Menu</span>
			<button class="mobile-close" onclick={closeMobileMenu} aria-label="Close menu" type="button">
				<IconX size={24} />
			</button>
		</div>

		<!-- User Info -->
		{#if $isAuthenticated && $user}
			<div class="mobile-user">
				<div class="mobile-avatar">{$user.name?.charAt(0).toUpperCase() || 'U'}</div>
				<div class="mobile-user-info">
					<span class="mobile-user-name">{$user.name || 'User'}</span>
					<span class="mobile-user-email">{$user.email || ''}</span>
				</div>
			</div>
		{/if}

		<!-- Nav Items -->
		<nav class="mobile-nav">
			{#each navItems as item (item.id)}
				{#if item.submenu}
					<div class="mobile-nav-group">
						<button
							class="mobile-nav-item"
							class:expanded={mobileExpandedSection === item.id}
							onclick={() => toggleMobileSection(item.id)}
							aria-expanded={mobileExpandedSection === item.id}
							type="button"
						>
							<span>{item.label}</span>
							<IconChevronDown size={18} class={mobileExpandedSection === item.id ? 'rotate' : ''} />
						</button>
						{#if mobileExpandedSection === item.id}
							<div class="mobile-submenu">
								{#each item.submenu as sub (sub.href)}
									<a
										href={sub.href}
										class="mobile-submenu-item"
										class:active={currentPath === sub.href}
										onclick={() => handleNavClick(sub.href)}
									>
										{sub.label}
									</a>
								{/each}
							</div>
						{/if}
					</div>
				{:else}
					<a
						href={item.href}
						class="mobile-nav-item"
						class:active={currentPath === item.href}
						onclick={() => item.href && handleNavClick(item.href)}
					>
						{item.label}
					</a>
				{/if}
			{/each}

			<!-- Dashboard (authenticated) -->
			{#if $isAuthenticated}
				<div class="mobile-divider"></div>
				<div class="mobile-nav-group">
					<button
						class="mobile-nav-item"
						class:expanded={mobileExpandedSection === 'dashboard'}
						onclick={() => toggleMobileSection('dashboard')}
						aria-expanded={mobileExpandedSection === 'dashboard'}
						type="button"
					>
						<span>Dashboard</span>
						<IconChevronDown size={18} class={mobileExpandedSection === 'dashboard' ? 'rotate' : ''} />
					</button>
					{#if mobileExpandedSection === 'dashboard'}
						<div class="mobile-submenu">
							{#each dashboardItems as item (item.href)}
								<a
									href={item.href}
									class="mobile-submenu-item"
									class:active={currentPath === item.href}
									onclick={() => handleNavClick(item.href)}
								>
									{item.label}
								</a>
							{/each}
						</div>
					{/if}
				</div>
			{/if}
		</nav>

		<!-- Footer Actions -->
		<div class="mobile-footer">
			{#if $hasCartItems}
				<a href="/cart" class="mobile-cart" onclick={() => handleNavClick('/cart')}>
					<IconShoppingCart size={20} />
					<span>Cart</span>
					{#if $cartItemCount > 0}
						<span class="mobile-badge">{$cartItemCount}</span>
					{/if}
				</a>
			{/if}

			{#if $isAuthenticated}
				<button class="mobile-logout" onclick={handleLogout}>Logout</button>
			{:else}
				<a href="/login" class="mobile-login" onclick={() => handleNavClick('/login')}>Login</a>
				<a href="/get-started" class="mobile-cta" onclick={() => handleNavClick('/get-started')}>Get Started</a>
			{/if}
		</div>
	</div>
{/if}

<!-- ═══════════════════════════════════════════════════════════════════════════
     MAIN NAVBAR
     ═══════════════════════════════════════════════════════════════════════════ -->
<header class="navbar" class:scrolled={isScrolled}>
	<div class="navbar-container">
		<!-- LOGO -->
		<a href="/" class="logo" aria-label="Revolution Trading Pros Home">
			<img
				src="/revolution-trading-pros.png"
				alt="Revolution Trading Pros"
				width="200"
				height="50"
				loading="eager"
				fetchpriority="high"
			/>
		</a>

		<!-- DESKTOP NAV -->
		<nav class="desktop-nav" aria-label="Main navigation">
			{#each navItems as item (item.id)}
				{#if item.submenu}
					<div class="dropdown" data-dropdown={item.id}>
						<button
							class="dropdown-trigger"
							class:active={item.submenu?.some(s => currentPath === s.href)}
							class:open={activeDropdown === item.id}
							onclick={() => toggleDropdown(item.id)}
							aria-expanded={activeDropdown === item.id}
							aria-haspopup="true"
							type="button"
						>
							<span>{item.label}</span>
							<IconChevronDown size={14} class={activeDropdown === item.id ? 'rotate' : ''} />
						</button>
						{#if activeDropdown === item.id}
							<div class="dropdown-menu" role="menu">
								{#each item.submenu as sub (sub.href)}
									<a
										href={sub.href}
										class="dropdown-item"
										class:active={currentPath === sub.href}
										role="menuitem"
										onclick={() => { activeDropdown = null; }}
									>
										{sub.label}
									</a>
								{/each}
							</div>
						{/if}
					</div>
				{:else}
					<a
						href={item.href}
						class="nav-link"
						class:active={currentPath === item.href}
					>
						{item.label}
					</a>
				{/if}
			{/each}

			{#if $isAuthenticated}
				<div class="dropdown" data-dropdown="dashboard">
					<button
						class="dropdown-trigger"
						class:active={currentPath.startsWith('/dashboard')}
						class:open={activeDropdown === 'dashboard'}
						onclick={() => toggleDropdown('dashboard')}
						aria-expanded={activeDropdown === 'dashboard'}
						aria-haspopup="true"
						type="button"
					>
						<span>Dashboard</span>
						<IconChevronDown size={14} class={activeDropdown === 'dashboard' ? 'rotate' : ''} />
					</button>
					{#if activeDropdown === 'dashboard'}
						<div class="dropdown-menu" role="menu">
							{#each dashboardItems as sub (sub.href)}
								<a
									href={sub.href}
									class="dropdown-item"
									class:active={currentPath === sub.href}
									role="menuitem"
									onclick={() => { activeDropdown = null; }}
								>
									{sub.label}
								</a>
							{/each}
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
						<span class="cart-badge">{$cartItemCount}</span>
					{/if}
				</a>
			{/if}

			{#if !$isAuthenticated}
				<a href="/login" class="login-btn">Login</a>
				<a href="/get-started" class="cta-btn">Get Started</a>
			{/if}

			<!-- HAMBURGER -->
			<button
				class="hamburger"
				onclick={toggleMobileMenu}
				aria-label={mobileMenuOpen ? 'Close menu' : 'Open menu'}
				aria-expanded={mobileMenuOpen}
			>
				<IconMenu2 size={24} />
			</button>
		</div>
	</div>
</header>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   NAVBAR - Fixed Height, Sticky
	   ═══════════════════════════════════════════════════════════════════════════ */
	.navbar {
		position: sticky;
		top: 0;
		z-index: 1000;
		width: 100%;
		height: 72px;
		background: #151f31;
		border-bottom: 1px solid rgba(255, 255, 255, 0.08);
		transition: background-color 0.2s ease, box-shadow 0.2s ease;
	}

	.navbar.scrolled {
		background: rgba(21, 31, 49, 0.97);
		backdrop-filter: blur(12px);
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
	}

	.navbar-container {
		display: flex;
		align-items: center;
		justify-content: space-between;
		height: 100%;
		max-width: 1440px;
		margin: 0 auto;
		padding: 0 32px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LOGO - Fixed Width, Zero CLS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.logo {
		display: flex;
		align-items: center;
		flex-shrink: 0;
		width: 200px;
	}

	.logo img {
		width: 200px;
		height: 50px;
		object-fit: contain;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DESKTOP NAV
	   ═══════════════════════════════════════════════════════════════════════════ */
	.desktop-nav {
		display: flex;
		align-items: center;
		gap: 4px;
		flex: 1;
		margin: 0 24px;
	}

	.nav-link {
		display: flex;
		align-items: center;
		padding: 10px 14px;
		color: rgba(255, 255, 255, 0.75);
		font-size: 14px;
		font-weight: 500;
		text-decoration: none;
		border-radius: 8px;
		transition: color 0.15s, background 0.15s;
	}

	.nav-link:hover, .nav-link.active {
		color: #facc15;
		background: rgba(255, 255, 255, 0.05);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DROPDOWN
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dropdown {
		position: relative;
	}

	.dropdown-trigger {
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 10px 14px;
		color: rgba(255, 255, 255, 0.75);
		font-size: 14px;
		font-weight: 500;
		background: none;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		transition: color 0.15s, background 0.15s;
	}

	.dropdown-trigger:hover, .dropdown-trigger.active, .dropdown-trigger.open {
		color: #facc15;
		background: rgba(255, 255, 255, 0.05);
	}

	.dropdown-trigger :global(.rotate) {
		transform: rotate(180deg);
	}

	.dropdown-menu {
		position: absolute;
		top: calc(100% + 8px);
		left: 0;
		min-width: 220px;
		padding: 8px;
		background: #1a2538;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.4);
		z-index: 100;
	}

	.dropdown-item {
		display: block;
		padding: 12px 16px;
		color: rgba(255, 255, 255, 0.8);
		font-size: 14px;
		text-decoration: none;
		border-radius: 8px;
		transition: background 0.15s, color 0.15s;
	}

	.dropdown-item:hover, .dropdown-item.active {
		background: rgba(250, 204, 21, 0.1);
		color: #facc15;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACTIONS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.actions {
		display: flex;
		align-items: center;
		gap: 12px;
		flex-shrink: 0;
	}

	.cart-btn {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		color: #a78bfa;
		background: rgba(139, 92, 246, 0.1);
		border-radius: 10px;
		text-decoration: none;
	}

	.cart-btn:hover {
		background: rgba(139, 92, 246, 0.2);
	}

	.cart-badge {
		position: absolute;
		top: -4px;
		right: -4px;
		min-width: 20px;
		height: 20px;
		padding: 0 6px;
		background: #facc15;
		color: #151f31;
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
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		background: transparent;
		border: 1px solid rgba(255, 255, 255, 0.25);
		border-radius: 10px;
		transition: background 0.15s, border-color 0.15s;
	}

	.login-btn:hover {
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(255, 255, 255, 0.4);
	}

	.cta-btn {
		display: flex;
		align-items: center;
		height: 44px;
		padding: 0 24px;
		color: #0a101c;
		font-size: 14px;
		font-weight: 700;
		text-decoration: none;
		background: linear-gradient(135deg, #facc15, #eab308);
		border-radius: 10px;
		box-shadow: 0 4px 12px rgba(250, 204, 21, 0.3);
		transition: transform 0.15s, box-shadow 0.15s;
	}

	.cta-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 6px 20px rgba(250, 204, 21, 0.4);
	}

	/* Hamburger - hidden on desktop */
	.hamburger {
		display: none;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		color: rgba(255, 255, 255, 0.8);
		background: transparent;
		border: none;
		border-radius: 10px;
		cursor: pointer;
	}

	.hamburger:hover {
		background: rgba(255, 255, 255, 0.1);
		color: white;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MOBILE MENU STYLES
	   ═══════════════════════════════════════════════════════════════════════════ */
	.mobile-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
		z-index: 9998;
		border: none;
		cursor: pointer;
	}

	.mobile-panel {
		position: fixed;
		top: 0;
		right: 0;
		width: min(85vw, 360px);
		height: 100%;
		display: flex;
		flex-direction: column;
		background: #151f31;
		border-left: 1px solid rgba(255, 255, 255, 0.1);
		z-index: 9999;
		animation: slideIn 0.25s ease-out;
	}

	@keyframes slideIn {
		from { transform: translateX(100%); }
		to { transform: translateX(0); }
	}

	.mobile-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		height: 64px;
		padding: 0 20px;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		flex-shrink: 0;
	}

	.mobile-title {
		font-size: 18px;
		font-weight: 600;
		color: white;
	}

	.mobile-close {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		color: rgba(255, 255, 255, 0.7);
		background: transparent;
		border: none;
		border-radius: 10px;
		cursor: pointer;
	}

	.mobile-close:hover {
		background: rgba(255, 255, 255, 0.1);
		color: white;
	}

	.mobile-user {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 16px 20px;
		background: rgba(255, 255, 255, 0.03);
	}

	.mobile-avatar {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		background: linear-gradient(135deg, #facc15, #eab308);
		color: #0a101c;
		font-size: 18px;
		font-weight: 700;
		border-radius: 50%;
	}

	.mobile-user-info {
		display: flex;
		flex-direction: column;
	}

	.mobile-user-name {
		font-size: 15px;
		font-weight: 600;
		color: white;
	}

	.mobile-user-email {
		font-size: 13px;
		color: rgba(255, 255, 255, 0.5);
	}

	.mobile-nav {
		flex: 1;
		overflow-y: auto;
		padding: 8px 0;
	}

	.mobile-nav-group {
		display: flex;
		flex-direction: column;
	}

	.mobile-nav-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		padding: 14px 20px;
		color: rgba(255, 255, 255, 0.85);
		font-size: 15px;
		font-weight: 500;
		text-decoration: none;
		background: transparent;
		border: none;
		cursor: pointer;
		text-align: left;
	}

	.mobile-nav-item:hover {
		background: rgba(255, 255, 255, 0.05);
		color: white;
	}

	.mobile-nav-item.active {
		color: #facc15;
	}

	.mobile-nav-item :global(.rotate) {
		transform: rotate(180deg);
	}

	.mobile-submenu {
		display: flex;
		flex-direction: column;
		background: rgba(0, 0, 0, 0.15);
	}

	.mobile-submenu-item {
		padding: 12px 20px 12px 36px;
		color: rgba(255, 255, 255, 0.7);
		font-size: 14px;
		text-decoration: none;
	}

	.mobile-submenu-item:hover {
		background: rgba(255, 255, 255, 0.05);
		color: white;
	}

	.mobile-submenu-item.active {
		color: #facc15;
	}

	.mobile-divider {
		height: 1px;
		margin: 12px 20px;
		background: rgba(255, 255, 255, 0.1);
	}

	.mobile-footer {
		display: flex;
		flex-direction: column;
		gap: 10px;
		padding: 16px 20px;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
		flex-shrink: 0;
	}

	.mobile-cart {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		height: 48px;
		color: #a78bfa;
		font-size: 15px;
		font-weight: 600;
		text-decoration: none;
		background: rgba(139, 92, 246, 0.1);
		border-radius: 10px;
	}

	.mobile-badge {
		min-width: 22px;
		height: 22px;
		padding: 0 6px;
		background: #facc15;
		color: #0a101c;
		font-size: 12px;
		font-weight: 700;
		line-height: 22px;
		text-align: center;
		border-radius: 11px;
	}

	.mobile-login {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 48px;
		color: white;
		font-size: 15px;
		font-weight: 600;
		text-decoration: none;
		background: transparent;
		border: 1px solid rgba(255, 255, 255, 0.25);
		border-radius: 10px;
	}

	.mobile-cta {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 48px;
		color: #0a101c;
		font-size: 15px;
		font-weight: 700;
		text-decoration: none;
		background: linear-gradient(135deg, #facc15, #eab308);
		border-radius: 10px;
	}

	.mobile-logout {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 48px;
		color: #f87171;
		font-size: 15px;
		font-weight: 600;
		background: rgba(248, 113, 113, 0.1);
		border: none;
		border-radius: 10px;
		cursor: pointer;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE: TABLET & BELOW (< 1024px)
	   ═══════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 1023px) {
		.navbar {
			height: 64px;
		}

		.navbar-container {
			padding: 0 20px;
		}

		.logo {
			width: 180px;
		}

		.logo img {
			width: 180px;
			height: 45px;
		}

		/* Hide desktop nav */
		.desktop-nav {
			display: none;
		}

		/* Hide login */
		.login-btn {
			display: none;
		}

		/* Show hamburger */
		.hamburger {
			display: flex;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE: MOBILE (< 768px)
	   ═══════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 767px) {
		.navbar {
			height: 60px;
		}

		.navbar-container {
			padding: 0 16px;
		}

		.logo {
			width: 150px;
		}

		.logo img {
			width: 150px;
			height: 38px;
		}

		/* Hide CTA */
		.cta-btn {
			display: none;
		}

		.cart-btn, .hamburger {
			width: 40px;
			height: 40px;
		}
	}
</style>
