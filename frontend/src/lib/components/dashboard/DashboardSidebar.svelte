<script lang="ts">
	/**
	 * DashboardSidebar - Primary navigation sidebar
	 *
	 * Svelte 5 Runes:
	 * - $props() for component props
	 * - $state() for local state
	 * - $derived() for computed values
	 *
	 * Features:
	 * - Collapsible sidebar
	 * - Mobile responsive with overlay
	 * - Active route highlighting
	 * - User profile section
	 * - Dynamic membership navigation
	 */

	import { page } from '$app/stores';
	import IconHome from '@tabler/icons-svelte/icons/home';
	import IconSchool from '@tabler/icons-svelte/icons/school';
	import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
	import IconSettings from '@tabler/icons-svelte/icons/settings';
	import IconHelp from '@tabler/icons-svelte/icons/help';
	import IconMenu2 from '@tabler/icons-svelte/icons/menu-2';
	import IconX from '@tabler/icons-svelte/icons/x';

	// Props with Svelte 5 $props()
	interface Props {
		user: {
			name: string;
			email: string;
			avatar?: string;
		};
		memberships?: Array<{
			slug: string;
			name: string;
			icon?: string;
		}>;
		isCollapsed?: boolean;
		isOpen?: boolean;
		onToggleCollapse?: () => void;
		onToggleOpen?: () => void;
	}

	let {
		user,
		memberships = [],
		isCollapsed = false,
		isOpen = false,
		onToggleCollapse,
		onToggleOpen
	}: Props = $props();

	// Derived state for current path
	let currentPath = $derived($page.url.pathname);

	// Check if a path is active
	function isActive(href: string): boolean {
		if (href === '/dashboard' || href === '/dashboard/') {
			return currentPath === '/dashboard' || currentPath === '/dashboard/';
		}
		return currentPath.startsWith(href);
	}

	// Navigation items - Main links
	const mainLinks = [
		{ href: '/dashboard', label: 'Member Dashboard', icon: IconHome },
		{ href: '/dashboard/classes', label: 'My Classes', icon: IconSchool, bold: true },
		{ href: '/dashboard/indicators', label: 'My Indicators', icon: IconChartLine, bold: true }
	];

	// Tools section
	const toolsLinks = [
		{ href: '/dashboard/ww', label: 'Weekly Watchlist', icon: IconChartLine },
		{ href: 'https://intercom.help/simpler-trading/en/', label: 'Support', icon: IconHelp, external: true }
	];

	// Account section
	const accountLinks = [
		{ href: '/dashboard/account', label: 'My Account', icon: IconSettings }
	];

	// Generate gravatar URL
	function getGravatarUrl(email: string, size = 40): string {
		// Simple hash for gravatar - in production use proper MD5
		const defaultUrl = `https://ui-avatars.com/api/?name=${encodeURIComponent(user.name)}&size=${size}&background=0984ae&color=fff`;
		return user.avatar || defaultUrl;
	}
</script>

<aside
	class="dashboard__sidebar"
	class:is-collapsed={isCollapsed}
	class:is-open={isOpen}
>
	<!-- Primary Navigation -->
	<nav class="dashboard__nav-primary">
		<!-- User Profile -->
		<a href="/dashboard/account" class="dashboard__profile-nav-item">
			<span
				class="dashboard__profile-photo"
				style="background-image: url({getGravatarUrl(user.email)});"
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
								<link.icon size={20} />
							</span>
							<span
								class="dashboard__nav-item-text"
								style={link.bold ? 'font-weight: bold; color: white;' : ''}
							>
								{link.label}
							</span>
						</a>
					</li>
				{/each}
			</ul>
		</ul>

		<!-- Memberships Section -->
		{#if memberships.length > 0}
			<ul>
				<li>
					<p class="dashboard__nav-category">memberships</p>
				</li>
				<ul class="dash_main_links">
					{#each memberships as membership}
						<li class:is-active={isActive(`/dashboard/${membership.slug}`)}>
							<a href="/dashboard/{membership.slug}">
								<span class="dashboard__nav-item-icon">
									<IconChartLine size={20} />
								</span>
								<span class="dashboard__nav-item-text">{membership.name}</span>
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
					<li class:is-active={!link.external && isActive(link.href)}>
						<a
							href={link.href}
							target={link.external ? '_blank' : undefined}
							rel={link.external ? 'noopener noreferrer' : undefined}
						>
							<span class="dashboard__nav-item-icon">
								<link.icon size={20} />
							</span>
							<span class="dashboard__nav-item-text">{link.label}</span>
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
								<link.icon size={20} />
							</span>
							<span class="dashboard__nav-item-text">{link.label}</span>
						</a>
					</li>
				{/each}
			</ul>
		</ul>
	</nav>

	<!-- Sidebar Toggle Footer -->
	<footer class="dashboard__toggle">
		<button
			class="dashboard__toggle-button"
			onclick={onToggleCollapse}
			aria-label={isCollapsed ? 'Expand sidebar' : 'Collapse sidebar'}
		>
			<div class="dashboard__toggle-button-icon">
				<span></span>
				<span></span>
				<span></span>
			</div>
			<span class="framework__toggle-button-label">Dashboard Menu</span>
		</button>
	</footer>
</aside>

<!-- Mobile Overlay -->
{#if isOpen}
	<div
		class="dashboard__overlay"
		onclick={onToggleOpen}
		onkeydown={(e) => e.key === 'Escape' && onToggleOpen?.()}
		role="button"
		tabindex="0"
		aria-label="Close sidebar"
	></div>
{/if}
