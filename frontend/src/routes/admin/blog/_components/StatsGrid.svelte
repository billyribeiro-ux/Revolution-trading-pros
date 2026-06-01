<script lang="ts">
	import { IconChartBar, IconCheck, IconEdit, IconEye, IconTrendingUp } from '$lib/icons';

	type Stats = {
		total: number;
		published: number;
		draft: number;
		scheduled?: number;
		new_this_month?: number;
		total_views?: number;
		views_growth?: number;
	};

	type Props = {
		stats: Stats | null;
		formatNumber: (n: number) => string;
	};

	const { stats, formatNumber }: Props = $props();
</script>

{#if stats}
	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-icon"><IconChartBar size={24} /></div>
			<div class="stat-content">
				<div class="stat-value">{stats.total}</div>
				<div class="stat-label">Total Posts</div>
				<div class="stat-change">+{stats.new_this_month || 0} this month</div>
			</div>
		</div>
		<div class="stat-card published">
			<div class="stat-icon"><IconCheck size={24} /></div>
			<div class="stat-content">
				<div class="stat-value">{stats.published}</div>
				<div class="stat-label">Published</div>
				<div class="stat-change">{Math.round((stats.published / stats.total) * 100)}%</div>
			</div>
		</div>
		<div class="stat-card draft">
			<div class="stat-icon"><IconEdit size={24} /></div>
			<div class="stat-content">
				<div class="stat-value">{stats.draft}</div>
				<div class="stat-label">Drafts</div>
				<div class="stat-change">{stats.scheduled || 0} scheduled</div>
			</div>
		</div>
		<div class="stat-card views">
			<div class="stat-icon"><IconEye size={24} /></div>
			<div class="stat-content">
				<div class="stat-value">{formatNumber(stats.total_views || 0)}</div>
				<div class="stat-label">Total Views</div>
				<div class="stat-change">
					<IconTrendingUp size={16} />
					{stats.views_growth || 0}%
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.2);
		padding: 1.5rem;
		border-radius: 8px;
		display: flex;
		gap: 1rem;
		transition: all 0.3s;
	}

	.stat-card:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 20px rgba(0, 0, 0, 0.2);
	}

	.stat-card.published {
		background: linear-gradient(135deg, rgba(16, 185, 129, 0.1), rgba(16, 185, 129, 0.05));
		border-color: rgba(16, 185, 129, 0.3);
	}

	.stat-card.draft {
		background: linear-gradient(135deg, rgba(245, 158, 11, 0.1), rgba(245, 158, 11, 0.05));
		border-color: rgba(245, 158, 11, 0.3);
	}

	.stat-card.views {
		background: linear-gradient(135deg, rgba(59, 130, 246, 0.1), rgba(230, 184, 0, 0.05));
		border-color: rgba(59, 130, 246, 0.3);
	}

	.stat-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		color: #94a3b8;
	}

	.stat-content {
		flex: 1;
	}

	.stat-value {
		font-size: 2rem;
		font-weight: 700;
		color: var(--admin-text-primary);
		margin-bottom: 0.25rem;
	}

	.stat-label {
		color: var(--admin-text-muted);
		font-size: 0.9rem;
	}

	.stat-change {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		color: var(--admin-success);
		font-size: 0.85rem;
		margin-top: 0.5rem;
	}

	@media (max-width: 1023.98px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 767.98px) {
		.stats-grid {
			grid-template-columns: 1fr;
		}
	}

	@media (max-width: 639.98px) {
		.stat-card {
			padding: 1rem;
		}
	}

	@media (hover: none) and (pointer: coarse) {
		.stat-card {
			min-height: 80px;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.stat-card {
			transition: none;
		}

		.stat-card:hover {
			transform: none;
		}
	}

	@media (prefers-contrast: high) {
		.stat-card {
			border-width: 2px;
		}

		.stat-value {
			font-weight: 800;
		}
	}

	@media print {
		.stat-card {
			break-inside: avoid;
			box-shadow: none;
			border: 1px solid #ccc;
		}
	}
</style>
