<!--
	LearningCenterFilter Component
	═══════════════════════════════════════════════════════════════════════════
	
	Apple ICT 11+ Principal Engineer Grade - January 2026
	
	Reusable category filter component for learning centers across all trading rooms.
	Matches WordPress term_filter structure exactly.
	
	@version 1.0.0
-->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';

	interface Category {
		id: string;
		label: string;
	}

	interface Props {
		categories: Category[];
		activeFilter?: string;
	}

	let { categories, activeFilter = 'all' }: Props = $props();

	// Filter resources by navigating to new URL with query params
	function filterResources(categoryId: string) {
		const url = new URL(page.url);
		if (categoryId === 'all' || categoryId === '0') {
			url.searchParams.delete('category');
		} else {
			url.searchParams.set('category', categoryId);
		}
		url.searchParams.delete('page'); // Reset to page 1
		goto(url.toString(), { replaceState: true });
	}
</script>

<form action="#" method="POST" id="term_filter" onsubmit={(e) => e.preventDefault()}>
	<div class="reset_filter">
		<input
			type="radio"
			id="filter-0"
			value="0"
			name="categoryfilter"
			checked={activeFilter === 'all' || activeFilter === '0'}
			onchange={() => filterResources('all')}
		/>
		<label for="filter-0">
			<svg
				aria-hidden="true"
				focusable="false"
				data-prefix="fas"
				data-icon="undo"
				class="svg-inline--fa fa-undo fa-w-16"
				role="img"
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 512 512"
			>
				<path
					fill="currentColor"
					d="M212.333 224.333H12c-6.627 0-12-5.373-12-12V12C0 5.373 5.373 0 12 0h48c6.627 0 12 5.373 12 12v78.112C117.773 39.279 184.26 7.47 258.175 8.007c136.906.994 246.448 111.623 246.157 248.532C504.041 393.258 393.12 504 256.333 504c-64.089 0-122.496-24.313-166.51-64.215-5.099-4.622-5.334-12.554-.467-17.42l33.967-33.967c4.474-4.474 11.662-4.717 16.401-.525C170.76 415.336 211.58 432 256.333 432c97.268 0 176-78.716 176-176 0-97.267-78.716-176-176-176-58.496 0-110.28 28.476-142.274 72.333h98.274c6.627 0 12 5.373 12 12v48c0 6.627-5.373 12-12 12z"
				></path>
			</svg>
		</label>
	</div>
	{#each categories as category}
		<div class="filter_btn">
			<input
				type="radio"
				id="filter-{category.id}"
				value={category.id}
				name="categoryfilter"
				checked={activeFilter === category.id}
				onchange={() => filterResources(category.id)}
			/>
			<label for="filter-{category.id}">{category.label}</label>
		</div>
	{/each}
	<button class="apply_filter">Apply filter</button>
</form>

<style>
	/* Term Filter Form - Exact WordPress CSS from dashboard.8f78208b.css */
	#term_filter {
		display: -webkit-flex;
		display: -ms-flexbox;
		display: flex;
		overflow-x: scroll;
		-webkit-align-items: center;
		-ms-flex-align: center;
		align-items: center;
		background: #fff;
		padding: 20px;
		-webkit-box-shadow: 0 3px 6px #00000029;
		box-shadow: 0 3px 6px #00000029;
	}

	#term_filter .apply_filter {
		display: none;
	}

	/* Reset Filter Button */
	#term_filter .reset_filter {
		margin-right: 10px;
	}

	#term_filter .reset_filter :global(svg) {
		min-width: 20px;
		color: #666;
	}

	#term_filter .reset_filter label {
		border-radius: 25px;
		cursor: pointer;
		padding: 13px 32px;
		border: 2px solid;
		display: -webkit-flex;
		display: -ms-flexbox;
		display: flex;
		margin-bottom: 0;
		-webkit-align-items: center;
		-ms-flex-align: center;
		align-items: center;
		font-family: 'Montserrat', sans-serif;
		color: #666;
	}

	#term_filter .reset_filter input {
		display: none;
	}

	/* Filter Buttons */
	#term_filter .filter_btn {
		margin-right: 10px;
	}

	#term_filter .filter_btn input {
		display: none;
	}

	#term_filter .filter_btn input:checked + label {
		background: #333;
		color: #fff;
	}

	#term_filter .filter_btn label {
		cursor: pointer;
		padding: 11px 32px;
		border-radius: 25px;
		border: 2px solid;
		white-space: nowrap;
		font-weight: 700;
		color: #666;
		display: block;
		margin-bottom: 0;
		font-family: 'Montserrat', sans-serif;
	}
</style>
