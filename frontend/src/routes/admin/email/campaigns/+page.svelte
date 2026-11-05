<script lang="ts">
	/**
	 * Email Campaigns - Apple ICT7 Principal Engineer Grade
	 *
	 * Svelte 5 runes implementation with:
	 * - $state for reactive state management
	 * - $derived for computed values
	 * - $effect for lifecycle and data loading
	 * - Full TypeScript type safety
	 * - Dynamic segments loading from API
	 *
	 */

	// FIX-2026-04-26: onMount imported for $effect-cascade fix below.
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { toastStore } from '#lib/stores/toast.svelte.js';
	import { adminFetch } from '#lib/utils/adminFetch.js';
	import { listMemberSegments } from '#lib/api/member-segments.js';
	import { catalogTemplatePage } from '#lib/api/email-template-catalog.js';
	import { connections, getIsEmailConnected } from '#lib/stores/connections.svelte.js';
	import SkeletonLoader from '#lib/components/SkeletonLoader.svelte';
	import {
		IconMail,
		IconPlus,
		IconCalendar,
		IconUsers,
		IconChartBar,
		IconRefresh,
		IconSend,
		IconClock,
		IconCheck,
		IconX,
		IconEdit,
		IconTrash,
		IconEye,
		IconCopy,
		IconTrendingUp,
		IconArrowLeft
	} from '#lib/icons/index.js';
	import ConfirmationModal from '#lib/components/admin/ConfirmationModal.svelte';
	import {
		getCampaigns,
		getCampaignStats,
		createCampaign,
		deleteCampaign as apiDeleteCampaign,
		duplicateCampaign as apiDuplicateCampaign,
		sendCampaign,
		scheduleCampaign,
		pauseCampaign,
		resumeCampaign,
		cancelCampaign,
		type Campaign as APICampaign,
		type CampaignStats
	} from '#lib/api/campaigns.js';

	// Types

	interface Segment {
		id: number;
		name: string;
		count: number;
	}

	// State - Svelte 5 Runes

	// Connection-aware state
	let connectionLoading = $state(true);

	// State
	let loading = $state(true);
	let error = $state('');
	let statsError = $state('');
	let templatesError = $state('');
	let segmentsError = $state('');
	let creating = $state(false);
	let actionPending = $state<number | null>(null);
	let currentPage = $state(1);
	let total = $state(0);
	let totalPages = $state(1);
	let search = $state('');
	let listGeneration = 0;
	let activeTab = $state<'all' | 'scheduled' | 'sent' | 'drafts'>('all');

	// Campaign data
	let campaigns = $state<APICampaign[]>([]);
	let stats = $state<CampaignStats | null>(null);

	// Create modal
	let showCreateModal = $state(false);
	let newCampaign = $state({
		name: '',
		subject: '',
		subjectB: '',
		useABTest: false,
		abTestSplit: 50,
		scheduledFor: '',
		segmentId: '',
		templateId: ''
	});

	// Segments - dynamically loaded from API
	let segments = $state<Segment[]>([]);

	// Templates (loaded from API)
	let templates = $state<{ id: number; name: string }[]>([]);

	// Derived State

	let filteredCampaigns = $derived(campaigns);
	let scheduledCount = $derived(stats?.scheduled ?? '—');
	let sentCount = $derived(stats?.sent ?? '—');
	let draftCount = $derived(stats?.drafts ?? '—');
	async function selectTab(tab: typeof activeTab) {
		activeTab = tab;
		currentPage = 1;
		await Promise.all([loadCampaigns(), loadStats()]);
	}

	// Lifecycle - onMount (NOT $effect)
	// FIX-2026-04-26: converted from $effect to onMount.
	// `initializeData()` calls `connections.load()` which mutates `connectionsState`
	// AND reads it back via `getIsEmailConnected()`. Inside an $effect that's the
	// classic write-while-reading-tracked-dep pattern that fires
	// effect_update_depth_exceeded on the post-login `goto('/admin/email/campaigns')`
	// flush. Same recipe as the dashboard fix at admin/analytics/+page.svelte:131-163
	// (commit dad9af2cf).
	// See https://svelte.dev/docs/svelte/$effect#When-not-to-use-$effect.
	//
	// Old code (kept for one revision per FIX-2026-04-26 marker — delete in follow-up):
	onMount(() => {
		void initializeData();
		let polling = false;
		let refreshedAt = Date.now();
		const timer = setInterval(() => {
			const delay = campaigns.some((c) => c.status === 'sending')
				? 5000
				: campaigns.some((c) => c.status === 'scheduled')
					? 15000
					: 0;
			if (!delay || polling || loading || Date.now() - refreshedAt < delay) return;
			polling = true;
			refreshedAt = Date.now();
			void Promise.all([loadCampaigns(), loadStats()]).finally(() => {
				polling = false;
			});
		}, 5000);
		return () => clearInterval(timer);
	});
	async function campaignAction(
		id: number,
		action: (id: number) => Promise<APICampaign>,
		message: string
	) {
		if (actionPending !== null) return;
		actionPending = id;
		try {
			const result = await action(id);
			toastStore.success(`${message}. Current status: ${result.status}.`);
		} catch (error) {
			toastStore.error(error instanceof Error ? error.message : 'Campaign action failed');
		} finally {
			actionPending = null;
			await Promise.all([loadCampaigns(), loadStats()]);
		}
	}
	async function scheduleSaved(campaign: APICampaign) {
		if (!campaign.planned_at) {
			toastStore.error('Choose a requested time before scheduling');
			return;
		}
		await campaignAction(
			campaign.id,
			(id) => scheduleCampaign(id, campaign.planned_at ?? ''),
			'Campaign scheduled'
		);
	}
	async function initializeData() {
		await Promise.allSettled([
			connections.load(),
			loadCampaigns(),
			loadStats(),
			loadTemplates(),
			loadSegments()
		]);
		connectionLoading = false;
	}
	async function loadCampaigns() {
		const generation = ++listGeneration;
		loading = true;
		error = '';
		try {
			const response = await getCampaigns({
				status: activeTab === 'all' ? undefined : activeTab === 'drafts' ? 'draft' : activeTab,
				page: currentPage,
				per_page: 20,
				search: search.trim() || undefined
			});
			if (generation !== listGeneration) return;
			if (response.meta.current_page > response.meta.last_page) {
				currentPage = response.meta.last_page;
				await Promise.all([loadCampaigns(), loadStats()]);
				return;
			}
			campaigns = response.data;
			total = response.meta.total;
			currentPage = response.meta.current_page;
			totalPages = response.meta.last_page;
		} catch (err) {
			if (generation !== listGeneration) return;
			campaigns = [];
			total = 0;
			totalPages = 1;
			error = err instanceof Error ? err.message : 'Failed to load campaigns';
		} finally {
			if (generation === listGeneration) loading = false;
		}
	}
	async function loadStats() {
		statsError = '';
		try {
			stats = await getCampaignStats();
		} catch (err) {
			stats = null;
			statsError = err instanceof Error ? err.message : 'Campaign statistics unavailable';
		}
	}
	async function loadTemplates() {
		templatesError = '';
		try {
			const result = catalogTemplatePage(
				await adminFetch<unknown>('/api/admin/email/templates?per_page=100')
			);
			templates = result.templates
				.filter((t) => t.is_active)
				.map((t) => ({ id: Number(t.id), name: t.name }));
		} catch (err) {
			templates = [];
			templatesError = err instanceof Error ? err.message : 'Email templates unavailable';
		}
	}
	async function loadSegments() {
		segmentsError = '';
		try {
			const result = await listMemberSegments({ per_page: 100, is_active: true });
			segments = result.segments.map((s) => ({ id: s.id, name: s.name, count: s.memberCount }));
		} catch (err) {
			segments = [];
			segmentsError = err instanceof Error ? err.message : 'Member segments unavailable';
		}
	}

	// View Report Handler

	function viewCampaignReport(campaign: APICampaign) {
		goto(`/admin/email/campaigns/${campaign.id}/report`);
	}

	function formatDate(dateStr: string | null): string {
		if (!dateStr) return '-';
		return new Date(dateStr).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'sent':
				return 'status-sent';
			case 'scheduled':
				return 'status-scheduled';
			case 'sending':
				return 'status-sending';
			default:
				return 'status-default';
		}
	}

	async function handleCreateCampaign() {
		if (creating) return;
		if (templatesError || segmentsError) {
			toastStore.error(
				'Reload the unavailable template or segment options before creating a campaign'
			);
			return;
		}
		// Validation
		if (!newCampaign.name || !newCampaign.subject) {
			toastStore.error('Please fill in campaign name and subject');
			return;
		}
		if (!newCampaign.templateId) {
			toastStore.error('Please select an email template');
			return;
		}
		if (newCampaign.useABTest && !newCampaign.subjectB) {
			toastStore.error('Please enter Subject B for A/B test');
			return;
		}

		creating = true;
		try {
			const campaignData = {
				name: newCampaign.name,
				subject: newCampaign.subject,
				template_id: parseInt(newCampaign.templateId),
				segment_id: newCampaign.segmentId ? parseInt(newCampaign.segmentId) : null,
				scheduled_at: newCampaign.scheduledFor
					? new Date(newCampaign.scheduledFor).toISOString()
					: null,
				ab_test_config: newCampaign.useABTest
					? {
							enabled: true,
							subject_b: newCampaign.subjectB,
							split_percentage: newCampaign.abTestSplit
						}
					: null
			};

			if (campaignData.scheduled_at && new Date(campaignData.scheduled_at).getTime() <= Date.now())
				throw new Error('Choose a future time to schedule this campaign');
			const saved = await createCampaign(campaignData);
			showCreateModal = false;
			activeTab = 'drafts';
			currentPage = 1;
			search = '';
			if (campaignData.scheduled_at) {
				try {
					const submitted = await scheduleCampaign(saved.id, campaignData.scheduled_at);
					activeTab = submitted.status === 'scheduled' ? 'scheduled' : 'all';
					toastStore.success(`Campaign scheduled. Current status: ${submitted.status}.`);
				} catch (error) {
					toastStore.error(
						`Draft saved. Scheduling was not confirmed. Refresh its status before retrying: ${error instanceof Error ? error.message : 'Refresh and try again'}`
					);
				}
			} else {
				toastStore.success('Campaign draft saved');
			}

			// Reset form
			newCampaign = {
				name: '',
				subject: '',
				subjectB: '',
				useABTest: false,
				abTestSplit: 50,
				scheduledFor: '',
				segmentId: '',
				templateId: ''
			};

			// Reload campaigns
			await Promise.all([loadCampaigns(), loadStats()]);
		} catch (err) {
			toastStore.error(err instanceof Error ? err.message : 'Failed to create campaign');
		} finally {
			creating = false;
		}
	}

	// Modal state for campaign actions
	let showDeleteModal = $state(false);
	let showSendModal = $state(false);
	let showCancelModal = $state(false);
	let pendingCampaignId = $state<number | null>(null);

	function handleDeleteCampaign(id: number) {
		pendingCampaignId = id;
		showDeleteModal = true;
	}

	async function confirmDeleteCampaign() {
		if (!pendingCampaignId) return;
		showDeleteModal = false;
		const id = pendingCampaignId;
		pendingCampaignId = null;
		try {
			await apiDeleteCampaign(id);
			toastStore.success('Campaign deleted');
			await Promise.all([loadCampaigns(), loadStats()]);
		} catch (err) {
			toastStore.error(err instanceof Error ? err.message : 'Failed to delete campaign');
		}
	}

	async function handleDuplicateCampaign(campaign: APICampaign) {
		try {
			await apiDuplicateCampaign(campaign.id);
			toastStore.success('Campaign duplicated');
			await Promise.all([loadCampaigns(), loadStats()]);
		} catch (err) {
			toastStore.error(err instanceof Error ? err.message : 'Failed to duplicate campaign');
		}
	}

	function handleSendCampaign(id: number) {
		pendingCampaignId = id;
		showSendModal = true;
	}

	async function confirmSendCampaign() {
		if (!pendingCampaignId) return;
		showSendModal = false;
		const id = pendingCampaignId;
		pendingCampaignId = null;
		await campaignAction(id, sendCampaign, 'Campaign submitted');
	}

	function handleCancelCampaign(id: number) {
		pendingCampaignId = id;
		showCancelModal = true;
	}

	async function confirmCancelCampaign() {
		if (!pendingCampaignId) return;
		showCancelModal = false;
		const id = pendingCampaignId;
		pendingCampaignId = null;
		await campaignAction(id, cancelCampaign, 'Campaign cancelled');
	}

	// Helper to get segment name
	function getSegmentName(segmentId: number | null): string {
		if (!segmentId) return 'All Members';
		const segment = segments.find((s) => s.id === segmentId);
		return segment?.name || 'Unknown';
	}
</script>

<div class="campaigns-page">
	<!-- Header -->
	<div class="page-header">
		<button class="back-btn" onclick={() => goto('/admin/email/templates')}>
			<IconArrowLeft size={20} />
			Back to Templates
		</button>

		<div class="header-content">
			<div class="header-title">
				<div class="title-icon">
					<IconMail size={28} />
				</div>
				<div>
					<h1>Email Campaigns</h1>
					<p class="subtitle">Create, schedule, and analyze email campaigns</p>
				</div>
			</div>

			<div class="header-actions">
				<button class="btn-secondary" onclick={loadCampaigns}>
					<IconRefresh size={18} />
					Refresh
				</button>
				<button class="btn-primary" onclick={() => (showCreateModal = true)}>
					<IconPlus size={18} />
					New Campaign
				</button>
			</div>
		</div>
	</div>

	{#if !connectionLoading && !getIsEmailConnected()}
		<p role="status">
			Email delivery is disconnected. Saved drafts remain available. Configure a delivery provider
			in Settings → Connections before sending.
		</p>
	{/if}
	<!-- Connection Check -->
	{#if connectionLoading}
		<SkeletonLoader variant="dashboard" />
	{:else if loading}
		<div class="loading-grid">
			{#each [1, 2, 3, 4] as _, i (i)}
				<div class="skeleton skeleton-metric"></div>
			{/each}
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button class="btn-primary" onclick={loadCampaigns}>Try Again</button>
		</div>
	{:else}
		{#if statsError}<p role="alert">
				{statsError} <button class="btn-secondary" onclick={loadStats}>Retry statistics</button>
			</p>{/if}
		<!-- Stats Overview -->
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon purple">
					<IconSend size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-value">{stats ? formatNumber(stats.total_sent) : '—'}</div>
					<div class="stat-label">Recorded Provider Acceptances</div>
				</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon emerald">
					<IconEye size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-value">{stats ? formatNumber(stats.total_opened) : '—'}</div>
					<div class="stat-label">Recorded Open Signals</div>
				</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon blue">
					<IconTrendingUp size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-value">
						{stats?.avg_open_rate == null ? '—' : `${stats.avg_open_rate}%`}
					</div>
					<div class="stat-label">Avg Open Rate</div>
				</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon yellow">
					<IconChartBar size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-value">
						{stats?.avg_click_rate == null ? '—' : `${stats.avg_click_rate}%`}
					</div>
					<div class="stat-label">Avg Click Rate</div>
				</div>
			</div>
		</div>

		<!-- Tabs -->
		<div class="tabs-container">
			<div class="tabs">
				<button class={{ active: activeTab === 'all' }} onclick={() => selectTab('all')}>
					All ({stats?.total_campaigns ?? '—'})
				</button>
				<button
					class={{ active: activeTab === 'scheduled' }}
					onclick={() => selectTab('scheduled')}
				>
					<IconClock size={16} />
					Scheduled ({scheduledCount})
				</button>
				<button class={{ active: activeTab === 'sent' }} onclick={() => selectTab('sent')}>
					<IconCheck size={16} />
					Sent ({sentCount})
				</button>
				<button class={{ active: activeTab === 'drafts' }} onclick={() => selectTab('drafts')}>
					<IconEdit size={16} />
					Drafts ({draftCount})
				</button>
			</div>
		</div>

		<!-- Campaigns List -->
		<form
			onsubmit={(event) => {
				event.preventDefault();
				currentPage = 1;
				void loadCampaigns();
			}}
		>
			<label for="campaign-search">Search campaigns</label>
			<input id="campaign-search" type="search" bind:value={search} />
			<button class="btn-secondary" type="submit">Search</button>
		</form>
		<div class="campaigns-list">
			{#each filteredCampaigns as campaign (campaign.id)}
				<div class="campaign-card">
					<div class="campaign-header">
						<div class="campaign-info">
							<div class="campaign-name-row">
								<h3>{campaign.name}</h3>
								{#if campaign.ab_test_config?.enabled}
									<span class="badge ab-badge">A/B Test</span>
								{/if}
							</div>
							<p class="campaign-subject">{campaign.subject || '(No subject)'}</p>
						</div>
						<span class={['status-badge', getStatusColor(campaign.status)]}>
							{campaign.status}
						</span>
					</div>

					<div class="campaign-meta">
						<div class="meta-item">
							<IconUsers size={16} />
							<span>{getSegmentName(campaign.segment_id)}</span>
							<span class="meta-count"
								>({formatNumber(campaign.total_recipients)} submitted recipients)</span
							>
						</div>
						{#if campaign.status === 'draft' && campaign.planned_at}
							<div class="meta-item">
								<IconCalendar size={16} /><span
									>Requested: {formatDate(campaign.planned_at)} · Draft not submitted</span
								>
							</div>
						{/if}
						{#if campaign.scheduled_at}
							<div class="meta-item">
								<IconCalendar size={16} />
								<span>Scheduled: {formatDate(campaign.scheduled_at)}</span>
							</div>
						{/if}
						{#if campaign.sent_at}
							<div class="meta-item">
								<IconClock size={16} />
								<span>Sent: {formatDate(campaign.sent_at)}</span>
							</div>
						{/if}
					</div>

					{#if campaign.status === 'sent' && campaign.sent_count > 0}
						<div class="campaign-stats">
							<div class="stat-item">
								<span class="stat-label">Sent</span>
								<span class="stat-value">{formatNumber(campaign.sent_count)}</span>
							</div>
							<div class="stat-item">
								<span class="stat-label">Opened</span>
								<span class="stat-value highlight">
									{formatNumber(campaign.opened_count)}
									<small>({campaign.open_rate?.toFixed(1) || 0}%)</small>
								</span>
							</div>
							<div class="stat-item">
								<span class="stat-label">Clicked</span>
								<span class="stat-value highlight-blue">
									{formatNumber(campaign.clicked_count)}
									<small>({campaign.click_rate?.toFixed(1) || 0}%)</small>
								</span>
							</div>
							<div class="stat-item">
								<span class="stat-label">Bounced</span>
								<span class="stat-value">{campaign.bounced_count}</span>
							</div>
						</div>

						{#if campaign.ab_test_config?.enabled && campaign.ab_test_config.variant_a}
							<div class="ab-test-results">
								<span class="ab-label">A/B Test Results:</span>
								<div class="ab-variants">
									<div class={['ab-variant', { winner: campaign.ab_test_config.winner === 'a' }]}>
										<span class="variant-label">A:</span>
										<span class="variant-subject">{campaign.subject}</span>
										<span class="variant-rate"
											>{campaign.ab_test_config.variant_a?.open_rate || 0}%</span
										>
									</div>
									<div class={['ab-variant', { winner: campaign.ab_test_config.winner === 'b' }]}>
										<span class="variant-label">B:</span>
										<span class="variant-subject">{campaign.ab_test_config.subject_b}</span>
										<span class="variant-rate"
											>{campaign.ab_test_config.variant_b?.open_rate || 0}%</span
										>
									</div>
								</div>
							</div>
						{/if}
					{/if}

					<div class="campaign-actions">
						{#if campaign.status === 'draft'}
							{#if campaign.planned_at}<button
									class="btn-secondary small"
									disabled={actionPending !== null}
									onclick={() => scheduleSaved(campaign)}
									><IconCalendar size={16} />Schedule requested time</button
								>{/if}
							<button class="btn-primary small" onclick={() => handleSendCampaign(campaign.id)}>
								<IconSend size={16} />
								Send Now
							</button>
							<button class="btn-secondary small" onclick={() => handleDuplicateCampaign(campaign)}>
								<IconCopy size={16} />
								Duplicate
							</button>
							<button
								class="btn-danger small"
								aria-label={`Delete ${campaign.name}`}
								onclick={() => handleDeleteCampaign(campaign.id)}
							>
								<IconTrash size={16} />
							</button>
						{:else if campaign.status === 'scheduled' || campaign.status === 'sending'}
							<button
								class="btn-secondary small"
								disabled={actionPending !== null}
								onclick={() => campaignAction(campaign.id, pauseCampaign, 'Future sends paused')}
								>Pause future sends</button
							>
							<button class="btn-secondary small" onclick={() => handleCancelCampaign(campaign.id)}>
								<IconX size={16} />
								Cancel
							</button>
							<button class="btn-secondary small" onclick={() => handleDuplicateCampaign(campaign)}>
								<IconCopy size={16} />
								Duplicate
							</button>
						{:else}
							{#if campaign.status === 'paused' || campaign.status === 'failed'}
								<button
									class="btn-primary small"
									disabled={actionPending !== null}
									onclick={() => campaignAction(campaign.id, resumeCampaign, 'Campaign submitted')}
									>Resume</button
								>
								<button
									class="btn-secondary small"
									onclick={() => handleCancelCampaign(campaign.id)}>Cancel</button
								>
							{/if}
							<button class="btn-secondary small" onclick={() => viewCampaignReport(campaign)}>
								<IconChartBar size={16} />
								View Report
							</button>
							<button class="btn-secondary small" onclick={() => handleDuplicateCampaign(campaign)}>
								<IconCopy size={16} />
								Duplicate
							</button>
						{/if}
					</div>
				</div>
			{/each}

			{#if filteredCampaigns.length === 0}
				<div class="empty-state">
					<IconMail size={48} stroke={1} />
					<h3>No campaigns found</h3>
					<p>Create a campaign or change the current filters</p>
					<button class="btn-primary" onclick={() => (showCreateModal = true)}>
						<IconPlus size={18} />
						Create Campaign
					</button>
				</div>
			{/if}
		</div>
		{#if totalPages > 1}
			<nav aria-label="Campaign pagination">
				<button
					class="btn-secondary"
					disabled={currentPage <= 1}
					onclick={() => {
						currentPage--;
						void loadCampaigns();
					}}>Previous</button
				>
				<span>Page {currentPage} of {totalPages} · {total} campaigns</span>
				<button
					class="btn-secondary"
					disabled={currentPage >= totalPages}
					onclick={() => {
						currentPage++;
						void loadCampaigns();
					}}>Next</button
				>
			</nav>
		{/if}
	{/if}
</div>

<!-- Create Campaign Modal -->
{#if showCreateModal}
	<div
		class="modal-overlay"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		onclick={(e: MouseEvent) => {
			if (e.target === e.currentTarget) showCreateModal = false;
		}}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showCreateModal = false)}
	>
		<div class="modal-content large" role="document">
			<div class="modal-header">
				<h2>Create New Campaign</h2>
				<button class="close-btn" onclick={() => (showCreateModal = false)}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				{#if templatesError}<p role="alert">
						{templatesError}
						<button class="btn-secondary" onclick={loadTemplates}>Retry templates</button>
					</p>{/if}
				{#if segmentsError}<p role="alert">
						{segmentsError}
						<button class="btn-secondary" onclick={loadSegments}>Retry segments</button>
					</p>{/if}
				<div class="form-grid">
					<div class="form-group full-width">
						<label for="campaign-name">Campaign Name</label>
						<input
							autocomplete="off"
							id="campaign-name"
							name="campaign-name"
							type="text"
							bind:value={newCampaign.name}
							placeholder="e.g., December Newsletter"
						/>
					</div>

					<div class="form-group full-width">
						<label for="campaign-subject">Subject Line</label>
						<input
							autocomplete="off"
							id="campaign-subject"
							name="campaign-subject"
							type="text"
							bind:value={newCampaign.subject}
							placeholder="e.g., Your Weekly Trading Insights"
						/>
					</div>

					<div class="form-group full-width ab-toggle">
						<label class="checkbox-label">
							<input
								id="enable-ab-test"
								name="enable-ab-test"
								type="checkbox"
								bind:checked={newCampaign.useABTest}
							/>
							<span>Enable A/B Testing</span>
						</label>
					</div>

					{#if newCampaign.useABTest}
						<div class="form-group full-width">
							<label for="campaign-subject-b">Subject Line B</label>
							<input
								autocomplete="off"
								id="campaign-subject-b"
								name="campaign-subject-b"
								type="text"
								bind:value={newCampaign.subjectB}
								placeholder="Alternative subject to test"
							/>
						</div>
						<div class="form-group">
							<label for="ab-split">A/B Split (%)</label>
							<input
								id="ab-split"
								name="ab-split"
								type="range"
								min="10"
								max="90"
								bind:value={newCampaign.abTestSplit}
							/>
							<span class="split-label"
								>{newCampaign.abTestSplit}% / {100 - newCampaign.abTestSplit}%</span
							>
						</div>
					{/if}

					<div class="form-group">
						<label for="campaign-segment">Target Segment</label>
						<select id="campaign-segment" bind:value={newCampaign.segmentId}>
							<option value="">All members with newsletter confirmation</option>
							{#each segments as segment (segment.id)}
								<option value={segment.id}>{segment.name} ({formatNumber(segment.count)})</option>
							{/each}
						</select>
					</div>

					<div class="form-group">
						<label for="campaign-template">Email Template</label>
						<select id="campaign-template" bind:value={newCampaign.templateId}>
							<option value="">Select template...</option>
							{#each templates as template (template.id)}
								<option value={template.id}>{template.name}</option>
							{/each}
						</select>
					</div>

					<div class="form-group full-width">
						<label for="campaign-schedule">Schedule (optional)</label>
						<input
							autocomplete="off"
							id="campaign-schedule"
							name="campaign-schedule"
							type="datetime-local"
							bind:value={newCampaign.scheduledFor}
						/>
						<small>Leave empty to save as draft</small>
					</div>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showCreateModal = false)}>Cancel</button>
				<button class="btn-primary" onclick={handleCreateCampaign} disabled={creating}>
					{#if newCampaign.scheduledFor}
						<IconCalendar size={18} />
						Schedule Campaign
					{:else}
						<IconEdit size={18} />
						Save as Draft
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete Campaign"
	message="Delete this campaign? This action cannot be undone."
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteCampaign}
	onCancel={() => {
		showDeleteModal = false;
		pendingCampaignId = null;
	}}
/>

<ConfirmationModal
	isOpen={showSendModal}
	title="Send Campaign"
	message="Submit this campaign now to confirmed subscribers in its selected audience?"
	confirmText="Send Now"
	variant="info"
	onConfirm={confirmSendCampaign}
	onCancel={() => {
		showSendModal = false;
		pendingCampaignId = null;
	}}
/>

<ConfirmationModal
	isOpen={showCancelModal}
	title="Cancel Campaign"
	message="Stop future submissions for this campaign? Previously accepted messages cannot be recalled."
	confirmText="Cancel Campaign"
	variant="warning"
	onConfirm={confirmCancelCampaign}
	onCancel={() => {
		showCancelModal = false;
		pendingCampaignId = null;
	}}
/>

<style>
	.campaigns-page {
		padding: 2rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	/* Header */
	.back-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0;
		color: var(--text-secondary, #8b949e);
		background: none;
		border: none;
		cursor: pointer;
		font-size: 0.875rem;
		margin-bottom: 1rem;
		transition: color 0.2s;
	}

	.back-btn:hover {
		color: var(--primary-500);
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.title-icon {
		width: 56px;
		height: 56px;
		background: var(--primary-500);
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--bg-base);
	}

	.header-title h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		margin: 0;
	}

	.subtitle {
		color: var(--text-tertiary, #6e7681);
		margin: 0.25rem 0 0;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	/* Stats */
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
	.stat-icon.yellow {
		background: rgba(210, 153, 34, 0.15);
		color: var(--warning-emphasis, #d29922);
	}

	.stat-content .stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
	}

	.stat-content .stat-label {
		font-size: 0.8125rem;
		color: var(--text-tertiary, #6e7681);
	}

	/* Tabs */
	.tabs-container {
		margin-bottom: 1.5rem;
		border-bottom: 1px solid var(--border-muted, #21262d);
	}

	.tabs {
		display: flex;
		gap: 0.5rem;
	}

	.tabs button {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: none;
		border: none;
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		border-bottom: 2px solid transparent;
		margin-bottom: -1px;
		transition: all 0.2s;
	}

	.tabs button:hover {
		color: var(--primary-500);
	}

	.tabs button.active {
		color: var(--primary-500);
		border-bottom-color: var(--primary-500);
	}

	/* Campaign List */
	.campaigns-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.campaign-card {
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 16px;
		padding: 1.5rem;
	}

	.campaign-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 1rem;
	}

	.campaign-name-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.campaign-name-row h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0;
	}

	.campaign-subject {
		color: var(--text-secondary, #8b949e);
		font-size: 0.875rem;
		margin: 0.25rem 0 0;
	}

	.badge {
		padding: 0.25rem 0.5rem;
		border-radius: 6px;
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.ab-badge {
		background: rgba(210, 153, 34, 0.2);
		color: var(--warning-emphasis, #d29922);
	}

	.status-badge {
		padding: 0.375rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.status-badge.status-sent {
		background: rgba(63, 185, 80, 0.2);
		color: var(--success-emphasis, #3fb950);
	}

	.status-badge.status-scheduled {
		background: rgba(88, 166, 255, 0.2);
		color: var(--info-emphasis, #58a6ff);
	}

	.status-badge.status-sending {
		background: rgba(210, 153, 34, 0.2);
		color: var(--warning-emphasis, #d29922);
	}

	.status-badge.status-default {
		background: var(--bg-hover, #252b33);
		color: var(--text-secondary, #8b949e);
	}

	.campaign-meta {
		display: flex;
		flex-wrap: wrap;
		gap: 1.5rem;
		margin-bottom: 1rem;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8125rem;
		color: var(--text-secondary, #8b949e);
	}

	.meta-count {
		color: var(--text-tertiary, #6e7681);
	}

	.campaign-stats {
		display: flex;
		gap: 2rem;
		padding: 1rem 0;
		border-top: 1px solid var(--border-muted, #21262d);
	}

	.campaign-stats .stat-item {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.campaign-stats .stat-label {
		font-size: 0.6875rem;
		color: var(--text-tertiary, #6e7681);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.campaign-stats .stat-value {
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
	}

	.campaign-stats .stat-value.highlight {
		color: var(--success-emphasis, #3fb950);
	}

	.campaign-stats .stat-value.highlight-blue {
		color: var(--info-emphasis, #58a6ff);
	}

	.campaign-stats .stat-value small {
		font-size: 0.75rem;
		font-weight: 400;
		color: var(--text-tertiary, #6e7681);
	}

	/* A/B Test Results */
	.ab-test-results {
		padding: 1rem;
		background: var(--bg-base, #0d1117);
		border-radius: 10px;
		margin-top: 1rem;
	}

	.ab-label {
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-secondary, #8b949e);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		display: block;
		margin-bottom: 0.75rem;
	}

	.ab-variants {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.ab-variant {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.5rem 0.75rem;
		background: var(--bg-surface, #1c2128);
		border-radius: 6px;
	}

	.ab-variant.winner {
		background: rgba(63, 185, 80, 0.1);
		border: 1px solid rgba(63, 185, 80, 0.3);
	}

	.variant-label {
		font-weight: 600;
		color: var(--primary-500);
	}

	.variant-subject {
		flex: 1;
		color: var(--text-secondary, #8b949e);
		font-size: 0.875rem;
	}

	.variant-rate {
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
	}

	.ab-variant.winner .variant-rate {
		color: var(--success-emphasis, #3fb950);
	}

	/* Campaign Actions */
	.campaign-actions {
		display: flex;
		gap: 0.5rem;
		padding-top: 1rem;
		border-top: 1px solid var(--border-muted, #21262d);
		margin-top: 1rem;
	}

	/* Buttons */
	.btn-primary,
	.btn-secondary,
	.btn-danger {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
		font-size: 0.875rem;
	}

	.btn-primary {
		background: var(--primary-500);
		color: var(--bg-base);
	}

	.btn-secondary {
		background: var(--bg-surface, #1c2128);
		color: var(--text-secondary, #8b949e);
		border: 1px solid var(--border-default, #30363d);
	}

	.btn-danger {
		background: rgba(248, 81, 73, 0.1);
		color: var(--error-emphasis, #f85149);
		border: 1px solid rgba(248, 81, 73, 0.2);
	}

	.btn-primary.small,
	.btn-secondary.small,
	.btn-danger.small {
		padding: 0.5rem 0.875rem;
		font-size: 0.8125rem;
	}

	/* Error State */
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 2rem;
		color: var(--error-emphasis, #f85149);
		text-align: center;
		background: rgba(248, 81, 73, 0.1);
		border-radius: 16px;
		border: 1px solid rgba(248, 81, 73, 0.2);
		gap: 1rem;
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: var(--text-tertiary, #6e7681);
		text-align: center;
		background: rgba(30, 41, 59, 0.3);
		border-radius: 16px;
		border: 2px dashed var(--border-default, #30363d);
	}

	.empty-state h3 {
		color: var(--text-primary, #f0f6fc);
		margin: 1rem 0 0.5rem;
	}

	.empty-state p {
		margin-bottom: 1.5rem;
	}

	/* Loading */
	.loading-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.skeleton {
		background: linear-gradient(
			90deg,
			var(--bg-surface, #1c2128) 25%,
			var(--bg-hover, #252b33) 50%,
			var(--bg-surface, #1c2128) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 16px;
	}

	.skeleton-metric {
		height: 100px;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.8);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 50;
		padding: 2rem;
	}

	.modal-content {
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 20px;
		width: 100%;
		max-width: 600px;
		max-height: 90vh;
		overflow-y: auto;
	}

	.modal-content.large {
		max-width: 700px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid var(--border-muted, #21262d);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-surface, #1c2128);
		border: none;
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
	}

	.modal-body {
		padding: 1.5rem;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid var(--border-muted, #21262d);
	}

	/* Form */
	.form-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group.full-width {
		grid-column: 1 / -1;
	}

	.form-group label {
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--text-secondary, #8b949e);
	}

	.form-group input,
	.form-group select {
		padding: 0.75rem 1rem;
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 10px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.9375rem;
	}

	.form-group input:focus,
	.form-group select:focus {
		outline: none;
		border-color: rgba(230, 184, 0, 0.5);
	}

	.form-group small {
		color: var(--text-tertiary, #6e7681);
		font-size: 0.75rem;
	}

	.ab-toggle {
		padding: 0.75rem 0;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
	}

	.checkbox-label input[type='checkbox'] {
		width: 18px;
		height: 18px;
		accent-color: var(--primary-500);
	}

	.checkbox-label span {
		color: var(--text-primary, #f0f6fc);
		font-weight: 500;
	}

	.split-label {
		display: block;
		text-align: center;
		color: var(--text-secondary, #8b949e);
		font-size: 0.875rem;
		margin-top: 0.25rem;
	}

	input[type='range'] {
		width: 100%;
		accent-color: var(--primary-500);
	}

	@media (max-width: 767.98px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.header-content {
			flex-direction: column;
			gap: 1rem;
			align-items: flex-start;
		}

		.form-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
