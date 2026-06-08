<script lang="ts">
	// Sidebar quality-score widget.
	// Behaviour preserved 1:1 from courses/create +page.svelte (sidebar block,
	// originally lines 1853-1883).

	interface ValidationResult {
		field: string;
		status: 'good' | 'warning' | 'error';
		message: string;
		score: number;
	}

	interface Props {
		overallScore: number;
		validationResults: ValidationResult[];
	}

	let { overallScore, validationResults }: Props = $props();
</script>

<div class="validation-card">
	<h3>Quality Score</h3>
	<div class="score-circle">
		<svg aria-hidden="true" viewBox="0 0 100 100">
			<circle cx="50" cy="50" r="45" fill="none" stroke="#1e293b" stroke-width="8" />
			<circle
				cx="50"
				cy="50"
				r="45"
				fill="none"
				stroke={overallScore > 70 ? '#10b981' : overallScore > 40 ? '#f59e0b' : '#ef4444'}
				stroke-width="8"
				stroke-dasharray={`${overallScore * 2.83} 283`}
				transform="rotate(-90 50 50)"
				style="transition: stroke-dasharray 0.5s ease"
			/>
		</svg>
		<div class="score-text">{overallScore}</div>
	</div>

	{#if validationResults.length > 0}
		<div class="validation-items">
			{#each validationResults as result (result.message)}
				<div class={['validation-item', result.status]}>
					<span class="item-label">{result.message}</span>
					<span class="item-score">+{result.score}</span>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	/* Mirrors the original +page.svelte rules so the card renders identically. */
	.validation-card {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		padding: 1.25rem;
		backdrop-filter: blur(10px);
	}

	.validation-card h3 {
		font-size: 1.05rem;
		color: #f1f5f9;
		margin: 0 0 1rem 0;
		font-weight: 600;
	}

	.score-circle {
		position: relative;
		width: 120px;
		height: 120px;
		margin: 0 auto 1.25rem;
	}

	.score-circle svg {
		width: 100%;
		height: 100%;
		transform: rotate(-90deg);
	}

	.score-text {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.validation-items {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.validation-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.625rem 0.875rem;
		background: rgba(148, 163, 184, 0.05);
		border-radius: 6px;
		font-size: 0.875rem;
	}

	.validation-item.good {
		background: rgba(16, 185, 129, 0.1);
		color: #10b981;
		border: 1px solid rgba(16, 185, 129, 0.2);
	}

	.validation-item.warning {
		background: rgba(245, 158, 11, 0.1);
		color: #f59e0b;
		border: 1px solid rgba(245, 158, 11, 0.2);
	}

	.validation-item.error {
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
		border: 1px solid rgba(239, 68, 68, 0.2);
	}

	.item-score {
		font-weight: 600;
		font-size: 0.75rem;
	}
</style>
