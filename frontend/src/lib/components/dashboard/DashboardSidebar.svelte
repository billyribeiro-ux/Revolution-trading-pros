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
	 * Source: dashboard.8f78208b.css from Simpler Trading
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Sidebar Container - WordPress exact: #0a2335 */
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
		transition: transform 0.3s ease-in-out;
	}

	/* Scrollbar Styling - on sidebar container per WordPress */
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

	.dashboard__sidebar.is-collapsed {
		width: 80px;
	}

	/* Primary Navigation */
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
	 * Profile Section - WordPress exact specs
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__profile-nav-item {
		display: flex;
		align-items: center;
		padding: 20px 30px;
		text-decoration: none;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		transition: background-color 0.2s ease;
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
	}

	.dashboard__profile-name {
		color: #ffffff;
		font-size: 14px;
		font-weight: 600;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.4;
	}

	.is-collapsed .dashboard__profile-name {
		opacity: 0;
		width: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Category Headers - WordPress exact specs
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
	}

	.is-collapsed .dashboard__nav-category {
		opacity: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Navigation Links - WordPress exact specs
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
		transition: all 0.2s ease;
		position: relative;
	}

	/* Hover background effect layer */
	.dash_main_links a::after {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: rgba(255, 255, 255, 0);
		transition: background-color 0.2s ease;
		pointer-events: none;
	}

	.dash_main_links a:hover::after {
		background-color: rgba(255, 255, 255, 0.1);
	}

	.dash_main_links a:hover {
		background-color: rgba(255, 255, 255, 0.1);
		color: #ffffff;
	}

	/* Active State - WordPress: white text with cyan border */
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
	 * Navigation Icons - WordPress exact specs
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
		transition: color 0.2s ease;
	}

	.dash_main_links a:hover .dashboard__nav-item-icon {
		color: #ffffff;
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

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Navigation Text - WordPress exact specs
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-item-text {
		font-size: 14px;
		font-weight: 400;
		line-height: 1.4;
		color: inherit;
		transition: color 0.2s ease;
	}

	.dashboard__nav-item-text.font-bold {
		font-weight: bold;
		color: white;
	}

	.is-collapsed .dashboard__nav-item-text {
		opacity: 0;
		width: 0;
	}

	/* Collapsed State Text Color */
	.dashboard__nav-primary.is-collapsed .dash_main_links .dashboard__nav-item-text {
		color: #0984ae !important;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Toggle Button (Mobile) - WordPress exact specs
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__toggle {
		padding: 20px 30px;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
		background-color: #0a2335;
	}

	.dashboard__toggle-button {
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

	.dashboard__toggle-button:hover {
		color: #ffffff;
	}

	.dashboard__toggle-button:focus-visible {
		outline: 2px solid #0984ae;
		outline-offset: 2px;
	}

	/* Hamburger Icon - WordPress exact specs */
	.dashboard__toggle-button-icon {
		width: 24px;
		height: 18px;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		margin-right: 12px;
	}

	.dashboard__toggle-button-icon span {
		display: block;
		width: 100%;
		height: 2px;
		background-color: currentColor;
		transition: all 0.3s ease;
	}

	/* Toggle Button Label */
	.dashboard__toggle-button-label {
		font-size: 14px;
		font-weight: 600;
	}

	.is-collapsed .dashboard__toggle-button-label {
		opacity: 0;
		width: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Mobile Overlay - WordPress exact specs
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: rgba(0, 0, 0, 0.5);
		z-index: 99;
		display: none;
		opacity: 0;
		transition: opacity 0.3s ease;
	}

	.dashboard__overlay.is-active {
		display: block;
		opacity: 1;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Responsive - Mobile (≤768px)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 768px) {
		.dashboard__sidebar {
			transform: translateX(-100%);
		}

		.dashboard__sidebar.is-active {
			transform: translateX(0);
		}

		.dashboard__overlay {
			display: block;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Responsive - Tablet (769-1024px)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (min-width: 769px) and (max-width: 1024px) {
		.dashboard__sidebar {
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
	 * Reduced Motion
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.dashboard__sidebar,
		.dashboard__profile-nav-item,
		.dashboard__profile-photo,
		.dash_main_links a,
		.dashboard__nav-item-icon,
		.dashboard__toggle-button,
		.dashboard__toggle-button-icon span,
		.dashboard__overlay {
			transition: none;
		}

		.dash_main_links li.is-active a::before {
			animation: none;
		}
	}
</style>
