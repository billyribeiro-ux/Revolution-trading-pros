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
-->

<script lang="ts">
	/**
	 * Deal Detail Page - FluentCRM Pro Style
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Full deal profile with:
	 * - Deal information and stage
	 * - Associated contact
	 * - Timeline of activities
	 * - Notes section
	 * - Quick actions
	 *
	 * @version 1.0.0 (January 2026)
	 */

	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	// Svelte 5 individual icon imports (Dec 2025 pattern)
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconBriefcase from '@tabler/icons-svelte/icons/briefcase';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconUser from '@tabler/icons-svelte/icons/user';
	import IconBuilding from '@tabler/icons-svelte/icons/building';
	import IconCurrencyDollar from '@tabler/icons-svelte/icons/currency-dollar';
	import IconCalendar from '@tabler/icons-svelte/icons/calendar';
	import IconClock from '@tabler/icons-svelte/icons/clock';
	import IconTarget from '@tabler/icons-svelte/icons/target';
	import IconActivity from '@tabler/icons-svelte/icons/activity';
	import IconNotes from '@tabler/icons-svelte/icons/notes';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import IconTrophy from '@tabler/icons-svelte/icons/trophy';
	import IconAlertTriangle from '@tabler/icons-svelte/icons/alert-triangle';
	import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';
	import IconMail from '@tabler/icons-svelte/icons/mail';
	import IconPhone from '@tabler/icons-svelte/icons/phone';
	import IconChartBar from '@tabler/icons-svelte/icons/chart-bar';
	import IconFlag from '@tabler/icons-svelte/icons/flag';
	import IconTag from '@tabler/icons-svelte/icons/tag';
	import IconHistory from '@tabler/icons-svelte/icons/history';
	import { crmAPI } from '$lib/api/crm';
	import type { Deal, Pipeline, Stage, TimelineEvent, Note } from '$lib/crm/types';
	import { api } from '$lib/api/config';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	interface DealNote {
		id: string;
		content: string;
		created_at: string;
		created_by?: { name: string };
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE (Svelte 5 Runes)
	// ═══════════════════════════════════════════════════════════════════════════

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

	// Toast notification state
	let toastMessage = $state<{ type: 'success' | 'error'; text: string } | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE (Svelte 5 $derived)
	// ═══════════════════════════════════════════════════════════════════════════

	let dealId = $derived(page.params.id as string);

	let currentStage = $derived(
		pipeline?.stages?.find(s => s.id === deal?.stage_id) || null
	);

	let stageProgress = $derived(() => {
		if (!pipeline?.stages || !deal) return 0;
		const stageIndex = pipeline.stages.findIndex(s => s.id === deal?.stage_id);
		return stageIndex >= 0 ? ((stageIndex + 1) / pipeline.stages.length) * 100 : 0;
	});

	let isWon = $derived(deal?.status === 'won');
	let isLost = $derived(deal?.status === 'lost');
	let isOpen = $derived(deal?.status === 'open');

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadDeal() {
		loading = true;
		error = null;

		try {
			const [dealRes, timelineRes, notesRes] = await Promise.allSettled([
				crmAPI.getDeal(dealId),
				api.get(`/api/admin/crm/deals/${dealId}/timeline`),
				api.get(`/api/admin/crm/deals/${dealId}/notes`)
			]);

			if (dealRes.status === 'fulfilled') {
				deal = dealRes.value;

				// Load pipeline for stages
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
				timeline = timelineRes.value?.data || timelineRes.value || [];
			}

			if (notesRes.status === 'fulfilled') {
				notes = notesRes.value?.data || notesRes.value || [];
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
		} catch (e) {
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
		} catch (e) {
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
		} catch (e) {
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
		} catch (e) {
			showToast('error', 'Failed to add note');
		} finally {
			processingAction = false;
		}
	}

	async function deleteDeal() {
		if (!deal) return;
		if (!confirm(`Are you sure you want to delete "${deal.name}"? This action cannot be undone.`)) return;

		try {
			await crmAPI.updateDeal(deal.id, { status: 'abandoned' } as any);
			showToast('success', 'Deal deleted');
			goto('/admin/crm/deals');
		} catch (e) {
			showToast('error', 'Failed to delete deal');
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

	function formatCurrency(amount: number | undefined): string {
		if (!amount) return '$0';
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0,
			maximumFractionDigits: 0
		}).format(amount);
	}

	function formatDate(dateString: string | undefined): string {
		if (!dateString) return 'N/A';
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function formatDateTime(dateString: string | undefined): string {
		if (!dateString) return 'N/A';
		return new Date(dateString).toLocaleString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getStageColor(stage: Stage | null | undefined): string {
		if (!stage) return '#6366f1';
		if (stage.is_closed_won) return '#22c55e';
		if (stage.is_closed_lost) return '#ef4444';
		return stage.color || '#6366f1';
	}

	function getPriorityColor(priority: string | undefined): string {
		const colors: Record<string, string> = {
			low: '#64748b',
			normal: '#3b82f6',
			high: '#f59e0b',
			urgent: '#ef4444'
		};
		return colors[priority || 'normal'] || colors.normal;
	}

	function getStatusBadge(status: string | undefined): { bg: string; color: string; text: string } {
		const badges: Record<string, { bg: string; color: string; text: string }> = {
			open: { bg: 'rgba(59, 130, 246, 0.15)', color: '#60a5fa', text: 'Open' },
			won: { bg: 'rgba(34, 197, 94, 0.15)', color: '#4ade80', text: 'Won' },
			lost: { bg: 'rgba(239, 68, 68, 0.15)', color: '#f87171', text: 'Lost' },
			abandoned: { bg: 'rgba(100, 116, 139, 0.15)', color: '#94a3b8', text: 'Abandoned' }
		};
		return badges[status || 'open'] || badges.open;
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

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(() => {
		loadDeal();
	});
</script>

<svelte:head>
	<title>{deal ? `${deal.name} | Deal` : 'Deal'} - CRM Admin</title>
</svelte:head>

<div class="deal-detail-page">
	<!-- Back Button -->
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
		<!-- Header Section -->
		<header class="deal-header">
			<div class="deal-identity">
				<div class="deal-icon" style="background: {getStageColor(currentStage)}20; color: {getStageColor(currentStage)}">
					<IconBriefcase size={28} />
				</div>
				<div class="deal-info">
					<div class="name-row">
						<h1>{deal.name}</h1>
						<span class="status-badge" style="background: {getStatusBadge(deal.status).bg}; color: {getStatusBadge(deal.status).color}">
							{getStatusBadge(deal.status).text}
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
								<IconFlag size={14} style="color: {getStageColor(currentStage)}" />
								{currentStage.name}
							</span>
						{/if}
						{#if deal.priority && deal.priority !== 'normal'}
							<span class="meta-item priority" style="color: {getPriorityColor(deal.priority)}">
								{deal.priority} priority
							</span>
						{/if}
					</div>
				</div>
			</div>

			<div class="header-actions">
				<button class="btn-icon" onclick={loadDeal} title="Refresh">
					<IconRefresh size={18} />
				</button>
				{#if isOpen}
					<button class="btn-success" onclick={() => showWinModal = true}>
						<IconTrophy size={18} />
						Mark Won
					</button>
					<button class="btn-danger-outline" onclick={() => showLoseModal = true}>
						<IconX size={18} />
						Mark Lost
					</button>
				{/if}
				<a href="/admin/crm/deals/{dealId}/edit" class="btn-primary">
					<IconEdit size={18} />
					Edit Deal
				</a>
				<button class="btn-icon danger" onclick={deleteDeal} title="Delete">
					<IconTrash size={18} />
				</button>
			</div>
		</header>

		<!-- Deal Value Card -->
		<div class="value-card">
			<div class="value-main">
				<span class="value-label">Deal Value</span>
				<span class="value-amount">{formatCurrency(deal.amount)}</span>
			</div>
			<div class="value-stats">
				<div class="value-stat">
					<span class="stat-label">Probability</span>
					<div class="probability-bar">
						<div class="bar-fill" style="width: {deal.probability}%"></div>
						<span class="bar-text">{deal.probability}%</span>
					</div>
				</div>
				<div class="value-stat">
					<span class="stat-label">Weighted Value</span>
					<span class="stat-value">{formatCurrency(deal.weighted_value)}</span>
				</div>
				<div class="value-stat">
					<span class="stat-label">Expected Close</span>
					<span class="stat-value">{formatDate(deal.expected_close_date)}</span>
				</div>
				<div class="value-stat">
					<span class="stat-label">Days in Pipeline</span>
					<span class="stat-value">{deal.days_in_pipeline} days</span>
				</div>
			</div>
		</div>

		<!-- Stage Progress (if open) -->
		{#if isOpen && pipeline?.stages}
			<div class="stage-progress-section">
				<h3>Stage Progress</h3>
				<div class="stages-row">
					{#each pipeline.stages.filter(s => !s.is_closed_won && !s.is_closed_lost) as stage (stage.id)}
						{@const isCurrent = stage.id === deal?.stage_id}
						{@const isPast = pipeline.stages.findIndex(s => s.id === stage.id) < pipeline.stages.findIndex(s => s.id === deal?.stage_id)}
						<button
							class="stage-item"
							class:current={isCurrent}
							class:past={isPast}
							onclick={() => !isCurrent && openStageChange(stage)}
							disabled={isCurrent}
							style="--stage-color: {stage.color || '#6366f1'}"
						>
							<span class="stage-indicator"></span>
							<span class="stage-name">{stage.name}</span>
							{#if stage.probability > 0}
								<span class="stage-probability">{stage.probability}%</span>
							{/if}
						</button>
						{#if pipeline.stages.indexOf(stage) < pipeline.stages.filter(s => !s.is_closed_won && !s.is_closed_lost).length - 1}
							<div class="stage-connector" class:active={isPast}></div>
						{/if}
					{/each}
				</div>
			</div>
		{/if}

		<!-- Tabs -->
		<nav class="tabs-nav">
			<button class="tab-btn" class:active={activeTab === 'overview'} onclick={() => activeTab = 'overview'}>
				<IconBriefcase size={18} />
				Overview
			</button>
			<button class="tab-btn" class:active={activeTab === 'timeline'} onclick={() => activeTab = 'timeline'}>
				<IconHistory size={18} />
				Timeline ({timeline.length})
			</button>
			<button class="tab-btn" class:active={activeTab === 'notes'} onclick={() => activeTab = 'notes'}>
				<IconNotes size={18} />
				Notes ({notes.length})
			</button>
		</nav>

		<!-- Tab Content -->
		<div class="tab-content">
			{#if activeTab === 'overview'}
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
								<span class="info-value" style="color: {getStageColor(currentStage)}">
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
								{#each deal.tags as tag}
									<span class="tag-pill">{tag}</span>
								{/each}
							</div>
						</div>
					{/if}
				</div>

			{:else if activeTab === 'timeline'}
				<div class="timeline-section">
					{#if timeline.length === 0}
						<div class="empty-state">
							<IconHistory size={48} />
							<h3>No activity yet</h3>
							<p>Activity timeline will appear here as events occur</p>
						</div>
					{:else}
						<div class="timeline">
							{#each timeline as event (event.id)}
								<div class="timeline-item">
									<div class="timeline-dot"></div>
									<div class="timeline-content">
										<div class="timeline-header">
											<span class="timeline-title">{event.title}</span>
											<span class="timeline-time">{formatDateTime(event.occurred_at)}</span>
										</div>
										{#if event.description}
											<p class="timeline-description">{event.description}</p>
										{/if}
										{#if event.created_by}
											<span class="timeline-author">by {event.created_by.name}</span>
										{/if}
									</div>
								</div>
							{/each}
						</div>
					{/if}
				</div>

			{:else if activeTab === 'notes'}
				<div class="notes-section">
					<div class="notes-header">
						<button class="btn-primary" onclick={() => showAddNoteModal = true}>
							<IconPlus size={18} />
							Add Note
						</button>
					</div>
					{#if notes.length === 0}
						<div class="empty-state">
							<IconNotes size={48} />
							<h3>No notes yet</h3>
							<p>Add notes to keep track of important information about this deal</p>
						</div>
					{:else}
						<div class="notes-list">
							{#each notes as note (note.id)}
								<div class="note-item">
									<div class="note-header">
										<span class="note-author">{note.created_by?.name || 'Unknown'}</span>
										<span class="note-date">{formatDateTime(note.created_at)}</span>
									</div>
									<p class="note-content">{note.content}</p>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			{/if}
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

<!-- Win Deal Modal -->
{#if showWinModal && deal}
	<div
		class="modal-overlay"
		onclick={() => showWinModal = false}
		onkeydown={(e) => e.key === 'Escape' && (showWinModal = false)}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div class="modal" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="document">
			<div class="modal-header success">
				<IconTrophy size={24} />
				<h3>Mark Deal as Won</h3>
				<button class="modal-close" onclick={() => showWinModal = false}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<div class="deal-summary">
					<p class="summary-name">{deal.name}</p>
					<p class="summary-value">{formatCurrency(deal.amount)}</p>
				</div>
				<div class="form-group">
					<label for="win-details">Win Details (optional)</label>
					<textarea
						id="win-details"
						bind:value={winDetails}
						placeholder="Add notes about how this deal was won..."
						rows="4"
					></textarea>
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => showWinModal = false} disabled={processingAction}>
					Cancel
				</button>
				<button class="btn-success" onclick={winDeal} disabled={processingAction}>
					{#if processingAction}
						<div class="btn-spinner"></div>
					{:else}
						<IconCheck size={18} />
					{/if}
					Mark as Won
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Lose Deal Modal -->
{#if showLoseModal && deal}
	<div
		class="modal-overlay"
		onclick={() => showLoseModal = false}
		onkeydown={(e) => e.key === 'Escape' && (showLoseModal = false)}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div class="modal" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="document">
			<div class="modal-header danger">
				<IconX size={24} />
				<h3>Mark Deal as Lost</h3>
				<button class="modal-close" onclick={() => showLoseModal = false}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<div class="deal-summary">
					<p class="summary-name">{deal.name}</p>
					<p class="summary-value">{formatCurrency(deal.amount)}</p>
				</div>
				<div class="form-group">
					<label for="lost-reason">Lost Reason <span class="required">*</span></label>
					<textarea
						id="lost-reason"
						bind:value={lostReason}
						placeholder="Why was this deal lost? (required)"
						rows="4"
						required
					></textarea>
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => showLoseModal = false} disabled={processingAction}>
					Cancel
				</button>
				<button class="btn-danger" onclick={loseDeal} disabled={processingAction || !lostReason.trim()}>
					{#if processingAction}
						<div class="btn-spinner"></div>
					{:else}
						<IconX size={18} />
					{/if}
					Mark as Lost
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Stage Change Modal -->
{#if showStageChangeModal && selectedStage && deal}
	<div
		class="modal-overlay"
		onclick={() => showStageChangeModal = false}
		onkeydown={(e) => e.key === 'Escape' && (showStageChangeModal = false)}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div class="modal" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="document">
			<div class="modal-header">
				<IconArrowRight size={24} />
				<h3>Change Stage</h3>
				<button class="modal-close" onclick={() => showStageChangeModal = false}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<div class="stage-change-preview">
					<div class="stage-from">
						<span class="stage-label">From</span>
						<span class="stage-name" style="color: {getStageColor(currentStage)}">{currentStage?.name}</span>
					</div>
					<IconArrowRight size={20} class="stage-arrow" />
					<div class="stage-to">
						<span class="stage-label">To</span>
						<span class="stage-name" style="color: {getStageColor(selectedStage)}">{selectedStage.name}</span>
					</div>
				</div>
				<div class="form-group">
					<label for="stage-reason">Reason (optional)</label>
					<textarea
						id="stage-reason"
						bind:value={stageChangeReason}
						placeholder="Why is this stage changing?"
						rows="3"
					></textarea>
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => showStageChangeModal = false} disabled={processingAction}>
					Cancel
				</button>
				<button class="btn-primary" onclick={updateStage} disabled={processingAction}>
					{#if processingAction}
						<div class="btn-spinner"></div>
					{:else}
						<IconCheck size={18} />
					{/if}
					Update Stage
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Add Note Modal -->
{#if showAddNoteModal}
	<div
		class="modal-overlay"
		onclick={() => showAddNoteModal = false}
		onkeydown={(e) => e.key === 'Escape' && (showAddNoteModal = false)}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div class="modal" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="document">
			<div class="modal-header">
				<IconNotes size={24} />
				<h3>Add Note</h3>
				<button class="modal-close" onclick={() => showAddNoteModal = false}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<div class="form-group">
					<label for="note-content">Note Content</label>
					<textarea
						id="note-content"
						bind:value={newNoteContent}
						placeholder="Write your note here..."
						rows="6"
					></textarea>
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => showAddNoteModal = false} disabled={processingAction}>
					Cancel
				</button>
				<button class="btn-primary" onclick={addNote} disabled={processingAction || !newNoteContent.trim()}>
					{#if processingAction}
						<div class="btn-spinner"></div>
					{:else}
						<IconCheck size={18} />
					{/if}
					Save Note
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Toast Notification -->
{#if toastMessage}
	<div class="toast toast-{toastMessage.type}" role="alert" aria-live="polite">
		{#if toastMessage.type === 'success'}
			<IconCheck size={18} />
		{:else}
			<IconX size={18} />
		{/if}
		<span>{toastMessage.text}</span>
		<button class="toast-close" onclick={() => toastMessage = null} aria-label="Dismiss">
			<IconX size={14} />
		</button>
	</div>
{/if}

<style>
	.deal-detail-page {
		max-width: 1200px;
		margin: 0 auto;
		padding: 24px;
	}

	/* Back Button */
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

	/* Header */
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
		color: white;
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
		color: #94a3b8;
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
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: rgba(249, 115, 22, 0.1);
		color: #f97316;
		border-color: rgba(249, 115, 22, 0.3);
	}

	.btn-icon.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-color: rgba(239, 68, 68, 0.3);
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

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none;
	}

	.btn-success {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 20px;
		background: linear-gradient(135deg, #22c55e, #16a34a);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-success:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(34, 197, 94, 0.4);
	}

	.btn-success:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-danger-outline {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 20px;
		background: transparent;
		border: 1px solid rgba(239, 68, 68, 0.5);
		color: #f87171;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-danger-outline:hover {
		background: rgba(239, 68, 68, 0.1);
	}

	/* Value Card */
	.value-card {
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 16px;
		padding: 24px;
		margin-bottom: 24px;
		display: grid;
		grid-template-columns: auto 1fr;
		gap: 32px;
		align-items: center;
	}

	.value-main {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.value-label {
		font-size: 0.85rem;
		color: #64748b;
	}

	.value-amount {
		font-size: 2.5rem;
		font-weight: 700;
		color: #4ade80;
	}

	.value-stats {
		display: flex;
		gap: 32px;
		flex-wrap: wrap;
	}

	.value-stat {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.value-stat .stat-label {
		font-size: 0.75rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.value-stat .stat-value {
		font-size: 1.1rem;
		font-weight: 600;
		color: white;
	}

	.probability-bar {
		position: relative;
		width: 100px;
		height: 24px;
		background: #0f172a;
		border-radius: 12px;
		overflow: hidden;
	}

	.bar-fill {
		position: absolute;
		left: 0;
		top: 0;
		height: 100%;
		background: linear-gradient(135deg, #f97316, #ea580c);
		transition: width 0.3s;
	}

	.bar-text {
		position: relative;
		z-index: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		font-size: 0.75rem;
		font-weight: 600;
		color: white;
	}

	/* Stage Progress */
	.stage-progress-section {
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 16px;
		padding: 24px;
		margin-bottom: 24px;
	}

	.stage-progress-section h3 {
		margin: 0 0 20px;
		font-size: 1rem;
		color: white;
	}

	.stages-row {
		display: flex;
		align-items: center;
		gap: 0;
		overflow-x: auto;
		padding-bottom: 8px;
	}

	.stage-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 8px;
		padding: 12px 16px;
		background: transparent;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
		min-width: 100px;
	}

	.stage-item:disabled {
		cursor: default;
	}

	.stage-item:hover:not(:disabled) {
		background: rgba(249, 115, 22, 0.05);
		border-radius: 10px;
	}

	.stage-indicator {
		width: 16px;
		height: 16px;
		border-radius: 50%;
		background: #334155;
		transition: all 0.2s;
	}

	.stage-item.past .stage-indicator {
		background: var(--stage-color);
	}

	.stage-item.current .stage-indicator {
		background: var(--stage-color);
		box-shadow: 0 0 0 4px rgba(249, 115, 22, 0.2);
		transform: scale(1.2);
	}

	.stage-item .stage-name {
		font-size: 0.8rem;
		color: #64748b;
		text-align: center;
	}

	.stage-item.current .stage-name,
	.stage-item.past .stage-name {
		color: white;
		font-weight: 600;
	}

	.stage-item .stage-probability {
		font-size: 0.7rem;
		color: #475569;
	}

	.stage-connector {
		width: 32px;
		height: 2px;
		background: #334155;
	}

	.stage-connector.active {
		background: #f97316;
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

	/* Tab Content */
	.tab-content {
		min-height: 300px;
	}

	/* Overview Grid */
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

	/* Contact Preview */
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

	.contact-email, .contact-phone {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 0.8rem;
		color: #60a5fa;
		text-decoration: none;
	}

	.contact-email:hover, .contact-phone:hover {
		text-decoration: underline;
	}

	/* Tags */
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

	/* Timeline */
	.timeline-section {
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 14px;
		padding: 20px;
	}

	.timeline {
		display: flex;
		flex-direction: column;
	}

	.timeline-item {
		display: flex;
		gap: 16px;
		padding: 16px 0;
		border-bottom: 1px solid #334155;
	}

	.timeline-item:last-child {
		border-bottom: none;
	}

	.timeline-dot {
		width: 12px;
		height: 12px;
		border-radius: 50%;
		background: #f97316;
		flex-shrink: 0;
		margin-top: 4px;
	}

	.timeline-content {
		flex: 1;
	}

	.timeline-header {
		display: flex;
		justify-content: space-between;
		gap: 12px;
		margin-bottom: 4px;
	}

	.timeline-title {
		font-weight: 600;
		color: white;
		font-size: 0.9rem;
	}

	.timeline-time {
		font-size: 0.75rem;
		color: #64748b;
		white-space: nowrap;
	}

	.timeline-description {
		margin: 0 0 4px;
		font-size: 0.85rem;
		color: #94a3b8;
	}

	.timeline-author {
		font-size: 0.75rem;
		color: #64748b;
	}

	/* Notes */
	.notes-section {
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 14px;
		padding: 20px;
	}

	.notes-header {
		margin-bottom: 20px;
	}

	.notes-list {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.note-item {
		padding: 16px;
		background: #0f172a;
		border-radius: 10px;
	}

	.note-header {
		display: flex;
		justify-content: space-between;
		margin-bottom: 8px;
	}

	.note-author {
		font-weight: 600;
		color: #e2e8f0;
		font-size: 0.875rem;
	}

	.note-date {
		font-size: 0.75rem;
		color: #64748b;
	}

	.note-content {
		margin: 0;
		color: #94a3b8;
		font-size: 0.9rem;
		line-height: 1.6;
		white-space: pre-wrap;
	}

	/* Empty States */
	.empty-state, .loading-state, .error-state {
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

	.empty-text {
		color: #64748b;
		margin: 0;
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
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 20px;
	}

	.modal {
		width: 100%;
		max-width: 480px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 16px;
		overflow: hidden;
	}

	.modal-header {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 20px;
		border-bottom: 1px solid #334155;
	}

	.modal-header.success {
		background: rgba(34, 197, 94, 0.1);
	}

	.modal-header.success :global(svg) {
		color: #4ade80;
	}

	.modal-header.danger {
		background: rgba(239, 68, 68, 0.1);
	}

	.modal-header.danger :global(svg) {
		color: #f87171;
	}

	.modal-header :global(svg:first-child) {
		color: #f97316;
	}

	.modal-header h3 {
		flex: 1;
		margin: 0;
		font-size: 1.1rem;
		color: white;
	}

	.modal-close {
		display: flex;
		padding: 8px;
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
	}

	.modal-close:hover {
		color: white;
	}

	.modal-body {
		padding: 20px;
	}

	.deal-summary {
		padding: 16px;
		background: #0f172a;
		border-radius: 10px;
		margin-bottom: 20px;
	}

	.summary-name {
		margin: 0 0 4px;
		font-weight: 600;
		color: white;
	}

	.summary-value {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 700;
		color: #4ade80;
	}

	.stage-change-preview {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 16px;
		padding: 16px;
		background: #0f172a;
		border-radius: 10px;
		margin-bottom: 20px;
	}

	.stage-from, .stage-to {
		display: flex;
		flex-direction: column;
		gap: 4px;
		text-align: center;
	}

	.stage-label {
		font-size: 0.7rem;
		color: #64748b;
		text-transform: uppercase;
	}

	.stage-change-preview .stage-name {
		font-weight: 600;
		font-size: 0.9rem;
	}

	.stage-arrow {
		color: #64748b;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.form-group label {
		font-size: 0.85rem;
		font-weight: 600;
		color: #e2e8f0;
	}

	.form-group .required {
		color: #f87171;
	}

	.form-group textarea {
		padding: 12px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 8px;
		color: #e2e8f0;
		font-size: 0.9rem;
		font-family: inherit;
		resize: vertical;
	}

	.form-group textarea:focus {
		outline: none;
		border-color: #f97316;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		padding: 20px;
		border-top: 1px solid #334155;
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 10px;
		color: #e2e8f0;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: #1e293b;
	}

	.btn-danger {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: linear-gradient(135deg, #ef4444, #dc2626);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-danger:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(239, 68, 68, 0.4);
	}

	.btn-danger:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-spinner {
		width: 18px;
		height: 18px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	/* Toast */
	.toast {
		position: fixed;
		bottom: 24px;
		right: 24px;
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 14px 18px;
		border-radius: 12px;
		font-size: 0.9rem;
		font-weight: 500;
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
		z-index: 2000;
		animation: slideIn 0.3s ease-out;
	}

	.toast-success {
		background: linear-gradient(135deg, #065f46, #047857);
		border: 1px solid #10b981;
		color: #ecfdf5;
	}

	.toast-success :global(svg) {
		color: #34d399;
	}

	.toast-error {
		background: linear-gradient(135deg, #7f1d1d, #991b1b);
		border: 1px solid #f87171;
		color: #fef2f2;
	}

	.toast-error :global(svg) {
		color: #fca5a5;
	}

	.toast-close {
		display: flex;
		padding: 4px;
		margin-left: 8px;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		border-radius: 6px;
		color: inherit;
		cursor: pointer;
		transition: background 0.2s;
	}

	.toast-close:hover {
		background: rgba(255, 255, 255, 0.2);
	}

	@keyframes slideIn {
		from {
			opacity: 0;
			transform: translateY(20px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	/* Responsive */
	@media (max-width: 768px) {
		.deal-detail-page {
			padding: 16px;
		}

		.deal-header {
			flex-direction: column;
		}

		.header-actions {
			width: 100%;
			flex-wrap: wrap;
		}

		.value-card {
			grid-template-columns: 1fr;
		}

		.value-stats {
			justify-content: flex-start;
		}

		.toast {
			left: 16px;
			right: 16px;
			bottom: 16px;
		}
	}
</style>
