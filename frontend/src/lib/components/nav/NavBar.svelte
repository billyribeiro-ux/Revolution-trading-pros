<script lang="ts">
	/**
	 * NavBar - Enterprise Navigation Component
	 * Apple/Google/Microsoft/Netflix Quality Standard
	 * 
	 * Features:
	 * - Three-zone layout (logo | nav | actions)
	 * - Static logo (zero CLS)
	 * - Click-based dropdowns with keyboard support
	 * - Full mobile panel with focus trap
	 * - Sticky with scroll-aware background
	 * - WCAG 2.1 AA accessible
	 * 
	 * @version 2.0.0 - Navbar Architect Spec
	 * @author Revolution Trading Pros
	 */
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import {
		IconShoppingCart,
		IconMenu2
	} from '@tabler/icons-svelte';
	import { authStore, isAuthenticated } from '$lib/stores/auth';
	import { cartItemCount, hasCartItems } from '$lib/stores/cart';
	import { logout as logoutApi } from '$lib/api/auth';
	import NavDropdown from './NavDropdown.svelte';
	import NavMobilePanel from './NavMobilePanel.svelte';
	import type { NavItem } from './types';

	// Props
	interface Props {
		isAdmin?: boolean;
	}
	let { isAdmin = false }: Props = $props();

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

	// Dashboard items for authenticated users
	const dashboardItems = [
		{ href: '/dashboard', label: 'Dashboard Home' },
		{ href: '/dashboard/courses', label: 'My Courses' },
		{ href: '/dashboard/indicators', label: 'My Indicators' },
		{ href: '/dashboard/account', label: 'My Account' }
	];

	// State
	let activeDropdown = $state<string | null>(null);
	let isMobileMenuOpen = $state(false);
	let isScrolled = $state(false);
	let headerRef = $state<HTMLElement | null>(null);

	// Derived
	const currentPath = $derived($page.url.pathname);

	function isActiveRoute(item: NavItem): boolean {
		if (item.href && currentPath === item.href) return true;
		if (item.submenu) {
			return item.submenu.some(sub => 
				currentPath === sub.href || currentPath.startsWith(sub.href + '/')
			);
		}
		return false;
	}

	// Handlers
	function handleDropdownToggle(id: string) {
		activeDropdown = activeDropdown === id ? null : id;
	}

	function handleDropdownClose() {
		activeDropdown = null;
	}

	function openMobileMenu() {
		isMobileMenuOpen = true;
	}

	function closeMobileMenu() {
		isMobileMenuOpen = false;
	}

	async function handleLogout() {
		activeDropdown = null;
		try {
			await logoutApi();
		} catch (e) {
			console.error('Logout error:', e);
		}
		authStore.clearAuth();
		await goto('/login');
	}

	// Close dropdowns on click outside
	function handleClickOutside(event: MouseEvent) {
		const target = event.target as HTMLElement;
		if (activeDropdown && !target.closest('[data-dropdown]')) {
			activeDropdown = null;
		}
	}

	// Scroll detection for sticky background
	function handleScroll() {
		isScrolled = window.scrollY > 20;
	}

	onMount(() => {
		document.addEventListener('click', handleClickOutside);
		window.addEventListener('scroll', handleScroll, { passive: true });
		handleScroll(); // Check initial state
		
		return () => {
			document.removeEventListener('click', handleClickOutside);
			window.removeEventListener('scroll', handleScroll);
		};
	});
</script>

<header 
	bind:this={headerRef}
	class="navbar"
	class:scrolled={isScrolled}
>
	<div class="navbar-container">
		<!-- ZONE 1: Logo (Static, Non-Moving) -->
		<a href="/" class="navbar-logo" aria-label="Revolution Trading Pros Home">
			<img
				src="/revolution-trading-pros.png"
				alt="Revolution Trading Pros"
				width="200"
				height="50"
				loading="eager"
				fetchpriority="high"
			/>
		</a>

		<!-- ZONE 2: Primary Navigation (Desktop) -->
		<nav class="navbar-nav" aria-label="Main navigation">
			{#each navItems as item (item.id)}
				{#if item.submenu}
					<NavDropdown
						id={item.id}
						label={item.label}
						items={item.submenu}
						isActive={isActiveRoute(item)}
						isOpen={activeDropdown === item.id}
						onToggle={handleDropdownToggle}
						onClose={handleDropdownClose}
					/>
				{:else}
					<a 
						href={item.href} 
						class="navbar-link"
						class:active={currentPath === item.href}
					>
						{item.label}
					</a>
				{/if}
			{/each}

			<!-- Dashboard dropdown (authenticated users) -->
			{#if $isAuthenticated}
				<NavDropdown
					id="dashboard"
					label="Dashboard"
					items={[
						...dashboardItems,
						{ href: '#logout', label: 'Logout' }
					]}
					isActive={currentPath.startsWith('/dashboard')}
					isOpen={activeDropdown === 'dashboard'}
					onToggle={handleDropdownToggle}
					onClose={handleDropdownClose}
					onNavigate={() => {
						// Handle logout specially
						const lastItem = dashboardItems[dashboardItems.length - 1];
						if (activeDropdown === 'dashboard') {
							// Check if logout was clicked via URL
						}
					}}
				/>
			{/if}
		</nav>

		<!-- ZONE 3: Actions (Cart, Auth, Mobile Toggle) -->
		<div class="navbar-actions">
			<!-- Cart (if items) -->
			{#if $hasCartItems}
				<a 
					href="/cart" 
					class="navbar-cart"
					aria-label="Shopping cart with {$cartItemCount} items"
				>
					<IconShoppingCart size={22} />
					{#if $cartItemCount > 0}
						<span class="navbar-cart-badge">{$cartItemCount}</span>
					{/if}
				</a>
			{/if}

			<!-- Auth buttons (desktop) -->
			{#if !$isAuthenticated}
				<a href="/login" class="navbar-login">Login</a>
				<a href="/get-started" class="navbar-cta">Get Started</a>
			{/if}

			<!-- Mobile menu toggle -->
			<button 
				class="navbar-mobile-toggle"
				onclick={openMobileMenu}
				aria-label="Open navigation menu"
				aria-expanded={isMobileMenuOpen}
			>
				<IconMenu2 size={24} />
			</button>
		</div>
	</div>
</header>

<!-- Mobile Navigation Panel -->
<NavMobilePanel
	isOpen={isMobileMenuOpen}
	items={navItems}
	onClose={closeMobileMenu}
/>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   NAVBAR SHELL
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

	/* When admin toolbar is present */
	:global(.has-admin-toolbar) .navbar {
		top: 0;
	}

	.navbar.scrolled {
		background: rgba(21, 31, 49, 0.97);
		backdrop-filter: blur(12px);
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
	}

	/* Tablet height */
	@media (max-width: 1023px) {
		.navbar {
			height: 64px;
		}
	}

	/* Mobile height */
	@media (max-width: 767px) {
		.navbar {
			height: 60px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTAINER - Three Zone Layout
	   ═══════════════════════════════════════════════════════════════════════════ */
	.navbar-container {
		display: flex;
		align-items: center;
		justify-content: space-between;
		height: 100%;
		max-width: 1440px;
		margin: 0 auto;
		padding: 0 32px;
		gap: 24px;
	}

	@media (max-width: 1279px) {
		.navbar-container {
			padding: 0 24px;
			gap: 16px;
		}
	}

	@media (max-width: 767px) {
		.navbar-container {
			padding: 0 16px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ZONE 1: LOGO (Static, Non-Moving)
	   ═══════════════════════════════════════════════════════════════════════════ */
	.navbar-logo {
		flex-shrink: 0;
		display: flex;
		align-items: center;
		width: 200px; /* Fixed width prevents CLS */
	}

	.navbar-logo img {
		width: 200px;
		height: 50px;
		object-fit: contain;
	}

	@media (max-width: 1023px) {
		.navbar-logo {
			width: 180px;
		}
		.navbar-logo img {
			width: 180px;
			height: 45px;
		}
	}

	@media (max-width: 767px) {
		.navbar-logo {
			width: 160px;
		}
		.navbar-logo img {
			width: 160px;
			height: 40px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ZONE 2: PRIMARY NAVIGATION
	   ═══════════════════════════════════════════════════════════════════════════ */
	.navbar-nav {
		display: flex;
		align-items: center;
		gap: 4px;
		flex: 1;
	}

	/* Hide on tablet and below */
	@media (max-width: 1023px) {
		.navbar-nav {
			display: none;
		}
	}

	.navbar-link {
		display: flex;
		align-items: center;
		padding: 10px 14px;
		color: rgba(255, 255, 255, 0.75);
		font-size: 14px;
		font-weight: 500;
		text-decoration: none;
		border-radius: 8px;
		transition: color 0.15s ease, background-color 0.15s ease;
	}

	.navbar-link:hover,
	.navbar-link.active {
		color: #facc15;
		background: rgba(255, 255, 255, 0.05);
	}

	.navbar-link:focus-visible {
		outline: none;
		box-shadow: 0 0 0 2px rgba(250, 204, 21, 0.5);
	}

	/* Reduce spacing on smaller desktops */
	@media (max-width: 1279px) {
		.navbar-nav {
			gap: 2px;
		}
		.navbar-link {
			padding: 10px 10px;
			font-size: 13px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ZONE 3: ACTIONS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.navbar-actions {
		display: flex;
		align-items: center;
		gap: 12px;
		flex-shrink: 0;
	}

	/* Cart Button */
	.navbar-cart {
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
		transition: background-color 0.15s ease;
	}

	.navbar-cart:hover {
		background: rgba(139, 92, 246, 0.2);
	}

	.navbar-cart:focus-visible {
		outline: none;
		box-shadow: 0 0 0 2px rgba(139, 92, 246, 0.5);
	}

	.navbar-cart-badge {
		position: absolute;
		top: -4px;
		right: -4px;
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 20px;
		height: 20px;
		padding: 0 6px;
		background: #facc15;
		color: #0a101c;
		font-size: 11px;
		font-weight: 700;
		border-radius: 10px;
	}

	/* Login Button */
	.navbar-login {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 44px;
		padding: 0 20px;
		color: white;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		background: transparent;
		border: 1px solid rgba(255, 255, 255, 0.25);
		border-radius: 10px;
		transition: background-color 0.15s ease, border-color 0.15s ease;
	}

	.navbar-login:hover {
		background: rgba(255, 255, 255, 0.05);
		border-color: rgba(255, 255, 255, 0.4);
	}

	.navbar-login:focus-visible {
		outline: none;
		box-shadow: 0 0 0 2px rgba(255, 255, 255, 0.3);
	}

	/* Hide login on tablet and below */
	@media (max-width: 1023px) {
		.navbar-login {
			display: none;
		}
	}

	/* CTA Button */
	.navbar-cta {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 44px;
		padding: 0 24px;
		color: #0a101c;
		font-size: 14px;
		font-weight: 700;
		text-decoration: none;
		background: linear-gradient(135deg, #facc15, #eab308);
		border-radius: 10px;
		box-shadow: 0 4px 12px rgba(250, 204, 21, 0.3);
		transition: transform 0.15s ease, box-shadow 0.15s ease;
	}

	.navbar-cta:hover {
		transform: translateY(-2px);
		box-shadow: 0 6px 20px rgba(250, 204, 21, 0.4);
	}

	.navbar-cta:focus-visible {
		outline: none;
		box-shadow: 0 0 0 2px rgba(250, 204, 21, 0.5), 0 4px 12px rgba(250, 204, 21, 0.3);
	}

	/* Hide CTA on mobile */
	@media (max-width: 767px) {
		.navbar-cta {
			display: none;
		}
	}

	/* Mobile Menu Toggle */
	.navbar-mobile-toggle {
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
		transition: background-color 0.15s ease, color 0.15s ease;
	}

	.navbar-mobile-toggle:hover {
		background: rgba(255, 255, 255, 0.1);
		color: white;
	}

	.navbar-mobile-toggle:focus-visible {
		outline: none;
		box-shadow: 0 0 0 2px rgba(250, 204, 21, 0.5);
	}

	/* Show mobile toggle on tablet and below */
	@media (max-width: 1023px) {
		.navbar-mobile-toggle {
			display: flex;
		}
	}
</style>
