<script lang="ts">
	/**
	 * Dashboard Sidebar Component - Svelte 5 Implementation
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Matches WordPress WooCommerce account navigation structure with:
	 * - Svelte 5 runes ($state, $derived, $effect, $props)
	 * - Two-part sidebar: collapsed icons + expanded menu
	 * - Mobile responsive with overlay support
	 * - Service-specific icon backgrounds
	 * - Accessibility improvements
	 *
	 * @version 3.0.0 (Svelte 5 / December 2025)
	 */

	import { page } from '$app/stores';
	import { browser } from '$app/environment';
	import {
		IconHome,
		IconUser,
		IconCreditCard,
		IconMapPin,
		IconReceipt,
		IconLogout,
		IconSettings,
		IconChartBar,
		IconBook,
		IconBell,
		IconChevronLeft,
		IconChevronRight
	} from '@tabler/icons-svelte';
	import type { UserMembership } from '$lib/api/user-memberships';
	import '$lib/styles/st-icons.css';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	interface MenuItem {
		href: string;
		label: string;
		icon: typeof IconHome;
		id: string;
		badge?: number;
	}

	interface Props {
		memberships?: UserMembership[];
		isCollapsed?: boolean;
		isMobileOpen?: boolean;
		onToggleCollapse?: () => void;
		onCloseMobile?: () => void;
		userAvatar?: string;
		userName?: string;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS & STATE (Svelte 5 Runes)
	// ═══════════════════════════════════════════════════════════════════════════

	let {
		memberships = [],
		isCollapsed = $bindable(false),
		isMobileOpen = $bindable(false),
		onToggleCollapse,
		onCloseMobile,
		userAvatar = '',
		userName = 'My Account'
	}: Props = $props();

	// Local state using Svelte 5 $state rune
	let hoveredItem = $state<string | null>(null);
	let expandedSection = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// MENU CONFIGURATION
	// ═══════════════════════════════════════════════════════════════════════════

	const accountMenuItems: MenuItem[] = [
		{ href: '/dashboard', label: 'Dashboard', icon: IconHome, id: 'dashboard' },
		{ href: '/dashboard/account', label: 'Account Details', icon: IconUser, id: 'account' },
		{ href: '/dashboard/orders', label: 'Orders', icon: IconReceipt, id: 'orders' },
		{ href: '/dashboard/subscriptions', label: 'Subscriptions', icon: IconCreditCard, id: 'subscriptions' },
		{ href: '/dashboard/addresses', label: 'Addresses', icon: IconMapPin, id: 'addresses' },
		{ href: '/dashboard/payment-methods', label: 'Payment Methods', icon: IconCreditCard, id: 'payment-methods' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE (Svelte 5 $derived rune)
	// ═══════════════════════════════════════════════════════════════════════════

	// Group memberships by type
	const tradingRooms = $derived(memberships.filter(m => m.type === 'trading-room'));
	const alertServices = $derived(memberships.filter(m => m.type === 'alert-service'));
	const courses = $derived(memberships.filter(m => m.type === 'course'));
	const indicators = $derived(memberships.filter(m => m.type === 'indicator'));

	// Check if any memberships exist
	const hasMemberships = $derived(memberships.length > 0);
	const hasTradingRooms = $derived(tradingRooms.length > 0);
	const hasAlertServices = $derived(alertServices.length > 0);
	const hasCourses = $derived(courses.length > 0);
	const hasIndicators = $derived(indicators.length > 0);

	// Current path for active state
	const currentPath = $derived($page.url.pathname);

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPER FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function isActive(href: string): boolean {
		if (href === '/dashboard') {
			return currentPath === '/dashboard';
		}
		return currentPath === href || currentPath.startsWith(href + '/');
	}

	function getServiceIconClass(slug: string): string {
		const iconMap: Record<string, string> = {
			'day-trading': 'st-icon-day-trading',
			'small-accounts': 'st-icon-small-accounts',
			'spx-profit-pulse': 'st-icon-spx-profit-pulse',
			'explosive-swings': 'st-icon-explosive-swings',
			'swing-trading': 'st-icon-swing-trading'
		};
		return iconMap[slug] || 'st-icon-chart';
	}

	function handleKeyDown(event: KeyboardEvent, action: () => void): void {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			action();
		}
	}

	function toggleSection(section: string): void {
		expandedSection = expandedSection === section ? null : section;
	}

	function handleToggleCollapse(): void {
		isCollapsed = !isCollapsed;
		onToggleCollapse?.();

		// Persist preference
		if (browser) {
			localStorage.setItem('sidebar-collapsed', String(isCollapsed));
		}
	}

	function handleCloseMobile(): void {
		isMobileOpen = false;
		onCloseMobile?.();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS (Svelte 5 $effect rune)
	// ═══════════════════════════════════════════════════════════════════════════

	// Load saved collapse preference
	$effect(() => {
		if (browser) {
			const saved = localStorage.getItem('sidebar-collapsed');
			if (saved !== null) {
				isCollapsed = saved === 'true';
			}
		}
	});

	// Close mobile menu on route change
	$effect(() => {
		if (browser && isMobileOpen) {
			// This runs when currentPath changes
			currentPath;
			handleCloseMobile();
		}
	});

	// Handle escape key to close mobile menu
	$effect(() => {
		if (!browser || !isMobileOpen) return;

		function handleEscape(event: KeyboardEvent): void {
			if (event.key === 'Escape') {
				handleCloseMobile();
			}
		}

		document.addEventListener('keydown', handleEscape);
		return () => document.removeEventListener('keydown', handleEscape);
	});
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     TEMPLATE
     ═══════════════════════════════════════════════════════════════════════════ -->

<nav
	class="dashboard-sidebar"
	class:is-collapsed={isCollapsed}
	class:is-mobile-open={isMobileOpen}
	role="navigation"
	aria-label="Dashboard navigation"
>
	<!-- Collapse Toggle Button -->
	<button
		class="collapse-toggle"
		onclick={handleToggleCollapse}
		aria-label={isCollapsed ? 'Expand sidebar' : 'Collapse sidebar'}
		aria-expanded={!isCollapsed}
	>
		{#if isCollapsed}
			<IconChevronRight size={18} />
		{:else}
			<IconChevronLeft size={18} />
		{/if}
	</button>

	<!-- Primary Menu (Icons) -->
	<ul class="account-primary-menu" class:is-collapsed={isCollapsed} role="menubar">
		<!-- Profile Section -->
		<li class="profile-section" role="none">
			<a
				href="/dashboard"
				class="dashboard-profile-nav-item"
				role="menuitem"
				aria-current={isActive('/dashboard') ? 'page' : undefined}
			>
				<span
					class="dashboard-profile-photo"
					style:background-image={userAvatar ? `url(${userAvatar})` : undefined}
					aria-hidden="true"
				></span>
				{#if !isCollapsed}
					<span class="dashboard-profile-name">{userName}</span>
				{/if}
			</a>
		</li>

		<!-- Trading Rooms Section -->
		{#if hasTradingRooms}
			<li class="menu-category" role="none">
				{#if !isCollapsed}
					<p class="dashboard-menu-category">Trading Rooms</p>
				{/if}
			</li>
			{#each tradingRooms as room (room.id)}
				<li
					class="{room.slug}-mp cstooltip"
					class:is-active={isActive(`/live-trading-rooms/${room.slug}`)}
					role="none"
					onmouseenter={() => hoveredItem = room.id}
					onmouseleave={() => hoveredItem = null}
				>
					{#if isCollapsed}
						<span class="cstooltiptext" role="tooltip">{room.name}</span>
					{/if}
					<a
						href="/live-trading-rooms/{room.slug}"
						role="menuitem"
						aria-label={room.name}
						aria-current={isActive(`/live-trading-rooms/${room.slug}`) ? 'page' : undefined}
					>
						<span class="dashboard-menu-item-icon {getServiceIconClass(room.slug)}">
							{#if room.icon}
								<img src={room.icon} alt="" class="service-icon-img" loading="lazy" />
							{:else}
								<i class="fa fa-graduation-cap" aria-hidden="true"></i>
							{/if}
						</span>
						{#if !isCollapsed}
							<span class="menu-label">{room.name}</span>
						{/if}
					</a>
				</li>
			{/each}
		{/if}

		<!-- Alert Services Section -->
		{#if hasAlertServices}
			<li class="menu-category" role="none">
				{#if !isCollapsed}
					<p class="dashboard-menu-category">Alert Services</p>
				{/if}
			</li>
			{#each alertServices as alert (alert.id)}
				<li
					class="{alert.slug}-mp cstooltip"
					class:is-active={isActive(`/alerts/${alert.slug}`)}
					role="none"
					onmouseenter={() => hoveredItem = alert.id}
					onmouseleave={() => hoveredItem = null}
				>
					{#if isCollapsed}
						<span class="cstooltiptext" role="tooltip">{alert.name}</span>
					{/if}
					<a
						href="/alerts/{alert.slug}"
						role="menuitem"
						aria-label={alert.name}
					>
						<span class="dashboard-menu-item-icon {getServiceIconClass(alert.slug)}">
							{#if alert.icon}
								<img src={alert.icon} alt="" class="service-icon-img" loading="lazy" />
							{:else}
								<IconBell size={20} />
							{/if}
						</span>
						{#if !isCollapsed}
							<span class="menu-label">{alert.name}</span>
						{/if}
					</a>
				</li>
			{/each}
		{/if}

		<!-- Courses Section -->
		{#if hasCourses}
			<li class="menu-category" role="none">
				{#if !isCollapsed}
					<p class="dashboard-menu-category">Courses</p>
				{/if}
			</li>
			{#each courses as course (course.id)}
				<li
					class="{course.slug}-mp cstooltip"
					class:is-active={isActive(`/dashboard/courses/${course.slug}`)}
					role="none"
				>
					{#if isCollapsed}
						<span class="cstooltiptext" role="tooltip">{course.name}</span>
					{/if}
					<a href="/dashboard/courses/{course.slug}" role="menuitem">
						<span class="dashboard-menu-item-icon st-icon-learning-center">
							<IconBook size={20} />
						</span>
						{#if !isCollapsed}
							<span class="menu-label">{course.name}</span>
						{/if}
					</a>
				</li>
			{/each}
		{/if}

		<!-- Indicators Section -->
		{#if hasIndicators}
			<li class="menu-category" role="none">
				{#if !isCollapsed}
					<p class="dashboard-menu-category">My Indicators</p>
				{/if}
			</li>
			{#each indicators as indicator (indicator.id)}
				<li
					class="{indicator.slug}-mp cstooltip"
					class:is-active={isActive(`/dashboard/indicators/${indicator.slug}`)}
					role="none"
				>
					{#if isCollapsed}
						<span class="cstooltiptext" role="tooltip">{indicator.name}</span>
					{/if}
					<a href="/dashboard/indicators/{indicator.slug}" role="menuitem">
						<span class="dashboard-menu-item-icon icon-handle-stick">
							<IconChartBar size={20} />
						</span>
						{#if !isCollapsed}
							<span class="menu-label">{indicator.name}</span>
						{/if}
					</a>
				</li>
			{/each}
		{/if}
	</ul>

	<!-- Secondary Menu (Expanded with labels) -->
	{#if !isCollapsed}
		<ul class="account-secondry-menu" role="menubar">
			<li class="woocommerce-MyAccount-navigation-link" role="none">
				<p class="dashboard-menu-category">Account</p>
			</li>
			{#each accountMenuItems as item (item.id)}
				{@const Icon = item.icon}
				<li
					class="woocommerce-MyAccount-navigation-link woocommerce-MyAccount-navigation-link--{item.id}"
					class:is-active={isActive(item.href)}
					role="none"
				>
					<a
						href={item.href}
						role="menuitem"
						aria-current={isActive(item.href) ? 'page' : undefined}
					>
						<span class="dashboard-menu-item-icon" aria-hidden="true">
							<Icon size={18} />
						</span>
						<span>{item.label}</span>
						{#if item.badge}
							<span class="menu-badge" aria-label="{item.badge} new items">{item.badge}</span>
						{/if}
					</a>
				</li>
			{/each}
			<li class="woocommerce-MyAccount-navigation-link woocommerce-MyAccount-navigation-link--logout" role="none">
				<a href="/logout" role="menuitem">
					<span class="dashboard-menu-item-icon" aria-hidden="true">
						<IconLogout size={18} />
					</span>
					<span>Logout</span>
				</a>
			</li>
		</ul>
	{/if}
</nav>

<!-- Mobile Overlay -->
{#if isMobileOpen}
	<button
		class="sidebar-overlay"
		onclick={handleCloseMobile}
		onkeydown={(e) => handleKeyDown(e, handleCloseMobile)}
		aria-label="Close sidebar"
		tabindex="0"
	></button>
{/if}

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   CSS CUSTOM PROPERTIES
	   ═══════════════════════════════════════════════════════════════════════════ */

	:root {
		--sidebar-bg-primary: #0f2d41;
		--sidebar-bg-secondary: #153e59;
		--sidebar-bg-hover: #12354c;
		--sidebar-text: hsla(0, 0%, 100%, 0.5);
		--sidebar-text-active: #fff;
		--sidebar-accent: #0984ae;
		--sidebar-accent-hover: #076787;
		--sidebar-icon-color: #8796A0;
		--sidebar-tooltip-bg: #fff;
		--sidebar-tooltip-text: #0984ae;
		--sidebar-width-expanded: 280px;
		--sidebar-width-collapsed: 80px;
		--sidebar-transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MAIN SIDEBAR
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard-sidebar {
		display: flex;
		flex: 0 0 auto;
		flex-flow: row;
		background-color: var(--sidebar-bg-primary);
		min-height: calc(100vh - 80px);
		position: relative;
		transition: var(--sidebar-transition);
		width: var(--sidebar-width-expanded);
	}

	.dashboard-sidebar.is-collapsed {
		width: var(--sidebar-width-collapsed);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   COLLAPSE TOGGLE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.collapse-toggle {
		position: absolute;
		right: -12px;
		top: 50%;
		transform: translateY(-50%);
		width: 24px;
		height: 24px;
		border-radius: 50%;
		background: var(--sidebar-accent);
		border: 2px solid var(--sidebar-bg-primary);
		color: white;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 10;
		opacity: 0;
		transition: opacity 0.2s ease;
	}

	.dashboard-sidebar:hover .collapse-toggle {
		opacity: 1;
	}

	.collapse-toggle:hover {
		background: var(--sidebar-accent-hover);
		transform: translateY(-50%) scale(1.1);
	}

	.collapse-toggle:focus-visible {
		opacity: 1;
		outline: 2px solid var(--sidebar-accent);
		outline-offset: 2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PRIMARY MENU (Icon sidebar)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.account-primary-menu {
		display: block;
		padding: 30px 0 30px 30px;
		font-size: 16px;
		width: 100%;
		list-style: none;
		margin: 0;
		transition: var(--sidebar-transition);
	}

	.account-primary-menu.is-collapsed {
		width: var(--sidebar-width-collapsed);
		padding: 30px 0 30px 10px;
	}

	.account-primary-menu li {
		position: relative;
		list-style: none;
		margin-bottom: 10px;
	}

	.account-primary-menu a {
		color: var(--sidebar-text);
		min-height: 40px;
		display: flex;
		align-items: center;
		gap: 10px;
		font-weight: 300;
		text-decoration: none;
		transition: var(--sidebar-transition);
		padding: 8px 12px;
		border-radius: 8px;
	}

	.account-primary-menu a:hover {
		color: var(--sidebar-text-active);
		background: rgba(255, 255, 255, 0.05);
	}

	.account-primary-menu a:focus-visible {
		outline: 2px solid var(--sidebar-accent);
		outline-offset: 2px;
	}

	.account-primary-menu li.is-active a {
		color: var(--sidebar-text-active);
		background: var(--sidebar-accent);
	}

	.account-primary-menu.is-collapsed a {
		padding: 5px 0 0;
		justify-content: center;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PROFILE SECTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	.profile-section {
		margin-bottom: 20px !important;
	}

	.dashboard-profile-nav-item {
		height: 50px;
		line-height: 50px;
		margin-top: 20px;
		margin-bottom: 20px;
	}

	.dashboard-profile-photo {
		display: block;
		width: 34px;
		height: 34px;
		border: 2px solid #fff;
		border-radius: 50%;
		background: url('https://secure.gravatar.com/avatar/?s=32&d=mm&r=g') no-repeat center;
		background-size: cover;
		transition: var(--sidebar-transition);
		flex-shrink: 0;
	}

	.dashboard-profile-name {
		display: block;
		color: var(--sidebar-text-active);
		font-weight: 600;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MENU CATEGORY HEADERS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard-menu-category {
		font-weight: 700;
		margin-bottom: 5px;
		margin-top: 20px;
		color: var(--sidebar-text-active);
		text-transform: uppercase;
		font-size: 11px;
		letter-spacing: 0.5px;
		opacity: 0.7;
	}

	.menu-category {
		margin-bottom: 5px !important;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MENU ITEM ICONS
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard-menu-item-icon {
		line-height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--sidebar-icon-color);
		transition: var(--sidebar-transition);
		flex-shrink: 0;
		width: 24px;
		height: 24px;
	}

	.account-primary-menu a:hover .dashboard-menu-item-icon,
	.account-primary-menu li.is-active .dashboard-menu-item-icon {
		color: var(--sidebar-text-active);
	}

	.service-icon-img {
		width: 24px;
		height: 24px;
		border-radius: 50%;
		object-fit: cover;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   TOOLTIP (Collapsed state)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.cstooltip {
		position: relative;
	}

	.cstooltiptext {
		background-color: var(--sidebar-tooltip-bg);
		color: var(--sidebar-tooltip-text);
		transform: translateX(5px);
		transition: var(--sidebar-transition);
		border-radius: 6px;
		z-index: 100;
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
		left: 60px;
		top: 50%;
		transform: translateY(-50%);
		height: auto;
		visibility: hidden;
		opacity: 0;
		line-height: 1.4;
		padding: 8px 12px;
		position: absolute;
		white-space: nowrap;
		font-size: 14px;
		font-weight: 500;
		pointer-events: none;
	}

	.cstooltip:hover .cstooltiptext,
	.cstooltip:focus-within .cstooltiptext {
		visibility: visible;
		opacity: 1;
		transform: translateY(-50%) translateX(0);
	}

	/* Tooltip arrow */
	.cstooltiptext::before {
		content: '';
		position: absolute;
		left: -6px;
		top: 50%;
		transform: translateY(-50%);
		border-width: 6px;
		border-style: solid;
		border-color: transparent var(--sidebar-tooltip-bg) transparent transparent;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SECONDARY MENU
	   ═══════════════════════════════════════════════════════════════════════════ */

	.account-secondry-menu {
		width: calc(100% - 20px);
		font-size: 14px;
		font-weight: 600;
		background-color: var(--sidebar-bg-secondary);
		padding: 20px 15px;
		list-style: none;
		margin: 0;
		transition: var(--sidebar-transition);
		border-radius: 0;
	}

	.account-secondry-menu li {
		display: block;
		list-style: none;
	}

	.account-secondry-menu li > a {
		cursor: pointer;
		display: inline-flex;
		align-items: center;
		padding: 12px 15px;
		color: var(--sidebar-text);
		border-radius: 8px;
		background-color: transparent;
		gap: 10px;
		text-decoration: none;
		width: 100%;
		transition: var(--sidebar-transition);
	}

	.account-secondry-menu li a:hover {
		background: var(--sidebar-bg-hover);
		color: var(--sidebar-text-active);
	}

	.account-secondry-menu li a:focus-visible {
		outline: 2px solid var(--sidebar-accent);
		outline-offset: -2px;
	}

	.account-secondry-menu .is-active a {
		background: var(--sidebar-accent);
		color: var(--sidebar-text-active);
	}

	.woocommerce-MyAccount-navigation-link {
		padding: 0;
	}

	/* Menu Badge */
	.menu-badge {
		margin-left: auto;
		background: var(--sidebar-accent);
		color: white;
		font-size: 11px;
		font-weight: 700;
		padding: 2px 6px;
		border-radius: 10px;
		min-width: 18px;
		text-align: center;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MOBILE OVERLAY
	   ═══════════════════════════════════════════════════════════════════════════ */

	.sidebar-overlay {
		display: none;
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.5);
		z-index: 999;
		border: none;
		cursor: pointer;
		backdrop-filter: blur(2px);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE STYLES
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 980px) {
		.dashboard-sidebar {
			width: 70%;
			max-width: 280px;
			bottom: 0;
			left: 0;
			opacity: 0;
			overflow-x: hidden;
			overflow-y: auto;
			position: fixed;
			top: 0;
			transition: var(--sidebar-transition);
			visibility: hidden;
			z-index: 1000;
			transform: translateX(-100%);
		}

		.dashboard-sidebar.is-mobile-open {
			opacity: 1;
			visibility: visible;
			transform: translateX(0);
		}

		.dashboard-sidebar.is-collapsed {
			width: var(--sidebar-width-collapsed);
		}

		.collapse-toggle {
			display: none;
		}

		.sidebar-overlay {
			display: block;
		}

		.account-secondry-menu {
			padding-bottom: 80px; /* Space for mobile nav */
		}
	}

	@media screen and (max-width: 480px) {
		.dashboard-sidebar {
			width: 85%;
			max-width: none;
		}

		.account-primary-menu {
			padding: 20px 15px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   REDUCED MOTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.dashboard-sidebar,
		.collapse-toggle,
		.cstooltiptext,
		.account-primary-menu a,
		.account-secondry-menu li a {
			transition: none;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   HIGH CONTRAST MODE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-contrast: high) {
		.dashboard-sidebar {
			border-right: 2px solid white;
		}

		.account-primary-menu a,
		.account-secondry-menu li a {
			border: 1px solid transparent;
		}

		.account-primary-menu a:focus-visible,
		.account-secondry-menu li a:focus-visible {
			border-color: white;
		}
	}
</style>
