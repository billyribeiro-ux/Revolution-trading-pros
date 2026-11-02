<script lang="ts">
	import { IconCreditCard, IconReceipt, IconCalendar, IconChartBar } from '#lib/icons/index.js';
	import type { Member } from '#lib/api/members.js';
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
		<div class={['stat-icon', engagementIconClass]}>
			<IconChartBar size={24} />
		</div>
		<div class="stat-content">
			<div class="stat-value">{engagementScore}%</div>
			<div class={['stat-label', engagement.color]}>
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
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
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
		background: rgba(63, 185, 80, 0.15);
		color: var(--success-emphasis, #3fb950);
	}
	.stat-icon.blue {
		background: rgba(88, 166, 255, 0.15);
		color: var(--info-emphasis, #58a6ff);
	}
	.stat-icon.emerald-400 {
		background: rgba(52, 211, 153, 0.15);
		color: var(--success-emphasis, #3fb950);
	}
	.stat-icon.yellow-400 {
		background: rgba(210, 153, 34, 0.15);
		color: var(--warning-emphasis, #d29922);
	}
	.stat-icon.red-400 {
		background: rgba(248, 113, 113, 0.15);
		color: var(--error-emphasis, #f85149);
	}

	.stat-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
	}

	.stat-label {
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	@media (max-width: 1023.98px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
