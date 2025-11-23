<script lang="ts">
	export let headers: string[] = [];
	export let striped: boolean = false;
	export let hoverable: boolean = true;
</script>

<div class="overflow-x-auto">
	<table class="min-w-full divide-y divide-gray-200">
		{#if headers.length > 0}
			<thead class="bg-gray-50">
				<tr>
					{#each headers as header}
						<th
							class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
						>
							{header}
						</th>
					{/each}
				</tr>
			</thead>
		{/if}

		<tbody
			class="bg-white divide-y divide-gray-200 {striped
				? '[&>tr:nth-child(even)]:bg-gray-50'
				: ''} {hoverable ? 'hoverable' : ''}"
		>
			<slot />
		</tbody>
	</table>
</div>

<style>
	tbody.hoverable :global(tr:hover) {
		background-color: rgb(249 250 251);
		transition: colors 150ms;
	}

	tbody :global(td) {
		padding-left: 1.5rem;
		padding-right: 1.5rem;
		padding-top: 1rem;
		padding-bottom: 1rem;
		white-space: nowrap;
		font-size: 0.875rem;
		color: rgb(17 24 39);
	}
</style>
