<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	interface DashboardTemplate {
		id: string;
		name: string;
		description: string;
		preview: string;
		widgets: Array<{
			widget_type: string;
			title: string;
			position_x: number;
			position_y: number;
			width: number;
			height: number;
		}>;
	}

	const templates: DashboardTemplate[] = [
		{
			id: 'executive',
			name: 'Executive Overview',
			description: 'High-level metrics for executives',
			preview: '📊',
			widgets: [
				{
					widget_type: 'revenue_mrr',
					title: 'Revenue',
					position_x: 0,
					position_y: 0,
					width: 6,
					height: 4
				},
				{
					widget_type: 'user_growth',
					title: 'Users',
					position_x: 6,
					position_y: 0,
					width: 6,
					height: 4
				},
				{
					widget_type: 'subscription_churn',
					title: 'Churn',
					position_x: 0,
					position_y: 4,
					width: 4,
					height: 4
				},
				{
					widget_type: 'crm_pipeline',
					title: 'Pipeline',
					position_x: 4,
					position_y: 4,
					width: 8,
					height: 4
				}
			]
		},
		{
			id: 'marketing',
			name: 'Marketing Dashboard',
			description: 'Track campaigns and conversions',
			preview: '📢',
			widgets: [
				{
					widget_type: 'email_performance',
					title: 'Email',
					position_x: 0,
					position_y: 0,
					width: 6,
					height: 4
				},
				{
					widget_type: 'funnel_conversion',
					title: 'Funnel',
					position_x: 6,
					position_y: 0,
					width: 6,
					height: 6
				},
				{
					widget_type: 'attribution_model',
					title: 'Attribution',
					position_x: 0,
					position_y: 4,
					width: 6,
					height: 6
				},
				{
					widget_type: 'popup_performance',
					title: 'Popups',
					position_x: 0,
					position_y: 10,
					width: 6,
					height: 4
				},
				{
					widget_type: 'form_submissions',
					title: 'Forms',
					position_x: 6,
					position_y: 6,
					width: 6,
					height: 4
				}
			]
		},
		{
			id: 'operations',
			name: 'Operations Dashboard',
			description: 'System health and performance',
			preview: '⚙️',
			widgets: [
				{
					widget_type: 'system_health',
					title: 'System Health',
					position_x: 0,
					position_y: 0,
					width: 8,
					height: 4
				},
				{
					widget_type: 'integration_health',
					title: 'Integrations',
					position_x: 8,
					position_y: 0,
					width: 4,
					height: 4
				},
				{
					widget_type: 'website_speed',
					title: 'Performance',
					position_x: 0,
					position_y: 4,
					width: 6,
					height: 4
				},
				{
					widget_type: 'automation_runs',
					title: 'Automations',
					position_x: 6,
					position_y: 4,
					width: 6,
					height: 4
				},
				{
					widget_type: 'recent_activity',
					title: 'Activity',
					position_x: 0,
					position_y: 8,
					width: 12,
					height: 6
				}
			]
		},
		{
			id: 'trader',
			name: 'Trader Dashboard',
			description: 'Trading performance and alerts',
			preview: '📈',
			widgets: [
				{
					widget_type: 'trading_performance',
					title: 'Performance',
					position_x: 0,
					position_y: 0,
					width: 8,
					height: 6
				},
				{
					widget_type: 'notifications',
					title: 'Alerts',
					position_x: 8,
					position_y: 0,
					width: 4,
					height: 6
				},
				{
					widget_type: 'subscription_status',
					title: 'Subscription',
					position_x: 0,
					position_y: 6,
					width: 6,
					height: 4
				},
				{
					widget_type: 'recent_courses',
					title: 'Courses',
					position_x: 6,
					position_y: 6,
					width: 6,
					height: 4
				}
			]
		},
		{
			id: 'minimal',
			name: 'Minimal Dashboard',
			description: 'Clean and focused',
			preview: '✨',
			widgets: [
				{
					widget_type: 'revenue_mrr',
					title: 'Revenue',
					position_x: 0,
					position_y: 0,
					width: 12,
					height: 6
				},
				{
					widget_type: 'user_growth',
					title: 'Growth',
					position_x: 0,
					position_y: 6,
					width: 6,
					height: 4
				},
				{
					widget_type: 'recent_activity',
					title: 'Activity',
					position_x: 6,
					position_y: 6,
					width: 6,
					height: 4
				}
			]
		}
	];

	function handleSelectTemplate(template: DashboardTemplate) {
		dispatch('select', { template });
	}
</script>

<div class="dashboard-templates">
	<div class="templates-header">
		<h2>Dashboard Templates</h2>
		<p class="subtitle">Start with a pre-built dashboard layout</p>
	</div>

	<div class="templates-grid">
		{#each templates as template}
			<button class="template-card" on:click={() => handleSelectTemplate(template)}>
				<div class="template-preview">
					<span class="preview-icon">{template.preview}</span>
					<div class="widget-count">{template.widgets.length} widgets</div>
				</div>
				<div class="template-info">
					<h3>{template.name}</h3>
					<p>{template.description}</p>
				</div>
				<div class="template-action">
					<span>Use Template</span>
					<svg
						width="20"
						height="20"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M5 12h14M12 5l7 7-7 7" />
					</svg>
				</div>
			</button>
		{/each}
	</div>
</div>

<style>
	.dashboard-templates {
		padding: 1.5rem;
	}

	.templates-header h2 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #1f2937;
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		color: #6b7280;
		margin: 0 0 2rem 0;
	}

	.templates-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 1.5rem;
	}

	.template-card {
		background: white;
		border: 2px solid #e5e7eb;
		border-radius: 12px;
		padding: 1.5rem;
		text-align: left;
		cursor: pointer;
		transition: all 0.3s;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.template-card:hover {
		border-color: #3b82f6;
		box-shadow: 0 8px 24px rgba(59, 130, 246, 0.15);
		transform: translateY(-4px);
	}

	.template-preview {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 2rem;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		border-radius: 8px;
		position: relative;
		overflow: hidden;
	}

	.template-preview::before {
		content: '';
		position: absolute;
		top: -50%;
		right: -50%;
		width: 200%;
		height: 200%;
		background: radial-gradient(circle, rgba(255, 255, 255, 0.1) 0%, transparent 70%);
	}

	.preview-icon {
		font-size: 3rem;
		position: relative;
		z-index: 1;
	}

	.widget-count {
		background: rgba(255, 255, 255, 0.2);
		backdrop-filter: blur(10px);
		padding: 0.25rem 0.75rem;
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 600;
		color: white;
		position: relative;
		z-index: 1;
	}

	.template-info h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 0.5rem 0;
	}

	.template-info p {
		font-size: 0.875rem;
		color: #6b7280;
		margin: 0;
		line-height: 1.5;
	}

	.template-action {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding-top: 1rem;
		border-top: 1px solid #e5e7eb;
		color: #3b82f6;
		font-weight: 600;
		font-size: 0.875rem;
	}

	.template-card:hover .template-action {
		color: #2563eb;
	}

	.template-action svg {
		transition: transform 0.3s;
	}

	.template-card:hover .template-action svg {
		transform: translateX(4px);
	}

	@media (max-width: 768px) {
		.templates-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
