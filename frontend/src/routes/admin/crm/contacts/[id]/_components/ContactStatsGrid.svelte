<script lang="ts">
	import IconTrendingUp from '@tabler/icons-svelte-runes/icons/trending-up';
	import IconChartBar from '@tabler/icons-svelte-runes/icons/chart-bar';
	import IconActivity from '@tabler/icons-svelte-runes/icons/activity';
	import IconMailOpen from '@tabler/icons-svelte-runes/icons/mail-opened';
	import IconClick from '@tabler/icons-svelte-runes/icons/click';
	import IconCurrencyDollar from '@tabler/icons-svelte-runes/icons/currency-dollar';
	import type { Contact } from './types';

	interface Props {
		contact: Contact;
	}

	let { contact }: Props = $props();
</script>

<section class="quick-stats">
	<div class="stat-box">
		<IconTrendingUp size={20} class="stat-icon green" />
		<div class="stat-info">
			<span class="stat-value">{contact.lead_score}</span>
			<span class="stat-label">Lead Score</span>
		</div>
	</div>
	<div class="stat-box">
		<IconChartBar size={20} class="stat-icon blue" />
		<div class="stat-info">
			<span class="stat-value">{contact.health_score}</span>
			<span class="stat-label">Health Score</span>
		</div>
	</div>
	<div class="stat-box">
		<IconActivity size={20} class="stat-icon gold" />
		<div class="stat-info">
			<span class="stat-value">{contact.engagement_score}</span>
			<span class="stat-label">Engagement</span>
		</div>
	</div>
	<div class="stat-box">
		<IconMailOpen size={20} class="stat-icon cyan" />
		<div class="stat-info">
			<span class="stat-value">{contact.email_opens}</span>
			<span class="stat-label">Email Opens</span>
		</div>
	</div>
	<div class="stat-box">
		<IconClick size={20} class="stat-icon amber" />
		<div class="stat-info">
			<span class="stat-value">{contact.email_clicks}</span>
			<span class="stat-label">Email Clicks</span>
		</div>
	</div>
	<div class="stat-box">
		<IconCurrencyDollar size={20} class="stat-icon emerald" />
		<div class="stat-info">
			<span class="stat-value">${(contact.lifetime_value || 0).toLocaleString()}</span>
			<span class="stat-label">Lifetime Value</span>
		</div>
	</div>
</section>

<style>
	.quick-stats {
		display: grid;
		grid-template-columns: repeat(6, 1fr);
		gap: 12px;
		margin-bottom: 24px;
	}

	.stat-box {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 16px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 12px;
	}

	.stat-box :global(.stat-icon) {
		flex-shrink: 0;
	}

	.stat-box :global(.green) {
		color: #4ade80;
	}
	.stat-box :global(.blue) {
		color: #60a5fa;
	}
	.stat-box :global(.gold) {
		color: var(--primary-500);
	}
	.stat-box :global(.cyan) {
		color: #22d3ee;
	}
	.stat-box :global(.amber) {
		color: #fbbf24;
	}
	.stat-box :global(.emerald) {
		color: #34d399;
	}

	.stat-info {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.stat-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: white;
	}

	.stat-label {
		font-size: 0.7rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	@media (max-width: 1199.98px) {
		.quick-stats {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: 900px) {
		.quick-stats {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 639.98px) {
		.quick-stats {
			grid-template-columns: 1fr;
		}
	}
</style>
