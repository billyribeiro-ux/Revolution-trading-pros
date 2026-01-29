<!--
	CMS v2 Admin Layout
	═══════════════════════════════════════════════════════════════════════════════

	Sub-navigation for CMS v2 admin section with:
	- Dashboard
	- Content
	- Assets
	- SEO
	- Settings

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import { page } from '$app/stores';
	import type { Snippet } from 'svelte';
	import {
		IconLayoutDashboard,
		IconFileText,
		IconPhoto,
		IconSearch,
		IconSettings,
		IconTags,
		IconMenu2,
		IconLink,
		IconHistory
	} from '$lib/icons';

	let { children }: { children: Snippet } = $props();

	const navItems = [
		{ href: '/admin/cms-v2', label: 'Dashboard', icon: IconLayoutDashboard, exact: true },
		{ href: '/admin/cms-v2/content', label: 'Content', icon: IconFileText },
		{ href: '/admin/cms-v2/assets', label: 'Assets', icon: IconPhoto },
		{ href: '/admin/cms-v2/tags', label: 'Tags', icon: IconTags },
		{ href: '/admin/cms-v2/menus', label: 'Menus', icon: IconMenu2 },
		{ href: '/admin/cms-v2/redirects', label: 'Redirects', icon: IconLink },
		{ href: '/admin/cms-v2/revisions', label: 'Revisions', icon: IconHistory },
		{ href: '/admin/cms-v2/seo', label: 'SEO', icon: IconSearch },
		{ href: '/admin/cms-v2/settings', label: 'Settings', icon: IconSettings }
	];

	function isActive(href: string, exact: boolean = false): boolean {
		if (exact) {
			return $page.url.pathname === href;
		}
		return $page.url.pathname.startsWith(href);
	}
</script>

<div class="cms-layout">
	<!-- Sub Navigation -->
	<nav class="cms-nav">
		<div class="cms-nav-header">
			<span class="cms-badge">v2</span>
			<h2 class="cms-title">CMS</h2>
		</div>

		<ul class="cms-nav-list">
			{#each navItems as item}
				{@const Icon = item.icon}
				<li>
					<a href={item.href} class="cms-nav-item" class:active={isActive(item.href, item.exact)}>
						<Icon size={18} />
						<span>{item.label}</span>
					</a>
				</li>
			{/each}
		</ul>
	</nav>

	<!-- Content -->
	<div class="cms-content">
		{@render children()}
	</div>
</div>

<style>
	.cms-layout {
		display: flex;
		flex-direction: column;
		min-height: calc(100vh - 140px);
	}

	.cms-nav {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0 0.5rem;
		background: rgba(30, 41, 59, 0.3);
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
		overflow-x: auto;
		-webkit-overflow-scrolling: touch;
	}

	.cms-nav-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 0.5rem;
		flex-shrink: 0;
	}

	.cms-badge {
		font-size: 0.625rem;
		font-weight: 700;
		text-transform: uppercase;
		padding: 0.125rem 0.375rem;
		background: linear-gradient(135deg, var(--primary-500), #d4a600);
		color: #0f172a;
		border-radius: 0.25rem;
	}

	.cms-title {
		font-size: 1rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.cms-nav-list {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.cms-nav-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		color: #94a3b8;
		text-decoration: none;
		font-size: 0.875rem;
		font-weight: 500;
		border-bottom: 2px solid transparent;
		transition: all 0.15s;
		white-space: nowrap;
	}

	.cms-nav-item:hover {
		color: #f1f5f9;
		background: rgba(51, 65, 85, 0.3);
	}

	.cms-nav-item.active {
		color: var(--primary-500);
		border-bottom-color: var(--primary-500);
	}

	.cms-content {
		flex: 1;
	}

	@media (max-width: 768px) {
		.cms-nav {
			padding: 0;
		}

		.cms-nav-header {
			display: none;
		}

		.cms-nav-item {
			padding: 0.625rem 0.75rem;
			font-size: 0.8125rem;
		}

		.cms-nav-item span {
			display: none;
		}
	}
</style>
