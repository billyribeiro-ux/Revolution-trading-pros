<script lang="ts">
	/**
	 * Breadcrumb Navigation - WordPress EXACT
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Matches WordPress breadcrumb: Home / Member Dashboard / Page Title
	 *
	 * @version 1.0.0 (December 2025)
	 */

	interface BreadcrumbItem {
		label: string;
		href?: string;
	}

	interface Props {
		items: BreadcrumbItem[];
	}

	let { items }: Props = $props();
</script>

<!-- WordPress EXACT: #breadcrumbs.breadcrumbs -->
<nav id="breadcrumbs" class="breadcrumbs" aria-label="Breadcrumb">
	<div class="container-fluid">
		<ul>
			{#each items as item, index}
				{#if index > 0}
					<li class="separator"> / </li>
				{/if}
				{#if item.href && index < items.length - 1}
					<li class="item-{index === 0 ? 'home' : 'parent'}">
						<a
							class="breadcrumb-link breadcrumb-{index === 0 ? 'home' : 'parent'}"
							href={item.href}
							title={item.label}
						>
							{item.label}
						</a>
					</li>
				{:else}
					<li class="item-current">
						<strong title={item.label}>{item.label}</strong>
					</li>
				{/if}
			{/each}
		</ul>
	</div>
</nav>

<style>
	/* WordPress EXACT: .breadcrumbs */
	.breadcrumbs {
		background: #f8f9fa;
		border-bottom: 1px solid #e5e7eb;
		font-size: 13px;
		font-family: 'Open Sans', sans-serif;
	}

	.container-fluid {
		max-width: 1700px;
		margin: 0 auto;
		padding: 12px 20px;
	}

	@media (min-width: 1280px) {
		.container-fluid {
			padding: 12px 30px;
		}
	}

	ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 0;
	}

	li {
		display: inline;
		color: #6b7280;
	}

	.separator {
		margin: 0 8px;
		color: #9ca3af;
	}

	a {
		color: #6b7280;
		text-decoration: none;
		transition: color 0.15s ease;
	}

	a:hover {
		color: #0984ae;
		text-decoration: underline;
	}

	.item-current strong {
		color: #333;
		font-weight: 600;
	}

	/* Responsive */
	@media (max-width: 767px) {
		.container-fluid {
			padding: 10px 16px;
		}

		.breadcrumbs {
			font-size: 12px;
		}
	}
</style>
