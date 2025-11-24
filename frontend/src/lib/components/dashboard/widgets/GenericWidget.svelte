<script lang="ts">
	export let data: any;
	export let config: {
		display_mode?: 'json' | 'table' | 'list';
		max_depth?: number;
		show_empty?: boolean;
		highlight_keys?: string[];
	} = {};

	$: displayMode = config.display_mode || 'json';
	$: showEmpty = config.show_empty !== false;

	function renderData(obj: any, depth: number = 0): any {
		if (config.max_depth && depth >= config.max_depth) {
			return '...';
		}
		return obj;
	}

	function isHighlighted(key: string): boolean {
		return config.highlight_keys?.includes(key) || false;
	}
</script>

<div class="generic-widget">
	{#if data}
		{#if displayMode === 'json'}
			<pre class="data-display">{JSON.stringify(renderData(data), null, 2)}</pre>
		{:else if displayMode === 'table' && Array.isArray(data)}
			<table class="data-table">
				<thead>
					<tr>
						{#each Object.keys(data[0] || {}) as key}
							<th class:highlighted={isHighlighted(key)}>{key}</th>
						{/each}
					</tr>
				</thead>
				<tbody>
					{#each data as row}
						<tr>
							{#each Object.entries(row) as [key, value]}
								<td class:highlighted={isHighlighted(key)}>{value}</td>
							{/each}
						</tr>
					{/each}
				</tbody>
			</table>
		{:else if displayMode === 'list'}
			<ul class="data-list">
				{#each Object.entries(data) as [key, value]}
					<li class:highlighted={isHighlighted(key)}>
						<strong>{key}:</strong>
						{JSON.stringify(value)}
					</li>
				{/each}
			</ul>
		{/if}
	{:else if showEmpty}
		<div class="no-data">No data available</div>
	{/if}
</div>

<style>
	.generic-widget {
		height: 100%;
		overflow: auto;
	}

	.data-display {
		font-family: 'Monaco', 'Courier New', monospace;
		font-size: 0.75rem;
		background: #f9fafb;
		padding: 1rem;
		border-radius: 6px;
		overflow-x: auto;
		color: #1f2937;
	}

	.no-data {
		text-align: center;
		color: #6b7280;
		padding: 2rem;
	}

	.data-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.875rem;
	}

	.data-table th,
	.data-table td {
		padding: 0.5rem;
		text-align: left;
		border-bottom: 1px solid #e5e7eb;
	}

	.data-table th {
		background: #f9fafb;
		font-weight: 600;
		color: #1f2937;
	}

	.data-table td {
		color: #6b7280;
	}

	.data-list {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	.data-list li {
		padding: 0.75rem;
		border-bottom: 1px solid #e5e7eb;
		font-size: 0.875rem;
	}

	.data-list li:last-child {
		border-bottom: none;
	}

	.highlighted {
		background: #fef3c7;
		font-weight: 600;
	}
</style>
