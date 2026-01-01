<!--
	DashboardSidebar - Member Dashboard Navigation
	Pixel-perfect match to WordPress Simpler Trading reference

	Phase Two: Collapsible sidebar with secondary hover panel

	@version 2.1.0 - RtpIcon Integration
	@author Revolution Trading Pros

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

	// Svelte 5 props with bindable for parent access
	interface Props {
		user: {
			name: string;
			avatar: string | null;
			memberships?: string[];
		};
		collapsed?: boolean;
	}

	let { user, collapsed = $bindable(false) }: Props = $props();

	// Svelte 5 state
	let isHovered = $state(false);
	let isMobileMenuOpen = $state(false);

	// Current path for active state - Svelte 5 $app/state (no store subscription needed)
	let currentPath = $derived(page.url.pathname);

	// Route-based collapse - sidebar collapses when navigating to sub-pages
	// Full sidebar (280px) on /dashboard/ home, collapsed (80px) on sub-pages
	$effect(() => {
		if (browser) {
			const isDashboardHome = currentPath === '/dashboard' || currentPath === '/dashboard/';
			collapsed = !isDashboardHome;
		}
	});

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
		{ href: '/dashboard/indicators/', icon: 'indicators', text: 'My Indicators', bold: true }
	];

	// Membership links - Live Trading Rooms (dynamically shown based on user memberships)
	let membershipLinks = $derived.by(() => {
		const links: NavLink[] = [];
		if (user.memberships?.includes('options_day_trading_room')) {
			links.push({ href: '/dashboard/options-day-trading/', icon: 'chart-candle', text: 'Options Day Trading Room' });
		}
		if (user.memberships?.includes('swing_trading_room')) {
			links.push({ href: '/dashboard/swing-trading/', icon: 'trending-up', text: 'Swing Trading Room' });
		}
		return links;
	});

	// Mastery links - Mentorship Programs
	let masteryLinks = $derived.by(() => {
		const links: NavLink[] = [];
		if (user.memberships?.includes('small_accounts_mentorship')) {
			links.push({ href: '/dashboard/small-accounts-mentorship/', icon: 'school', text: 'Small Accounts Mentorship' });
		}
		return links;
	});

	// Scanner links - Trading Scanners (NEW)
	let scannerLinks = $derived.by(() => {
		const links: NavLink[] = [];
		if (user.memberships?.includes('high_octane_scanner')) {
			links.push({ href: '/dashboard/high-octane-scanner/', icon: 'bolt', text: 'High Octane Scanner' });
		}
		if (user.memberships?.includes('h2_scanner')) {
			links.push({ href: '/dashboard/h2-scanner/', icon: 'search', text: 'H2 Scanner' });
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
		{ title: 'memberships', links: membershipLinks },
		{ title: 'mastery', links: masteryLinks },
		{ title: 'scanner', links: scannerLinks },
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

		<!-- Memberships Section -->
		{#if membershipLinks.length > 0}
			<ul>
				<li>
					<p class="dashboard__nav-category">memberships</p>
				</li>
				<ul class="dash_main_links">
					{#each membershipLinks as link}
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

		<!-- Mastery Section -->
		{#if masteryLinks.length > 0}
			<ul>
				<li>
					<p class="dashboard__nav-category">mastery</p>
				</li>
				<ul class="dash_main_links">
					{#each masteryLinks as link}
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

		<!-- Scanner Section -->
		{#if scannerLinks.length > 0}
			<ul>
				<li>
					<p class="dashboard__nav-category">scanner</p>
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
</aside>

<!-- Secondary Sidebar - Shows on hover when collapsed -->
{#if collapsed && isHovered}
	<aside
		class="dashboard__sidebar-secondary"
		onmouseenter={() => isHovered = true}
		onmouseleave={() => isHovered = false}
		role="navigation"
		aria-label="Quick access navigation"
	>
		<div class="dashboard__sidebar-secondary-content">
			<h3 class="dashboard__sidebar-secondary-title">Quick Access</h3>

			{#each allSections as section}
				{#if section.links.length > 0}
					<div class="dashboard__sidebar-secondary-section">
						<p class="dashboard__nav-category">{section.title}</p>
						<ul class="dash_secondary_links">
							{#each section.links as link}
								<li>
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
					</div>
				{/if}
			{/each}
		</div>
	</aside>
{/if}

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
	 * Phase Two: Collapsible with Secondary Sidebar
	 * Source: dashboard.8f78208b.css from Simpler Trading
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* ═══════════════════════════════════════════════════════════════════════════
	 * MAIN SIDEBAR - Collapsible (280px → 80px)
	 * WordPress Reference: Full viewport height, no NavBar in dashboard area
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__sidebar {
		background-color: #0f2d41;
		width: 280px;
		height: 100vh;
		position: fixed;
		top: 0;
		left: 0;
		overflow-y: auto;
		overflow-x: hidden;
		z-index: 100;
		display: flex;
		flex: 0 0 auto;
		flex-direction: column;
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
	 * COLLAPSED STATE
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__sidebar.is-collapsed {
		width: 80px;
	}

	.dashboard__sidebar.is-collapsed .dashboard__nav-primary {
		padding-top: 30px;
	}

	.dashboard__sidebar.is-collapsed .dashboard__profile-name,
	.dashboard__sidebar.is-collapsed .dashboard__nav-item-text,
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
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-primary {
		flex: 1;
		overflow-y: auto;
		overflow-x: hidden;
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
		color: #0984ae;
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
		background-color: rgba(255, 255, 255, 0.05);
	}

	/* Active State - Light blue left border indicator */
	.dash_main_links li.is-active a {
		color: #fff;
		background-color: rgba(255, 255, 255, 0.08);
	}

	.dash_main_links li.is-active a::after {
		content: '';
		position: absolute;
		left: 0;
		top: 0;
		bottom: 0;
		width: 4px;
		background-color: #0984ae;
	}

	.dash_main_links li.is-active .dashboard__nav-item-icon {
		color: #fff;
	}

	.dash_main_links li.is-active a:hover {
		background-color: rgba(255, 255, 255, 0.1);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * NAVIGATION ICONS
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

	/* Icons are now rendered via RtpIcon component using Tabler Icons */

	/* ═══════════════════════════════════════════════════════════════════════════
	 * NAVIGATION TEXT
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-item-text {
		font-size: 14px;
		font-weight: inherit; /* Inherits 300 from parent .dash_main_links a */
		line-height: 1.4;
		color: inherit;
		white-space: nowrap;
		transition: opacity 0.3s ease, visibility 0.3s ease;
	}

	.dashboard__nav-item-text.font-bold {
		font-weight: bold;
		color: white;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SECONDARY SIDEBAR - Expanded Detail View
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__sidebar-secondary {
		background-color: #0f2d41;
		width: 280px;
		height: 100vh;
		position: fixed;
		top: 0;
		left: 80px;
		overflow-y: auto;
		overflow-x: hidden;
		z-index: 99;
		box-shadow: 2px 0 10px rgba(0, 0, 0, 0.3);
		animation: slideInSecondary 0.3s ease;
	}

	@keyframes slideInSecondary {
		from {
			opacity: 0;
			transform: translateX(-20px);
		}
		to {
			opacity: 1;
			transform: translateX(0);
		}
	}

	.dashboard__sidebar-secondary-content {
		padding: 30px 20px;
	}

	.dashboard__sidebar-secondary-title {
		color: #ffffff;
		font-size: 18px;
		font-weight: 700;
		margin-bottom: 20px;
		padding: 0 10px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.dashboard__sidebar-secondary-section {
		margin-bottom: 30px;
	}

	.dashboard__sidebar-secondary .dashboard__nav-category {
		padding: 10px 10px 8px;
	}

	.dash_secondary_links {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.dash_secondary_links li {
		list-style: none;
		margin: 0;
	}

	.dash_secondary_links a {
		display: flex;
		align-items: center;
		padding: 12px 20px;
		text-decoration: none;
		color: rgba(255, 255, 255, 0.5);
		font-size: 14px;
		font-weight: 300;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		transition: all 0.2s ease;
		border-radius: 6px;
	}

	.dash_secondary_links a:hover {
		background-color: rgba(255, 255, 255, 0.1);
		color: #ffffff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * TOGGLE FOOTER - Mobile Menu Trigger (WordPress Reference Match)
	 * Background: #0d2532
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__toggle {
		display: none; /* Hidden on desktop, shown on mobile */
		background-color: #0d2532;
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
	 * MOBILE FLOATING TRIGGER - Opens sidebar when hidden
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__mobile-trigger {
		display: none; /* Hidden on desktop, shown on mobile */
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
	 * RESPONSIVE - Mobile (< 1080px) - Sidebar hidden by default
	 * Reference: dashboard.8f78208b.css breakpoints
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 1079px) {
		.dashboard__sidebar {
			transform: translateX(-100%);
			width: 280px;
		}

		.dashboard__sidebar.is-mobile-open {
			transform: translateX(0);
		}

		.dashboard__sidebar.is-collapsed {
			width: 280px;
		}

		/* Show mobile floating trigger */
		.dashboard__mobile-trigger {
			display: flex;
		}

		/* Show toggle footer inside sidebar on mobile */
		.dashboard__toggle {
			display: block;
		}

		.dashboard__sidebar-secondary {
			display: none;
		}

		/* Reset collapsed styles on mobile */
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
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE - Desktop Small (1080px - 1279px)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (min-width: 1080px) and (max-width: 1279px) {
		.dashboard__sidebar {
			width: 260px;
		}

		.dashboard__nav-item-text {
			font-size: 13px;
		}

		.dashboard__nav-category {
			font-size: 14px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE - Desktop Large (1440px+) - Increased padding
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
		.dashboard__sidebar-secondary {
			transition: none;
		}

		.dash_main_links li.is-active a::before {
			animation: none;
		}

		.dashboard__sidebar-secondary {
			animation: none;
		}
	}
</style>
