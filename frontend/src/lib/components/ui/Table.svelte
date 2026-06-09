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
	const tableBodyClass = $derived([
		'table-body',
		striped && 'table-body--striped',
		hoverable && 'table-body--hoverable'
	]);
</script>

<div class="table-scroll">
	<table class="data-table">
		{#if headers.length > 0}
			<thead class="table-head">
				<tr>
					{#each headers as header (header)}
						<th class="table-heading">
							{header}
						</th>
					{/each}
				</tr>
			</thead>
		{/if}

		<tbody class={tableBodyClass}>
			{@render props.children?.()}
		</tbody>
	</table>
</div>

<style>
	.table-scroll {
		overflow-x: auto;
	}

	.data-table {
		min-width: 100%;
		border-collapse: collapse;
		border-spacing: 0;
	}

	.data-table,
	.table-body {
		background: #fff;
	}

	.data-table :global(tr) {
		border-bottom: 1px solid rgb(229 231 235);
	}

	.table-head {
		background: rgb(249 250 251);
	}

	.table-heading {
		padding: 0.75rem 1.5rem;
		color: rgb(107 114 128);
		font-size: 0.75rem;
		font-weight: 500;
		letter-spacing: 0.05em;
		line-height: 1.2;
		text-align: left;
		text-transform: uppercase;
	}

	.table-body--striped :global(tr:nth-child(even)) {
		background-color: rgb(249 250 251);
	}

	.table-body--hoverable :global(tr:hover) {
		background-color: rgb(249 250 251);
		transition: background-color 150ms ease;
	}

	.table-body :global(td) {
		padding-left: 1.5rem;
		padding-right: 1.5rem;
		padding-top: 1rem;
		padding-bottom: 1rem;
		white-space: nowrap;
		font-size: 0.875rem;
		color: rgb(17 24 39);
	}
</style>
