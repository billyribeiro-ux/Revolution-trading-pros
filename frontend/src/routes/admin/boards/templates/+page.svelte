<script lang="ts">
	/** Pick dark or light text for a data-driven hex colour (WCAG relative luminance). */
	function readableOn(hex: string | undefined | null): string {
		const m = /^#?([0-9a-f]{6})$/i.exec((hex ?? '').trim());
		if (!m) return 'var(--text-primary, #f0f6fc)';
		const n = parseInt(m[1]!, 16);
		const lin = (v: number) => {
			const c = v / 255;
			return c <= 0.03928 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4);
		};
		const L = 0.2126 * lin((n >> 16) & 255) + 0.7152 * lin((n >> 8) & 255) + 0.0722 * lin(n & 255);
		// Pick whichever of the two text colours has the higher WCAG contrast ratio.
		return (L + 0.05) / 0.056 > 0.97 / (L + 0.05) ? '#0d1117' : '#f0f6fc';
	}
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { boardsAPI } from '#lib/api/boards.js';
	import type { BoardTemplate } from '#lib/boards/types.js';
	import { logger } from '#lib/utils/logger.js';
	import {
		IconTemplate,
		IconArrowLeft,
		IconSearch,
		IconPlus,
		IconLayoutKanban,
		IconCode,
		IconBriefcase,
		IconUsers,
		IconRocket,
		IconBook,
		IconChartBar
	} from '#lib/icons/index.js';

	// State
	let templates = $state<BoardTemplate[]>([]);
	let loading = $state(true);
	let searchQuery = $state('');
	let selectedCategory = $state<string | null>(null);
	let creatingBoard = $state<string | null>(null);

	// Categories
	const categories = [
		{ id: 'software', name: 'Software Development', icon: IconCode },
		{ id: 'marketing', name: 'Marketing', icon: IconChartBar },
		{ id: 'business', name: 'Business', icon: IconBriefcase },
		{ id: 'team', name: 'Team Management', icon: IconUsers },
		{ id: 'product', name: 'Product Launch', icon: IconRocket },
		{ id: 'education', name: 'Education', icon: IconBook }
	];

	// Default templates (in case API doesn't return any)
	const defaultTemplates: BoardTemplate[] = [
		{
			id: 'kanban-basic',
			title: 'Basic Kanban',
			description: 'Simple kanban board with To Do, In Progress, and Done columns',
			category: 'team',
			stages: [
				{ title: 'To Do', color: '#6b7280', position: 1, auto_complete: false },
				{ title: 'In Progress', color: '#3b82f6', position: 2, auto_complete: false },
				{ title: 'Done', color: '#22c55e', position: 3, auto_complete: true }
			],
			labels: [
				{ title: 'Bug', color: '#ef4444' },
				{ title: 'Feature', color: '#B38F00' },
				{ title: 'Enhancement', color: '#06b6d4' }
			],
			custom_fields: [],
			is_default: true,
			created_at: new Date().toISOString()
		},
		{
			id: 'software-sprint',
			title: 'Software Sprint',
			description: 'Agile sprint board with backlog, development, review, and deployment stages',
			category: 'software',
			stages: [
				{ title: 'Backlog', color: '#6b7280', position: 1, auto_complete: false },
				{ title: 'Sprint Planning', color: '#B38F00', position: 2, auto_complete: false },
				{ title: 'In Development', color: '#3b82f6', position: 3, auto_complete: false },
				{ title: 'Code Review', color: '#f59e0b', position: 4, auto_complete: false },
				{ title: 'QA Testing', color: '#ec4899', position: 5, auto_complete: false },
				{ title: 'Done', color: '#22c55e', position: 6, auto_complete: true }
			],
			labels: [
				{ title: 'Bug', color: '#ef4444' },
				{ title: 'Feature', color: '#B38F00' },
				{ title: 'Tech Debt', color: '#f59e0b' },
				{ title: 'Documentation', color: '#06b6d4' },
				{ title: 'Critical', color: '#dc2626' }
			],
			custom_fields: [],
			is_default: false,
			created_at: new Date().toISOString()
		},
		{
			id: 'content-pipeline',
			title: 'Content Pipeline',
			description: 'Track content from ideation to publication',
			category: 'marketing',
			stages: [
				{ title: 'Ideas', color: '#B38F00', position: 1, auto_complete: false },
				{ title: 'Research', color: '#3b82f6', position: 2, auto_complete: false },
				{ title: 'Writing', color: '#f59e0b', position: 3, auto_complete: false },
				{ title: 'Editing', color: '#ec4899', position: 4, auto_complete: false },
				{ title: 'Review', color: '#06b6d4', position: 5, auto_complete: false },
				{ title: 'Scheduled', color: '#10b981', position: 6, auto_complete: false },
				{ title: 'Published', color: '#22c55e', position: 7, auto_complete: true }
			],
			labels: [
				{ title: 'Blog', color: '#3b82f6' },
				{ title: 'Social', color: '#ec4899' },
				{ title: 'Video', color: '#ef4444' },
				{ title: 'Newsletter', color: '#B38F00' }
			],
			custom_fields: [],
			is_default: false,
			created_at: new Date().toISOString()
		},
		{
			id: 'course-creation',
			title: 'Course Creation',
			description: 'Manage online course development from planning to launch',
			category: 'education',
			stages: [
				{ title: 'Planning', color: '#6b7280', position: 1, auto_complete: false },
				{ title: 'Scripting', color: '#B38F00', position: 2, auto_complete: false },
				{ title: 'Recording', color: '#3b82f6', position: 3, auto_complete: false },
				{ title: 'Editing', color: '#f59e0b', position: 4, auto_complete: false },
				{ title: 'Review', color: '#ec4899', position: 5, auto_complete: false },
				{ title: 'Published', color: '#22c55e', position: 6, auto_complete: true }
			],
			labels: [
				{ title: 'Video', color: '#ef4444' },
				{ title: 'Quiz', color: '#B38F00' },
				{ title: 'Resource', color: '#06b6d4' },
				{ title: 'Assignment', color: '#f59e0b' }
			],
			custom_fields: [],
			is_default: false,
			created_at: new Date().toISOString()
		},
		{
			id: 'product-launch',
			title: 'Product Launch',
			description: 'Plan and execute a successful product launch',
			category: 'product',
			stages: [
				{ title: 'Planning', color: '#6b7280', position: 1, auto_complete: false },
				{ title: 'Development', color: '#3b82f6', position: 2, auto_complete: false },
				{ title: 'Testing', color: '#f59e0b', position: 3, auto_complete: false },
				{ title: 'Marketing Prep', color: '#ec4899', position: 4, auto_complete: false },
				{ title: 'Pre-Launch', color: '#B38F00', position: 5, auto_complete: false },
				{ title: 'Launched', color: '#22c55e', position: 6, auto_complete: true }
			],
			labels: [
				{ title: 'Must Have', color: '#ef4444' },
				{ title: 'Nice to Have', color: '#f59e0b' },
				{ title: 'Marketing', color: '#ec4899' },
				{ title: 'Technical', color: '#3b82f6' }
			],
			custom_fields: [],
			is_default: false,
			created_at: new Date().toISOString()
		},
		{
			id: 'crm-pipeline',
			title: 'Sales Pipeline',
			description: 'Track leads and deals through your sales process',
			category: 'business',
			stages: [
				{ title: 'Lead', color: '#6b7280', position: 1, auto_complete: false },
				{ title: 'Contacted', color: '#3b82f6', position: 2, auto_complete: false },
				{ title: 'Qualified', color: '#B38F00', position: 3, auto_complete: false },
				{ title: 'Proposal', color: '#f59e0b', position: 4, auto_complete: false },
				{ title: 'Negotiation', color: '#ec4899', position: 5, auto_complete: false },
				{ title: 'Won', color: '#22c55e', position: 6, auto_complete: true },
				{ title: 'Lost', color: '#ef4444', position: 7, auto_complete: false }
			],
			labels: [
				{ title: 'Hot Lead', color: '#ef4444' },
				{ title: 'Warm Lead', color: '#f59e0b' },
				{ title: 'Cold Lead', color: '#3b82f6' },
				{ title: 'Enterprise', color: '#B38F00' }
			],
			custom_fields: [],
			is_default: false,
			created_at: new Date().toISOString()
		}
	];

	onMount(async () => {
		await loadTemplates();
	});

	async function loadTemplates() {
		loading = true;
		try {
			const res = await boardsAPI.getTemplates();
			templates = res.length > 0 ? res : defaultTemplates;
		} catch (error) {
			logger.error('Failed to load templates:', error);
			templates = defaultTemplates;
		} finally {
			loading = false;
		}
	}

	let filteredTemplates = $derived.by(() => {
		return templates.filter((t) => {
			if (searchQuery) {
				const query = searchQuery.toLowerCase();
				if (
					!t.title.toLowerCase().includes(query) &&
					!t.description.toLowerCase().includes(query)
				) {
					return false;
				}
			}
			if (selectedCategory && t.category !== selectedCategory) {
				return false;
			}
			return true;
		});
	});

	async function useTemplate(template: BoardTemplate) {
		creatingBoard = template.id;
		try {
			const board = await boardsAPI.createFromTemplate(template.id, template.title);
			goto(`/admin/boards/${board.id}`);
		} catch (error) {
			logger.error('Failed to create board from template:', error);
			// FIX-2026-04-26 (P1-3): manual fallback — if any stage/label creation fails
			// mid-flight, roll back the partially-created board so admins don't end up
			// with orphaned half-built boards. Best-effort: a delete failure is logged
			// but the original error is preserved as the user-visible reason.
			let createdBoardId: string | null = null;
			try {
				const board = await boardsAPI.createBoard({
					title: template.title,
					description: template.description,
					type: 'kanban'
				});
				createdBoardId = board.id;

				// Create stages
				for (const stage of template.stages) {
					await boardsAPI.createStage(board.id, stage);
				}

				// Create labels
				for (const label of template.labels) {
					await boardsAPI.createLabel(board.id, label);
				}

				goto(`/admin/boards/${board.id}`);
			} catch (err) {
				logger.error('Failed to create board:', err);
				// Compensating delete to keep state consistent.
				if (createdBoardId) {
					try {
						await boardsAPI.deleteBoard(createdBoardId);
					} catch (rollbackErr) {
						logger.error(
							'Failed to roll back partially-created board ',
							createdBoardId,
							rollbackErr
						);
					}
				}
				// TODO(2026-04-26-audit): replace console-only failure with toast surface.
			}
		} finally {
			creatingBoard = null;
		}
	}

	function getCategoryIcon(categoryId: string) {
		const category = categories.find((c) => c.id === categoryId);
		return category?.icon || IconLayoutKanban;
	}

	function getFilterButtonClass(active: boolean): Record<string, boolean> {
		return {
			'filter-button': true,
			'filter-button--active': active
		};
	}
</script>

<svelte:head>
	<title>Board Templates | Project Boards</title>
</svelte:head>

<div class="templates-page">
	<!-- Header -->
	<div class="page-header">
		<div class="page-container page-container--header">
			<div class="header-layout">
				<div class="header-title-row">
					<a href="/admin/boards" class="back-link" aria-label="Back to boards">
						<IconArrowLeft class="back-link__icon" />
					</a>
					<div class="title-cluster">
						<div class="title-icon">
							<IconTemplate class="title-icon__svg" />
						</div>
						<div>
							<h1 class="page-title">Board Templates</h1>
							<p class="page-subtitle">Start with a pre-built board configuration</p>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>

	<div class="page-container page-content">
		<!-- Search and Filters -->
		<div class="toolbar">
			<div class="search-field">
				<IconSearch class="search-field__icon" />
				<input autocomplete="off"
					id="page-searchquery"
					name="page-searchquery"
					type="text"
					placeholder="Search templates..."
					bind:value={searchQuery}
					class="search-field__input"
				/>
			</div>

			<div class="filter-row" aria-label="Template categories">
				<button
					type="button"
					onclick={() => (selectedCategory = null)}
					class={getFilterButtonClass(selectedCategory === null)}
				>
					All
				</button>
				{#each categories as category (category.id)}
					<button
						type="button"
						onclick={() => (selectedCategory = category.id)}
						class={getFilterButtonClass(selectedCategory === category.id)}
					>
						<category.icon class="filter-button__icon" />
						{category.name}
					</button>
				{/each}
			</div>
		</div>

		<!-- Templates Grid -->
		{#if loading}
			<div class="loading-state">
				<div class="spinner" aria-label="Loading templates"></div>
			</div>
		{:else if filteredTemplates.length === 0}
			<div class="empty-card">
				<IconTemplate class="empty-card__icon" />
				<h3 class="empty-card__title">No templates found</h3>
				<p class="empty-card__text">Try adjusting your search or filters</p>
			</div>
		{:else}
			<div class="templates-grid">
				{#each filteredTemplates as template (template.id)}
					{@const CategoryIcon = getCategoryIcon(template.category)}
					<article class="template-card">
						<!-- Preview Header -->
						<div class="template-preview">
							<div class="stage-preview-list">
								{#each template.stages.slice(0, 4) as stage, i (i)}
									<div class="stage-preview" style:background-color={stage.color} style:color={readableOn(stage.color)}>
										<div class="stage-preview__body">
											<div class="stage-preview__title">{stage.title}</div>
											<div class="stage-preview__lines">
												<div class="stage-preview__line"></div>
												<div class="stage-preview__line stage-preview__line--short"></div>
											</div>
										</div>
									</div>
								{/each}
								{#if template.stages.length > 4}
									<div class="stage-preview stage-preview--more">
										<span class="stage-preview__more-count">+{template.stages.length - 4}</span>
									</div>
								{/if}
							</div>
						</div>

						<!-- Content -->
						<div class="template-card__content">
							<div class="template-meta">
								<CategoryIcon class="template-meta__icon" />
								<span class="template-meta__category">{template.category}</span>
								{#if template.is_default}
									<span class="default-badge">Default</span>
								{/if}
							</div>
							<h3 class="template-card__title">{template.title}</h3>
							<p class="template-card__description">{template.description}</p>

							<!-- Template Info -->
							<div class="template-stats">
								<span>{template.stages.length} stages</span>
								<span>{template.labels.length} labels</span>
								{#if template.custom_fields.length > 0}
									<span>{template.custom_fields.length} custom fields</span>
								{/if}
							</div>

							<!-- Labels Preview -->
							<div class="label-list">
								{#each template.labels.slice(0, 4) as label, i (i)}
									<span class="label-pill" style:background-color={label.color} style:color={readableOn(label.color)}>
										{label.title}
									</span>
								{/each}
								{#if template.labels.length > 4}
									<span class="label-overflow">+{template.labels.length - 4}</span>
								{/if}
							</div>

							<!-- Action -->
							<button
								type="button"
								onclick={() => useTemplate(template)}
								disabled={creatingBoard === template.id}
								class="use-template-button"
							>
								{#if creatingBoard === template.id}
									<div class="button-spinner" aria-hidden="true"></div>
									Creating...
								{:else}
									<IconPlus class="use-template-button__icon" />
									Use Template
								{/if}
							</button>
						</div>
					</article>
				{/each}
			</div>
		{/if}

		<!-- Create Custom Template -->
		<div class="custom-template-card">
			<div class="custom-template-card__layout">
				<div>
					<h3 class="custom-template-card__title">Create Custom Template</h3>
					<p class="custom-template-card__text">Save any board as a template to reuse later</p>
				</div>
				<a href="/admin/boards" class="custom-template-card__link"> Go to Boards </a>
			</div>
		</div>
	</div>
</div>

<style>
	.templates-page {
		min-height: 100vh;
		background: var(--bg-elevated, #161b22);
		color: var(--text-primary, #f0f6fc);
	}

	.templates-page {
		background: var(--bg-base, #0d1117);
		color: var(--text-primary, #f0f6fc);
	}

	.page-header {
		border-bottom: 1px solid var(--border-default, #30363d);
		background: var(--bg-elevated, #161b22);
	}

	.page-header {
		border-color: var(--border-default, #30363d);
		background: var(--bg-elevated, #161b22);
	}

	.page-container {
		width: min(100% - 2rem, 80rem);
		margin: 0 auto;
	}

	.page-container--header {
		padding-block: 1.5rem;
	}

	.page-content {
		padding-block: 2rem;
	}

	.header-layout {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.header-title-row,
	.title-cluster {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.title-cluster {
		gap: 0.75rem;
	}

	.back-link,
	.title-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		border-radius: 0.5rem;
	}

	.back-link {
		width: 2.25rem;
		height: 2.25rem;
		color: var(--text-tertiary, #6e7681);
		transition:
			background-color 160ms ease,
			color 160ms ease;
	}

	.back-link:hover {
		background: var(--bg-surface, #1c2128);
		color: var(--text-primary, #f0f6fc);
	}

	.back-link {
		color: var(--text-secondary, #8b949e);
	}

	.back-link:hover {
		background: var(--bg-hover, #252b33);
		color: var(--text-primary, #f0f6fc);
	}

	.back-link__icon {
		width: 1.25rem;
		height: 1.25rem;
	}

	.title-icon {
		width: 2.5rem;
		height: 2.5rem;
		background: rgba(230, 184, 0, 0.1);
		color: var(--primary-500, #e6b800);
	}

	.title-icon {
		background: rgba(179, 143, 0, 0.3);
		color: var(--primary-400, #ffd11a);
	}

	.title-icon__svg {
		width: 1.5rem;
		height: 1.5rem;
	}

	.page-title {
		margin: 0;
		color: var(--text-primary, #f0f6fc);
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 1.25;
	}

	.page-title {
		color: #ffffff;
	}

	.page-subtitle {
		margin: 0.125rem 0 0;
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
	}

	.page-subtitle {
		color: var(--text-secondary, #8b949e);
	}

	.toolbar {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.search-field {
		position: relative;
		flex: 1 1 auto;
	}

	.search-field__icon {
		position: absolute;
		top: 50%;
		left: 0.75rem;
		width: 1.25rem;
		height: 1.25rem;
		color: var(--text-secondary, #8b949e);
		transform: translateY(-50%);
		pointer-events: none;
	}

	.search-field__input {
		width: 100%;
		border: 1px solid var(--border-default, #30363d);
		border-radius: 0.5rem;
		background: var(--bg-elevated, #161b22);
		color: var(--text-primary, #f0f6fc);
		padding: 0.5rem 1rem 0.5rem 2.5rem;
		font: inherit;
	}

	.search-field__input:focus {
		border-color: #e6b800;
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.24);
		outline: none;
	}

	.search-field__input {
		border-color: var(--border-default, #30363d);
		background: var(--bg-elevated, #161b22);
		color: #ffffff;
	}

	.filter-row {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		overflow-x: auto;
		padding-bottom: 0.5rem;
	}

	.filter-button {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		white-space: nowrap;
		border: 1px solid var(--border-default, #30363d);
		border-radius: 0.5rem;
		background: var(--bg-elevated, #161b22);
		color: var(--text-primary, #f0f6fc);
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
		line-height: 1.25rem;
		transition:
			background-color 160ms ease,
			border-color 160ms ease,
			color 160ms ease;
	}

	.filter-button:hover {
		background: var(--bg-elevated, #161b22);
	}

	.filter-button--active,
	.filter-button--active:hover {
		border-color: #e6b800;
		background: #e6b800;
		color: #0d1117;
	}

	.filter-button {
		border-color: var(--border-default, #30363d);
		background: var(--bg-elevated, #161b22);
		color: var(--text-secondary, #8b949e);
	}

	.filter-button:hover {
		background: var(--bg-hover, #252b33);
	}

	.filter-button--active,
	.filter-button--active:hover {
		border-color: #e6b800;
		background: #e6b800;
		color: #0d1117;
	}

	.filter-button__icon {
		width: 1rem;
		height: 1rem;
	}

	.loading-state {
		display: flex;
		align-items: center;
		justify-content: center;
		padding-block: 3rem;
	}

	.spinner,
	.button-spinner {
		border-radius: 999px;
		animation: spin 800ms linear infinite;
	}

	.spinner {
		width: 2rem;
		height: 2rem;
		border: 2px solid rgba(230, 184, 0, 0.2);
		border-bottom-color: #e6b800;
	}

	.empty-card {
		border: 1px solid var(--border-default, #30363d);
		border-radius: 0.75rem;
		background: var(--bg-elevated, #161b22);
		padding: 3rem 1rem;
		text-align: center;
	}

	.empty-card {
		border-color: var(--border-default, #30363d);
		background: var(--bg-elevated, #161b22);
	}

	.empty-card__icon {
		width: 3rem;
		height: 3rem;
		margin: 0 auto 1rem;
		color: var(--text-secondary, #8b949e);
	}

	.empty-card__title {
		margin: 0 0 0.5rem;
		color: var(--text-primary, #f0f6fc);
		font-size: 1.125rem;
		font-weight: 500;
		line-height: 1.5rem;
	}

	.empty-card__title {
		color: #ffffff;
	}

	.empty-card__text {
		margin: 0;
		color: var(--text-tertiary, #6e7681);
	}

	.empty-card__text {
		color: var(--text-secondary, #8b949e);
	}

	.templates-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1.5rem;
	}

	.template-card {
		overflow: hidden;
		border: 1px solid var(--border-default, #30363d);
		border-radius: 0.75rem;
		background: var(--bg-elevated, #161b22);
		transition:
			box-shadow 180ms ease,
			transform 180ms ease;
	}

	.template-card:hover {
		box-shadow: 0 4px 6px -4px rgba(17, 24, 39, 0.1);
	}

	.template-card {
		border-color: var(--border-default, #30363d);
		background: var(--bg-elevated, #161b22);
	}

	.template-preview {
		display: flex;
		align-items: flex-end;
		height: 8rem;
		background: var(--bg-surface, #1c2128);
		padding: 1rem;
	}

	.stage-preview-list {
		display: flex;
		gap: 0.5rem;
		max-width: 100%;
		overflow: hidden;
	}

	.stage-preview {
		width: 4rem;
		height: 5rem;
		flex: 0 0 auto;
		overflow: hidden;
		border-radius: 0.5rem 0.5rem 0 0;
		opacity: 0.9;
	}

	.stage-preview--more {
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(156, 163, 175, 0.5);
	}

	.stage-preview__body {
		padding: 0.375rem;
	}

	.stage-preview__title {
		overflow: hidden;
		color: inherit;
		font-size: 0.5rem;
		font-weight: 500;
		line-height: 1;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.stage-preview__lines {
		display: grid;
		gap: 0.25rem;
		margin-top: 0.25rem;
	}

	.stage-preview__line {
		height: 0.375rem;
		border-radius: 999px;
		background: rgba(255, 255, 255, 0.3);
	}

	.stage-preview__line--short {
		width: 75%;
	}

	.stage-preview__more-count {
		color: var(--text-primary, #f0f6fc);
		font-size: 0.75rem;
		line-height: 1rem;
	}

	.template-card__content {
		padding: 1rem;
	}

	.template-meta {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.template-meta__icon {
		width: 1rem;
		height: 1rem;
		color: var(--text-secondary, #8b949e);
	}

	.template-meta__category {
		color: var(--text-tertiary, #6e7681);
		font-size: 0.75rem;
		text-transform: capitalize;
	}

	.template-meta__category {
		color: var(--text-secondary, #8b949e);
	}

	.default-badge {
		border-radius: 0.25rem;
		background: rgba(230, 184, 0, 0.1);
		color: var(--primary-600, #b38f00);
		padding: 0.125rem 0.5rem;
		font-size: 0.75rem;
		line-height: 1rem;
	}

	.default-badge {
		background: rgba(179, 143, 0, 0.3);
		color: var(--primary-400, #ffd11a);
	}

	.template-card__title {
		margin: 0 0 0.5rem;
		color: var(--text-primary, #f0f6fc);
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.5rem;
	}

	.template-card__title {
		color: #ffffff;
	}

	.template-card__description {
		display: -webkit-box;
		min-height: 2.5rem;
		margin: 0 0 1rem;
		overflow: hidden;
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
		line-height: 1.25rem;
		-webkit-box-orient: vertical;
		-webkit-line-clamp: 2;
		line-clamp: 2;
	}

	.template-card__description {
		color: var(--text-secondary, #8b949e);
	}

	.template-stats {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 0.5rem 1rem;
		margin-bottom: 1rem;
		color: var(--text-tertiary, #6e7681);
		font-size: 0.75rem;
	}

	.template-stats {
		color: var(--text-secondary, #8b949e);
	}

	.label-list {
		display: flex;
		flex-wrap: wrap;
		gap: 0.25rem;
		margin-bottom: 1rem;
	}

	.label-pill {
		border-radius: 0.25rem;
		color: #ffffff;
		padding: 0.125rem 0.5rem;
		font-size: 0.75rem;
		line-height: 1rem;
	}

	.label-overflow {
		color: var(--text-tertiary, #6e7681);
		font-size: 0.75rem;
		line-height: 1rem;
	}

	.label-overflow {
		color: var(--text-secondary, #8b949e);
	}

	.use-template-button,
	.custom-template-card__link {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		border: 0;
		border-radius: 0.5rem;
		background: #e6b800;
		color: #0d1117;
		font-weight: 600;
		text-decoration: none;
		transition:
			background-color 160ms ease,
			opacity 160ms ease;
	}

	.use-template-button {
		width: 100%;
		padding: 0.5rem 1rem;
	}

	.use-template-button:hover,
	.custom-template-card__link:hover {
		background: #b38f00;
	}

	.use-template-button:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.use-template-button__icon {
		width: 1rem;
		height: 1rem;
	}

	.button-spinner {
		width: 1rem;
		height: 1rem;
		border: 2px solid rgba(255, 255, 255, 0.35);
		border-bottom-color: var(--border-default, #30363d);
	}

	.custom-template-card {
		margin-top: 2rem;
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 0.75rem;
		background: rgba(230, 184, 0, 0.05);
		padding: 1.5rem;
	}

	.custom-template-card {
		border-color: #b38f00;
		background: rgba(179, 143, 0, 0.2);
	}

	.custom-template-card__layout {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
	}

	.custom-template-card__title {
		margin: 0;
		color: var(--primary-600, #b38f00);
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.5rem;
	}

	.custom-template-card__text {
		margin: 0.25rem 0 0;
		color: rgba(179, 143, 0, 0.8);
		font-size: 0.875rem;
	}

	.custom-template-card__title {
		color: var(--primary-400, #ffd11a);
	}

	.custom-template-card__text {
		color: rgba(255, 209, 26, 0.8);
	}

	.custom-template-card__link {
		flex: 0 0 auto;
		padding: 0.5rem 1rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@media (min-width: 768px) {
		.toolbar {
			flex-direction: row;
		}

		.filter-row {
			padding-bottom: 0;
		}

		.templates-grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}

	@media (min-width: 1024px) {
		.page-container {
			width: min(100% - 3rem, 80rem);
		}

		.templates-grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}

	@media (max-width: 640px) {
		.page-container {
			width: min(100% - 1rem, 80rem);
		}

		.header-title-row {
			align-items: flex-start;
		}

		.title-cluster {
			align-items: flex-start;
		}

		.page-title {
			font-size: 1.25rem;
		}

		.custom-template-card__layout {
			align-items: stretch;
			flex-direction: column;
		}

		.custom-template-card__link {
			width: 100%;
		}
	}
</style>
