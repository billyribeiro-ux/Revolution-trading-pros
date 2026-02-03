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
		'campaigns';

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
			<IconMail size={32} class="text-yellow-400" />
			<div>
				<h1 class="text-3xl font-bold text-white">Email Marketing</h1>
				<p class="text-gray-400 mt-1">Campaigns, sequences, and automation</p>
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
				<div class="stat-icon bg-blue-500/20">
					<IconSend size={24} class="text-blue-400" />
				</div>
				<div class="stat-content">
					<div class="stat-label">Emails Sent</div>
					<div class="stat-value">{formatNumber(emailStore?.analytics?.emails_sent || 0)}</div>
					<div class="stat-change positive">+12.5% vs last month</div>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon bg-green-500/20">
					<IconChartBar size={24} class="text-green-400" />
				</div>
				<div class="stat-content">
					<div class="stat-label">Avg Open Rate</div>
					<div class="stat-value">{formatPercent(emailStore.analytics.average_open_rate)}</div>
					<div class="stat-change positive">+2.3% vs last month</div>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon bg-purple-500/20">
					<IconUsers size={24} class="text-purple-400" />
				</div>
				<div class="stat-content">
					<div class="stat-label">Avg Click Rate</div>
					<div class="stat-value">{formatPercent(emailStore.analytics.average_click_rate)}</div>
					<div class="stat-change positive">+1.8% vs last month</div>
				</div>
			</div>

			<div class="stat-card">
				<div class="stat-icon bg-yellow-500/20">
					<IconRocket size={24} class="text-yellow-400" />
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
		{#if getIsEmailLoading}
			<div class="loading-state">
				<div class="spinner"></div>
				<p class="text-gray-400 mt-4">Loading...</p>
			</div>
		{:else if selectedTab === 'campaigns'}
			<div class="campaigns-list">
				<div class="list-header">
					<h2 class="text-xl font-bold text-white">Email Campaigns</h2>
					<button class="btn-primary">
						<IconPlus size={18} />
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
						<IconMail size={64} class="text-gray-600" />
						<h3 class="text-xl font-semibold text-gray-400 mt-4">No campaigns yet</h3>
						<p class="text-gray-500 mt-2">Create your first email campaign to get started</p>
						<button class="btn-primary mt-4">
							<IconPlus size={18} />
							Create Campaign
						</button>
					</div>
				{/if}
			</div>
		{:else if selectedTab === 'sequences'}
			<div class="sequences-list">
				<div class="list-header">
					<h2 class="text-xl font-bold text-white">Email Sequences</h2>
					<button class="btn-primary">
						<IconPlus size={18} />
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
						<IconBolt size={64} class="text-gray-600" />
						<h3 class="text-xl font-semibold text-gray-400 mt-4">No sequences yet</h3>
						<p class="text-gray-500 mt-2">Create automated email sequences</p>
						<button class="btn-primary mt-4">
							<IconPlus size={18} />
							Create Sequence
						</button>
					</div>
				{/if}
			</div>
		{:else if selectedTab === 'templates'}
			<div class="templates-list">
				<div class="list-header">
					<h2 class="text-xl font-bold text-white">Email Templates</h2>
					<button class="btn-primary">
						<IconPlus size={18} />
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
										<IconFileText size={48} class="text-gray-600" />
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
						<IconFileText size={64} class="text-gray-600" />
						<h3 class="text-xl font-semibold text-gray-400 mt-4">No templates yet</h3>
						<p class="text-gray-500 mt-2">Create reusable email templates</p>
						<button class="btn-primary mt-4">
							<IconPlus size={18} />
							Create Template
						</button>
					</div>
				{/if}
			</div>
		{:else}
			<div class="analytics-view">
				<h2 class="text-xl font-bold text-white mb-6">Email Analytics</h2>
				<p class="text-gray-400">Detailed analytics coming soon...</p>
			</div>
		{/if}
	</div>
</div>

<style lang="postcss">
	@reference "../../app.css";
	.email-dashboard {
		background: linear-gradient(to bottom right, #0f172a, #1e293b, #0f172a);
		padding: 1.5rem;
	}

	.dashboard-header {
		@apply flex items-center justify-between mb-8;
	}

	.header-title {
		@apply flex items-center gap-4;
	}

	.header-actions {
		@apply flex items-center gap-3;
	}

	.btn-primary {
		@apply flex items-center gap-2 px-6 py-3 bg-yellow-500 text-gray-900 font-semibold rounded-lg;
		@apply hover:bg-yellow-400 transition-colors;
	}

	.btn-secondary {
		@apply flex items-center gap-2 px-6 py-3 bg-gray-700 text-white font-semibold rounded-lg;
		@apply hover:bg-gray-600 transition-colors;
	}

	.stats-grid {
		@apply grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8;
	}

	.stat-card {
		@apply bg-gray-800/50 rounded-xl p-6 border border-gray-700/50 flex items-start gap-4;
	}

	.stat-icon {
		@apply w-12 h-12 rounded-lg flex items-center justify-center flex-shrink-0;
	}

	.stat-content {
		@apply flex-1;
	}

	.stat-label {
		@apply text-sm text-gray-400;
	}

	.stat-value {
		@apply text-2xl font-bold text-white mt-1;
	}

	.stat-change {
		@apply text-sm mt-1;
	}

	.stat-change.positive {
		@apply text-green-400;
	}

	.tabs {
		@apply flex gap-2 mb-6 bg-gray-800/50 p-2 rounded-xl border border-gray-700/50;
	}

	.tab {
		@apply flex items-center gap-2 px-4 py-2 text-gray-400 rounded-lg transition-colors;
		@apply hover:text-white hover:bg-gray-700/50;
	}

	.tab.active {
		@apply bg-yellow-500 text-gray-900 font-semibold;
	}

	.tab-content {
		@apply bg-gray-800/50 rounded-xl p-6 border border-gray-700/50 min-h-[500px];
	}

	.list-header {
		@apply flex items-center justify-between mb-6;
	}

	.campaigns-grid,
	.sequences-grid,
	.templates-grid {
		@apply grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6;
	}

	.campaign-card,
	.sequence-card,
	.template-card {
		@apply bg-gray-900/50 rounded-lg p-6 border border-gray-700/50;
		@apply hover:border-yellow-500/50 transition-colors;
	}

	.campaign-header,
	.sequence-header {
		@apply flex items-center justify-between mb-4;
	}

	.campaign-status,
	.sequence-status {
		@apply px-3 py-1 bg-gray-700 text-gray-300 rounded-lg text-sm font-medium;
	}

	.campaign-status.active,
	.sequence-status.active {
		@apply bg-green-500/20 text-green-400;
	}

	.campaign-type {
		@apply text-sm text-gray-500;
	}

	.campaign-name,
	.sequence-name,
	.template-name {
		@apply text-lg font-semibold text-white mb-2;
	}

	.campaign-subject,
	.sequence-description {
		@apply text-sm text-gray-400 mb-4;
	}

	.campaign-stats {
		@apply grid grid-cols-3 gap-4 mb-4 pb-4 border-b border-gray-700/50;
	}

	.stat-item {
		@apply flex flex-col;
	}

	.stat-item .stat-label {
		@apply text-xs text-gray-500;
	}

	.stat-item .stat-value {
		@apply text-lg font-semibold text-white;
	}

	.sequence-info {
		@apply flex gap-4 mb-4;
	}

	.info-item {
		@apply flex flex-col;
	}

	.info-label {
		@apply text-xs text-gray-500;
	}

	.info-value {
		@apply text-sm font-semibold text-white;
	}

	.campaign-actions,
	.sequence-actions,
	.template-actions {
		@apply flex gap-2;
	}

	.action-btn {
		@apply px-4 py-2 text-sm text-gray-300 bg-gray-700/50 rounded;
		@apply hover:bg-gray-600 transition-colors;
	}

	.template-thumbnail {
		@apply w-full h-48 object-cover rounded-lg mb-4;
	}

	.template-placeholder {
		@apply w-full h-48 bg-gray-900 rounded-lg mb-4 flex items-center justify-center;
	}

	.template-info {
		@apply mb-4;
	}

	.template-category {
		@apply text-sm text-gray-500;
	}

	.template-usage {
		@apply text-xs text-gray-600 mt-1;
	}

	.empty-state,
	.loading-state {
		@apply flex flex-col items-center justify-center py-20 text-center;
	}

	.spinner {
		@apply w-12 h-12 border-4 border-gray-700 border-t-yellow-400 rounded-full animate-spin;
	}
</style>
