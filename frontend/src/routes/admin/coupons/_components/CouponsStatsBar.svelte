<script lang="ts">
	/**
	 * R27-C extraction (2026-05-20): three-column stats bar for the coupons
	 * list (total / active / inactive). Pure presentation — all numbers
	 * come pre-computed from the parent's `$derived` rollups.
	 */
	interface Props {
		total: number;
		active: number;
	}

	let { total, active }: Props = $props();
	let inactive = $derived(total - active);
</script>

<div class="stats-bar">
	<div class="stat-item">
		<span class="stat-value">{total}</span>
		<span class="stat-label">Total Coupons</span>
	</div>
	<div class="stat-item">
		<span class="stat-value stat-active">{active}</span>
		<span class="stat-label">Active</span>
	</div>
	<div class="stat-item">
		<span class="stat-value stat-inactive">{inactive}</span>
		<span class="stat-label">Inactive</span>
	</div>
</div>

<style>
	.stats-bar {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 3rem;
		padding: 1rem 1.5rem;
		background: var(--bg-elevated);
		border: 1px solid var(--border-muted);
		border-radius: 12px;
		margin-bottom: 1.5rem;
	}

	.stat-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.125rem;
	}

	.stat-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--text-primary);
		line-height: 1.2;
	}

	.stat-value.stat-active {
		color: var(--success-emphasis);
	}

	.stat-value.stat-inactive {
		color: var(--text-secondary);
	}

	.stat-label {
		font-size: 0.75rem;
		color: var(--text-tertiary);
		font-weight: 500;
	}

	@media (max-width: 767.98px) {
		.stats-bar {
			gap: 1.5rem;
			padding: 0.875rem 1rem;
		}

		.stat-value {
			font-size: 1.125rem;
		}
	}

	@media (max-width: 480px) {
		.stats-bar {
			gap: 1rem;
		}
	}
</style>
