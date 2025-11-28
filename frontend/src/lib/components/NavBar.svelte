<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * NavBar Component - Revolution Trading Pros
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * 
	 * Enterprise-Grade Navigation | Netflix/Apple/Google Standard
	 * SvelteKit 5 | Svelte 5 Runes | Zero CLS | WCAG 2.1 AA
	 * 
	 * @version 2.0.0
	 * @license Proprietary
	 * ═══════════════════════════════════════════════════════════════════════════════
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
	// Configuration Constants
	// ═══════════════════════════════════════════════════════════════════════════════
	const BREAKPOINT_DESKTOP = 1024;
	const BREAKPOINT_TABLET = 768;
	const HOVER_INTENT_DELAY = 150;
	const DROPDOWN_CLOSE_DELAY = 120;

	// ═══════════════════════════════════════════════════════════════════════════════
	// Navigation Configuration (Config-Driven Architecture)
	// ═══════════════════════════════════════════════════════════════════════════════
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
	// Reactive State (Svelte 5 Runes)
	// ═══════════════════════════════════════════════════════════════════════════════
	let isMobileMenuOpen = $state(false);
	let activeDropdown = $state<string | null>(null);
	let activeMobileSubmenu = $state<string | null>(null);
	let isUserMenuOpen = $state(false);
	let windowWidth = $state(BREAKPOINT_DESKTOP + 1);
	let isScrolled = $state(false);
	let dropdownPositions = $state<Record<string, 'center' | 'left' | 'right'>>({});

	// Hover tracking for diagonal movement protection
	let isInDropdownZone = $state(false);
	let pendingDropdown = $state<string | null>(null);

	// Timers (non-reactive - no need to track these)
	let hoverIntentTimer: ReturnType<typeof setTimeout> | null = null;
	let dropdownCloseTimer: ReturnType<typeof setTimeout> | null = null;

	// DOM References
	let mobileNavRef = $state<HTMLElement | null>(null);
	let hamburgerRef = $state<HTMLButtonElement | null>(null);
	let headerRef = $state<HTMLElement | null>(null);
	let dropdownRefs = $state<Record<string, HTMLElement | null>>({});

	// ═══════════════════════════════════════════════════════════════════════════════
	// Derived State
	// ═══════════════════════════════════════════════════════════════════════════════
	const isDesktop = $derived(windowWidth >= BREAKPOINT_DESKTOP);
	const currentPath = $derived($page.url.pathname);

	// Check if a nav item or its children match the current route
	function isActiveRoute(item: NavItem): boolean {
		if (item.href && currentPath === item.href) return true;
		if (item.submenu) {
			return item.submenu.some(sub => currentPath === sub.href || currentPath.startsWith(sub.href + '/'));
		}
		return false;
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// Dropdown Position Detection (Edge-Aware)
	// ═══════════════════════════════════════════════════════════════════════════════
	function calculateDropdownPosition(element: HTMLElement): 'center' | 'left' | 'right' {
		const rect = element.getBoundingClientRect();
		const centerX = rect.left + rect.width / 2;
		const viewportWidth = window.innerWidth;
		const dropdownWidth = 240;
		const edgePadding = 24;

		if (centerX + dropdownWidth / 2 > viewportWidth - edgePadding) {
			return 'right';
		}
		if (centerX - dropdownWidth / 2 < edgePadding) {
			return 'left';
		}
		return 'center';
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// Desktop Dropdown Handlers (With Diagonal Hover Protection)
	// ═══════════════════════════════════════════════════════════════════════════════
	function handleTriggerEnter(itemId: string, event: MouseEvent): void {
		// Clear any pending close
		if (dropdownCloseTimer) {
			clearTimeout(dropdownCloseTimer);
			dropdownCloseTimer = null;
		}

		// Clear any pending open for different dropdown
		if (hoverIntentTimer && pendingDropdown !== itemId) {
			clearTimeout(hoverIntentTimer);
			hoverIntentTimer = null;
		}

		const target = event.currentTarget as HTMLElement;
		dropdownPositions = { ...dropdownPositions, [itemId]: calculateDropdownPosition(target) };
		pendingDropdown = itemId;
		isInDropdownZone = true;

		// Hover intent delay
		hoverIntentTimer = setTimeout(() => {
			activeDropdown = itemId;
			pendingDropdown = null;
		}, HOVER_INTENT_DELAY);
	}

	function handleTriggerLeave(): void {
		// Clear hover intent if still pending
		if (hoverIntentTimer) {
			clearTimeout(hoverIntentTimer);
			hoverIntentTimer = null;
		}
		pendingDropdown = null;

		// Delay close to allow diagonal movement to dropdown
		dropdownCloseTimer = setTimeout(() => {
			if (!isInDropdownZone) {
				activeDropdown = null;
			}
		}, DROPDOWN_CLOSE_DELAY);
	}

	function handleDropdownEnter(): void {
		// User made it to dropdown - cancel any pending close
		if (dropdownCloseTimer) {
			clearTimeout(dropdownCloseTimer);
			dropdownCloseTimer = null;
		}
		isInDropdownZone = true;
	}

	function handleDropdownLeave(): void {
		isInDropdownZone = false;

		// Small delay before closing
		dropdownCloseTimer = setTimeout(() => {
			if (!isInDropdownZone) {
				activeDropdown = null;
			}
		}, DROPDOWN_CLOSE_DELAY);
	}

	function handleDropdownClick(itemId: string, event: MouseEvent): void {
		event.stopPropagation();
		const target = event.currentTarget as HTMLElement;
		dropdownPositions = { ...dropdownPositions, [itemId]: calculateDropdownPosition(target) };
		activeDropdown = activeDropdown === itemId ? null : itemId;
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// Mobile Menu Handlers
	// ═══════════════════════════════════════════════════════════════════════════════
	function toggleMobileMenu(): void {
		isMobileMenuOpen = !isMobileMenuOpen;
		activeMobileSubmenu = null;

		if (isMobileMenuOpen) {
			document.body.style.overflow = 'hidden';
			document.body.classList.add('nav-menu-open');
			tick().then(() => {
				const closeBtn = mobileNavRef?.querySelector<HTMLElement>('.mobile-panel__close');
				closeBtn?.focus();
			});
		} else {
			document.body.style.overflow = '';
			document.body.classList.remove('nav-menu-open');
		}
	}

	function closeMobileMenu(): void {
		isMobileMenuOpen = false;
		activeMobileSubmenu = null;
		document.body.style.overflow = '';
		document.body.classList.remove('nav-menu-open');
		hamburgerRef?.focus();
	}

	function toggleMobileSubmenu(itemId: string): void {
		activeMobileSubmenu = activeMobileSubmenu === itemId ? null : itemId;
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// Global Event Handlers
	// ═══════════════════════════════════════════════════════════════════════════════
	function handleDocumentClick(event: MouseEvent): void {
		const target = event.target as HTMLElement;

		// Close desktop dropdowns if clicking outside
		if (activeDropdown && !target.closest('[data-dropdown]')) {
			activeDropdown = null;
		}

		// Close user menu if clicking outside
		if (isUserMenuOpen && !target.closest('[data-user-menu]')) {
			isUserMenuOpen = false;
		}
	}

	function handleKeydown(event: KeyboardEvent): void {
		if (event.key === 'Escape') {
			if (isMobileMenuOpen) {
				closeMobileMenu();
			} else if (activeDropdown) {
				activeDropdown = null;
			} else if (isUserMenuOpen) {
				isUserMenuOpen = false;
			}
		}
	}

	// Focus trap for mobile menu
	function handleMobileMenuKeydown(event: KeyboardEvent): void {
		if (event.key !== 'Tab' || !isMobileMenuOpen || !mobileNavRef) return;

		const focusableSelector = 'button:not([disabled]), a[href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])';
		const focusableElements = Array.from(mobileNavRef.querySelectorAll<HTMLElement>(focusableSelector));

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

	// Keyboard navigation within dropdowns
	function handleDropdownKeydown(event: KeyboardEvent, items: SubMenuItem[], dropdownId: string): void {
		const dropdownEl = dropdownRefs[dropdownId];
		if (!dropdownEl) return;

		const links = Array.from(dropdownEl.querySelectorAll<HTMLAnchorElement>('.dropdown__item'));
		const currentIndex = links.findIndex(link => link === document.activeElement);

		switch (event.key) {
			case 'ArrowDown':
				event.preventDefault();
				const nextIndex = currentIndex < links.length - 1 ? currentIndex + 1 : 0;
				links[nextIndex]?.focus();
				break;

			case 'ArrowUp':
				event.preventDefault();
				const prevIndex = currentIndex > 0 ? currentIndex - 1 : links.length - 1;
				links[prevIndex]?.focus();
				break;

			case 'Home':
				event.preventDefault();
				links[0]?.focus();
				break;

			case 'End':
				event.preventDefault();
				links[links.length - 1]?.focus();
				break;

			case 'Tab':
				// Let default tab behavior work, but close dropdown after
				setTimeout(() => {
					if (!dropdownEl.contains(document.activeElement)) {
						activeDropdown = null;
					}
				}, 0);
				break;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// Auth Handlers
	// ═══════════════════════════════════════════════════════════════════════════════
	async function handleLogout(): Promise<void> {
		closeMobileMenu();
		isUserMenuOpen = false;

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
		// Auto-close mobile menu when switching to desktop
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
			document.body.classList.remove('nav-menu-open');

			if (hoverIntentTimer) clearTimeout(hoverIntentTimer);
			if (dropdownCloseTimer) clearTimeout(dropdownCloseTimer);
		};
	});
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════════ -->
<!-- HEADER -->
<!-- ═══════════════════════════════════════════════════════════════════════════════ -->
<header
	class="header"
	class:header--scrolled={isScrolled}
	bind:this={headerRef}
>
	<div class="header__container">
		<!-- LOGO - Fixed dimensions for zero CLS -->
		<div class="header__logo">
			<a
				href="/"
				class="logo"
				onclick={closeMobileMenu}
				aria-label="Revolution Trading Pros - Home"
			>
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

		<!-- DESKTOP NAVIGATION -->
		{#if isDesktop}
			<nav class="header__nav" aria-label="Main navigation">
				<ul class="nav" role="menubar">
					{#each navItems as item (item.id)}
						{#if item.submenu}
							<!-- Dropdown Nav Item -->
							<li
								class="nav__item"
								data-dropdown={item.id}
								role="none"
								onmouseenter={(e) => handleTriggerEnter(item.id, e)}
								onmouseleave={handleTriggerLeave}
							>
								<button
									type="button"
									id={`nav-trigger-${item.id}`}
									class="nav__link nav__link--dropdown"
									class:nav__link--open={activeDropdown === item.id}
									class:nav__link--active={isActiveRoute(item)}
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

								<!-- Dropdown Panel -->
								<!-- svelte-ignore a11y_interactive_supports_focus -->
								<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
								<div
									id={`dropdown-${item.id}`}
									class="dropdown"
									class:dropdown--visible={activeDropdown === item.id}
									class:dropdown--left={dropdownPositions[item.id] === 'left'}
									class:dropdown--right={dropdownPositions[item.id] === 'right'}
									role="menu"
									aria-labelledby={`nav-trigger-${item.id}`}
									bind:this={dropdownRefs[item.id]}
									onmouseenter={handleDropdownEnter}
									onmouseleave={handleDropdownLeave}
									onkeydown={(e) => handleDropdownKeydown(e, item.submenu || [], item.id)}
								>
									{#each item.submenu as sub, index (sub.href)}
										<a
											href={sub.href}
											class="dropdown__item"
											class:dropdown__item--active={currentPath === sub.href}
											role="menuitem"
											tabindex={activeDropdown === item.id ? 0 : -1}
											aria-current={currentPath === sub.href ? 'page' : undefined}
											style="--item-index: {index}"
											onclick={() => (activeDropdown = null)}
										>
											{sub.label}
										</a>
									{/each}
								</div>
							</li>
						{:else}
							<!-- Standard Nav Link -->
							<li class="nav__item" role="none">
								<a
									href={item.href}
									class="nav__link"
									class:nav__link--active={currentPath === item.href}
									role="menuitem"
									aria-current={currentPath === item.href ? 'page' : undefined}
								>
									{item.label}
								</a>
							</li>
						{/if}
					{/each}
				</ul>
			</nav>
		{/if}

		<!-- ACTION BUTTONS -->
		<div class="header__actions">
			<!-- Cart Button -->
			{#if $hasCartItems}
				<a
					href="/cart"
					class="action-btn action-btn--cart"
					aria-label="Shopping cart with {$cartItemCount} {$cartItemCount === 1 ? 'item' : 'items'}"
				>
					<IconShoppingCart size={22} aria-hidden="true" />
					{#if $cartItemCount > 0}
						<span class="action-btn__badge" aria-hidden="true">{$cartItemCount}</span>
					{/if}
				</a>
			{/if}

			<!-- Desktop Auth Section -->
			{#if isDesktop}
				{#if $isAuthenticated}
					<!-- User Menu -->
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
							aria-label="User account menu"
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
					<!-- Login Button -->
					<a href="/login" class="login-btn login-btn--outline">
						<span>Login</span>
					</a>
					<!-- Get Started CTA -->
					<a href="/get-started" class="cta-btn">
						<span>Get Started</span>
					</a>
				{/if}
			{/if}

			<!-- Mobile/Tablet: Hamburger -->
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

<!-- ═══════════════════════════════════════════════════════════════════════════════ -->
<!-- MOBILE NAVIGATION PANEL -->
<!-- ═══════════════════════════════════════════════════════════════════════════════ -->
{#if !isDesktop && isMobileMenuOpen}
	<!-- Backdrop Overlay -->
	<div
		class="mobile-overlay"
		onclick={closeMobileMenu}
		role="presentation"
		aria-hidden="true"
	></div>

	<!-- Slide-in Panel -->
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<nav
		id="mobile-nav"
		class="mobile-panel"
		bind:this={mobileNavRef}
		aria-label="Mobile navigation"
		onkeydown={handleMobileMenuKeydown}
	>
		<!-- Panel Header -->
		<div class="mobile-panel__header">
			<button
				type="button"
				class="mobile-panel__close"
				onclick={closeMobileMenu}
				aria-label="Close navigation menu"
			>
				<IconX size={24} aria-hidden="true" />
			</button>
		</div>

		<!-- Navigation Items -->
		<div class="mobile-panel__body">
			{#each navItems as item, index (item.id)}
				{#if item.submenu}
					<!-- Expandable Group -->
					<div class="mobile-nav__group">
						<button
							type="button"
							class="mobile-nav__link mobile-nav__link--parent"
							class:mobile-nav__link--active={isActiveRoute(item)}
							aria-expanded={activeMobileSubmenu === item.id}
							aria-controls={`mobile-submenu-${item.id}`}
							onclick={() => toggleMobileSubmenu(item.id)}
						>
							<span>{item.label}</span>
							<span
								class="mobile-nav__chevron"
								class:mobile-nav__chevron--open={activeMobileSubmenu === item.id}
							>
								<IconChevronRight size={18} stroke={2.5} aria-hidden="true" />
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
									class:mobile-nav__sublink--active={currentPath === sub.href}
									aria-current={currentPath === sub.href ? 'page' : undefined}
									onclick={closeMobileMenu}
								>
									{sub.label}
								</a>
							{/each}
						</div>
					</div>
				{:else}
					<!-- Standard Link -->
					<a
						href={item.href}
						class="mobile-nav__link"
						class:mobile-nav__link--active={currentPath === item.href}
						aria-current={currentPath === item.href ? 'page' : undefined}
						onclick={closeMobileMenu}
					>
						{item.label}
					</a>
				{/if}
			{/each}
		</div>
	</nav>
{/if}

<!-- ═══════════════════════════════════════════════════════════════════════════════ -->
<!-- STYLES -->
<!-- ═══════════════════════════════════════════════════════════════════════════════ -->
<style>
	/* ═══════════════════════════════════════════════════════════════════════════════
	   CSS Custom Properties (Design Tokens)
	   ═══════════════════════════════════════════════════════════════════════════════ */
	:root {
		/* Dimensions - Fixed for Zero CLS */
		--nav-height-desktop: 72px;
		--nav-height-tablet: 64px;
		--nav-height-mobile: 60px;

		--logo-width-desktop: 200px;
		--logo-width-tablet: 180px;
		--logo-width-mobile: 160px;

		--touch-target-min: 44px;
		--dropdown-width: 240px;

		/* Colors - Premium Dark Theme */
		--nav-bg: #151F31;
		--nav-bg-scrolled: #151F31;
		--nav-border: rgba(255, 255, 255, 0.06);
		--nav-border-scrolled: rgba(255, 255, 255, 0.1);

		--text-primary: rgba(255, 255, 255, 0.95);
		--text-secondary: rgba(255, 255, 255, 0.65);
		--text-muted: rgba(255, 255, 255, 0.45);

		--accent-primary: #facc15;
		--accent-primary-hover: #fde047;
		--accent-glow: rgba(250, 204, 21, 0.15);

		--danger: #ff4757;
		--danger-hover: #ff5a68;

		/* Transitions */
		--transition-fast: 120ms ease-out;
		--transition-base: 200ms ease-out;
		--transition-smooth: 300ms cubic-bezier(0.4, 0, 0.2, 1);

		/* Shadows */
		--shadow-dropdown: 0 12px 40px rgba(0, 0, 0, 0.5), 0 4px 12px rgba(0, 0, 0, 0.3);
		--shadow-header: 0 1px 0 var(--nav-border);
		--shadow-header-scrolled: 0 4px 20px rgba(0, 0, 0, 0.4), 0 1px 0 var(--nav-border-scrolled);

		/* Z-Index Scale */
		--z-header: 1000;
		--z-dropdown: 1010;
		--z-mobile-overlay: 1020;
		--z-mobile-panel: 1030;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Header - Fixed Height Container
	   ═══════════════════════════════════════════════════════════════════════════════ */
	.header {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		height: var(--nav-height-desktop);
		background: var(--nav-bg);
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
		box-shadow: var(--shadow-header);
		z-index: var(--z-header);
		transition:
			background var(--transition-base),
			box-shadow var(--transition-base);
	}

	.header--scrolled {
		background: var(--nav-bg-scrolled);
		box-shadow: var(--shadow-header-scrolled);
	}

	.header__container {
		display: grid;
		grid-template-columns: var(--logo-width-desktop) 1fr auto;
		align-items: center;
		gap: 32px;
		height: 100%;
		max-width: 1440px;
		margin: 0 auto;
		padding: 0 24px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Logo - Fixed Dimensions for Zero CLS
	   ═══════════════════════════════════════════════════════════════════════════════ */
	.header__logo {
		width: var(--logo-width-desktop);
		height: var(--nav-height-desktop);
		display: flex;
		align-items: center;
		flex-shrink: 0;
	}

	.logo {
		display: flex;
		align-items: center;
		height: 100%;
		text-decoration: none;
	}

	.logo__image {
		width: 200px;
		height: 50px;
		object-fit: contain;
		object-position: left center;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Desktop Navigation
	   ═══════════════════════════════════════════════════════════════════════════════ */
	.header__nav {
		display: flex;
		justify-content: flex-start;
	}

	.nav {
		display: flex;
		align-items: center;
		gap: 4px;
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.nav__item {
		position: relative;
	}

	.nav__link {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		min-height: var(--touch-target-min);
		padding: 0 14px;
		font-size: 14px;
		font-weight: 500;
		letter-spacing: 0.01em;
		color: var(--text-secondary);
		text-decoration: none;
		background: transparent;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		transition:
			color var(--transition-fast),
			background var(--transition-fast);
		white-space: nowrap;
	}

	.nav__link:hover,
	.nav__link:focus-visible {
		color: var(--text-primary);
		background: rgba(255, 255, 255, 0.06);
	}

	.nav__link:focus-visible {
		outline: 2px solid var(--accent-primary);
		outline-offset: 2px;
	}

	.nav__link--active {
		color: var(--accent-primary);
	}

	.nav__link--dropdown {
		padding-right: 10px;
	}

	.nav__link--open {
		color: var(--text-primary);
		background: rgba(255, 255, 255, 0.08);
	}

	.nav__link :global(.nav__chevron) {
		transition: transform var(--transition-fast);
		flex-shrink: 0;
	}

	.nav__link--open :global(.nav__chevron) {
		transform: rotate(180deg);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Dropdown Menu
	   ═══════════════════════════════════════════════════════════════════════════════ */
	.dropdown {
		position: absolute;
		top: calc(100% + 8px);
		left: 50%;
		transform: translateX(-50%);
		width: var(--dropdown-width);
		background: var(--nav-bg-scrolled);
		border: 1px solid var(--nav-border-scrolled);
		border-radius: 12px;
		box-shadow: var(--shadow-dropdown);
		padding: 8px;
		opacity: 0;
		visibility: hidden;
		pointer-events: none;
		transition:
			opacity var(--transition-base),
			transform var(--transition-base),
			visibility var(--transition-base);
		transform: translateX(-50%) translateY(-8px);
		z-index: var(--z-dropdown);
	}

	.dropdown--visible {
		opacity: 1;
		visibility: visible;
		pointer-events: auto;
		transform: translateX(-50%) translateY(0);
	}

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
		width: 100%;
		padding: 12px 16px;
		font-size: 14px;
		font-weight: 450;
		color: var(--text-secondary);
		text-decoration: none;
		border-radius: 8px;
		transition:
			color var(--transition-fast),
			background var(--transition-fast);
		animation: dropdownItemIn var(--transition-smooth) backwards;
		animation-delay: calc(var(--item-index) * 30ms);
	}

	.dropdown__item:hover,
	.dropdown__item:focus-visible {
		color: var(--text-primary);
		background: rgba(255, 255, 255, 0.08);
	}

	.dropdown__item:focus-visible {
		outline: 2px solid var(--accent-primary);
		outline-offset: -2px;
	}

	.dropdown__item--active {
		color: var(--accent-primary);
		background: var(--accent-glow);
	}

	@keyframes dropdownItemIn {
		from {
			opacity: 0;
			transform: translateX(-8px);
		}
		to {
			opacity: 1;
			transform: translateX(0);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Header Actions
	   ═══════════════════════════════════════════════════════════════════════════════ */
	.header__actions {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.action-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		position: relative;
		width: var(--touch-target-min);
		height: var(--touch-target-min);
		color: var(--text-secondary);
		background: transparent;
		border: none;
		border-radius: 10px;
		cursor: pointer;
		text-decoration: none;
		transition:
			color var(--transition-fast),
			background var(--transition-fast);
	}

	.action-btn:hover,
	.action-btn:focus-visible {
		color: var(--text-primary);
		background: rgba(255, 255, 255, 0.08);
	}

	.action-btn:focus-visible {
		outline: 2px solid var(--accent-primary);
		outline-offset: 2px;
	}

	.action-btn__badge {
		position: absolute;
		top: 4px;
		right: 4px;
		min-width: 18px;
		height: 18px;
		padding: 0 5px;
		font-size: 11px;
		font-weight: 600;
		line-height: 18px;
		text-align: center;
		color: #000;
		background: var(--accent-primary);
		border-radius: 9px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Login Button (Outline Style)
	   ═══════════════════════════════════════════════════════════════════════════════ */
	.login-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		min-height: var(--touch-target-min);
		padding: 0 20px;
		font-size: 14px;
		font-weight: 600;
		color: var(--text-primary);
		background: transparent;
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 10px;
		text-decoration: none;
		cursor: pointer;
		transition:
			background var(--transition-fast),
			border-color var(--transition-fast);
	}

	.login-btn:hover {
		background: rgba(255, 255, 255, 0.08);
		border-color: rgba(255, 255, 255, 0.3);
	}

	.login-btn:focus-visible {
		outline: 2px solid var(--accent-primary);
		outline-offset: 2px;
	}

	
	/* ═══════════════════════════════════════════════════════════════════════════════
	   Get Started CTA Button (Yellow)
	   ═══════════════════════════════════════════════════════════════════════════════ */
	.cta-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		min-height: var(--touch-target-min);
		padding: 0 24px;
		font-size: 14px;
		font-weight: 700;
		color: #0a101c;
		background: linear-gradient(135deg, #facc15, #eab308);
		border: none;
		border-radius: 10px;
		text-decoration: none;
		cursor: pointer;
		box-shadow: 0 4px 12px rgba(250, 204, 21, 0.3);
		transition:
			transform var(--transition-fast),
			box-shadow var(--transition-fast);
	}

	.cta-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 6px 20px rgba(250, 204, 21, 0.4);
	}

	.cta-btn:focus-visible {
		outline: 2px solid #facc15;
		outline-offset: 3px;
	}

	.cta-btn:active {
		transform: translateY(0) scale(0.98);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   User Menu (Desktop)
	   ═══════════════════════════════════════════════════════════════════════════════ */
	.user-menu {
		position: relative;
	}

	.user-menu__trigger {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		min-height: var(--touch-target-min);
		padding: 0 16px;
		font-size: 14px;
		font-weight: 500;
		color: var(--text-secondary);
		background: rgba(255, 255, 255, 0.06);
		border: 1px solid var(--nav-border);
		border-radius: 10px;
		cursor: pointer;
		transition:
			color var(--transition-fast),
			background var(--transition-fast),
			border-color var(--transition-fast);
	}

	.user-menu__trigger:hover,
	.user-menu__trigger:focus-visible {
		color: var(--text-primary);
		background: rgba(255, 255, 255, 0.1);
		border-color: var(--nav-border-scrolled);
	}

	.user-menu__trigger:focus-visible {
		outline: 2px solid var(--accent-primary);
		outline-offset: 2px;
	}

	.user-menu__name {
		max-width: 120px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.user-menu__dropdown {
		position: absolute;
		top: calc(100% + 8px);
		right: 0;
		width: 200px;
		background: var(--nav-bg-scrolled);
		border: 1px solid var(--nav-border-scrolled);
		border-radius: 12px;
		box-shadow: var(--shadow-dropdown);
		padding: 8px;
		opacity: 0;
		visibility: hidden;
		pointer-events: none;
		transform: translateY(-8px);
		transition:
			opacity var(--transition-base),
			transform var(--transition-base),
			visibility var(--transition-base);
		z-index: var(--z-dropdown);
	}

	.user-menu__dropdown--visible {
		opacity: 1;
		visibility: visible;
		pointer-events: auto;
		transform: translateY(0);
	}

	.user-menu__item {
		display: flex;
		align-items: center;
		gap: 10px;
		width: 100%;
		padding: 10px 14px;
		font-size: 14px;
		font-weight: 450;
		color: var(--text-secondary);
		text-decoration: none;
		background: transparent;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		transition:
			color var(--transition-fast),
			background var(--transition-fast);
	}

	.user-menu__item:hover,
	.user-menu__item:focus-visible {
		color: var(--text-primary);
		background: rgba(255, 255, 255, 0.08);
	}

	.user-menu__item:focus-visible {
		outline: 2px solid var(--accent-primary);
		outline-offset: -2px;
	}

	.user-menu__item--danger {
		color: var(--danger);
	}

	.user-menu__item--danger:hover,
	.user-menu__item--danger:focus-visible {
		color: var(--danger-hover);
		background: rgba(255, 71, 87, 0.1);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Hamburger Button
	   ═══════════════════════════════════════════════════════════════════════════════ */
	.hamburger {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: var(--touch-target-min);
		height: var(--touch-target-min);
		color: var(--text-primary);
		background: transparent;
		border: none;
		border-radius: 10px;
		cursor: pointer;
		transition: background var(--transition-fast);
	}

	.hamburger:hover,
	.hamburger:focus-visible {
		background: rgba(255, 255, 255, 0.08);
	}

	.hamburger:focus-visible {
		outline: 2px solid var(--accent-primary);
		outline-offset: 2px;
	}

	.hamburger__icon {
		display: flex;
		transition: transform var(--transition-base);
	}

	.hamburger__icon--open {
		transform: rotate(90deg);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Mobile Overlay
	   ═══════════════════════════════════════════════════════════════════════════════ */
	.mobile-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
		z-index: var(--z-mobile-overlay);
		animation: overlayIn var(--transition-smooth) forwards;
	}

	@keyframes overlayIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Mobile Panel
	   ═══════════════════════════════════════════════════════════════════════════════ */
	.mobile-panel {
		position: fixed;
		top: 0;
		right: 0;
		bottom: 0;
		width: min(320px, 85vw);
		background: var(--nav-bg-scrolled);
		border-left: 1px solid var(--nav-border-scrolled);
		z-index: var(--z-mobile-panel);
		display: flex;
		flex-direction: column;
		animation: panelSlideIn var(--transition-smooth) forwards;
		overflow: hidden;
	}

	@keyframes panelSlideIn {
		from {
			transform: translateX(100%);
		}
		to {
			transform: translateX(0);
		}
	}

	.mobile-panel__header {
		display: flex;
		justify-content: flex-end;
		padding: 16px 20px;
		border-bottom: 1px solid var(--nav-border);
	}

	.mobile-panel__close {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: var(--touch-target-min);
		height: var(--touch-target-min);
		color: var(--text-secondary);
		background: transparent;
		border: none;
		border-radius: 10px;
		cursor: pointer;
		transition:
			color var(--transition-fast),
			background var(--transition-fast);
	}

	.mobile-panel__close:hover,
	.mobile-panel__close:focus-visible {
		color: var(--text-primary);
		background: rgba(255, 255, 255, 0.08);
	}

	.mobile-panel__close:focus-visible {
		outline: 2px solid var(--accent-primary);
		outline-offset: 2px;
	}

	.mobile-panel__body {
		flex: 1;
		padding: 20px;
		display: flex;
		flex-direction: column;
		gap: 2px;
		overflow-y: auto;
	}

	.mobile-panel__footer {
		flex-shrink: 0;
		padding: 20px;
		background: var(--nav-bg-scrolled);
		border-top: 1px solid var(--nav-border);
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Mobile Navigation Links
	   ═══════════════════════════════════════════════════════════════════════════════ */
	.mobile-nav__group {
		margin-bottom: 2px;
	}

	.mobile-nav__link {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		min-height: 48px;
		padding: 14px 16px;
		font-size: 16px;
		font-weight: 500;
		color: var(--text-primary);
		text-decoration: none;
		background: transparent;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		transition:
			color var(--transition-fast),
			background var(--transition-fast);
	}

	.mobile-nav__link:hover,
	.mobile-nav__link:focus-visible {
		color: var(--text-primary);
		background: rgba(255, 255, 255, 0.06);
	}

	.mobile-nav__link:focus-visible {
		outline: 2px solid var(--accent-primary);
		outline-offset: 2px;
	}

	.mobile-nav__link--active {
		color: var(--accent-primary);
	}

	.mobile-nav__link--parent {
		text-align: left;
	}

	.mobile-nav__chevron {
		display: flex;
		color: var(--text-muted);
		transition: transform var(--transition-base);
	}

	.mobile-nav__chevron--open {
		transform: rotate(90deg);
	}

	.mobile-nav__submenu {
		max-height: 0;
		overflow: hidden;
		transition: max-height var(--transition-smooth);
		padding-left: 16px;
	}

	.mobile-nav__submenu--open {
		max-height: 500px;
	}

	.mobile-nav__sublink {
		display: block;
		width: 100%;
		min-height: var(--touch-target-min);
		padding: 12px 16px;
		font-size: 14px;
		font-weight: 450;
		color: var(--text-secondary);
		text-decoration: none;
		text-align: left;
		background: transparent;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		transition:
			color var(--transition-fast),
			background var(--transition-fast);
	}

	.mobile-nav__sublink:hover,
	.mobile-nav__sublink:focus-visible {
		color: var(--text-primary);
		background: rgba(255, 255, 255, 0.06);
	}

	.mobile-nav__sublink:focus-visible {
		outline: 2px solid var(--accent-primary);
		outline-offset: 2px;
	}

	.mobile-nav__sublink--active {
		color: var(--accent-primary);
	}

	.mobile-nav__sublink--danger {
		color: var(--danger);
	}

	.mobile-nav__sublink--danger:hover,
	.mobile-nav__sublink--danger:focus-visible {
		color: var(--danger-hover);
		background: rgba(255, 71, 87, 0.1);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Mobile User Section
	   ═══════════════════════════════════════════════════════════════════════════════ */
	.mobile-user {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.mobile-user__name {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 12px 16px;
		font-size: 14px;
		font-weight: 600;
		color: var(--text-primary);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Mobile CTA & Login Buttons
	   ═══════════════════════════════════════════════════════════════════════════════ */
	.mobile-cta {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		height: 52px;
		font-size: 16px;
		font-weight: 700;
		text-align: center;
		text-decoration: none;
		color: #0a101c;
		background: linear-gradient(135deg, #facc15, #eab308);
		border-radius: 12px;
		box-shadow: 0 4px 16px rgba(250, 204, 21, 0.35);
	}

	.mobile-login {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		height: 48px;
		font-size: 15px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		color: var(--text-primary);
		background: transparent;
		border: 1px solid rgba(255, 255, 255, 0.25);
		border-radius: 12px;
	}

	.mobile-login:hover {
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(255, 255, 255, 0.35);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Tablet Responsive (768px - 1023px)
	   ═══════════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 1023px) {
		.header {
			height: var(--nav-height-tablet);
		}

		.header__container {
			grid-template-columns: var(--logo-width-tablet) 1fr auto;
			gap: 16px;
			padding: 0 20px;
		}

		.header__logo {
			width: var(--logo-width-tablet);
			height: var(--nav-height-tablet);
		}

		.logo__image {
			width: 180px;
			height: 45px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Mobile Responsive (< 768px)
	   ═══════════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 767px) {
		.header {
			height: var(--nav-height-mobile);
		}

		.header__container {
			grid-template-columns: var(--logo-width-mobile) 1fr;
			gap: 12px;
			padding: 0 16px;
		}

		.header__logo {
			width: var(--logo-width-mobile);
			height: var(--nav-height-mobile);
		}

		.logo__image {
			width: 160px;
			height: 40px;
		}

		.header__actions {
			justify-content: flex-end;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Reduced Motion
	   ═══════════════════════════════════════════════════════════════════════════════ */
	@media (prefers-reduced-motion: reduce) {
		*,
		*::before,
		*::after {
			animation-duration: 0.01ms !important;
			animation-iteration-count: 1 !important;
			transition-duration: 0.01ms !important;
		}

		.mobile-panel {
			animation: none;
			transform: translateX(0);
		}

		.mobile-overlay {
			animation: none;
			opacity: 1;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   High Contrast Mode
	   ═══════════════════════════════════════════════════════════════════════════════ */
	@media (prefers-contrast: high) {
		.nav__link:focus-visible,
		.dropdown__item:focus-visible,
		.action-btn:focus-visible,
		.login-btn:focus-visible,
		.hamburger:focus-visible,
		.mobile-nav__link:focus-visible,
		.mobile-nav__sublink:focus-visible {
			outline-width: 3px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Global Body Scroll Lock
	   ═══════════════════════════════════════════════════════════════════════════════ */
	:global(body.nav-menu-open) {
		overflow: hidden;
		position: fixed;
		width: 100%;
	}
</style>