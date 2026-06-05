<!--
	Email Marketing Dashboard
	═══════════════════════════════════════════════════════════════════════════
	
	Complete email marketing and automation dashboard.
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { emailStore, getIsEmailLoading } from '$lib/stores/email.svelte';
	import {
		IconMail,
		IconSend,
		IconUsers,
		IconChartBar,
		IconPlus,
		IconFileText,
		IconRocket,
		IconBolt
	} from '$lib/icons';

	let selectedTab: 'campaigns' | 'sequences' | 'automations' | 'templates' | 'analytics' =
		$state('campaigns');

	onMount(() => {
		loadData();
	});

	async function loadData() {
		await Promise.all([
			emailStore.loadCampaigns(),
			emailStore.loadSequences(),
			emailStore.loadAutomations(),
			emailStore.loadTemplates(),
			emailStore.loadAnalytics()
		]);
	}

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	function formatPercent(num: number): string {
		return num.toFixed(1) + '%';
	}
</script>

<svelte:head>
	<title>Email Marketing | Revolution Trading Pros</title>
</svelte:head>

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
			<button class="btn-primary" onclick={() => (selectedTab = 'campaigns')}>
				<IconPlus size={18} />
				New Campaign
			</button>
		</div>
	</div>

	<!-- Stats Overview -->
	{#if emailStore.analytics}
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon stat-icon--blue">
					<IconSend size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-label">Emails Sent</div>
					<div class="stat-value">{formatNumber(emailStore?.analytics?.emails_sent || 0)}</div>
					<div class="stat-change positive">+12.5% vs last month</div>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon stat-icon--green">
					<IconChartBar size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-label">Avg Open Rate</div>
					<div class="stat-value">{formatPercent(emailStore.analytics.average_open_rate)}</div>
					<div class="stat-change positive">+2.3% vs last month</div>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon stat-icon--purple">
					<IconUsers size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-label">Avg Click Rate</div>
					<div class="stat-value">{formatPercent(emailStore.analytics.average_click_rate)}</div>
					<div class="stat-change positive">+1.8% vs last month</div>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon stat-icon--yellow">
					<IconRocket size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-label">Active Campaigns</div>
					<div class="stat-value">{emailStore.analytics.campaigns}</div>
					<div class="stat-change">Running now</div>
				</div>
			</div>
		</div>
	{/if}

	<!-- Tabs -->
	<div class="tabs">
		<button
			class="tab"
			class:active={selectedTab === 'campaigns'}
			onclick={() => (selectedTab = 'campaigns')}
		>
			<IconMail size={20} />
			Campaigns
		</button>
		<button
			class="tab"
			class:active={selectedTab === 'sequences'}
			onclick={() => (selectedTab = 'sequences')}
		>
			<IconBolt size={20} />
			Sequences
		</button>
		<button
			class="tab"
			class:active={selectedTab === 'automations'}
			onclick={() => (selectedTab = 'automations')}
		>
			<IconRocket size={20} />
			Automations
		</button>
		<button
			class="tab"
			class:active={selectedTab === 'templates'}
			onclick={() => (selectedTab = 'templates')}
		>
			<IconFileText size={20} />
			Templates
		</button>
		<button
			class="tab"
			class:active={selectedTab === 'analytics'}
			onclick={() => (selectedTab = 'analytics')}
		>
			<IconChartBar size={20} />
			Analytics
		</button>
	</div>

	<!-- Content -->
	<div class="tab-content">
		{#if getIsEmailLoading()}
			<div class="loading-state">
				<div class="spinner"></div>
				<p class="state-text state-text--spaced">Loading...</p>
			</div>
		{:else if selectedTab === 'campaigns'}
			<div class="campaigns-list">
				<div class="list-header">
					<h2 class="section-heading">Email Campaigns</h2>
					<button class="btn-primary">
						<IconPlus size={18} />
						Create Campaign
					</button>
				</div>

				{#if emailStore.campaigns.length > 0}
					<div class="campaigns-grid">
						{#each emailStore.campaigns as campaign (campaign.id)}
							<div class="campaign-card">
								<div class="campaign-header">
									<div class="campaign-status" class:active={campaign.status === 'sending'}>
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
									<button class="action-btn">View</button>
									<button class="action-btn">Edit</button>
									<button class="action-btn">Duplicate</button>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<div class="empty-state">
						<IconMail size={64} class="empty-icon" />
						<h3 class="empty-title">No campaigns yet</h3>
						<p class="empty-copy">Create your first email campaign to get started</p>
						<button class="btn-primary empty-action">
							<IconPlus size={18} />
							Create Campaign
						</button>
					</div>
				{/if}
			</div>
		{:else if selectedTab === 'sequences'}
			<div class="sequences-list">
				<div class="list-header">
					<h2 class="section-heading">Email Sequences</h2>
					<button class="btn-primary">
						<IconPlus size={18} />
						Create Sequence
					</button>
				</div>

				{#if emailStore.sequences.length > 0}
					<div class="sequences-grid">
						{#each emailStore.sequences as sequence (sequence.id)}
							<div class="sequence-card">
								<div class="sequence-header">
									<h3 class="sequence-name">{sequence.name}</h3>
									<div class="sequence-status" class:active={sequence.status === 'active'}>
										{sequence.status}
									</div>
								</div>

								<p class="sequence-description">{sequence.description || 'No description'}</p>

								<div class="sequence-info">
									<div class="info-item">
										<span class="info-label">Emails</span>
										<span class="info-value">{sequence.emails?.length || 0}</span>
									</div>
									<div class="info-item">
										<span class="info-label">Subscribers</span>
										<span class="info-value">{formatNumber(sequence.subscribers_count)}</span>
									</div>
								</div>

								<div class="sequence-actions">
									<button class="action-btn">View</button>
									<button class="action-btn">Edit</button>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<div class="empty-state">
						<IconBolt size={64} class="empty-icon" />
						<h3 class="empty-title">No sequences yet</h3>
						<p class="empty-copy">Create automated email sequences</p>
						<button class="btn-primary empty-action">
							<IconPlus size={18} />
							Create Sequence
						</button>
					</div>
				{/if}
			</div>
		{:else if selectedTab === 'templates'}
			<div class="templates-list">
				<div class="list-header">
					<h2 class="section-heading">Email Templates</h2>
					<button class="btn-primary">
						<IconPlus size={18} />
						Create Template
					</button>
				</div>

				{#if emailStore.templates.length > 0}
					<div class="templates-grid">
						{#each emailStore.templates as template (template.id)}
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
									<div class="template-usage">Used {template.usage_count} times</div>
								</div>

								<div class="template-actions">
									<button class="action-btn">Use Template</button>
									<button class="action-btn">Preview</button>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<div class="empty-state">
						<IconFileText size={64} class="empty-icon" />
						<h3 class="empty-title">No templates yet</h3>
						<p class="empty-copy">Create reusable email templates</p>
						<button class="btn-primary empty-action">
							<IconPlus size={18} />
							Create Template
						</button>
					</div>
				{/if}
			</div>
		{:else}
			<div class="analytics-view">
				<h2 class="section-heading analytics-heading">Email Analytics</h2>
				<p class="analytics-copy">Detailed analytics coming soon...</p>
			</div>
		{/if}
	</div>
</div>

<style>
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

	.stat-change.positive {
		color: #4ade80;
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
