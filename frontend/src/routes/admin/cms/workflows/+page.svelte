<!--
	CMS Workflow Manager - Apple ICT 11+ Principal Engineer Grade
	10/10 Multi-Stage Approval System
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { fly, scale, fade } from 'svelte/transition';
	import { cubicOut, backOut } from 'svelte/easing';
	import {
		IconGitBranch,
		IconArrowLeft,
		IconCheck,
		IconX,
		IconClock,
		IconUser,
		IconMessageCircle,
		IconAlertTriangle,
		IconChevronRight,
		IconFilter,
		IconSearch,
		IconCalendar,
		IconFileText,
		IconSend,
		IconArrowUpRight
	} from '$lib/icons';

	let mounted = false;
	let isLoading = true;
	let workflows: any[] = [];
	let selectedWorkflow: any = null;
	let filterStage = 'all';
	let searchQuery = '';

	const stages = [
		{ id: 'all', label: 'All Stages', color: 'slate' },
		{ id: 'draft', label: 'Draft', color: 'slate' },
		{ id: 'review', label: 'In Review', color: 'amber' },
		{ id: 'pending_approval', label: 'Pending Approval', color: 'purple' },
		{ id: 'approved', label: 'Approved', color: 'emerald' },
		{ id: 'published', label: 'Published', color: 'blue' },
		{ id: 'rejected', label: 'Rejected', color: 'red' }
	];

	async function fetchWorkflows() {
		isLoading = true;
		try {
			const params = new URLSearchParams();
			if (filterStage !== 'all') params.set('stage', filterStage);
			if (searchQuery) params.set('search', searchQuery);

			const response = await fetch(`/api/admin/cms/workflow?${params}`, {
				credentials: 'include'
			});
			if (response.ok) {
				workflows = await response.json();
			}
		} catch (e) {
			console.error('Failed to fetch workflows:', e);
		} finally {
			isLoading = false;
		}
	}

	async function transitionWorkflow(workflowId: number, action: string, comment?: string) {
		try {
			const response = await fetch(`/api/admin/cms/workflow/${workflowId}/transition`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify({ action, comment })
			});
			if (response.ok) {
				fetchWorkflows();
				selectedWorkflow = null;
			}
		} catch (e) {
			console.error('Failed to transition workflow:', e);
		}
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getStageInfo(stage: string) {
		return stages.find(s => s.id === stage) || stages[0];
	}

	function getPriorityClass(priority: string): string {
		switch (priority) {
			case 'urgent': return 'priority-urgent';
			case 'high': return 'priority-high';
			case 'normal': return 'priority-normal';
			default: return 'priority-low';
		}
	}

	onMount(() => {
		mounted = true;
		fetchWorkflows();
	});
</script>

<div class="workflows-page" class:mounted>
	<!-- Header -->
	<header class="page-header" in:fly={{ y: -20, duration: 500 }}>
		<div class="header-left">
			<a href="/admin/cms" class="back-link">
				<IconArrowLeft size={18} />
				<span>Back to CMS</span>
			</a>
			<div class="header-title">
				<div class="header-icon">
					<IconGitBranch size={24} />
				</div>
				<div>
					<h1>Workflow Manager</h1>
					<p>Manage content approval workflows</p>
				</div>
			</div>
		</div>

		<div class="header-stats">
			{#each stages.slice(1) as stage}
				{@const count = workflows.filter(w => w.current_stage === stage.id).length}
				{#if count > 0}
					<div class="stat-pill {stage.color}">
						<span class="stat-count">{count}</span>
						<span class="stat-label">{stage.label}</span>
					</div>
				{/if}
			{/each}
		</div>
	</header>

	<!-- Filters -->
	<section class="filters-section" in:fly={{ y: 20, duration: 500, delay: 100 }}>
		<div class="search-input">
			<IconSearch size={18} />
			<input
				type="text"
				placeholder="Search workflows..."
				bind:value={searchQuery}
				onkeyup={(e) => e.key === 'Enter' && fetchWorkflows()}
			/>
		</div>

		<div class="stage-tabs">
			{#each stages as stage}
				<button
					class="stage-tab {stage.color}"
					class:active={filterStage === stage.id}
					onclick={() => { filterStage = stage.id; fetchWorkflows(); }}
				>
					{stage.label}
				</button>
			{/each}
		</div>
	</section>

	<!-- Kanban Board -->
	<section class="kanban-board" in:fly={{ y: 20, duration: 500, delay: 200 }}>
		{#if isLoading}
			<div class="loading-state">
				<div class="spinner"></div>
				<p>Loading workflows...</p>
			</div>
		{:else if workflows.length === 0}
			<div class="empty-state" in:scale={{ duration: 400 }}>
				<div class="empty-icon">
					<IconGitBranch size={48} />
				</div>
				<h3>No workflows found</h3>
				<p>Content awaiting review will appear here</p>
			</div>
		{:else}
			<div class="workflow-grid">
				{#each workflows as workflow, i}
					{@const stageInfo = getStageInfo(workflow.current_stage)}
					<div
						class="workflow-card"
						in:fly={{ y: 20, duration: 400, delay: i * 50 }}
						onclick={() => selectedWorkflow = workflow}
					>
						<div class="card-header">
							<span class="stage-badge {stageInfo.color}">{stageInfo.label}</span>
							{#if workflow.priority}
								<span class="priority-indicator {getPriorityClass(workflow.priority)}">
									{workflow.priority}
								</span>
							{/if}
						</div>

						<div class="card-content">
							<div class="content-info">
								<IconFileText size={16} />
								<span class="content-type">{workflow.content_type}</span>
								<span class="content-id">#{workflow.content_id}</span>
							</div>

							{#if workflow.assigned_to_email}
								<div class="assignee">
									<IconUser size={14} />
									<span>{workflow.assigned_to_email}</span>
								</div>
							{/if}
						</div>

						<div class="card-footer">
							{#if workflow.due_date}
								<div class="due-date" class:overdue={new Date(workflow.due_date) < new Date()}>
									<IconClock size={14} />
									<span>{formatDate(workflow.due_date)}</span>
								</div>
							{/if}

							<div class="card-meta">
								<IconCalendar size={14} />
								<span>{formatDate(workflow.updated_at)}</span>
							</div>
						</div>

						<div class="quick-actions">
							{#if workflow.current_stage === 'review' || workflow.current_stage === 'pending_approval'}
								<button
									class="quick-action approve"
									title="Approve"
									onclick={(e) => { e.stopPropagation(); transitionWorkflow(workflow.id, 'approve'); }}
								>
									<IconCheck size={16} />
								</button>
								<button
									class="quick-action reject"
									title="Reject"
									onclick={(e) => { e.stopPropagation(); transitionWorkflow(workflow.id, 'reject'); }}
								>
									<IconX size={16} />
								</button>
							{/if}
							{#if workflow.current_stage === 'approved'}
								<button
									class="quick-action publish"
									title="Publish"
									onclick={(e) => { e.stopPropagation(); transitionWorkflow(workflow.id, 'publish'); }}
								>
									<IconSend size={16} />
								</button>
							{/if}
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</section>

	<!-- Detail Panel -->
	{#if selectedWorkflow}
		<div class="overlay" onclick={() => selectedWorkflow = null} transition:fade={{ duration: 200 }}></div>
		<aside class="detail-panel" in:fly={{ x: 300, duration: 400 }}>
			<div class="panel-header">
				<h2>Workflow Details</h2>
				<button class="close-btn" onclick={() => selectedWorkflow = null}>
					<IconX size={18} />
				</button>
			</div>

			<div class="panel-content">
				<div class="workflow-stages">
					{#each ['draft', 'review', 'pending_approval', 'approved', 'published'] as stage, i}
						{@const info = getStageInfo(stage)}
						{@const isActive = selectedWorkflow.current_stage === stage}
						{@const isPast = stages.findIndex(s => s.id === selectedWorkflow.current_stage) > i}
						<div class="stage-item" class:active={isActive} class:completed={isPast}>
							<div class="stage-dot"></div>
							<span class="stage-name">{info.label}</span>
							{#if i < 4}
								<IconChevronRight size={14} class="stage-arrow" />
							{/if}
						</div>
					{/each}
				</div>

				<div class="info-grid">
					<div class="info-item">
						<span class="info-label">Content</span>
						<span class="info-value">{selectedWorkflow.content_type} #{selectedWorkflow.content_id}</span>
					</div>
					{#if selectedWorkflow.assigned_to_email}
						<div class="info-item">
							<span class="info-label">Assigned To</span>
							<span class="info-value">{selectedWorkflow.assigned_to_email}</span>
						</div>
					{/if}
					{#if selectedWorkflow.due_date}
						<div class="info-item">
							<span class="info-label">Due Date</span>
							<span class="info-value">{formatDate(selectedWorkflow.due_date)}</span>
						</div>
					{/if}
					<div class="info-item">
						<span class="info-label">Last Updated</span>
						<span class="info-value">{formatDate(selectedWorkflow.updated_at)}</span>
					</div>
				</div>

				{#if selectedWorkflow.notes}
					<div class="notes-section">
						<h3>Notes</h3>
						<p>{selectedWorkflow.notes}</p>
					</div>
				{/if}

				<div class="action-buttons">
					{#if selectedWorkflow.current_stage === 'draft'}
						<button class="btn-action amber" onclick={() => transitionWorkflow(selectedWorkflow.id, 'submit_for_review')}>
							Submit for Review
						</button>
					{:else if selectedWorkflow.current_stage === 'review'}
						<button class="btn-action emerald" onclick={() => transitionWorkflow(selectedWorkflow.id, 'approve')}>
							<IconCheck size={16} /> Approve
						</button>
						<button class="btn-action red" onclick={() => transitionWorkflow(selectedWorkflow.id, 'reject')}>
							<IconX size={16} /> Reject
						</button>
					{:else if selectedWorkflow.current_stage === 'pending_approval'}
						<button class="btn-action emerald" onclick={() => transitionWorkflow(selectedWorkflow.id, 'approve')}>
							<IconCheck size={16} /> Final Approval
						</button>
						<button class="btn-action red" onclick={() => transitionWorkflow(selectedWorkflow.id, 'reject')}>
							<IconX size={16} /> Reject
						</button>
					{:else if selectedWorkflow.current_stage === 'approved'}
						<button class="btn-action blue" onclick={() => transitionWorkflow(selectedWorkflow.id, 'publish')}>
							<IconSend size={16} /> Publish Now
						</button>
					{/if}
				</div>
			</div>
		</aside>
	{/if}
</div>

<style>
	.workflows-page {
		max-width: 1600px;
		margin: 0 auto;
		padding: 0 1.5rem 3rem;
		opacity: 0;
		transform: translateY(10px);
		transition: all 0.5s ease;
	}

	.workflows-page.mounted {
		opacity: 1;
		transform: translateY(0);
	}

	/* Header */
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: 1.5rem 0 2rem;
		flex-wrap: wrap;
		gap: 1.5rem;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.85rem;
		font-weight: 500;
		color: #6366f1;
		text-decoration: none;
		margin-bottom: 1rem;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.header-icon {
		width: 56px;
		height: 56px;
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, rgba(245, 158, 11, 0.12) 0%, rgba(245, 158, 11, 0.06) 100%);
		color: #d97706;
	}

	.header-title h1 {
		font-size: 1.75rem;
		font-weight: 800;
		color: #1e293b;
		margin: 0;
	}

	.header-title p {
		font-size: 0.9rem;
		color: #64748b;
		margin: 0.25rem 0 0 0;
	}

	.header-stats {
		display: flex;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	.stat-pill {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		border-radius: 100px;
		font-size: 0.8rem;
	}

	.stat-pill.slate { background: rgba(100, 116, 139, 0.1); color: #475569; }
	.stat-pill.amber { background: rgba(245, 158, 11, 0.1); color: #b45309; }
	.stat-pill.purple { background: rgba(139, 92, 246, 0.1); color: #7c3aed; }
	.stat-pill.emerald { background: rgba(16, 185, 129, 0.1); color: #059669; }
	.stat-pill.blue { background: rgba(59, 130, 246, 0.1); color: #2563eb; }
	.stat-pill.red { background: rgba(239, 68, 68, 0.1); color: #dc2626; }

	.stat-count {
		font-weight: 700;
	}

	/* Filters */
	.filters-section {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.search-input {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.875rem 1.25rem;
		background: #ffffff;
		border: 1px solid rgba(0, 0, 0, 0.06);
		border-radius: 14px;
		max-width: 400px;
		color: #64748b;
	}

	.search-input input {
		flex: 1;
		border: none;
		background: none;
		font-size: 0.9rem;
		color: #1e293b;
		outline: none;
	}

	.stage-tabs {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.stage-tab {
		padding: 0.625rem 1.25rem;
		background: #ffffff;
		border: 1px solid rgba(0, 0, 0, 0.06);
		border-radius: 10px;
		font-size: 0.85rem;
		font-weight: 500;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s;
	}

	.stage-tab:hover {
		background: #f8fafc;
	}

	.stage-tab.active {
		color: #ffffff;
	}

	.stage-tab.active.slate { background: #475569; border-color: #475569; }
	.stage-tab.active.amber { background: #d97706; border-color: #d97706; }
	.stage-tab.active.purple { background: #7c3aed; border-color: #7c3aed; }
	.stage-tab.active.emerald { background: #059669; border-color: #059669; }
	.stage-tab.active.blue { background: #2563eb; border-color: #2563eb; }
	.stage-tab.active.red { background: #dc2626; border-color: #dc2626; }

	/* Kanban Board */
	.kanban-board {
		background: #ffffff;
		border: 1px solid rgba(0, 0, 0, 0.06);
		border-radius: 20px;
		padding: 1.5rem;
		min-height: 500px;
	}

	.workflow-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 1rem;
	}

	.workflow-card {
		background: linear-gradient(135deg, rgba(248, 250, 252, 0.9) 0%, rgba(255, 255, 255, 0.7) 100%);
		border: 1px solid rgba(0, 0, 0, 0.05);
		border-radius: 16px;
		padding: 1.25rem;
		cursor: pointer;
		transition: all 0.25s ease;
		position: relative;
	}

	.workflow-card:hover {
		border-color: rgba(99, 102, 241, 0.25);
		transform: translateY(-2px);
		box-shadow: 0 8px 30px rgba(0, 0, 0, 0.06);
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.stage-badge {
		font-size: 0.7rem;
		font-weight: 700;
		padding: 0.3rem 0.75rem;
		border-radius: 6px;
		text-transform: uppercase;
	}

	.stage-badge.slate { background: rgba(100, 116, 139, 0.12); color: #475569; }
	.stage-badge.amber { background: rgba(245, 158, 11, 0.12); color: #b45309; }
	.stage-badge.purple { background: rgba(139, 92, 246, 0.12); color: #7c3aed; }
	.stage-badge.emerald { background: rgba(16, 185, 129, 0.12); color: #059669; }
	.stage-badge.blue { background: rgba(59, 130, 246, 0.12); color: #2563eb; }
	.stage-badge.red { background: rgba(239, 68, 68, 0.12); color: #dc2626; }

	.priority-indicator {
		font-size: 0.65rem;
		font-weight: 700;
		padding: 0.2rem 0.5rem;
		border-radius: 4px;
		text-transform: uppercase;
	}

	.priority-urgent { background: rgba(239, 68, 68, 0.12); color: #dc2626; }
	.priority-high { background: rgba(249, 115, 22, 0.12); color: #c2410c; }
	.priority-normal { background: rgba(59, 130, 246, 0.12); color: #2563eb; }
	.priority-low { background: rgba(148, 163, 184, 0.12); color: #64748b; }

	.card-content {
		margin-bottom: 1rem;
	}

	.content-info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
		color: #64748b;
	}

	.content-type {
		font-size: 0.95rem;
		font-weight: 600;
		color: #1e293b;
		text-transform: capitalize;
	}

	.content-id {
		font-size: 0.8rem;
		color: #94a3b8;
	}

	.assignee {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		font-size: 0.8rem;
		color: #64748b;
	}

	.card-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-top: 0.75rem;
		border-top: 1px solid rgba(0, 0, 0, 0.04);
	}

	.due-date,
	.card-meta {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		font-size: 0.75rem;
		color: #94a3b8;
	}

	.due-date.overdue {
		color: #dc2626;
	}

	.quick-actions {
		position: absolute;
		top: 1rem;
		right: 1rem;
		display: flex;
		gap: 0.4rem;
		opacity: 0;
		transition: opacity 0.2s;
	}

	.workflow-card:hover .quick-actions {
		opacity: 1;
	}

	.quick-action {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 8px;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.quick-action.approve {
		background: rgba(16, 185, 129, 0.1);
		color: #059669;
	}

	.quick-action.approve:hover {
		background: #059669;
		color: #ffffff;
	}

	.quick-action.reject {
		background: rgba(239, 68, 68, 0.1);
		color: #dc2626;
	}

	.quick-action.reject:hover {
		background: #dc2626;
		color: #ffffff;
	}

	.quick-action.publish {
		background: rgba(59, 130, 246, 0.1);
		color: #2563eb;
	}

	.quick-action.publish:hover {
		background: #2563eb;
		color: #ffffff;
	}

	/* Loading & Empty */
	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem;
		text-align: center;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #f1f5f9;
		border-top-color: #6366f1;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.empty-icon {
		width: 80px;
		height: 80px;
		border-radius: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, rgba(245, 158, 11, 0.1) 0%, rgba(245, 158, 11, 0.05) 100%);
		color: #d97706;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		font-size: 1.1rem;
		font-weight: 700;
		color: #475569;
		margin: 0 0 0.5rem 0;
	}

	.empty-state p,
	.loading-state p {
		font-size: 0.9rem;
		color: #94a3b8;
		margin: 0.5rem 0 0 0;
	}

	/* Detail Panel */
	.overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.3);
		backdrop-filter: blur(4px);
		z-index: 99;
	}

	.detail-panel {
		position: fixed;
		top: 0;
		right: 0;
		width: 480px;
		height: 100vh;
		background: #ffffff;
		border-left: 1px solid rgba(0, 0, 0, 0.08);
		box-shadow: -20px 0 60px rgba(0, 0, 0, 0.1);
		z-index: 100;
		display: flex;
		flex-direction: column;
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid rgba(0, 0, 0, 0.06);
	}

	.panel-header h2 {
		font-size: 1.1rem;
		font-weight: 700;
		color: #1e293b;
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #f8fafc;
		border: none;
		border-radius: 10px;
		color: #64748b;
		cursor: pointer;
	}

	.panel-content {
		flex: 1;
		overflow-y: auto;
		padding: 1.5rem;
	}

	.workflow-stages {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 1rem;
		background: #f8fafc;
		border-radius: 12px;
		margin-bottom: 1.5rem;
		overflow-x: auto;
	}

	.stage-item {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		font-size: 0.75rem;
		color: #94a3b8;
		white-space: nowrap;
	}

	.stage-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: #e2e8f0;
	}

	.stage-item.active .stage-dot {
		background: #6366f1;
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.2);
	}

	.stage-item.completed .stage-dot {
		background: #10b981;
	}

	.stage-item.active .stage-name {
		color: #6366f1;
		font-weight: 600;
	}

	.stage-item.completed .stage-name {
		color: #059669;
	}

	.stage-item :global(.stage-arrow) {
		color: #e2e8f0;
	}

	.info-grid {
		display: grid;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.info-item {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.info-label {
		font-size: 0.75rem;
		font-weight: 500;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.info-value {
		font-size: 0.9rem;
		font-weight: 500;
		color: #1e293b;
	}

	.notes-section {
		margin-bottom: 1.5rem;
	}

	.notes-section h3 {
		font-size: 0.8rem;
		font-weight: 600;
		color: #64748b;
		margin: 0 0 0.5rem 0;
	}

	.notes-section p {
		font-size: 0.9rem;
		color: #475569;
		line-height: 1.6;
		margin: 0;
		padding: 1rem;
		background: #f8fafc;
		border-radius: 10px;
	}

	.action-buttons {
		display: flex;
		gap: 0.75rem;
	}

	.btn-action {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.875rem 1.25rem;
		border: none;
		border-radius: 12px;
		font-size: 0.9rem;
		font-weight: 600;
		color: #ffffff;
		cursor: pointer;
		transition: all 0.25s;
	}

	.btn-action.amber { background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%); }
	.btn-action.emerald { background: linear-gradient(135deg, #10b981 0%, #059669 100%); }
	.btn-action.red { background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%); }
	.btn-action.blue { background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%); }

	.btn-action:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
	}

	@media (max-width: 768px) {
		.detail-panel {
			width: 100%;
		}
	}
</style>
