<script lang="ts">
	/**
	 * Component Palette - Drag Source Sidebar
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 * 
	 * Beautiful sidebar with draggable component blocks organized by category.
	 */

	import { componentRegistry, getComponentsByCategory } from '../registry';
	import type { BuilderStore } from '../store.svelte';
	import type { ComponentType } from '../types';

	interface Props {
		store: BuilderStore;
	}

	let { store }: Props = $props();

	const contentComponents = getComponentsByCategory('content');
	const mediaComponents = getComponentsByCategory('media');
	const layoutComponents = getComponentsByCategory('layout');

	let draggedType = $state<ComponentType | null>(null);

	function handleDragStart(e: DragEvent, type: ComponentType) {
		draggedType = type;
		store.startDragNew(type);
		
		if (e.dataTransfer) {
			e.dataTransfer.effectAllowed = 'copy';
			e.dataTransfer.setData('text/plain', type);
		}
	}

	function handleDragEnd() {
		draggedType = null;
		store.cancelDrag();
	}
</script>

<aside class="palette">
	<div class="palette-header">
		<h2>Components</h2>
		<span class="component-count">{componentRegistry.length}</span>
	</div>

	<div class="palette-content">
		<!-- Content Components -->
		<section class="category">
			<h3 class="category-title">
				<span class="category-icon">📝</span>
				Content
			</h3>
			<div class="component-grid">
				{#each contentComponents as component}
					<button
						class="component-card"
						class:dragging={draggedType === component.type}
						draggable="true"
						ondragstart={(e) => handleDragStart(e, component.type)}
						ondragend={handleDragEnd}
						onclick={() => store.addBlock(component.type)}
					>
						<span class="component-icon">{component.icon}</span>
						<span class="component-name">{component.name}</span>
					</button>
				{/each}
			</div>
		</section>

		<!-- Media Components -->
		<section class="category">
			<h3 class="category-title">
				<span class="category-icon">🎥</span>
				Media
			</h3>
			<div class="component-grid">
				{#each mediaComponents as component}
					<button
						class="component-card"
						class:dragging={draggedType === component.type}
						draggable="true"
						ondragstart={(e) => handleDragStart(e, component.type)}
						ondragend={handleDragEnd}
						onclick={() => store.addBlock(component.type)}
					>
						<span class="component-icon">{component.icon}</span>
						<span class="component-name">{component.name}</span>
					</button>
				{/each}
			</div>
		</section>

		<!-- Layout Components -->
		<section class="category">
			<h3 class="category-title">
				<span class="category-icon">📐</span>
				Layout
			</h3>
			<div class="component-grid">
				{#each layoutComponents as component}
					<button
						class="component-card"
						class:dragging={draggedType === component.type}
						draggable="true"
						ondragstart={(e) => handleDragStart(e, component.type)}
						ondragend={handleDragEnd}
						onclick={() => store.addBlock(component.type)}
					>
						<span class="component-icon">{component.icon}</span>
						<span class="component-name">{component.name}</span>
					</button>
				{/each}
			</div>
		</section>
	</div>

	<div class="palette-footer">
		<p class="tip">💡 Drag or click to add</p>
	</div>
</aside>

<style>
	.palette {
		width: 260px;
		height: 100%;
		background: #FFFFFF;
		border-right: 1px solid #E5E7EB;
		display: flex;
		flex-direction: column;
		flex-shrink: 0;
	}

	.palette-header {
		padding: 20px;
		border-bottom: 1px solid #E5E7EB;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.palette-header h2 {
		font-size: 16px;
		font-weight: 600;
		color: #1F2937;
		margin: 0;
	}

	.component-count {
		background: #143E59;
		color: white;
		font-size: 11px;
		font-weight: 600;
		padding: 2px 8px;
		border-radius: 10px;
	}

	.palette-content {
		flex: 1;
		overflow-y: auto;
		padding: 16px;
	}

	.category {
		margin-bottom: 24px;
	}

	.category:last-child {
		margin-bottom: 0;
	}

	.category-title {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 12px;
		font-weight: 600;
		color: #6B7280;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		margin: 0 0 12px 0;
	}

	.category-icon {
		font-size: 14px;
	}

	.component-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 8px;
	}

	.component-card {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 6px;
		padding: 16px 8px;
		background: #F9FAFB;
		border: 1px solid #E5E7EB;
		border-radius: 10px;
		cursor: grab;
		transition: all 0.15s ease;
	}

	.component-card:hover {
		background: #F3F4F6;
		border-color: #143E59;
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.15);
	}

	.component-card:active {
		cursor: grabbing;
		transform: translateY(0);
	}

	.component-card.dragging {
		opacity: 0.5;
		transform: scale(0.95);
	}

	.component-icon {
		font-size: 24px;
	}

	.component-name {
		font-size: 11px;
		font-weight: 500;
		color: #374151;
		text-align: center;
		line-height: 1.2;
	}

	.palette-footer {
		padding: 16px;
		border-top: 1px solid #E5E7EB;
		background: #F9FAFB;
	}

	.tip {
		font-size: 12px;
		color: #6B7280;
		margin: 0;
		text-align: center;
	}

	/* Scrollbar styling */
	.palette-content::-webkit-scrollbar {
		width: 6px;
	}

	.palette-content::-webkit-scrollbar-track {
		background: transparent;
	}

	.palette-content::-webkit-scrollbar-thumb {
		background: #D1D5DB;
		border-radius: 3px;
	}

	.palette-content::-webkit-scrollbar-thumb:hover {
		background: #9CA3AF;
	}
</style>
