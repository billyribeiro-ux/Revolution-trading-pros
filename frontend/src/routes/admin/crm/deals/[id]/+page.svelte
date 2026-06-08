<!--
	/admin/crm/deals/[id] - Deal Detail Page
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- Deal profile with stage progression
	- Contact association
	- Activity timeline
	- Notes management
	- Stage change history
	- Win/Loss actions
	- Full Svelte 5 $state/$derived/$effect reactivity

	R17-C 2026-05-20: Extracted 11 components + helpers.ts into _components/
	to bring +page.svelte under 250 LOC. Pattern mirrors R16-C members/[id].
-->

<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import IconArrowLeft from '@tabler/icons-svelte-runes/icons/arrow-left';
	import IconBriefcase from '@tabler/icons-svelte-runes/icons/briefcase';
	import IconNotes from '@tabler/icons-svelte-runes/icons/notes';
	import IconAlertTriangle from '@tabler/icons-svelte-runes/icons/alert-triangle';
	import IconHistory from '@tabler/icons-svelte-runes/icons/history';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	import { crmAPI } from '$lib/api/crm';
	import type { Deal, Pipeline, Stage, TimelineEvent } from '$lib/crm/types';
	import { api } from '$lib/api/config';

	import DealHeader from './_components/DealHeader.svelte';
	import DealValueCard from './_components/DealValueCard.svelte';
	import StageProgress from './_components/StageProgress.svelte';
	import OverviewTab from './_components/OverviewTab.svelte';
	import TimelineTab from './_components/TimelineTab.svelte';
	import NotesTab from './_components/NotesTab.svelte';
	import WinDealModal from './_components/WinDealModal.svelte';
	import LoseDealModal from './_components/LoseDealModal.svelte';
	import StageChangeModal from './_components/StageChangeModal.svelte';
	import AddNoteModal from './_components/AddNoteModal.svelte';
	import Toast from './_components/Toast.svelte';
	import type { DealNote, ToastMessage } from './_components/helpers';

	// STATE (Svelte 5 Runes)

	let deal = $state<Deal | null>(null);
	let pipeline = $state<Pipeline | null>(null);
	let timeline = $state<TimelineEvent[]>([]);
	let notes = $state<DealNote[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let activeTab = $state<'overview' | 'timeline' | 'notes'>('overview');

	// Modal states
	let showWinModal = $state(false);
	let showLoseModal = $state(false);
	let showAddNoteModal = $state(false);
	let showStageChangeModal = $state(false);
	let winDetails = $state('');
	let lostReason = $state('');
	let newNoteContent = $state('');
	let selectedStage = $state<Stage | null>(null);
	let stageChangeReason = $state('');
	let processingAction = $state(false);

	let showDeleteModal = $state(false);
	let toastMessage = $state<ToastMessage | null>(null);

	// DERIVED STATE

	let dealId = $derived(page.params.id as string);
	let currentStage = $derived(pipeline?.stages?.find((s) => s.id === deal?.stage_id) || null);
	let isWon = $derived(deal?.status === 'won');
	let isLost = $derived(deal?.status === 'lost');
	let isOpen = $derived(deal?.status === 'open');

	// API FUNCTIONS

	async function loadDeal() {
		loading = true;
		error = null;

		try {
			type TimelineRes = { data?: TimelineEvent[] } | TimelineEvent[];
			type NotesRes = { data?: DealNote[] } | DealNote[];
			const [dealRes, timelineRes, notesRes] = await Promise.allSettled([
				crmAPI.getDeal(dealId),
				api.get<TimelineRes>(`/api/admin/crm/deals/${dealId}/timeline`),
				api.get<NotesRes>(`/api/admin/crm/deals/${dealId}/notes`)
			]);

			if (dealRes.status === 'fulfilled') {
				deal = dealRes.value;

				if (deal?.pipeline_id) {
					try {
						pipeline = await crmAPI.getPipeline(deal.pipeline_id);
					} catch {
						// Pipeline loading is optional
					}
				}
			} else {
				throw new Error('Failed to load deal');
			}

			if (timelineRes.status === 'fulfilled') {
				const v = timelineRes.value;
				timeline = Array.isArray(v) ? v : v?.data || [];
			}

			if (notesRes.status === 'fulfilled') {
				const v = notesRes.value;
				notes = Array.isArray(v) ? v : v?.data || [];
			}
		} catch (e) {
			console.error('Failed to load deal:', e);
			error = 'Failed to load deal. Please try again.';
		} finally {
			loading = false;
		}
	}

	async function updateStage() {
		if (!selectedStage || !deal) return;
		processingAction = true;

		try {
			await crmAPI.updateDealStage(deal.id, selectedStage.id, stageChangeReason || undefined);
			showStageChangeModal = false;
			selectedStage = null;
			stageChangeReason = '';
			showToast('success', 'Deal stage updated successfully');
			await loadDeal();
		} catch (_e) {
			showToast('error', 'Failed to update deal stage');
		} finally {
			processingAction = false;
		}
	}

	async function winDeal() {
		if (!deal) return;
		processingAction = true;

		try {
			await crmAPI.winDeal(deal.id, winDetails || undefined);
			showWinModal = false;
			winDetails = '';
			showToast('success', 'Deal marked as won!');
			await loadDeal();
		} catch (_e) {
			showToast('error', 'Failed to mark deal as won');
		} finally {
			processingAction = false;
		}
	}

	async function loseDeal() {
		if (!deal || !lostReason.trim()) return;
		processingAction = true;

		try {
			await crmAPI.loseDeal(deal.id, lostReason);
			showLoseModal = false;
			lostReason = '';
			showToast('success', 'Deal marked as lost');
			await loadDeal();
		} catch (_e) {
			showToast('error', 'Failed to mark deal as lost');
		} finally {
			processingAction = false;
		}
	}

	async function addNote() {
		if (!newNoteContent.trim() || !deal) return;
		processingAction = true;

		try {
			await api.post(`/api/admin/crm/deals/${deal.id}/notes`, { content: newNoteContent });
			newNoteContent = '';
			showAddNoteModal = false;
			showToast('success', 'Note added successfully');
			await loadDeal();
		} catch (_e) {
			showToast('error', 'Failed to add note');
		} finally {
			processingAction = false;
		}
	}

	function deleteDeal() {
		if (!deal) return;
		showDeleteModal = true;
	}

	async function confirmDeleteDeal() {
		if (!deal) return;
		showDeleteModal = false;

		try {
			await crmAPI.updateDeal(deal.id, { status: 'abandoned' });
			showToast('success', 'Deal deleted');
			goto('/admin/crm/deals');
		} catch (_e) {
			showToast('error', 'Failed to delete deal');
		}
	}

	function showToast(type: 'success' | 'error', text: string) {
		toastMessage = { type, text };
		setTimeout(() => {
			toastMessage = null;
		}, 5000);
	}

	function goBack() {
		goto('/admin/crm/deals');
	}

	function openStageChange(stage: Stage) {
		selectedStage = stage;
		stageChangeReason = '';
		showStageChangeModal = true;
	}

	// Audit P2 #10: was a bare `$effect(() => loadDeal())`. Migrated to
	// `onMount` to keep one-shot init off the reactive graph.
	onMount(() => {
		loadDeal();
	});
</script>

<svelte:head>
	<title>{deal ? `${deal.name} | Deal` : 'Deal'} - CRM Admin</title>
</svelte:head>

<div class="deal-detail-page">
	<button class="back-btn" onclick={goBack}>
		<IconArrowLeft size={18} />
		Back to Deals
	</button>

	{#if loading && !deal}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading deal...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<IconAlertTriangle size={48} />
			<p>{error}</p>
			<button class="btn-primary" onclick={loadDeal}>Try Again</button>
		</div>
	{:else if deal}
		<DealHeader
			{deal}
			{dealId}
			{currentStage}
			{isOpen}
			onRefresh={loadDeal}
			onOpenWin={() => (showWinModal = true)}
			onOpenLose={() => (showLoseModal = true)}
			onDelete={deleteDeal}
		/>

		<DealValueCard {deal} />

		{#if isOpen && pipeline?.stages}
			<StageProgress {deal} {pipeline} onSelectStage={openStageChange} />
		{/if}

		<!-- Tabs -->
		<nav class="tabs-nav">
			<button
				class={['tab-btn', { active: activeTab === 'overview' }]}
				onclick={() => (activeTab = 'overview')}
			>
				<IconBriefcase size={18} />
				Overview
			</button>
			<button
				class={['tab-btn', { active: activeTab === 'timeline' }]}
				onclick={() => (activeTab = 'timeline')}
			>
				<IconHistory size={18} />
				Timeline ({timeline.length})
			</button>
			<button
				class={['tab-btn', { active: activeTab === 'notes' }]}
				onclick={() => (activeTab = 'notes')}
			>
				<IconNotes size={18} />
				Notes ({notes.length})
			</button>
		</nav>

		<!-- Tab Content - Layout Shift Free Pattern -->
		<div class="tab-content">
			<div
				class={['tab-panel', { active: activeTab === 'overview' }]}
				inert={activeTab !== 'overview' ? true : undefined}
			>
				<OverviewTab {deal} {currentStage} {isWon} {isLost} />
			</div>

			<div
				class={['tab-panel', { active: activeTab === 'timeline' }]}
				inert={activeTab !== 'timeline' ? true : undefined}
			>
				<TimelineTab {timeline} />
			</div>

			<div
				class={['tab-panel', { active: activeTab === 'notes' }]}
				inert={activeTab !== 'notes' ? true : undefined}
			>
				<NotesTab {notes} onOpenAdd={() => (showAddNoteModal = true)} />
			</div>
		</div>
	{:else}
		<div class="empty-state">
			<IconBriefcase size={48} />
			<h3>Deal not found</h3>
			<p>The deal you're looking for doesn't exist or has been deleted</p>
			<button class="btn-primary" onclick={goBack}>Go Back</button>
		</div>
	{/if}
</div>

{#if showWinModal && deal}
	<WinDealModal
		{deal}
		{winDetails}
		{processingAction}
		onUpdateWinDetails={(v) => (winDetails = v)}
		onConfirm={winDeal}
		onCancel={() => (showWinModal = false)}
	/>
{/if}

{#if showLoseModal && deal}
	<LoseDealModal
		{deal}
		{lostReason}
		{processingAction}
		onUpdateReason={(v) => (lostReason = v)}
		onConfirm={loseDeal}
		onCancel={() => (showLoseModal = false)}
	/>
{/if}

{#if showStageChangeModal && selectedStage && deal}
	<StageChangeModal
		{currentStage}
		{selectedStage}
		{stageChangeReason}
		{processingAction}
		onUpdateReason={(v) => (stageChangeReason = v)}
		onConfirm={updateStage}
		onCancel={() => (showStageChangeModal = false)}
	/>
{/if}

{#if showAddNoteModal}
	<AddNoteModal
		{newNoteContent}
		{processingAction}
		onUpdateContent={(v) => (newNoteContent = v)}
		onConfirm={addNote}
		onCancel={() => (showAddNoteModal = false)}
	/>
{/if}

{#if toastMessage}
	<Toast toast={toastMessage} onDismiss={() => (toastMessage = null)} />
{/if}

<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete Deal"
	message={deal
		? `Are you sure you want to delete "${deal.name}"? This action cannot be undone.`
		: ''}
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteDeal}
	onCancel={() => {
		showDeleteModal = false;
	}}
/>

<style>
	.deal-detail-page {
		max-width: 1200px;
		margin: 0 auto;
		padding: 24px;
	}

	.back-btn {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 8px 16px;
		background: transparent;
		border: 1px solid #334155;
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
		margin-bottom: 24px;
	}

	.back-btn:hover {
		background: #1e293b;
		color: #f97316;
		border-color: #f97316;
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 20px;
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(249, 115, 22, 0.4);
	}

	/* Tabs */
	.tabs-nav {
		display: flex;
		gap: 4px;
		margin-bottom: 24px;
		padding: 4px;
		background: #1e293b;
		border-radius: 12px;
		overflow-x: auto;
	}

	.tab-btn {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 20px;
		background: transparent;
		border: none;
		border-radius: 8px;
		color: #64748b;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		white-space: nowrap;
	}

	.tab-btn:hover {
		color: #e2e8f0;
		background: rgba(249, 115, 22, 0.1);
	}

	.tab-btn.active {
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: white;
	}

	/* Tab Content - Layout Shift Prevention */
	.tab-content {
		position: relative;
		min-height: 400px;
		contain: layout style;
		isolation: isolate;
	}

	.tab-panel {
		position: absolute;
		inset: 0;
		width: 100%;
		contain: content;
		opacity: 0;
		visibility: hidden;
		transform: translateY(8px);
		transition:
			opacity 0.2s ease,
			visibility 0.2s ease,
			transform 0.2s ease;
		z-index: 0;
		pointer-events: none;
	}

	.tab-panel.active {
		position: relative;
		opacity: 1;
		visibility: visible;
		transform: translateY(0);
		z-index: 1;
		pointer-events: auto;
	}

	@media (prefers-reduced-motion: reduce) {
		.tab-panel {
			transition: none;
			transform: none;
		}
		.tab-panel:not(.active) {
			display: none;
		}
	}

	/* Empty States */
	.empty-state,
	.loading-state,
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		text-align: center;
		color: #64748b;
	}

	.empty-state :global(svg) {
		color: #475569;
		margin-bottom: 16px;
	}

	.empty-state h3 {
		margin: 0 0 8px;
		color: white;
	}

	.empty-state p {
		margin: 0 0 20px;
	}

	.error-state :global(svg) {
		color: #f87171;
		margin-bottom: 16px;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(249, 115, 22, 0.2);
		border-top-color: #f97316;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 16px;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	@media (max-width: 767.98px) {
		.deal-detail-page {
			padding: 16px;
		}
	}
</style>
