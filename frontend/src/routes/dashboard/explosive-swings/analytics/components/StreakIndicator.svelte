<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * StreakIndicator Component - Win/Loss Streak Display
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays current and historical trading streak information
	 * @version 1.0.0
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA | Svelte 5
	 */
	import type { StreakAnalysis } from '../analytics.state.svelte';

	interface Props {
		streak: StreakAnalysis;
		isLoading?: boolean;
	}

	const { streak, isLoading = false }: Props = $props();

	// Determine streak type styling
	const streakColor = $derived(
		streak.current_streak_type === 'WIN' ? 'profit' :
		streak.current_streak_type === 'LOSS' ? 'loss' : 'neutral'
	);

	const streakLabel = $derived(
		streak.current_streak_type === 'WIN' ? 'Win Streak' :
		streak.current_streak_type === 'LOSS' ? 'Loss Streak' : 'No Active Streak'
	);

	const streakIcon = $derived(
		streak.current_streak_type === 'WIN' ? 'fire' :
		streak.current_streak_type === 'LOSS' ? 'snowflake' : 'minus'
	);
</script>

<div class="streak-indicator" role="region" aria-label="Trading streak information">
	<h3 class="section-title">Streak Analysis</h3>

	{#if isLoading}
		<div class="skeleton-content">
			<div class="skel-main"></div>
			<div class="skel-stats"></div>
		</div>
	{:else}
		<!-- Current Streak Display -->
		<div class="current-streak {streakColor}">
			<div class="streak-icon">
				{#if streakIcon === 'fire'}
					<svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<path d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z"/>
					</svg>
				{:else if streakIcon === 'snowflake'}
					<svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<line x1="2" y1="12" x2="22" y2="12"/>
						<line x1="12" y1="2" x2="12" y2="22"/>
						<path d="m20 16-4-4 4-4"/>
						<path d="m4 8 4 4-4 4"/>
						<path d="m16 4-4 4-4-4"/>
						<path d="m8 20 4-4 4 4"/>
					</svg>
				{:else}
					<svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<line x1="5" y1="12" x2="19" y2="12"/>
					</svg>
				{/if}
			</div>

			<div class="streak-content">
				<div class="streak-value">
					{streak.current_streak}
				</div>
				<div class="streak-label">{streakLabel}</div>
			</div>
		</div>

		<!-- Streak Stats -->
		<div class="streak-stats">
			<div class="stat-item">
				<div class="stat-icon profit">
					<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<polyline points="23 6 13.5 15.5 8.5 10.5 1 18"/>
						<polyline points="17 6 23 6 23 12"/>
					</svg>
				</div>
				<div class="stat-info">
					<span class="stat-value profit">{streak.max_win_streak}</span>
					<span class="stat-label">Best Win Streak</span>
				</div>
			</div>

			<div class="stat-item">
				<div class="stat-icon loss">
					<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<polyline points="23 18 13.5 8.5 8.5 13.5 1 6"/>
						<polyline points="17 18 23 18 23 12"/>
					</svg>
				</div>
				<div class="stat-info">
					<span class="stat-value loss">{streak.max_loss_streak}</span>
					<span class="stat-label">Worst Loss Streak</span>
				</div>
			</div>
		</div>

		<!-- Average Streaks -->
		<div class="avg-streaks">
			<span class="avg-label">Avg Win Streak: {streak.avg_win_streak.toFixed(1)}</span>
			<span class="avg-label">Avg Loss Streak: {streak.avg_loss_streak.toFixed(1)}</span>
		</div>

		<!-- Streak Visual -->
		{#if streak.current_streak > 0}
			<div class="streak-visual">
				{#each Array(Math.min(streak.current_streak, 10)) as _, i}
					<div
						class="streak-dot {streakColor}"
						style="animation-delay: {i * 0.05}s"
					></div>
				{/each}
				{#if streak.current_streak > 10}
					<span class="streak-more">+{streak.current_streak - 10}</span>
				{/if}
			</div>
		{/if}
	{/if}
</div>

<style>
	.streak-indicator {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 12px;
		padding: 20px;
		box-shadow: var(--shadow-sm);
		height: 100%;
		display: flex;
		flex-direction: column;
	}

	.section-title {
		font-size: 16px;
		font-weight: 700;
		color: var(--color-text-primary);
		margin: 0 0 16px 0;
	}

	/* Current Streak */
	.current-streak {
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 16px;
		border-radius: 10px;
		margin-bottom: 16px;
	}

	.current-streak.profit {
		background: var(--color-profit-bg);
		border: 1px solid var(--color-profit-border, rgba(16, 185, 129, 0.2));
	}

	.current-streak.loss {
		background: var(--color-loss-bg);
		border: 1px solid var(--color-loss-border, rgba(239, 68, 68, 0.2));
	}

	.current-streak.neutral {
		background: var(--color-bg-subtle);
		border: 1px solid var(--color-border-subtle);
	}

	.streak-icon {
		width: 48px;
		height: 48px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 12px;
		background: var(--color-bg-card);
	}

	.current-streak.profit .streak-icon {
		color: var(--color-profit);
	}

	.current-streak.loss .streak-icon {
		color: var(--color-loss);
	}

	.current-streak.neutral .streak-icon {
		color: var(--color-text-tertiary);
	}

	.streak-content {
		flex: 1;
	}

	.streak-value {
		font-size: 32px;
		font-weight: 800;
		line-height: 1;
		margin-bottom: 4px;
	}

	.current-streak.profit .streak-value {
		color: var(--color-profit);
	}

	.current-streak.loss .streak-value {
		color: var(--color-loss);
	}

	.current-streak.neutral .streak-value {
		color: var(--color-text-secondary);
	}

	.streak-label {
		font-size: 13px;
		font-weight: 600;
		color: var(--color-text-secondary);
	}

	/* Streak Stats */
	.streak-stats {
		display: flex;
		gap: 12px;
		margin-bottom: 16px;
	}

	.stat-item {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 12px;
		background: var(--color-bg-subtle);
		border-radius: 8px;
	}

	.stat-icon {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 6px;
	}

	.stat-icon.profit {
		background: var(--color-profit-bg);
		color: var(--color-profit);
	}

	.stat-icon.loss {
		background: var(--color-loss-bg);
		color: var(--color-loss);
	}

	.stat-info {
		display: flex;
		flex-direction: column;
	}

	.stat-value {
		font-size: 18px;
		font-weight: 700;
		line-height: 1.2;
	}

	.stat-value.profit {
		color: var(--color-profit);
	}

	.stat-value.loss {
		color: var(--color-loss);
	}

	.stat-label {
		font-size: 10px;
		font-weight: 600;
		color: var(--color-text-tertiary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	/* Average Streaks */
	.avg-streaks {
		display: flex;
		justify-content: space-between;
		padding: 10px 12px;
		background: var(--color-bg-subtle);
		border-radius: 6px;
		font-size: 12px;
		margin-bottom: 16px;
	}

	.avg-label {
		color: var(--color-text-tertiary);
	}

	/* Streak Visual */
	.streak-visual {
		display: flex;
		align-items: center;
		gap: 6px;
		margin-top: auto;
	}

	.streak-dot {
		width: 12px;
		height: 12px;
		border-radius: 50%;
		animation: pulse 1s ease-in-out infinite;
	}

	.streak-dot.profit {
		background: var(--color-profit);
	}

	.streak-dot.loss {
		background: var(--color-loss);
	}

	.streak-dot.neutral {
		background: var(--color-text-tertiary);
	}

	.streak-more {
		font-size: 11px;
		font-weight: 600;
		color: var(--color-text-tertiary);
		margin-left: 4px;
	}

	@keyframes pulse {
		0%, 100% {
			opacity: 1;
			transform: scale(1);
		}
		50% {
			opacity: 0.7;
			transform: scale(0.9);
		}
	}

	/* Skeleton */
	.skeleton-content {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.skel-main {
		height: 80px;
		background: linear-gradient(90deg, var(--color-bg-subtle) 25%, var(--color-bg-muted) 50%, var(--color-bg-subtle) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 10px;
	}

	.skel-stats {
		height: 60px;
		background: linear-gradient(90deg, var(--color-bg-subtle) 25%, var(--color-bg-muted) 50%, var(--color-bg-subtle) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 8px;
	}

	@keyframes shimmer {
		0% { background-position: 200% 0; }
		100% { background-position: -200% 0; }
	}

	@media (max-width: 640px) {
		.streak-indicator {
			padding: 16px;
		}

		.streak-stats {
			flex-direction: column;
		}

		.streak-value {
			font-size: 28px;
		}
	}
</style>
