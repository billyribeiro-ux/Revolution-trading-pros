<script lang="ts">
	import { IconUsers, IconCheck, IconX, IconMail } from '$lib/icons';

	interface Stats {
		total: number;
		subscribed: number;
		unsubscribed: number;
		bounced: number;
	}

	interface Props {
		stats: Stats;
	}

	let { stats }: Props = $props();
</script>

<div class="stats-grid">
	<div class="stat-card">
		<div class="stat-icon blue">
			<IconUsers size={24} />
		</div>
		<div class="stat-content">
			<div class="stat-value">{stats.total.toLocaleString()}</div>
			<div class="stat-label">Total Subscribers</div>
		</div>
	</div>
	<div class="stat-card">
		<div class="stat-icon emerald">
			<IconCheck size={24} />
		</div>
		<div class="stat-content">
			<div class="stat-value">{stats.subscribed.toLocaleString()}</div>
			<div class="stat-label">Active</div>
		</div>
	</div>
	<div class="stat-card">
		<div class="stat-icon slate">
			<IconX size={24} />
		</div>
		<div class="stat-content">
			<div class="stat-value">{stats.unsubscribed.toLocaleString()}</div>
			<div class="stat-label">Unsubscribed</div>
		</div>
	</div>
	<div class="stat-card">
		<div class="stat-icon red">
			<IconMail size={24} />
		</div>
		<div class="stat-content">
			<div class="stat-value">{stats.bounced.toLocaleString()}</div>
			<div class="stat-label">Bounced</div>
		</div>
	</div>
</div>

<style>
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
	}

	.stat-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-icon.blue {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}
	.stat-icon.emerald {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}
	.stat-icon.slate {
		background: rgba(148, 163, 184, 0.15);
		color: #94a3b8;
	}
	.stat-icon.red {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}

	.stat-content .stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-content .stat-label {
		font-size: 0.8125rem;
		color: #64748b;
	}

	@media (max-width: 767.98px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
