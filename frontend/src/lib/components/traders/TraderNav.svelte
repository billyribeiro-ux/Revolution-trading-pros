<!--
	Trader Navigation Pills Component
	═══════════════════════════════════════════════════════════════════════════
	Reusable navigation for trader sub-pages.
	
	@version 1.0.0
-->
<script lang="ts">
	import { traderSubPages } from '$lib/data/traders';
	
	// ICT 7 FIX: Svelte 5 $props() syntax (Nov 2025 best practice)
	interface Props {
		traderSlug: string;
		currentPath?: string;
	}
	
	let { traderSlug, currentPath = '' }: Props = $props();
</script>

<div class="trader_pills">
	{#each traderSubPages as pill}
		<div class="trader_pill">
			<a 
				href="/dashboard/day-trading-room/meet-the-traders/{traderSlug}{pill.path}"
				class:active={currentPath === pill.path}
			>
				{pill.label}
			</a>
		</div>
	{/each}
</div>

<style>
	.trader_pills {
		display: flex;
		flex-wrap: wrap;
		gap: 10px;
		margin-bottom: 30px;
	}

	.trader_pill {
		display: inline-block;
	}

	.trader_pill a {
		display: inline-block;
		padding: 10px 20px;
		background: #f5f5f5;
		color: #333;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		border-radius: 25px;
		border: 1px solid #e0e0e0;
		transition: all 0.2s ease;
	}

	.trader_pill a:hover {
		background: #F69532;
		color: #fff;
		border-color: #F69532;
	}

	.trader_pill a.active {
		background: #F69532;
		color: #fff;
		border-color: #F69532;
	}

	@media (max-width: 767px) {
		.trader_pills {
			justify-content: center;
		}
	}
</style>
