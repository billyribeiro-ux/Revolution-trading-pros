<!--
	Email Marketing Dashboard
	═══════════════════════════════════════════════════════════════════════════
	
	Complete email marketing and automation dashboard.
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { emailStore, getIsEmailLoading } from '$lib/stores/email.svelte';
	import {
		Icon,
		IconBolt,
		IconChartBar,
		IconFileText,
		IconMail,
		IconPlus,
		IconRocket,
		IconSend,
		IconUsers
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
			<Icon icon={IconMail} size={32} />
			<div>
				<h1 class="page-heading">Email Marketing</h1>
				<p class="page-subheading">Campaigns, sequences, and automation</p>
			</div>
		</div>

		<div class="header-actions">
			<button class="btn-secondary" onclick={() => (selectedTab = 'templates')}>
				<Icon icon={IconFileText} size={18} />
				Templates
			</button>
			<button class="btn-primary" onclick={() => (selectedTab = 'campaigns')}>
				<Icon icon={IconPlus} size={18} />
				New Campaign
			</button>
		</div>
	</div>

	<!-- Stats Overview -->
	{#if emailStore.analytics}
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon" data-color="blue">
					<Icon icon={IconSend} size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-label">Emails Sent</div>
					<div class="stat-value">{formatNumber(emailStore?.analytics?.emails_sent || 0)}</div>
					<div class="stat-change positive">+12.5% vs last month</div>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon" data-color="green">
					<Icon icon={IconChartBar} size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-label">Avg Open Rate</div>
					<div class="stat-value">{formatPercent(emailStore.analytics.average_open_rate)}</div>
					<div class="stat-change positive">+2.3% vs last month</div>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon" data-color="purple">
					<Icon icon={IconUsers} size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-label">Avg Click Rate</div>
					<div class="stat-value">{formatPercent(emailStore.analytics.average_click_rate)}</div>
					<div class="stat-change positive">+1.8% vs last month</div>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon" data-color="yellow">
					<Icon icon={IconRocket} size={24} />
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
			<Icon icon={IconMail} size={20} />
			Campaigns
		</button>
		<button
			class="tab"
			class:active={selectedTab === 'sequences'}
			onclick={() => (selectedTab = 'sequences')}
		>
			<Icon icon={IconBolt} size={20} />
			Sequences
		</button>
		<button
			class="tab"
			class:active={selectedTab === 'automations'}
			onclick={() => (selectedTab = 'automations')}
		>
			<Icon icon={IconRocket} size={20} />
			Automations
		</button>
		<button
			class="tab"
			class:active={selectedTab === 'templates'}
			onclick={() => (selectedTab = 'templates')}
		>
			<Icon icon={IconFileText} size={20} />
			Templates
		</button>
		<button
			class="tab"
			class:active={selectedTab === 'analytics'}
			onclick={() => (selectedTab = 'analytics')}
		>
			<Icon icon={IconChartBar} size={20} />
			Analytics
		</button>
	</div>

	<!-- Content -->
	<div class="tab-content">
		{#if getIsEmailLoading()}
			<div class="loading-state">
				<div class="spinner"></div>
				<p class="loading-text">Loading...</p>
			</div>
		{:else if selectedTab === 'campaigns'}
			<div class="campaigns-list">
				<div class="list-header">
					<h2 class="list-title">Email Campaigns</h2>
					<button class="btn-primary">
						<Icon icon={IconPlus} size={18} />
						Create Campaign
					</button>
				</div>

				{#if emailStore.campaigns.length > 0}
					<div class="campaigns-grid">
						{#each emailStore.campaigns as campaign}
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
						<Icon icon={IconMail} size={64} />
						<h3 class="empty-title">No campaigns yet</h3>
						<p class="empty-subtitle">Create your first email campaign to get started</p>
						<button class="btn-primary empty-action">
							<Icon icon={IconPlus} size={18} />
							Create Campaign
						</button>
					</div>
				{/if}
			</div>
		{:else if selectedTab === 'sequences'}
			<div class="sequences-list">
				<div class="list-header">
					<h2 class="list-title">Email Sequences</h2>
					<button class="btn-primary">
						<Icon icon={IconPlus} size={18} />
						Create Sequence
					</button>
				</div>

				{#if emailStore.sequences.length > 0}
					<div class="sequences-grid">
						{#each emailStore.sequences as sequence}
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
						<Icon icon={IconBolt} size={64} />
						<h3 class="empty-title">No sequences yet</h3>
						<p class="empty-subtitle">Create automated email sequences</p>
						<button class="btn-primary empty-action">
							<Icon icon={IconPlus} size={18} />
							Create Sequence
						</button>
					</div>
				{/if}
			</div>
		{:else if selectedTab === 'templates'}
			<div class="templates-list">
				<div class="list-header">
					<h2 class="list-title">Email Templates</h2>
					<button class="btn-primary">
						<Icon icon={IconPlus} size={18} />
						Create Template
					</button>
				</div>

				{#if emailStore.templates.length > 0}
					<div class="templates-grid">
						{#each emailStore.templates as template}
							<div class="template-card">
								{#if template.thumbnail}
									<img src={template.thumbnail} alt={template.name} class="template-thumbnail" />
								{:else}
									<div class="template-placeholder">
										<Icon icon={IconFileText} size={48} />
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
						<Icon icon={IconFileText} size={64} />
						<h3 class="empty-title">No templates yet</h3>
						<p class="empty-subtitle">Create reusable email templates</p>
						<button class="btn-primary empty-action">
							<Icon icon={IconPlus} size={18} />
							Create Template
						</button>
					</div>
				{/if}
			</div>
		{:else}
			<div class="analytics-view">
				<h2 class="analytics-title">Email Analytics</h2>
				<p class="analytics-placeholder">Detailed analytics coming soon...</p>
			</div>
		{/if}
	</div>
</div>

<style>
	.email-dashboard {
		background: linear-gradient(
			to bottom right,
			oklch(0.13 0.02 260),
			oklch(0.2 0.02 250),
			oklch(0.13 0.02 260)
		);
		padding: var(--space-6);
	}

	.dashboard-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: var(--space-8);
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: var(--space-4);
		color: oklch(0.8 0.18 90);
	}

	.page-heading {
		font-size: var(--text-3xl);
		font-weight: var(--weight-bold);
		color: oklch(1 0 0);
	}

	.page-subheading {
		color: oklch(0.65 0.01 250);
		margin-block-start: var(--space-1);
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding-inline: var(--space-6);
		padding-block: var(--space-3);
		background-color: oklch(0.8 0.18 90);
		color: oklch(0.15 0.02 90);
		font-weight: var(--weight-semibold);
		border-radius: var(--radius-lg);
		border: none;
		cursor: pointer;
		transition: background-color var(--duration-fast) var(--ease-default);
		&:hover {
			background-color: oklch(0.85 0.16 90);
		}
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding-inline: var(--space-6);
		padding-block: var(--space-3);
		background-color: oklch(0.38 0.01 250);
		color: oklch(1 0 0);
		font-weight: var(--weight-semibold);
		border-radius: var(--radius-lg);
		border: none;
		cursor: pointer;
		transition: background-color var(--duration-fast) var(--ease-default);
		&:hover {
			background-color: oklch(0.45 0.01 250);
		}
	}

	.stats-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: var(--space-6);
		margin-block-end: var(--space-8);
		@media (min-width: 768px) {
			grid-template-columns: repeat(2, 1fr);
		}
		@media (min-width: 1024px) {
			grid-template-columns: repeat(4, 1fr);
		}
	}

	.stat-card {
		background-color: oklch(0.25 0.01 250 / 50%);
		border-radius: var(--radius-xl);
		padding: var(--space-6);
		border: 1px solid oklch(0.38 0.01 250 / 50%);
		display: flex;
		align-items: flex-start;
		gap: var(--space-4);
	}

	.stat-icon {
		inline-size: 3rem;
		block-size: 3rem;
		border-radius: var(--radius-lg);
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;

		&[data-color='blue'] {
			background-color: oklch(0.6 0.2 260 / 20%);
			color: oklch(0.7 0.18 260);
		}
		&[data-color='green'] {
			background-color: oklch(0.6 0.18 160 / 20%);
			color: oklch(0.7 0.18 160);
		}
		&[data-color='purple'] {
			background-color: oklch(0.55 0.2 300 / 20%);
			color: oklch(0.7 0.18 300);
		}
		&[data-color='yellow'] {
			background-color: oklch(0.8 0.18 90 / 20%);
			color: oklch(0.8 0.18 90);
		}
	}

	.stat-content {
		flex: 1;
	}

	.stat-label {
		font-size: var(--text-sm);
		color: oklch(0.65 0.01 250);
	}

	.stat-value {
		font-size: var(--text-2xl);
		font-weight: var(--weight-bold);
		color: oklch(1 0 0);
		margin-block-start: var(--space-1);
	}

	.stat-change {
		font-size: var(--text-sm);
		color: oklch(0.65 0.01 250);
		margin-block-start: var(--space-1);
		&.positive {
			color: oklch(0.7 0.18 160);
		}
	}

	.tabs {
		display: flex;
		gap: var(--space-2);
		margin-block-end: var(--space-6);
		background-color: oklch(0.25 0.01 250 / 50%);
		padding: var(--space-2);
		border-radius: var(--radius-xl);
		border: 1px solid oklch(0.38 0.01 250 / 50%);
	}

	.tab {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding-inline: var(--space-4);
		padding-block: var(--space-2);
		color: oklch(0.65 0.01 250);
		border-radius: var(--radius-lg);
		border: none;
		cursor: pointer;
		transition: all var(--duration-fast) var(--ease-default);

		&:hover {
			color: oklch(1 0 0);
			background-color: oklch(0.38 0.01 250 / 50%);
		}
		&.active {
			background-color: oklch(0.8 0.18 90);
			color: oklch(0.15 0.02 90);
			font-weight: var(--weight-semibold);
		}
	}

	.tab-content {
		background-color: oklch(0.25 0.01 250 / 50%);
		border-radius: var(--radius-xl);
		padding: var(--space-6);
		border: 1px solid oklch(0.38 0.01 250 / 50%);
		min-block-size: 500px;
	}

	.list-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: var(--space-6);
	}

	.list-title {
		font-size: var(--text-xl);
		font-weight: var(--weight-bold);
		color: oklch(1 0 0);
	}

	.campaigns-grid,
	.sequences-grid,
	.templates-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: var(--space-6);
		@media (min-width: 768px) {
			grid-template-columns: repeat(2, 1fr);
		}
		@media (min-width: 1024px) {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.campaign-card,
	.sequence-card,
	.template-card {
		background-color: oklch(0.15 0.01 250 / 50%);
		border-radius: var(--radius-lg);
		padding: var(--space-6);
		border: 1px solid oklch(0.38 0.01 250 / 50%);
		transition: border-color var(--duration-fast) var(--ease-default);
		&:hover {
			border-color: oklch(0.8 0.18 90 / 50%);
		}
	}

	.campaign-header,
	.sequence-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: var(--space-4);
	}

	.campaign-status,
	.sequence-status {
		padding-inline: var(--space-3);
		padding-block: var(--space-1);
		background-color: oklch(0.38 0.01 250);
		color: oklch(0.75 0.01 250);
		border-radius: var(--radius-lg);
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);

		&.active {
			background-color: oklch(0.6 0.18 160 / 20%);
			color: oklch(0.7 0.18 160);
		}
	}

	.campaign-type {
		font-size: var(--text-sm);
		color: oklch(0.55 0.01 250);
	}

	.campaign-name,
	.sequence-name,
	.template-name {
		font-size: var(--text-lg);
		font-weight: var(--weight-semibold);
		color: oklch(1 0 0);
		margin-block-end: var(--space-2);
	}

	.campaign-subject,
	.sequence-description {
		font-size: var(--text-sm);
		color: oklch(0.65 0.01 250);
		margin-block-end: var(--space-4);
	}

	.campaign-stats {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: var(--space-4);
		margin-block-end: var(--space-4);
		padding-block-end: var(--space-4);
		border-block-end: 1px solid oklch(0.38 0.01 250 / 50%);
	}

	.stat-item {
		display: flex;
		flex-direction: column;
	}

	.stat-item .stat-label {
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 250);
	}

	.stat-item .stat-value {
		font-size: var(--text-lg);
		font-weight: var(--weight-semibold);
		color: oklch(1 0 0);
		margin-block-start: 0;
	}

	.sequence-info {
		display: flex;
		gap: var(--space-4);
		margin-block-end: var(--space-4);
	}

	.info-item {
		display: flex;
		flex-direction: column;
	}
	.info-label {
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 250);
	}
	.info-value {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: oklch(1 0 0);
	}

	.campaign-actions,
	.sequence-actions,
	.template-actions {
		display: flex;
		gap: var(--space-2);
	}

	.action-btn {
		padding-inline: var(--space-4);
		padding-block: var(--space-2);
		font-size: var(--text-sm);
		color: oklch(0.75 0.01 250);
		background-color: oklch(0.38 0.01 250 / 50%);
		border-radius: var(--radius-sm);
		border: none;
		cursor: pointer;
		transition: background-color var(--duration-fast) var(--ease-default);
		&:hover {
			background-color: oklch(0.45 0.01 250);
		}
	}

	.template-thumbnail {
		inline-size: 100%;
		block-size: 12rem;
		object-fit: cover;
		border-radius: var(--radius-lg);
		margin-block-end: var(--space-4);
	}

	.template-placeholder {
		inline-size: 100%;
		block-size: 12rem;
		background-color: oklch(0.15 0.01 250);
		border-radius: var(--radius-lg);
		margin-block-end: var(--space-4);
		display: flex;
		align-items: center;
		justify-content: center;
		color: oklch(0.45 0.01 250);
	}

	.template-info {
		margin-block-end: var(--space-4);
	}
	.template-category {
		font-size: var(--text-sm);
		color: oklch(0.55 0.01 250);
	}
	.template-usage {
		font-size: var(--text-xs);
		color: oklch(0.45 0.01 250);
		margin-block-start: var(--space-1);
	}

	.empty-state,
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding-block: 5rem;
		text-align: center;
		color: oklch(0.45 0.01 250);
	}

	.loading-text {
		color: oklch(0.65 0.01 250);
		margin-block-start: var(--space-4);
	}

	.empty-title {
		font-size: var(--text-xl);
		font-weight: var(--weight-semibold);
		color: oklch(0.65 0.01 250);
		margin-block-start: var(--space-4);
	}

	.empty-subtitle {
		color: oklch(0.55 0.01 250);
		margin-block-start: var(--space-2);
	}

	.empty-action {
		margin-block-start: var(--space-4);
	}

	.analytics-title {
		font-size: var(--text-xl);
		font-weight: var(--weight-bold);
		color: oklch(1 0 0);
		margin-block-end: var(--space-6);
	}

	.analytics-placeholder {
		color: oklch(0.65 0.01 250);
	}

	.spinner {
		inline-size: 3rem;
		block-size: 3rem;
		border: 4px solid oklch(0.38 0.01 250);
		border-block-start-color: oklch(0.8 0.18 90);
		border-radius: 9999px;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
