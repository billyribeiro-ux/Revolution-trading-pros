<!--
	DashboardSidebar - Member Dashboard Navigation
	Pixel-perfect match to WordPress Simpler Trading reference

	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { page } from '$app/stores';

	// Svelte 5 props
	interface Props {
		user: {
			name: string;
			avatar: string | null;
			memberships?: string[];
		};
	}

	let { user }: Props = $props();

	// Svelte 5 state
	let isCollapsed = $state(false);
	let isMobileMenuOpen = $state(false);

	// Current path for active state
	let currentPath = $derived($page.url.pathname);

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
		// Also toggle collapse state for mobile
		isCollapsed = !isMobileMenuOpen;
	}

	// Close mobile menu when clicking overlay
	function closeMobileMenu() {
		isMobileMenuOpen = false;
	}

	// Navigation items matching WordPress reference
	const mainLinks = [
		{ href: '/dashboard/', icon: 'home', text: 'Member Dashboard' },
		{ href: '/dashboard/classes/', icon: 'courses', text: 'My Classes', bold: true },
		{ href: '/dashboard/indicators/', icon: 'indicators', text: 'My Indicators', bold: true }
	];

	// Membership links - dynamically shown based on user memberships
	const membershipLinks = $derived(() => {
		const links = [];
		if (user.memberships?.includes('mastering_the_trade')) {
			links.push({ href: '/dashboard/mastering-the-trade', icon: 'mastering-the-trade', text: 'Mastering the Trade' });
		}
		if (user.memberships?.includes('simpler_showcase')) {
			links.push({ href: '/dashboard/simpler-showcase/', icon: 'simpler-showcase', text: 'Simpler Showcase' });
		}
		return links;
	});

	const toolsLinks = [
		{ href: '/dashboard/ww/', icon: 'trade-of-the-week', text: 'Weekly Watchlist' },
		{ href: 'https://intercom.help/simpler-trading/en/', icon: 'support', text: 'Support', external: true }
	];

	const accountLinks = [
		{ href: '/dashboard/account/', icon: 'settings', text: 'My Account' }
	];
</script>

<aside class="dashboard__sidebar" class:is-collapsed={isCollapsed}>
	<nav class="dashboard__nav-primary" class:is-collapsed={isCollapsed}>
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
							<span
								class="dashboard__nav-item-text"
								class:font-bold={link.bold}
							>{link.text}</span>
						</a>
					</li>
				{/each}
			</ul>
		</ul>

		<!-- Memberships Section -->
		{#if membershipLinks().length > 0}
			<ul>
				<li>
					<p class="dashboard__nav-category">memberships</p>
				</li>
				<ul class="dash_main_links">
					{#each membershipLinks() as link}
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

	<!-- Toggle Button Footer -->
	<footer class="dashboard__toggle" class:is-collapsed={isCollapsed}>
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

	<!-- Mobile Overlay -->
	{#if isMobileMenuOpen}
		<div
			class="dashboard__overlay"
			onclick={closeMobileMenu}
			onkeydown={(e) => e.key === 'Escape' && closeMobileMenu()}
			role="button"
			tabindex="0"
			aria-label="Close menu"
		></div>
	{/if}
</aside>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Dashboard Sidebar - Pixel Perfect WordPress Match
	 * Simpler Trading Member Dashboard Sidebar Styles
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Sidebar Container - WordPress exact: #0e2433 */
	.dashboard__sidebar {
		position: fixed;
		top: 0;
		left: 0;
		bottom: 0;
		width: 260px;
		background-color: #0e2433;
		z-index: 100;
		display: flex;
		flex-direction: column;
		transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1),
					width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		overflow: hidden;
	}

	.dashboard__sidebar.is-collapsed {
		width: 80px;
	}

	/* Primary Navigation */
	.dashboard__nav-primary {
		flex: 1;
		overflow-y: auto;
		overflow-x: hidden;
		padding: 0;
		scrollbar-width: thin;
		scrollbar-color: rgba(255, 255, 255, 0.2) transparent;
	}

	.dashboard__nav-primary::-webkit-scrollbar {
		width: 6px;
	}

	.dashboard__nav-primary::-webkit-scrollbar-track {
		background: transparent;
	}

	.dashboard__nav-primary::-webkit-scrollbar-thumb {
		background: rgba(255, 255, 255, 0.2);
		border-radius: 3px;
	}

	.dashboard__nav-primary::-webkit-scrollbar-thumb:hover {
		background: rgba(255, 255, 255, 0.3);
	}

	/* Profile Section */
	.dashboard__profile-nav-item {
		display: flex;
		align-items: center;
		padding: 20px 20px 15px;
		text-decoration: none;
		color: #ffffff;
		transition: background-color 0.2s ease;
		border-bottom: 1px solid rgba(255, 255, 255, 0.08);
	}

	.dashboard__profile-nav-item:hover {
		background-color: rgba(255, 255, 255, 0.05);
	}

	.dashboard__profile-photo {
		width: 40px;
		height: 40px;
		border-radius: 50%;
		background-size: cover;
		background-position: center;
		background-color: rgba(255, 255, 255, 0.1);
		flex-shrink: 0;
		margin-right: 12px;
		border: 2px solid rgba(255, 255, 255, 0.15);
		transition: border-color 0.2s ease;
	}

	.dashboard__profile-nav-item:hover .dashboard__profile-photo {
		border-color: rgba(255, 255, 255, 0.3);
	}

	.dashboard__profile-name {
		font-family: 'Open Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
		font-size: 14px;
		font-weight: 600;
		color: #ffffff;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		transition: opacity 0.2s ease;
	}

	.is-collapsed .dashboard__profile-name {
		opacity: 0;
		width: 0;
	}

	/* Navigation Lists */
	.dashboard__nav-primary > ul {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.dashboard__nav-primary > ul > li {
		list-style: none;
	}

	/* Main Links Container */
	.dash_main_links {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.dash_main_links li {
		list-style: none;
		margin: 0;
	}

	.dash_main_links li a {
		display: flex;
		align-items: center;
		padding: 12px 20px;
		color: rgba(255, 255, 255, 0.75);
		text-decoration: none;
		transition: background-color 0.2s ease,
					color 0.2s ease,
					padding-left 0.2s ease;
		position: relative;
	}

	.dash_main_links li a:hover {
		background-color: rgba(255, 255, 255, 0.08);
		color: #ffffff;
	}

	/* Active State - WordPress: #0984ae accent */
	.dash_main_links li.is-active a {
		background-color: rgba(9, 132, 174, 0.15);
		color: #0984ae;
		border-left: 3px solid #0984ae;
		padding-left: 17px;
	}

	.dash_main_links li.is-active .dashboard__nav-item-text {
		color: #0984ae;
	}

	.dash_main_links li.is-active .dashboard__nav-item-icon {
		opacity: 1;
	}

	/* Navigation Icons */
	.dashboard__nav-item-icon {
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		margin-right: 12px;
		font-size: 18px;
		opacity: 0.8;
		transition: opacity 0.2s ease, transform 0.2s ease;
	}

	.dash_main_links li a:hover .dashboard__nav-item-icon {
		opacity: 1;
		transform: translateX(2px);
	}

	/* Icon Classes - Using CSS for icons (can be replaced with actual icon font) */
	.st-icon-home::before { content: '🏠'; }
	.st-icon-courses::before { content: '📚'; }
	.st-icon-indicators::before { content: '📊'; }
	.st-icon-mastering-the-trade::before { content: '📈'; }
	.st-icon-simpler-showcase::before { content: '🎯'; }
	.st-icon-trade-of-the-week::before { content: '📋'; }
	.st-icon-support::before { content: '💬'; }
	.st-icon-settings::before { content: '⚙️'; }

	/* Navigation Text */
	.dashboard__nav-item-text {
		font-family: 'Open Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
		font-size: 14px;
		font-weight: 400;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		transition: opacity 0.2s ease;
	}

	.dashboard__nav-item-text.font-bold {
		font-weight: 700;
		color: #ffffff;
	}

	.is-collapsed .dashboard__nav-item-text {
		opacity: 0;
		width: 0;
	}

	/* Category Labels */
	.dashboard__nav-category {
		font-family: 'Open Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.08em;
		color: rgba(255, 255, 255, 0.45);
		padding: 20px 20px 8px;
		margin: 0;
		transition: opacity 0.2s ease;
	}

	.is-collapsed .dashboard__nav-category {
		opacity: 0;
	}

	/* Toggle Footer */
	.dashboard__toggle {
		padding: 15px;
		border-top: 1px solid rgba(255, 255, 255, 0.08);
		background-color: rgba(0, 0, 0, 0.1);
	}

	.dashboard__toggle-button {
		display: flex;
		align-items: center;
		width: 100%;
		padding: 10px;
		background: transparent;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 6px;
		color: rgba(255, 255, 255, 0.7);
		cursor: pointer;
		transition: background-color 0.2s ease, border-color 0.2s ease;
	}

	.dashboard__toggle-button:hover {
		background-color: rgba(255, 255, 255, 0.05);
		border-color: rgba(255, 255, 255, 0.2);
	}

	.dashboard__toggle-button:focus-visible {
		outline: 2px solid #0984ae;
		outline-offset: 2px;
	}

	/* Hamburger Icon */
	.dashboard__toggle-button-icon {
		width: 20px;
		height: 14px;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		margin-right: 10px;
		flex-shrink: 0;
	}

	.dashboard__toggle-button-icon span {
		display: block;
		width: 100%;
		height: 2px;
		background-color: currentColor;
		border-radius: 1px;
		transition: transform 0.2s ease, opacity 0.2s ease;
	}

	/* Toggle Button Label */
	.dashboard__toggle-button-label {
		font-family: 'Open Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
		font-size: 13px;
		font-weight: 500;
		white-space: nowrap;
		transition: opacity 0.2s ease;
	}

	.is-collapsed .dashboard__toggle-button-label {
		opacity: 0;
		width: 0;
	}

	/* Mobile Overlay */
	.dashboard__overlay {
		display: none;
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: rgba(0, 0, 0, 0.5);
		z-index: 99;
		backdrop-filter: blur(2px);
		-webkit-backdrop-filter: blur(2px);
		animation: fadeIn 0.2s ease;
	}

	@keyframes fadeIn {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Responsive Styles
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Tablet and below */
	@media (max-width: 1024px) {
		.dashboard__sidebar {
			transform: translateX(-100%);
		}

		.dashboard__sidebar:has(.dashboard__overlay) {
			transform: translateX(0);
		}

		.dashboard__overlay {
			display: block;
		}
	}

	/* Mobile */
	@media (max-width: 768px) {
		.dashboard__sidebar {
			width: 280px;
		}

		.dashboard__profile-nav-item {
			padding: 15px;
		}

		.dash_main_links li a {
			padding: 14px 15px;
		}

		.dashboard__nav-category {
			padding: 15px 15px 8px;
		}
	}

	/* Small Mobile */
	@media (max-width: 480px) {
		.dashboard__sidebar {
			width: 100%;
			max-width: 300px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Reduced Motion
	 * ═══════════════════════════════════════════════════════════════════════════ */
	@media (prefers-reduced-motion: reduce) {
		.dashboard__sidebar,
		.dashboard__profile-nav-item,
		.dashboard__profile-photo,
		.dash_main_links li a,
		.dashboard__nav-item-icon,
		.dashboard__toggle-button,
		.dashboard__toggle-button-icon span {
			transition: none;
		}

		.dashboard__overlay {
			animation: none;
		}
	}
</style>
