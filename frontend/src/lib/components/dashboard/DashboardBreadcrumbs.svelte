<!--
	DashboardBreadcrumbs - WordPress Reference Match
	Source: Line 2729 of DashboardHome

	Structure:
	<nav id="breadcrumbs" class="breadcrumbs">
	  <div class="container-fluid">
	    <ul>
	      <li class="item-home"><a>Home</a></li>
	      <li class="separator"> / </li>
	      <li class="item-current"><strong>Page Title</strong></li>
	    </ul>
	  </div>
	</nav>
-->
<script lang="ts">
	import { page } from '$app/state';
	import { onMount } from 'svelte';

	interface BreadcrumbItem {
		label: string;
		href: string | null;
		isCurrent: boolean;
		className?: string;
	}

	// Generate breadcrumb items from current path
	// Apple ICT 11+ Pattern: Works on both SSR and client
	let breadcrumbs = $derived.by(() => {
		// Access pathname safely - $page store works on both server and client
		const pathname = page?.url?.pathname || '/';
		const segments = pathname.split('/').filter(Boolean);

		const items: BreadcrumbItem[] = [
			{ label: 'Home', href: '/', isCurrent: false, className: 'item-home' }
		];

		// Check if we're on a class detail page (e.g., /classes/tax-loss-harvest)
		const isClassDetailPage = segments[0] === 'classes' && segments.length > 1;
		
		// Check if we're on My Classes list page
		const isMyClassesPage = pathname === '/dashboard/classes';
		
		// Check if we're on My Indicators list page
		const isMyIndicatorsPage = pathname === '/dashboard/indicators';

		if (isClassDetailPage) {
			// Class detail page breadcrumb: Home / Classes / [Class Name]
			items.push({
				label: 'Classes',
				href: '/dashboard/classes',
				isCurrent: false,
				className: 'item-cat item-custom-post-type-classes'
			});

			// Add class name as current item
			const className = segments[1]
				.split('-')
				.map(word => word.charAt(0).toUpperCase() + word.slice(1))
				.join(' ');

			items.push({
				label: className,
				href: null,
				isCurrent: true,
				className: 'item-current'
			});
		} else if (isMyClassesPage) {
			// My Classes list page: Home / Member Dashboard / My Classes
			items.push({
				label: 'Member Dashboard',
				href: '/dashboard',
				isCurrent: false,
				className: 'item-parent item-parent-401190'
			});

			items.push({
				label: 'My Classes',
				href: null,
				isCurrent: true,
				className: 'item-current item-402845'
			});
		} else if (isMyIndicatorsPage) {
			// My Indicators list page: Home / Member Dashboard / My Indicators
			items.push({
				label: 'Member Dashboard',
				href: '/dashboard',
				isCurrent: false,
				className: 'item-parent item-parent-401190'
			});

			items.push({
				label: 'My Indicators',
				href: null,
				isCurrent: true,
				className: 'item-current item-1021444'
			});
		} else {
			// Default breadcrumb generation for other pages
			let currentPath = '';
			segments.forEach((segment, index) => {
				currentPath += `/${segment}`;
				const isLast = index === segments.length - 1;

				// Format segment to readable label
				const label = segment
					.split('-')
					.map(word => word.charAt(0).toUpperCase() + word.slice(1))
					.join(' ');

				// Map dashboard to "Member Dashboard"
				const displayLabel = segment === 'dashboard' ? 'Member Dashboard' : label;

				items.push({
					label: displayLabel,
					href: isLast ? null : currentPath,
					isCurrent: isLast
				});
			});
		}

		return items;
	});

	// Fix Classes breadcrumb link on mount (matches WordPress implementation)
	onMount(() => {
		const classesLink = document.querySelector('a.breadcrumb-custom-post-type-classes');
		if (classesLink) {
			classesLink.setAttribute('href', '/dashboard/classes');
		}
	});
</script>

<nav id="breadcrumbs" class="breadcrumbs" aria-label="Breadcrumb">
	<div class="container-fluid">
		<ul>
			{#each breadcrumbs as item, index}
				{#if index > 0}
					<li class="separator{index === 1 ? ' separator-home' : ''}" aria-hidden="true"> / </li>
				{/if}
				<li class={item.className || (item.isCurrent ? 'item-current' : 'item-home')}>
					{#if item.isCurrent}
						<strong class="breadcrumb-current" title={item.label}>{item.label}</strong>
					{:else}
						<a 
							class="breadcrumb-link{index === 0 ? ' breadcrumb-home' : ''}{item.className?.includes('custom-post-type-classes') ? ' breadcrumb-cat breadcrumb-custom-post-type-classes' : ''}" 
							href={item.href}
							title={item.label}
						>
							{item.label}
						</a>
					{/if}
				</li>
			{/each}
		</ul>
	</div>
</nav>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Breadcrumbs - Mobile-First Responsive Design
	 * Apple ICT 11+ Standards
	 * Position: Below main navigation, above dashboard content
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Base styles - Mobile (320px+) */
	.breadcrumbs {
		z-index: 1;
		background-color: #efefef;
		border-bottom: 1px solid #dbdbdb;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 12px;
		line-height: 1.5;
	}

	.container-fluid {
		max-width: 100%;
		padding: 8px 15px;
	}

	ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 4px;
	}

	li {
		display: inline-flex;
		align-items: center;
	}

	.separator {
		color: #999;
		padding: 0 4px;
		font-size: 11px;
	}

	.breadcrumb-link {
		color: #1e73be;
		text-decoration: none;
		transition: color 0.15s ease-in-out;
		font-size: 12px;
		font-weight: 800;
	}

	.breadcrumb-link:hover {
		color: #143E59;
		text-decoration: underline;
	}

	.breadcrumb-current {
		color: #666;
		font-weight: 800;
		font-size: 12px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Tablet (768px+)
	 * ═══════════════════════════════════════════════════════════════════════════ */
	@media (min-width: 768px) {
		.breadcrumbs {
			font-size: 13px;
		}

		.container-fluid {
			padding: 10px 20px;
		}

		.separator {
			padding: 0 8px;
			font-size: 13px;
		}

		.breadcrumb-link,
		.breadcrumb-current {
			font-size: 13px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Desktop (1280px+)
	 * ═══════════════════════════════════════════════════════════════════════════ */
	@media (min-width: 1280px) {
		.container-fluid {
			padding: 12px 30px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Large Desktop (1440px+)
	 * ═══════════════════════════════════════════════════════════════════════════ */
	@media (min-width: 1440px) {
		.container-fluid {
			padding: 12px 40px;
		}
	}
</style>
