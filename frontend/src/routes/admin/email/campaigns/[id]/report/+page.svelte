<script lang="ts">
	/**
	 * Email Campaign Report - Svelte 5 Runes Implementation
	 */

	import type { PageData } from './$types';
	import { fade, fly } from 'svelte/transition';
	import { afterNavigate } from '$app/navigation';
	import { adminFetch } from '#lib/utils/adminFetch.js';
	import {
		campaignReport,
		type CampaignReport,
		formatCampaignRate
	} from '#lib/api/campaign-report.js';
	import ExportButton from '#lib/components/ExportButton.svelte';
	import { IconArrowLeft, IconRefresh, IconAlertTriangle, IconChartBar } from '#lib/icons/index.js';

	import CampaignOverview from './_components/CampaignOverview.svelte';
	import MetricsGrid from './_components/MetricsGrid.svelte';
	import EngagementChart from './_components/EngagementChart.svelte';
	import EngagementQuality from './_components/EngagementQuality.svelte';
	import TopLinksTable from './_components/TopLinksTable.svelte';
	import DeviceBreakdown from './_components/DeviceBreakdown.svelte';
	import GeoDistribution from './_components/GeoDistribution.svelte';
	import AbTestResults from './_components/AbTestResults.svelte';

	// Props (Svelte 5 - no destructuring)

	interface Props {
		data: PageData;
	}
	let props: Props = $props();
	let data = $derived(props.data);

	// Types

	// State - Svelte 5 Runes

	let loading = $state(true);
	let error = $state<string | null>(null);
	let report = $state<CampaignReport | null>(null);
	let loadVersion = 0;

	// Derived State

	let campaignId = $derived(data.campaignId);

	let exportData = $derived.by(() => {
		if (!report) return [];

		const data = [
			{
				category: 'Overview',
				metric: 'Campaign Name',
				value: report.name
			},

			{
				category: 'Overview',
				metric: 'Subject',
				value: report.subject
			},

			{
				category: 'Overview',
				metric: 'Sent Date',
				value: report.sent_at || 'Not sent'
			},

			{
				category: 'Overview',
				metric: 'Segment',
				value: report.segment_name || 'All'
			},

			{
				category: 'Metrics',
				metric: 'Emails Sent',
				value: report.metrics.sent
			},

			{
				category: 'Metrics',
				metric: 'Delivered',
				value: report.metrics.delivered
			},

			{
				category: 'Metrics',
				metric: 'Opened',
				value: report.metrics.opened
			},

			{
				category: 'Metrics',
				metric: 'Unique Opens',
				value: report.metrics.unique_opens
			},

			{
				category: 'Metrics',
				metric: 'Clicked',
				value: report.metrics.clicked
			},

			{
				category: 'Metrics',
				metric: 'Unique Clicks',
				value: report.metrics.unique_clicks
			},

			{
				category: 'Metrics',
				metric: 'Bounced',
				value: report.metrics.bounced
			},

			{
				category: 'Metrics',
				metric: 'Unsubscribed',
				value: report.metrics.unsubscribed
			},

			{
				category: 'Rates',
				metric: 'Delivery Rate',
				value: formatCampaignRate(report.rates.delivery_rate)
			},

			{
				category: 'Rates',
				metric: 'Open Rate',
				value: formatCampaignRate(report.rates.open_rate)
			},

			{
				category: 'Rates',
				metric: 'Click Rate',
				value: formatCampaignRate(report.rates.click_rate)
			},

			{
				category: 'Rates',
				metric: 'Click-to-Open Rate',
				value: formatCampaignRate(report.rates.click_to_open_rate)
			},
			{
				category: 'Rates',
				metric: 'Bounce Rate',
				value: formatCampaignRate(report.rates.bounce_rate)
			},
			{
				category: 'Rates',
				metric: 'Unsubscribe Rate',
				value: formatCampaignRate(report.rates.unsubscribe_rate)
			},
			...report.top_links.map((link, i) => ({
				category: 'Top Links',
				metric: `Link ${i + 1}`,
				value: `${link.url} (${link.clicks} clicks)`
			})),
			...report.geographic_distribution.slice(0, 10).map((geo) => ({
				category: 'Geography',
				metric: geo.country,
				value: `${geo.opens} opens, ${geo.clicks} clicks`
			}))
		];

		return data;
	});

	let maxEngagement = $derived.by(() => {
		if (!report?.engagement_over_time?.length) return 1;
		const opens = Math.max(...report.engagement_over_time.map((e) => e.opens));
		const clicks = Math.max(...report.engagement_over_time.map((e) => e.clicks));
		return Math.max(opens, clicks, 1);
	});

	let totalDevices = $derived.by(() => {
		if (!report?.device_breakdown) return 0;
		const d = report.device_breakdown;
		return d.desktop + d.mobile + d.tablet + d.unknown;
	});

	// Lifecycle
	afterNavigate(({ shallow }) => {
		if (shallow) return;

		if (campaignId) {
			void loadReport();
		}
	});

	// API Functions

	async function loadReport() {
		const version = ++loadVersion;
		const requested = campaignId;
		loading = true;
		error = null;
		report = null;
		try {
			const response = await adminFetch<unknown>(`/api/admin/email/campaigns/${requested}/report`);
			const actual = campaignReport(response, Number(requested));
			if (version === loadVersion && requested === campaignId) report = actual;
		} catch (err) {
			if (version === loadVersion)
				error = err instanceof Error ? err.message : 'Unable to load recorded campaign results';
		} finally {
			if (version === loadVersion) loading = false;
		}
	}

	// Helper Functions

	function formatNumber(num: number): string {
		if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
		if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
		return num.toLocaleString();
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

	function formatHour(hour: number): string {
		if (hour === 0) return '0h';
		if (hour < 24) return `${hour}h`;
		return `${Math.floor(hour / 24)}d`;
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'sent':
				return 'success';
			case 'scheduled':
				return 'info';
			case 'sending':
				return 'warning';
			case 'draft':
				return 'muted';
			case 'failed':
				return 'error';
			default:
				return 'muted';
		}
	}

	function truncateUrl(url: string, maxLength: number = 50): string {
		if (url.length <= maxLength) return url;
		return url.substring(0, maxLength) + '...';
	}

	// Country code to flag emoji converter
	function getFlagEmoji(countryCode: string): string {
		if (!countryCode || countryCode.length !== 2) return '';
		const codePoints = countryCode
			.toUpperCase()
			.split('')
			.map((char) => 127397 + char.charCodeAt(0));
		return String.fromCodePoint(...codePoints);
	}

	function refreshIconClass() {
		return {
			spinning: loading
		};
	}
</script>

<!--
	Email Campaign Report - Apple ICT7 Principal Engineer Grade
	═══════════════════════════════════════════════════════════════════════════════

	Comprehensive email campaign analytics with:
	- Campaign overview (name, subject, sent date)
	- Key metrics (sent, delivered, opened, clicked, bounced, unsubscribed)
	- Open/click rate charts
	- Top links clicked
	- Device/client breakdown
	- Geographic distribution
	- Export functionality

	R22-C extracted 8 read-only display components into _components/.
-->

<div class="report-page">
	<!-- Header -->
	<header class="page-header" in:fly={{ y: -20, duration: 400 }}>
		<a class="back-btn" href="/admin/email/campaigns">
			<IconArrowLeft size={20} />
			Back to Campaigns
		</a>

		<div class="header-content">
			<div class="header-title">
				<div class="title-icon">
					<IconChartBar size={28} />
				</div>
				<div>
					<h1>Campaign Report</h1>
					<p class="subtitle">Detailed performance analytics</p>
				</div>
			</div>

			{#if report}
				<div class="header-actions">
					<button class="btn-secondary" onclick={loadReport} disabled={loading}>
						<span class={refreshIconClass()}>
							<IconRefresh size={18} />
						</span>
						Refresh
					</button>
					<ExportButton
						data={exportData}
						filename="campaign-report-{campaignId}"
						formats={['csv', 'json']}
						label="Export Report"
						disabled={loading || !report}
					/>
				</div>
			{/if}
		</div>
	</header>

	{#if loading}
		<!-- Loading State -->
		<div class="loading-state" in:fade={{ duration: 300 }}>
			<div class="loading-spinner"></div>
			<p>Loading campaign report...</p>
		</div>
	{:else if error}
		<!-- Error State -->
		<div class="error-state" in:fade={{ duration: 300 }}>
			<IconAlertTriangle size={48} />
			<h3>Unable to Load Report</h3>
			<p>{error}</p>
			<button class="btn-primary" onclick={loadReport}>Try Again</button>
		</div>
	{:else if report}
		<!-- Report Content -->
		<div class="report-content" in:fade={{ duration: 400 }}>
			<p class="report-scope">
				Recorded outbox results. Provider acceptance does not confirm inbox delivery. Opens and
				clicks may include automated requests. Device, client and geography show only supplied
				observations; a dash means no rate denominator. Historical totals are retained separately.
			</p>
			{#if report.last_error}<p role="status">{report.last_error}</p>{/if}
			<CampaignOverview
				overview={{
					name: report.name,
					subject: report.subject,
					status: report.status,
					sent_at: report.sent_at,
					segment_name: report.segment_name,
					template_name: report.template_name
				}}
				{formatDate}
				{getStatusColor}
			/>

			<MetricsGrid metrics={report.metrics} rates={report.rates} {formatNumber} />

			<div class="dual-panel-row">
				<EngagementChart
					engagementOverTime={report.engagement_over_time}
					{maxEngagement}
					{formatHour}
				/>
				<EngagementQuality
					clickToOpenRate={report.rates.click_to_open_rate}
					opened={report.metrics.opened}
					clicked={report.metrics.clicked}
					{formatNumber}
				/>
			</div>

			<TopLinksTable
				links={report.top_links}
				accepted={report.metrics.sent}
				{formatNumber}
				{truncateUrl}
			/>

			<div class="dual-panel-row">
				<DeviceBreakdown
					deviceBreakdown={report.device_breakdown}
					{totalDevices}
					emailClientBreakdown={report.email_client_breakdown}
				/>
				<GeoDistribution
					distribution={report.geographic_distribution}
					{formatNumber}
					{getFlagEmoji}
				/>
			</div>

			{#if report.ab_test_results?.enabled}
				<AbTestResults results={report.ab_test_results} {formatNumber} />
			{/if}
		</div>
	{/if}
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * EMAIL CAMPAIGN REPORT - shell layout + states.
	 * Section-level styles live inside each extracted component (R22-C).
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.report-page {
		max-width: 1600px;
		padding: 0 1.5rem 2rem;
	}

	/* Header */
	.back-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0;
		color: var(--text-secondary);
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
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary);
		margin: 0;
	}

	.subtitle {
		color: var(--text-tertiary);
		margin: 0.25rem 0 0;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	/* Buttons */
	.btn-primary,
	.btn-secondary {
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
		background: var(--bg-surface);
		color: var(--text-primary);
		border: 1px solid var(--border-default);
	}

	.btn-secondary:hover {
		background: var(--bg-hover);
		border-color: var(--primary-500);
	}

	.btn-secondary :global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	/* Dual Panel Row layout */
	.dual-panel-row {
		display: grid;
		grid-template-columns: 1.2fr 0.8fr;
		gap: 1.5rem;
		margin-bottom: 1.5rem;
	}

	@media (max-width: 1100px) {
		.dual-panel-row {
			grid-template-columns: 1fr;
		}
	}

	/* Loading State */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 6rem 2rem;
		gap: 1.5rem;
	}

	.loading-spinner {
		width: 48px;
		height: 48px;
		border: 3px solid var(--admin-accent-primary-soft, rgba(230, 184, 0, 0.15));
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	.loading-state p {
		color: var(--text-tertiary);
		font-size: 0.875rem;
	}

	/* Error State */
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		background: var(--admin-error-bg, rgba(218, 54, 51, 0.15));
		border: 1px solid var(--admin-error-border, rgba(218, 54, 51, 0.3));
		border-radius: 16px;
		color: var(--admin-error-text, #f85149);
	}

	.error-state h3 {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 1rem 0 0.5rem;
	}

	.error-state p {
		color: var(--text-secondary);
		margin-bottom: 1.5rem;
	}

	/* Responsive */
	@media (max-width: 767.98px) {
		.report-page {
			padding: 0 1rem 1.5rem;
		}

		.header-content {
			flex-direction: column;
			align-items: flex-start;
			gap: 1rem;
		}

		.header-actions {
			width: 100%;
			flex-wrap: wrap;
		}
	}

	@media (max-width: 479.98px) {
		.title-icon {
			width: 48px;
			height: 48px;
		}

		.header-title h1 {
			font-size: 1.5rem;
		}
	}

	/* Touch Device Optimizations */
	@media (hover: none) and (pointer: coarse) {
		.btn-primary,
		.btn-secondary,
		.back-btn {
			min-height: 44px;
		}
	}
</style>
