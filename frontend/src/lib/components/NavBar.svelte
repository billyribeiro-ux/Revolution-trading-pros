<script lang="ts">
	/**
	 * NavBar Component - Google L8+ Production Standard
	 * ══════════════════════════════════════════════════════════════════════════════
	 * SvelteKit 5 | Zero CLS | Static Logo | Proper Flex Containment
	 * ══════════════════════════════════════════════════════════════════════════════
	 */
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { browser } from '$app/environment';
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
			label: 'Live Trading',
			submenu: [
				{ href: '/day-trading', label: 'Day Trading Room' },
				{ href: '/swing-trading', label: 'Swing Trading Room' },
				{ href: '/small-accounts', label: 'Small Accounts Room' }
			]
		},
		{
			id: 'alerts',
			label: 'Alerts',
			submenu: [
				{ href: '/spx-profit-pulse', label: 'SPX Profit Pulse' },
				{ href: '/explosive-swings', label: 'Explosive Swings' }
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
		{ id: 'about', label: 'About', href: '/about' },
		{ id: 'blog', label: 'Blog', href: '/blog' }
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

		if (isMobileMenuOpen) {
			document.body.style.overflow = 'hidden';
		} else {
			document.body.style.overflow = '';
		}
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

<header class="nav-header" class:scrolled={isScrolled}>
	<div class="nav-container">
		<!-- LOGO: FIXED dimensions, flex-shrink: 0, NEVER changes size -->
		<a href="/" class="nav-logo" onclick={closeMobileMenu}>
			<img
				src="/revolution-trading-pros.png"
				alt="Revolution Trading Pros"
				width="200"
				height="68"
				class="nav-logo-img"
			/>
		</a>

		<!-- DESKTOP NAV: flex-grow, min-width: 0, overflow: hidden -->
		{#if isDesktop}
			<nav class="nav-desktop">
				{#each navItems as item (item.id)}
					{#if item.submenu}
						<div class="nav-dropdown" data-dropdown={item.id}>
							<button
								type="button"
								class="nav-link"
								aria-expanded={activeDropdown === item.id}
								aria-haspopup="true"
								onclick={(e) => {
									e.stopPropagation();
									activeDropdown = activeDropdown === item.id ? null : item.id;
								}}
							>
								{item.label}
								<IconChevronDown
									size={14}
									style="transform: rotate({activeDropdown === item.id ? 180 : 0}deg); transition: transform 0.2s;"
								/>
							</button>

							{#if activeDropdown === item.id}
								<div class="nav-dropdown-menu">
									{#each item.submenu as sub (sub.href)}
										<a
											href={sub.href}
											class="nav-dropdown-item"
											class:active={$page.url.pathname === sub.href}
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
							class="nav-link"
							class:active={$page.url.pathname === item.href}
						>
							{item.label}
						</a>
					{/if}
				{/each}
			</nav>
		{/if}

		<!-- ACTIONS: flex-shrink: 0, fixed size -->
		<div class="nav-actions">
			{#if $hasCartItems}
				<a href="/cart" class="nav-cart" aria-label="Shopping cart">
					<IconShoppingCart size={22} />
					{#if $cartItemCount > 0}
						<span class="nav-cart-badge">{$cartItemCount}</span>
					{/if}
				</a>
			{/if}

			{#if isDesktop}
				{#if $isAuthenticated}
					<div class="nav-user" data-user-menu>
						<button
							type="button"
							class="nav-user-btn"
							aria-expanded={isUserMenuOpen}
							aria-haspopup="true"
							onclick={(e) => {
								e.stopPropagation();
								isUserMenuOpen = !isUserMenuOpen;
							}}
						>
							<IconUser size={18} />
							<span class="nav-user-name">{$user?.name || 'Account'}</span>
							<IconChevronDown size={12} />
						</button>

						{#if isUserMenuOpen}
							<div class="nav-user-menu">
								{#each userMenuItems as menuItem (menuItem.href)}
									<a
										href={menuItem.href}
										class="nav-user-item"
										onclick={() => (isUserMenuOpen = false)}
									>
										<svelte:component this={menuItem.icon} size={16} />
										{menuItem.label}
									</a>
								{/each}
								<button type="button" class="nav-user-item danger" onclick={handleLogout}>
									<IconLogout size={16} />
									Logout
								</button>
							</div>
						{/if}
					</div>
				{:else}
					<a href="/login" class="nav-login-btn">
						<IconUser size={18} />
						<span>Login</span>
					</a>
				{/if}
			{/if}

			{#if !isDesktop}
				<button
					type="button"
					bind:this={hamburgerRef}
					class="nav-hamburger"
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

<!-- MOBILE MENU -->
{#if !isDesktop && isMobileMenuOpen}
	<div class="mobile-backdrop" onclick={closeMobileMenu} aria-hidden="true"></div>

	<nav class="mobile-panel" bind:this={mobileNavRef} aria-label="Mobile navigation">
		<div class="mobile-header">
			<button type="button" class="mobile-close" onclick={closeMobileMenu} aria-label="Close menu">
				<IconX size={24} />
			</button>
		</div>

		<div class="mobile-content">
			{#each navItems as item (item.id)}
				{#if item.submenu}
					<div class="mobile-group">
						<button
							type="button"
							class="mobile-link"
							aria-expanded={activeMobileSubmenu === item.id}
							onclick={() => (activeMobileSubmenu = activeMobileSubmenu === item.id ? null : item.id)}
						>
							<span>{item.label}</span>
							<IconChevronRight
								size={18}
								style="transform: rotate({activeMobileSubmenu === item.id ? 90 : 0}deg); transition: transform 0.2s;"
							/>
						</button>

						{#if activeMobileSubmenu === item.id}
							<div class="mobile-submenu">
								{#each item.submenu as sub (sub.href)}
									<a
										href={sub.href}
										class="mobile-sublink"
										class:active={$page.url.pathname === sub.href}
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
						class="mobile-link"
						class:active={$page.url.pathname === item.href}
						onclick={closeMobileMenu}
					>
						{item.label}
					</a>
				{/if}
			{/each}

			<div class="mobile-footer">
				{#if $isAuthenticated}
					<div class="mobile-user-section">
						<div class="mobile-user-name">{$user?.name}</div>
						{#each userMenuItems as menuItem (menuItem.href)}
							<a href={menuItem.href} class="mobile-sublink" onclick={closeMobileMenu}>
								{menuItem.label}
							</a>
						{/each}
						<button type="button" class="mobile-sublink danger" onclick={handleLogout}>
							Logout
						</button>
					</div>
				{:else}
					<a href="/login" class="nav-login-btn full-width" onclick={closeMobileMenu}>
						Login
					</a>
				{/if}
			</div>
		</div>
	</nav>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════════
	 * HEADER - Fixed height, sticky, zero CLS
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.nav-header {
		position: sticky;
		top: 0;
		left: 0;
		right: 0;
		z-index: 1000;
		height: 80px;
		background: rgba(5, 20, 43, 0.95);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		border-bottom: 1px solid rgba(148, 163, 253, 0.15);
		transition: background-color 0.2s ease, box-shadow 0.2s ease;
	}

	.nav-header.scrolled {
		background: rgba(5, 20, 43, 0.99);
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
	}

	@media (max-width: 768px) {
		.nav-header {
			height: 64px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * CONTAINER - Three-column flex layout
	 * Logo (fixed) | Nav (grows/shrinks) | Actions (fixed)
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.nav-container {
		display: flex;
		align-items: center;
		height: 100%;
		max-width: 1440px;
		margin: 0 auto;
		padding: 0 24px;
	}

	@media (max-width: 768px) {
		.nav-container {
			padding: 0 16px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * LOGO - STATIC FIXED SIZE, NEVER SHRINKS, NEVER GROWS
	 * This is the KEY fix - explicit width/height, flex: 0 0 auto
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.nav-logo {
		flex: 0 0 200px; /* FIXED basis, no grow, no shrink */
		width: 200px;
		height: 68px;
		display: flex;
		align-items: center;
	}

	.nav-logo-img {
		width: 200px;
		height: 68px;
		object-fit: contain;
		object-position: left center;
	}

	@media (max-width: 768px) {
		.nav-logo {
			flex: 0 0 140px;
			width: 140px;
			height: 48px;
		}

		.nav-logo-img {
			width: 140px;
			height: 48px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * DESKTOP NAV - Grows to fill space, clips overflow, centered
	 * min-width: 0 allows flex item to shrink below content size
	 * overflow: hidden prevents content from escaping
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.nav-desktop {
		flex: 1 1 0;      /* Grow and shrink equally */
		min-width: 0;     /* CRITICAL: Allow shrinking below content */
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 4px;
		padding: 0 24px;
		overflow: hidden; /* CRITICAL: Clip overflow */
	}

	.nav-link {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		padding: 8px 14px;
		color: #e5e7eb;
		font-family: 'Montserrat', system-ui, sans-serif;
		font-weight: 600;
		font-size: 0.875rem;
		text-decoration: none;
		white-space: nowrap;
		border: 1px solid transparent;
		border-radius: 999px;
		background: transparent;
		cursor: pointer;
		transition: color 0.15s, background-color 0.15s, border-color 0.15s;
	}

	.nav-link:hover,
	.nav-link:focus-visible,
	.nav-link.active {
		color: #facc15;
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(250, 204, 21, 0.2);
	}

	.nav-link:focus-visible {
		outline: 2px solid #facc15;
		outline-offset: 2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * DROPDOWN
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.nav-dropdown {
		position: relative;
	}

	.nav-dropdown-menu {
		position: absolute;
		top: 100%;
		left: 50%;
		transform: translateX(-50%);
		margin-top: 8px;
		min-width: 200px;
		background: #05142b;
		border: 1px solid rgba(148, 163, 253, 0.2);
		border-radius: 12px;
		padding: 8px;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.4);
		z-index: 100;
	}

	.nav-dropdown-item {
		display: block;
		padding: 10px 14px;
		color: #94a3b8;
		font-size: 0.9rem;
		text-decoration: none;
		border-radius: 8px;
		transition: background-color 0.15s, color 0.15s;
	}

	.nav-dropdown-item:hover,
	.nav-dropdown-item.active {
		background: rgba(250, 204, 21, 0.1);
		color: #facc15;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * ACTIONS - Fixed size, never shrinks
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.nav-actions {
		flex: 0 0 auto; /* FIXED: No grow, no shrink */
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.nav-cart {
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

	.nav-cart:hover {
		background: rgba(139, 92, 246, 0.2);
	}

	.nav-cart-badge {
		position: absolute;
		top: -4px;
		right: -4px;
		min-width: 18px;
		height: 18px;
		padding: 0 5px;
		background: #facc15;
		color: #05142b;
		font-size: 0.7rem;
		font-weight: 700;
		line-height: 18px;
		text-align: center;
		border-radius: 999px;
	}

	.nav-login-btn {
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

	.nav-login-btn:hover {
		box-shadow: 0 0 20px rgba(37, 99, 235, 0.5);
	}

	.nav-login-btn.full-width {
		width: 100%;
		justify-content: center;
		padding: 14px 20px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * USER MENU
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.nav-user {
		position: relative;
	}

	.nav-user-btn {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 8px 14px;
		background: rgba(250, 204, 21, 0.1);
		border: 1px solid rgba(250, 204, 21, 0.3);
		color: #facc15;
		font-size: 0.9rem;
		font-family: inherit;
		border-radius: 8px;
		cursor: pointer;
		transition: background-color 0.15s, border-color 0.15s;
	}

	.nav-user-btn:hover {
		background: rgba(250, 204, 21, 0.15);
		border-color: rgba(250, 204, 21, 0.5);
	}

	.nav-user-name {
		max-width: 100px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.nav-user-menu {
		position: absolute;
		top: 100%;
		right: 0;
		margin-top: 8px;
		width: 200px;
		background: #05142b;
		border: 1px solid rgba(148, 163, 253, 0.2);
		border-radius: 12px;
		padding: 8px;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.4);
		z-index: 100;
	}

	.nav-user-item {
		display: flex;
		align-items: center;
		gap: 10px;
		width: 100%;
		padding: 10px 12px;
		color: #e2e8f0;
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

	.nav-user-item:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.nav-user-item.danger {
		color: #f87171;
	}

	.nav-user-item.danger:hover {
		background: rgba(248, 113, 113, 0.1);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * HAMBURGER
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.nav-hamburger {
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

	.nav-hamburger:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * MOBILE MENU
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.mobile-backdrop {
		position: fixed;
		inset: 0;
		z-index: 1001;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
	}

	.mobile-panel {
		position: fixed;
		top: 0;
		right: 0;
		bottom: 0;
		z-index: 1002;
		width: min(85vw, 360px);
		background: #05142b;
		border-left: 1px solid rgba(148, 163, 253, 0.15);
		box-shadow: -10px 0 40px rgba(0, 0, 0, 0.5);
		display: flex;
		flex-direction: column;
		animation: slideIn 0.25s ease-out;
	}

	@keyframes slideIn {
		from { transform: translateX(100%); }
		to { transform: translateX(0); }
	}

	.mobile-header {
		display: flex;
		justify-content: flex-end;
		padding: 16px;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.mobile-close {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		color: #e5e7eb;
		background: transparent;
		border: none;
		border-radius: 8px;
		cursor: pointer;
	}

	.mobile-close:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	.mobile-content {
		flex: 1;
		overflow-y: auto;
		padding: 16px;
	}

	.mobile-link {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		padding: 14px 0;
		color: #e5e7eb;
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

	.mobile-link:hover,
	.mobile-link.active {
		color: #facc15;
	}

	.mobile-submenu {
		padding: 8px 0 8px 16px;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.mobile-sublink {
		display: block;
		width: 100%;
		padding: 10px 0;
		color: #94a3b8;
		font-size: 0.95rem;
		font-family: inherit;
		text-decoration: none;
		text-align: left;
		background: transparent;
		border: none;
		cursor: pointer;
	}

	.mobile-sublink:hover,
	.mobile-sublink.active {
		color: #facc15;
	}

	.mobile-sublink.danger {
		color: #f87171;
	}

	.mobile-footer {
		margin-top: 24px;
		padding-top: 24px;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
	}

	.mobile-user-section {
		padding: 16px;
		background: rgba(255, 255, 255, 0.03);
		border-radius: 12px;
	}

	.mobile-user-name {
		margin-bottom: 12px;
		color: #facc15;
		font-weight: 600;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * GLOBAL SCROLL PADDING
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	:global(html) {
		scroll-padding-top: 80px;
	}

	@media (max-width: 768px) {
		:global(html) {
			scroll-padding-top: 64px;
		}
	}
</style>