<!--
	DashboardContentSidebar - Right-Side Content Sidebar
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	WordPress → Svelte 5 Pixel-Perfect Conversion

	SOURCE TRUTH:
	- WordPress: frontend/Implementation/DashboardHome:4406-4409
	- CSS Specs: DASHBOARD_DESIGN_SPECIFICATIONS.md:460-474

	COMPONENT DESCRIPTION:
	This is the RIGHT-SIDE content sidebar (dashboard__content-sidebar).
	NOT the same as:
	- dashboard__sidebar (LEFT navigation sidebar)
	- dashboard__sidebar-secondary (hover panel when collapsed)

	BEHAVIOR:
	- Hidden by default (display: none)
	- Shows at 1080px+ breakpoint
	- 260px fixed width
	- Contains contextual content sections

	Svelte 5 Features:
	- $props() for component props
	- Snippet for children content
	- $props.id() for SSR-safe unique IDs (Svelte 5.20+)

	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import type { Snippet } from 'svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	interface ContentSection {
		title?: string;
		content: Snippet;
	}

	interface Props {
		/** Child content to render inside the sidebar */
		children?: Snippet;
		/** Optional sections with titles */
		sections?: ContentSection[];
		/** Additional CSS classes */
		class?: string;
		/** Whether sidebar is visible (overrides responsive behavior) */
		visible?: boolean;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS
	// ═══════════════════════════════════════════════════════════════════════════

	let {
		children,
		sections = [],
		class: className = '',
		visible
	}: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED
	// ═══════════════════════════════════════════════════════════════════════════

	// Combine base class with any additional classes
	const sidebarClasses = $derived(
		['dashboard__content-sidebar', className, visible === true && 'is-visible', visible === false && 'is-hidden']
			.filter(Boolean)
			.join(' ')
	);
</script>

<!-- Right-Side Content Sidebar - WordPress Exact Match -->
<aside class={sidebarClasses} aria-label="Content sidebar">
	<!-- Render sections if provided -->
	{#if sections.length > 0}
		{#each sections as section}
			<section class="content-sidebar__section">
				{#if section.title}
					<h3 class="content-sidebar__title">{section.title}</h3>
				{/if}
				{@render section.content()}
			</section>
		{/each}
	{/if}

	<!-- Render children if provided -->
	{#if children}
		<section class="content-sidebar__section">
			{@render children()}
		</section>
	{/if}
</aside>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * CONTENT SIDEBAR - WordPress Exact Match
	 * Source: DASHBOARD_DESIGN_SPECIFICATIONS.md:460-474
	 *
	 * This is the RIGHT-SIDE content sidebar, NOT the left navigation sidebar.
	 * Hidden by default, shows at 1080px+ breakpoint.
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content-sidebar {
		display: none;
		width: 260px;
		flex: 0 0 auto;
		margin-top: -1px;
		border-right: 1px solid #dbdbdb;
		border-top: 1px solid #dbdbdb;
		background-color: #ffffff;
	}

	/* Show at 1080px+ (WordPress exact breakpoint) */
	@media (min-width: 1080px) {
		.dashboard__content-sidebar {
			display: block;
		}
	}

	/* Force visibility override */
	.dashboard__content-sidebar.is-visible {
		display: block;
	}

	/* Force hidden override */
	.dashboard__content-sidebar.is-hidden {
		display: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * CONTENT SIDEBAR SECTIONS
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.content-sidebar__section {
		padding: 20px;
		border-bottom: 1px solid #ededed;
	}

	.content-sidebar__section:last-child {
		border-bottom: none;
	}

	.content-sidebar__title {
		margin: 0 0 15px;
		font-size: 14px;
		font-weight: 700;
		color: #333;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * REDUCED MOTION
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.dashboard__content-sidebar {
			transition: none;
		}
	}
</style>
