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
	import { useCrmAccess } from '#lib/crm/access.svelte.js';
	const access = useCrmAccess();
	import { goto } from '$app/navigation';
	import { onMount, onDestroy } from 'svelte';
	import IconArrowLeft from '@tabler/icons-svelte-runes/icons/arrow-left';
	import IconBriefcase from '@tabler/icons-svelte-runes/icons/briefcase';
	import IconNotes from '@tabler/icons-svelte-runes/icons/notes';
	import IconAlertTriangle from '@tabler/icons-svelte-runes/icons/alert-triangle';
	import IconHistory from '@tabler/icons-svelte-runes/icons/history';
	import ConfirmationModal from '#lib/components/admin/ConfirmationModal.svelte';
	import { crmAPI } from '#lib/api/crm.js';
	import type { Deal, Pipeline, Stage, TimelineEvent } from '#lib/crm/types.js';
	import { apiClient } from '#lib/api/client.svelte.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';

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

	let { dealId }: { dealId: string } = $props();
	let notePage = $state(1),
		notePages = $state(1),
		noteTotal = $state<number | null>(null);
	let timelinePage = $state(1),
		timelinePages = $state(1),
		timelineTotal = $state<number | null>(null);
	let generation = 0;
	type ActivityPage<T> = { data: T[]; meta: { total: number; last_page: number } };
	let currentStage = $derived(pipeline?.stages?.find((s) => s.id === deal?.stage_id) || null);
	let isWon = $derived(deal?.status === 'won');
	let isLost = $derived(deal?.status === 'lost');
	let isOpen = $derived(deal?.status === 'open');

	// API FUNCTIONS

	async function loadDeal() {
		const request = ++generation;
		loading = true;
		error = null;
		try {
			const [current, events, savedNotes] = await Promise.all([
				crmAPI.getDeal(dealId),
				access.can('GET', '/deals/{id}/timeline')
					? apiClient.get<ActivityPage<TimelineEvent>>(`/admin/crm/deals/${dealId}/timeline`, {
							cache: { enabled: false },
							preserveEnvelope: true,
							params: { page: timelinePage, per_page: 25 }
						})
					: Promise.resolve(null),
				access.can('GET', '/deals/{id}/notes')
					? apiClient.get<ActivityPage<DealNote>>(`/admin/crm/deals/${dealId}/notes`, {
							cache: { enabled: false },
							preserveEnvelope: true,
							params: { page: notePage, per_page: 25 }
						})
					: Promise.resolve(null)
			]);
			const selected = access.can('GET', '/pipelines/{id}')
				? await crmAPI.getPipeline(current.pipeline_id)
				: null;
			if (request !== generation) return;
			deal = current;
			pipeline = selected;
			timeline = events?.data ?? [];
			notes = savedNotes?.data ?? [];
			timelineTotal = events?.meta.total ?? null;
			timelinePages = events?.meta.last_page ?? 1;
			noteTotal = savedNotes?.meta.total ?? null;
			notePages = savedNotes?.meta.last_page ?? 1;
		} catch (cause) {
			if (request === generation) error = apiErrorMessage(cause, 'Unable to load deal activity');
		} finally {
			if (request === generation) loading = false;
		}
	}
	function changeNotes(next: number) {
		notePage = next;
		void loadDeal();
	}
	function changeTimeline(next: number) {
		timelinePage = next;
		void loadDeal();
	}
	onDestroy(() => {
		generation++;
	});

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
		} catch (cause) {
			showToast('error', apiErrorMessage(cause, 'Failed to update deal stage'));
		} finally {
			processingAction = false;
		}
	}

	async function winDeal() {
		if (!deal) return;
		processingAction = true;

		try {
			await crmAPI.winDeal(
				deal.id,
				winDetails || undefined,
				undefined,
				selectedStage?.is_closed_won ? selectedStage.id : undefined
			);
			showWinModal = false;
			winDetails = '';
			showToast('success', 'Deal marked as won!');
			await loadDeal();
		} catch (cause) {
			showToast('error', apiErrorMessage(cause, 'Failed to mark deal as won'));
		} finally {
			processingAction = false;
		}
	}

	async function loseDeal() {
		if (!deal || !lostReason.trim()) return;
		processingAction = true;

		try {
			await crmAPI.loseDeal(
				deal.id,
				lostReason,
				selectedStage?.is_closed_lost ? selectedStage.id : undefined
			);
			showLoseModal = false;
			lostReason = '';
			showToast('success', 'Deal marked as lost');
			await loadDeal();
		} catch (cause) {
			showToast('error', apiErrorMessage(cause, 'Failed to mark deal as lost'));
		} finally {
			processingAction = false;
		}
	}

	async function addNote() {
		if (!newNoteContent.trim() || !deal) return;
		processingAction = true;

		try {
			await apiClient.post(`/admin/crm/deals/${deal.id}/notes`, { content: newNoteContent });
			notePage = 1;
			timelinePage = 1;
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
			await crmAPI.deleteDeal(deal.id);
			showToast('success', 'Deal deleted');
			void goto('/admin/crm/deals');
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
		void goto('/admin/crm/deals');
	}

	function openStageChange(stage: Stage) {
		selectedStage = stage;
		stageChangeReason = '';
		if (stage.is_closed_won) showWinModal = true;
		else if (stage.is_closed_lost) showLoseModal = true;
		else showStageChangeModal = true;
	}

	// Audit P2 #10: was a bare `$effect(() => loadDeal())`. Migrated to
	// `onMount` to keep one-shot init off the reactive graph.
	onMount(() => {
		void loadDeal();
	});
</script>

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
				Timeline ({timelineTotal ?? 'Not permitted'})
			</button>
			<button
				class={['tab-btn', { active: activeTab === 'notes' }]}
				onclick={() => (activeTab = 'notes')}
			>
				<IconNotes size={18} />
				Notes ({noteTotal ?? 'Not permitted'})
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
				{#if access.can('GET', '/deals/{id}/timeline')}<TimelineTab {timeline} />{:else}<p>
						Timeline read permission is required.
					</p>{/if}
				<nav aria-label="Timeline pages">
					<button
						disabled={loading || timelinePage <= 1}
						onclick={() => changeTimeline(timelinePage - 1)}>Previous</button
					><span>Page {timelinePage} of {timelinePages}</span><button
						disabled={loading || timelinePage >= timelinePages}
						onclick={() => changeTimeline(timelinePage + 1)}>Next</button
					>
				</nav>
			</div>

			<div
				class={['tab-panel', { active: activeTab === 'notes' }]}
				inert={activeTab !== 'notes' ? true : undefined}
			>
				{#if access.can('GET', '/deals/{id}/notes')}<NotesTab
						{notes}
						onOpenAdd={() => (showAddNoteModal = true)}
					/>{:else}<p>Note read permission is required.</p>{/if}
				<nav aria-label="Note pages">
					<button disabled={loading || notePage <= 1} onclick={() => changeNotes(notePage - 1)}
						>Previous</button
					><span>Page {notePage} of {notePages}</span><button
						disabled={loading || notePage >= notePages}
						onclick={() => changeNotes(notePage + 1)}>Next</button
					>
				</nav>
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
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
		margin-bottom: 24px;
	}

	.back-btn:hover {
		background: var(--bg-elevated, #161b22);
		color: var(--warning-emphasis, #d29922);
		border-color: var(--primary-500, #e6b800);
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

	/* Tabs */
	.tabs-nav {
		display: flex;
		gap: 4px;
		margin-bottom: 24px;
		padding: 4px;
		background: var(--bg-elevated, #161b22);
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
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		white-space: nowrap;
	}

	.tab-btn:hover {
		color: var(--text-primary, #f0f6fc);
		background: rgba(210, 153, 34, 0.1);
	}

	.tab-btn.active {
		background: var(--primary-500, #e6b800);
		color: var(--bg-base, #0d1117);
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
		color: var(--text-tertiary, #6e7681);
	}

	.empty-state :global(svg) {
		color: var(--text-tertiary, #6e7681);
		margin-bottom: 16px;
	}

	.empty-state h3 {
		margin: 0 0 8px;
		color: #ffffff;
	}

	.empty-state p {
		margin: 0 0 20px;
	}

	.error-state :global(svg) {
		color: var(--error-emphasis, #f85149);
		margin-bottom: 16px;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(210, 153, 34, 0.2);
		border-top-color: var(--primary-500, #e6b800);
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
