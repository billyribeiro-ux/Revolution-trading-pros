<!--
	DashboardSidebar - Member Dashboard Navigation
	Pixel-perfect match to WordPress Simpler Trading reference

	Phase Two: Collapsible sidebar with secondary hover panel

	@version 2.0.0
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
	import { page } from '$app/stores';
	import { browser } from '$app/environment';

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

	// Current path for active state - SSR safe
	let currentPath = $derived($page.url.pathname);

	// Persist collapsed state to localStorage - SSR safe with $effect
	$effect(() => {
		if (browser) {
			// Read initial state from localStorage on mount
			const saved = localStorage.getItem('dashboard_sidebar_collapsed');
			if (saved !== null) {
				collapsed = saved === 'true';
			}
		}
	});

	// Save collapsed state when it changes
	$effect(() => {
		if (browser) {
			localStorage.setItem('dashboard_sidebar_collapsed', String(collapsed));
		}
	});

	// Check if a link is active
	function isActive(href: string): boolean {
		if (href === '/dashboard/' || href === '/dashboard') {
			return currentPath === '/dashboard' || currentPath === '/dashboard/';
		}
		return currentPath.startsWith(href);
	}

	// Toggle collapse state
	function toggleCollapse() {
		collapsed = !collapsed;
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

	// Navigation items matching WordPress reference
	const mainLinks: NavLink[] = [
		{ href: '/dashboard/', icon: 'home', text: 'Member Dashboard' },
		{ href: '/dashboard/classes/', icon: 'courses', text: 'My Classes', bold: true },
		{ href: '/dashboard/indicators/', icon: 'indicators', text: 'My Indicators', bold: true }
	];

	// Membership links - dynamically shown based on user memberships
	let membershipLinks = $derived.by(() => {
		const links: NavLink[] = [];
		if (user.memberships?.includes('mastering_the_trade')) {
			links.push({ href: '/dashboard/mastering-the-trade', icon: 'mastering-the-trade', text: 'Mastering the Trade' });
		}
		if (user.memberships?.includes('simpler_showcase')) {
			links.push({ href: '/dashboard/simpler-showcase/', icon: 'simpler-showcase', text: 'Simpler Showcase' });
		}
		if (user.memberships?.includes('tr3ndy_spx_alerts')) {
			links.push({ href: '/dashboard/tr3ndy-spx-alerts/', icon: 'tr3ndy-spx-alerts-circle', text: 'Tr3ndy SPX Alerts Service' });
		}
		return links;
	});

	// Mastery links
	let masteryLinks = $derived.by(() => {
		const links: NavLink[] = [];
		if (user.memberships?.includes('compounding_growth_mastery')) {
			links.push({ href: '/dashboard/cgm/', icon: 'consistent-growth', text: 'Compounding Growth Mastery' });
		}
		return links;
	});

	const toolsLinks: NavLink[] = [
		{ href: '/dashboard/ww/', icon: 'trade-of-the-week', text: 'Weekly Watchlist' },
		{ href: 'https://intercom.help/simpler-trading/en/', icon: 'support', text: 'Support', external: true }
	];

	const accountLinks: NavLink[] = [
		{ href: '/dashboard/account/', icon: 'settings', text: 'My Account' }
	];

	// All navigation sections for secondary sidebar
	let allSections = $derived([
		{ title: 'memberships', links: membershipLinks },
		{ title: 'mastery', links: masteryLinks },
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
							<span class="dashboard__nav-item-icon st-icon-{link.icon}"></span>
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
								<span class="dashboard__nav-item-icon st-icon-{link.icon}"></span>
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
								<span class="dashboard__nav-item-icon st-icon-{link.icon}"></span>
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
							<span class="dashboard__nav-item-icon st-icon-{link.icon}"></span>
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
							<span class="dashboard__nav-item-icon st-icon-{link.icon}"></span>
							<span class="dashboard__nav-item-text">{link.text}</span>
						</a>
					</li>
				{/each}
			</ul>
		</ul>
	</nav>

	<!-- Collapse Toggle Button -->
	<footer class="dashboard__toggle">
		<button
			class="dashboard__collapse-button"
			onclick={toggleCollapse}
			aria-label={collapsed ? 'Expand sidebar' : 'Collapse sidebar'}
			aria-expanded={!collapsed}
		>
			<div class="dashboard__collapse-icon" class:is-collapsed={collapsed}>
				<span class="collapse-arrow">&#9664;</span>
			</div>
			<span class="dashboard__collapse-label">Collapse</span>
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
										<span class="dashboard__nav-item-icon st-icon-{link.icon}"></span>
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

<!-- Mobile Menu Toggle (visible on mobile only) -->
<button
	class="dashboard__mobile-toggle"
	onclick={toggleMobileMenu}
	aria-label="Toggle mobile menu"
	aria-expanded={isMobileMenuOpen}
>
	<span class="dashboard__mobile-toggle-icon">
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
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__sidebar {
		background-color: #0a2335;
		width: 280px;
		height: 100vh;
		position: fixed;
		top: 0;
		left: 0;
		overflow-y: auto;
		overflow-x: hidden;
		z-index: 100;
		display: flex;
		flex-direction: column;
		transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
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

	.dashboard__sidebar.is-collapsed .dashboard__profile-name,
	.dashboard__sidebar.is-collapsed .dashboard__nav-item-text,
	.dashboard__sidebar.is-collapsed .dashboard__nav-category,
	.dashboard__sidebar.is-collapsed .dashboard__collapse-label {
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
		margin-right: 0;
	}

	.dashboard__sidebar.is-collapsed .dashboard__profile-photo {
		margin-right: 0;
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
		padding: 20px 30px;
		text-decoration: none;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.dashboard__profile-nav-item:hover {
		background-color: rgba(255, 255, 255, 0.05);
	}

	.dashboard__profile-photo {
		width: 32px;
		height: 32px;
		border-radius: 50%;
		background-size: cover;
		background-position: center;
		margin-right: 12px;
		flex-shrink: 0;
		border: 2px solid rgba(255, 255, 255, 0.15);
		transition: all 0.3s ease;
	}

	.dashboard__profile-nav-item:hover .dashboard__profile-photo {
		border-color: rgba(255, 255, 255, 0.3);
	}

	.dashboard__profile-name {
		color: #ffffff;
		font-size: 14px;
		font-weight: 600;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.4;
		white-space: nowrap;
		transition: opacity 0.3s ease, visibility 0.3s ease;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * CATEGORY HEADERS
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-category {
		padding: 20px 30px 10px;
		font-size: 10px;
		font-weight: 700;
		letter-spacing: 0.2em;
		text-transform: uppercase;
		color: rgba(255, 255, 255, 0.5);
		margin: 0;
		font-family: 'Open Sans', sans-serif;
		white-space: nowrap;
		transition: opacity 0.3s ease, visibility 0.3s ease;
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
		padding: 12px 30px;
		text-decoration: none;
		color: rgba(255, 255, 255, 0.7);
		font-size: 14px;
		font-weight: 400;
		font-family: 'Open Sans', sans-serif;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		position: relative;
	}

	.dash_main_links a:hover {
		background-color: rgba(255, 255, 255, 0.1);
		color: #ffffff;
	}

	/* Active State */
	.dash_main_links li.is-active a {
		background-color: rgba(255, 255, 255, 0.15);
		color: #ffffff;
		font-weight: 600;
	}

	.dash_main_links li.is-active a::before {
		content: '';
		position: absolute;
		left: 0;
		top: 0;
		bottom: 0;
		width: 4px;
		background-color: #0984ae;
		animation: slideIn 0.3s ease-out;
	}

	@keyframes slideIn {
		from {
			transform: scaleY(0);
			opacity: 0;
		}
		to {
			transform: scaleY(1);
			opacity: 1;
		}
	}

	.dash_main_links li.is-active .dashboard__nav-item-icon {
		color: #ffffff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * NAVIGATION ICONS
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-item-icon {
		font-size: 20px;
		width: 24px;
		height: 24px;
		margin-right: 12px;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		color: rgba(255, 255, 255, 0.7);
		transition: all 0.3s ease;
	}

	.dash_main_links a:hover .dashboard__nav-item-icon {
		color: #ffffff;
	}

	/* Icon Classes */
	.st-icon-home::before { content: '🏠'; }
	.st-icon-courses::before { content: '📚'; }
	.st-icon-indicators::before { content: '📊'; }
	.st-icon-mastering-the-trade::before { content: '📈'; }
	.st-icon-simpler-showcase::before { content: '🎯'; }
	.st-icon-tr3ndy-spx-alerts-circle::before { content: '⚡'; }
	.st-icon-consistent-growth::before { content: '📉'; }
	.st-icon-trade-of-the-week::before { content: '📋'; }
	.st-icon-support::before { content: '💬'; }
	.st-icon-settings::before { content: '⚙️'; }

	/* ═══════════════════════════════════════════════════════════════════════════
	 * NAVIGATION TEXT
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-item-text {
		font-size: 14px;
		font-weight: 400;
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
	 * COLLAPSE BUTTON
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__toggle {
		padding: 20px 30px;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
		background-color: #0a2335;
	}

	.dashboard__collapse-button {
		display: flex;
		align-items: center;
		width: 100%;
		padding: 12px 0;
		background: none;
		border: none;
		color: rgba(255, 255, 255, 0.7);
		font-size: 14px;
		font-weight: 600;
		font-family: 'Open Sans', sans-serif;
		cursor: pointer;
		transition: color 0.2s ease;
	}

	.dashboard__collapse-button:hover {
		color: #ffffff;
	}

	.dashboard__collapse-button:focus-visible {
		outline: 2px solid #0984ae;
		outline-offset: 2px;
	}

	.dashboard__collapse-icon {
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-right: 12px;
		transition: transform 0.3s ease;
	}

	.dashboard__collapse-icon.is-collapsed {
		transform: rotate(180deg);
	}

	.collapse-arrow {
		font-size: 16px;
		line-height: 1;
	}

	.dashboard__collapse-label {
		font-size: 14px;
		font-weight: 600;
		white-space: nowrap;
		transition: opacity 0.3s ease, visibility 0.3s ease;
	}

	/* Collapsed state adjustments */
	.dashboard__sidebar.is-collapsed .dashboard__toggle {
		padding: 20px 0;
		display: flex;
		justify-content: center;
	}

	.dashboard__sidebar.is-collapsed .dashboard__collapse-button {
		justify-content: center;
		width: auto;
	}

	.dashboard__sidebar.is-collapsed .dashboard__collapse-icon {
		margin-right: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SECONDARY SIDEBAR - Expanded Detail View
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__sidebar-secondary {
		background-color: #0d2d45;
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
		font-family: 'Open Sans', sans-serif;
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
		color: rgba(255, 255, 255, 0.7);
		font-size: 14px;
		font-weight: 400;
		font-family: 'Open Sans', sans-serif;
		transition: all 0.2s ease;
		border-radius: 6px;
	}

	.dash_secondary_links a:hover {
		background-color: rgba(255, 255, 255, 0.1);
		color: #ffffff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * MOBILE TOGGLE BUTTON
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__mobile-toggle {
		display: none;
		position: fixed;
		top: 20px;
		left: 20px;
		z-index: 101;
		width: 44px;
		height: 44px;
		background-color: #0a2335;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		align-items: center;
		justify-content: center;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
	}

	.dashboard__mobile-toggle-icon {
		width: 24px;
		height: 18px;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
	}

	.dashboard__mobile-toggle-icon span {
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
	 * RESPONSIVE - Mobile (≤768px)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 768px) {
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

		.dashboard__mobile-toggle {
			display: flex;
		}

		.dashboard__sidebar-secondary {
			display: none;
		}

		/* Reset collapsed styles on mobile */
		.dashboard__sidebar.is-collapsed .dashboard__profile-name,
		.dashboard__sidebar.is-collapsed .dashboard__nav-item-text,
		.dashboard__sidebar.is-collapsed .dashboard__nav-category,
		.dashboard__sidebar.is-collapsed .dashboard__collapse-label {
			opacity: 1;
			visibility: visible;
			width: auto;
		}

		.dashboard__sidebar.is-collapsed .dashboard__profile-nav-item,
		.dashboard__sidebar.is-collapsed .dash_main_links a {
			justify-content: flex-start;
			padding-left: 30px;
			padding-right: 30px;
		}

		.dashboard__sidebar.is-collapsed .dashboard__nav-item-icon,
		.dashboard__sidebar.is-collapsed .dashboard__profile-photo {
			margin-right: 12px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE - Tablet (769-1024px)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (min-width: 769px) and (max-width: 1024px) {
		.dashboard__sidebar:not(.is-collapsed) {
			width: 240px;
		}

		.dashboard__nav-item-text {
			font-size: 13px;
		}

		.dashboard__nav-category {
			font-size: 9px;
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
		.dashboard__collapse-button,
		.dashboard__collapse-icon,
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
