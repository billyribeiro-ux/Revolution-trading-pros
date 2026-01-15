<script lang="ts">
	/**
	 * PageHeader - Consistent page header with title, description, and actions
	 * 
	 * @version 2.0.0
	 * @author Revolution Trading Pros
	 */
	import type { Snippet } from 'svelte';

	interface Props {
		title: string;
		description?: string;
		breadcrumbs?: { label: string; href?: string }[];
		actions?: Snippet;
		children?: Snippet;
	}

	let {
		title,
		description = '',
		breadcrumbs = [],
		actions,
		children
	}: Props = $props();
</script>

<header class="page-header">
	{#if breadcrumbs.length > 0}
		<nav class="breadcrumbs" aria-label="Breadcrumb">
			{#each breadcrumbs as crumb, i}
				{#if i > 0}
					<span class="breadcrumb-separator">/</span>
				{/if}
				{#if crumb.href}
					<a href={crumb.href} class="breadcrumb-link">{crumb.label}</a>
				{:else}
					<span class="breadcrumb-current">{crumb.label}</span>
				{/if}
			{/each}
		</nav>
	{/if}

	<div class="header-content">
		<div class="header-text">
			<h1 class="page-title">{title}</h1>
			{#if description}
				<p class="page-description">{description}</p>
			{/if}
		</div>
		
		<div class="header-actions">
			{@render actions?.()}
		</div>
	</div>

	{@render children?.()}
</header>

<style>
	.page-header {
		margin-bottom: 2rem;
	}

	.breadcrumbs {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 1rem;
		font-size: 0.875rem;
	}

	.breadcrumb-link {
		color: var(--color-rtp-muted, #64748b);
		text-decoration: none;
		transition: color 0.2s;
	}

	.breadcrumb-link:hover {
		color: var(--color-rtp-primary, #FFD11A);
	}

	.breadcrumb-separator {
		color: var(--color-rtp-muted, #475569);
	}

	.breadcrumb-current {
		color: var(--color-rtp-text, #e2e8f0);
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: 1.5rem;
		flex-wrap: wrap;
	}

	.header-text {
		flex: 1;
		min-width: 200px;
	}

	.page-title {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--color-rtp-text, #f1f5f9);
		margin: 0 0 0.25rem 0;
		line-height: 1.2;
	}

	.page-description {
		font-size: 0.9375rem;
		color: var(--color-rtp-muted, #64748b);
		margin: 0;
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	@media (max-width: 640px) {
		.page-title {
			font-size: 1.5rem;
		}

		.header-content {
			flex-direction: column;
		}

		.header-actions {
			width: 100%;
		}
	}
</style>
