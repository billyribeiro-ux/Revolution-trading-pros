<script lang="ts">
	// Centered page title + subtitle banner.
	// Behaviour preserved 1:1 from courses/create +page.svelte
	// (originally lines 1645-1648).
	interface Props {
		title: string;
		subtitle: string;
	}

	let { title, subtitle }: Props = $props();
</script>

<div class="page-header">
	<h1>{title}</h1>
	<p class="subtitle">{subtitle}</p>
</div>

<style>
	/* Mirrors the original +page.svelte rules, including the mobile media query. */
	.page-header {
		text-align: center;
		margin-bottom: 1.5rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		margin: 0;
	}

	.subtitle {
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
		margin: 0.25rem 0 0;
	}

	@media (max-width: 639.98px) {
		.page-header {
			flex-direction: column;
			align-items: flex-start;
		}
	}
</style>
