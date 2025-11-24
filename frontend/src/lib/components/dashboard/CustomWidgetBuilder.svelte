<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { WidgetType } from '$lib/types/dashboard';

	const dispatch = createEventDispatcher();

	let widgetConfig = {
		widget_type: 'system_health' as WidgetType,
		title: '',
		width: 6,
		height: 4,
		position_x: 0,
		position_y: 0,
		refresh_interval: 300,
		config: {} as Record<string, any>
	};

	let configFields: Array<{
		key: string;
		type: 'text' | 'number' | 'boolean' | 'select' | 'array';
		label: string;
		value: any;
		options?: string[];
	}> = [];

	const widgetTypes = [
		{ value: 'system_health', label: 'System Health' },
		{ value: 'revenue_mrr', label: 'Revenue MRR' },
		{ value: 'user_growth', label: 'User Growth' },
		{ value: 'subscription_churn', label: 'Subscription Churn' },
		{ value: 'email_performance', label: 'Email Performance' },
		{ value: 'crm_pipeline', label: 'CRM Pipeline' },
		{ value: 'recent_activity', label: 'Recent Activity' },
		{ value: 'trading_performance', label: 'Trading Performance' },
		{ value: 'notifications', label: 'Notifications' },
		{ value: 'funnel_conversion', label: 'Funnel Conversion' },
		{ value: 'behavior_friction', label: 'Behavior Friction' },
		{ value: 'attribution_model', label: 'Attribution Model' }
	];

	function updateConfigFields() {
		// Define config fields based on widget type
		switch (widgetConfig.widget_type) {
			case 'system_health':
				configFields = [
					{ key: 'show_all_services', type: 'boolean', label: 'Show All Services', value: true },
					{ key: 'show_metrics', type: 'boolean', label: 'Show Metrics', value: true },
					{ key: 'services_filter', type: 'array', label: 'Filter Services', value: [] }
				];
				break;
			case 'revenue_mrr':
				configFields = [
					{ key: 'currency', type: 'text', label: 'Currency Symbol', value: '$' },
					{
						key: 'format',
						type: 'select',
						label: 'Number Format',
						value: 'full',
						options: ['compact', 'full']
					},
					{ key: 'show_chart', type: 'boolean', label: 'Show Chart', value: true },
					{ key: 'show_growth', type: 'boolean', label: 'Show Growth', value: true }
				];
				break;
			case 'user_growth':
				configFields = [
					{ key: 'show_total', type: 'boolean', label: 'Show Total Users', value: true },
					{ key: 'show_growth', type: 'boolean', label: 'Show Growth', value: true },
					{
						key: 'highlight_threshold',
						type: 'number',
						label: 'Highlight Threshold (%)',
						value: 20
					},
					{
						key: 'format',
						type: 'select',
						label: 'Number Format',
						value: 'full',
						options: ['compact', 'full']
					}
				];
				break;
			case 'recent_activity':
				configFields = [
					{ key: 'limit', type: 'number', label: 'Max Activities', value: 20 },
					{ key: 'show_user', type: 'boolean', label: 'Show User Names', value: true },
					{ key: 'show_time', type: 'boolean', label: 'Show Timestamps', value: true }
				];
				break;
			default:
				configFields = [];
		}

		// Build config object
		widgetConfig.config = {};
		configFields.forEach((field) => {
			widgetConfig.config[field.key] = field.value;
		});
	}

	function handleWidgetTypeChange() {
		updateConfigFields();
	}

	function handleConfigChange(key: string, value: any) {
		widgetConfig.config[key] = value;
		const field = configFields.find((f) => f.key === key);
		if (field) {
			field.value = value;
		}
	}

	function handleCreate() {
		if (!widgetConfig.title) {
			alert('Please enter a widget title');
			return;
		}

		dispatch('create', widgetConfig);
	}

	function handlePreview() {
		dispatch('preview', widgetConfig);
	}

	// Initialize
	updateConfigFields();
</script>

<div class="widget-builder">
	<div class="builder-header">
		<h2>Custom Widget Builder</h2>
		<p class="subtitle">Create a custom widget with your preferred configuration</p>
	</div>

	<div class="builder-form">
		<div class="form-section">
			<h3>Basic Settings</h3>

			<div class="form-group">
				<label for="widget-type">Widget Type</label>
				<select
					id="widget-type"
					bind:value={widgetConfig.widget_type}
					on:change={handleWidgetTypeChange}
				>
					{#each widgetTypes as type}
						<option value={type.value}>{type.label}</option>
					{/each}
				</select>
			</div>

			<div class="form-group">
				<label for="widget-title">Widget Title</label>
				<input
					id="widget-title"
					type="text"
					placeholder="Enter widget title"
					bind:value={widgetConfig.title}
				/>
			</div>

			<div class="form-row">
				<div class="form-group">
					<label for="widget-width">Width (columns)</label>
					<input id="widget-width" type="number" min="2" max="12" bind:value={widgetConfig.width} />
				</div>

				<div class="form-group">
					<label for="widget-height">Height (rows)</label>
					<input
						id="widget-height"
						type="number"
						min="2"
						max="12"
						bind:value={widgetConfig.height}
					/>
				</div>
			</div>

			<div class="form-group">
				<label for="refresh-interval">Refresh Interval (seconds)</label>
				<input
					id="refresh-interval"
					type="number"
					min="60"
					max="3600"
					step="60"
					bind:value={widgetConfig.refresh_interval}
				/>
			</div>
		</div>

		{#if configFields.length > 0}
			<div class="form-section">
				<h3>Widget Configuration</h3>

				{#each configFields as field}
					<div class="form-group">
						<label for={field.key}>{field.label}</label>

						{#if field.type === 'text'}
							<input
								id={field.key}
								type="text"
								bind:value={field.value}
								on:change={() => handleConfigChange(field.key, field.value)}
							/>
						{:else if field.type === 'number'}
							<input
								id={field.key}
								type="number"
								bind:value={field.value}
								on:change={() => handleConfigChange(field.key, field.value)}
							/>
						{:else if field.type === 'boolean'}
							<label class="checkbox-label">
								<input
									type="checkbox"
									bind:checked={field.value}
									on:change={() => handleConfigChange(field.key, field.value)}
								/>
								<span>Enable</span>
							</label>
						{:else if field.type === 'select' && field.options}
							<select
								id={field.key}
								bind:value={field.value}
								on:change={() => handleConfigChange(field.key, field.value)}
							>
								{#each field.options as option}
									<option value={option}>{option}</option>
								{/each}
							</select>
						{/if}
					</div>
				{/each}
			</div>
		{/if}

		<div class="form-actions">
			<button class="btn-secondary" on:click={handlePreview}> Preview Widget </button>
			<button class="btn-primary" on:click={handleCreate}> Create Widget </button>
		</div>
	</div>
</div>

<style>
	.widget-builder {
		background: white;
		border-radius: 12px;
		padding: 2rem;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		max-width: 600px;
		margin: 0 auto;
	}

	.builder-header {
		margin-bottom: 2rem;
	}

	h2 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #1f2937;
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		color: #6b7280;
		font-size: 0.875rem;
		margin: 0;
	}

	.form-section {
		margin-bottom: 2rem;
		padding-bottom: 2rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.form-section:last-of-type {
		border-bottom: none;
	}

	h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #374151;
		margin: 0 0 1.5rem 0;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
		margin-bottom: 0.5rem;
	}

	input[type='text'],
	input[type='number'],
	select {
		width: 100%;
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		font-size: 0.875rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
	}

	.form-actions {
		display: flex;
		gap: 1rem;
		margin-top: 2rem;
	}

	.btn-primary,
	.btn-secondary {
		flex: 1;
		padding: 0.75rem 1.5rem;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary {
		background: #3b82f6;
		color: white;
	}

	.btn-primary:hover {
		background: #2563eb;
	}

	.btn-secondary {
		background: #f3f4f6;
		color: #374151;
	}

	.btn-secondary:hover {
		background: #e5e7eb;
	}
</style>
