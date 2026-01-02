<!--
	SecondaryDashboardSidebar - Secondary Navigation Panel
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation

	Fixed sidebar that appears when main DashboardSidebar is collapsed.
	Provides course/membership-specific navigation.

	Svelte 5 Features:
	- $props() for type-safe props
	- $derived() for computed values
	- $state() for local state management

	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { page } from '$app/state';
	import RtpIcon from '$lib/components/icons/RtpIcon.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	interface SubMenuItem {
		href: string;
		icon: string;
		text: string;
	}

	interface MenuItem {
		href: string;
		icon: string;
		text: string;
		simplerIcon?: string;
		submenu?: SubMenuItem[];
	}

	interface Props {
		courseName: string;
		menuItems: MenuItem[];
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS & STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let { courseName, menuItems }: Props = $props();

	let expandedMenus = $state<Set<string>>(new Set());

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED
	// ═══════════════════════════════════════════════════════════════════════════

	let currentPath = $derived(page.url.pathname);

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function isActive(href: string): boolean {
		if (href === '#') return false;
		return currentPath === href || currentPath.startsWith(href + '/');
	}

	function isExactActive(href: string): boolean {
		return currentPath === href;
	}

	function toggleSubmenu(text: string): void {
		const newSet = new Set(expandedMenus);
		if (newSet.has(text)) {
			newSet.delete(text);
		} else {
			newSet.add(text);
		}
		expandedMenus = newSet;
	}

	function hasActiveSubmenuItem(submenu: SubMenuItem[] | undefined): boolean {
		if (!submenu) return false;
		return submenu.some(item => isActive(item.href));
	}
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
	 SECONDARY SIDEBAR - Fixed position next to collapsed main sidebar
	 ═══════════════════════════════════════════════════════════════════════════ -->
<aside
	class="dashboard__nav-secondary"
	role="navigation"
	aria-label="{courseName} navigation"
>
	<div class="dashboard__nav-secondary-content">
		<h3 class="dashboard__nav-secondary-title">{courseName}</h3>

		<nav>
			<ul class="dashboard__nav-secondary-menu">
				{#each menuItems as item}
					<li>
						{#if item.submenu && item.submenu.length > 0}
							<!-- Menu item with submenu -->
							<button
								type="button"
								class="dashboard__nav-secondary-item"
								class:is-active={hasActiveSubmenuItem(item.submenu)}
								class:is-expanded={expandedMenus.has(item.text)}
								onclick={() => toggleSubmenu(item.text)}
								aria-expanded={expandedMenus.has(item.text)}
							>
								{#if item.icon}
									<RtpIcon name={item.icon} size={20} />
								{/if}
								<span class="dashboard__nav-secondary-item-text">{item.text}</span>
								<span class="dashboard__nav-secondary-arrow">
									<RtpIcon name="chevron-down" size={16} />
								</span>
							</button>

							{#if expandedMenus.has(item.text)}
								<ul class="dashboard__nav-secondary-submenu">
									{#each item.submenu as subitem}
										<li>
											<a
												href={subitem.href}
												class="dashboard__nav-secondary-subitem"
												class:is-active={isActive(subitem.href)}
												aria-current={isExactActive(subitem.href) ? 'page' : undefined}
											>
												{subitem.text}
											</a>
										</li>
									{/each}
								</ul>
							{/if}
						{:else}
							<!-- Regular menu item -->
							<a
								href={item.href}
								class="dashboard__nav-secondary-item"
								class:is-active={isActive(item.href)}
								aria-current={isExactActive(item.href) ? 'page' : undefined}
							>
								{#if item.icon}
									<RtpIcon name={item.icon} size={20} />
								{/if}
								<span class="dashboard__nav-secondary-item-text">{item.text}</span>
							</a>
						{/if}
					</li>
				{/each}
			</ul>
		</nav>
	</div>
</aside>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * SECONDARY SIDEBAR - Fixed Position Layout
	 * Positioned at left: 80px (after collapsed main sidebar)
	 * Full viewport height
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-secondary {
		position: fixed;
		top: 0;
		left: 80px;
		width: 280px;
		height: 100vh;
		background-color: #143E59;
		overflow-y: auto;
		overflow-x: hidden;
		z-index: 98;
		box-shadow: 2px 0 10px rgba(0, 0, 0, 0.2);
	}

	.dashboard__nav-secondary-content {
		padding: 30px 0;
	}

	.dashboard__nav-secondary-title {
		color: #ffffff;
		font-size: 16px;
		font-weight: 700;
		margin: 0 0 24px;
		padding: 0 24px;
		font-family: var(--font-heading, 'Montserrat', sans-serif);
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * MENU LIST
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-secondary-menu {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * MENU ITEMS (Links & Buttons)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-secondary-item {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 14px 24px;
		color: rgba(255, 255, 255, 0.75);
		text-decoration: none;
		font-size: 14px;
		font-weight: 500;
		transition: all 0.2s ease;
		width: 100%;
		background: none;
		border: none;
		cursor: pointer;
		text-align: left;
		font-family: var(--font-heading, 'Montserrat', sans-serif);
		position: relative;
	}

	.dashboard__nav-secondary-item:hover {
		background-color: rgba(255, 255, 255, 0.08);
		color: #ffffff;
	}

	.dashboard__nav-secondary-item.is-active {
		background-color: rgba(255, 255, 255, 0.12);
		color: #ffffff;
		font-weight: 600;
	}

	.dashboard__nav-secondary-item.is-active::before {
		content: '';
		position: absolute;
		left: 0;
		top: 0;
		bottom: 0;
		width: 3px;
		background-color: #f7931e;
	}

	.dashboard__nav-secondary-item-text {
		flex: 1;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SUBMENU ARROW
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-secondary-arrow {
		display: flex;
		align-items: center;
		justify-content: center;
		transition: transform 0.2s ease;
		opacity: 0.7;
	}

	.dashboard__nav-secondary-item.is-expanded .dashboard__nav-secondary-arrow {
		transform: rotate(180deg);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SUBMENU LIST
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-secondary-submenu {
		list-style: none;
		margin: 0;
		padding: 4px 0 8px;
		background-color: rgba(0, 0, 0, 0.15);
	}

	.dashboard__nav-secondary-subitem {
		display: block;
		padding: 10px 24px 10px 56px;
		color: rgba(255, 255, 255, 0.65);
		text-decoration: none;
		font-size: 13px;
		font-weight: 400;
		transition: all 0.2s ease;
		font-family: var(--font-heading, 'Montserrat', sans-serif);
		position: relative;
	}

	.dashboard__nav-secondary-subitem:hover {
		background-color: rgba(255, 255, 255, 0.05);
		color: rgba(255, 255, 255, 0.9);
	}

	.dashboard__nav-secondary-subitem.is-active {
		color: #ffffff;
		font-weight: 600;
	}

	.dashboard__nav-secondary-subitem.is-active::before {
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
	 * RESPONSIVE - Hide on smaller screens
	 * Only show on xl breakpoint (1280px+) when main sidebar would be collapsed
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 1279px) {
		.dashboard__nav-secondary {
			display: none;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * REDUCED MOTION
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.dashboard__nav-secondary-item,
		.dashboard__nav-secondary-subitem,
		.dashboard__nav-secondary-arrow {
			transition: none;
		}
	}
</style>
