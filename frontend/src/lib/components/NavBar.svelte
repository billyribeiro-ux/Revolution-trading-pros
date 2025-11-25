<script lang="ts">
	/**
	 * NavBar Component - Google L8+ Production Standard
	 * ══════════════════════════════════════════════════════════════════════════════
	 * SvelteKit 5 | CSS Grid Layout | Zero CLS | Static Logo | No Overflow
	 * ══════════════════════════════════════════════════════════════════════════════
	 */
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import type { ComponentType } from 'svelte';
	import {
		IconMenu2,
		IconX,
		IconUser,
		IconLogout,
		IconShoppingCart,
		IconBook,
		IconChartLine,
		IconSettings,
		IconChevronDown,
		IconChevronRight
	} from '@tabler/icons-svelte';
	import { authStore, user, isAuthenticated } from '$lib/stores/auth';
	import { cartItemCount, hasCartItems } from '$lib/stores/cart';
	import { logout as logoutApi } from '$lib/api/auth';

	// ═══════════════════════════════════════════════════════════════════════════════
	// Types
	// ═══════════════════════════════════════════════════════════════════════════════
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

	// ═══════════════════════════════════════════════════════════════════════════════
	// Constants
	// ═══════════════════════════════════════════════════════════════════════════════
	const BREAKPOINT_DESKTOP = 1024;

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

	// ═══════════════════════════════════════════════════════════════════════════════
	// State - Svelte 5 Runes
	// ═══════════════════════════════════════════════════════════════════════════════
	let isMobileMenuOpen = $state(false);
	let activeDropdown = $state<string | null>(null);
	let activeMobileSubmenu = $state<string | null>(null);
	let isUserMenuOpen = $state(false);
	let windowWidth = $state(BREAKPOINT_DESKTOP + 1);
	let isScrolled = $state(false);

	// DOM refs
	let mobileNavRef = $state<HTMLElement | null>(null);
	let hamburgerRef = $state<HTMLButtonElement | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════════
	// Derived
	// ═══════════════════════════════════════════════════════════════════════════════
	const isDesktop = $derived(windowWidth >= BREAKPOINT_DESKTOP);

	// ═══════════════════════════════════════════════════════════════════════════════
	// Handlers
	// ═══════════════════════════════════════════════════════════════════════════════
	function toggleMobileMenu(): void {
		isMobileMenuOpen = !isMobileMenuOpen;
		activeMobileSubmenu = null;
		document.body.style.overflow = isMobileMenuOpen ? 'hidden' : '';
	}

	function closeMobileMenu(): void {
		isMobileMenuOpen = false;
		activeMobileSubmenu = null;
		document.body.style.overflow = '';
	}

	function handleDocumentClick(event: MouseEvent): void {
		const target = event.target as HTMLElement;
		if (!target.closest('[data-dropdown]')) {
			activeDropdown = null;
		}
		if (!target.closest('[data-user-menu]')) {
			isUserMenuOpen = false;
		}
	}

	function handleKeydown(event: KeyboardEvent): void {
		if (event.key === 'Escape') {
			if (isMobileMenuOpen) {
				closeMobileMenu();
				hamburgerRef?.focus();
			} else {
				activeDropdown = null;
				isUserMenuOpen = false;
			}
		}
	}

	async function handleLogout(): Promise<void> {
		closeMobileMenu();
		try {
			await logoutApi();
		} catch (e) {
			console.error('Logout error:', e);
		}
		authStore.clearAuth();
		await goto('/login');
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// Effects
	// ═══════════════════════════════════════════════════════════════════════════════
	$effect(() => {
		if (isDesktop && isMobileMenuOpen) {
			closeMobileMenu();
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════════
	// Lifecycle
	// ═══════════════════════════════════════════════════════════════════════════════
	onMount(() => {
		windowWidth = window.innerWidth;
		isScrolled = window.scrollY > 20;

		const handleResize = () => {
			windowWidth = window.innerWidth;
		};

		const handleScroll = () => {
			isScrolled = window.scrollY > 20;
		};

		window.addEventListener('resize', handleResize, { passive: true });
		window.addEventListener('scroll', handleScroll, { passive: true });
		document.addEventListener('click', handleDocumentClick);
		document.addEventListener('keydown', handleKeydown);

		return () => {
			window.removeEventListener('resize', handleResize);
			window.removeEventListener('scroll', handleScroll);
			document.removeEventListener('click', handleDocumentClick);
			document.removeEventListener('keydown', handleKeydown);
			document.body.style.overflow = '';
		};
	});
</script>

<header class="header" class:header--scrolled={isScrolled}>
	<div class="header__container">
		<!-- COLUMN 1: LOGO - Fixed width, immutable -->
		<div class="header__logo">
			<a href="/" class="logo" onclick={closeMobileMenu}>
				<img
					src="/revolution-trading-pros.png"
					alt="Revolution Trading Pros"
					width="200"
					height="68"
					class="logo__image"
				/>
			</a>
		</div>

		<!-- COLUMN 2: NAV - Only on desktop, fills remaining space -->
		{#if isDesktop}
			<nav class="header__nav">
				<div class="nav">
					{#each navItems as item (item.id)}
						{#if item.submenu}
							<div class="nav__dropdown" data-dropdown={item.id}>
								<button
									type="button"
									class="nav__link"
									aria-expanded={activeDropdown === item.id}
									aria-haspopup="true"
									onclick={(e) => {
										e.stopPropagation();
										activeDropdown = activeDropdown === item.id ? null : item.id;
									}}
								>
									<span>{item.label}</span>
									<IconChevronDown
										size={16}
										stroke={2.5}
										style="transform: rotate({activeDropdown === item.id ? 180 : 0}deg); transition: transform 0.2s;"
									/>
								</button>

								{#if activeDropdown === item.id}
									<div class="dropdown">
										{#each item.submenu as sub (sub.href)}
											<a
												href={sub.href}
												class="dropdown__item"
												class:dropdown__item--active={$page.url.pathname === sub.href}
												onclick={() => (activeDropdown = null)}
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
								class="nav__link"
								class:nav__link--active={$page.url.pathname === item.href}
							>
								{item.label}
							</a>
						{/if}
					{/each}
				</div>
			</nav>
		{/if}

		<!-- COLUMN 3: ACTIONS - Auto width, fixed position -->
		<div class="header__actions">
			{#if $hasCartItems}
				<a href="/cart" class="cart-btn" aria-label="Shopping cart">
					<IconShoppingCart size={22} />
					{#if $cartItemCount > 0}
						<span class="cart-btn__badge">{$cartItemCount}</span>
					{/if}
				</a>
			{/if}

			{#if isDesktop}
				{#if $isAuthenticated}
					<div class="user-menu" data-user-menu>
						<button
							type="button"
							class="user-menu__trigger"
							aria-expanded={isUserMenuOpen}
							aria-haspopup="true"
							onclick={(e) => {
								e.stopPropagation();
								isUserMenuOpen = !isUserMenuOpen;
							}}
						>
							<IconUser size={18} />
							<span class="user-menu__name">{$user?.name || 'Account'}</span>
							<IconChevronDown size={12} />
						</button>

						{#if isUserMenuOpen}
							<div class="user-menu__dropdown">
								{#each userMenuItems as menuItem (menuItem.href)}
									{@const Icon = menuItem.icon}
									<a
										href={menuItem.href}
										class="user-menu__item"
										onclick={() => (isUserMenuOpen = false)}
									>
										<Icon size={16} />
										<span>{menuItem.label}</span>
									</a>
								{/each}
								<button type="button" class="user-menu__item user-menu__item--danger" onclick={handleLogout}>
									<IconLogout size={16} />
									<span>Logout</span>
								</button>
							</div>
						{/if}
					</div>
				{:else}
					<a href="/login" class="login-btn">
						<IconUser size={18} />
						<span>Login</span>
					</a>
				{/if}
			{/if}

			{#if !isDesktop}
				<button
					type="button"
					bind:this={hamburgerRef}
					class="hamburger"
					aria-label={isMobileMenuOpen ? 'Close menu' : 'Open menu'}
					aria-expanded={isMobileMenuOpen}
					onclick={toggleMobileMenu}
				>
					{#if isMobileMenuOpen}
						<IconX size={26} />
					{:else}
						<IconMenu2 size={26} />
					{/if}
				</button>
			{/if}
		</div>
	</div>
</header>

<!-- MOBILE PANEL -->
{#if !isDesktop && isMobileMenuOpen}
	<div class="mobile-overlay" onclick={closeMobileMenu} aria-hidden="true"></div>

	<nav class="mobile-panel" bind:this={mobileNavRef} aria-label="Mobile navigation">
		<div class="mobile-panel__header">
			<button type="button" class="mobile-panel__close" onclick={closeMobileMenu} aria-label="Close menu">
				<IconX size={24} />
			</button>
		</div>

		<div class="mobile-panel__body">
			{#each navItems as item (item.id)}
				{#if item.submenu}
					<div class="mobile-nav__group">
						<button
							type="button"
							class="mobile-nav__link"
							aria-expanded={activeMobileSubmenu === item.id}
							onclick={() => (activeMobileSubmenu = activeMobileSubmenu === item.id ? null : item.id)}
						>
							<span>{item.label}</span>
							<IconChevronRight
								size={18}
								stroke={2.5}
								style="transform: rotate({activeMobileSubmenu === item.id ? 90 : 0}deg); transition: transform 0.2s;"
							/>
						</button>

						{#if activeMobileSubmenu === item.id}
							<div class="mobile-nav__submenu">
								{#each item.submenu as sub (sub.href)}
									<a
										href={sub.href}
										class="mobile-nav__sublink"
										class:mobile-nav__sublink--active={$page.url.pathname === sub.href}
										onclick={closeMobileMenu}
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
						class="mobile-nav__link"
						class:mobile-nav__link--active={$page.url.pathname === item.href}
						onclick={closeMobileMenu}
					>
						{item.label}
					</a>
				{/if}
			{/each}

			<div class="mobile-panel__footer">
				{#if $isAuthenticated}
					<div class="mobile-user">
						<div class="mobile-user__name">{$user?.name}</div>
						{#each userMenuItems as menuItem (menuItem.href)}
							<a href={menuItem.href} class="mobile-nav__sublink" onclick={closeMobileMenu}>
								{menuItem.label}
							</a>
						{/each}
						<button type="button" class="mobile-nav__sublink mobile-nav__sublink--danger" onclick={handleLogout}>
							Logout
						</button>
					</div>
				{:else}
					<a href="/login" class="login-btn login-btn--full" onclick={closeMobileMenu}>
						Login
					</a>
				{/if}
			</div>
		</div>
	</nav>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════════
	 * VARIABLES
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	:root {
		--header-height: 80px;
		--header-height-mobile: 64px;
		--logo-width: 200px;
		--logo-width-mobile: 140px;
		--logo-height: 68px;
		--logo-height-mobile: 48px;
		--color-bg: #05142b;
		--color-text: #e5e7eb;
		--color-text-muted: #94a3b8;
		--color-accent: #facc15;
		--color-danger: #f87171;
		--color-border: rgba(148, 163, 253, 0.15);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * HEADER - Fixed height, sticky
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.header {
		position: sticky;
		top: 0;
		left: 0;
		right: 0;
		z-index: 1000;
		height: var(--header-height);
		background: rgba(5, 20, 43, 0.95);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		border-bottom: 1px solid var(--color-border);
		transition: background-color 0.2s, box-shadow 0.2s;
	}

	.header--scrolled {
		background: rgba(5, 20, 43, 0.99);
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
	}

	@media (max-width: 768px) {
		.header {
			height: var(--header-height-mobile);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * CONTAINER - CSS GRID for bulletproof layout
	 * 3 columns: [logo: fixed] [nav: 1fr] [actions: auto]
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.header__container {
		display: grid;
		grid-template-columns: var(--logo-width) 1fr auto;
		align-items: center;
		height: 100%;
		max-width: 1440px;
		margin: 0 auto;
		padding: 0 24px;
		gap: 16px;
	}

	@media (max-width: 1024px) {
		.header__container {
			grid-template-columns: var(--logo-width-mobile) 1fr auto;
		}
	}

	@media (max-width: 768px) {
		.header__container {
			padding: 0 16px;
			gap: 12px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * LOGO - Fixed dimensions, NEVER changes
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.header__logo {
		width: var(--logo-width);
		height: var(--logo-height);
		overflow: hidden; /* Ensure nothing escapes */
	}

	.logo {
		display: block;
		width: var(--logo-width);
		height: var(--logo-height);
	}

	.logo__image {
		display: block;
		width: var(--logo-width);
		height: var(--logo-height);
		object-fit: contain;
		object-position: left center;
	}

	@media (max-width: 1024px) {
		.header__logo {
			width: var(--logo-width-mobile);
			height: var(--logo-height-mobile);
		}

		.logo {
			width: var(--logo-width-mobile);
			height: var(--logo-height-mobile);
		}

		.logo__image {
			width: var(--logo-width-mobile);
			height: var(--logo-height-mobile);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * NAV - Fills middle column, clips overflow
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.header__nav {
		min-width: 0; /* Allow grid item to shrink */
		overflow: hidden; /* Clip any overflow */
	}

	.nav {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 2px;
		overflow: hidden; /* Double protection */
	}

	.nav__dropdown {
		position: relative;
	}

	.nav__link {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		padding: 8px 12px;
		color: var(--color-text);
		font-family: 'Montserrat', system-ui, sans-serif;
		font-weight: 700;
		font-size: 0.9rem;
		text-decoration: none;
		white-space: nowrap;
		background: transparent;
		border: 1px solid transparent;
		border-radius: 999px;
		cursor: pointer;
		transition: color 0.15s, background-color 0.15s, border-color 0.15s;
	}

	.nav__link:hover,
	.nav__link:focus-visible,
	.nav__link--active {
		color: var(--color-accent);
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(250, 204, 21, 0.2);
	}

	.nav__link:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * DROPDOWN MENU
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.dropdown {
		position: absolute;
		top: 100%;
		left: 50%;
		transform: translateX(-50%);
		margin-top: 8px;
		min-width: 200px;
		background: var(--color-bg);
		border: 1px solid var(--color-border);
		border-radius: 12px;
		padding: 8px;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.4);
		z-index: 100;
	}

	.dropdown__item {
		display: block;
		padding: 10px 14px;
		color: var(--color-text-muted);
		font-size: 0.9rem;
		text-decoration: none;
		border-radius: 8px;
		transition: background-color 0.15s, color 0.15s;
	}

	.dropdown__item:hover,
	.dropdown__item--active {
		background: rgba(250, 204, 21, 0.1);
		color: var(--color-accent);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * ACTIONS - Auto width column
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.header__actions {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	/* Cart Button */
	.cart-btn {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		color: #a78bfa;
		background: rgba(139, 92, 246, 0.1);
		border-radius: 8px;
		transition: background-color 0.15s;
	}

	.cart-btn:hover {
		background: rgba(139, 92, 246, 0.2);
	}

	.cart-btn__badge {
		position: absolute;
		top: -4px;
		right: -4px;
		min-width: 18px;
		height: 18px;
		padding: 0 5px;
		background: var(--color-accent);
		color: var(--color-bg);
		font-size: 0.7rem;
		font-weight: 700;
		line-height: 18px;
		text-align: center;
		border-radius: 999px;
	}

	/* Login Button */
	.login-btn {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: linear-gradient(135deg, #2563eb, #1d4ed8);
		color: white;
		font-weight: 600;
		font-size: 0.9rem;
		text-decoration: none;
		border-radius: 10px;
		transition: box-shadow 0.2s;
	}

	.login-btn:hover {
		box-shadow: 0 0 20px rgba(37, 99, 235, 0.5);
	}

	.login-btn--full {
		width: 100%;
		justify-content: center;
		padding: 14px 20px;
	}

	/* User Menu */
	.user-menu {
		position: relative;
	}

	.user-menu__trigger {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 8px 14px;
		background: rgba(250, 204, 21, 0.1);
		border: 1px solid rgba(250, 204, 21, 0.3);
		color: var(--color-accent);
		font-size: 0.9rem;
		font-family: inherit;
		border-radius: 8px;
		cursor: pointer;
		transition: background-color 0.15s, border-color 0.15s;
	}

	.user-menu__trigger:hover {
		background: rgba(250, 204, 21, 0.15);
		border-color: rgba(250, 204, 21, 0.5);
	}

	.user-menu__name {
		max-width: 100px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.user-menu__dropdown {
		position: absolute;
		top: 100%;
		right: 0;
		margin-top: 8px;
		width: 200px;
		background: var(--color-bg);
		border: 1px solid var(--color-border);
		border-radius: 12px;
		padding: 8px;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.4);
		z-index: 100;
	}

	.user-menu__item {
		display: flex;
		align-items: center;
		gap: 10px;
		width: 100%;
		padding: 10px 12px;
		color: var(--color-text);
		font-size: 0.9rem;
		font-family: inherit;
		text-decoration: none;
		text-align: left;
		background: transparent;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		transition: background-color 0.15s;
	}

	.user-menu__item:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.user-menu__item--danger {
		color: var(--color-danger);
	}

	.user-menu__item--danger:hover {
		background: rgba(248, 113, 113, 0.1);
	}

	/* Hamburger */
	.hamburger {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		color: white;
		background: transparent;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		transition: background-color 0.15s;
	}

	.hamburger:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * MOBILE OVERLAY
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.mobile-overlay {
		position: fixed;
		inset: 0;
		z-index: 1001;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * MOBILE PANEL
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.mobile-panel {
		position: fixed;
		top: 0;
		right: 0;
		bottom: 0;
		z-index: 1002;
		width: min(85vw, 360px);
		background: var(--color-bg);
		border-left: 1px solid var(--color-border);
		box-shadow: -10px 0 40px rgba(0, 0, 0, 0.5);
		display: flex;
		flex-direction: column;
		animation: slideIn 0.25s ease-out;
	}

	@keyframes slideIn {
		from { transform: translateX(100%); }
		to { transform: translateX(0); }
	}

	.mobile-panel__header {
		display: flex;
		justify-content: flex-end;
		padding: 16px;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.mobile-panel__close {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		color: var(--color-text);
		background: transparent;
		border: none;
		border-radius: 8px;
		cursor: pointer;
	}

	.mobile-panel__close:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	.mobile-panel__body {
		flex: 1;
		overflow-y: auto;
		padding: 16px;
	}

	.mobile-panel__footer {
		margin-top: 24px;
		padding-top: 24px;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
	}

	/* Mobile Nav Links */
	.mobile-nav__link {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		padding: 14px 0;
		color: var(--color-text);
		font-size: 1.05rem;
		font-weight: 600;
		font-family: inherit;
		text-decoration: none;
		background: transparent;
		border: none;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		cursor: pointer;
		text-align: left;
	}

	.mobile-nav__link:hover,
	.mobile-nav__link--active {
		color: var(--color-accent);
	}

	.mobile-nav__submenu {
		padding: 8px 0 8px 16px;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.mobile-nav__sublink {
		display: block;
		width: 100%;
		padding: 10px 0;
		color: var(--color-text-muted);
		font-size: 0.95rem;
		font-family: inherit;
		text-decoration: none;
		text-align: left;
		background: transparent;
		border: none;
		cursor: pointer;
	}

	.mobile-nav__sublink:hover,
	.mobile-nav__sublink--active {
		color: var(--color-accent);
	}

	.mobile-nav__sublink--danger {
		color: var(--color-danger);
	}

	/* Mobile User */
	.mobile-user {
		padding: 16px;
		background: rgba(255, 255, 255, 0.03);
		border-radius: 12px;
	}

	.mobile-user__name {
		margin-bottom: 12px;
		color: var(--color-accent);
		font-weight: 600;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * GLOBAL
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	:global(html) {
		scroll-padding-top: var(--header-height);
	}

	@media (max-width: 768px) {
		:global(html) {
			scroll-padding-top: var(--header-height-mobile);
		}
	}
</style>