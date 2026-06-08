<script lang="ts">
	import IconBriefcase from '@tabler/icons-svelte-runes/icons/briefcase';
	import IconUser from '@tabler/icons-svelte-runes/icons/user';
	import IconMail from '@tabler/icons-svelte-runes/icons/mail';
	import IconPhone from '@tabler/icons-svelte-runes/icons/phone';
	import IconTrophy from '@tabler/icons-svelte-runes/icons/trophy';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconTag from '@tabler/icons-svelte-runes/icons/tag';
	import type { Deal, Stage } from '$lib/crm/types';
	import { formatDate, getStageColor } from './helpers';

	interface Props {
		deal: Deal;
		currentStage: Stage | null;
		isWon: boolean;
		isLost: boolean;
	}

	let { deal, currentStage, isWon, isLost }: Props = $props();
</script>

<div class="overview-grid">
	<!-- Deal Details Card -->
	<div class="info-card">
		<h3>
			<IconBriefcase size={18} />
			Deal Details
		</h3>
		<div class="info-grid">
			<div class="info-item">
				<span class="info-label">Stage</span>
				<span class="info-value" style:color={getStageColor(currentStage)}>
					{currentStage?.name || 'Unknown'}
				</span>
			</div>
			<div class="info-item">
				<span class="info-label">Days in Stage</span>
				<span class="info-value">{deal.days_in_stage} days</span>
			</div>
			<div class="info-item">
				<span class="info-label">Stage Changes</span>
				<span class="info-value">{deal.stage_changes_count}</span>
			</div>
			<div class="info-item">
				<span class="info-label">Currency</span>
				<span class="info-value">{deal.currency || 'USD'}</span>
			</div>
			{#if deal.source_channel}
				<div class="info-item">
					<span class="info-label">Source Channel</span>
					<span class="info-value">{deal.source_channel}</span>
				</div>
			{/if}
			{#if deal.source_campaign}
				<div class="info-item">
					<span class="info-label">Source Campaign</span>
					<span class="info-value">{deal.source_campaign}</span>
				</div>
			{/if}
			<div class="info-item">
				<span class="info-label">Created</span>
				<span class="info-value">{formatDate(deal.created_at)}</span>
			</div>
			<div class="info-item">
				<span class="info-label">Last Updated</span>
				<span class="info-value">{formatDate(deal.updated_at)}</span>
			</div>
			{#if deal.closed_at}
				<div class="info-item">
					<span class="info-label">Closed Date</span>
					<span class="info-value">{formatDate(deal.closed_at)}</span>
				</div>
			{/if}
		</div>
	</div>

	<!-- Contact Card -->
	<div class="info-card">
		<h3>
			<IconUser size={18} />
			Associated Contact
		</h3>
		{#if deal.contact}
			<div class="contact-preview">
				<div class="contact-avatar">
					{deal.contact.full_name?.charAt(0).toUpperCase() || '?'}
				</div>
				<div class="contact-info">
					<a href="/admin/crm/contacts/{deal.contact_id}" class="contact-name">
						{deal.contact.full_name}
					</a>
					{#if deal.contact.email}
						<a href="mailto:{deal.contact.email}" class="contact-email">
							<IconMail size={12} />
							{deal.contact.email}
						</a>
					{/if}
					{#if deal.contact.phone}
						<a href="tel:{deal.contact.phone}" class="contact-phone">
							<IconPhone size={12} />
							{deal.contact.phone}
						</a>
					{/if}
				</div>
			</div>
		{:else}
			<p class="empty-text">No contact associated</p>
		{/if}
	</div>

	<!-- Win/Loss Details -->
	{#if isWon && deal.won_details}
		<div class="info-card success">
			<h3>
				<IconTrophy size={18} />
				Won Details
			</h3>
			<p class="detail-text">{deal.won_details}</p>
		</div>
	{/if}

	{#if isLost && deal.lost_reason}
		<div class="info-card danger">
			<h3>
				<IconX size={18} />
				Lost Reason
			</h3>
			<p class="detail-text">{deal.lost_reason}</p>
		</div>
	{/if}

	<!-- Tags -->
	{#if deal.tags && deal.tags.length > 0}
		<div class="info-card">
			<h3>
				<IconTag size={18} />
				Tags
			</h3>
			<div class="tags-list">
				{#each deal.tags as tag (tag)}
					<span class="tag-pill">{tag}</span>
				{/each}
			</div>
		</div>
	{/if}
</div>

<style>
	.overview-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 20px;
	}

	@media (max-width: 900px) {
		.overview-grid {
			grid-template-columns: 1fr;
		}
	}

	.info-card {
		padding: 20px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 14px;
	}

	.info-card.success {
		border-color: rgba(34, 197, 94, 0.3);
		background: rgba(34, 197, 94, 0.05);
	}

	.info-card.danger {
		border-color: rgba(239, 68, 68, 0.3);
		background: rgba(239, 68, 68, 0.05);
	}

	.info-card h3 {
		display: flex;
		align-items: center;
		gap: 10px;
		margin: 0 0 16px;
		font-size: 1rem;
		font-weight: 600;
		color: white;
	}

	.info-card h3 :global(svg) {
		color: #f97316;
	}

	.info-card.success h3 :global(svg) {
		color: #4ade80;
	}

	.info-card.danger h3 :global(svg) {
		color: #f87171;
	}

	.info-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 12px;
	}

	.info-item {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.info-label {
		font-size: 0.7rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.info-value {
		font-size: 0.9rem;
		color: #e2e8f0;
	}

	.detail-text {
		margin: 0;
		color: #94a3b8;
		font-size: 0.9rem;
		line-height: 1.6;
	}

	.contact-preview {
		display: flex;
		gap: 16px;
		align-items: center;
	}

	.contact-avatar {
		width: 48px;
		height: 48px;
		border-radius: 50%;
		background: linear-gradient(135deg, #f97316, #ea580c);
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1.25rem;
		font-weight: 700;
		color: white;
		flex-shrink: 0;
	}

	.contact-info {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.contact-name {
		font-weight: 600;
		color: white;
		text-decoration: none;
	}

	.contact-name:hover {
		color: #f97316;
	}

	.contact-email,
	.contact-phone {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 0.8rem;
		color: #60a5fa;
		text-decoration: none;
	}

	.contact-email:hover,
	.contact-phone:hover {
		text-decoration: underline;
	}

	.tags-list {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
	}

	.tag-pill {
		padding: 4px 12px;
		background: rgba(249, 115, 22, 0.15);
		border-radius: 20px;
		font-size: 0.8rem;
		color: #f97316;
	}

	.empty-text {
		color: #64748b;
		margin: 0;
	}
</style>
