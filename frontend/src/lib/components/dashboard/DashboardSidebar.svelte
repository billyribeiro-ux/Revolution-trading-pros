<script lang="ts">
	/**
	 * DashboardSidebar - Simpler Trading Style
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Svelte 5 component for the dark blue sidebar navigation:
	 * - Primary nav (#0e2433 background)
	 * - Profile section with avatar
	 * - Category headers
	 * - Navigation links with blue active indicator
	 * - Collapsible state for membership pages
	 * - Secondary nav for membership sub-pages
	 *
	 * @version 1.0.0 - Svelte 5 with $props()
	 */
	import type { Snippet } from 'svelte';
	import { page } from '$app/stores';

	// Types
	interface NavItem {
		label: string;
		href: string;
		icon?: Snippet;
		bold?: boolean;
	}

	interface NavCategory {
		title: string;
		items: NavItem[];
	}

	interface Props {
		/** User name to display */
		userName?: string;
		/** User avatar URL */
		userAvatar?: string;
		/** Navigation categories */
		categories?: NavCategory[];
		/** Whether sidebar is collapsed (for membership pages) */
		isCollapsed?: boolean;
		/** Whether to show secondary nav */
		hasSecondaryNav?: boolean;
		/** Secondary nav content */
		secondaryNav?: Snippet;
		/** Current membership slug (for secondary nav) */
		currentMembershipSlug?: string | null;
	}

	let {
		userName = 'Member',
		userAvatar = 'https://www.gravatar.com/avatar/?d=mp&s=88',
		categories = [],
		isCollapsed = false,
		hasSecondaryNav = false,
		secondaryNav,
		currentMembershipSlug = null
	}: Props = $props();

	// Check if a link is active
	function isActive(href: string): boolean {
		const pathname = $page.url.pathname;
		if (href === '/dashboard/' || href === '/dashboard') {
			return pathname === '/dashboard' || pathname === '/dashboard/';
		}
		return pathname.startsWith(href);
	}
</script>

<aside class="sidebar" class:has-secondary={hasSecondaryNav}>
	<nav class="nav-primary" class:is-collapsed={isCollapsed}>
		<!-- Profile Section -->
		<a href="/dashboard/account/" class="profile-item">
			<span
				class="profile-photo"
				style="background-image: url({userAvatar});"
			></span>
			<span class="profile-name">{userName}</span>
		</a>

		<!-- Navigation Categories -->
		{#each categories as category (category.title)}
			{#if category.title}
				<p class="nav-category">{category.title}</p>
			{/if}
			<ul class="nav-links">
				{#each category.items as item (item.href)}
					<li class:is-active={isActive(item.href)}>
						<a href={item.href}>
							<span class="nav-icon">
								{#if item.icon}
									{@render item.icon()}
								{:else}
									<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
										<circle cx="12" cy="12" r="10"></circle>
									</svg>
								{/if}
							</span>
							<span class="nav-text" class:nav-text--bold={item.bold}>{item.label}</span>
						</a>
					</li>
				{/each}
			</ul>
		{/each}
	</nav>

	<!-- Secondary Navigation (rendered when provided) -->
	{#if secondaryNav && hasSecondaryNav}
		<nav class="nav-secondary">
			{@render secondaryNav()}
		</nav>
	{/if}
</aside>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   SIDEBAR - Exact Simpler Trading #0e2433
	   ═══════════════════════════════════════════════════════════════════════════ */
	.sidebar {
		display: flex;
		flex: 0 0 auto;
		flex-flow: row nowrap;
		width: 280px;
		background-color: #0e2433;
		min-height: 100%;
	}

	.sidebar.has-secondary {
		width: 360px;
		min-width: 360px;
		max-width: 360px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PRIMARY NAV
	   ═══════════════════════════════════════════════════════════════════════════ */
	.nav-primary {
		width: 280px;
		padding-bottom: 30px;
		font-size: 16px;
		line-height: 1;
		background-color: #0e2433;
		transition: width 0.3s ease-in-out;
	}

	.nav-primary.is-collapsed {
		width: 80px;
		padding-top: 30px;
		flex: 0 0 80px;
		min-width: 80px;
		max-width: 80px;
		border-right: 1px solid rgba(255, 255, 255, 0.08);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PROFILE SECTION
	   ═══════════════════════════════════════════════════════════════════════════ */
	.profile-item {
		display: block;
		height: auto;
		line-height: 1.4;
		padding: 32px 20px 28px 80px;
		position: relative;
		text-decoration: none;
		color: hsla(0, 0%, 100%, 0.5);
		transition: all 0.15s ease-in-out;
	}

	.profile-item:hover {
		background: rgba(255, 255, 255, 0.05);
		color: #fff;
	}

	.profile-item:hover .profile-photo {
		border-color: #0984ae;
	}

	.profile-photo {
		position: absolute;
		left: 20px;
		top: 50%;
		transform: translateY(-50%);
		width: 44px;
		height: 44px;
		border-radius: 50%;
		border: 2px solid hsla(0, 0%, 100%, 0.2);
		background-color: #0e2433;
		background-size: cover;
		background-position: center;
		transition: border-color 0.15s ease-in-out;
	}

	.profile-name {
		display: block;
		font-size: 16px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CATEGORY HEADERS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.nav-category {
		color: hsla(0, 0%, 100%, 0.3);
		font-size: 11px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		letter-spacing: 0.5px;
		line-height: 1;
		margin: 0;
		padding: 20px 20px 12px;
		text-transform: uppercase;
	}

	.nav-primary.is-collapsed .nav-category {
		display: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   NAVIGATION LINKS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.nav-links {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.nav-links li {
		position: relative;
	}

	.nav-links a {
		display: flex;
		align-items: center;
		height: 50px;
		padding: 0 20px 0 80px;
		position: relative;
		color: hsla(0, 0%, 100%, 0.5);
		text-decoration: none;
		font-weight: 300;
		transition: all 0.15s ease-in-out;
	}

	.nav-links a:hover {
		background-color: rgba(255, 255, 255, 0.05);
		color: #fff;
	}

	/* Active State - Blue bar on right */
	.nav-links a::after {
		content: "";
		position: absolute;
		top: 0;
		right: 0;
		bottom: 0;
		width: 5px;
		background: transparent;
		transition: background-color 0.15s ease-in-out;
	}

	.nav-links li.is-active a {
		color: #fff;
	}

	.nav-links li.is-active a::after {
		background-color: #0984ae;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   NAV ICONS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.nav-icon {
		position: absolute;
		top: 50%;
		left: 30px;
		margin-top: -14px;
		width: 28px;
		height: 28px;
		font-size: 28px;
		line-height: 28px;
		color: inherit;
	}

	.nav-icon :global(svg) {
		width: 28px;
		height: 28px;
		color: inherit;
		stroke: currentColor;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   NAV TEXT
	   ═══════════════════════════════════════════════════════════════════════════ */
	.nav-text {
		font-size: 17px;
		font-weight: 400;
		font-family: 'Open Sans', sans-serif;
		color: hsla(0, 0%, 100%, 0.7);
		line-height: 1.4;
	}

	.nav-text--bold {
		font-weight: 700;
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   COLLAPSED STATE
	   ═══════════════════════════════════════════════════════════════════════════ */
	.nav-primary.is-collapsed .nav-links li {
		margin-top: 20px;
	}

	.nav-primary.is-collapsed .nav-links a {
		padding: 0;
	}

	/* Circle hover effect */
	.nav-primary.is-collapsed .nav-links a::before {
		content: "";
		position: absolute;
		display: block;
		top: 50%;
		left: 50%;
		width: 50px;
		height: 50px;
		margin-top: -25px;
		margin-left: -25px;
		border-radius: 50%;
		transform: scale(0.9);
		background: transparent;
		transition: all 0.15s ease-in-out;
	}

	.nav-primary.is-collapsed .nav-links a:hover::before {
		transform: scale(1);
		background-color: rgba(0, 0, 0, 0.2);
	}

	/* Center icons in collapsed state */
	.nav-primary.is-collapsed .nav-icon {
		position: absolute;
		left: 50%;
		top: 50%;
		transform: translate(-50%, -50%);
		margin: 0;
	}

	/* Tooltip - hidden by default */
	.nav-primary.is-collapsed .nav-text,
	.nav-primary.is-collapsed .profile-name {
		z-index: 100;
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
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
	}

	/* Show tooltip on hover */
	.nav-primary.is-collapsed .nav-links a:hover .nav-text,
	.nav-primary.is-collapsed .profile-item:hover .profile-name {
		opacity: 1;
		visibility: visible;
		transform: translate(0);
	}

	/* Tooltip arrow */
	.nav-primary.is-collapsed .nav-text::before,
	.nav-primary.is-collapsed .profile-name::before {
		content: '';
		position: absolute;
		left: -6px;
		top: 50%;
		transform: translateY(-50%);
		border-width: 6px;
		border-style: solid;
		border-color: transparent #fff transparent transparent;
	}

	/* Profile in collapsed state */
	.nav-primary.is-collapsed .profile-item {
		height: 50px;
		line-height: 50px;
		padding: 0;
	}

	.nav-primary.is-collapsed .profile-photo {
		position: absolute;
		left: 50%;
		top: 50%;
		transform: translate(-50%, -50%);
		margin: 0;
		width: 32px;
		height: 32px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SECONDARY NAVIGATION
	   ═══════════════════════════════════════════════════════════════════════════ */
	.nav-secondary {
		width: 280px;
		flex: 0 0 280px;
		min-width: 280px;
		max-width: 280px;
		font-size: 14px;
		font-weight: 600;
		background-color: #0e2433;
		min-height: 100%;
		padding: 20px;
	}

	.nav-secondary :global(ul) {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.nav-secondary :global(li + li) {
		margin-top: 10px;
	}

	.nav-secondary :global(a) {
		display: flex;
		align-items: center;
		padding: 12px 20px;
		color: hsla(0, 0%, 100%, 0.7);
		text-decoration: none;
		border-radius: 5px;
		background-color: transparent;
		transition: all 0.15s ease-in-out;
	}

	.nav-secondary :global(a:hover) {
		color: #fff;
		background-color: rgba(255, 255, 255, 0.05);
	}

	.nav-secondary :global(li.is-active a) {
		color: #fff;
		background-color: rgba(255, 255, 255, 0.1);
	}

	.nav-secondary :global(.nav-icon) {
		display: inline-block;
		width: 24px;
		height: 24px;
		margin-right: 10px;
		color: #0984ae;
	}

	.nav-secondary :global(.nav-icon svg) {
		width: 24px;
		height: 24px;
		color: #0984ae;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - Mobile
	   ═══════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 1279px) {
		.sidebar {
			position: fixed;
			top: 0;
			left: 0;
			bottom: 50px;
			z-index: 100010;
			opacity: 0;
			visibility: hidden;
			transform: translateX(-100%);
			transition: all 0.3s ease-in-out;
		}

		:global(.html--dashboard-menu-open) .sidebar {
			opacity: 1;
			visibility: visible;
			transform: translateX(0);
		}
	}
</style>
