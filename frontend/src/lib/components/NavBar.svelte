<script lang="ts">
	/**
	 * NavBar - Clean, Simple, Bulletproof
	 * SvelteKit 5 | Static Logo | Zero CLS
	 */
	import { page } from '$app/stores';
	import {
		IconMenu2,
		IconX,
		IconUser,
		IconLogout,
		IconShoppingCart,
		IconChevronDown,
		IconChevronRight,
		IconBook,
		IconChartLine,
		IconSettings
	} from '@tabler/icons-svelte';
	import { user, isAuthenticated } from '$lib/stores/auth';
	import { cartItemCount, hasCartItems } from '$lib/stores/cart';
	import { logout as logoutApi } from '$lib/api/auth';

	// Nav config
	const navItems = [
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

	const userMenuItems = [
		{ href: '/dashboard/courses', label: 'My Courses', icon: IconBook },
		{ href: '/dashboard/indicators', label: 'My Indicators', icon: IconChartLine },
		{ href: '/dashboard/account', label: 'My Account', icon: IconSettings }
	];

	// State
	let mobileOpen = $state(false);
	let activeDropdown = $state<string | null>(null);
	let mobileSubmenu = $state<string | null>(null);
	let userMenuOpen = $state(false);

	// Responsive
	let innerWidth = $state(1024);
	const isDesktop = $derived(innerWidth >= 1024);

	// Handlers
	function toggleMobile() {
		mobileOpen = !mobileOpen;
		if (mobileOpen) {
			document.body.style.overflow = 'hidden';
		} else {
			document.body.style.overflow = '';
			mobileSubmenu = null;
		}
	}

	function closeMobile() {
		mobileOpen = false;
		document.body.style.overflow = '';
		mobileSubmenu = null;
	}

	function handleLogout() {
		logoutApi();
		closeMobile();
		userMenuOpen = false;
	}

	function isActive(item: typeof navItems[0]): boolean {
		const path = $page.url.pathname;
		if (item.href && (path === item.href || path.startsWith(item.href + '/'))) return true;
		if (item.submenu?.some(s => path === s.href || path.startsWith(s.href + '/'))) return true;
		return false;
	}

	// Close dropdowns on outside click
	function handleClickOutside(e: MouseEvent) {
		const target = e.target as HTMLElement;
		if (!target.closest('[data-dropdown]') && !target.closest('[data-user-menu]')) {
			activeDropdown = null;
			userMenuOpen = false;
		}
	}
</script>

<svelte:window bind:innerWidth onclick={handleClickOutside} />

<header class="header">
	<div class="container">
		<!-- Logo - FIXED SIZE, NEVER MOVES -->
		<a href="/" class="logo">
			<img
				src="/revolution-trading-pros.png"
				alt="Revolution Trading Pros"
				width="200"
				height="50"
			/>
		</a>

		<!-- Desktop Nav -->
		{#if isDesktop}
			<nav class="nav">
				{#each navItems as item (item.id)}
					{#if item.submenu}
						<div class="nav-item" data-dropdown={item.id}>
							<button
								class="nav-link"
								class:active={isActive(item)}
								aria-expanded={activeDropdown === item.id}
								onmouseenter={() => activeDropdown = item.id}
								onmouseleave={() => activeDropdown = null}
								onclick={() => activeDropdown = activeDropdown === item.id ? null : item.id}
							>
								{item.label}
								<IconChevronDown size={14} />
							</button>
							<!-- svelte-ignore a11y_no_static_element_interactions -->
							<div
								class="dropdown"
								class:open={activeDropdown === item.id}
								onmouseenter={() => activeDropdown = item.id}
								onmouseleave={() => activeDropdown = null}
							>
								{#each item.submenu as sub (sub.href)}
									<a href={sub.href} class="dropdown-item" onclick={() => activeDropdown = null}>
										{sub.label}
									</a>
								{/each}
							</div>
						</div>
					{:else}
						<a href={item.href} class="nav-link" class:active={isActive(item)}>
							{item.label}
						</a>
					{/if}
				{/each}
			</nav>
		{/if}

		<!-- Actions -->
		<div class="actions">
			{#if $hasCartItems}
				<a href="/cart" class="cart-btn" aria-label="Cart ({$cartItemCount} items)">
					<IconShoppingCart size={22} />
					{#if $cartItemCount > 0}
						<span class="badge">{$cartItemCount}</span>
					{/if}
				</a>
			{/if}

			{#if isDesktop}
				<a href="/login" class="cta-btn">Get Started</a>

				{#if $isAuthenticated}
					<div class="user-menu" data-user-menu>
						<button
							class="user-trigger"
							aria-expanded={userMenuOpen}
							onclick={() => userMenuOpen = !userMenuOpen}
						>
							<IconUser size={18} />
							<span class="user-name">{$user?.name || 'Account'}</span>
							<IconChevronDown size={12} />
						</button>
						{#if userMenuOpen}
							<div class="user-dropdown">
								{#each userMenuItems as item (item.href)}
									{@const Icon = item.icon}
									<a href={item.href} class="user-item" onclick={() => userMenuOpen = false}>
										<Icon size={16} />
										{item.label}
									</a>
								{/each}
								<button class="user-item danger" onclick={handleLogout}>
									<IconLogout size={16} />
									Logout
								</button>
							</div>
						{/if}
					</div>
				{:else}
					<a href="/login" class="login-btn">
						<IconUser size={18} />
						Login
					</a>
				{/if}
			{:else}
				<button class="hamburger" aria-label="Menu" onclick={toggleMobile}>
					{#if mobileOpen}
						<IconX size={28} />
					{:else}
						<IconMenu2 size={28} />
					{/if}
				</button>
			{/if}
		</div>
	</div>
</header>

<!-- Mobile Menu -->
{#if !isDesktop && mobileOpen}
	<div
		class="mobile-overlay"
		onclick={closeMobile}
		onkeydown={(e) => e.key === 'Escape' && closeMobile()}
		role="button"
		tabindex="-1"
		aria-label="Close menu"
	></div>
	<nav class="mobile-panel">
		<div class="mobile-header">
			<button class="mobile-close" onclick={closeMobile} aria-label="Close">
				<IconX size={24} />
			</button>
		</div>
		<div class="mobile-body">
			{#each navItems as item (item.id)}
				{#if item.submenu}
					<button
						class="mobile-link"
						onclick={() => mobileSubmenu = mobileSubmenu === item.id ? null : item.id}
					>
						{item.label}
						<span class="chevron" class:open={mobileSubmenu === item.id}>
							<IconChevronRight size={18} />
						</span>
					</button>
					{#if mobileSubmenu === item.id}
						<div class="mobile-submenu">
							{#each item.submenu as sub (sub.href)}
								<a href={sub.href} class="mobile-sublink" onclick={closeMobile}>
									{sub.label}
								</a>
							{/each}
						</div>
					{/if}
				{:else}
					<a href={item.href} class="mobile-link" onclick={closeMobile}>
						{item.label}
					</a>
				{/if}
			{/each}

			<div class="mobile-footer">
				{#if $isAuthenticated}
					{#each userMenuItems as item (item.href)}
						<a href={item.href} class="mobile-sublink" onclick={closeMobile}>
							{item.label}
						</a>
					{/each}
					<button class="mobile-sublink danger" onclick={handleLogout}>
						Logout
					</button>
				{:else}
					<a href="/login" class="mobile-cta" onclick={closeMobile}>
						Get Started
					</a>
				{/if}
			</div>
		</div>
	</nav>
{/if}

<style>
	/* Variables */
	:root {
		--header-h: 112px;
		--header-h-mobile: 80px;
		--logo-w: 200px;
		--logo-h: 50px;
		--nav-bg: #05142b;
		--nav-text: #e5e7eb;
		--nav-muted: #94a3b8;
		--nav-accent: #facc15;
	}

	/* Header */
	.header {
		position: sticky;
		top: 0;
		z-index: 1000;
		height: var(--header-h);
		background: var(--nav-bg);
		border-bottom: 1px solid rgba(255,255,255,0.1);
	}

	@media (max-width: 1023px) {
		.header { height: var(--header-h-mobile); }
	}

	/* Container */
	.container {
		display: flex;
		align-items: center;
		justify-content: space-between;
		height: 100%;
		max-width: 1440px;
		margin: 0 auto;
		padding: 0 24px;
		gap: 24px;
	}

	@media (max-width: 767px) {
		.container { padding: 0 16px; gap: 16px; }
	}

	/* Logo - BULLETPROOF */
	.logo {
		flex-shrink: 0;
		width: var(--logo-w);
		height: var(--logo-h);
	}

	.logo img {
		display: block;
		width: var(--logo-w);
		height: var(--logo-h);
		object-fit: contain;
		object-position: left center;
	}

	/* Desktop Nav */
	.nav {
		display: flex;
		align-items: center;
		gap: 4px;
	}

	.nav-item {
		position: relative;
	}

	.nav-link {
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 12px 14px;
		color: var(--nav-text);
		font-weight: 600;
		font-size: 0.875rem;
		text-decoration: none;
		background: none;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		transition: color 0.15s, background 0.15s;
	}

	.nav-link:hover, .nav-link.active {
		color: var(--nav-accent);
		background: rgba(255,255,255,0.05);
	}

	/* Dropdown */
	.dropdown {
		position: absolute;
		top: 100%;
		left: 50%;
		transform: translateX(-50%);
		min-width: 200px;
		padding: 8px;
		background: var(--nav-bg);
		border: 1px solid rgba(255,255,255,0.1);
		border-radius: 12px;
		box-shadow: 0 10px 40px rgba(0,0,0,0.4);
		opacity: 0;
		visibility: hidden;
		transition: opacity 0.2s, visibility 0.2s;
	}

	.dropdown.open {
		opacity: 1;
		visibility: visible;
	}

	.dropdown-item {
		display: block;
		padding: 12px 16px;
		color: var(--nav-muted);
		font-size: 0.9rem;
		text-decoration: none;
		border-radius: 8px;
		transition: background 0.15s, color 0.15s;
	}

	.dropdown-item:hover {
		background: rgba(250,204,21,0.1);
		color: var(--nav-accent);
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
		background: var(--nav-accent);
		color: var(--nav-bg);
		font-size: 0.7rem;
		font-weight: 700;
		line-height: 20px;
		text-align: center;
		border-radius: 999px;
	}

	.cta-btn {
		padding: 14px 28px;
		background: linear-gradient(135deg, #facc15, #eab308);
		color: #0a101c;
		font-weight: 700;
		font-size: 0.95rem;
		text-decoration: none;
		border-radius: 12px;
		box-shadow: 0 4px 12px rgba(250,204,21,0.3);
		transition: transform 0.2s, box-shadow 0.2s;
	}

	.cta-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 6px 20px rgba(250,204,21,0.5);
	}

	.login-btn {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 20px;
		background: linear-gradient(135deg, #2563eb, #1d4ed8);
		color: white;
		font-weight: 600;
		text-decoration: none;
		border-radius: 10px;
	}

	/* User Menu */
	.user-menu {
		position: relative;
	}

	.user-trigger {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 14px;
		background: rgba(250,204,21,0.1);
		border: 1px solid rgba(250,204,21,0.3);
		color: var(--nav-accent);
		font-size: 0.9rem;
		border-radius: 8px;
		cursor: pointer;
	}

	.user-name {
		max-width: 100px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.user-dropdown {
		position: absolute;
		top: calc(100% + 8px);
		right: 0;
		width: 200px;
		padding: 8px;
		background: var(--nav-bg);
		border: 1px solid rgba(255,255,255,0.1);
		border-radius: 12px;
		box-shadow: 0 10px 40px rgba(0,0,0,0.4);
	}

	.user-item {
		display: flex;
		align-items: center;
		gap: 10px;
		width: 100%;
		padding: 10px 12px;
		color: var(--nav-text);
		font-size: 0.9rem;
		text-decoration: none;
		text-align: left;
		background: none;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		transition: background 0.15s;
	}

	.user-item:hover {
		background: rgba(255,255,255,0.05);
	}

	.user-item.danger {
		color: #f87171;
	}

	.user-item.danger:hover {
		background: rgba(248,113,113,0.1);
	}

	/* Hamburger */
	.hamburger {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		color: white;
		background: none;
		border: none;
		cursor: pointer;
	}

	/* Mobile Overlay */
	.mobile-overlay {
		position: fixed;
		inset: 0;
		z-index: 1001;
		background: rgba(0,0,0,0.6);
	}

	/* Mobile Panel */
	.mobile-panel {
		position: fixed;
		top: 0;
		right: 0;
		bottom: 0;
		z-index: 1002;
		width: min(85vw, 360px);
		background: var(--nav-bg);
		display: flex;
		flex-direction: column;
		animation: slideIn 0.3s ease-out;
	}

	@keyframes slideIn {
		from { transform: translateX(100%); }
		to { transform: translateX(0); }
	}

	.mobile-header {
		display: flex;
		justify-content: flex-end;
		padding: 16px;
		border-bottom: 1px solid rgba(255,255,255,0.1);
	}

	.mobile-close {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		color: var(--nav-text);
		background: none;
		border: none;
		cursor: pointer;
	}

	.mobile-body {
		flex: 1;
		overflow-y: auto;
		padding: 16px;
	}

	.mobile-link {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		padding: 16px 0;
		color: var(--nav-text);
		font-size: 1.05rem;
		font-weight: 600;
		text-decoration: none;
		background: none;
		border: none;
		border-bottom: 1px solid rgba(255,255,255,0.1);
		cursor: pointer;
		text-align: left;
	}

	.chevron {
		transition: transform 0.2s;
	}

	.chevron.open {
		transform: rotate(90deg);
	}

	.mobile-submenu {
		padding-left: 16px;
	}

	.mobile-sublink {
		display: block;
		width: 100%;
		padding: 12px 0;
		color: var(--nav-muted);
		font-size: 0.95rem;
		text-decoration: none;
		text-align: left;
		background: none;
		border: none;
		cursor: pointer;
	}

	.mobile-sublink:hover {
		color: var(--nav-accent);
	}

	.mobile-sublink.danger {
		color: #f87171;
	}

	.mobile-footer {
		margin-top: 24px;
		padding-top: 24px;
		border-top: 1px solid rgba(255,255,255,0.1);
	}

	.mobile-cta {
		display: block;
		width: 100%;
		padding: 16px;
		background: linear-gradient(135deg, #facc15, #eab308);
		color: #0a101c;
		font-weight: 700;
		text-align: center;
		text-decoration: none;
		border-radius: 12px;
	}

	/* Scroll padding */
	:global(html) {
		scroll-padding-top: var(--header-h);
	}

	@media (max-width: 1023px) {
		:global(html) {
			scroll-padding-top: var(--header-h-mobile);
		}
	}
</style>
