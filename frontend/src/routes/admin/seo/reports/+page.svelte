<script lang="ts">
	import { onMount } from 'svelte';
	import {
		IconMail,
		IconPlus,
		IconEdit,
		IconTrash,
		IconPlayerPlay,
		IconCalendar,
		IconClock,
		IconCheck,
		IconX,
		IconDownload
	} from '$lib/icons';
	import {
		reportTemplates,
		formatSchedule,
		defaultBranding,
		defaultSections,
		type ReportTemplate,
		type ReportFrequency
	} from '$lib/seo';

	// State using Svelte 5 runes
	let templates = $state<ReportTemplate[]>([]);
	let reports = $state<any[]>([]);
	let showAddModal = $state(false);
	let editingTemplate = $state<ReportTemplate | null>(null);

	// Form state
	let formData = $state({
		name: '',
		frequency: 'weekly' as ReportFrequency,
		recipients: '',
		dayOfWeek: 1,
		dayOfMonth: 1,
		hour: 9,
		minute: 0
	});

	// Stats
	let stats = $derived({
		totalTemplates: templates.length,
		activeTemplates: templates.filter((t) => t.isActive).length,
		reportsGenerated: reports.length
	});

	onMount(() => {
		const unsubscribe = reportTemplates.subscribe((t) => {
			templates = t;
		});

		// Add sample template if empty
		if (templates.length === 0) {
			addSampleTemplate();
		}

		// Sample generated reports
		reports = [
			{
				id: '1',
				templateName: 'Weekly SEO Report',
				generatedAt: new Date(Date.now() - 7 * 24 * 60 * 60 * 1000).toISOString(),
				status: 'sent',
				recipients: 2
			},
			{
				id: '2',
				templateName: 'Weekly SEO Report',
				generatedAt: new Date(Date.now() - 14 * 24 * 60 * 60 * 1000).toISOString(),
				status: 'sent',
				recipients: 2
			}
		];

		return () => unsubscribe();
	});

	function addSampleTemplate() {
		reportTemplates.add({
			name: 'Weekly SEO Report',
			description: 'Comprehensive weekly SEO performance report',
			frequency: 'weekly',
			format: 'html',
			sections: defaultSections,
			recipients: [{ id: '1', name: 'Admin', email: 'admin@revolutiontradingpros.com' }],
			branding: defaultBranding,
			schedule: {
				dayOfWeek: 1, // Monday
				hour: 9,
				minute: 0,
				timezone: 'America/New_York'
			},
			isActive: true
		});
	}

	function openAddModal() {
		editingTemplate = null;
		formData = {
			name: '',
			frequency: 'weekly',
			recipients: '',
			dayOfWeek: 1,
			dayOfMonth: 1,
			hour: 9,
			minute: 0
		};
		showAddModal = true;
	}

	function openEditModal(template: ReportTemplate) {
		editingTemplate = template;
		formData = {
			name: template.name,
			frequency: template.frequency,
			recipients: template.recipients?.map((r) => r?.email || '').join(', ') || '',
			dayOfWeek: template.schedule.dayOfWeek || 1,
			dayOfMonth: template.schedule.dayOfMonth || 1,
			hour: template.schedule.hour,
			minute: template.schedule.minute
		};
		showAddModal = true;
	}

	function saveTemplate() {
		const emails = formData.recipients
			.split(',')
			.map((e) => e.trim())
			.filter(Boolean);
		const recipients = emails.map((email, i) => ({
			id: String(i + 1),
			name: email.split('@')[0] || '',
			email
		}));

		const templateData = {
			name: formData.name,
			description: '',
			frequency: formData.frequency,
			format: 'html' as const,
			sections: defaultSections,
			recipients,
			branding: defaultBranding,
			schedule: {
				dayOfWeek: formData.dayOfWeek,
				dayOfMonth: formData.dayOfMonth,
				hour: formData.hour,
				minute: formData.minute,
				timezone: 'America/New_York'
			},
			isActive: true
		};

		if (editingTemplate) {
			reportTemplates.update(editingTemplate.id, templateData);
		} else {
			reportTemplates.add(templateData);
		}

		showAddModal = false;
	}

	function deleteTemplate(id: string) {
		if (confirm('Delete this report template?')) {
			reportTemplates.remove(id);
		}
	}

	function toggleTemplate(id: string) {
		reportTemplates.toggle(id);
	}

	async function runReport(template: ReportTemplate) {
		alert(`Generating report: ${template.name}...`);
		// In production, call API to generate report
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'sent':
				return '#10b981';
			case 'failed':
				return '#ef4444';
			default:
				return '#f59e0b';
		}
	}

	const frequencyOptions = [
		{ value: 'daily', label: 'Daily' },
		{ value: 'weekly', label: 'Weekly' },
		{ value: 'monthly', label: 'Monthly' },
		{ value: 'quarterly', label: 'Quarterly' }
	];

	const dayOptions = [
		{ value: 0, label: 'Sunday' },
		{ value: 1, label: 'Monday' },
		{ value: 2, label: 'Tuesday' },
		{ value: 3, label: 'Wednesday' },
		{ value: 4, label: 'Thursday' },
		{ value: 5, label: 'Friday' },
		{ value: 6, label: 'Saturday' }
	];
</script>

<svelte:head>
	<title>SEO Reports | SEO</title>
</svelte:head>

<div class="reports-page">
	<header class="page-header">
		<div>
			<h1>
				<IconMail size={28} />
				SEO Email Reports
			</h1>
			<p>Schedule automated SEO reports delivered to your inbox</p>
		</div>
		<div class="header-actions">
			<button class="btn-primary" onclick={openAddModal}>
				<IconPlus size={18} />
				New Report Template
			</button>
		</div>
	</header>

	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-value">{stats.totalTemplates}</div>
			<div class="stat-label">Report Templates</div>
		</div>
		<div class="stat-card">
			<div class="stat-value">{stats.activeTemplates}</div>
			<div class="stat-label">Active</div>
		</div>
		<div class="stat-card">
			<div class="stat-value">{stats.reportsGenerated}</div>
			<div class="stat-label">Reports Generated</div>
		</div>
	</div>

	<div class="section">
		<h2>Report Templates</h2>
		{#if templates.length === 0}
			<div class="empty-state">
				<IconMail size={48} />
				<p>No report templates yet. Create your first automated report.</p>
				<button class="btn-primary" onclick={openAddModal}>
					<IconPlus size={18} />
					Create Template
				</button>
			</div>
		{:else}
			<div class="templates-list">
				{#each templates as template}
					<div class="template-card" class:inactive={!template.isActive}>
						<div class="template-header">
							<h3>{template.name}</h3>
							<div class="template-status">
								{#if template.isActive}
									<span class="active"><IconCheck size={14} /> Active</span>
								{:else}
									<span class="inactive"><IconX size={14} /> Inactive</span>
								{/if}
							</div>
						</div>

						<div class="template-details">
							<div class="detail">
								<IconCalendar size={16} />
								<span
									>{template.frequency.charAt(0).toUpperCase() + template.frequency.slice(1)}</span
								>
							</div>
							<div class="detail">
								<IconClock size={16} />
								<span>{formatSchedule(template.schedule, template.frequency)}</span>
							</div>
							<div class="detail">
								<IconMail size={16} />
								<span>{template.recipients.length} recipient(s)</span>
							</div>
						</div>

						{#if template.schedule.nextRun}
							<div class="next-run">
								Next run: {new Date(template.schedule.nextRun).toLocaleString()}
							</div>
						{/if}

						<div class="template-actions">
							<button
								class="action-btn primary"
								onclick={() => runReport(template)}
								title="Run Now"
							>
								<IconPlayerPlay size={16} />
								Run Now
							</button>
							<button class="action-btn" onclick={() => openEditModal(template)} title="Edit">
								<IconEdit size={16} />
							</button>
							<button
								class="action-btn"
								onclick={() => toggleTemplate(template.id)}
								title={template.isActive ? 'Pause' : 'Activate'}
							>
								{template.isActive ? 'Pause' : 'Activate'}
							</button>
							<button
								class="action-btn danger"
								onclick={() => deleteTemplate(template.id)}
								title="Delete"
							>
								<IconTrash size={16} />
							</button>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>

	<div class="section">
		<h2>Recent Reports</h2>
		{#if reports.length === 0}
			<div class="empty-state small">
				<p>No reports generated yet.</p>
			</div>
		{:else}
			<div class="reports-list">
				{#each reports as report}
					<div class="report-row">
						<div class="report-info">
							<span class="report-name">{report.templateName}</span>
							<span class="report-date">{new Date(report.generatedAt).toLocaleDateString()}</span>
						</div>
						<div class="report-meta">
							<span class="report-status" style="color: {getStatusColor(report.status)}">
								{report.status}
							</span>
							<span class="report-recipients">{report.recipients} recipients</span>
						</div>
						<button class="action-btn" title="Download">
							<IconDownload size={16} />
						</button>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>

{#if showAddModal}
	<div
		class="modal-overlay"
		onclick={() => (showAddModal = false)}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showAddModal = false)}
		role="button"
		tabindex="-1"
		aria-label="Close modal"
	>
		<div
			class="modal"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
		>
			<h3>{editingTemplate ? 'Edit Report Template' : 'New Report Template'}</h3>

			<div class="form-group">
				<label for="name">Report Name *</label>
				<input type="text" id="name" name="name" bind:value={formData.name} placeholder="Weekly SEO Report" />
			</div>

			<div class="form-group">
				<label for="frequency">Frequency</label>
				<select id="frequency" bind:value={formData.frequency}>
					{#each frequencyOptions as opt}
						<option value={opt.value}>{opt.label}</option>
					{/each}
				</select>
			</div>

			{#if formData.frequency === 'weekly'}
				<div class="form-group">
					<label for="dayOfWeek">Day of Week</label>
					<select id="dayOfWeek" bind:value={formData.dayOfWeek}>
						{#each dayOptions as day}
							<option value={day.value}>{day.label}</option>
						{/each}
					</select>
				</div>
			{/if}

			{#if formData.frequency === 'monthly' || formData.frequency === 'quarterly'}
				<div class="form-group">
					<label for="dayOfMonth">Day of Month</label>
					<input type="number" id="dayOfMonth" name="dayOfMonth" bind:value={formData.dayOfMonth} min="1" max="28" />
				</div>
			{/if}

			<div class="form-row">
				<div class="form-group">
					<label for="hour">Hour</label>
					<input type="number" id="hour" name="hour" bind:value={formData.hour} min="0" max="23" />
				</div>
				<div class="form-group">
					<label for="minute">Minute</label>
					<input type="number" id="minute" name="minute" bind:value={formData.minute} min="0" max="59" />
				</div>
			</div>

			<div class="form-group">
				<label for="recipients">Recipients (comma-separated emails) *</label>
				<input
					type="text"
					id="recipients" name="recipients"
					bind:value={formData.recipients}
					placeholder="admin@example.com, team@example.com"
				/>
			</div>

			<div class="modal-actions">
				<button class="btn-secondary" onclick={() => (showAddModal = false)}>Cancel</button>
				<button class="btn-primary" onclick={saveTemplate}>
					{editingTemplate ? 'Save Changes' : 'Create Template'}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.reports-page {
		padding: 2rem;
		max-width: 1200px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 1.75rem;
		font-weight: 700;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: #666;
		font-size: 0.95rem;
	}

	.header-actions {
		display: flex;
		gap: 1rem;
	}

	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
		border: none;
	}

	.btn-primary {
		background: #3b82f6;
		color: white;
	}

	.btn-primary:hover {
		background: #2563eb;
	}

	.btn-secondary {
		background: white;
		color: #374151;
		border: 1px solid #e5e5e5;
	}

	.btn-secondary:hover {
		background: #f9fafb;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		padding: 1.5rem;
		text-align: center;
	}

	.stat-value {
		font-size: 1.75rem;
		font-weight: 700;
		color: #1a1a1a;
	}

	.stat-label {
		color: #666;
		font-size: 0.85rem;
	}

	.section {
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 12px;
		padding: 1.5rem;
		margin-bottom: 2rem;
	}

	.section h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 1.5rem;
	}

	.empty-state {
		text-align: center;
		padding: 3rem;
		color: #666;
	}

	.empty-state.small {
		padding: 2rem;
	}

	.empty-state p {
		margin: 1rem 0;
	}

	.templates-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.template-card {
		padding: 1.5rem;
		background: #f9fafb;
		border-radius: 8px;
		border: 1px solid #e5e5e5;
	}

	.template-card.inactive {
		opacity: 0.6;
	}

	.template-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.template-header h3 {
		font-size: 1.1rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0;
	}

	.template-status {
		font-size: 0.85rem;
	}

	.template-status .active {
		color: #10b981;
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.template-status .inactive {
		color: #6b7280;
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.template-details {
		display: flex;
		gap: 1.5rem;
		margin-bottom: 1rem;
		flex-wrap: wrap;
	}

	.detail {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.9rem;
		color: #374151;
	}

	.next-run {
		font-size: 0.85rem;
		color: #666;
		margin-bottom: 1rem;
	}

	.template-actions {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.action-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.5rem 0.75rem;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 4px;
		font-size: 0.8rem;
		cursor: pointer;
		color: #374151;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: #f3f4f6;
	}

	.action-btn.primary {
		background: #3b82f6;
		color: white;
		border-color: #3b82f6;
	}

	.action-btn.primary:hover {
		background: #2563eb;
	}

	.action-btn.danger {
		color: #ef4444;
	}

	.action-btn.danger:hover {
		background: #fee2e2;
	}

	.reports-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.report-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem;
		background: #f9fafb;
		border-radius: 6px;
	}

	.report-info {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.report-name {
		font-weight: 500;
		color: #1a1a1a;
	}

	.report-date {
		font-size: 0.85rem;
		color: #666;
	}

	.report-meta {
		display: flex;
		gap: 1rem;
		font-size: 0.85rem;
	}

	.report-status {
		font-weight: 500;
		text-transform: capitalize;
	}

	.report-recipients {
		color: #666;
	}

	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1rem;
	}

	.modal {
		background: white;
		border-radius: 12px;
		padding: 2rem;
		max-width: 500px;
		width: 100%;
	}

	.modal h3 {
		font-size: 1.25rem;
		font-weight: 600;
		margin-bottom: 1.5rem;
	}

	.form-group {
		margin-bottom: 1rem;
	}

	.form-group label {
		display: block;
		font-weight: 500;
		font-size: 0.9rem;
		color: #374151;
		margin-bottom: 0.5rem;
	}

	.form-group input,
	.form-group select {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.95rem;
	}

	.form-group input:focus,
	.form-group select:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.form-row {
		display: flex;
		gap: 1rem;
	}

	.form-row .form-group {
		flex: 1;
	}

	.modal-actions {
		display: flex;
		gap: 1rem;
		justify-content: flex-end;
		margin-top: 1.5rem;
	}

	@media (max-width: 768px) {
		.reports-page {
			padding: 1rem;
		}

		.page-header {
			flex-direction: column;
			gap: 1rem;
		}

		.template-details {
			flex-direction: column;
			gap: 0.5rem;
		}
	}
</style>
