<script lang="ts">
	import IconMail from '@tabler/icons-svelte-runes/icons/mail';
	import IconPhone from '@tabler/icons-svelte-runes/icons/phone';
	import IconBuilding from '@tabler/icons-svelte-runes/icons/building';
	import IconBriefcase from '@tabler/icons-svelte-runes/icons/briefcase';
	import IconWorld from '@tabler/icons-svelte-runes/icons/world';
	import IconCalendar from '@tabler/icons-svelte-runes/icons/calendar';
	import IconClock from '@tabler/icons-svelte-runes/icons/clock';
	import IconTag from '@tabler/icons-svelte-runes/icons/tag';
	import IconList from '@tabler/icons-svelte-runes/icons/list';
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import type { Contact } from './types';
	import { formatDate } from './helpers';

	interface Props {
		contact: Contact;
		onAddTag: () => void;
		onRemoveTag: (tagId: string) => void;
		onAddList: () => void;
		onRemoveFromList: (listId: string) => void;
	}

	let { contact, onAddTag, onRemoveTag, onAddList, onRemoveFromList }: Props = $props();
</script>

<div class="overview-grid">
	<!-- Contact Info Card -->
	<div class="info-card">
		<h3>Contact Information</h3>
		<div class="info-grid">
			<div class="info-item">
				<IconMail size={16} />
				<div>
					<span class="info-label">Email</span>
					<a href="mailto:{contact.email || ''}" class="info-value link">{contact.email || ''}</a>
				</div>
			</div>
			{#if contact.phone}
				<div class="info-item">
					<IconPhone size={16} />
					<div>
						<span class="info-label">Phone</span>
						<a href="tel:{contact.phone}" class="info-value link">{contact.phone}</a>
					</div>
				</div>
			{/if}
			{#if contact.mobile}
				<div class="info-item">
					<IconPhone size={16} />
					<div>
						<span class="info-label">Mobile</span>
						<a href="tel:{contact.mobile}" class="info-value link">{contact.mobile}</a>
					</div>
				</div>
			{/if}
			{#if contact.company_name}
				<div class="info-item">
					<IconBuilding size={16} />
					<div>
						<span class="info-label">Company</span>
						<span class="info-value">{contact.company_name}</span>
					</div>
				</div>
			{/if}
			{#if contact.job_title}
				<div class="info-item">
					<IconBriefcase size={16} />
					<div>
						<span class="info-label">Job Title</span>
						<span class="info-value">{contact.job_title}</span>
					</div>
				</div>
			{/if}
			{#if contact.website}
				<div class="info-item">
					<IconWorld size={16} />
					<div>
						<span class="info-label">Website</span>
						<a href={contact.website} target="_blank" class="info-value link">{contact.website}</a>
					</div>
				</div>
			{/if}
			{#if contact.city || contact.country}
				<div class="info-item">
					<IconWorld size={16} />
					<div>
						<span class="info-label">Location</span>
						<span class="info-value">
							{[contact.city, contact.state, contact.country].filter(Boolean).join(', ')}
						</span>
					</div>
				</div>
			{/if}
			<div class="info-item">
				<IconCalendar size={16} />
				<div>
					<span class="info-label">Created</span>
					<span class="info-value">{formatDate(contact.created_at)}</span>
				</div>
			</div>
			<div class="info-item">
				<IconClock size={16} />
				<div>
					<span class="info-label">Last Activity</span>
					<span class="info-value">{formatDate(contact.last_activity_at)}</span>
				</div>
			</div>
		</div>
	</div>

	<!-- Tags Card -->
	<div class="info-card">
		<div class="card-header">
			<h3>
				<IconTag size={18} />
				Tags
			</h3>
			<button class="btn-add" onclick={onAddTag}>
				<IconPlus size={14} />
				Add
			</button>
		</div>
		<div class="tags-list">
			{#if contact.tags && contact.tags.length > 0}
				{#each contact.tags as tag (tag.id)}
					{const tagColor = tag.color || '#E6B800'}
					<span class="tag-pill" style:background-color={`${tagColor}20`} style:color={tagColor}>
						{tag.name}
						<button class="tag-remove" onclick={() => onRemoveTag(tag.id)}>
							<IconX size={12} />
						</button>
					</span>
				{/each}
			{:else}
				<p class="empty-text">No tags assigned</p>
			{/if}
		</div>
	</div>

	<!-- Lists Card -->
	<div class="info-card">
		<div class="card-header">
			<h3>
				<IconList size={18} />
				Lists
			</h3>
			<button class="btn-add" onclick={onAddList}>
				<IconPlus size={14} />
				Add
			</button>
		</div>
		<div class="lists-section">
			{#if contact.lists && contact.lists.length > 0}
				{#each contact.lists as list (list.id)}
					<div class="list-item">
						<IconList size={16} />
						<span>{list.name}</span>
						<button class="list-remove" onclick={() => onRemoveFromList(list.id)}>
							<IconX size={12} />
						</button>
					</div>
				{/each}
			{:else}
				<p class="empty-text">Not added to any list</p>
			{/if}
		</div>
	</div>

	<!-- Engagement Card -->
	<div class="info-card">
		<h3>Engagement Metrics</h3>
		<div class="metrics-grid">
			<div class="metric-item">
				<span class="metric-label">Total Sessions</span>
				<span class="metric-value">{contact.total_sessions}</span>
			</div>
			<div class="metric-item">
				<span class="metric-label">Avg Engagement</span>
				<span class="metric-value">{contact.avg_engagement_score?.toFixed(1) || 0}</span>
			</div>
			<div class="metric-item">
				<span class="metric-label">Avg Intent</span>
				<span class="metric-value">{contact.avg_intent_score?.toFixed(1) || 0}</span>
			</div>
			<div class="metric-item">
				<span class="metric-label">Friction Events</span>
				<span class="metric-value">{contact.friction_events_count || 0}</span>
			</div>
			<div class="metric-item">
				<span class="metric-label">Deals</span>
				<span class="metric-value">{contact.deals_count || 0}</span>
			</div>
			<div class="metric-item">
				<span class="metric-label">MRR</span>
				<span class="metric-value">${contact.subscription_mrr?.toFixed(0) || 0}</span>
			</div>
		</div>
	</div>
</div>

<style>
	.overview-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 20px;
	}

	.info-card {
		padding: 20px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 14px;
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
		color: var(--primary-500);
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 16px;
	}

	.card-header h3 {
		margin: 0;
	}

	.btn-add {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 6px 12px;
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.3);
		border-radius: 6px;
		color: var(--primary-500);
		font-size: 0.75rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-add:hover {
		background: rgba(230, 184, 0, 0.2);
	}

	.info-grid {
		display: grid;
		gap: 12px;
	}

	.info-item {
		display: flex;
		align-items: flex-start;
		gap: 12px;
		padding: 10px;
		background: #0f172a;
		border-radius: 8px;
	}

	.info-item :global(svg) {
		color: #64748b;
		flex-shrink: 0;
		margin-top: 2px;
	}

	.info-item > div {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.info-label {
		font-size: 0.7rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.info-value {
		font-size: 0.875rem;
		color: #e2e8f0;
	}

	.info-value.link {
		color: #60a5fa;
		text-decoration: none;
	}

	.info-value.link:hover {
		text-decoration: underline;
	}

	.tags-list {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
	}

	.tag-pill {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 6px 12px;
		border-radius: 20px;
		font-size: 0.8rem;
		font-weight: 500;
	}

	.tag-remove {
		display: flex;
		padding: 2px;
		background: transparent;
		border: none;
		color: inherit;
		opacity: 0.6;
		cursor: pointer;
	}

	.tag-remove:hover {
		opacity: 1;
	}

	.lists-section {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.list-item {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 10px 12px;
		background: #0f172a;
		border-radius: 8px;
		font-size: 0.875rem;
		color: #e2e8f0;
	}

	.list-item :global(svg) {
		color: #64748b;
	}

	.list-remove {
		margin-left: auto;
		display: flex;
		padding: 4px;
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
		opacity: 0;
		transition: opacity 0.2s;
	}

	.list-item:hover .list-remove {
		opacity: 1;
	}

	.list-remove:hover {
		color: #f87171;
	}

	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 12px;
	}

	.metric-item {
		display: flex;
		flex-direction: column;
		gap: 4px;
		padding: 12px;
		background: #0f172a;
		border-radius: 8px;
	}

	.metric-label {
		font-size: 0.7rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.metric-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: white;
	}

	.empty-text {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0;
	}

	@media (max-width: 900px) {
		.overview-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
