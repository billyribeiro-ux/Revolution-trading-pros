<script lang="ts">
	import { IconCreditCard, IconReceipt, IconCalendar, IconChartBar } from '$lib/icons';
	import type { Member } from '$lib/api/members';
	import { formatCurrency, formatDate, getEngagementLabel } from './helpers';

	interface Props {
		member: Member;
		engagementScore: number;
	}

	let { member, engagementScore }: Props = $props();

	let engagement = $derived(getEngagementLabel(engagementScore));
	// The `stat-icon` modifier mirrors the engagement color's Tailwind class
	// stem (e.g. `text-emerald-400` -> `emerald-400`) so the existing static
	// CSS map (.stat-icon.emerald-400 / .yellow-400 / .red-400) lights up.
	// Per CLAUDE.md "match-arm exhaustiveness vs metric labels": this is a
	// bounded set (only 4 outputs from getEngagementLabel), so no unbounded
	// class injection risk.
	let engagementIconClass = $derived(engagement.color.replace('text-', ''));
</script>

<div class="stats-grid">
	<div class="stat-card">
		<div class="stat-icon purple">
			<IconCreditCard size={24} />
		</div>
		<div class="stat-content">
			<div class="stat-value">{formatCurrency(member.total_spent)}</div>
			<div class="stat-label">Lifetime Value</div>
		</div>
	</div>
	<div class="stat-card">
		<div class="stat-icon emerald">
			<IconReceipt size={24} />
		</div>
		<div class="stat-content">
			<div class="stat-value">{member.active_subscriptions_count}</div>
			<div class="stat-label">Active Subscriptions</div>
		</div>
	</div>
	<div class="stat-card">
		<div class="stat-icon blue">
			<IconCalendar size={24} />
		</div>
		<div class="stat-content">
			<div class="stat-value">{formatDate(member.joined_at)}</div>
			<div class="stat-label">Member Since</div>
		</div>
	</div>
	<div class="stat-card">
		<div class="stat-icon {engagementIconClass}">
			<IconChartBar size={24} />
		</div>
		<div class="stat-content">
			<div class="stat-value">{engagementScore}%</div>
			<div class="stat-label {engagement.color}">
				{engagement.label}
			</div>
		</div>
	</div>
</div>

<style>
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
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

	.stat-icon.purple {
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-400);
	}
	.stat-icon.emerald {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}
	.stat-icon.blue {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}
	.stat-icon.emerald-400 {
		background: rgba(52, 211, 153, 0.15);
		color: #34d399;
	}
	.stat-icon.yellow-400 {
		background: rgba(251, 191, 36, 0.15);
		color: #fbbf24;
	}
	.stat-icon.red-400 {
		background: rgba(248, 113, 113, 0.15);
		color: #f87171;
	}

	.stat-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-label {
		font-size: 0.75rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	@media (max-width: 1023.98px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
