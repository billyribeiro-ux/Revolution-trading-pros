<script lang="ts">
	/**
	 * NavBar Component - Google L8+ Principal Engineer Production Standard
	 * ══════════════════════════════════════════════════════════════════════════════
	 * SvelteKit 5 | CSS Grid Layout | Zero CLS | Static Logo | WCAG 2.1 AA
	 *
	 * Architecture Principles:
	 * 1. Logo NEVER moves - absolute positioning with containment
	 * 2. Hover-triggered dropdowns with 150ms intent delay (Google pattern)
	 * 3. Focus trap for mobile panel
	 * 4. Full keyboard navigation (Tab, Arrow keys, Escape)
	 * 5. Edge-aware dropdown positioning
	 * ══════════════════════════════════════════════════════════════════════════════
	 */
	import { onMount, tick } from 'svelte';
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
	// Constants - Google Design System Values
	// ═══════════════════════════════════════════════════════════════════════════════
	const BREAKPOINT_DESKTOP = 1024;
	const HOVER_INTENT_DELAY = 150; // Google's standard hover intent delay
	const DROPDOWN_ANIMATION_MS = 200;

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
	let hoverIntentTimer = $state<ReturnType<typeof setTimeout> | null>(null);
	let dropdownPositions = $state<Record<string, 'center' | 'left' | 'right'>>({});

	// DOM refs
	let mobileNavRef = $state<HTMLElement | null>(null);
	let hamburgerRef = $state<HTMLButtonElement | null>(null);
	let headerRef = $state<HTMLElement | null>(null);
	let firstMobileFocusable = $state<HTMLElement | null>(null);
	let lastMobileFocusable = $state<HTMLElement | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════════
	// Derived
	// ═══════════════════════════════════════════════════════════════════════════════
	const isDesktop = $derived(windowWidth >= BREAKPOINT_DESKTOP);

	// ═══════════════════════════════════════════════════════════════════════════════
	// Dropdown Position Detection (Edge-Aware)
	// ═══════════════════════════════════════════════════════════════════════════════
	function calculateDropdownPosition(element: HTMLElement): 'center' | 'left' | 'right' {
		const rect = element.getBoundingClientRect();
		const centerX = rect.left + rect.width / 2;
		const viewportWidth = window.innerWidth;
		const dropdownWidth = 220; // Approximate dropdown width

		// Check if dropdown would overflow right edge
		if (centerX + dropdownWidth / 2 > viewportWidth - 20) {
			return 'right';
		}
		// Check if dropdown would overflow left edge
		if (centerX - dropdownWidth / 2 < 20) {
			return 'left';
		}
		return 'center';
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// Handlers - Google-Style Hover Intent
	// ═══════════════════════════════════════════════════════════════════════════════
	function handleDropdownEnter(itemId: string, event: MouseEvent): void {
		// Clear any existing timer
		if (hoverIntentTimer) {
			clearTimeout(hoverIntentTimer);
		}

		// Calculate position before showing
		const target = event.currentTarget as HTMLElement;
		dropdownPositions[itemId] = calculateDropdownPosition(target);

		// Google-style hover intent delay
		hoverIntentTimer = setTimeout(() => {
			activeDropdown = itemId;
		}, HOVER_INTENT_DELAY);
	}

	function handleDropdownLeave(): void {
		if (hoverIntentTimer) {
			clearTimeout(hoverIntentTimer);
			hoverIntentTimer = null;
		}
		// Small delay before closing to allow moving to dropdown
		setTimeout(() => {
			// Only close if mouse isn't over the dropdown area
			activeDropdown = null;
		}, 100);
	}

	function handleDropdownClick(itemId: string, event: MouseEvent): void {
		event.stopPropagation();
		const target = event.currentTarget as HTMLElement;
		dropdownPositions[itemId] = calculateDropdownPosition(target);
		activeDropdown = activeDropdown === itemId ? null : itemId;
	}

	function toggleMobileMenu(): void {
		isMobileMenuOpen = !isMobileMenuOpen;
		activeMobileSubmenu = null;
		document.body.style.overflow = isMobileMenuOpen ? 'hidden' : '';

		if (isMobileMenuOpen) {
			// Focus first focusable element after menu opens
			tick().then(() => {
				firstMobileFocusable?.focus();
			});
		}
	}

	function closeMobileMenu(): void {
		isMobileMenuOpen = false;
		activeMobileSubmenu = null;
		document.body.style.overflow = '';
		hamburgerRef?.focus();
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
			} else {
				activeDropdown = null;
				isUserMenuOpen = false;
			}
		}
	}

	// Focus trap for mobile menu (WCAG 2.1 requirement)
	function handleMobileMenuKeydown(event: KeyboardEvent): void {
		if (event.key === 'Tab' && isMobileMenuOpen) {
			const focusableElements = mobileNavRef?.querySelectorAll<HTMLElement>(
				'button, a, input, select, textarea, [tabindex]:not([tabindex="-1"])'
			);

			if (!focusableElements || focusableElements.length === 0) return;

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
	}

	// Keyboard navigation for dropdowns (Google standard)
	function handleDropdownKeydown(event: KeyboardEvent, items: SubMenuItem[]): void {
		const currentIndex = items.findIndex(
			item => document.activeElement?.getAttribute('href') === item.href
		);

		switch (event.key) {
			case 'ArrowDown':
				event.preventDefault();
				const nextIndex = currentIndex < items.length - 1 ? currentIndex + 1 : 0;
				(document.querySelector(`[href="${items[nextIndex].href}"]`) as HTMLElement)?.focus();
				break;
			case 'ArrowUp':
				event.preventDefault();
				const prevIndex = currentIndex > 0 ? currentIndex - 1 : items.length - 1;
				(document.querySelector(`[href="${items[prevIndex].href}"]`) as HTMLElement)?.focus();
				break;
			case 'Home':
				event.preventDefault();
				(document.querySelector(`[href="${items[0].href}"]`) as HTMLElement)?.focus();
				break;
			case 'End':
				event.preventDefault();
				(document.querySelector(`[href="${items[items.length - 1].href}"]`) as HTMLElement)?.focus();
				break;
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
			if (hoverIntentTimer) clearTimeout(hoverIntentTimer);
		};
	});
</script>

<header
	class="header"
	class:header--scrolled={isScrolled}
	bind:this={headerRef}
>
	<div class="header__container">
		<!-- ═══════════════════════════════════════════════════════════════════════
		     LOGO ZONE - ABSOLUTE LOCK (Google Principal Engineer Standard)
		     This section is engineered to NEVER move under ANY circumstances.
		     Uses: contain, flex-shrink:0, min-width, explicit dimensions
		═══════════════════════════════════════════════════════════════════════ -->
		<div class="header__logo">
			<a href="/" class="logo" onclick={closeMobileMenu} aria-label="Revolution Trading Pros - Home">
				<img
					src="/revolution-trading-pros.png"
					alt="Revolution Trading Pros"
					width="200"
					height="68"
					class="logo__image"
					fetchpriority="high"
					decoding="async"
				/>
			</a>
		</div>

		<!-- ═══════════════════════════════════════════════════════════════════════
		     NAVIGATION ZONE - Desktop Only
		     Hover-triggered dropdowns with 150ms intent delay (Google pattern)
		═══════════════════════════════════════════════════════════════════════ -->
		{#if isDesktop}
			<nav class="header__nav" aria-label="Main navigation">
				<ul class="nav" role="menubar">
					{#each navItems as item (item.id)}
						{#if item.submenu}
							<li
								class="nav__item"
								data-dropdown={item.id}
								role="none"
								onmouseenter={(e) => handleDropdownEnter(item.id, e)}
								onmouseleave={handleDropdownLeave}
							>
								<button
									type="button"
									class="nav__link nav__link--dropdown"
									class:nav__link--open={activeDropdown === item.id}
									aria-expanded={activeDropdown === item.id}
									aria-haspopup="menu"
									aria-controls={`dropdown-${item.id}`}
									onclick={(e) => handleDropdownClick(item.id, e)}
								>
									<span>{item.label}</span>
									<IconChevronDown
										size={14}
										stroke={2.5}
										class="nav__chevron"
										aria-hidden="true"
									/>
								</button>

								<div
									id={`dropdown-${item.id}`}
									class="dropdown"
									class:dropdown--visible={activeDropdown === item.id}
									class:dropdown--left={dropdownPositions[item.id] === 'left'}
									class:dropdown--right={dropdownPositions[item.id] === 'right'}
									role="menu"
									aria-labelledby={`nav-${item.id}`}
									onkeydown={(e) => handleDropdownKeydown(e, item.submenu || [])}
								>
									{#each item.submenu as sub, index (sub.href)}
										<a
											href={sub.href}
											class="dropdown__item"
											class:dropdown__item--active={$page.url.pathname === sub.href}
											role="menuitem"
											tabindex={activeDropdown === item.id ? 0 : -1}
											style="--item-index: {index}"
											onclick={() => (activeDropdown = null)}
										>
											{sub.label}
										</a>
									{/each}
								</div>
							</li>
						{:else}
							<li class="nav__item" role="none">
								<a
									href={item.href}
									class="nav__link"
									class:nav__link--active={$page.url.pathname === item.href}
									role="menuitem"
								>
									{item.label}
								</a>
							</li>
						{/if}
					{/each}
				</ul>
			</nav>
		{/if}

		<!-- ═══════════════════════════════════════════════════════════════════════
		     ACTIONS ZONE - Cart, User Menu, Hamburger
		═══════════════════════════════════════════════════════════════════════ -->
		<div class="header__actions">
			{#if $hasCartItems}
				<a href="/cart" class="action-btn action-btn--cart" aria-label="Shopping cart ({$cartItemCount} items)">
					<IconShoppingCart size={22} aria-hidden="true" />
					{#if $cartItemCount > 0}
						<span class="action-btn__badge" aria-hidden="true">{$cartItemCount}</span>
					{/if}
				</a>
			{/if}

			{#if isDesktop}
				<!-- Get Started CTA Button -->
				<a href="/login" class="cta-btn">
					Get Started
				</a>

				{#if $isAuthenticated}
					<div class="user-menu" data-user-menu>
						<button
							type="button"
							class="user-menu__trigger"
							aria-expanded={isUserMenuOpen}
							aria-haspopup="menu"
							aria-controls="user-dropdown"
							onclick={(e) => {
								e.stopPropagation();
								isUserMenuOpen = !isUserMenuOpen;
							}}
						>
							<IconUser size={18} aria-hidden="true" />
							<span class="user-menu__name">{$user?.name || 'Account'}</span>
							<IconChevronDown size={12} aria-hidden="true" />
						</button>

						<div
							id="user-dropdown"
							class="user-menu__dropdown"
							class:user-menu__dropdown--visible={isUserMenuOpen}
							role="menu"
							aria-label="User menu"
						>
							{#each userMenuItems as menuItem (menuItem.href)}
								{@const Icon = menuItem.icon}
								<a
									href={menuItem.href}
									class="user-menu__item"
									role="menuitem"
									tabindex={isUserMenuOpen ? 0 : -1}
									onclick={() => (isUserMenuOpen = false)}
								>
									<Icon size={16} aria-hidden="true" />
									<span>{menuItem.label}</span>
								</a>
							{/each}
							<button
								type="button"
								class="user-menu__item user-menu__item--danger"
								role="menuitem"
								tabindex={isUserMenuOpen ? 0 : -1}
								onclick={handleLogout}
							>
								<IconLogout size={16} aria-hidden="true" />
								<span>Logout</span>
							</button>
						</div>
					</div>
				{:else}
					<a href="/login" class="login-btn">
						<IconUser size={18} aria-hidden="true" />
						<span>Login</span>
					</a>
				{/if}
			{/if}

			{#if !isDesktop}
				<button
					type="button"
					bind:this={hamburgerRef}
					class="hamburger"
					aria-label={isMobileMenuOpen ? 'Close navigation menu' : 'Open navigation menu'}
					aria-expanded={isMobileMenuOpen}
					aria-controls="mobile-nav"
					onclick={toggleMobileMenu}
				>
					<span class="hamburger__icon" class:hamburger__icon--open={isMobileMenuOpen}>
						{#if isMobileMenuOpen}
							<IconX size={26} aria-hidden="true" />
						{:else}
							<IconMenu2 size={26} aria-hidden="true" />
						{/if}
					</span>
				</button>
			{/if}
		</div>
	</div>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════════
     MOBILE NAVIGATION PANEL
     Features: Focus trap, slide animation, sequential item reveal
═══════════════════════════════════════════════════════════════════════════════ -->
{#if !isDesktop && isMobileMenuOpen}
	<!-- Backdrop -->
	<div
		class="mobile-overlay"
		onclick={closeMobileMenu}
		onkeydown={(e) => e.key === 'Enter' && closeMobileMenu()}
		role="button"
		tabindex="-1"
		aria-label="Close menu"
	></div>

	<!-- Panel -->
	<nav
		id="mobile-nav"
		class="mobile-panel"
		bind:this={mobileNavRef}
		aria-label="Mobile navigation"
		onkeydown={handleMobileMenuKeydown}
	>
		<div class="mobile-panel__header">
			<button
				type="button"
				class="mobile-panel__close"
				onclick={closeMobileMenu}
				aria-label="Close menu"
				bind:this={firstMobileFocusable}
			>
				<IconX size={24} aria-hidden="true" />
			</button>
		</div>

		<div class="mobile-panel__body">
			{#each navItems as item, index (item.id)}
				{#if item.submenu}
					<div class="mobile-nav__group" style="--item-index: {index}">
						<button
							type="button"
							class="mobile-nav__link"
							aria-expanded={activeMobileSubmenu === item.id}
							aria-controls={`mobile-submenu-${item.id}`}
							onclick={() => (activeMobileSubmenu = activeMobileSubmenu === item.id ? null : item.id)}
						>
							<span>{item.label}</span>
							<span
								class="mobile-nav__chevron"
								class:mobile-nav__chevron--open={activeMobileSubmenu === item.id}
								aria-hidden="true"
							>
								<IconChevronRight size={18} stroke={2.5} />
							</span>
						</button>

						<div
							id={`mobile-submenu-${item.id}`}
							class="mobile-nav__submenu"
							class:mobile-nav__submenu--open={activeMobileSubmenu === item.id}
						>
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
					</div>
				{:else}
					<a
						href={item.href}
						class="mobile-nav__link"
						class:mobile-nav__link--active={$page.url.pathname === item.href}
						style="--item-index: {index}"
						onclick={closeMobileMenu}
					>
						{item.label}
					</a>
				{/if}
			{/each}

			<div class="mobile-panel__footer" style="--item-index: {navItems.length}">
				{#if $isAuthenticated}
					<div class="mobile-user">
						<div class="mobile-user__name">{$user?.name}</div>
						{#each userMenuItems as menuItem (menuItem.href)}
							<a href={menuItem.href} class="mobile-nav__sublink" onclick={closeMobileMenu}>
								{menuItem.label}
							</a>
						{/each}
						<button
							type="button"
							class="mobile-nav__sublink mobile-nav__sublink--danger"
							onclick={handleLogout}
							bind:this={lastMobileFocusable}
						>
							Logout
						</button>
					</div>
				{:else}
					<a
						href="/login"
						class="login-btn login-btn--full"
						onclick={closeMobileMenu}
						bind:this={lastMobileFocusable}
					>
						Login
					</a>
				{/if}
			</div>
		</div>
	</nav>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════════
	 * CSS CUSTOM PROPERTIES - Google Design System Tokens
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	:root {
		/* Layout Tokens - NEVER CHANGE THESE */
		--header-height: 80px;
		--header-height-mobile: 64px;
		--logo-width: 200px;
		--logo-width-mobile: 140px;
		--logo-height: 68px;
		--logo-height-mobile: 48px;

		/* Color Tokens */
		--nav-bg: #05142b;
		--nav-bg-alpha: rgba(5, 20, 43, 0.97);
		--nav-text: #e5e7eb;
		--nav-text-muted: #94a3b8;
		--nav-accent: #facc15;
		--nav-danger: #f87171;
		--nav-border: rgba(148, 163, 253, 0.15);
		--nav-hover-bg: rgba(255, 255, 255, 0.05);

		/* Animation Tokens - Google timing functions */
		--ease-out-expo: cubic-bezier(0.16, 1, 0.3, 1);
		--ease-in-out: cubic-bezier(0.4, 0, 0.2, 1);
		--duration-fast: 150ms;
		--duration-normal: 200ms;
		--duration-slow: 300ms;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * HEADER - Sticky, GPU-accelerated
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.header {
		position: sticky;
		top: 0;
		left: 0;
		right: 0;
		z-index: 1000;
		height: var(--header-height);
		background: var(--nav-bg-alpha);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		border-bottom: 1px solid var(--nav-border);
		transition: background-color var(--duration-normal) var(--ease-in-out),
					box-shadow var(--duration-normal) var(--ease-in-out);
		/* GPU acceleration */
		transform: translateZ(0);
		will-change: background-color, box-shadow;
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
	 * CONTAINER - CSS GRID (Google L8 Standard)
	 * 3-column layout: [logo: FIXED] [nav: FLEXIBLE] [actions: FIXED]
	 * Logo and actions have fixed widths to prevent any movement
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.header__container {
		display: grid;
		grid-template-columns: var(--logo-width) 1fr minmax(200px, auto);
		align-items: center;
		height: 100%;
		max-width: 1440px;
		margin: 0 auto;
		padding: 0 24px;
		gap: 24px;
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
	 * LOGO ZONE - BULLETPROOF POSITIONING (Zero CLS Guarantee)
	 *
	 * Engineering guarantees:
	 * 1. contain: layout style - isolates from parent layout changes
	 * 2. flex-shrink: 0 - NEVER compresses
	 * 3. min-width/max-width - explicit bounds
	 * 4. Explicit width/height on img - prevents reflow
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.header__logo {
		/* Lock dimensions - IMMUTABLE */
		width: var(--logo-width);
		min-width: var(--logo-width);
		max-width: var(--logo-width);
		height: var(--logo-height);
		min-height: var(--logo-height);
		max-height: var(--logo-height);

		/* Prevent ANY shrinking or growth */
		flex-shrink: 0;
		flex-grow: 0;

		/* Layout containment - isolate from parent */
		contain: layout style;

		/* Clip any overflow */
		overflow: hidden;
	}

	.logo {
		display: block;
		width: var(--logo-width);
		height: var(--logo-height);
		text-decoration: none;
	}

	.logo__image {
		display: block;
		width: var(--logo-width);
		height: var(--logo-height);
		min-width: var(--logo-width);
		min-height: var(--logo-height);
		object-fit: contain;
		object-position: left center;
		/* Prevent text selection glow */
		user-select: none;
		-webkit-user-drag: none;
	}

	@media (max-width: 1024px) {
		.header__logo {
			width: var(--logo-width-mobile);
			min-width: var(--logo-width-mobile);
			max-width: var(--logo-width-mobile);
			height: var(--logo-height-mobile);
			min-height: var(--logo-height-mobile);
			max-height: var(--logo-height-mobile);
		}

		.logo {
			width: var(--logo-width-mobile);
			height: var(--logo-height-mobile);
		}

		.logo__image {
			width: var(--logo-width-mobile);
			height: var(--logo-height-mobile);
			min-width: var(--logo-width-mobile);
			min-height: var(--logo-height-mobile);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * NAVIGATION - Centered flex, overflow handling
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.header__nav {
		min-width: 0; /* Allow grid shrinking */
		overflow: hidden;
	}

	.nav {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 2px;
		margin: 0;
		padding: 0;
		list-style: none;
	}

	.nav__item {
		position: relative;
	}

	.nav__link {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		padding: 10px 14px;
		color: var(--nav-text);
		font-family: 'Montserrat', system-ui, sans-serif;
		font-weight: 600;
		font-size: 0.875rem;
		text-decoration: none;
		white-space: nowrap;
		background: transparent;
		border: 1px solid transparent;
		border-radius: 6px;
		cursor: pointer;
		transition: color var(--duration-fast) var(--ease-in-out),
					background-color var(--duration-fast) var(--ease-in-out),
					border-color var(--duration-fast) var(--ease-in-out);
	}

	.nav__link:hover,
	.nav__link:focus-visible,
	.nav__link--active,
	.nav__link--open {
		color: var(--nav-accent);
		background: var(--nav-hover-bg);
		border-color: rgba(250, 204, 21, 0.2);
	}

	.nav__link:focus-visible {
		outline: 2px solid var(--nav-accent);
		outline-offset: 2px;
	}

	/* Chevron rotation */
	.nav__link :global(.nav__chevron) {
		transition: transform var(--duration-fast) var(--ease-in-out);
	}

	.nav__link--open :global(.nav__chevron) {
		transform: rotate(180deg);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * DROPDOWN - Animated, edge-aware positioning
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.dropdown {
		position: absolute;
		top: calc(100% + 8px);
		left: 50%;
		transform: translateX(-50%) translateY(-8px);
		min-width: 220px;
		padding: 8px;
		background: var(--nav-bg);
		border: 1px solid var(--nav-border);
		border-radius: 12px;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.4),
					0 0 0 1px rgba(255, 255, 255, 0.05);
		z-index: 100;

		/* Animation - start hidden */
		opacity: 0;
		visibility: hidden;
		pointer-events: none;
		transition: opacity var(--duration-normal) var(--ease-out-expo),
					transform var(--duration-normal) var(--ease-out-expo),
					visibility var(--duration-normal);
	}

	.dropdown--visible {
		opacity: 1;
		visibility: visible;
		pointer-events: auto;
		transform: translateX(-50%) translateY(0);
	}

	/* Edge detection positioning */
	.dropdown--left {
		left: 0;
		transform: translateX(0) translateY(-8px);
	}

	.dropdown--left.dropdown--visible {
		transform: translateX(0) translateY(0);
	}

	.dropdown--right {
		left: auto;
		right: 0;
		transform: translateX(0) translateY(-8px);
	}

	.dropdown--right.dropdown--visible {
		transform: translateX(0) translateY(0);
	}

	.dropdown__item {
		display: block;
		padding: 12px 16px;
		color: var(--nav-text-muted);
		font-size: 0.9rem;
		text-decoration: none;
		border-radius: 8px;
		transition: background-color var(--duration-fast) var(--ease-in-out),
					color var(--duration-fast) var(--ease-in-out);
		/* Stagger animation */
		animation: dropdownItemIn var(--duration-normal) var(--ease-out-expo) forwards;
		animation-delay: calc(var(--item-index, 0) * 30ms);
		opacity: 0;
		transform: translateY(-4px);
	}

	.dropdown--visible .dropdown__item {
		opacity: 1;
		transform: translateY(0);
	}

	@keyframes dropdownItemIn {
		from {
			opacity: 0;
			transform: translateY(-4px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.dropdown__item:hover,
	.dropdown__item:focus-visible,
	.dropdown__item--active {
		background: rgba(250, 204, 21, 0.1);
		color: var(--nav-accent);
	}

	.dropdown__item:focus-visible {
		outline: 2px solid var(--nav-accent);
		outline-offset: -2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * ACTIONS - Right column utilities (FIXED WIDTH to prevent logo movement)
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.header__actions {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		gap: 12px;
		min-width: 200px; /* Prevent shrinking that causes logo movement */
		flex-shrink: 0;
	}

	/* Action Button (Cart) */
	.action-btn {
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
		transition: background-color var(--duration-fast) var(--ease-in-out),
					transform var(--duration-fast) var(--ease-in-out);
	}

	.action-btn:hover {
		background: rgba(139, 92, 246, 0.2);
		transform: scale(1.05);
	}

	.action-btn:active {
		transform: scale(0.98);
	}

	.action-btn__badge {
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
		/* Pop animation on update */
		animation: badgePop 200ms var(--ease-out-expo);
	}

	@keyframes badgePop {
		0% { transform: scale(0.5); }
		50% { transform: scale(1.2); }
		100% { transform: scale(1); }
	}

	/* CTA Button (Yellow Get Started) */
	.cta-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 12px 28px;
		background: linear-gradient(135deg, #facc15, #eab308);
		color: #0a101c;
		font-weight: 700;
		font-size: 0.95rem;
		text-decoration: none;
		border-radius: 12px;
		box-shadow: 0 4px 12px rgba(250, 204, 21, 0.3);
		transition: all var(--duration-normal) var(--ease-in-out);
		letter-spacing: 0.02em;
	}

	.cta-btn:hover {
		background: linear-gradient(135deg, #fde047, #facc15);
		box-shadow: 0 6px 20px rgba(250, 204, 21, 0.5);
		transform: translateY(-2px);
	}

	.cta-btn:active {
		transform: translateY(0);
		box-shadow: 0 2px 8px rgba(250, 204, 21, 0.4);
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
		transition: box-shadow var(--duration-normal) var(--ease-in-out),
					transform var(--duration-fast) var(--ease-in-out);
	}

	.login-btn:hover {
		box-shadow: 0 0 24px rgba(37, 99, 235, 0.5);
		transform: translateY(-1px);
	}

	.login-btn:active {
		transform: translateY(0);
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
		color: var(--nav-accent);
		font-size: 0.9rem;
		font-family: inherit;
		border-radius: 8px;
		cursor: pointer;
		transition: background-color var(--duration-fast) var(--ease-in-out),
					border-color var(--duration-fast) var(--ease-in-out);
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
		top: calc(100% + 8px);
		right: 0;
		width: 200px;
		padding: 8px;
		background: var(--nav-bg);
		border: 1px solid var(--nav-border);
		border-radius: 12px;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.4);
		z-index: 100;

		/* Animation */
		opacity: 0;
		visibility: hidden;
		transform: translateY(-8px);
		transition: opacity var(--duration-normal) var(--ease-out-expo),
					transform var(--duration-normal) var(--ease-out-expo),
					visibility var(--duration-normal);
	}

	.user-menu__dropdown--visible {
		opacity: 1;
		visibility: visible;
		transform: translateY(0);
	}

	.user-menu__item {
		display: flex;
		align-items: center;
		gap: 10px;
		width: 100%;
		padding: 10px 12px;
		color: var(--nav-text);
		font-size: 0.9rem;
		font-family: inherit;
		text-decoration: none;
		text-align: left;
		background: transparent;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		transition: background-color var(--duration-fast) var(--ease-in-out);
	}

	.user-menu__item:hover {
		background: var(--nav-hover-bg);
	}

	.user-menu__item--danger {
		color: var(--nav-danger);
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
		transition: background-color var(--duration-fast) var(--ease-in-out);
	}

	.hamburger:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	.hamburger__icon {
		display: flex;
		align-items: center;
		justify-content: center;
		transition: transform var(--duration-normal) var(--ease-out-expo);
	}

	.hamburger__icon--open {
		transform: rotate(90deg);
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
		-webkit-backdrop-filter: blur(4px);
		animation: fadeIn var(--duration-normal) var(--ease-out-expo);
	}

	@keyframes fadeIn {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * MOBILE PANEL - Slide-in with staggered content
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	.mobile-panel {
		position: fixed;
		top: 0;
		right: 0;
		bottom: 0;
		z-index: 1002;
		width: min(85vw, 360px);
		background: var(--nav-bg);
		border-left: 1px solid var(--nav-border);
		box-shadow: -10px 0 40px rgba(0, 0, 0, 0.5);
		display: flex;
		flex-direction: column;
		animation: slideInRight var(--duration-slow) var(--ease-out-expo);
	}

	@keyframes slideInRight {
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
		width: 44px;
		height: 44px;
		color: var(--nav-text);
		background: transparent;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		transition: background-color var(--duration-fast) var(--ease-in-out);
	}

	.mobile-panel__close:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	.mobile-panel__body {
		flex: 1;
		overflow-y: auto;
		padding: 16px;
		/* Smooth iOS scrolling */
		-webkit-overflow-scrolling: touch;
	}

	.mobile-panel__footer {
		margin-top: 24px;
		padding-top: 24px;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
		/* Stagger animation */
		animation: mobileItemIn var(--duration-normal) var(--ease-out-expo) forwards;
		animation-delay: calc(var(--item-index, 0) * 50ms);
		opacity: 0;
	}

	@keyframes mobileItemIn {
		from {
			opacity: 0;
			transform: translateX(20px);
		}
		to {
			opacity: 1;
			transform: translateX(0);
		}
	}

	/* Mobile Nav Links */
	.mobile-nav__group {
		/* Stagger animation */
		animation: mobileItemIn var(--duration-normal) var(--ease-out-expo) forwards;
		animation-delay: calc(var(--item-index, 0) * 50ms);
		opacity: 0;
	}

	.mobile-nav__link {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		padding: 16px 0;
		color: var(--nav-text);
		font-size: 1.05rem;
		font-weight: 600;
		font-family: inherit;
		text-decoration: none;
		background: transparent;
		border: none;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		cursor: pointer;
		text-align: left;
		transition: color var(--duration-fast) var(--ease-in-out);
		/* Stagger animation */
		animation: mobileItemIn var(--duration-normal) var(--ease-out-expo) forwards;
		animation-delay: calc(var(--item-index, 0) * 50ms);
		opacity: 0;
	}

	.mobile-nav__link:hover,
	.mobile-nav__link--active {
		color: var(--nav-accent);
	}

	/* Chevron rotation */
	.mobile-nav__link :global(.mobile-nav__chevron) {
		transition: transform var(--duration-fast) var(--ease-in-out);
	}

	.mobile-nav__link :global(.mobile-nav__chevron--open) {
		transform: rotate(90deg);
	}

	.mobile-nav__submenu {
		max-height: 0;
		overflow: hidden;
		padding-left: 16px;
		transition: max-height var(--duration-normal) var(--ease-in-out),
					padding var(--duration-normal) var(--ease-in-out);
	}

	.mobile-nav__submenu--open {
		max-height: 500px;
		padding-top: 8px;
		padding-bottom: 8px;
	}

	.mobile-nav__sublink {
		display: block;
		width: 100%;
		padding: 12px 0;
		color: var(--nav-text-muted);
		font-size: 0.95rem;
		font-family: inherit;
		text-decoration: none;
		text-align: left;
		background: transparent;
		border: none;
		cursor: pointer;
		transition: color var(--duration-fast) var(--ease-in-out);
	}

	.mobile-nav__sublink:hover,
	.mobile-nav__sublink--active {
		color: var(--nav-accent);
	}

	.mobile-nav__sublink--danger {
		color: var(--nav-danger);
	}

	/* Mobile User */
	.mobile-user {
		padding: 16px;
		background: rgba(255, 255, 255, 0.03);
		border-radius: 12px;
	}

	.mobile-user__name {
		margin-bottom: 12px;
		color: var(--nav-accent);
		font-weight: 600;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * GLOBAL STYLES
	 * ═══════════════════════════════════════════════════════════════════════════════ */
	:global(html) {
		scroll-padding-top: var(--header-height);
	}

	@media (max-width: 768px) {
		:global(html) {
			scroll-padding-top: var(--header-height-mobile);
		}
	}

	/* Prevent body scroll when mobile menu is open */
	:global(body.menu-open) {
		overflow: hidden;
		position: fixed;
		width: 100%;
	}
</style>
