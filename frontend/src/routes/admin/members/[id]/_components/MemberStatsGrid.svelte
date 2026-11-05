<script lang="ts">
	import { IconCreditCard, IconReceipt, IconCalendar, IconChartBar } from '#lib/icons/index.js';
	import type { MemberProfile } from '#lib/api/member-profile.js';
	import { recordedUsdCents, recordedWallClock } from './profile-display.js';
	let { profile }: { profile: MemberProfile } = $props();
</script>

<div class="stats-grid">
	<div class="stat-card">
		<IconCreditCard size={24} />
		<div>
			<div class="stat-value">{recordedUsdCents(profile.recorded_order_value.net_cents)}</div>
			<div class="stat-label">Recorded net USD order value</div>
			<p>
				{profile.recorded_order_value.order_count} qualifying orders · {profile.recorded_order_value
					.excluded_non_usd_orders} non-USD and {profile.recorded_order_value
					.excluded_unknown_currency_orders} unknown-currency orders excluded.
			</p>
		</div>
	</div>
	<div class="stat-card">
		<IconReceipt size={24} />
		<div>
			<div class="stat-value">{profile.member.active_subscriptions_count}</div>
			<div class="stat-label">Current active membership records</div>
		</div>
	</div>
	<div class="stat-card">
		<IconCalendar size={24} />
		<div>
			<div class="stat-value date">{recordedWallClock(profile.member.joined_at)}</div>
			<div class="stat-label">Recorded join date</div>
		</div>
	</div>
	<div class="stat-card">
		<IconChartBar size={24} />
		<div>
			<div class="stat-value">Not measured</div>
			<div class="stat-label">Engagement score</div>
			<p>{profile.engagement.reason}</p>
		</div>
	</div>
</div>
<p class="coverage">{profile.recorded_order_value.limitations} {profile.billing_coverage}</p>

<style>
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, minmax(0, 1fr));
		gap: 1rem;
	}
	.stat-card {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
		padding: 1.25rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 16px;
		overflow-wrap: anywhere;
	}
	.stat-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
	}
	.stat-label {
		text-transform: uppercase;
		font-size: 0.75rem;
		color: var(--text-secondary, #8b949e);
		margin-top: 0.4rem;
	}
	.date {
		font-size: 0.95rem;
	}
	.coverage,
	p {
		font-size: 0.8rem;
		color: var(--text-secondary, #8b949e);
		line-height: 1.6;
	}
	.coverage {
		margin-bottom: 2rem;
	}
	@media (max-width: 1100px) {
		.stats-grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}
	@media (max-width: 650px) {
		.stats-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
