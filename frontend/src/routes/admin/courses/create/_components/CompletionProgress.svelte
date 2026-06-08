<script lang="ts">
	// Compact "X% Complete" progress widget that sits above the tab content.
	// Behaviour preserved 1:1 from courses/create +page.svelte
	// (originally lines 1696-1708).
	interface Props {
		percentage: number;
		completed: number;
		total: number;
		moduleCount: number;
		durationHours: number;
	}

	let { percentage, completed, total, moduleCount, durationHours }: Props = $props();
</script>

<div class="completion-progress">
	<div class="progress-bar">
		<div class="progress-fill" style:width={`${percentage}%`}></div>
	</div>
	<div class="progress-details">
		<span class="progress-text">{percentage}% Complete</span>
		<span class="progress-items"
			>{completed}/{total} items • {moduleCount} modules • {durationHours}h</span
		>
	</div>
</div>

<style>
	/* Mirrors the original +page.svelte rules so the bar renders identically. */
	.completion-progress {
		max-width: 500px;
		margin: 0 auto 1.5rem;
	}

	.progress-bar {
		height: 8px;
		background: rgba(148, 163, 184, 0.2);
		border-radius: 4px;
		overflow: hidden;
		margin-bottom: 0.5rem;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, var(--primary-500), var(--primary-600));
		transition: width 0.5s ease;
	}

	.progress-details {
		display: flex;
		justify-content: space-between;
		font-size: 0.8125rem;
		color: #64748b;
	}
</style>
