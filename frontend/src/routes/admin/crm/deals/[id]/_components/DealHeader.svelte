<script lang="ts">
	import { useCrmAccess } from '#lib/crm/access.svelte.js';
	const access = useCrmAccess();
	import IconBriefcase from '@tabler/icons-svelte-runes/icons/briefcase';
	import IconEdit from '@tabler/icons-svelte-runes/icons/edit';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import IconUser from '@tabler/icons-svelte-runes/icons/user';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconRefresh from '@tabler/icons-svelte-runes/icons/refresh';
	import IconTrophy from '@tabler/icons-svelte-runes/icons/trophy';
	import IconFlag from '@tabler/icons-svelte-runes/icons/flag';
	import type { Deal, Stage } from '#lib/crm/types.js';
	import { getStageColor, getPriorityColor, getStatusBadge } from './helpers';

	interface Props {
		deal: Deal;
		dealId: string;
		currentStage: Stage | null;
		isOpen: boolean;
		onRefresh: () => void;
		onOpenWin: () => void;
		onOpenLose: () => void;
		onDelete: () => void;
	}

	let { deal, dealId, currentStage, isOpen, onRefresh, onOpenWin, onOpenLose, onDelete }: Props =
		$props();

	let stageColor = $derived(getStageColor(currentStage));
	let priorityColor = $derived(getPriorityColor(deal.priority));
	let statusBadge = $derived(getStatusBadge(deal.status));
</script>

<header class="deal-header">
	<div class="deal-identity">
		<div class="deal-icon" style:background={`${stageColor}20`} style:color={stageColor}>
			<IconBriefcase size={28} />
		</div>
		<div class="deal-info">
			<div class="name-row">
				<h1>{deal.name}</h1>
				<span
					class="status-badge"
					style:background={statusBadge.bg}
					style:color={statusBadge.color}
				>
					{statusBadge.text}
				</span>
			</div>
			<div class="deal-meta-row">
				{#if deal.contact}
					<span class="meta-item">
						<IconUser size={14} />
						{deal.contact.full_name}
					</span>
				{/if}
				{#if currentStage}
					<span class="meta-item">
						<IconFlag size={14} style={`color: ${stageColor}`} />
						{currentStage.name}
					</span>
				{/if}
				{#if deal.priority && deal.priority !== 'medium'}
					<span class="meta-item priority" style:color={priorityColor}>
						{deal.priority} priority
					</span>
				{/if}
			</div>
		</div>
	</div>

	<div class="header-actions">
		<button class="btn-icon" onclick={onRefresh} title="Refresh">
			<IconRefresh size={18} />
		</button>
		{#if isOpen}
			<button
				class="btn-success"
				disabled={!access.can('POST', '/deals/{id}/win')}
				onclick={onOpenWin}
			>
				<IconTrophy size={18} />
				Mark Won
			</button>
			<button
				class="btn-danger-outline"
				disabled={!access.can('POST', '/deals/{id}/lose')}
				onclick={onOpenLose}
			>
				<IconX size={18} />
				Mark Lost
			</button>
		{/if}
		{#if access.page(`/admin/crm/deals/${dealId}/edit`)}<a
				href="/admin/crm/deals/{dealId}/edit"
				class="btn-primary"
			>
				<IconEdit size={18} />
				Edit Deal
			</a>{/if}
		<button
			class="btn-icon danger"
			disabled={!access.can('DELETE', '/deals/{id}')}
			onclick={onDelete}
			title="Delete"
		>
			<IconTrash size={18} />
		</button>
	</div>
</header>

<style>
	.deal-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: 24px;
		margin-bottom: 24px;
		flex-wrap: wrap;
	}

	.deal-identity {
		display: flex;
		gap: 16px;
	}

	.deal-icon {
		width: 64px;
		height: 64px;
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.deal-info {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.name-row {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.name-row h1 {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 700;
		color: #ffffff;
	}

	.status-badge {
		padding: 4px 12px;
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.deal-meta-row {
		display: flex;
		gap: 16px;
		flex-wrap: wrap;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 0.85rem;
		color: var(--text-secondary, #8b949e);
	}

	.meta-item.priority {
		text-transform: capitalize;
		font-weight: 600;
	}

	.header-actions {
		display: flex;
		gap: 12px;
		flex-wrap: wrap;
	}

	.btn-icon {
		width: 44px;
		height: 44px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 10px;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: rgba(210, 153, 34, 0.1);
		color: var(--warning-emphasis, #d29922);
		border-color: rgba(210, 153, 34, 0.3);
	}

	.btn-icon.danger:hover {
		background: rgba(248, 81, 73, 0.1);
		color: var(--error-emphasis, #f85149);
		border-color: rgba(248, 81, 73, 0.3);
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 20px;
		background: var(--primary-500, #e6b800);
		color: var(--bg-base, #0d1117);
		border: none;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-success {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 20px;
		background: var(--success-base, #2ea043);
		color: #ffffff;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-danger-outline {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 20px;
		background: transparent;
		border: 1px solid rgba(248, 81, 73, 0.5);
		color: var(--error-emphasis, #f85149);
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-danger-outline:hover {
		background: rgba(248, 81, 73, 0.1);
	}

	@media (max-width: 767.98px) {
		.deal-header {
			flex-direction: column;
		}

		.header-actions {
			width: 100%;
			flex-wrap: wrap;
		}
	}
</style>
