<script lang="ts">
	/**
	 * PageHeader Component
	 * Consistent page header with title, breadcrumbs, and actions
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */
	import type { Snippet } from 'svelte';

	interface Breadcrumb {
		label: string;
		href?: string;
	}

	interface Props {
		title: string;
		subtitle?: string;
		breadcrumbs?: Breadcrumb[];
		actions?: Snippet;
	}

	let props: Props = $props();

	let breadcrumbs = $derived(props.breadcrumbs ?? []);
</script>

<header class="page-header">
	{#if breadcrumbs.length > 0}
		<nav class="breadcrumbs" aria-label="Breadcrumb">
			<ol>
				{#each breadcrumbs as crumb, i}
					<li>
						{#if crumb.href && i < breadcrumbs.length - 1}
							<a href={crumb.href}>{crumb.label}</a>
						{:else}
							<span aria-current={i === breadcrumbs.length - 1 ? 'page' : undefined}>
								{crumb.label}
							</span>
						{/if}
					</li>
				{/each}
			</ol>
		</nav>
	{/if}

	<div class="header-content">
		<div class="header-text">
			<h1 class="page-title">{props.title}</h1>
			{#if props.subtitle}
				<p class="page-subtitle">{props.subtitle}</p>
			{/if}
		</div>

		{#if props.actions}
			<div class="header-actions">
				{@render props.actions()}
			</div>
		{/if}
	</div>
</header>

<style>
	.page-header {
		margin-bottom: 1.5rem;
	}

	.breadcrumbs {
		margin-bottom: 0.75rem;
	}

	.breadcrumbs ol {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		list-style: none;
		margin: 0;
		padding: 0;
		font-size: 0.875rem;
	}

	.breadcrumbs li {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.breadcrumbs li:not(:last-child)::after {
		content: '/';
		color: var(--color-text-muted, #9ca3af);
	}

	.breadcrumbs a {
		color: var(--color-text-secondary, #6b7280);
		text-decoration: none;
		transition: color 0.15s;
	}

	.breadcrumbs a:hover {
		color: var(--color-primary, #6366f1);
	}

	.breadcrumbs span[aria-current='page'] {
		color: var(--color-text-primary, #111827);
		font-weight: 500;
	}

	.header-content {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.header-text {
		flex: 1;
	}

	.page-title {
		font-size: clamp(1.5rem, 4vw, 2rem);
		font-weight: 700;
		color: var(--color-text-primary, #111827);
		margin: 0;
		line-height: 1.2;
	}

	.page-subtitle {
		font-size: 0.9375rem;
		color: var(--color-text-secondary, #6b7280);
		margin: 0.25rem 0 0;
	}

	.header-actions {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	/* Tablet+ */
	@media (min-width: 640px) {
		.header-content {
			flex-direction: row;
			align-items: flex-start;
			justify-content: space-between;
		}

		.header-actions {
			flex-shrink: 0;
		}
	}
</style>
