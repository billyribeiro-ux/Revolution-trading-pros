<script lang="ts">
	/**
	 * NavBar Component - Google L8+ Enterprise Standard
	 * SvelteKit 5 | Absolute Logo & Actions | Zero CLS | CSS-Driven Breakpoints
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

	// Superadmin emails that show admin toolbar
	const SUPERADMIN_EMAILS = ['welberribeirodrums@gmail.com'];
	import { cartItemCount, hasCartItems } from '$lib/stores/cart';
	import { logout as logoutApi } from '$lib/api/auth';

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
		{ href: '/dashboard/memberships', label: 'My Memberships', icon: IconBook },
		{ href: '/dashboard/classes', label: 'My Classes', icon: IconBook },
		{ href: '/dashboard/indicators', label: 'My Indicators', icon: IconChartLine },
		{ href: '/dashboard/account', label: 'My Account', icon: IconSettings },
		{ href: '/support', label: 'Support', icon: IconUser }
	];

	// ─────────────────────────────────────────────────────────────
	// State - Svelte 5 Runes
	// ─────────────────────────────────────────────────────────────
	let isMobileMenuOpen = $state(false);
	let activeDropdown = $state<string | null>(null);
	let activeMobileSubmenu = $state<string | null>(null);
	let isUserMenuOpen = $state(false);
	let isScrolled = $state(false);

	let hamburgerRef = $state<HTMLButtonElement | null>(null);

	// Check if admin toolbar is visible (same logic as AdminToolbar)
	const showsAdminToolbar = $derived(
		(() => {
			const currentUser = $user;
			if (!currentUser) return false;

			// Check superadmin email
			const isSuperadminEmail = SUPERADMIN_EMAILS.includes(currentUser.email?.toLowerCase() ?? '');
			if (isSuperadminEmail) return true;

			// Check admin flags
			const isAdminFlag = Boolean(currentUser.is_admin);
			const roles = currentUser.roles ?? [];
			const hasAdminRole = roles.some((role) =>
				['admin', 'super-admin', 'administrator'].includes(role.toLowerCase())
			);

			return isAdminFlag || hasAdminRole;
		})()
	);

	// ─────────────────────────────────────────────────────────────
	// Handlers
	// ─────────────────────────────────────────────────────────────
	function toggleMobileMenu(): void {
		isMobileMenuOpen = !isMobileMenuOpen;
		activeMobileSubmenu = null;

		if (typeof document !== 'undefined') {
			document.body.style.overflow = isMobileMenuOpen ? 'hidden' : '';
		}
	}

	function closeMobileMenu(): void {
		isMobileMenuOpen = false;
		activeMobileSubmenu = null;

		if (typeof document !== 'undefined') {
			document.body.style.overflow = '';
		}
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

	// ─────────────────────────────────────────────────────────────
	// Lifecycle (browser-only)
	// ─────────────────────────────────────────────────────────────
	onMount(() => {
		isScrolled = window.scrollY > 20;

		const handleResize = () => {
			// If user rotates device / resizes to desktop, auto-close mobile nav
			if (window.innerWidth >= BREAKPOINT_DESKTOP && isMobileMenuOpen) {
				closeMobileMenu();
			}
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

<header
	class="header"
	class:header--scrolled={isScrolled}
	class:header--with-toolbar={showsAdminToolbar}
>
	<!-- LOGO: absolutely positioned, removed from flow -->
	<a
		href="/"
		class="logo"
		onclick={closeMobileMenu}
		aria-label="Revolution Trading Pros home"
	>
		<img
			src="/revolution-trading-pros.png"
			alt="Revolution Trading Pros"
			width="200"
			height="68"
			class="logo__img"
		/>
	</a>

	<!-- CONTENT: centered nav; logo + actions are absolute -->
	<div class="header__content">
		<!-- DESKTOP NAV -->
		<nav class="nav" aria-label="Primary navigation">
			{#each navItems as item (item.id)}
				{#if item.submenu}
					<div class="nav__item" data-dropdown={item.id}>
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
								style={`transform: rotate(${activeDropdown === item.id ? 180 : 0}deg); transition: transform 0.2s;`}
							/>
						</button>

						{#if activeDropdown === item.id}
							<div class="dropdown" role="menu">
								{#each item.submenu as sub (sub.href)}
									<a
										href={sub.href}
										class="dropdown__link"
										class:dropdown__link--active={$page.url.pathname === sub.href}
										role="menuitem"
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
		</nav>

		<!-- ACTIONS (right side, absolute on desktop) -->
		<div class="actions">
			{#if $hasCartItems}
				<a href="/cart" class="actions__cart" aria-label="Shopping cart">
					<IconShoppingCart size={22} />
					{#if $cartItemCount > 0}
						<span class="actions__badge">{$cartItemCount}</span>
					{/if}
				</a>
			{/if}

			<!-- DESKTOP CTA + AUTH -->
			<div class="actions__desktop">
				{#if !$isAuthenticated}
					<a href="/live-trading-rooms" class="actions__cta">
						GET STARTED
					</a>
				{/if}

				{#if $isAuthenticated}
					<div class="nav__item" data-user-menu>
						<button
							type="button"
							class="nav__link"
							aria-expanded={isUserMenuOpen}
							aria-haspopup="true"
							onclick={(e) => {
								e.stopPropagation();
								isUserMenuOpen = !isUserMenuOpen;
							}}
						>
							<span>Dashboard</span>
							<IconChevronDown
								size={16}
								stroke={2.5}
								style={`transform: rotate(${isUserMenuOpen ? 180 : 0}deg); transition: transform 0.2s;`}
							/>
						</button>

						{#if isUserMenuOpen}
							<div class="dropdown" role="menu">
								{#each userMenuItems as menuItem (menuItem.href)}
									<a
										href={menuItem.href}
										class="dropdown__link"
										class:dropdown__link--active={$page.url.pathname === menuItem.href}
										role="menuitem"
										onclick={() => (isUserMenuOpen = false)}
									>
										{menuItem.label}
									</a>
								{/each}
								<button
									type="button"
									class="dropdown__link dropdown__link--danger"
									role="menuitem"
									onclick={handleLogout}
								>
									Logout
								</button>
							</div>
						{/if}
					</div>
				{:else}
					<a href="/login" class="actions__login">
						<IconUser size={18} />
						<span>Login</span>
					</a>
				{/if}
			</div>

			<!-- MOBILE HAMBURGER -->
			<button
				type="button"
				bind:this={hamburgerRef}
				class="actions__hamburger"
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
		</div>
	</div>
</header>

<!-- MOBILE PANEL -->
{#if isMobileMenuOpen}
	<div class="mobile-overlay" onclick={closeMobileMenu} aria-hidden="true"></div>

	<nav class="mobile" aria-label="Mobile navigation">
		<div class="mobile__header">
			<button type="button" class="mobile__close" onclick={closeMobileMenu} aria-label="Close menu">
				<IconX size={24} />
			</button>
		</div>

		<div class="mobile__body">
			{#each navItems as item (item.id)}
				{#if item.submenu}
					<div class="mobile__group">
						<button
							type="button"
							class="mobile__link"
							aria-expanded={activeMobileSubmenu === item.id}
							onclick={() =>
								(activeMobileSubmenu =
									activeMobileSubmenu === item.id ? null : item.id)}
						>
							<span>{item.label}</span>
							<IconChevronRight
								size={18}
								stroke={2.5}
								style={`transform: rotate(${activeMobileSubmenu === item.id ? 90 : 0}deg); transition: transform 0.2s;`}
							/>
						</button>

						{#if activeMobileSubmenu === item.id}
							<div class="mobile__submenu">
								{#each item.submenu as sub (sub.href)}
									<a
										href={sub.href}
										class="mobile__sublink"
										class:mobile__sublink--active={$page.url.pathname === sub.href}
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
						class="mobile__link"
						class:mobile__link--active={$page.url.pathname === item.href}
						onclick={closeMobileMenu}
					>
						{item.label}
					</a>
				{/if}
			{/each}

			<!-- Mobile CTA + auth -->
			<div class="mobile__footer">
				{#if !$isAuthenticated}
				<a
					href="/live-trading-rooms"
					class="mobile__cta"
					onclick={closeMobileMenu}
				>
					GET STARTED
				</a>
				{/if}

				{#if $isAuthenticated}
					<div class="mobile__user">
						<div class="mobile__user-name mobile__user-name--dashboard">Dashboard</div>
						{#each userMenuItems as menuItem (menuItem.href)}
							<a href={menuItem.href} class="mobile__sublink" onclick={closeMobileMenu}>
								{menuItem.label}
							</a>
						{/each}
						<button
							type="button"
							class="mobile__sublink mobile__sublink--danger"
							onclick={handleLogout}
						>
							Logout
						</button>
					</div>
				{:else}
					<a
						href="/login"
						class="actions__login actions__login--full"
						onclick={closeMobileMenu}
					>
						Login
					</a>
				{/if}
			</div>
		</div>
	</nav>
{/if}

<style>
	/* HEADER */
	.header {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		height: 120px;
		background: rgba(5, 20, 43, 0.95);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		border-bottom: 1px solid rgba(148, 163, 253, 0.15);
		transition: background-color 0.2s, box-shadow 0.2s;
		z-index: 1000;
		overflow: visible; /* Allow dropdowns to extend beyond header */
	}

	.header--scrolled {
		background: rgba(5, 20, 43, 0.99);
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
	}

	.header--with-toolbar {
		top: 46px;
	}

	@media (max-width: 768px) {
		.header {
			height: 104px;
		}
	}

	/* LOGO - absolute, zero movement */
	.logo {
		position: absolute;
		left: 24px;
		top: 50%;
		transform: translateY(-50%);
		z-index: 10;
		display: flex;
		align-items: center;
		justify-content: flex-start;
		width: 220px;
		height: 68px;
		text-decoration: none;
	}

	.logo__img {
		display: block;
		width: 200px;
		height: 68px;
		object-fit: contain;
		object-position: left center;
		transform: none !important;
		transition: none !important;
	}

	@media (max-width: 768px) {
		.logo {
			left: 16px;
			width: 170px;
			height: 48px;
		}

		.logo__img {
			width: 140px;
			height: 48px;
		}
	}

	/* CONTENT: nav centered; padding reserves space for logo + actions */
	.header__content {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		max-width: 1440px;
		margin: 0 auto;
		padding: 0 260px; /* roughly: left logo area + right actions area */
	}

	@media (max-width: 1024px) {
		.header__content {
			justify-content: flex-end;
			padding: 0 16px;
			padding-left: 170px; /* room for logo only */
		}
	}

	/* NAV (desktop only via CSS) */
	.nav {
		flex: 0 1 auto;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 2px;
		min-width: 0;
		overflow: visible; /* Allow dropdowns to overflow */
	}

	@media (max-width: 1024px) {
		.nav {
			display: none;
		}
	}

	.nav__item {
		position: relative;
	}

	.nav__link {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		padding: 8px 12px;
		color: #e5e7eb;
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
		color: #facc15;
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(250, 204, 21, 0.2);
	}

	.nav__link:focus-visible {
		outline: 2px solid #facc15;
		outline-offset: 2px;
	}

	/* DROPDOWN */
	.dropdown {
		position: absolute;
		top: calc(100% + 4px);
		left: 50%;
		transform: translateX(-50%);
		min-width: 220px;
		background: rgba(5, 20, 43, 0.98);
		border: 1px solid rgba(148, 163, 253, 0.25);
		border-radius: 12px;
		padding: 8px;
		box-shadow: 0 12px 48px rgba(0, 0, 0, 0.5);
		z-index: 9999;
		backdrop-filter: blur(12px);
	}

	.dropdown__link {
		display: block;
		padding: 10px 14px;
		color: #94a3b8;
		font-size: 0.9rem;
		text-decoration: none;
		border-radius: 8px;
		transition: background-color 0.15s, color 0.15s;
	}

	.dropdown__link:hover,
	.dropdown__link--active {
		background: rgba(250, 204, 21, 0.1);
		color: #facc15;
	}

	.dropdown__link--danger {
		color: #f87171;
		width: 100%;
		text-align: left;
		background: transparent;
		border: none;
		font-family: inherit;
		cursor: pointer;
	}

	.dropdown__link--danger:hover {
		background: rgba(248, 113, 113, 0.1);
		color: #f87171;
	}

	/* ACTIONS (right side block) */
	.actions {
		position: absolute;
		right: 24px;
		top: 50%;
		transform: translateY(-50%);
		display: flex;
		align-items: center;
		gap: 12px;
		flex-shrink: 0;
	}

	@media (max-width: 1024px) {
		.actions {
			position: static;
			right: auto;
			top: auto;
			transform: none;
		}
	}

	.actions__cart {
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

	.actions__cart:hover {
		background: rgba(139, 92, 246, 0.2);
	}

	.actions__badge {
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

	/* Desktop CTA + auth grouping */
	.actions__desktop {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	@media (max-width: 1024px) {
		.actions__desktop {
			display: none;
		}
	}

	.actions__cta {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 10px 22px;
		border-radius: 999px;
		background: linear-gradient(135deg, #facc15, #f97316);
		color: #05142b;
		font-weight: 700;
		font-size: 0.88rem;
		text-decoration: none;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		box-shadow: 0 0 18px rgba(250, 204, 21, 0.35);
		transition: box-shadow 0.2s, transform 0.15s;
	}

	.actions__cta:hover {
		box-shadow: 0 0 22px rgba(250, 204, 21, 0.55);
		transform: translateY(-1px);
	}

	.actions__login {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 10px 18px;
		background: linear-gradient(135deg, #2563eb, #1d4ed8);
		color: white;
		font-weight: 600;
		font-size: 0.9rem;
		text-decoration: none;
		border-radius: 10px;
		transition: box-shadow 0.2s;
	}

	.actions__login:hover {
		box-shadow: 0 0 20px rgba(37, 99, 235, 0.5);
	}

	.actions__login--full {
		width: 100%;
		justify-content: center;
		padding: 14px 20px;
	}

	/* Hamburger: mobile only */
	.actions__hamburger {
		display: none;
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

	.actions__hamburger:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	@media (max-width: 1024px) {
		.actions__hamburger {
			display: flex;
		}
	}

	/* MOBILE OVERLAY + PANEL */
	.mobile-overlay {
		position: fixed;
		inset: 0;
		z-index: 1001;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
	}

	.mobile {
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
	}

	.mobile__header {
		display: flex;
		justify-content: flex-end;
		padding: 16px;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.mobile__close {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		color: #e5e7eb;
		background: transparent;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		transition: background-color 0.15s;
	}

	.mobile__close:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	.mobile__body {
		flex: 1;
		overflow-y: auto;
		padding: 16px;
		-webkit-overflow-scrolling: touch;
	}

	.mobile__group {
		margin-bottom: 4px;
	}

	.mobile__link {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		padding: 14px 0;
		color: #e5e7eb;
		font-size: 1rem;
		font-weight: 600;
		font-family: inherit;
		text-decoration: none;
		background: transparent;
		border: none;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		cursor: pointer;
		text-align: left;
		transition: color 0.15s;
	}

	.mobile__link:hover,
	.mobile__link--active {
		color: #facc15;
	}

	.mobile__submenu {
		padding-left: 16px;
		padding-bottom: 8px;
	}

	.mobile__sublink {
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
		transition: color 0.15s;
	}

	.mobile__sublink:hover,
	.mobile__sublink--active {
		color: #facc15;
	}

	.mobile__sublink--danger {
		color: #f87171;
	}

	.mobile__footer {
		margin-top: 24px;
		padding-top: 24px;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
	}

	.mobile__cta {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		padding: 14px 20px;
		margin-bottom: 16px;
		border-radius: 999px;
		background: linear-gradient(135deg, #facc15, #f97316);
		color: #05142b;
		font-weight: 700;
		font-size: 0.9rem;
		text-decoration: none;
		letter-spacing: 0.06em;
		text-transform: uppercase;
	}

	.mobile__user {
		padding: 16px;
		background: rgba(255, 255, 255, 0.03);
		border-radius: 12px;
	}

	.mobile__user-name {
		margin-bottom: 12px;
		color: #facc15;
		font-weight: 600;
	}

	:global(html) {
		scroll-padding-top: 80px;
	}

	@media (max-width: 768px) {
		:global(html) {
			scroll-padding-top: 64px;
		}
	}
</style>
