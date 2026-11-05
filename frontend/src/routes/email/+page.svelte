<!--
	Email Marketing Dashboard
	═══════════════════════════════════════════════════════════════════════════
	
	Email campaign authoring, recorded analytics and automation navigation.
-->

<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import {
		emailApi,
		type EmailTemplate,
		type EmailCampaign,
		type EmailAnalytics,
		type EmailSequence
	} from '#lib/api/email.js';
	import { emailStore, getIsEmailLoading } from '#lib/stores/email.svelte.js';
	import ConfirmationModal from '#lib/components/admin/ConfirmationModal.svelte';
	import SequenceTestDialog from '#lib/components/admin/crm/SequenceTestDialog.svelte';
	import {
		IconMail,
		IconSend,
		IconUsers,
		IconChartBar,
		IconPlus,
		IconFileText,
		IconRocket,
		IconBolt
	} from '#lib/icons/index.js';

	let selectedTab: 'campaigns' | 'sequences' | 'automations' | 'templates' | 'analytics' =
		$state('campaigns');

	let sequenceCatalog = $state<EmailSequence[]>([]);
	let sequenceLoading = $state(false), sequenceError = $state('');
	let sequenceSearch = $state(''), sequenceStatus = $state('');
	let sequencePage = $state(1), sequenceTotal = $state(0), sequencePages = $state(1);
	let sequenceGeneration = 0;
	let sequenceActionPending = $state<string | null>(null), sequenceActionError = $state('');
	let sequenceNotice = $state(''), sequenceCopyId = $state<string | null>(null);
	let sequenceToDelete = $state<EmailSequence | null>(null);
	let testSequenceId = $state<string | null>(null);
	async function loadSequences(page = 1) {
		const generation = ++sequenceGeneration;
		sequenceLoading = true; sequenceError = '';
		try {
			const result = await emailApi.getSequences({ page, per_page: 20, status: sequenceStatus || undefined, search: sequenceSearch || undefined });
			if (generation !== sequenceGeneration) return;
			if (result.page > result.last_page) { await loadSequences(result.last_page); return; }
			sequenceCatalog = result.sequences; sequencePage = result.page; sequenceTotal = result.total; sequencePages = result.last_page;
		} catch (cause) { if (generation === sequenceGeneration) { sequenceCatalog = []; sequenceError = cause instanceof Error ? cause.message : 'Saved sequences are unavailable'; } }
		finally { if (generation === sequenceGeneration) sequenceLoading = false; }
	}
	async function sequenceAction(sequence: EmailSequence, action: 'duplicate' | 'status' | 'delete') {
		if (sequenceActionPending !== null) return;
		sequenceActionPending = sequence.id; sequenceActionError = ''; sequenceNotice = ''; sequenceCopyId = null;
		try {
			if (action === 'duplicate') {
				const result = await emailApi.duplicateSequence(sequence.id);
				sequenceCopyId = result.sequence.id; sequenceNotice = 'Sequence duplicated as a separate draft.';
				sequenceStatus = ''; await loadSequences(1);
			} else if (action === 'status') {
				const status = sequence.status === 'active' ? 'paused' : 'active';
				await emailApi.updateSequence(sequence.id, { status }, sequence.authoring_version);
				sequenceNotice = status === 'active' ? 'Saved sequence activated.' : 'Sequence paused. Existing in-flight submissions retain their actual receipts.';
				await loadSequences(sequencePage);
			} else {
				await emailApi.deleteSequence(sequence.id); sequenceToDelete = null;
				sequenceNotice = 'Sequence archived. Existing enrollment history is retained.';
				await loadSequences(sequencePage);
			}
		} catch (cause) { sequenceActionError = `The action was not confirmed. Refresh saved state before retrying: ${cause instanceof Error ? cause.message : 'Sequence operation unavailable'}`; }
		finally { sequenceActionPending = null; }
	}
	let campaignCatalog = $state<EmailCampaign[]>([]);
	let campaignLoading = $state(true),
		campaignError = $state('');
	let campaignSearch = $state(''),
		campaignStatus = $state('');
	let campaignPage = $state(1),
		campaignTotal = $state(0),
		campaignPerPage = $state(20);
	let campaignGeneration = 0;
	let campaignActionPending = $state<string | null>(null),
		campaignActionError = $state('');
	async function loadCampaigns(targetPage = 1) {
		const generation = ++campaignGeneration;
		campaignLoading = true;
		campaignError = '';
		try {
			const result = await emailApi.getCampaigns({
				page: targetPage,
				per_page: campaignPerPage,
				search: campaignSearch.trim() || undefined,
				status: campaignStatus || undefined
			});
			if (generation !== campaignGeneration) return;
			const lastPage = Math.max(1, Math.ceil(result.total / result.per_page));
			if (result.page > lastPage) {
				await loadCampaigns(lastPage);
				return;
			}
			campaignCatalog = result.campaigns;
			campaignTotal = result.total;
			campaignPage = result.page;
			campaignPerPage = result.per_page;
		} catch (error) {
			if (generation === campaignGeneration) {
				campaignCatalog = [];
				campaignTotal = 0;
				campaignError = error instanceof Error ? error.message : 'Unable to load campaigns';
			}
		} finally {
			if (generation === campaignGeneration) campaignLoading = false;
		}
	}
	async function duplicateCampaign(id: string) {
		if (campaignActionPending !== null) return;
		campaignActionPending = id;
		campaignActionError = '';
		try {
			await emailApi.duplicateCampaign(id);
			campaignSearch = '';
			campaignStatus = '';
			await loadCampaigns(1);
		} catch (error) {
			campaignActionError = `Duplication was not confirmed. Refresh before retrying: ${error instanceof Error ? error.message : 'Unable to duplicate campaign'}`;
		} finally {
			campaignActionPending = null;
		}
	}
	let analytics = $state<EmailAnalytics | null>(null);
	let analyticsLoading = $state(true),
		analyticsError = $state('');
	let analyticsPeriod = $state<EmailAnalytics['period']>('30d');
	let analyticsGeneration = 0;
	async function loadAnalytics(period: string = analyticsPeriod) {
		const generation = ++analyticsGeneration;
		analyticsLoading = true;
		analyticsError = '';
		analytics = null;
		try {
			if (!['7d', '30d', '90d', '365d'].includes(period))
				throw new Error('Choose a supported reporting period');
			analyticsPeriod = period as EmailAnalytics['period'];
			const response = await emailApi.getAnalytics(period);
			if (generation === analyticsGeneration) analytics = response.analytics;
		} catch (error) {
			if (generation === analyticsGeneration)
				analyticsError =
					error instanceof Error ? error.message : 'Recorded analytics are unavailable';
		} finally {
			if (generation === analyticsGeneration) analyticsLoading = false;
		}
	}
	function selectTab(tab: typeof selectedTab) {
		selectedTab = tab;
		if (tab === 'sequences') void loadSequences();
		else if (tab === 'automations') void emailStore.loadAutomations();
	}
	function formatChange(value: number | null, suffix: string): string {
		return value === null
			? 'No comparable prior cohort'
			: `${value >= 0 ? '+' : ''}${value.toFixed(1)}${suffix} vs previous period`;
	}
	let templateCatalog = $state<EmailTemplate[]>([]);
	let templateLoading = $state(true);
	let templateError = $state('');
	let templateSearch = $state('');
	let templateCategory = $state('');
	let templatePage = $state(1);
	let templateTotal = $state(0);
	let templateGeneration = 0;
	const templatePerPage = 20;
	onMount(() => {
		void loadAnalytics();
		void loadTemplates();
		void loadCampaigns();
	});
	onDestroy(() => {
		templateGeneration++;
		campaignGeneration++;
		analyticsGeneration++;
		sequenceGeneration++;
	});
	async function loadTemplates(page = 1) {
		const generation = ++templateGeneration;
		templateLoading = true;
		templateError = '';
		try {
			const result = await emailApi.getTemplates({
				page,
				per_page: templatePerPage,
				search: templateSearch.trim() || undefined,
				category: templateCategory.trim() || undefined
			});
			if (generation !== templateGeneration) return;
			templateCatalog = result.templates;
			templateTotal = result.total;
			templatePage = page;
		} catch (error) {
			if (generation === templateGeneration)
				templateError = error instanceof Error ? error.message : 'Unable to load templates';
		} finally {
			if (generation === templateGeneration) templateLoading = false;
		}
	}

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	function formatPercent(num: number | null | undefined): string {
		if (num == null) return '—';
		return num.toFixed(1) + '%';
	}
</script>

<div class="email-dashboard">
	<!-- Header -->
	<div class="dashboard-header">
		<div class="header-title">
			<IconMail size={32} class="email-header-icon" />
			<div>
				<h1 class="email-title">Email Marketing</h1>
				<p class="email-subtitle">Campaigns, sequences, and automation</p>
			</div>
		</div>

		<div class="header-actions">
			<button class="btn-secondary" onclick={() => (selectedTab = 'templates')}>
				<IconFileText size={18} />
				Templates
			</button>
			<a class="btn-primary" href="/admin/email/campaigns?new=1"
				><IconPlus size={18} />New Campaign</a
			>
		</div>
	</div>

	<div class="analytics-controls">
		<label for="email-analytics-period">Reporting period</label><select
			id="email-analytics-period"
			bind:value={analyticsPeriod}
			onchange={(event) => loadAnalytics(event.currentTarget.value)}
			><option value="7d">Last 7 days</option><option value="30d">Last 30 days</option><option
				value="90d">Last 90 days</option
			><option value="365d">Last 365 days</option></select
		><button class="btn-secondary" onclick={() => loadAnalytics()}>Refresh analytics</button>
	</div>
	{#if analyticsLoading}<p role="status">Loading recorded campaign analytics...</p>{/if}
	{#if analyticsError}<p role="alert">
			{analyticsError}<button class="btn-secondary" onclick={() => loadAnalytics()}
				>Retry analytics</button
			>
		</p>{/if}
	<!-- Stats Overview -->
	{#if analytics}
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon stat-icon--blue">
					<IconSend size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-label">Campaign Provider Acceptances</div>
					<div class="stat-value">{formatNumber(analytics.emails_sent)}</div>
					<div class="stat-change">
						{formatChange(analytics.comparison.emails_sent_change_pct, '%')}
					</div>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon stat-icon--green">
					<IconChartBar size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-label">Recipient Open Rate</div>
					<div class="stat-value">{formatPercent(analytics.average_open_rate)}</div>
					<div class="stat-change">
						{formatChange(analytics.comparison.open_rate_change_points, ' pp')}
					</div>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon stat-icon--purple">
					<IconUsers size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-label">Recipient Click Rate</div>
					<div class="stat-value">{formatPercent(analytics.average_click_rate)}</div>
					<div class="stat-change">
						{formatChange(analytics.comparison.click_rate_change_points, ' pp')}
					</div>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon stat-icon--yellow">
					<IconRocket size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-label">Active Campaigns</div>
					<div class="stat-value">{analytics.campaigns}</div>
					<div class="stat-change">
						Scheduled or sending · {analytics.new_campaigns} created this period
					</div>
				</div>
			</div>
		</div>
	{/if}

	<!-- Tabs -->
	<div class="tabs">
		<button
			class={['tab', { active: selectedTab === 'campaigns' }]}
			onclick={() => (selectedTab = 'campaigns')}
		>
			<IconMail size={20} />
			Campaigns
		</button>
		<button
			class={['tab', { active: selectedTab === 'sequences' }]}
			onclick={() => selectTab('sequences')}
		>
			<IconBolt size={20} />
			Sequences
		</button>
		<button
			class={['tab', { active: selectedTab === 'automations' }]}
			onclick={() => selectTab('automations')}
		>
			<IconRocket size={20} />
			Automations
		</button>
		<button
			class={['tab', { active: selectedTab === 'templates' }]}
			onclick={() => (selectedTab = 'templates')}
		>
			<IconFileText size={20} />
			Templates
		</button>
		<button
			class={['tab', { active: selectedTab === 'analytics' }]}
			onclick={() => (selectedTab = 'analytics')}
		>
			<IconChartBar size={20} />
			Analytics
		</button>
	</div>

	<!-- Content -->
	<div class="tab-content">
		{#if selectedTab === 'templates' ? templateLoading : selectedTab === 'campaigns' ? campaignLoading : selectedTab === 'analytics' ? analyticsLoading : selectedTab === 'sequences' ? sequenceLoading : getIsEmailLoading()}
			<div class="loading-state">
				<div class="spinner"></div>
				<p class="state-text state-text--spaced">Loading...</p>
			</div>
		{:else if selectedTab === 'campaigns'}
			<div class="campaigns-list">
				<div class="list-header">
					<h2 class="section-heading">Email Campaigns</h2>
					<a class="btn-primary" href="/admin/email/campaigns?new=1"
						><IconPlus size={18} />Create Campaign</a
					>
				</div>

				<form
					class="campaign-filters"
					onsubmit={(event) => {
						event.preventDefault();
						void loadCampaigns(1);
					}}
				>
					<label for="secondary-campaign-search">Search campaigns</label><input
						id="secondary-campaign-search"
						type="search"
						bind:value={campaignSearch}
					/>
					<label for="secondary-campaign-status">Campaign status</label><select
						id="secondary-campaign-status"
						bind:value={campaignStatus}
						><option value="">All statuses</option
						>{#each ['draft', 'scheduled', 'sending', 'sent', 'paused', 'failed', 'cancelled', 'archived'] as status (status)}<option
								value={status}>{status}</option
							>{/each}</select
					>
					<button class="btn-secondary" type="submit">Search campaigns</button><button
						class="btn-secondary"
						type="button"
						onclick={() => loadCampaigns(campaignPage)}>Refresh campaigns</button
					>
				</form>
				{#if campaignActionError}<p role="alert">{campaignActionError}</p>{/if}
				{#if campaignError}<p role="alert">{campaignError}</p>
					<button class="btn-secondary" onclick={() => loadCampaigns(campaignPage)}
						>Retry campaigns</button
					>
				{:else if campaignCatalog.length > 0}
					<div class="campaigns-grid">
						{#each campaignCatalog as campaign (campaign.id)}
							<div class="campaign-card">
								<div class="campaign-header">
									<div class={['campaign-status', { active: campaign.status === 'sending' }]}>
										{campaign.status}
									</div>
									<div class="campaign-type">{campaign.type}</div>
								</div>

								<h3 class="campaign-name">{campaign.name}</h3>
								<p class="campaign-subject">{campaign.subject}</p>

								{#if campaign.stats}
									<div class="campaign-stats">
										<div class="stat-item">
											<span class="stat-label">Sent</span>
											<span class="stat-value">{formatNumber(campaign.stats.sent)}</span>
										</div>
										<div class="stat-item">
											<span class="stat-label">Opens</span>
											<span class="stat-value">{formatPercent(campaign.stats.open_rate)}</span>
										</div>
										<div class="stat-item">
											<span class="stat-label">Clicks</span>
											<span class="stat-value">{formatPercent(campaign.stats.click_rate)}</span>
										</div>
									</div>
								{/if}

								<div class="campaign-actions">
									<a class="action-btn" href={`/admin/email/campaigns/${campaign.id}/report`}
										>View</a
									>
									{#if ['draft', 'paused', 'failed'].includes(campaign.status)}<a
											class="action-btn"
											href={`/admin/email/campaigns?edit=${campaign.id}`}>Edit</a
										>{/if}
									<button
										class="action-btn"
										disabled={campaignActionPending !== null}
										onclick={() => duplicateCampaign(campaign.id)}>Duplicate</button
									>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<div class="empty-state">
						<IconMail size={64} class="empty-icon" />
						<h3 class="empty-title">No campaigns yet</h3>
						<p class="empty-copy">Create your first email campaign to get started</p>
						<a class="btn-primary empty-action" href="/admin/email/campaigns?new=1"
							><IconPlus size={18} />Create Campaign</a
						>
					</div>
				{/if}
				{#if campaignTotal > 0}<nav aria-label="Campaign pages">
						<button
							class="btn-secondary"
							disabled={campaignPage <= 1}
							onclick={() => loadCampaigns(campaignPage - 1)}>Previous campaigns</button
						><span
							>Page {campaignPage} of {Math.max(1, Math.ceil(campaignTotal / campaignPerPage))} · {campaignTotal}
							campaigns</span
						><button
							class="btn-secondary"
							disabled={campaignPage * campaignPerPage >= campaignTotal}
							onclick={() => loadCampaigns(campaignPage + 1)}>Next campaigns</button
						>
					</nav>{/if}
			</div>
		{:else if selectedTab === 'sequences'}
			<div class="sequences-list">
				<div class="list-header">
					<h2 class="section-heading">Email Sequences</h2>
					<a class="btn-primary" href="/admin/crm/sequences/new"><IconPlus size={18} />Create Sequence</a>
				</div>

				<form class="campaign-filters" onsubmit={(event) => { event.preventDefault(); void loadSequences(); }}>
					<label for="sequence-search">Search sequences</label><input id="sequence-search" bind:value={sequenceSearch} maxlength={255} />
					<label for="sequence-status">Sequence status</label><select id="sequence-status" bind:value={sequenceStatus}><option value="">All statuses</option><option value="draft">Draft</option><option value="active">Active</option><option value="paused">Paused</option><option value="completed">Completed</option></select><button class="btn-secondary" type="submit">Filter sequences</button>
				</form>
				{#if sequenceActionError}<p role="alert">{sequenceActionError}</p><button class="btn-secondary" disabled={sequenceLoading || sequenceActionPending !== null} onclick={() => void loadSequences(sequencePage)}>Refresh saved sequences</button>{/if}
				{#if sequenceNotice}<p role="status">{sequenceNotice}{#if sequenceCopyId} <a href="/admin/crm/sequences/{sequenceCopyId}/edit">Edit duplicated sequence</a>{/if}</p>{/if}
				{#if sequenceLoading}<p role="status">Loading saved sequences…</p>
				{:else if sequenceError}<p role="alert">{sequenceError}</p>
					<button class="btn-secondary" onclick={() => loadSequences(sequencePage)}
						>Retry sequences</button
					>{:else if sequenceCatalog.length > 0}
					<div class="sequences-grid">
						{#each sequenceCatalog as sequence (sequence.id)}
							<div class="sequence-card">
								<div class="sequence-header">
									<h3 class="sequence-name">{sequence.name}</h3>
									<div class={['sequence-status', { active: sequence.status === 'active' }]}>
										{sequence.status}
									</div>
								</div>

								<p class="sequence-description">{sequence.description || 'No description'}</p>
								{#if sequence.definition_error}<p role="alert">{sequence.definition_error}</p>{/if}
								{#if sequence.trigger_error}<p role="alert">Trigger needs review: {sequence.trigger_error}.{#if sequence.trigger_type === 'legacy'} Stored value: <code>{sequence.historical_trigger_type}</code>.{/if} <a href="/admin/crm/sequences/{sequence.id}/edit">Review trigger</a></p>{/if}

								<div class="sequence-info">
									<div class="info-item">
										<span class="info-label">Emails</span>
										<span class="info-value"
											>{sequence.definition_error ? 'Needs review' : sequence.emails.length}</span
										>
									</div>
									<div class="info-item">
										<span class="info-label">Subscribers</span>
										<span class="info-value">{formatNumber(sequence.subscribers_count)}</span>
									</div>
								</div>

								<div class="sequence-actions">
									<a class="action-btn" href="/admin/crm/sequences/{sequence.id}">View</a>
									<a class="action-btn" href="/admin/crm/sequences/{sequence.id}/edit">Edit</a>
									<a class="action-btn" href="/admin/crm/sequences/{sequence.id}/subscribers">Subscribers</a>
									<button class="action-btn" disabled={sequenceActionPending !== null} onclick={() => void sequenceAction(sequence, 'duplicate')}>Duplicate</button>
									<button class="action-btn" disabled={sequenceActionPending !== null} onclick={() => void sequenceAction(sequence, 'status')}>{sequence.status === 'active' ? 'Pause' : 'Activate'}</button>
									<button class="action-btn" disabled={sequenceActionPending !== null} onclick={() => { testSequenceId = sequence.id; }}>Test message</button>
									<button class="action-btn" disabled={sequenceActionPending !== null || sequence.status === 'active'} title={sequence.status === 'active' ? 'Pause the sequence before deleting it' : 'Archive this sequence and preserve enrollment history'} onclick={() => { sequenceToDelete = sequence; }}>Delete</button>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<div class="empty-state">
						<IconBolt size={64} class="empty-icon" />
						<h3 class="empty-title">No sequences yet</h3>
						<p class="empty-copy">Create automated email sequences</p>
						<a class="btn-primary empty-action" href="/admin/crm/sequences/new">
							<IconPlus size={18} />
							Create Sequence
						</a>
					</div>
				{/if}
				{#if !sequenceLoading && !sequenceError}<nav aria-label="Sequence pages"><button onclick={() => void loadSequences(sequencePage - 1)} disabled={sequencePage <= 1 || sequenceActionPending !== null}>Previous page</button><span>{sequenceTotal} matching sequences · Page {sequencePage} of {sequencePages}</span><button onclick={() => void loadSequences(sequencePage + 1)} disabled={sequencePage >= sequencePages || sequenceActionPending !== null}>Next page</button></nav>{/if}
			</div>
		{:else if selectedTab === 'templates'}
			<div class="templates-list">
				<div class="list-header">
					<h2 class="section-heading">Email Templates</h2>
					<a class="btn-primary" href="/admin/email/templates/new">
						<IconPlus size={18} />
						Create Template
					</a>
				</div>

				<form
					class="catalog-filters"
					onsubmit={(event) => {
						event.preventDefault();
						void loadTemplates();
					}}
				>
					<label>Search templates<input bind:value={templateSearch} type="search" /></label>
					<label>Template category<input bind:value={templateCategory} maxlength="50" /></label>
					<button class="btn-secondary" type="submit">Filter templates</button>
				</form>
				{#if templateError}
					<div role="alert" class="empty-state">
						<p class="empty-copy">{templateError}</p>
						<button class="btn-secondary" onclick={() => loadTemplates(templatePage)}
							>Retry templates</button
						>
					</div>
				{:else if templateCatalog.length > 0}
					<div class="templates-grid">
						{#each templateCatalog as template (template.id)}
							<div class="template-card">
								{#if template.thumbnail}
									<img
										src={template.thumbnail}
										alt={template.name}
										class="template-thumbnail"
										width="320"
										height="192"
										loading="lazy"
									/>
								{:else}
									<div class="template-placeholder">
										<IconFileText size={48} class="placeholder-icon" />
									</div>
								{/if}

								<div class="template-info">
									<h3 class="template-name">{template.name}</h3>
									<p class="template-category">{template.category}</p>
									<div
										class="template-usage"
										title="Provider-accepted messages linked to this template"
									>
										Recorded acceptances: {template.usage_count}
									</div>
								</div>

								<div class="template-actions">
									<a
										class="action-btn"
										href={`/admin/email/campaigns?new=1&template=${template.id}`}>Use Template</a
									>
									<a class="action-btn" href={`/admin/email/templates/preview/${template.id}`}
										>Preview</a
									>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<div class="empty-state">
						<IconFileText size={64} class="empty-icon" />
						<h3 class="empty-title">No templates yet</h3>
						<p class="empty-copy">Create reusable email templates</p>
						<a class="btn-primary empty-action" href="/admin/email/templates/new">
							<IconPlus size={18} />
							Create Template
						</a>
					</div>
				{/if}
				<nav class="catalog-pagination" aria-label="Template pages">
					<button
						class="btn-secondary"
						disabled={templatePage <= 1}
						onclick={() => loadTemplates(templatePage - 1)}>Previous templates</button
					>
					<span>Page {templatePage} · {templateTotal} templates</span>
					<button
						class="btn-secondary"
						disabled={templatePage * templatePerPage >= templateTotal}
						onclick={() => loadTemplates(templatePage + 1)}>Next templates</button
					>
				</nav>
			</div>
		{:else}
			<div class="analytics-view">
				<h2 class="section-heading analytics-heading">Email Analytics</h2>
				<p class="analytics-copy">
					Native campaign receipts and recorded observations in UTC. Recipient rates use unique
					opens or clicks among messages accepted in the selected period, observed through the end
					of the window. Signals may include automated requests. Historical totals and test messages
					are excluded.
				</p>
				{#if analytics}
					<p>
						{new Date(analytics.window.start).toLocaleString()} – {new Date(
							analytics.window.end
						).toLocaleString()}
					</p>
					<div class="analytics-panels">
						<section>
							<h3>Recorded signals by UTC date</h3>
							{#if analytics.engagement_trend.length}<table>
									<thead
										><tr
											><th scope="col">UTC date</th><th scope="col">Open signals</th><th scope="col"
												>Click signals</th
											></tr
										></thead
									><tbody
										>{#each analytics.engagement_trend as point (point.date)}<tr
												><td>{point.date}</td><td>{formatNumber(point.opens)}</td><td
													>{formatNumber(point.clicks)}</td
												></tr
											>{/each}</tbody
									>
								</table>{:else}<p>No recorded signals in this period.</p>{/if}
						</section>
						<section>
							<h3>Campaigns with accepted messages</h3>
							{#if analytics.top_campaigns.length}<table>
									<thead
										><tr
											><th scope="col">Campaign</th><th scope="col">Accepted</th><th scope="col"
												>Recipients with an open signal</th
											><th scope="col">Open rate</th></tr
										></thead
									><tbody
										>{#each analytics.top_campaigns as campaign (campaign.id)}<tr
												><td
													><a href={`/admin/email/campaigns/${campaign.id}/report`}
														>{campaign.name}</a
													></td
												><td>{formatNumber(campaign.sent)}</td><td
													>{formatNumber(campaign.opens)}</td
												><td>{formatPercent(campaign.open_rate)}</td></tr
											>{/each}</tbody
									>
								</table>{:else}<p>No messages accepted in this period.</p>{/if}
						</section>
						<section>
							<h3>Devices supplied with open observations</h3>
							<table>
								<thead><tr><th scope="col">Device</th><th scope="col">Observations</th></tr></thead
								><tbody
									>{#each Object.entries(analytics.device_breakdown) as [device, count] (device)}<tr
											><td>{device}</td><td>{formatNumber(count)}</td></tr
										>{/each}</tbody
								>
							</table>
						</section>
						<section>
							<h3>Email client observations</h3>
							{#if Object.keys(analytics.client_breakdown).length}<table>
									<thead
										><tr><th scope="col">Reported client</th><th scope="col">Observations</th></tr
										></thead
									><tbody
										>{#each Object.entries(analytics.client_breakdown) as [client, count] (client)}<tr
												><td>{client}</td><td>{formatNumber(count)}</td></tr
											>{/each}</tbody
									>
								</table>{:else}<p>No email client observations supplied.</p>{/if}
							<p>
								Other or unspecified client observations: {analytics.unlisted_client_observations}
							</p>
						</section>
					</div>
				{:else if !analyticsError}<p>No recorded analytics available.</p>{/if}
			</div>
		{/if}
	</div>
</div>

<ConfirmationModal
	isOpen={sequenceToDelete !== null}
	title="Delete Sequence"
	message="Archive this sequence and stop future submissions? Saved revisions and enrollment history are retained."
	confirmText="Delete"
	variant="danger"
	isLoading={sequenceActionPending !== null}
	onConfirm={() => { if (sequenceToDelete) void sequenceAction(sequenceToDelete, 'delete'); }}
	onCancel={() => { if (sequenceActionPending === null) sequenceToDelete = null; }}
/>
{#if testSequenceId}{#key testSequenceId}<SequenceTestDialog sequenceId={testSequenceId} onclose={() => { testSequenceId = null; }} />{/key}{/if}

<style>
	.catalog-filters,
	.catalog-pagination {
		display: flex;
		flex-wrap: wrap;
		align-items: end;
		gap: 1rem;
		margin-block: 1rem;
		color: #d1d5db;
	}
	.catalog-filters label {
		display: grid;
		gap: 0.4rem;
	}
	.catalog-filters input {
		background: #0f172a;
		border: 1px solid #64748b;
		border-radius: 0.4rem;
		color: white;
		padding: 0.5rem;
	}
	.catalog-pagination {
		align-items: center;
	}

	.campaign-filters {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		flex-wrap: wrap;
		margin-bottom: 1rem;
	}
	.campaign-filters input,
	.campaign-filters select {
		color: var(--text-primary);
		background: var(--bg-elevated);
		border: 1px solid var(--border-default);
		border-radius: 0.5rem;
		padding: 0.5rem;
	}
	.campaigns-list nav {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-top: 1rem;
	}
	a.btn-primary,
	a.action-btn {
		text-decoration: none;
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
	}
	.analytics-controls {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		flex-wrap: wrap;
		margin-block: 1rem;
	}
	.analytics-controls select {
		color: var(--text-primary);
		background: var(--bg-elevated);
		border: 1px solid var(--border-default);
		padding: 0.5rem;
		border-radius: 0.5rem;
	}
	.analytics-panels {
		display: grid;
		gap: 1.5rem;
		grid-template-columns: repeat(auto-fit, minmax(min(100%, 26rem), 1fr));
	}
	.analytics-panels section {
		background: var(--bg-elevated);
		border: 1px solid var(--border-default);
		border-radius: 0.75rem;
		padding: 1rem;
		overflow: auto;
	}
	.analytics-panels table {
		width: 100%;
		border-collapse: collapse;
	}
	.analytics-panels th,
	.analytics-panels td {
		padding: 0.6rem;
		text-align: left;
		border-bottom: 1px solid var(--border-default);
	}
	.email-dashboard {
		background: linear-gradient(to bottom right, #0f172a, #1e293b, #0f172a);
		padding: 1.5rem;
		min-height: 100%;
	}

	.dashboard-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.header-title :global(.email-header-icon) {
		color: #facc15;
	}

	.email-title {
		margin: 0;
		font-size: 1.875rem;
		line-height: 2.25rem;
		font-weight: 700;
		color: #ffffff;
	}

	.email-subtitle {
		margin: 0.25rem 0 0;
		color: #9ca3af;
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border: 0;
		border-radius: 0.5rem;
		background: #eab308;
		color: #111827;
		font: inherit;
		font-weight: 600;
		cursor: pointer;
		transition: background-color 0.2s ease;
	}

	.btn-primary:hover {
		background: #facc15;
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border: 0;
		border-radius: 0.5rem;
		background: #374151;
		color: #ffffff;
		font: inherit;
		font-weight: 600;
		cursor: pointer;
		transition: background-color 0.2s ease;
	}

	.btn-secondary:hover {
		background: #4b5563;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
		padding: 1.5rem;
		border: 1px solid rgba(55, 65, 81, 0.5);
		border-radius: 0.75rem;
		background: rgba(31, 41, 55, 0.5);
	}

	.stat-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		width: 3rem;
		height: 3rem;
		border-radius: 0.5rem;
	}

	.stat-icon--blue {
		background: rgba(59, 130, 246, 0.2);
		color: #60a5fa;
	}

	.stat-icon--green {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
	}

	.stat-icon--purple {
		background: rgba(168, 85, 247, 0.2);
		color: #c084fc;
	}

	.stat-icon--yellow {
		background: rgba(234, 179, 8, 0.2);
		color: #facc15;
	}

	.stat-content {
		flex: 1;
	}

	.stat-label {
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.stat-value {
		margin-top: 0.25rem;
		font-size: 1.5rem;
		line-height: 2rem;
		font-weight: 700;
		color: #ffffff;
	}

	.stat-change {
		margin-top: 0.25rem;
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.tabs {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 1.5rem;
		padding: 0.5rem;
		border: 1px solid rgba(55, 65, 81, 0.5);
		border-radius: 0.75rem;
		background: rgba(31, 41, 55, 0.5);
		overflow-x: auto;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		border: 0;
		border-radius: 0.5rem;
		background: transparent;
		color: #9ca3af;
		font: inherit;
		cursor: pointer;
		transition:
			background-color 0.2s ease,
			color 0.2s ease;
		white-space: nowrap;
	}

	.tab:hover {
		background: rgba(55, 65, 81, 0.5);
		color: #ffffff;
	}

	.tab.active {
		background: #eab308;
		color: #111827;
		font-weight: 600;
	}

	.tab-content {
		min-height: 500px;
		padding: 1.5rem;
		border: 1px solid rgba(55, 65, 81, 0.5);
		border-radius: 0.75rem;
		background: rgba(31, 41, 55, 0.5);
	}

	.list-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.section-heading {
		margin: 0;
		font-size: 1.25rem;
		line-height: 1.75rem;
		font-weight: 700;
		color: #ffffff;
	}

	.campaigns-grid,
	.sequences-grid,
	.templates-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1.5rem;
	}

	.campaign-card,
	.sequence-card,
	.template-card {
		padding: 1.5rem;
		border: 1px solid rgba(55, 65, 81, 0.5);
		border-radius: 0.5rem;
		background: rgba(17, 24, 39, 0.5);
		transition: border-color 0.2s ease;
	}

	.campaign-card:hover,
	.sequence-card:hover,
	.template-card:hover {
		border-color: rgba(234, 179, 8, 0.5);
	}

	.campaign-header,
	.sequence-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.campaign-status,
	.sequence-status {
		padding: 0.25rem 0.75rem;
		border-radius: 0.5rem;
		background: #374151;
		color: #d1d5db;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.campaign-status.active,
	.sequence-status.active {
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
	}

	.campaign-type {
		font-size: 0.875rem;
		color: #6b7280;
	}

	.campaign-name,
	.sequence-name,
	.template-name {
		margin: 0 0 0.5rem;
		font-size: 1.125rem;
		line-height: 1.75rem;
		font-weight: 600;
		color: #ffffff;
	}

	.campaign-subject,
	.sequence-description {
		margin: 0 0 1rem;
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.campaign-stats {
		display: grid;
		grid-template-columns: repeat(3, minmax(0, 1fr));
		gap: 1rem;
		margin-bottom: 1rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid rgba(55, 65, 81, 0.5);
	}

	.stat-item {
		display: flex;
		flex-direction: column;
	}

	.stat-item .stat-label {
		font-size: 0.75rem;
		color: #6b7280;
	}

	.stat-item .stat-value {
		margin-top: 0;
		font-size: 1.125rem;
		line-height: 1.75rem;
		font-weight: 600;
		color: #ffffff;
	}

	.sequence-info {
		display: flex;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.info-item {
		display: flex;
		flex-direction: column;
	}

	.info-label {
		font-size: 0.75rem;
		color: #6b7280;
	}

	.info-value {
		font-size: 0.875rem;
		font-weight: 600;
		color: #ffffff;
	}

	.campaign-actions,
	.sequence-actions,
	.template-actions {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.action-btn {
		padding: 0.5rem 1rem;
		border: 0;
		border-radius: 0.25rem;
		background: rgba(55, 65, 81, 0.5);
		color: #d1d5db;
		font: inherit;
		font-size: 0.875rem;
		cursor: pointer;
		transition: background-color 0.2s ease;
	}

	.action-btn:hover {
		background: #4b5563;
	}

	.template-thumbnail {
		width: 100%;
		height: 12rem;
		margin-bottom: 1rem;
		border-radius: 0.5rem;
		object-fit: cover;
	}

	.template-placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		height: 12rem;
		margin-bottom: 1rem;
		border-radius: 0.5rem;
		background: #111827;
	}

	.template-placeholder :global(.placeholder-icon) {
		color: #4b5563;
	}

	.template-info {
		margin-bottom: 1rem;
	}

	.template-category {
		margin: 0;
		font-size: 0.875rem;
		color: #6b7280;
	}

	.template-usage {
		margin-top: 0.25rem;
		font-size: 0.75rem;
		color: #4b5563;
	}

	.empty-state,
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding-block: 5rem;
		text-align: center;
	}

	.empty-state :global(.empty-icon) {
		color: #4b5563;
	}

	.empty-title {
		margin: 1rem 0 0;
		font-size: 1.25rem;
		line-height: 1.75rem;
		font-weight: 600;
		color: #9ca3af;
	}

	.empty-copy,
	.analytics-copy,
	.state-text {
		margin: 0;
		color: #9ca3af;
	}

	.empty-copy {
		margin-top: 0.5rem;
		color: #6b7280;
	}

	.empty-action {
		margin-top: 1rem;
	}

	.analytics-heading {
		margin-bottom: 1.5rem;
	}

	.state-text--spaced {
		margin-top: 1rem;
	}

	.spinner {
		width: 3rem;
		height: 3rem;
		border: 4px solid #374151;
		border-top-color: #facc15;
		border-radius: 999px;
		animation: email-spin 1s linear infinite;
	}

	@media (min-width: 768px) {
		.stats-grid,
		.campaigns-grid,
		.sequences-grid,
		.templates-grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}

	@media (min-width: 1024px) {
		.stats-grid {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}

		.campaigns-grid,
		.sequences-grid,
		.templates-grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}

	@media (max-width: 767px) {
		.dashboard-header,
		.list-header {
			align-items: flex-start;
			flex-direction: column;
		}

		.header-actions {
			width: 100%;
			flex-wrap: wrap;
		}
	}

	@keyframes email-spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
