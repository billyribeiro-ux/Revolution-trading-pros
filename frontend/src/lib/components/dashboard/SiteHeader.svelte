<script lang="ts">
	/**
	 * Dashboard Site Header - WordPress EXACT Implementation
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * The main site header that appears at the TOP of all dashboard pages.
	 * WordPress structure: #masthead.site-header + #site-navigation.main-navigation
	 *
	 * Contains:
	 * - Logo
	 * - Main navigation with dropdowns (Live Trading Rooms, Learning, Tools, etc.)
	 * - Phone number
	 * - Cart icon
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { page } from '$app/stores';
	import { isAuthenticated, user } from '$lib/stores/auth';
	import { cartItemCount, hasCartItems } from '$lib/stores/cart';
	import { goto } from '$app/navigation';
	import { logout as logoutApi } from '$lib/api/auth';

	// Icons
	import IconShoppingCart from '@tabler/icons-svelte/icons/shopping-cart';
	import IconMenu2 from '@tabler/icons-svelte/icons/menu-2';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconChevronDown from '@tabler/icons-svelte/icons/chevron-down';
	import IconPhone from '@tabler/icons-svelte/icons/phone';

	// State
	let isMobileMenuOpen = $state(false);
	let openDropdown = $state<string | null>(null);

	// Derived
	const currentPath = $derived($page.url.pathname);

	// Navigation items matching WordPress structure
	const navItems = [
		{
			id: 'live',
			label: 'Live Trading Rooms',
			submenu: [
				{ href: '/trading-rooms/mastering-the-trade', label: 'Mastering the Trade' },
				{ href: '/trading-rooms/long-term-growth', label: 'Long-Term Growth' },
				{ href: '/trading-rooms/small-lot', label: 'Small Lot' },
				{ href: '/trading-rooms/trendy', label: 'Tr3ndy Trading' },
				{ href: '/trading-rooms/bias', label: 'BIAS' },
				{ href: '/trading-rooms/sector-secrets', label: 'Sector Secrets' },
				{ href: '/trading-rooms/moxie-indicator-mastery', label: 'Moxie Indicator' },
				{ href: '/trading-rooms/compounding-growth', label: 'Compounding Growth' },
				{ href: '/trading-rooms/voodoo-mastery', label: 'Voodoo' }
			]
		},
		{
			id: 'learning',
			label: 'Learning',
			submenu: [
				{ href: '/courses', label: 'Courses' },
				{ href: '/trading-education-center', label: '101s' },
				{ href: '/research-tools', label: 'Research Reports' },
				{ href: '/premium-newsletters/top-tier-outlook', label: 'Top Tier Outlook' }
			]
		},
		{
			id: 'tools',
			label: 'Tools',
			submenu: [
				{ href: '/indicators', label: 'Indicators' },
				{ href: '/join/scanner', label: 'The Scanner' }
			]
		},
		{
			id: 'resources',
			label: 'Resources',
			submenu: [
				{ href: '/research-tools', label: 'Research Reports' },
				{ href: '/blog', label: 'Commentary' },
				{ href: '/daily-videos', label: 'Daily Videos' },
				{ href: '/news', label: 'News' },
				{ href: '/newsletters', label: 'Newsletters' }
			]
		},
		{
			id: 'about',
			label: 'About Us',
			submenu: [
				{ href: '/about', label: 'Who We Are' },
				{ href: '/traders', label: 'Our Traders' }
			]
		},
		{
			id: 'dashboard',
			label: 'Dashboard',
			submenu: [
				{ href: '/dashboard', label: 'My Memberships' },
				{ href: '/dashboard/courses', label: 'My Classes' },
				{ href: '/dashboard/indicators', label: 'My Indicators' },
				{ href: '/dashboard/account', label: 'My Account' },
				{ href: '/support', label: 'Support' },
				{ href: '/logout', label: 'Logout' }
			]
		}
	];

	function toggleDropdown(id: string): void {
		openDropdown = openDropdown === id ? null : id;
	}

	function closeDropdown(): void {
		openDropdown = null;
	}

	function toggleMobileMenu(): void {
		isMobileMenuOpen = !isMobileMenuOpen;
	}

	async function handleLogout(): Promise<void> {
		try {
			await logoutApi();
			goto('/login');
		} catch (error) {
			console.error('Logout failed:', error);
		}
	}
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     SITE HEADER - WordPress EXACT: #masthead.site-header
     ═══════════════════════════════════════════════════════════════════════════ -->

<header id="masthead" class="site-header">
	<div class="inside-header">
		<div class="site-logo">
			<a href="/" title="Revolution Trading Pros" rel="home">
				<img
					class="header-image"
					alt="Revolution Trading Pros"
					src="/images/logo-white.png"
					width="200"
					height="40"
				/>
			</a>
		</div>
	</div>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     MAIN NAVIGATION - WordPress EXACT: #site-navigation.main-navigation
     ═══════════════════════════════════════════════════════════════════════════ -->

<nav id="site-navigation" class="main-navigation">
	<div class="inside-navigation">
		<!-- Mobile Menu Toggle -->
		<button
			class="menu-toggle"
			onclick={toggleMobileMenu}
			aria-expanded={isMobileMenuOpen}
			aria-controls="primary-menu"
		>
			{#if isMobileMenuOpen}
				<IconX size={24} />
			{:else}
				<IconMenu2 size={24} />
			{/if}
			<span class="mobile-menu">Menu</span>
		</button>

		<!-- Desktop Navigation -->
		<div id="primary-menu" class="main-nav" class:is-open={isMobileMenuOpen}>
			<ul class="menu sf-menu">
				{#each navItems as item (item.id)}
					<li
						class="menu-item"
						class:menu-item-has-children={item.submenu && item.submenu.length > 0}
						class:is-open={openDropdown === item.id}
					>
						{#if item.submenu && item.submenu.length > 0}
							<button
								class="menu-link"
								onclick={() => toggleDropdown(item.id)}
								aria-expanded={openDropdown === item.id}
							>
								{item.label}
								<span class="dropdown-menu-toggle">
									<IconChevronDown size={14} />
								</span>
							</button>
							{#if openDropdown === item.id}
								<ul class="sub-menu">
									{#each item.submenu as subItem (subItem.href)}
										<li>
											<a href={subItem.href} onclick={closeDropdown}>
												{subItem.label}
											</a>
										</li>
									{/each}
								</ul>
							{/if}
						{:else}
							<a href={item.href || '#'} class="menu-link">
								{item.label}
							</a>
						{/if}
					</li>
				{/each}

				<!-- Phone Number -->
				<li class="menu-item menu-item-phone">
					<a href="tel:5122668659">
						<IconPhone size={16} />
						<span>(512) 266-8659</span>
					</a>
				</li>

				<!-- Cart -->
				<li class="menu-item menu-item-cart">
					<a href="/cart">
						<IconShoppingCart size={20} />
						{#if $hasCartItems}
							<span class="cart-items-count">{$cartItemCount}</span>
						{/if}
					</a>
				</li>
			</ul>
		</div>
	</div>
</nav>

<!-- Close dropdown when clicking outside -->
<svelte:window onclick={(e) => {
	const target = e.target as HTMLElement;
	if (!target.closest('.menu-item-has-children')) {
		closeDropdown();
	}
}} />

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - WordPress EXACT CSS
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   CSS CUSTOM PROPERTIES
	   ═══════════════════════════════════════════════════════════════════════════ */

	.site-header,
	.main-navigation {
		--header-bg: #0a101c;
		--header-text: #fff;
		--header-text-hover: #0984ae;
		--header-height: 60px;
		--nav-height: 50px;
		--font-family: "Open Sans", sans-serif;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SITE HEADER (WordPress EXACT: #masthead.site-header)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.site-header {
		background-color: var(--header-bg);
		height: var(--header-height);
		display: flex;
		align-items: center;
		padding: 0 30px;
	}

	.inside-header {
		width: 100%;
		display: flex;
		align-items: center;
	}

	.site-logo a {
		display: flex;
		align-items: center;
	}

	.site-logo img {
		height: 40px;
		width: auto;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MAIN NAVIGATION (WordPress EXACT: #site-navigation.main-navigation)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.main-navigation {
		background-color: var(--header-bg);
		border-top: 1px solid rgba(255, 255, 255, 0.1);
		height: var(--nav-height);
		font-family: var(--font-family);
	}

	.inside-navigation {
		display: flex;
		align-items: center;
		height: 100%;
		padding: 0 30px;
	}

	/* Mobile Menu Toggle */
	.menu-toggle {
		display: none;
		background: none;
		border: none;
		color: var(--header-text);
		cursor: pointer;
		padding: 8px;
		align-items: center;
		gap: 8px;
	}

	.mobile-menu {
		font-size: 14px;
		font-weight: 600;
		text-transform: uppercase;
	}

	/* Main Nav */
	.main-nav {
		flex: 1;
	}

	.menu.sf-menu {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		align-items: center;
		gap: 0;
	}

	.menu-item {
		position: relative;
	}

	.menu-link {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 0 16px;
		height: var(--nav-height);
		color: var(--header-text);
		text-decoration: none;
		font-size: 14px;
		font-weight: 400;
		background: none;
		border: none;
		cursor: pointer;
		transition: color 0.15s ease;
	}

	.menu-link:hover {
		color: var(--header-text-hover);
	}

	.dropdown-menu-toggle {
		display: flex;
		align-items: center;
		transition: transform 0.2s ease;
	}

	.menu-item.is-open .dropdown-menu-toggle {
		transform: rotate(180deg);
	}

	/* Submenu */
	.sub-menu {
		position: absolute;
		top: 100%;
		left: 0;
		background: #fff;
		min-width: 220px;
		border-radius: 4px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
		list-style: none;
		margin: 0;
		padding: 8px 0;
		z-index: 1000;
	}

	.sub-menu li a {
		display: block;
		padding: 10px 20px;
		color: #333;
		text-decoration: none;
		font-size: 14px;
		transition: background 0.15s ease;
	}

	.sub-menu li a:hover {
		background: #f5f5f5;
		color: var(--header-text-hover);
	}

	/* Phone & Cart */
	.menu-item-phone a,
	.menu-item-cart a {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 0 12px;
		height: var(--nav-height);
		color: var(--header-text);
		text-decoration: none;
		font-size: 14px;
	}

	.menu-item-cart {
		margin-left: auto;
	}

	.menu-item-cart a {
		position: relative;
	}

	.cart-items-count {
		position: absolute;
		top: 8px;
		right: 4px;
		background: #ef4444;
		color: #fff;
		font-size: 10px;
		font-weight: 700;
		padding: 2px 5px;
		border-radius: 10px;
		min-width: 16px;
		text-align: center;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 1279px) {
		.menu-toggle {
			display: flex;
		}

		.main-nav {
			display: none;
			position: fixed;
			top: calc(var(--header-height) + var(--nav-height));
			left: 0;
			right: 0;
			bottom: 0;
			background: var(--header-bg);
			overflow-y: auto;
			z-index: 1000;
		}

		.main-nav.is-open {
			display: block;
		}

		.menu.sf-menu {
			flex-direction: column;
			align-items: stretch;
			padding: 20px;
		}

		.menu-link {
			width: 100%;
			justify-content: space-between;
			padding: 14px 0;
			border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		}

		.sub-menu {
			position: static;
			background: rgba(255, 255, 255, 0.05);
			box-shadow: none;
			border-radius: 0;
			margin: 0;
			padding: 0;
		}

		.sub-menu li a {
			color: var(--header-text);
			padding: 12px 20px;
		}

		.sub-menu li a:hover {
			background: rgba(255, 255, 255, 0.1);
		}

		.menu-item-phone,
		.menu-item-cart {
			margin-left: 0;
		}

		.menu-item-phone a,
		.menu-item-cart a {
			padding: 14px 0;
			border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		}
	}

	@media screen and (min-width: 1280px) {
		.menu-toggle {
			display: none;
		}

		.main-nav {
			display: block !important;
		}
	}
</style>
