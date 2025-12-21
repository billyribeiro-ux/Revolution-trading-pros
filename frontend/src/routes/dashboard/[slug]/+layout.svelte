<script lang="ts">
	/**
	 * Membership Dashboard Layout - Simpler Trading EXACT
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * This layout creates the exact Simpler Trading structure:
	 * - Collapsed primary sidebar (icon-only vertical bar)
	 * - Secondary sidebar with membership-specific navigation
	 * - Main content area
	 * - Mobile-optimized with hamburger menu
	 *
	 * @version 2.0.0 (December 2025)
	 */

	import { page } from '$app/stores';
	import { browser } from '$app/environment';
	import type { Snippet } from 'svelte';

	// Tabler Icons
	import IconHome from '@tabler/icons-svelte/icons/home';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconChartCandle from '@tabler/icons-svelte/icons/chart-candle';
	import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
	import IconSettings from '@tabler/icons-svelte/icons/settings';
	import IconUsers from '@tabler/icons-svelte/icons/users';
	import IconBook from '@tabler/icons-svelte/icons/book';
	import IconArchive from '@tabler/icons-svelte/icons/archive';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import IconChevronRight from '@tabler/icons-svelte/icons/chevron-right';
	import IconShoppingBag from '@tabler/icons-svelte/icons/shopping-bag';
	import IconPresentation from '@tabler/icons-svelte/icons/presentation';
	import IconMenu2 from '@tabler/icons-svelte/icons/menu-2';
	import IconX from '@tabler/icons-svelte/icons/x';

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS
	// ═══════════════════════════════════════════════════════════════════════════

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const slug = $derived($page.params.slug);
	const currentPath = $derived($page.url.pathname);
	let mobileMenuOpen = $state(false);

	// Close menu on route change
	$effect(() => {
		if (browser && currentPath) {
			mobileMenuOpen = false;
		}
	});

	// Handle escape key
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape' && mobileMenuOpen) {
			mobileMenuOpen = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// MEMBERSHIP DATA
	// ═══════════════════════════════════════════════════════════════════════════

	interface MembershipInfo {
		name: string;
		slug: string;
		icon: typeof IconUsers;
	}

	const membershipData = $derived.by((): MembershipInfo => {
		const memberships: Record<string, MembershipInfo> = {
			'mastering-the-trade': {
				name: 'Mastering the Trade',
				slug: 'mastering-the-trade',
				icon: IconUsers
			},
			'simpler-showcase': {
				name: 'Simpler Showcase',
				slug: 'simpler-showcase',
				icon: IconPresentation
			},
			'mm': {
				name: 'Moxie Indicator™ Mastery',
				slug: 'mm',
				icon: IconChartLine
			}
		};

		return memberships[slug] || {
			name: slug.split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' '),
			slug,
			icon: IconUsers
		};
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// PRIMARY SIDEBAR ITEMS (Collapsed icon-only)
	// ═══════════════════════════════════════════════════════════════════════════

	const primaryNavItems = [
		{ icon: IconHome, href: '/dashboard', title: 'Member Dashboard' },
		{ icon: IconVideo, href: '/dashboard/courses', title: 'My Classes' },
		{ icon: IconChartCandle, href: '/dashboard/indicators', title: 'My Indicators' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// SECONDARY SIDEBAR ITEMS
	// ═══════════════════════════════════════════════════════════════════════════

	interface SecondaryNavItem {
		icon: typeof IconHome;
		label: string;
		href: string;
		hasSubmenu?: boolean;
		isExternal?: boolean;
	}

	const secondaryNavItems = $derived.by((): SecondaryNavItem[] => {
		const baseItems: SecondaryNavItem[] = [
			{
				icon: IconHome,
				label: `${membershipData.name} Dashboard`,
				href: `/dashboard/${slug}`
			},
			{
				icon: IconPlayerPlay,
				label: 'Premium Daily Videos',
				href: `/dashboard/${slug}/daily-videos`
			},
			{
				icon: IconBook,
				label: 'Learning Center',
				href: `/dashboard/${slug}/learning-center`
			},
			{
				icon: IconArchive,
				label: 'Trading Room Archives',
				href: `/dashboard/${slug}/archive`
			},
			{
				icon: IconUsers,
				label: 'Meet the Traders',
				href: `/dashboard/${slug}/traders`,
				hasSubmenu: true
			},
			{
				icon: IconShoppingBag,
				label: 'Trader Store',
				href: `/dashboard/${slug}/store`,
				hasSubmenu: true
			}
		];

		// Add Simpler Showcase link for mastering-the-trade
		if (slug === 'mastering-the-trade') {
			baseItems.push({
				icon: IconPresentation,
				label: 'Simpler Showcase',
				href: '/dashboard/simpler-showcase'
			});
		}

		return baseItems;
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

	function isActive(href: string): boolean {
		if (href === `/dashboard/${slug}`) {
			return currentPath === href;
		}
		return currentPath === href || currentPath.startsWith(href + '/');
	}

	function isPrimaryActive(href: string): boolean {
		if (href === '/dashboard') {
			return currentPath === '/dashboard';
		}
		return currentPath.startsWith(href);
	}
</script>

<svelte:window on:keydown={handleKeydown} />

<!-- ═══════════════════════════════════════════════════════════════════════════
     MEMBERSHIP DASHBOARD LAYOUT - Simpler Trading EXACT
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="membership-dashboard" class:menu-open={mobileMenuOpen}>
	<!-- Mobile Header -->
	<header class="mobile-header">
		<button
			class="mobile-menu-btn"
			onclick={() => mobileMenuOpen = !mobileMenuOpen}
			aria-label={mobileMenuOpen ? 'Close menu' : 'Open menu'}
			aria-expanded={mobileMenuOpen}
		>
			{#if mobileMenuOpen}
				<IconX size={24} />
			{:else}
				<IconMenu2 size={24} />
			{/if}
		</button>
		<span class="mobile-title">{membershipData.name}</span>
		<a href="/dashboard/account" class="mobile-settings" aria-label="Settings">
			<IconSettings size={20} />
		</a>
	</header>

	<!-- Mobile Overlay -->
	{#if mobileMenuOpen}
		<button
			class="mobile-overlay"
			onclick={() => mobileMenuOpen = false}
			aria-label="Close menu"
		></button>
	{/if}

	<!-- Collapsed Primary Sidebar (Icon-only vertical bar) -->
	<nav class="primary-sidebar" aria-label="Main navigation">
		<ul class="primary-nav">
			{#each primaryNavItems as item}
				{@const IconComponent = item.icon}
				<li class:is-active={isPrimaryActive(item.href)}>
					<a href={item.href} title={item.title}>
						<IconComponent size={20} />
					</a>
				</li>
			{/each}

			<!-- Divider -->
			<li class="nav-divider"></li>

			<!-- Current membership icon (active) -->
			{@const MembershipIcon = membershipData.icon}
			<li class="is-active membership-icon">
				<a href="/dashboard/{slug}" title={membershipData.name}>
					<MembershipIcon size={20} />
				</a>
			</li>
		</ul>

		<!-- Settings at bottom -->
		<div class="primary-nav-footer">
			<a href="/dashboard/account" title="Settings">
				<IconSettings size={20} />
			</a>
		</div>
	</nav>

	<!-- Secondary Sidebar (Membership-specific navigation) -->
	<nav class="secondary-sidebar" aria-label="{membershipData.name} navigation">
		<ul class="secondary-nav">
			{#each secondaryNavItems as item}
				{@const IconComponent = item.icon}
				<li class:is-active={isActive(item.href)}>
					<a href={item.href} class:has-submenu={item.hasSubmenu}>
						<span class="nav-icon">
							<IconComponent size={18} />
						</span>
						<span class="nav-label">{item.label}</span>
						{#if item.hasSubmenu}
							<IconChevronRight size={16} class="submenu-arrow" />
						{/if}
					</a>
				</li>
			{/each}
		</ul>

		<!-- Back to Dashboard link on mobile -->
		<div class="sidebar-footer">
			<a href="/dashboard" class="back-link">
				<IconHome size={18} />
				<span>Back to Member Dashboard</span>
			</a>
		</div>
	</nav>

	<!-- Main Content Area -->
	<main class="membership-main">
		{@render children()}
	</main>

	<!-- Mobile Bottom Navigation -->
	<nav class="mobile-bottom-nav" aria-label="Quick navigation">
		<a href="/dashboard" class="bottom-nav-item" class:active={currentPath === '/dashboard'}>
			<IconHome size={20} />
			<span>Dashboard</span>
		</a>
		<a href="/dashboard/{slug}/daily-videos" class="bottom-nav-item" class:active={currentPath.includes('daily-videos')}>
			<IconPlayerPlay size={20} />
			<span>Videos</span>
		</a>
		<a href="/dashboard/{slug}/learning-center" class="bottom-nav-item" class:active={currentPath.includes('learning-center')}>
			<IconBook size={20} />
			<span>Learn</span>
		</a>
		<a href="/dashboard/{slug}/archive" class="bottom-nav-item" class:active={currentPath.includes('archive')}>
			<IconArchive size={20} />
			<span>Archive</span>
		</a>
	</nav>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - Simpler Trading EXACT + Full Device Optimization
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   ROOT CONTAINER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-dashboard {
		display: flex;
		min-height: 100vh;
		background: #f4f4f4;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MOBILE HEADER (Hidden on desktop)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.mobile-header {
		display: none;
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		height: 56px;
		background: #0f2d41;
		color: #fff;
		align-items: center;
		justify-content: space-between;
		padding: 0 16px;
		z-index: 1000;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
	}

	.mobile-menu-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		background: none;
		border: none;
		color: #fff;
		cursor: pointer;
		border-radius: 8px;
		transition: background 0.15s ease;
		-webkit-tap-highlight-color: transparent;
	}

	.mobile-menu-btn:hover,
	.mobile-menu-btn:focus {
		background: rgba(255, 255, 255, 0.1);
	}

	.mobile-title {
		font-size: 16px;
		font-weight: 600;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		max-width: 200px;
	}

	.mobile-settings {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		color: rgba(255, 255, 255, 0.7);
		text-decoration: none;
		border-radius: 8px;
		transition: all 0.15s ease;
	}

	.mobile-settings:hover {
		color: #fff;
		background: rgba(255, 255, 255, 0.1);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MOBILE OVERLAY
	   ═══════════════════════════════════════════════════════════════════════════ */

	.mobile-overlay {
		display: none;
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		z-index: 998;
		border: none;
		cursor: pointer;
		-webkit-tap-highlight-color: transparent;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PRIMARY SIDEBAR - Collapsed Icon-only Bar (Simpler Trading EXACT)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.primary-sidebar {
		width: 60px;
		background: #0f2d41;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		padding: 20px 0;
		flex-shrink: 0;
		position: relative;
		z-index: 999;
	}

	.primary-nav {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 4px;
	}

	.primary-nav li {
		width: 100%;
	}

	.primary-nav li a {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		height: 44px;
		color: hsla(0, 0%, 100%, 0.5);
		text-decoration: none;
		transition: all 0.15s ease-in-out;
		position: relative;
	}

	.primary-nav li a:hover {
		color: #fff;
		background: rgba(255, 255, 255, 0.1);
	}

	.primary-nav li.is-active a {
		color: #fff;
		background: rgba(255, 255, 255, 0.1);
	}

	.primary-nav li.is-active a::after {
		content: '';
		position: absolute;
		right: 0;
		top: 0;
		bottom: 0;
		width: 3px;
		background: #0984ae;
	}

	.nav-divider {
		width: 30px;
		height: 1px;
		background: rgba(255, 255, 255, 0.2);
		margin: 12px auto;
	}

	.membership-icon a {
		background: rgba(9, 132, 174, 0.3) !important;
	}

	.primary-nav-footer {
		display: flex;
		justify-content: center;
		padding: 0 10px;
	}

	.primary-nav-footer a {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		color: hsla(0, 0%, 100%, 0.5);
		text-decoration: none;
		border-radius: 8px;
		transition: all 0.15s ease-in-out;
	}

	.primary-nav-footer a:hover {
		color: #fff;
		background: rgba(255, 255, 255, 0.1);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SECONDARY SIDEBAR - Membership Navigation (Simpler Trading EXACT)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.secondary-sidebar {
		width: 220px;
		background: #0f2d41;
		border-left: 1px solid rgba(255, 255, 255, 0.1);
		padding: 20px 0;
		flex-shrink: 0;
		overflow-y: auto;
		display: flex;
		flex-direction: column;
		position: relative;
		z-index: 999;
	}

	.secondary-nav {
		list-style: none;
		margin: 0;
		padding: 0;
		flex: 1;
	}

	.secondary-nav li {
		margin: 0;
	}

	.secondary-nav li a {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 20px;
		color: hsla(0, 0%, 100%, 0.7);
		text-decoration: none;
		font-size: 13px;
		font-weight: 400;
		transition: all 0.15s ease-in-out;
		position: relative;
		min-height: 44px;
	}

	.secondary-nav li a:hover {
		color: #fff;
		background: rgba(255, 255, 255, 0.05);
	}

	.secondary-nav li.is-active a {
		color: #fff;
		background: rgba(9, 132, 174, 0.2);
	}

	.secondary-nav li.is-active a::before {
		content: '';
		position: absolute;
		left: 0;
		top: 0;
		bottom: 0;
		width: 3px;
		background: #0984ae;
	}

	.nav-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 20px;
		color: inherit;
		opacity: 0.8;
		flex-shrink: 0;
	}

	.nav-label {
		flex: 1;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.secondary-nav li a.has-submenu {
		padding-right: 16px;
	}

	:global(.submenu-arrow) {
		opacity: 0.5;
		flex-shrink: 0;
	}

	.sidebar-footer {
		display: none;
		padding: 16px 20px;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
		margin-top: auto;
	}

	.back-link {
		display: flex;
		align-items: center;
		gap: 10px;
		color: rgba(255, 255, 255, 0.6);
		text-decoration: none;
		font-size: 13px;
		padding: 10px 0;
	}

	.back-link:hover {
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MAIN CONTENT AREA
	   ═══════════════════════════════════════════════════════════════════════════ */

	.membership-main {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
		background: #f4f4f4;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MOBILE BOTTOM NAVIGATION (Hidden on desktop)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.mobile-bottom-nav {
		display: none;
		position: fixed;
		bottom: 0;
		left: 0;
		right: 0;
		height: 64px;
		background: #0f2d41;
		justify-content: space-around;
		align-items: center;
		padding: 0 8px;
		z-index: 1000;
		box-shadow: 0 -2px 8px rgba(0, 0, 0, 0.15);
	}

	.bottom-nav-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 4px;
		padding: 8px 12px;
		color: rgba(255, 255, 255, 0.6);
		text-decoration: none;
		font-size: 10px;
		border-radius: 8px;
		transition: all 0.15s ease;
		min-width: 60px;
		-webkit-tap-highlight-color: transparent;
	}

	.bottom-nav-item:hover,
	.bottom-nav-item.active {
		color: #fff;
		background: rgba(255, 255, 255, 0.1);
	}

	.bottom-nav-item.active {
		color: #0984ae;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - TABLET (768px - 1279px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 1279px) and (min-width: 768px) {
		.membership-dashboard {
			padding-top: 56px;
		}

		.mobile-header {
			display: flex;
		}

		.primary-sidebar {
			position: fixed;
			left: 0;
			top: 56px;
			bottom: 0;
			transform: translateX(-100%);
			transition: transform 0.3s ease-in-out;
		}

		.secondary-sidebar {
			position: fixed;
			left: 60px;
			top: 56px;
			bottom: 0;
			transform: translateX(-100%);
			transition: transform 0.3s ease-in-out;
			width: 240px;
		}

		.menu-open .primary-sidebar,
		.menu-open .secondary-sidebar {
			transform: translateX(0);
		}

		.menu-open .mobile-overlay {
			display: block;
		}

		.sidebar-footer {
			display: block;
		}

		.membership-main {
			width: 100%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - MOBILE (< 768px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 767px) {
		.membership-dashboard {
			padding-top: 56px;
			padding-bottom: 64px;
		}

		.mobile-header {
			display: flex;
		}

		.mobile-bottom-nav {
			display: flex;
		}

		.primary-sidebar {
			display: none;
		}

		.secondary-sidebar {
			position: fixed;
			left: 0;
			top: 56px;
			bottom: 64px;
			width: 280px;
			transform: translateX(-100%);
			transition: transform 0.3s ease-in-out;
			border-left: none;
		}

		.menu-open .secondary-sidebar {
			transform: translateX(0);
		}

		.menu-open .mobile-overlay {
			display: block;
		}

		.sidebar-footer {
			display: block;
		}

		.membership-main {
			width: 100%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SMALL MOBILE (< 375px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 374px) {
		.mobile-title {
			font-size: 14px;
			max-width: 140px;
		}

		.bottom-nav-item {
			min-width: 50px;
			padding: 8px 8px;
			font-size: 9px;
		}

		.secondary-sidebar {
			width: 260px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   REDUCED MOTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.primary-nav li a,
		.secondary-nav li a,
		.primary-nav-footer a,
		.primary-sidebar,
		.secondary-sidebar,
		.mobile-menu-btn,
		.bottom-nav-item {
			transition: none;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   TOUCH DEVICE OPTIMIZATIONS
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (hover: none) and (pointer: coarse) {
		.primary-nav li a,
		.secondary-nav li a,
		.bottom-nav-item {
			min-height: 48px;
		}

		.secondary-nav li a {
			padding: 14px 20px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   LANDSCAPE MOBILE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-height: 500px) and (orientation: landscape) {
		.mobile-bottom-nav {
			height: 52px;
		}

		.bottom-nav-item {
			padding: 4px 12px;
		}

		.secondary-sidebar {
			bottom: 52px;
		}

		.membership-dashboard {
			padding-bottom: 52px;
		}
	}
</style>
