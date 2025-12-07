<script lang="ts">
	/**
	 * Dashboard Widget Manager - Apple ICT9+ Design
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Modal for customizing dashboard widgets with:
	 * - Drag and drop reordering
	 * - Toggle visibility
	 * - Size configuration
	 * - Category filtering
	 *
	 * @version 1.0.0
	 */

	import { fade, fly, scale } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import { flip } from 'svelte/animate';
	import {
		widgetStore,
		visibleWidgets,
		hiddenWidgets,
		widgetsByCategory,
		widgetLayout,
		autoRefreshEnabled,
		type DashboardWidget,
		type WidgetSize
	} from '$lib/stores/widgets';
	import {
		IconX,
		IconGripVertical,
		IconEye,
		IconEyeOff,
		IconLayoutGrid,
		IconList,
		IconRefresh,
		IconChevronUp,
		IconChevronDown,
		IconChartLine,
		IconWorld,
		IconFileText,
		IconUsers,
		IconCurrencyDollar,
		IconShoppingCart,
		IconPlugConnected,
		IconActivity,
		IconBolt,
		IconMail,
		IconSearch,
		IconEyeCheck,
		IconSettings
	} from '@tabler/icons-svelte';

	interface Props {
		isOpen?: boolean;
		onclose?: () => void;
	}

	let { isOpen = $bindable(false), onclose }: Props = $props();

	let activeTab = $state<'visible' | 'hidden' | 'settings'>('visible');
	let draggedWidget = $state<DashboardWidget | null>(null);
	let dragOverIndex = $state<number | null>(null);

	const iconMap: Record<string, typeof IconChartLine> = {
		'chart-line': IconChartLine,
		'world': IconWorld,
		'article': IconFileText,
		'users': IconUsers,
		'currency-dollar': IconCurrencyDollar,
		'shopping-cart': IconShoppingCart,
		'plug': IconPlugConnected,
		'heart-rate-monitor': IconActivity,
		'bolt': IconBolt,
		'mail': IconMail,
		'search': IconSearch,
		'eye': IconEyeCheck
	};

	const sizeLabels: Record<WidgetSize, string> = {
		small: 'S',
		medium: 'M',
		large: 'L',
		full: 'Full'
	};

	const categoryLabels: Record<string, string> = {
		analytics: 'Analytics',
		content: 'Content',
		commerce: 'Commerce',
		system: 'System'
	};

	const categoryColors: Record<string, string> = {
		analytics: '#3b82f6',
		content: '#10b981',
		commerce: '#f59e0b',
		system: '#8b5cf6'
	};

	function close() {
		isOpen = false;
		onclose?.();
	}

	function handleDragStart(widget: DashboardWidget) {
		draggedWidget = widget;
	}

	function handleDragOver(index: number) {
		if (draggedWidget) {
			dragOverIndex = index;
		}
	}

	function handleDragEnd() {
		if (draggedWidget && dragOverIndex !== null) {
			const fromIndex = $visibleWidgets.findIndex(w => w.id === draggedWidget!.id);
			if (fromIndex !== -1 && fromIndex !== dragOverIndex) {
				widgetStore.reorderWidgets(fromIndex, dragOverIndex);
			}
		}
		draggedWidget = null;
		dragOverIndex = null;
	}

	function cycleSizeNext(widget: DashboardWidget) {
		const sizes: WidgetSize[] = ['small', 'medium', 'large', 'full'];
		const currentIndex = sizes.indexOf(widget.size);
		const nextIndex = (currentIndex + 1) % sizes.length;
		widgetStore.setWidgetSize(widget.id, sizes[nextIndex]);
	}
</script>

{#if isOpen}
	<div
		class="widget-manager-overlay"
		transition:fade={{ duration: 150 }}
		onclick={close}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<div
			class="widget-manager"
			transition:scale={{ duration: 300, start: 0.95, easing: quintOut }}
			onclick={(e) => e.stopPropagation()}
			role="document"
		>
			<!-- Header -->
			<div class="manager-header">
				<div class="header-left">
					<IconSettings size={22} />
					<h2>Customize Dashboard</h2>
				</div>
				<button class="close-btn" onclick={close}>
					<IconX size={20} />
				</button>
			</div>

			<!-- Tabs -->
			<div class="manager-tabs">
				<button
					class="tab"
					class:active={activeTab === 'visible'}
					onclick={() => activeTab = 'visible'}
				>
					<IconEye size={16} />
					Visible ({$visibleWidgets.length})
				</button>
				<button
					class="tab"
					class:active={activeTab === 'hidden'}
					onclick={() => activeTab = 'hidden'}
				>
					<IconEyeOff size={16} />
					Hidden ({$hiddenWidgets.length})
				</button>
				<button
					class="tab"
					class:active={activeTab === 'settings'}
					onclick={() => activeTab = 'settings'}
				>
					<IconSettings size={16} />
					Settings
				</button>
			</div>

			<!-- Content -->
			<div class="manager-content">
				{#if activeTab === 'visible'}
					<div class="widget-list" in:fade={{ duration: 150 }}>
						<p class="helper-text">Drag to reorder widgets. Click the size button to cycle sizes.</p>
						{#each $visibleWidgets as widget, index (widget.id)}
							<div
								class="widget-item"
								class:dragging={draggedWidget?.id === widget.id}
								class:drag-over={dragOverIndex === index}
								draggable="true"
								ondragstart={() => handleDragStart(widget)}
								ondragover={() => handleDragOver(index)}
								ondragend={handleDragEnd}
								animate:flip={{ duration: 200 }}
							>
								<div class="drag-handle">
									<IconGripVertical size={18} />
								</div>
								<div
									class="widget-icon"
									style="background: {categoryColors[widget.category]}20; color: {categoryColors[widget.category]}"
								>
									<svelte:component this={iconMap[widget.icon] || IconChartLine} size={18} />
								</div>
								<div class="widget-info">
									<span class="widget-title">{widget.title}</span>
									<span class="widget-category">{categoryLabels[widget.category]}</span>
								</div>
								<div class="widget-actions">
									<button
										class="size-btn"
										onclick={() => cycleSizeNext(widget)}
										title="Size: {widget.size}"
									>
										{sizeLabels[widget.size]}
									</button>
									<button
										class="move-btn"
										onclick={() => widgetStore.moveWidget(widget.id, 'up')}
										disabled={index === 0}
									>
										<IconChevronUp size={16} />
									</button>
									<button
										class="move-btn"
										onclick={() => widgetStore.moveWidget(widget.id, 'down')}
										disabled={index === $visibleWidgets.length - 1}
									>
										<IconChevronDown size={16} />
									</button>
									<button
										class="hide-btn"
										onclick={() => widgetStore.toggleWidget(widget.id)}
										title="Hide widget"
									>
										<IconEyeOff size={16} />
									</button>
								</div>
							</div>
						{/each}
					</div>
				{:else if activeTab === 'hidden'}
					<div class="widget-list" in:fade={{ duration: 150 }}>
						{#if $hiddenWidgets.length === 0}
							<div class="empty-state">
								<IconEye size={48} />
								<h3>All widgets visible</h3>
								<p>No hidden widgets</p>
							</div>
						{:else}
							<p class="helper-text">Click the eye icon to show a widget on your dashboard.</p>
							{#each $hiddenWidgets as widget (widget.id)}
								<div class="widget-item hidden-item">
									<div
										class="widget-icon"
										style="background: {categoryColors[widget.category]}20; color: {categoryColors[widget.category]}"
									>
										<svelte:component this={iconMap[widget.icon] || IconChartLine} size={18} />
									</div>
									<div class="widget-info">
										<span class="widget-title">{widget.title}</span>
										<span class="widget-description">{widget.description}</span>
									</div>
									<button
										class="show-btn"
										onclick={() => widgetStore.toggleWidget(widget.id)}
										title="Show widget"
									>
										<IconEye size={16} />
										Show
									</button>
								</div>
							{/each}
						{/if}
					</div>
				{:else if activeTab === 'settings'}
					<div class="settings-panel" in:fade={{ duration: 150 }}>
						<!-- Layout -->
						<div class="setting-group">
							<label class="setting-label">Layout</label>
							<div class="layout-options">
								<button
									class="layout-btn"
									class:active={$widgetLayout === 'grid'}
									onclick={() => widgetStore.setLayout('grid')}
								>
									<IconLayoutGrid size={18} />
									Grid
								</button>
								<button
									class="layout-btn"
									class:active={$widgetLayout === 'list'}
									onclick={() => widgetStore.setLayout('list')}
								>
									<IconList size={18} />
									List
								</button>
							</div>
						</div>

						<!-- Auto Refresh -->
						<div class="setting-group">
							<label class="setting-label">Auto Refresh</label>
							<div class="toggle-row">
								<span class="toggle-description">
									Automatically refresh widget data
								</span>
								<button
									class="toggle-switch"
									class:active={$autoRefreshEnabled}
									onclick={() => widgetStore.toggleAutoRefresh()}
								>
									<span class="toggle-knob"></span>
								</button>
							</div>
						</div>

						<!-- Reset -->
						<div class="setting-group">
							<label class="setting-label">Reset</label>
							<button class="reset-btn" onclick={() => widgetStore.resetToDefaults()}>
								<IconRefresh size={16} />
								Reset to Defaults
							</button>
						</div>
					</div>
				{/if}
			</div>

			<!-- Footer -->
			<div class="manager-footer">
				<button class="done-btn" onclick={close}>
					Done
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.widget-manager-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(8px);
		z-index: 9999;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2rem;
	}

	.widget-manager {
		width: 100%;
		max-width: 600px;
		max-height: 80vh;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.98), rgba(15, 23, 42, 0.98));
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 20px;
		box-shadow:
			0 25px 80px rgba(0, 0, 0, 0.5),
			0 0 0 1px rgba(255, 255, 255, 0.05) inset;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.manager-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		color: #f1f5f9;
	}

	.header-left h2 {
		font-size: 1.125rem;
		font-weight: 600;
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(99, 102, 241, 0.1);
		border: none;
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.close-btn:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #f1f5f9;
	}

	.manager-tabs {
		display: flex;
		padding: 0 1.5rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1rem;
		background: none;
		border: none;
		border-bottom: 2px solid transparent;
		color: #64748b;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.tab:hover {
		color: #94a3b8;
	}

	.tab.active {
		color: #a5b4fc;
		border-bottom-color: #6366f1;
	}

	.manager-content {
		flex: 1;
		overflow-y: auto;
		padding: 1.25rem 1.5rem;
	}

	.manager-content::-webkit-scrollbar {
		width: 6px;
	}

	.manager-content::-webkit-scrollbar-track {
		background: transparent;
	}

	.manager-content::-webkit-scrollbar-thumb {
		background: rgba(99, 102, 241, 0.3);
		border-radius: 3px;
	}

	.helper-text {
		font-size: 0.8125rem;
		color: #64748b;
		margin: 0 0 1rem;
	}

	.widget-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.widget-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 12px;
		transition: all 0.2s;
	}

	.widget-item:hover {
		background: rgba(99, 102, 241, 0.1);
	}

	.widget-item.dragging {
		opacity: 0.5;
		transform: scale(0.98);
	}

	.widget-item.drag-over {
		border-color: #6366f1;
		background: rgba(99, 102, 241, 0.15);
	}

	.drag-handle {
		color: #64748b;
		cursor: grab;
		padding: 0.25rem;
	}

	.drag-handle:active {
		cursor: grabbing;
	}

	.widget-icon {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 10px;
		flex-shrink: 0;
	}

	.widget-info {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.widget-title {
		font-size: 0.875rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.widget-category,
	.widget-description {
		font-size: 0.75rem;
		color: #64748b;
	}

	.widget-actions {
		display: flex;
		align-items: center;
		gap: 0.375rem;
	}

	.size-btn {
		padding: 0.375rem 0.625rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 6px;
		color: #a5b4fc;
		font-size: 0.75rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.size-btn:hover {
		background: rgba(99, 102, 241, 0.2);
	}

	.move-btn,
	.hide-btn {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(99, 102, 241, 0.1);
		border: none;
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.move-btn:hover:not(:disabled),
	.hide-btn:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #a5b4fc;
	}

	.move-btn:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}

	.hide-btn:hover {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}

	.hidden-item {
		opacity: 0.7;
	}

	.show-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.75rem;
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.2);
		border-radius: 8px;
		color: #34d399;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.show-btn:hover {
		background: rgba(16, 185, 129, 0.2);
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 3rem;
		color: #64748b;
		text-align: center;
	}

	.empty-state h3 {
		margin: 1rem 0 0.25rem;
		color: #f1f5f9;
	}

	.empty-state p {
		margin: 0;
	}

	.settings-panel {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.setting-group {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.setting-label {
		font-size: 0.875rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.layout-options {
		display: flex;
		gap: 0.5rem;
	}

	.layout-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.layout-btn:hover {
		background: rgba(99, 102, 241, 0.15);
	}

	.layout-btn.active {
		background: rgba(99, 102, 241, 0.2);
		border-color: rgba(99, 102, 241, 0.4);
		color: #a5b4fc;
	}

	.toggle-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem;
		background: rgba(99, 102, 241, 0.05);
		border-radius: 10px;
	}

	.toggle-description {
		font-size: 0.8125rem;
		color: #94a3b8;
	}

	.toggle-switch {
		width: 44px;
		height: 24px;
		background: rgba(99, 102, 241, 0.2);
		border: none;
		border-radius: 12px;
		cursor: pointer;
		position: relative;
		transition: all 0.2s;
	}

	.toggle-switch.active {
		background: #6366f1;
	}

	.toggle-knob {
		position: absolute;
		top: 2px;
		left: 2px;
		width: 20px;
		height: 20px;
		background: white;
		border-radius: 50%;
		transition: transform 0.2s;
	}

	.toggle-switch.active .toggle-knob {
		transform: translateX(20px);
	}

	.reset-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.2);
		border-radius: 10px;
		color: #f87171;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		width: fit-content;
	}

	.reset-btn:hover {
		background: rgba(239, 68, 68, 0.2);
	}

	.manager-footer {
		padding: 1rem 1.5rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
		display: flex;
		justify-content: flex-end;
	}

	.done-btn {
		padding: 0.75rem 2rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		border: none;
		border-radius: 10px;
		color: white;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.done-btn:hover {
		transform: translateY(-1px);
		box-shadow: 0 8px 25px rgba(99, 102, 241, 0.3);
	}

	@media (max-width: 640px) {
		.widget-manager-overlay {
			padding: 1rem;
		}

		.widget-manager {
			max-height: 90vh;
		}

		.widget-item {
			flex-wrap: wrap;
		}

		.widget-actions {
			width: 100%;
			justify-content: flex-end;
			margin-top: 0.5rem;
			padding-left: 3.5rem;
		}
	}
</style>
