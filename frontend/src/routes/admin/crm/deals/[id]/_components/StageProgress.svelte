<script lang="ts">
	import type { Deal, Pipeline, Stage } from '$lib/crm/types';

	interface Props {
		deal: Deal;
		pipeline: Pipeline;
		onSelectStage: (stage: Stage) => void;
	}

	let { deal, pipeline, onSelectStage }: Props = $props();

	// `Pipeline.stages` is typed as optional in `$lib/crm/types`. The parent
	// only mounts this component when `pipeline?.stages` is truthy, but the
	// type system can't follow that — narrow once here via $derived.
	let stages = $derived(pipeline.stages ?? []);
	let openStages = $derived(stages.filter((s) => !s.is_closed_won && !s.is_closed_lost));
</script>

<div class="stage-progress-section">
	<h3>Stage Progress</h3>
	<div class="stages-row">
		{#each openStages as stage (stage.id)}
			{const isCurrent = stage.id === deal?.stage_id}
			{const isPast =
				stages.findIndex((s) => s.id === stage.id) <
				stages.findIndex((s) => s.id === deal?.stage_id)}
			<button
				class={['stage-item', { current: isCurrent, past: isPast }]}
				onclick={() => !isCurrent && onSelectStage(stage)}
				disabled={isCurrent}
				style:--stage-color={stage.color || '#6366f1'}
			>
				<span class="stage-indicator"></span>
				<span class="stage-name">{stage.name}</span>
				{#if stage.probability > 0}
					<span class="stage-probability">{stage.probability}%</span>
				{/if}
			</button>
			{#if stages.indexOf(stage) < openStages.length - 1}
				<div class={['stage-connector', { active: isPast }]}></div>
			{/if}
		{/each}
	</div>
</div>

<style>
	.stage-progress-section {
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 16px;
		padding: 24px;
		margin-bottom: 24px;
	}

	.stage-progress-section h3 {
		margin: 0 0 20px;
		font-size: 1rem;
		color: white;
	}

	.stages-row {
		display: flex;
		align-items: center;
		gap: 0;
		overflow-x: auto;
		padding-bottom: 8px;
	}

	.stage-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 8px;
		padding: 12px 16px;
		background: transparent;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
		min-width: 100px;
	}

	.stage-item:disabled {
		cursor: default;
	}

	.stage-item:hover:not(:disabled) {
		background: rgba(249, 115, 22, 0.05);
		border-radius: 10px;
	}

	.stage-indicator {
		width: 16px;
		height: 16px;
		border-radius: 50%;
		background: #334155;
		transition: all 0.2s;
	}

	.stage-item.past .stage-indicator {
		background: var(--stage-color);
	}

	.stage-item.current .stage-indicator {
		background: var(--stage-color);
		box-shadow: 0 0 0 4px rgba(249, 115, 22, 0.2);
		transform: scale(1.2);
	}

	.stage-item .stage-name {
		font-size: 0.8rem;
		color: #64748b;
		text-align: center;
	}

	.stage-item.current .stage-name,
	.stage-item.past .stage-name {
		color: white;
		font-weight: 600;
	}

	.stage-item .stage-probability {
		font-size: 0.7rem;
		color: #475569;
	}

	.stage-connector {
		width: 32px;
		height: 2px;
		background: #334155;
	}

	.stage-connector.active {
		background: #f97316;
	}
</style>
