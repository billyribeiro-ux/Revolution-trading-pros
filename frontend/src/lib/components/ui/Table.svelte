<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		headers?: string[];
		striped?: boolean;
		hoverable?: boolean;
		children?: Snippet;
	}

	let props: Props = $props();
	let headers = $derived(props.headers ?? []);
	let striped = $derived(props.striped ?? false);
	let hoverable = $derived(props.hoverable ?? true);
</script>

<div class="standalone-table-wrap">
	<table class="standalone-table">
		{#if headers.length > 0}
			<thead class="standalone-table-head">
				<tr>
					{#each headers as header}
						<th class="standalone-table-th">
							{header}
						</th>
					{/each}
				</tr>
			</thead>
		{/if}

		<tbody
			class="standalone-table-body"
			data-striped={striped || undefined}
			data-hoverable={hoverable || undefined}
		>
			{@render props.children?.()}
		</tbody>
	</table>
</div>

<style>
	.standalone-table-wrap {
		overflow-x: auto;
	}

	.standalone-table {
		min-inline-size: 100%;
		border-collapse: separate;
		border-spacing: 0;
	}

	.standalone-table-head {
		background-color: oklch(0.98 0.002 265);
	}

	.standalone-table-th {
		padding-inline: var(--space-6);
		padding-block: var(--space-3);
		text-align: start;
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
		color: oklch(0.55 0.01 265);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		border-block-end: 1px solid var(--border);
	}

	.standalone-table-body {
		background-color: var(--background);

		& :global(tr) {
			border-block-end: 1px solid var(--border);
		}

		& :global(td) {
			padding-inline: var(--space-6);
			padding-block: var(--space-4);
			white-space: nowrap;
			font-size: var(--text-sm);
			color: oklch(0.2 0.01 265);
		}

		&[data-striped] :global(tr:nth-child(even)) {
			background-color: oklch(0.98 0.002 265);
		}

		&[data-hoverable] :global(tr:hover) {
			background-color: oklch(0.98 0.002 265);
			transition: background-color var(--duration-fast) var(--ease-default);
		}
	}
</style>
