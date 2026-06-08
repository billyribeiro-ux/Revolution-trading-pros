<!--
	Breadcrumbs Navigation Component
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11 Principal Engineer Implementation
	Svelte 5 (December 2025) Best Practices
	
	Implements complete design specifications from DASHBOARD_DESIGN_SPECIFICATIONS.md:
	- Hierarchical navigation structure
	- JSON-LD schema markup for SEO
	- Accessibility features (ARIA, semantic HTML)
	- Responsive behavior (desktop/mobile)
	
-->
<script lang="ts">
	import { page } from '$app/state';

	interface BreadcrumbItem {
		name: string;
		href?: string;
		position: number;
	}

	interface Props {
		items?: BreadcrumbItem[];
	}

	let props: Props = $props();

	// Generate breadcrumb items from current page if not provided
	const breadcrumbItems = $derived.by(() => {
		if (props.items && props.items.length > 0) return props.items;

		// Default: Current page only (no Home link)
		const currentPath = page?.url?.pathname ?? '/dashboard';
		const pathSegments = currentPath.split('/').filter(Boolean);

		const defaultItems: BreadcrumbItem[] = [];

		// Add current page
		if (pathSegments.length > 0) {
			const pageName = pathSegments[pathSegments.length - 1]
				.split('-')
				.map((word: string) => word.charAt(0).toUpperCase() + word.slice(1))
				.join(' ');

			defaultItems.push({
				name: pageName === 'Dashboard' ? 'Member Dashboard' : pageName,
				position: 1
			});
		}

		return defaultItems;
	});

	// Generate JSON-LD schema for SEO. Removing it breaks breadcrumb structured
	// data (SEO regression).
	const breadcrumbSchema = $derived({
		'@context': 'https://schema.org',
		'@type': 'BreadcrumbList',
		'@id': `${page?.url?.href ?? ''}#breadcrumb`,
		itemListElement: breadcrumbItems.map((item: BreadcrumbItem) => ({
			'@type': 'ListItem',
			position: item.position,
			name: item.name,
			...(item.href && { item: item.href })
		}))
	});

	const breadcrumbJsonLd = $derived(JSON.stringify(breadcrumbSchema).replace(/</g, '\\u003c'));

	const routeClass = $derived(page.route?.id?.replace(/\//g, '-') ?? 'page');
</script>

<!-- JSON-LD Schema Markup -->
<svelte:head>
	<!-- eslint-disable-next-line svelte/no-at-html-tags -- trusted JSON-LD generated from structured breadcrumb fields and escaped above -->
	{@html '<script type="application/ld+json">' + breadcrumbJsonLd + '</scr' + 'ipt>'}
</svelte:head>

<!-- Breadcrumbs Navigation -->
<nav id="breadcrumbs" class="breadcrumbs" aria-label="Breadcrumb navigation">
	<ul>
		{#each breadcrumbItems as item (item.position)}
			{#if item.href}
				<!-- Linked breadcrumb item -->
				<li class="item-home">
					<a class="breadcrumb-link breadcrumb-home" href={item.href} title={item.name}>
						{item.name}
					</a>
				</li>
			{:else}
				<!-- Current page (no link) -->
				<li class={['item-current', `item-${routeClass}`]}>
					<strong class={['breadcrumb-current', `breadcrumb-${routeClass}`]}>{item.name}</strong>
				</li>
			{/if}
		{/each}
	</ul>
</nav>

<style>
	/* Breadcrumbs styles are defined in dashboard.css */
	/* This component uses the design specifications from DASHBOARD_DESIGN_SPECIFICATIONS.md */

	/* Component-specific overrides if needed */
	.breadcrumbs {
		padding: 12px 20px;
		background-color: transparent;
		max-width: 1200px;
		margin: 0 auto;
	}

	/* Responsive adjustments */
	@media (max-width: 767.98px) {
		.breadcrumbs {
			padding: 8px 16px;
			font-size: 14px;
		}
	}
</style>
