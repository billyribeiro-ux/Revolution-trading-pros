<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { WidgetType } from '$lib/types/dashboard';

	const dispatch = createEventDispatcher();

	interface WidgetTemplate {
		type: WidgetType;
		title: string;
		description: string;
		icon: string;
		defaultSize: { width: number; height: number };
		category: 'analytics' | 'crm' | 'system' | 'user' | 'marketing';
	}

	const widgetTemplates: WidgetTemplate[] = [
		{
			type: 'system_health',
			title: 'System Health',
			description: 'Monitor system status and service health',
			icon: '🏥',
			defaultSize: { width: 6, height: 4 },
			category: 'system'
		},
		{
			type: 'revenue_mrr',
			title: 'Revenue MRR',
			description: 'Monthly recurring revenue metrics',
			icon: '💰',
			defaultSize: { width: 6, height: 4 },
			category: 'analytics'
		},
		{
			type: 'user_growth',
			title: 'User Growth',
			description: 'Track user acquisition and growth',
			icon: '📈',
			defaultSize: { width: 4, height: 4 },
			category: 'analytics'
		},
		{
			type: 'subscription_churn',
			title: 'Churn Rate',
			description: 'Monitor subscription churn',
			icon: '📉',
			defaultSize: { width: 4, height: 4 },
			category: 'analytics'
		},
		{
			type: 'email_performance',
			title: 'Email Performance',
			description: 'Email campaign metrics',
			icon: '📧',
			defaultSize: { width: 4, height: 4 },
			category: 'marketing'
		},
		{
			type: 'crm_pipeline',
			title: 'CRM Pipeline',
			description: 'Sales pipeline overview',
			icon: '🎯',
			defaultSize: { width: 4, height: 4 },
			category: 'crm'
		},
		{
			type: 'recent_activity',
			title: 'Recent Activity',
			description: 'Activity timeline',
			icon: '📋',
			defaultSize: { width: 4, height: 8 },
			category: 'system'
		},
		{
			type: 'funnel_conversion',
			title: 'Funnel Conversion',
			description: 'Conversion funnel analysis',
			icon: '🔄',
			defaultSize: { width: 6, height: 6 },
			category: 'marketing'
		},
		{
			type: 'trading_performance',
			title: 'Trading Performance',
			description: 'Trading statistics and metrics',
			icon: '📊',
			defaultSize: { width: 8, height: 6 },
			category: 'user'
		},
		{
			type: 'notifications',
			title: 'Notifications',
			description: 'User notifications feed',
			icon: '🔔',
			defaultSize: { width: 4, height: 6 },
			category: 'user'
		}
	];

	let selectedCategory: string = 'all';
	let searchQuery = '';

	function handleAddWidget(template: WidgetTemplate) {
		dispatch('add', {
			widget_type: template.type,
			title: template.title,
			width: template.defaultSize.width,
			height: template.defaultSize.height,
			position_x: 0,
			position_y: 0,
			config: {}
		});
	}

	$: filteredWidgets = widgetTemplates.filter((widget) => {
		const matchesCategory = selectedCategory === 'all' || widget.category === selectedCategory;
		const matchesSearch =
			widget.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
			widget.description.toLowerCase().includes(searchQuery.toLowerCase());
		return matchesCategory && matchesSearch;
	});

	const categories = [
		{ id: 'all', label: 'All Widgets', icon: '📦' },
		{ id: 'analytics', label: 'Analytics', icon: '📊' },
		{ id: 'crm', label: 'CRM', icon: '🎯' },
		{ id: 'system', label: 'System', icon: '⚙️' },
		{ id: 'user', label: 'User', icon: '👤' },
		{ id: 'marketing', label: 'Marketing', icon: '📢' }
	];
</script>

<div class="widget-library">
	<div class="library-header">
		<h2>Widget Library</h2>
		<p class="subtitle">Add widgets to your dashboard</p>
	</div>

	<div class="search-bar">
		<svg
			width="20"
			height="20"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
		>
			<circle cx="11" cy="11" r="8" /><path d="m21 21-4.35-4.35" />
		</svg>
		<input type="text" placeholder="Search widgets..." bind:value={searchQuery} />
	</div>

	<div class="category-tabs">
		{#each categories as category}
			<button
				class="category-tab"
				class:active={selectedCategory === category.id}
				on:click={() => (selectedCategory = category.id)}
			>
				<span class="category-icon">{category.icon}</span>
				<span class="category-label">{category.label}</span>
			</button>
		{/each}
	</div>

	<div class="widgets-grid">
		{#each filteredWidgets as template}
			<div class="widget-template">
				<div class="template-icon">{template.icon}</div>
				<div class="template-info">
					<h3>{template.title}</h3>
					<p>{template.description}</p>
					<div class="template-meta">
						<span class="size-badge"
							>{template.defaultSize.width}×{template.defaultSize.height}</span
						>
						<span class="category-badge">{template.category}</span>
					</div>
				</div>
				<button
					class="add-btn"
					on:click={() => handleAddWidget(template)}
					aria-label="Add {template.title} widget"
				>
					<svg
						width="20"
						height="20"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M12 5v14M5 12h14" />
					</svg>
					Add
				</button>
			</div>
		{/each}
	</div>

	{#if filteredWidgets.length === 0}
		<div class="no-results">
			<p>No widgets found matching your search.</p>
		</div>
	{/if}
</div>

<style>
	.widget-library {
		background: white;
		border-radius: 12px;
		padding: 1.5rem;
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
		max-height: 80vh;
		overflow-y: auto;
	}

	.library-header h2 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #1f2937;
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		color: #6b7280;
		margin: 0 0 1.5rem 0;
	}

	.search-bar {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		margin-bottom: 1.5rem;
	}

	.search-bar svg {
		color: #9ca3af;
		flex-shrink: 0;
	}

	.search-bar input {
		flex: 1;
		border: none;
		background: none;
		outline: none;
		font-size: 0.95rem;
		color: #1f2937;
	}

	.category-tabs {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 1.5rem;
		overflow-x: auto;
		padding-bottom: 0.5rem;
	}

	.category-tab {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
		white-space: nowrap;
		font-size: 0.875rem;
		font-weight: 500;
		color: #6b7280;
	}

	.category-tab:hover {
		background: #f3f4f6;
		border-color: #d1d5db;
	}

	.category-tab.active {
		background: #3b82f6;
		border-color: #3b82f6;
		color: white;
	}

	.category-icon {
		font-size: 1.125rem;
	}

	.widgets-grid {
		display: grid;
		gap: 1rem;
	}

	.widget-template {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		background: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		transition: all 0.2s;
	}

	.widget-template:hover {
		background: white;
		border-color: #3b82f6;
		box-shadow: 0 2px 8px rgba(59, 130, 246, 0.1);
	}

	.template-icon {
		font-size: 2rem;
		flex-shrink: 0;
	}

	.template-info {
		flex: 1;
		min-width: 0;
	}

	.template-info h3 {
		font-size: 0.95rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 0.25rem 0;
	}

	.template-info p {
		font-size: 0.8rem;
		color: #6b7280;
		margin: 0 0 0.5rem 0;
	}

	.template-meta {
		display: flex;
		gap: 0.5rem;
	}

	.size-badge,
	.category-badge {
		font-size: 0.7rem;
		padding: 0.125rem 0.5rem;
		border-radius: 4px;
		font-weight: 500;
	}

	.size-badge {
		background: #dbeafe;
		color: #1e40af;
	}

	.category-badge {
		background: #f3e8ff;
		color: #6b21a8;
		text-transform: capitalize;
	}

	.add-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		font-weight: 500;
		transition: all 0.2s;
		flex-shrink: 0;
	}

	.add-btn:hover {
		background: #2563eb;
		transform: translateY(-1px);
		box-shadow: 0 4px 8px rgba(59, 130, 246, 0.3);
	}

	.no-results {
		text-align: center;
		padding: 3rem 1rem;
		color: #6b7280;
	}
</style>
