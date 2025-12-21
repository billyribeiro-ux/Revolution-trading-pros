<script lang="ts">
	/**
	 * Membership Dashboard Layout - Simpler Trading EXACT
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * This layout creates the exact Simpler Trading structure:
	 * - Collapsed primary sidebar (icon-only vertical bar)
	 * - Secondary sidebar with membership-specific navigation
	 * - Main content area
	 *
	 * @version 1.0.0 (December 2025)
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
	import IconStar from '@tabler/icons-svelte/icons/star';
	import IconBook from '@tabler/icons-svelte/icons/book';
	import IconArchive from '@tabler/icons-svelte/icons/archive';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import IconChevronRight from '@tabler/icons-svelte/icons/chevron-right';
	import IconShoppingBag from '@tabler/icons-svelte/icons/shopping-bag';
	import IconPresentation from '@tabler/icons-svelte/icons/presentation';

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

<!-- ═══════════════════════════════════════════════════════════════════════════
     MEMBERSHIP DASHBOARD LAYOUT - Simpler Trading EXACT
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="membership-dashboard">
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
	</nav>

	<!-- Main Content Area -->
	<main class="membership-main">
		{@render children()}
	</main>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - Simpler Trading EXACT
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
	}

	.secondary-nav {
		list-style: none;
		margin: 0;
		padding: 0;
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
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 1279px) {
		.primary-sidebar {
			display: none;
		}

		.secondary-sidebar {
			display: none;
		}

		.membership-main {
			width: 100%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   REDUCED MOTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.primary-nav li a,
		.secondary-nav li a,
		.primary-nav-footer a {
			transition: none;
		}
	}
</style>
