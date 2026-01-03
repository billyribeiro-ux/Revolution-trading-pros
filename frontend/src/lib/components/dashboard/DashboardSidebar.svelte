<!--
	DashboardSidebar - Member Dashboard Navigation
	Pixel-perfect match to WordPress Simpler Trading reference

	Phase Two: Collapsible sidebar with secondary hover panel

	@version 2.2.0 - FIXED: Secondary nav positioning & icon alignment
	@author Revolution Trading Pros

	FIXES APPLIED (ICT 11+ Forensic Investigation):
	1. Secondary nav desktop positioning - removed position:fixed bleed-through
	2. Icon/text alignment in secondary nav - switched to flexbox layout
	3. Removed !important wars causing cascade conflicts
	4. Added proper padding-top alignment for secondary nav

	Svelte 5 Features Used:
	- $props() with interface typing
	- $state() for reactive state
	- $derived() for computed values
	- $effect() for side effects (localStorage)
	- $bindable() for two-way binding with parent
	- browser check for SSR safety
-->
<script lang="ts">
	import { page } from '$app/state';
	import { browser } from '$app/environment';
	import RtpIcon from '$lib/components/icons/RtpIcon.svelte';

	// Types for navigation links
	interface NavLink {
		href: string;
		icon: string;
		text: string;
		bold?: boolean;
		external?: boolean;
	}

	// Types for secondary nav (course-specific navigation)
	interface SecondaryNavSubItem {
		href: string;
		icon: string;
		text: string;
	}

	interface SecondaryNavItem {
		href: string;
		icon: string;
		text: string;
		submenu?: SecondaryNavSubItem[];
	}

	// Membership data from API
	interface MembershipItem {
		name: string;
		slug: string;
		icon?: string;
		type?: 'trading-room' | 'alert-service' | 'course' | 'indicator' | 'weekly-watchlist' | 'premium-report';
	}

	// Svelte 5 props with bindable for parent access
	interface Props {
		user: {
			name: string;
			avatar: string | null;
			memberships?: MembershipItem[];  // Full membership objects, not just slugs
		};
		collapsed?: boolean;
		secondaryNavItems?: SecondaryNavItem[];  // Course-specific secondary nav
		secondarySidebarTitle?: string;  // Title for secondary sidebar
	}

	let { user, collapsed = $bindable(false), secondaryNavItems = [], secondarySidebarTitle = '' }: Props = $props();

	// State for expanded submenus in secondary nav
	let expandedSubmenus = $state<Set<string>>(new Set());

	// Check if secondary nav should be shown
	// WordPress: Secondary nav shows based on route/content, not collapse state
	let showSecondaryNav = $derived(secondaryNavItems.length > 0);

	// Toggle submenu expansion
	function toggleSubmenu(text: string): void {
		const newSet = new Set(expandedSubmenus);
		if (newSet.has(text)) {
			newSet.delete(text);
		} else {
			newSet.add(text);
		}
		expandedSubmenus = newSet;
	}

	// Check if submenu has active item
	function hasActiveSubmenuItem(submenu: SecondaryNavSubItem[] | undefined): boolean {
		if (!submenu) return false;
		return submenu.some(item => isActive(item.href));
	}

	// Svelte 5 state
	let isHovered = $state(false);
	let isMobileMenuOpen = $state(false);

	// Current path for active state - Svelte 5 $app/state (no store subscription needed)
	let currentPath = $derived(page.url.pathname);

	// NOTE: Collapsed state is now controlled by parent (+layout.svelte)
	// Parent uses $effect to auto-collapse on membership dashboard routes
	// DO NOT add $effect here that modifies collapsed - it will conflict with parent

	// Check if a link is active
	function isActive(href: string): boolean {
		if (href === '/dashboard/' || href === '/dashboard') {
			return currentPath === '/dashboard' || currentPath === '/dashboard/';
		}
		return currentPath.startsWith(href);
	}

	// Toggle mobile menu
	function toggleMobileMenu() {
		isMobileMenuOpen = !isMobileMenuOpen;
	}

	// Close mobile menu when clicking overlay
	function closeMobileMenu() {
		isMobileMenuOpen = false;
	}

	// Handle mouse enter/leave for secondary sidebar trigger
	function handleMouseEnter() {
		if (collapsed) {
			isHovered = true;
		}
	}

	function handleMouseLeave() {
		isHovered = false;
	}

	// Navigation items - RTP Product Structure
	// Icons use RTP naming convention (rtp-icon-*)
	const mainLinks: NavLink[] = [
		{ href: '/dashboard/', icon: 'home', text: 'Member Dashboard' },
		{ href: '/dashboard/classes/', icon: 'classes', text: 'My Classes', bold: true },
		{ href: '/dashboard/indicators/', icon: 'activity', text: 'My Indicators', bold: true }
	];

	// Membership links - Dynamically generated from user's actual memberships
	// Separate by type to match WordPress structure
	let tradingRoomLinks = $derived.by(() => {
		const links: NavLink[] = [];
		const memberships = user.memberships ?? [];

		for (const membership of memberships) {
			if (membership.type === 'trading-room' || membership.type === 'alert-service') {
				links.push({
					href: `/dashboard/${membership.slug}/`,
					icon: membership.icon ?? 'trending-up',
					text: membership.name
				});
			}
		}

		return links;
	});

	let mentorshipLinks = $derived.by(() => {
		const links: NavLink[] = [];
		const memberships = user.memberships ?? [];

		for (const membership of memberships) {
			// Show all courses in Mentorship section
			if (membership.type === 'course') {
				links.push({
					href: `/dashboard/${membership.slug}/`,
					icon: membership.icon ?? 'book',
					text: membership.name
				});
			}
		}

		return links;
	});

	let scannerLinks = $derived.by(() => {
		const links: NavLink[] = [];
		const memberships = user.memberships ?? [];

		for (const membership of memberships) {
			// Only show High Octane scanner
			if (membership.type === 'indicator' && membership.slug === 'high-octane-scanner') {
				links.push({
					href: `/dashboard/${membership.slug}/`,
					icon: membership.icon ?? 'chart-candle',
					text: membership.name
				});
			}
		}

		return links;
	});

	const toolsLinks: NavLink[] = [
		{ href: '/dashboard/weekly-watchlist/', icon: 'weekly-watchlist', text: 'Weekly Watchlist' },
		{ href: 'https://support.revolutiontradingpros.com/', icon: 'support', text: 'Support', external: true }
	];

	const accountLinks: NavLink[] = [
		{ href: '/dashboard/account/', icon: 'settings', text: 'My Account' }
	];

	// All navigation sections for secondary sidebar
	let allSections = $derived([
		{ title: 'memberships', links: tradingRoomLinks },
		{ title: 'mentorship', links: mentorshipLinks },
		{ title: 'scanners', links: scannerLinks },
		{ title: 'tools', links: toolsLinks }
	]);
</script>

<!-- Main Sidebar -->
<aside
	class="dashboard__sidebar"
	class:is-collapsed={collapsed}
	class:is-mobile-open={isMobileMenuOpen}
	onmouseenter={handleMouseEnter}
	onmouseleave={handleMouseLeave}
	role="navigation"
	aria-label="Dashboard navigation"
>
	<nav class="dashboard__nav-primary">
		<!-- User Profile Section -->
		<a href="/dashboard/account" class="dashboard__profile-nav-item">
			<span
				class="dashboard__profile-photo"
				style="background-image: url({user.avatar || 'https://secure.gravatar.com/avatar/?s=32&d=mm&r=g'});"
			></span>
			<span class="dashboard__profile-name">{user.name}</span>
		</a>

		<!-- Main Navigation Links -->
		<ul>
			<li></li>
			<ul class="dash_main_links">
				{#each mainLinks as link}
					<li class:is-active={isActive(link.href)}>
						<a href={link.href}>
							<span class="dashboard__nav-item-icon">
								<RtpIcon name={link.icon} size={32} />
							</span>
							<span class="dashboard__nav-item-text" class:font-bold={link.bold}>{link.text}</span>
						</a>
					</li>
				{/each}
			</ul>
		</ul>

		<!-- Memberships Section (Trading Rooms + Alert Services) -->
		{#if tradingRoomLinks.length > 0}
			<ul>
				<li>
					<p class="dashboard__nav-category">memberships</p>
				</li>
				<ul class="dash_main_links">
					{#each tradingRoomLinks as link}
						<li class:is-active={isActive(link.href)}>
							<a href={link.href}>
								<span class="dashboard__nav-item-icon">
									<RtpIcon name={link.icon} size={32} />
								</span>
								<span class="dashboard__nav-item-text">{link.text}</span>
							</a>
						</li>
					{/each}
				</ul>
			</ul>
		{/if}

		<!-- Mentorship Section (Courses) -->
		{#if mentorshipLinks.length > 0}
			<ul>
				<li>
					<p class="dashboard__nav-category">mentorship</p>
				</li>
				<ul class="dash_main_links">
					{#each mentorshipLinks as link}
						<li class:is-active={isActive(link.href)}>
							<a href={link.href}>
								<span class="dashboard__nav-item-icon">
									<RtpIcon name={link.icon} size={32} />
								</span>
								<span class="dashboard__nav-item-text">{link.text}</span>
							</a>
						</li>
					{/each}
				</ul>
			</ul>
		{/if}

		<!-- Scanners Section (High Octane only) -->
		{#if scannerLinks.length > 0}
			<ul>
				<li>
					<p class="dashboard__nav-category">scanners</p>
				</li>
				<ul class="dash_main_links">
					{#each scannerLinks as link}
						<li class:is-active={isActive(link.href)}>
							<a href={link.href}>
								<span class="dashboard__nav-item-icon">
									<RtpIcon name={link.icon} size={32} />
								</span>
								<span class="dashboard__nav-item-text">{link.text}</span>
							</a>
						</li>
					{/each}
				</ul>
			</ul>
		{/if}

		<!-- Tools Section -->
		<ul>
			<li>
				<p class="dashboard__nav-category">tools</p>
			</li>
			<ul class="dash_main_links">
				{#each toolsLinks as link}
					<li class:is-active={isActive(link.href)}>
						<a
							href={link.href}
							target={link.external ? '_blank' : undefined}
							rel={link.external ? 'noopener noreferrer' : undefined}
						>
							<span class="dashboard__nav-item-icon">
								<RtpIcon name={link.icon} size={32} />
							</span>
							<span class="dashboard__nav-item-text">{link.text}</span>
						</a>
					</li>
				{/each}
			</ul>
		</ul>

		<!-- Account Section -->
		<ul>
			<li>
				<p class="dashboard__nav-category">account</p>
			</li>
			<ul class="dash_main_links">
				{#each accountLinks as link}
					<li class:is-active={isActive(link.href)}>
						<a href={link.href}>
							<span class="dashboard__nav-item-icon">
								<RtpIcon name={link.icon} size={32} />
							</span>
							<span class="dashboard__nav-item-text">{link.text}</span>
						</a>
					</li>
				{/each}
			</ul>
		</ul>
	</nav>

	<!-- Secondary Navigation - IMMEDIATELY after primary nav (flex sibling, WordPress structure) -->
	{#if showSecondaryNav}
		<nav class="dashboard__nav-secondary" aria-label="{secondarySidebarTitle} navigation">
			<ul>
				{#each secondaryNavItems as item}
					<li class:has-submenu={item.submenu && item.submenu.length > 0}>
						{#if item.submenu && item.submenu.length > 0}
							<!-- Item with submenu -->
							<button
								type="button"
								class="dashboard__nav-secondary-item"
								class:is-active={hasActiveSubmenuItem(item.submenu)}
								class:is-expanded={expandedSubmenus.has(item.text)}
								onclick={() => toggleSubmenu(item.text)}
								aria-expanded={expandedSubmenus.has(item.text)}
							>
								<span class="dashboard__nav-secondary-icon">
									<RtpIcon name={item.icon} size={24} />
								</span>
								<span class="dashboard__nav-secondary-text">{item.text}</span>
								<span class="dashboard__nav-secondary-chevron">›</span>
							</button>

							{#if expandedSubmenus.has(item.text)}
								<ul class="dashboard__nav-submenu">
									{#each item.submenu as subitem}
										<li class:is-active={isActive(subitem.href)}>
											<a href={subitem.href}>{subitem.text}</a>
										</li>
									{/each}
								</ul>
							{/if}
						{:else}
							<!-- Regular item -->
							<a
								class="dashboard__nav-secondary-item"
								class:is-active={isActive(item.href)}
								href={item.href}
							>
								<span class="dashboard__nav-secondary-icon">
									<RtpIcon name={item.icon} size={24} />
								</span>
								<span class="dashboard__nav-secondary-text">{item.text}</span>
							</a>
						{/if}
					</li>
				{/each}
			</ul>
		</nav>
	{/if}

	<!-- Toggle Footer - Mobile Menu Trigger (matches WordPress reference) -->
	<footer class="dashboard__toggle">
		<button
			class="dashboard__toggle-button"
			onclick={toggleMobileMenu}
			aria-label="Toggle dashboard menu"
			aria-expanded={isMobileMenuOpen}
		>
			<div class="dashboard__toggle-button-icon">
				<span></span>
				<span></span>
				<span></span>
			</div>
			<span class="dashboard__toggle-button-label">Dashboard Menu</span>
		</button>
	</footer>

	<!-- Overlay for mobile -->
	<div
		class="dashboard__overlay"
		class:is-active={isMobileMenuOpen}
		onclick={closeMobileMenu}
		onkeydown={(e) => e.key === 'Escape' && closeMobileMenu()}
		role="button"
		tabindex="-1"
		aria-label="Close menu"
	></div>
</aside>

<!-- Mobile Floating Trigger (visible when sidebar is hidden on mobile) -->
<button
	class="dashboard__mobile-trigger"
	onclick={toggleMobileMenu}
	aria-label="Open dashboard menu"
	aria-expanded={isMobileMenuOpen}
	class:is-hidden={isMobileMenuOpen}
>
	<span class="dashboard__mobile-trigger-icon">
		<span></span>
		<span></span>
		<span></span>
	</span>
</button>

<!-- Mobile Overlay -->
{#if isMobileMenuOpen}
	<div
		class="dashboard__overlay is-active"
		onclick={closeMobileMenu}
		onkeydown={(e) => e.key === 'Escape' && closeMobileMenu()}
		role="button"
		tabindex="0"
		aria-label="Close menu"
	></div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Dashboard Sidebar - Pixel Perfect WordPress Match
	 * Source: dashboard.8f78208b.css from Simpler Trading
	 *
	 * VERSION 2.2.0 - ICT 11+ FIXES APPLIED:
	 * - FIX #1: Secondary nav positioning (removed position:fixed bleed-through)
	 * - FIX #2: Icon/text alignment (flexbox instead of absolute positioning)
	 * - FIX #3: Removed !important cascade conflicts
	 * - FIX #4: Proper padding-top alignment
	 *
	 * LAYOUT PATTERN (matching WordPress exactly):
	 * - Desktop (1280px+): position: static, part of flex layout
	 * - Mobile (<1280px): position: fixed, full height overlay
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SIDEBAR CONTAINER
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__sidebar {
		display: flex;
		flex: 0 0 auto;
		flex-direction: column;
		width: 280px;
		background-color: #0f2d41;
		position: static;
		min-height: 100vh;
		transition: all 0.3s ease-in-out;
	}

	/* Scrollbar Styling */
	.dashboard__sidebar::-webkit-scrollbar {
		width: 6px;
	}

	.dashboard__sidebar::-webkit-scrollbar-track {
		background: rgba(255, 255, 255, 0.05);
	}

	.dashboard__sidebar::-webkit-scrollbar-thumb {
		background: rgba(255, 255, 255, 0.2);
		border-radius: 3px;
	}

	.dashboard__sidebar::-webkit-scrollbar-thumb:hover {
		background: rgba(255, 255, 255, 0.3);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * COLLAPSED STATE (desktop only)
	 * WordPress: Sidebar becomes row flex, primary 80px + secondary 280px
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__sidebar.is-collapsed {
		flex-direction: row;
		flex-wrap: nowrap;
		width: auto;
	}

	.dashboard__sidebar.is-collapsed .dashboard__nav-primary {
		padding-top: 30px;
		flex: 0 0 80px;
		width: 80px;
	}

	.dashboard__sidebar.is-collapsed .dashboard__profile-name,
	.dashboard__sidebar.is-collapsed .dashboard__nav-primary .dashboard__nav-item-text,
	.dashboard__sidebar.is-collapsed .dashboard__nav-category {
		opacity: 0;
		visibility: hidden;
		width: 0;
		overflow: hidden;
		transition: opacity 0.2s ease, visibility 0.2s ease;
	}

	.dashboard__sidebar.is-collapsed .dashboard__profile-nav-item,
	.dashboard__sidebar.is-collapsed .dash_main_links a {
		justify-content: center;
		padding-left: 0;
		padding-right: 0;
	}

	.dashboard__sidebar.is-collapsed .dashboard__nav-item-icon {
		position: relative;
		left: auto;
	}

	.dashboard__sidebar.is-collapsed .dashboard__profile-photo {
		position: relative;
		left: auto;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * PRIMARY NAVIGATION
	 * WordPress CSS: width: 280px (default), width: 80px (collapsed)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-primary {
		flex: 1;
		width: 280px;
		padding-bottom: 30px;
		font-size: 16px;
		background-color: #0f2d41;
		overflow-y: auto;
		overflow-x: hidden;
		transition: width 0.3s ease-in-out;
	}

	.dashboard__nav-primary > ul {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.dashboard__nav-primary > ul > li {
		list-style: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * PROFILE SECTION
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__profile-nav-item {
		display: flex;
		align-items: center;
		position: relative;
		height: auto;
		line-height: 1.4;
		padding: 32px 30px 28px 80px;
		text-decoration: none;
		transition: all 0.15s ease-in-out;
	}

	.dashboard__profile-nav-item:hover .dashboard__profile-name {
		color: #fff;
	}

	.dashboard__profile-photo {
		position: absolute;
		top: 50%;
		left: 30px;
		margin-top: -17px;
		width: 34px;
		height: 34px;
		border-radius: 50%;
		background-size: 32px;
		background-position: center;
		background-repeat: no-repeat;
		flex-shrink: 0;
		border: 2px solid #fff;
		transition: all 0.15s ease-in-out;
	}

	.dashboard__profile-nav-item:hover .dashboard__profile-photo {
		border-color: #0984ae;
	}

	.dashboard__profile-name {
		color: #ffffff;
		font-size: 14px;
		font-weight: 600;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		line-height: 1.4;
		white-space: nowrap;
		transition: opacity 0.3s ease, visibility 0.3s ease;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * CATEGORY HEADERS
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-category {
		padding: 30px 30px 0;
		font-size: 16px;
		font-weight: 700;
		text-transform: uppercase;
		color: #fff;
		margin: 0;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		white-space: nowrap;
		transition: opacity 0.15s ease-in-out, visibility 0.15s ease-in-out;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * NAVIGATION LINKS
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dash_main_links {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.dash_main_links li {
		list-style: none;
		margin: 0;
	}

	.dash_main_links a {
		display: flex;
		align-items: center;
		height: 50px;
		padding: 0 20px 0 80px;
		text-decoration: none;
		color: hsla(0, 0%, 100%, 0.5);
		font-size: 14px;
		font-weight: 300;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		transition: all 0.15s ease-in-out;
		position: relative;
	}

	.dash_main_links a:hover {
		color: #fff;
	}

	.dash_main_links li.is-active a {
		color: #fff;
	}

	.dashboard__nav-primary a::after {
		position: absolute;
		display: block;
		content: '';
		top: 0;
		right: 0;
		bottom: 0;
		width: 5px;
		background: transparent;
		transform: scale(1);
		transition: all 0.15s ease-in-out;
		transform-origin: 100% 50%;
	}

	.dash_main_links li.is-active a::after {
		background-color: #0984ae;
	}

	.dash_main_links li.is-active .dashboard__nav-item-icon {
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * NAVIGATION ICONS (PRIMARY NAV)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-item-icon {
		position: absolute;
		top: 50%;
		left: 30px;
		margin-top: -16px;
		font-size: 32px;
		width: 32px;
		height: 32px;
		line-height: 32px;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		color: hsla(0, 0%, 100%, 0.5);
		transition: all 0.15s ease-in-out;
	}

	.dash_main_links a:hover .dashboard__nav-item-icon {
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * NAVIGATION TEXT (PRIMARY NAV)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-item-text {
		font-size: 14px;
		font-weight: inherit;
		line-height: 1.4;
		color: hsla(0, 0%, 100%, 0.5);
		white-space: nowrap;
		transition: color 0.15s ease-in-out;
	}

	.dashboard__nav-item-text.font-bold {
		font-weight: bold;
		color: hsla(0, 0%, 100%, 0.5);
	}

	.dash_main_links a:hover .dashboard__nav-item-text {
		color: #fff;
	}

	.dash_main_links li.is-active .dashboard__nav-item-text {
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * COLLAPSED STATE - Icon-only mode (80px) - PRIMARY NAV ONLY
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__sidebar.is-collapsed .dashboard__nav-category {
		display: none;
	}

	.dashboard__sidebar.is-collapsed .dashboard__profile-name {
		opacity: 0;
		visibility: hidden;
		width: 0;
	}

	.dashboard__sidebar.is-collapsed .dash_main_links li {
		margin-top: 20px;
	}

	.dashboard__sidebar.is-collapsed .dash_main_links li a {
		padding: 0;
		justify-content: center;
	}

	.dashboard__sidebar.is-collapsed .dashboard__profile-nav-item {
		height: 50px;
		line-height: 50px;
		padding: 0;
		justify-content: center;
	}

	/* Icon centering - PRIMARY NAV ONLY */
	.dashboard__sidebar.is-collapsed .dashboard__nav-primary .dashboard__nav-item-icon,
	.dashboard__sidebar.is-collapsed .dashboard__profile-photo {
		left: 50%;
		margin-left: -16px;
		transform: scale(1);
		transition: all 0.15s ease-in-out;
		position: absolute;
	}

	/* Circular background pseudo-element */
	.dashboard__sidebar.is-collapsed .dash_main_links li a:before {
		position: absolute;
		display: block;
		content: "";
		top: 50%;
		left: 50%;
		width: 50px;
		height: 50px;
		margin-top: -25px;
		margin-left: -25px;
		border-radius: 50%;
		transform: scale(.9);
		background: transparent;
		transition: all 0.15s ease-in-out;
	}

	/* Text label positioning (tooltip) - PRIMARY NAV ONLY */
	.dashboard__sidebar.is-collapsed .dashboard__nav-primary .dashboard__nav-item-text,
	.dashboard__sidebar.is-collapsed .dashboard__profile-name {
		z-index: 100020;
		position: absolute;
		top: 50%;
		left: 100%;
		margin-top: -15px;
		margin-left: -10px;
		height: 30px;
		line-height: 30px;
		padding: 0 12px;
		font-size: 14px;
		font-weight: 600;
		opacity: 0;
		visibility: hidden;
		color: #0984ae;
		background: #fff;
		border-radius: 5px;
		transform: translate(5px);
		transition: all 0.15s ease-in-out;
		white-space: nowrap;
		box-shadow: 0 10px 30px rgba(0,0,0,.15);
		width: auto;
	}

	/* HOVER EFFECTS - Circular background appears */
	.dashboard__sidebar.is-collapsed .dash_main_links li a:hover:before {
		transform: scale(1);
		background-color: rgba(0,0,0,.2);
	}

	/* Active indicator hides on hover */
	.dashboard__sidebar.is-collapsed .dash_main_links li a:hover:after {
		transform: scaleX(0);
	}

	/* Icon scales down on hover */
	.dashboard__sidebar.is-collapsed .dash_main_links li a:hover .dashboard__nav-item-icon,
	.dashboard__sidebar.is-collapsed .dashboard__profile-nav-item:hover .dashboard__profile-photo {
		transform: scale(.9);
	}

	/* White label slides in on hover - PRIMARY NAV ONLY */
	.dashboard__sidebar.is-collapsed .dashboard__nav-primary .dash_main_links li a:hover .dashboard__nav-item-text,
	.dashboard__sidebar.is-collapsed .dashboard__profile-nav-item:hover .dashboard__profile-name {
		opacity: 1;
		visibility: visible;
		transform: translate(0);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SECONDARY NAVIGATION - ICT 11+ FIX APPLIED
	 * 
	 * FIX #1: Desktop uses position:static (flex child), NOT position:fixed
	 * FIX #2: Mobile uses position:fixed as overlay
	 * FIX #3: Items use FLEXBOX for icon/text alignment, NOT absolute positioning
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-secondary {
		font-size: 14px;
		font-weight: 600;
		background-color: #153e59;
		overflow-y: auto;
		overflow-x: hidden;
		transition: all 0.3s ease-in-out;

		/* ═══════════════════════════════════════════════════════════════════════
		 * DESKTOP FIRST (≥1280px) - Static flex child
		 * This is the DEFAULT state on desktop - NOT fixed positioning
		 * ═══════════════════════════════════════════════════════════════════════ */
		display: block;
		position: static;
		width: 280px;
		flex: 0 0 280px;
		min-height: 100vh;
		padding-top: 30px;
		opacity: 1;
		visibility: visible;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * MOBILE/TABLET (<1280px) - Fixed overlay panel
	 * Only apply fixed positioning on smaller screens
	 * ═══════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 1279px) {
		.dashboard__nav-secondary {
			position: fixed;
			bottom: 50px;
			left: 80px;
			top: 0;
			width: 280px;
			min-height: auto;
			opacity: 0;
			visibility: hidden;
			z-index: 100010;
		}

		/* Show on hover when collapsed (mobile/tablet) */
		.dashboard__sidebar.is-collapsed:hover .dashboard__nav-secondary,
		.dashboard__sidebar.is-collapsed:focus-within .dashboard__nav-secondary {
			opacity: 1;
			visibility: visible;
		}
	}

	/* Secondary nav list */
	.dashboard__nav-secondary > ul {
		list-style: none;
		margin: 0;
		padding: 20px 15px;
	}

	@media screen and (min-width: 1440px) {
		.dashboard__nav-secondary > ul {
			padding: 20px;
		}
	}

	.dashboard__nav-secondary > ul > li {
		list-style: none;
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SECONDARY NAV ITEMS - ICT 11+ FIX: FLEXBOX LAYOUT
	 * 
	 * FIX #2: Use display:flex with gap instead of absolute icon positioning
	 * This prevents icon/text overlap issues
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-secondary-item {
		position: relative;
		display: flex;
		align-items: center;
		gap: 12px;
		width: 100%;
		padding: 16px 15px;
		color: hsla(0, 0%, 100%, 0.75);
		border-radius: 5px;
		background-color: transparent;
		text-align: left;
		text-decoration: none;
		border: none;
		cursor: pointer;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 14px;
		font-weight: 600;
		transition: all 0.2s ease;
	}

	@media screen and (min-width: 1440px) {
		.dashboard__nav-secondary-item {
			padding: 18px 20px;
		}
	}

	.dashboard__nav-secondary-item:hover {
		background-color: rgba(0, 0, 0, 0.15);
		color: #fff;
	}

	/* Active state - blue background */
	.dashboard__nav-secondary-item.is-active {
		color: #fff;
		background-color: #0984ae;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SECONDARY NAV ICON - ICT 11+ FIX: FLEX CHILD (not absolute)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-secondary-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		flex-shrink: 0;
		color: hsla(0, 0%, 100%, 0.75);
		transition: color 0.2s ease;
	}

	.dashboard__nav-secondary-item:hover .dashboard__nav-secondary-icon,
	.dashboard__nav-secondary-item.is-active .dashboard__nav-secondary-icon {
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SECONDARY NAV TEXT - ICT 11+ FIX: FLEX CHILD (normal flow)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-secondary-text {
		flex: 1;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		color: inherit;
	}

	/* Chevron for submenus */
	.dashboard__nav-secondary-chevron {
		font-size: 18px;
		transition: transform 0.2s ease;
		margin-left: auto;
	}

	.dashboard__nav-secondary-item.is-expanded .dashboard__nav-secondary-chevron {
		transform: rotate(90deg);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SUBMENU STYLES
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-submenu {
		list-style: none;
		margin: 0;
		padding: 4px 0 8px;
		background-color: rgba(0, 0, 0, 0.15);
	}

	.dashboard__nav-submenu li {
		list-style: none;
		margin: 0;
	}

	.dashboard__nav-submenu a {
		display: block;
		padding: 10px 24px 10px 56px;
		color: rgba(255, 255, 255, 0.65);
		text-decoration: none;
		font-size: 13px;
		font-weight: 400;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		transition: all 0.2s ease;
		position: relative;
	}

	.dashboard__nav-submenu a:hover {
		background-color: rgba(255, 255, 255, 0.05);
		color: rgba(255, 255, 255, 0.9);
	}

	.dashboard__nav-submenu li.is-active a {
		color: #ffffff;
		font-weight: 600;
	}

	.dashboard__nav-submenu li.is-active a::before {
		content: '';
		position: absolute;
		left: 40px;
		top: 50%;
		transform: translateY(-50%);
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background-color: #f7931e;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * TOGGLE FOOTER - Mobile Menu Trigger
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__toggle {
		display: none;
		background-color: #0f2d41;
		padding: 15px 20px;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
	}

	.dashboard__toggle-button {
		display: flex;
		align-items: center;
		gap: 12px;
		width: 100%;
		padding: 10px 15px;
		background-color: transparent;
		border: none;
		cursor: pointer;
		color: rgba(255, 255, 255, 0.7);
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 14px;
		font-weight: 500;
		transition: all 0.3s ease;
		border-radius: 6px;
	}

	.dashboard__toggle-button:hover {
		background-color: rgba(255, 255, 255, 0.1);
		color: #ffffff;
	}

	.dashboard__toggle-button-icon {
		width: 20px;
		height: 14px;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
	}

	.dashboard__toggle-button-icon span {
		display: block;
		width: 100%;
		height: 2px;
		background-color: currentColor;
		transition: all 0.3s ease;
	}

	.dashboard__toggle-button-label {
		font-size: 13px;
		font-weight: 500;
		letter-spacing: 0.02em;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * MOBILE FLOATING TRIGGER
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__mobile-trigger {
		display: none;
		position: fixed;
		top: 20px;
		left: 20px;
		z-index: 101;
		width: 44px;
		height: 44px;
		background-color: #0f2d41;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		align-items: center;
		justify-content: center;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
		transition: opacity 0.3s ease, visibility 0.3s ease;
	}

	.dashboard__mobile-trigger.is-hidden {
		opacity: 0;
		visibility: hidden;
	}

	.dashboard__mobile-trigger-icon {
		width: 24px;
		height: 18px;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
	}

	.dashboard__mobile-trigger-icon span {
		display: block;
		width: 100%;
		height: 2px;
		background-color: #ffffff;
		transition: all 0.3s ease;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * MOBILE OVERLAY
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: rgba(0, 0, 0, 0.5);
		z-index: 99;
		opacity: 0;
		visibility: hidden;
		transition: opacity 0.3s ease, visibility 0.3s ease;
	}

	.dashboard__overlay.is-active {
		opacity: 1;
		visibility: visible;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE - Mobile (<1280px)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 1279px) {
		.dashboard__sidebar {
			position: fixed;
			top: 0;
			left: 0;
			bottom: 50px;
			width: 280px;
			overflow-x: hidden;
			overflow-y: auto;
			opacity: 0;
			visibility: hidden;
			z-index: 100010;
			transform: translateX(-100%);
		}

		.dashboard__sidebar.is-mobile-open {
			opacity: 1;
			visibility: visible;
			transform: translateX(0);
		}

		/* Reset collapsed state on mobile */
		.dashboard__sidebar.is-collapsed {
			width: 280px;
		}

		.dashboard__sidebar.is-collapsed .dashboard__profile-name,
		.dashboard__sidebar.is-collapsed .dashboard__nav-item-text,
		.dashboard__sidebar.is-collapsed .dashboard__nav-category {
			opacity: 1;
			visibility: visible;
			width: auto;
		}

		.dashboard__sidebar.is-collapsed .dashboard__nav-primary {
			padding-top: 0;
		}

		.dashboard__sidebar.is-collapsed .dashboard__profile-nav-item,
		.dashboard__sidebar.is-collapsed .dash_main_links a {
			justify-content: flex-start;
			padding-left: 80px;
			padding-right: 20px;
		}

		.dashboard__sidebar.is-collapsed .dashboard__nav-item-icon,
		.dashboard__sidebar.is-collapsed .dashboard__profile-photo {
			position: absolute;
			left: 30px;
		}

		/* Show toggle footer on mobile */
		.dashboard__toggle {
			display: block;
			position: fixed;
			bottom: 0;
			left: 0;
			right: 0;
			height: 50px;
			line-height: 50px;
			z-index: 100010;
		}

		/* Show mobile floating trigger */
		.dashboard__mobile-trigger {
			display: flex;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE - Desktop (1280px+)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (min-width: 1280px) {
		.dashboard__sidebar {
			position: static;
			height: auto;
			min-height: 100vh;
			opacity: 1;
			visibility: visible;
			overflow: visible;
		}

		.dashboard__toggle {
			display: none;
		}

		.dashboard__mobile-trigger {
			display: none;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE - Desktop Large (1440px+)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (min-width: 1440px) {
		.dash_main_links a {
			padding: 0 40px 0 80px;
		}

		.dashboard__profile-nav-item {
			padding: 32px 40px 28px 80px;
		}

		.dashboard__nav-category {
			padding: 30px 40px 0;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * REDUCED MOTION
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.dashboard__sidebar,
		.dashboard__profile-nav-item,
		.dashboard__profile-photo,
		.dash_main_links a,
		.dashboard__nav-item-icon,
		.dashboard__overlay,
		.dashboard__nav-secondary,
		.dashboard__nav-secondary-item {
			transition: none;
		}

		.dash_main_links li.is-active a::before {
			animation: none;
		}
	}
</style>