<script lang="ts">
	/**
	 * NavBar - Enterprise-Grade Navigation Component
	 * ═══════════════════════════════════════════════════════════════════════════
	 * ICT7+ Principal Engineer Standards | Apple/Google/Microsoft Grade
	 * 
	 * Features:
	 * - Full Svelte 5 Runes Mode compliance
	 * - WCAG 2.1 AA Accessibility
	 * - Zero CLS (Cumulative Layout Shift)
	 * - Focus trap for mobile menu
	 * - Reduced motion support
	 * - Optimized scroll handling with RAF
	 * - Proper TypeScript typing
	 * ═══════════════════════════════════════════════════════════════════════════
	 */
	import { onMount, tick } from 'svelte';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import {
		IconShoppingCart,
		IconMenu2,
		IconX,
		IconChevronDown
	} from '@tabler/icons-svelte';
	import { authStore, isAuthenticated, user } from '$lib/stores/auth';
	import { cartItemCount, hasCartItems } from '$lib/stores/cart';
	import { logout as logoutApi } from '$lib/api/auth';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════
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

	// ═══════════════════════════════════════════════════════════════════════════
	// NAVIGATION CONFIGURATION
	// ═══════════════════════════════════════════════════════════════════════════
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
	] as const;

	const dashboardItems: SubMenuItem[] = [
		{ href: '/dashboard', label: 'Dashboard Home' },
		{ href: '/dashboard/courses', label: 'My Courses' },
		{ href: '/dashboard/indicators', label: 'My Indicators' },
		{ href: '/dashboard/account', label: 'My Account' }
	] as const;

	// ═══════════════════════════════════════════════════════════════════════════
	// REACTIVE STATE (Svelte 5 Runes)
	// ═══════════════════════════════════════════════════════════════════════════
	let mobileMenuOpen = $state(false);
	let activeDropdown = $state<string | null>(null);
	let mobileExpandedSection = $state<string | null>(null);
	let isScrolled = $state(false);
	let prefersReducedMotion = $state(false);

	// Element references for focus management
	let hamburgerButtonRef = $state<HTMLButtonElement | null>(null);
	let mobileCloseButtonRef = $state<HTMLButtonElement | null>(null);
	let mobilePanelRef = $state<HTMLDivElement | null>(null);
	let previousActiveElement = $state<HTMLElement | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════
	const currentPath = $derived(page.url?.pathname ?? '/');

	// ═══════════════════════════════════════════════════════════════════════════
	// FOCUS TRAP UTILITY
	// ═══════════════════════════════════════════════════════════════════════════
	function getFocusableElements(container: HTMLElement): HTMLElement[] {
		const focusableSelectors = [
			'a[href]:not([disabled]):not([tabindex="-1"])',
			'button:not([disabled]):not([tabindex="-1"])',
			'input:not([disabled]):not([tabindex="-1"])',
			'select:not([disabled]):not([tabindex="-1"])',
			'textarea:not([disabled]):not([tabindex="-1"])',
			'[tabindex]:not([tabindex="-1"]):not([disabled])'
		].join(',');
		
		return Array.from(container.querySelectorAll<HTMLElement>(focusableSelectors));
	}

	function trapFocus(event: KeyboardEvent): void {
		if (!mobilePanelRef || event.key !== 'Tab') return;

		const focusableElements = getFocusableElements(mobilePanelRef);
		if (focusableElements.length === 0) return;

		const firstElement = focusableElements[0];
		const lastElement = focusableElements[focusableElements.length - 1];

		if (event.shiftKey && document.activeElement === firstElement) {
			event.preventDefault();
			lastElement.focus();
		} else if (!event.shiftKey && document.activeElement === lastElement) {
			event.preventDefault();
			firstElement.focus();
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// MOBILE MENU HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════
	async function openMobileMenu(): Promise<void> {
		previousActiveElement = document.activeElement as HTMLElement;
		mobileMenuOpen = true;
		document.body.style.overflow = 'hidden';
		
		// Wait for DOM update, then focus close button
		await tick();
		mobileCloseButtonRef?.focus();
	}

	function closeMobileMenu(): void {
		mobileMenuOpen = false;
		document.body.style.overflow = '';
		mobileExpandedSection = null;
		
		// Restore focus to hamburger button
		previousActiveElement?.focus();
	}

	function toggleMobileMenu(): void {
		if (mobileMenuOpen) {
			closeMobileMenu();
		} else {
			openMobileMenu();
		}
	}

	function toggleMobileSection(id: string): void {
		mobileExpandedSection = mobileExpandedSection === id ? null : id;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// DESKTOP DROPDOWN HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════
	function toggleDropdown(id: string): void {
		activeDropdown = activeDropdown === id ? null : id;
	}

	function closeDropdown(): void {
		activeDropdown = null;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// NAVIGATION HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════
	function handleNavClick(href: string): void {
		closeMobileMenu();
		closeDropdown();
		goto(href);
	}

	async function handleLogout(): Promise<void> {
		closeMobileMenu();
		closeDropdown();
		
		try {
			await logoutApi();
		} catch (error) {
			console.error('[NavBar] Logout error:', error);
		}
		
		authStore.clearAuth();
		goto('/login');
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// GLOBAL EVENT HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════
	function handleClickOutside(event: MouseEvent): void {
		const target = event.target as HTMLElement;
		
		// Close desktop dropdown if clicking outside
		if (activeDropdown && !target.closest('[data-dropdown]')) {
			closeDropdown();
		}
	}

	function handleKeydown(event: KeyboardEvent): void {
		if (event.key === 'Escape') {
			if (mobileMenuOpen) {
				closeMobileMenu();
			} else if (activeDropdown) {
				closeDropdown();
			}
		}
		
		// Focus trap for mobile menu
		if (mobileMenuOpen) {
			trapFocus(event);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// SCROLL HANDLER (RAF Optimized)
	// ═══════════════════════════════════════════════════════════════════════════
	let scrollTicking = false;

	function handleScroll(): void {
		if (!scrollTicking) {
			requestAnimationFrame(() => {
				isScrolled = window.scrollY > 20;
				scrollTicking = false;
			});
			scrollTicking = true;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════
	onMount(() => {
		// Check reduced motion preference
		const motionQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
		prefersReducedMotion = motionQuery.matches;
		
		const handleMotionChange = (e: MediaQueryListEvent) => {
			prefersReducedMotion = e.matches;
		};
		motionQuery.addEventListener('change', handleMotionChange);

		// Event listeners
		document.addEventListener('click', handleClickOutside);
		document.addEventListener('keydown', handleKeydown);
		window.addEventListener('scroll', handleScroll, { passive: true });
		
		// Initial scroll check
		handleScroll();

		return () => {
			motionQuery.removeEventListener('change', handleMotionChange);
			document.removeEventListener('click', handleClickOutside);
			document.removeEventListener('keydown', handleKeydown);
			window.removeEventListener('scroll', handleScroll);
			document.body.style.overflow = '';
		};
	});
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     MAIN NAVBAR HEADER
     ═══════════════════════════════════════════════════════════════════════════ -->
<header 
	class="navbar" 
	class:scrolled={isScrolled}
	class:reduced-motion={prefersReducedMotion}
>
	<div class="navbar-container">
		<!-- ═══════════════════════════════════════════════════════════════════
		     LOGO - Static, Zero CLS with explicit dimensions
		     ═══════════════════════════════════════════════════════════════════ -->
		<a href="/" class="logo" aria-label="Revolution Trading Pros - Home">
			<img
				src="/revolution-trading-pros.png"
				alt=""
				width="200"
				height="50"
				loading="eager"
				fetchpriority="high"
				decoding="sync"
			/>
		</a>

		<!-- ═══════════════════════════════════════════════════════════════════
		     DESKTOP NAVIGATION
		     ═══════════════════════════════════════════════════════════════════ -->
		<nav class="desktop-nav" aria-label="Main navigation">
			{#each navItems as item (item.id)}
				{#if item.submenu}
					<div class="dropdown" data-dropdown={item.id}>
						<button
							class="dropdown-trigger"
							class:active={item.submenu.some(s => currentPath === s.href)}
							class:open={activeDropdown === item.id}
							onclick={() => toggleDropdown(item.id)}
							aria-expanded={activeDropdown === item.id}
							aria-haspopup="menu"
							type="button"
						>
							<span>{item.label}</span>
							<IconChevronDown 
								size={14} 
								class="chevron {activeDropdown === item.id ? 'rotate' : ''}" 
							/>
						</button>
						
						{#if activeDropdown === item.id}
							<ul class="dropdown-menu" role="menu" aria-label="{item.label} submenu">
								{#each item.submenu as sub (sub.href)}
									<li role="none">
										<a
											href={sub.href}
											class="dropdown-item"
											class:active={currentPath === sub.href}
											role="menuitem"
											onclick={() => closeDropdown()}
										>
											{sub.label}
										</a>
									</li>
								{/each}
							</ul>
						{/if}
					</div>
				{:else if item.href}
					<a
						href={item.href}
						class="nav-link"
						class:active={currentPath === item.href}
						aria-current={currentPath === item.href ? 'page' : undefined}
					>
						{item.label}
					</a>
				{/if}
			{/each}

			<!-- Dashboard Dropdown (Authenticated Users) -->
			{#if $isAuthenticated}
				<div class="dropdown" data-dropdown="dashboard">
					<button
						class="dropdown-trigger"
						class:active={currentPath.startsWith('/dashboard')}
						class:open={activeDropdown === 'dashboard'}
						onclick={() => toggleDropdown('dashboard')}
						aria-expanded={activeDropdown === 'dashboard'}
						aria-haspopup="menu"
						type="button"
					>
						<span>Dashboard</span>
						<IconChevronDown 
							size={14} 
							class="chevron {activeDropdown === 'dashboard' ? 'rotate' : ''}" 
						/>
					</button>
					
					{#if activeDropdown === 'dashboard'}
						<ul class="dropdown-menu" role="menu" aria-label="Dashboard submenu">
							{#each dashboardItems as sub (sub.href)}
								<li role="none">
									<a
										href={sub.href}
										class="dropdown-item"
										class:active={currentPath === sub.href}
										role="menuitem"
										onclick={() => closeDropdown()}
									>
										{sub.label}
									</a>
								</li>
							{/each}
						</ul>
					{/if}
				</div>
			{/if}
		</nav>

		<!-- ═══════════════════════════════════════════════════════════════════
		     ACTION BUTTONS
		     ═══════════════════════════════════════════════════════════════════ -->
		<div class="actions">
			<!-- Cart Button -->
			{#if $hasCartItems}
				<a 
					href="/cart" 
					class="cart-btn" 
					aria-label="Shopping cart with {$cartItemCount} {$cartItemCount === 1 ? 'item' : 'items'}"
				>
					<IconShoppingCart size={22} aria-hidden="true" />
					{#if $cartItemCount > 0}
						<span class="cart-badge" aria-hidden="true">{$cartItemCount}</span>
					{/if}
				</a>
			{/if}

			<!-- Auth Buttons (Desktop) -->
			{#if !$isAuthenticated}
				<a href="/login" class="login-btn">Login</a>
				<a href="/get-started" class="cta-btn">Get Started</a>
			{/if}

			<!-- Hamburger Menu Button -->
			<button
				bind:this={hamburgerButtonRef}
				class="hamburger"
				onclick={toggleMobileMenu}
				aria-label={mobileMenuOpen ? 'Close navigation menu' : 'Open navigation menu'}
				aria-expanded={mobileMenuOpen}
				aria-controls="mobile-nav-panel"
				type="button"
			>
				<IconMenu2 size={24} aria-hidden="true" />
			</button>
		</div>
	</div>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     MOBILE NAVIGATION OVERLAY
     ═══════════════════════════════════════════════════════════════════════════ -->
{#if mobileMenuOpen}
	<!-- Backdrop -->
	<div 
		class="mobile-backdrop"
		class:reduced-motion={prefersReducedMotion}
		onclick={closeMobileMenu}
		onkeydown={(e) => e.key === 'Enter' && closeMobileMenu()}
		role="button"
		tabindex="-1"
		aria-label="Close navigation menu"
	></div>

	<!-- Panel -->
	<div 
		bind:this={mobilePanelRef}
		id="mobile-nav-panel"
		class="mobile-panel"
		class:reduced-motion={prefersReducedMotion}
		role="dialog" 
		aria-modal="true" 
		aria-label="Navigation menu"
	>
		<!-- Header -->
		<div class="mobile-header">
			<span class="mobile-title" id="mobile-menu-title">Menu</span>
			<button 
				bind:this={mobileCloseButtonRef}
				class="mobile-close" 
				onclick={closeMobileMenu}
				aria-label="Close navigation menu" 
				type="button"
			>
				<IconX size={24} aria-hidden="true" />
			</button>
		</div>

		<!-- User Info (Authenticated) -->
		{#if $isAuthenticated && $user}
			<div class="mobile-user">
				<div class="mobile-avatar" aria-hidden="true">
					{$user.name?.charAt(0).toUpperCase() || 'U'}
				</div>
				<div class="mobile-user-info">
					<span class="mobile-user-name">{$user.name || 'User'}</span>
					<span class="mobile-user-email">{$user.email || ''}</span>
				</div>
			</div>
		{/if}

		<!-- Navigation Items -->
		<nav class="mobile-nav" aria-labelledby="mobile-menu-title">
			{#each navItems as item (item.id)}
				{#if item.submenu}
					<div class="mobile-nav-group">
						<button
							class="mobile-nav-item"
							class:expanded={mobileExpandedSection === item.id}
							onclick={() => toggleMobileSection(item.id)}
							aria-expanded={mobileExpandedSection === item.id}
							aria-controls="mobile-submenu-{item.id}"
							type="button"
						>
							<span>{item.label}</span>
							<IconChevronDown 
								size={18} 
								class="chevron {mobileExpandedSection === item.id ? 'rotate' : ''}"
								aria-hidden="true"
							/>
						</button>
						
						{#if mobileExpandedSection === item.id}
							<ul 
								id="mobile-submenu-{item.id}"
								class="mobile-submenu"
								role="list"
							>
								{#each item.submenu as sub (sub.href)}
									<li>
										<a
											href={sub.href}
											class="mobile-submenu-item"
											class:active={currentPath === sub.href}
											aria-current={currentPath === sub.href ? 'page' : undefined}
											onclick={() => handleNavClick(sub.href)}
										>
											{sub.label}
										</a>
									</li>
								{/each}
							</ul>
						{/if}
					</div>
				{:else if item.href}
					<a
						href={item.href}
						class="mobile-nav-item"
						class:active={currentPath === item.href}
						aria-current={currentPath === item.href ? 'page' : undefined}
						onclick={() => handleNavClick(item.href)}
					>
						{item.label}
					</a>
				{/if}
			{/each}

			<!-- Dashboard Section (Authenticated) -->
			{#if $isAuthenticated}
				<div class="mobile-divider" role="separator"></div>
				
				<div class="mobile-nav-group">
					<button
						class="mobile-nav-item"
						class:expanded={mobileExpandedSection === 'dashboard'}
						onclick={() => toggleMobileSection('dashboard')}
						aria-expanded={mobileExpandedSection === 'dashboard'}
						aria-controls="mobile-submenu-dashboard"
						type="button"
					>
						<span>Dashboard</span>
						<IconChevronDown 
							size={18} 
							class="chevron {mobileExpandedSection === 'dashboard' ? 'rotate' : ''}"
							aria-hidden="true"
						/>
					</button>
					
					{#if mobileExpandedSection === 'dashboard'}
						<ul 
							id="mobile-submenu-dashboard"
							class="mobile-submenu"
							role="list"
						>
							{#each dashboardItems as item (item.href)}
								<li>
									<a
										href={item.href}
										class="mobile-submenu-item"
										class:active={currentPath === item.href}
										aria-current={currentPath === item.href ? 'page' : undefined}
										onclick={() => handleNavClick(item.href)}
									>
										{item.label}
									</a>
								</li>
							{/each}
						</ul>
					{/if}
				</div>
			{/if}
		</nav>

		<!-- Footer Actions -->
		<div class="mobile-footer">
			{#if $hasCartItems}
				<a 
					href="/cart" 
					class="mobile-cart" 
					onclick={() => handleNavClick('/cart')}
					aria-label="Shopping cart with {$cartItemCount} items"
				>
					<IconShoppingCart size={20} aria-hidden="true" />
					<span>Cart</span>
					{#if $cartItemCount > 0}
						<span class="mobile-badge" aria-hidden="true">{$cartItemCount}</span>
					{/if}
				</a>
			{/if}

			{#if $isAuthenticated}
				<button class="mobile-logout" onclick={handleLogout} type="button">
					Logout
				</button>
			{:else}
				<a href="/login" class="mobile-login" onclick={() => handleNavClick('/login')}>
					Login
				</a>
				<a href="/get-started" class="mobile-cta" onclick={() => handleNavClick('/get-started')}>
					Get Started
				</a>
			{/if}
		</div>
	</div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   CSS CUSTOM PROPERTIES (Design Tokens)
	   ═══════════════════════════════════════════════════════════════════════════ */
	:root {
		--nav-height-desktop: 74px;
		--nav-height-tablet: 64px;
		--nav-height-mobile: 60px;
		
		--nav-bg: rgba(10, 22, 40, 0.85);
		--nav-bg-scrolled: rgba(10, 22, 40, 0.95);
		--nav-border: rgba(255, 255, 255, 0.1);
		
		--brand-primary: #0E6AC4;
		--brand-primary-dark: #0a4d8a;
		--brand-primary-light: rgba(14, 106, 196, 0.1);
		--brand-primary-glow: rgba(14, 106, 196, 0.4);
		
		--text-primary: #ffffff;
		--text-secondary: rgba(255, 255, 255, 0.9);
		--text-muted: rgba(255, 255, 255, 0.5);
		
		--transition-fast: 150ms ease;
		--transition-base: 200ms ease;
		--transition-smooth: 300ms cubic-bezier(0.4, 0, 0.2, 1);
		
		--font-family: 'Montserrat', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
		
		--z-navbar: 1000;
		--z-dropdown: 1001;
		--z-mobile-backdrop: 9998;
		--z-mobile-panel: 9999;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   NAVBAR HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */
	.navbar {
		position: sticky;
		/* Use CSS variable from parent for admin toolbar offset, default to 0 */
		top: var(--admin-toolbar-height, 0);
		z-index: var(--z-navbar);
		width: 100%;
		height: var(--nav-height-desktop);
		background-color: var(--nav-bg);
		backdrop-filter: blur(20px) saturate(180%);
		-webkit-backdrop-filter: blur(20px) saturate(180%);
		border-bottom: 1px solid var(--nav-border);
		transition: 
			background-color var(--transition-smooth),
			box-shadow var(--transition-smooth);
		will-change: background-color, box-shadow;
	}

	.navbar.scrolled {
		background-color: var(--nav-bg-scrolled);
		box-shadow: 0 4px 30px rgba(0, 0, 0, 0.4);
	}

	.navbar.reduced-motion,
	.navbar.reduced-motion * {
		transition-duration: 0.01ms !important;
		animation-duration: 0.01ms !important;
	}

	.navbar-container {
		display: flex;
		align-items: center;
		justify-content: space-between;
		height: 100%;
		max-width: 100%;
		margin: 0 auto;
		padding: 0 1.5rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LOGO - Fixed dimensions for Zero CLS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.logo {
		display: flex;
		align-items: center;
		flex-shrink: 0;
		width: 200px;
		height: 50px;
		text-decoration: none;
	}

	.logo img {
		width: 100%;
		height: 100%;
		object-fit: contain;
		object-position: left center;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DESKTOP NAVIGATION
	   ═══════════════════════════════════════════════════════════════════════════ */
	.desktop-nav {
		display: none;
		align-items: center;
		gap: 0.5rem;
		flex: 1;
		justify-content: center;
		padding: 0 2rem;
	}

	@media (min-width: 1024px) {
		.desktop-nav {
			display: flex;
		}
	}

	@media (min-width: 1280px) {
		.desktop-nav {
			gap: 0.75rem;
		}
	}

	/* Nav Link Base Styles */
	.nav-link {
		position: relative;
		display: flex;
		align-items: center;
		padding: 0.625rem 1rem;
		color: var(--text-primary);
		font-size: 0.9375rem;
		font-weight: 700;
		font-family: var(--font-family);
		letter-spacing: 0.02em;
		text-decoration: none;
		white-space: nowrap;
		background: linear-gradient(to bottom right, var(--brand-primary) 0%, transparent 30%);
		background-color: var(--brand-primary-light);
		border: 1px solid rgba(14, 106, 196, 0.2);
		border-radius: 0.5rem;
		cursor: pointer;
		transition: 
			background-color var(--transition-base),
			box-shadow var(--transition-base),
			border-color var(--transition-base),
			transform var(--transition-fast);
	}

	@media (min-width: 1280px) {
		.nav-link {
			padding: 0.75rem 1.25rem;
			font-size: 1rem;
		}
	}

	.nav-link:hover,
	.nav-link:focus-visible {
		background-color: rgba(14, 106, 196, 0.25);
		box-shadow: 0 0 20px var(--brand-primary-glow);
		border-color: rgba(14, 106, 196, 0.5);
		transform: translateY(-2px);
	}

	.nav-link.active {
		background-color: rgba(14, 106, 196, 0.3);
		border-color: rgba(14, 106, 196, 0.6);
	}

	.nav-link:focus-visible {
		outline: 2px solid var(--brand-primary);
		outline-offset: 2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DROPDOWN COMPONENT
	   ═══════════════════════════════════════════════════════════════════════════ */
	.dropdown {
		position: relative;
	}

	.dropdown-trigger {
		position: relative;
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.625rem 1rem;
		color: var(--text-primary);
		font-size: 0.9375rem;
		font-weight: 700;
		font-family: var(--font-family);
		letter-spacing: 0.02em;
		white-space: nowrap;
		background: linear-gradient(to bottom right, var(--brand-primary) 0%, transparent 30%);
		background-color: var(--brand-primary-light);
		border: 1px solid rgba(14, 106, 196, 0.2);
		border-radius: 0.5rem;
		cursor: pointer;
		transition: 
			background-color var(--transition-base),
			box-shadow var(--transition-base),
			border-color var(--transition-base),
			transform var(--transition-fast);
	}

	@media (min-width: 1280px) {
		.dropdown-trigger {
			gap: 0.5rem;
			padding: 0.75rem 1.25rem;
			font-size: 1rem;
		}
	}

	.dropdown-trigger:hover,
	.dropdown-trigger:focus-visible,
	.dropdown-trigger.active,
	.dropdown-trigger.open {
		background-color: rgba(14, 106, 196, 0.25);
		box-shadow: 0 0 20px var(--brand-primary-glow);
		border-color: rgba(14, 106, 196, 0.5);
		transform: translateY(-2px);
	}

	.dropdown-trigger:focus-visible {
		outline: 2px solid var(--brand-primary);
		outline-offset: 2px;
	}

	/* Chevron rotation */
	.dropdown-trigger :global(.chevron) {
		transition: transform var(--transition-base);
	}

	.dropdown-trigger :global(.chevron.rotate) {
		transform: rotate(180deg);
	}

	/* Dropdown Menu */
	.dropdown-menu {
		position: absolute;
		top: calc(100% + 0.5rem);
		left: 0;
		min-width: 13rem;
		margin: 0;
		padding: 0.25rem 0;
		list-style: none;
		background: linear-gradient(to bottom, rgba(20, 34, 62, 0.98), rgba(10, 22, 40, 0.98));
		border: 1px solid rgba(14, 106, 196, 0.2);
		border-radius: 0.75rem;
		box-shadow: 
			0 10px 40px rgba(0, 0, 0, 0.5),
			0 0 20px rgba(14, 106, 196, 0.1);
		backdrop-filter: blur(10px);
		-webkit-backdrop-filter: blur(10px);
		overflow: hidden;
		z-index: var(--z-dropdown);
		animation: dropdownFadeIn var(--transition-smooth) forwards;
	}

	@keyframes dropdownFadeIn {
		from {
			opacity: 0;
			transform: translateY(-8px) scale(0.96);
		}
		to {
			opacity: 1;
			transform: translateY(0) scale(1);
		}
	}

	.dropdown-item {
		display: block;
		position: relative;
		padding: 0.75rem 1rem;
		color: var(--text-secondary);
		font-size: 0.9375rem;
		font-weight: 600;
		font-family: var(--font-family);
		text-decoration: none;
		transition: 
			background-color var(--transition-fast),
			color var(--transition-fast),
			padding-left var(--transition-fast);
	}

	.dropdown-item::before {
		content: '';
		position: absolute;
		left: 0;
		top: 0;
		height: 100%;
		width: 3px;
		background: linear-gradient(to bottom, var(--brand-primary), var(--brand-primary-dark));
		transform: scaleY(0);
		transition: transform var(--transition-fast);
	}

	.dropdown-item:hover,
	.dropdown-item:focus-visible {
		background: linear-gradient(to right, rgba(14, 106, 196, 0.15), rgba(14, 106, 196, 0.05));
		color: var(--text-primary);
		padding-left: 1.25rem;
	}

	.dropdown-item:hover::before,
	.dropdown-item:focus-visible::before {
		transform: scaleY(1);
	}

	.dropdown-item.active {
		background: linear-gradient(to right, rgba(14, 106, 196, 0.2), rgba(14, 106, 196, 0.05));
		color: var(--text-primary);
	}

	.dropdown-item:focus-visible {
		outline: 2px solid var(--brand-primary);
		outline-offset: -2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACTION BUTTONS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		flex-shrink: 0;
	}

	@media (min-width: 1024px) {
		.actions {
			gap: 1rem;
		}
	}

	/* Cart Button */
	.cart-btn {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		color: var(--brand-primary);
		background: var(--brand-primary-light);
		border: 1px solid rgba(14, 106, 196, 0.2);
		border-radius: 0.5rem;
		text-decoration: none;
		transition: 
			background-color var(--transition-base),
			box-shadow var(--transition-base),
			transform var(--transition-fast);
	}

	.cart-btn:hover,
	.cart-btn:focus-visible {
		background: rgba(14, 106, 196, 0.2);
		box-shadow: 0 0 15px rgba(14, 106, 196, 0.3);
		transform: translateY(-2px);
	}

	.cart-btn:focus-visible {
		outline: 2px solid var(--brand-primary);
		outline-offset: 2px;
	}

	.cart-badge {
		position: absolute;
		top: -4px;
		right: -4px;
		min-width: 20px;
		height: 20px;
		padding: 0 6px;
		background: var(--brand-primary);
		color: var(--text-primary);
		font-size: 0.6875rem;
		font-weight: 700;
		line-height: 20px;
		text-align: center;
		border-radius: 10px;
		pointer-events: none;
	}

	/* Login Button */
	.login-btn {
		display: none;
		align-items: center;
		height: 44px;
		padding: 0 1.5rem;
		color: var(--text-primary);
		font-size: 0.875rem;
		font-weight: 700;
		font-family: var(--font-family);
		letter-spacing: 0.02em;
		text-decoration: none;
		background: linear-gradient(to bottom right, var(--brand-primary) 0%, transparent 30%);
		background-color: var(--brand-primary-light);
		border: 1px solid rgba(14, 106, 196, 0.2);
		border-radius: 0.5rem;
		transition: 
			background-color var(--transition-base),
			box-shadow var(--transition-base),
			border-color var(--transition-base),
			transform var(--transition-fast);
	}

	@media (min-width: 1024px) {
		.login-btn {
			display: flex;
		}
	}

	.login-btn:hover,
	.login-btn:focus-visible {
		background-color: rgba(14, 106, 196, 0.25);
		box-shadow: 0 0 20px var(--brand-primary-glow);
		border-color: rgba(14, 106, 196, 0.5);
		transform: translateY(-2px);
	}

	.login-btn:focus-visible {
		outline: 2px solid var(--brand-primary);
		outline-offset: 2px;
	}

	/* CTA Button */
	.cta-btn {
		display: none;
		align-items: center;
		height: 44px;
		padding: 0 1.5rem;
		color: var(--text-primary);
		font-size: 0.875rem;
		font-weight: 800;
		font-family: var(--font-family);
		letter-spacing: 0.02em;
		text-decoration: none;
		background: linear-gradient(135deg, var(--brand-primary) 0%, var(--brand-primary-dark) 100%);
		border: none;
		border-radius: 0.5rem;
		box-shadow: 0 4px 15px rgba(14, 106, 196, 0.3);
		transition: 
			box-shadow var(--transition-base),
			transform var(--transition-fast);
	}

	@media (min-width: 1024px) {
		.cta-btn {
			display: flex;
		}
	}

	.cta-btn:hover,
	.cta-btn:focus-visible {
		box-shadow: 0 6px 25px rgba(14, 106, 196, 0.5);
		transform: translateY(-2px);
	}

	.cta-btn:focus-visible {
		outline: 2px solid var(--text-primary);
		outline-offset: 2px;
	}

	/* Hamburger Button */
	.hamburger {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		color: var(--text-primary);
		background: transparent;
		border: none;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: background-color var(--transition-fast);
	}

	.hamburger:hover,
	.hamburger:focus-visible {
		background-color: rgba(107, 114, 128, 0.3);
	}

	.hamburger:focus-visible {
		outline: 2px solid var(--brand-primary);
		outline-offset: 2px;
	}

	@media (min-width: 1024px) {
		.hamburger {
			display: none;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MOBILE MENU OVERLAY
	   ═══════════════════════════════════════════════════════════════════════════ */
	.mobile-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
		z-index: var(--z-mobile-backdrop);
		animation: backdropFadeIn var(--transition-base) forwards;
	}

	.mobile-backdrop.reduced-motion {
		animation: none;
		opacity: 1;
	}

	@keyframes backdropFadeIn {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	.mobile-panel {
		position: fixed;
		top: 0;
		right: 0;
		width: min(85vw, 360px);
		height: 100%;
		height: 100dvh;
		display: flex;
		flex-direction: column;
		background: #0a1628;
		border-left: 1px solid rgba(107, 114, 128, 0.3);
		z-index: var(--z-mobile-panel);
		animation: panelSlideIn var(--transition-smooth) forwards;
		overflow: hidden;
	}

	.mobile-panel.reduced-motion {
		animation: none;
		transform: translateX(0);
	}

	@keyframes panelSlideIn {
		from { transform: translateX(100%); }
		to { transform: translateX(0); }
	}

	/* Mobile Header */
	.mobile-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		height: var(--nav-height-mobile);
		min-height: var(--nav-height-mobile);
		padding: 0 1rem;
		border-bottom: 1px solid rgba(107, 114, 128, 0.3);
		flex-shrink: 0;
	}

	.mobile-title {
		font-size: 1.125rem;
		font-weight: 700;
		font-family: var(--font-family);
		color: var(--text-primary);
	}

	.mobile-close {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		color: var(--text-primary);
		background: transparent;
		border: none;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: background-color var(--transition-fast);
	}

	.mobile-close:hover,
	.mobile-close:focus-visible {
		background-color: rgba(107, 114, 128, 0.3);
	}

	.mobile-close:focus-visible {
		outline: 2px solid var(--brand-primary);
		outline-offset: 2px;
	}

	/* Mobile User Info */
	.mobile-user {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		background: rgba(20, 34, 62, 0.5);
		flex-shrink: 0;
	}

	.mobile-avatar {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		background: linear-gradient(135deg, var(--brand-primary), var(--brand-primary-dark));
		color: var(--text-primary);
		font-size: 1.125rem;
		font-weight: 700;
		border-radius: 50%;
		flex-shrink: 0;
	}

	.mobile-user-info {
		display: flex;
		flex-direction: column;
		min-width: 0;
	}

	.mobile-user-name {
		font-size: 0.9375rem;
		font-weight: 600;
		font-family: var(--font-family);
		color: var(--text-primary);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.mobile-user-email {
		font-size: 0.8125rem;
		color: var(--text-muted);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	/* Mobile Navigation */
	.mobile-nav {
		flex: 1;
		padding: 0.75rem;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		overflow-y: auto;
		-webkit-overflow-scrolling: touch;
		overscroll-behavior: contain;
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
		padding: 0.75rem 1rem;
		color: var(--text-secondary);
		font-size: 1rem;
		font-weight: 700;
		font-family: var(--font-family);
		letter-spacing: 0.02em;
		text-decoration: none;
		background: transparent;
		border: none;
		border-radius: 0.5rem;
		cursor: pointer;
		text-align: left;
		transition: background-color var(--transition-fast);
	}

	.mobile-nav-item:hover,
	.mobile-nav-item:focus-visible {
		background: rgba(255, 255, 255, 0.08);
	}

	.mobile-nav-item:focus-visible {
		outline: 2px solid var(--brand-primary);
		outline-offset: -2px;
	}

	.mobile-nav-item.active {
		color: var(--brand-primary);
	}

	.mobile-nav-item :global(.chevron) {
		transition: transform var(--transition-fast);
		flex-shrink: 0;
	}

	.mobile-nav-item :global(.chevron.rotate) {
		transform: rotate(180deg);
	}

	/* Mobile Submenu */
	.mobile-submenu {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		margin: 0.25rem 0 0.5rem 0;
		padding: 0 0 0 0.75rem;
		list-style: none;
	}

	.mobile-submenu-item {
		display: block;
		padding: 0.625rem 0.875rem;
		color: rgba(255, 255, 255, 0.8);
		font-size: 0.9375rem;
		font-weight: 600;
		font-family: var(--font-family);
		text-decoration: none;
		background: rgba(20, 34, 62, 0.4);
		border-left: 2px solid rgba(14, 106, 196, 0.3);
		border-radius: 0.375rem;
		transition: 
			background-color var(--transition-fast),
			color var(--transition-fast),
			border-color var(--transition-fast);
	}

	.mobile-submenu-item:hover,
	.mobile-submenu-item:focus-visible {
		background: rgba(14, 106, 196, 0.15);
		color: var(--text-primary);
		border-left-color: var(--brand-primary);
	}

	.mobile-submenu-item:focus-visible {
		outline: 2px solid var(--brand-primary);
		outline-offset: -2px;
	}

	.mobile-submenu-item.active {
		color: var(--brand-primary);
		border-left-color: var(--brand-primary);
	}

	.mobile-divider {
		height: 1px;
		margin: 0.75rem 1.25rem;
		background: rgba(107, 114, 128, 0.3);
	}

	/* Mobile Footer */
	.mobile-footer {
		display: flex;
		flex-direction: column;
		gap: 0.625rem;
		padding: 1rem 1.25rem;
		padding-bottom: max(1rem, env(safe-area-inset-bottom));
		border-top: 1px solid rgba(107, 114, 128, 0.3);
		flex-shrink: 0;
	}

	.mobile-cart {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		height: 48px;
		color: var(--brand-primary);
		font-size: 0.9375rem;
		font-weight: 600;
		font-family: var(--font-family);
		text-decoration: none;
		background: var(--brand-primary-light);
		border: 1px solid rgba(14, 106, 196, 0.2);
		border-radius: 0.5rem;
		transition: background-color var(--transition-fast);
	}

	.mobile-cart:hover,
	.mobile-cart:focus-visible {
		background: rgba(14, 106, 196, 0.2);
	}

	.mobile-cart:focus-visible {
		outline: 2px solid var(--brand-primary);
		outline-offset: 2px;
	}

	.mobile-badge {
		min-width: 22px;
		height: 22px;
		padding: 0 6px;
		background: var(--brand-primary);
		color: var(--text-primary);
		font-size: 0.75rem;
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
		color: var(--text-primary);
		font-size: 0.9375rem;
		font-weight: 700;
		font-family: var(--font-family);
		letter-spacing: 0.02em;
		text-decoration: none;
		background: linear-gradient(to bottom right, var(--brand-primary) 0%, transparent 30%);
		background-color: var(--brand-primary-light);
		border: 1px solid rgba(14, 106, 196, 0.2);
		border-radius: 0.5rem;
		transition: background-color var(--transition-fast);
	}

	.mobile-login:hover,
	.mobile-login:focus-visible {
		background-color: rgba(14, 106, 196, 0.25);
	}

	.mobile-login:focus-visible {
		outline: 2px solid var(--brand-primary);
		outline-offset: 2px;
	}

	.mobile-cta {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 48px;
		color: var(--text-primary);
		font-size: 0.9375rem;
		font-weight: 800;
		font-family: var(--font-family);
		letter-spacing: 0.02em;
		text-decoration: none;
		background: linear-gradient(135deg, var(--brand-primary) 0%, var(--brand-primary-dark) 100%);
		border-radius: 0.5rem;
		transition: 
			transform var(--transition-fast),
			box-shadow var(--transition-fast);
	}

	.mobile-cta:hover,
	.mobile-cta:focus-visible {
		transform: translateY(-1px);
		box-shadow: 0 4px 15px rgba(14, 106, 196, 0.4);
	}

	.mobile-cta:focus-visible {
		outline: 2px solid var(--text-primary);
		outline-offset: 2px;
	}

	.mobile-logout {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 48px;
		color: #f87171;
		font-size: 0.9375rem;
		font-weight: 700;
		font-family: var(--font-family);
		background: rgba(248, 113, 113, 0.1);
		border: none;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: background-color var(--transition-fast);
	}

	.mobile-logout:hover,
	.mobile-logout:focus-visible {
		background: rgba(248, 113, 113, 0.2);
	}

	.mobile-logout:focus-visible {
		outline: 2px solid #f87171;
		outline-offset: 2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS
	   ═══════════════════════════════════════════════════════════════════════════ */
	
	/* Tablet (< 1024px) */
	@media (max-width: 1023px) {
		.navbar {
			height: var(--nav-height-tablet);
		}

		.navbar-container {
			padding: 0 1rem;
		}

		.logo {
			width: 180px;
			height: 45px;
		}
	}

	/* Mobile (< 768px) */
	@media (max-width: 767px) {
		.navbar {
			height: var(--nav-height-mobile);
		}

		.navbar-container {
			padding: 0 1rem;
		}

		.logo {
			width: 150px;
			height: 38px;
		}

		.cart-btn,
		.hamburger {
			width: 40px;
			height: 40px;
		}

		.cta-btn {
			display: none;
		}
	}

	/* Small Mobile (< 375px) */
	@media (max-width: 374px) {
		.logo {
			width: 130px;
			height: 33px;
		}

		.actions {
			gap: 0.5rem;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   REDUCED MOTION PREFERENCE
	   ═══════════════════════════════════════════════════════════════════════════ */
	@media (prefers-reduced-motion: reduce) {
		.navbar,
		.navbar *,
		.mobile-backdrop,
		.mobile-panel,
		.mobile-panel * {
			animation-duration: 0.01ms !important;
			animation-iteration-count: 1 !important;
			transition-duration: 0.01ms !important;
		}
	}
</style>