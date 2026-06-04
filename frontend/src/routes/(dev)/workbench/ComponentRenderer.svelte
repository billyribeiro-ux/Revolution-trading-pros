<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * Component Renderer - Dynamic Component Loader with Error Boundary
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Dynamically loads and renders components with error handling
	 * @version 1.0.0 - January 2026
	 * @standards Apple Principal Engineer ICT Level 7+
	 */
	import type { ComponentInfo } from './+page.server';
	import type { Component } from 'svelte';
	import { logger } from '$lib/utils/logger';

	interface Props {
		component: ComponentInfo | null;
		propValues: Record<string, unknown>;
		snippetContent: string;
		viewport: { width: number; height: number | null };
		background: string;
		showBorder: boolean;
		padding: number;
		zoom: number;
	}

	let {
		component,
		propValues,
		snippetContent: _snippetContent,
		viewport,
		background,
		showBorder,
		padding,
		zoom
	}: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let retryNonce = $state(0);

	// ═══════════════════════════════════════════════════════════════════════════
	// COMPONENT LOADING
	// ═══════════════════════════════════════════════════════════════════════════

	// Map of preview-safe components for dynamic import.
	const componentModules = import.meta.glob([
		'/src/lib/components/**/*.svelte',
		'!/src/lib/components/auth/Scene3D.svelte',
		'!/src/lib/components/auth/TradingScene3D.svelte'
	]);

	const componentLoad = $derived.by<Promise<{ component: Component; renderTime: number }> | null>(
		() => {
			if (!component) return null;
			void retryNonce;

			const startTime = performance.now();
			const importPath = `/src/lib/components/${component.relativePath}`;
			const moduleLoader = componentModules[importPath];

			if (!moduleLoader) {
				return Promise.reject(new Error(`Component not found: ${importPath}`));
			}

			return moduleLoader()
				.then((module) => ({
					component: (module as { default: Component }).default,
					renderTime: Math.round(performance.now() - startTime)
				}))
				.catch((e) => {
					logger.error('[Workbench] Load error:', e);
					throw e;
				});
		}
	);

	// ═══════════════════════════════════════════════════════════════════════════
	// COMPUTED STYLES
	// ═══════════════════════════════════════════════════════════════════════════

	const containerStyle = $derived(`
		width: ${viewport.width}px;
		${viewport.height ? `height: ${viewport.height}px;` : ''}
		background: ${background};
		padding: ${padding}px;
		transform: scale(${zoom / 100});
		transform-origin: top left;
		${showBorder ? 'outline: 1px dashed rgba(139, 92, 246, 0.5);' : ''}
	`);

	const wrapperStyle = $derived(`
		width: ${(viewport.width * zoom) / 100}px;
		${viewport.height ? `height: ${((viewport.height + padding * 2) * zoom) / 100}px;` : ''}
	`);
</script>

<div class="renderer">
	{#if !component}
		<div class="empty-state">
			<span class="empty-icon">📦</span>
			<h3>Select a Component</h3>
			<p>Choose a component from the sidebar to preview it here</p>
		</div>
	{:else if componentLoad}
		{#await componentLoad}
			<div class="loading-state">
				<div class="spinner"></div>
				<p>Loading {component.name}...</p>
			</div>
		{:then loaded}
			{@const LoadedComponent = loaded.component}
			<div class="preview-wrapper" style={wrapperStyle}>
				<div class="preview-container" style={containerStyle}>
					{#key JSON.stringify(propValues)}
						<LoadedComponent {...propValues} />
					{/key}
				</div>
			</div>

			<!-- Render info bar -->
			<div class="render-info">
				<span class="info-item">
					<span class="info-label">Component:</span>
					{component.name}
				</span>
				<span class="info-item">
					<span class="info-label">Viewport:</span>
					{viewport.width}×{viewport.height ?? 'auto'}
				</span>
				<span class="info-item">
					<span class="info-label">Zoom:</span>
					{zoom}%
				</span>
				<span class="info-item">
					<span class="info-label">Load:</span>
					{loaded.renderTime}ms
				</span>
			</div>
		{:catch error}
			<div class="error-state">
				<span class="error-icon">❌</span>
				<h3>Failed to Load</h3>
				<p class="error-message">
					{error instanceof Error ? error.message : 'Failed to load component'}
				</p>
				<button class="retry-btn" onclick={() => (retryNonce += 1)}> ↻ Retry </button>
			</div>
		{/await}
	{/if}
</div>

<style>
	.renderer {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		overflow: auto;
		background: #0a0a0a;
		position: relative;
	}

	.empty-state,
	.loading-state,
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		text-align: center;
		padding: 3rem;
		color: #71717a;
	}

	.empty-icon,
	.error-icon {
		font-size: 3rem;
		margin-bottom: 1rem;
	}

	.empty-state h3,
	.error-state h3 {
		margin: 0 0 0.5rem;
		font-size: 1.25rem;
		font-weight: 600;
		color: #fafafa;
	}

	.empty-state p,
	.error-state p {
		margin: 0;
		font-size: 0.875rem;
	}

	.loading-state {
		gap: 1rem;
	}

	.spinner {
		width: 32px;
		height: 32px;
		border: 3px solid rgba(139, 92, 246, 0.2);
		border-top-color: #8b5cf6;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.error-message {
		color: #ef4444;
		margin-bottom: 1rem;
	}

	.retry-btn {
		padding: 0.5rem 1rem;
		background: rgba(139, 92, 246, 0.2);
		border: 1px solid rgba(139, 92, 246, 0.4);
		border-radius: 0.375rem;
		color: #c4b5fd;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.retry-btn:hover {
		background: rgba(139, 92, 246, 0.3);
	}

	.preview-wrapper {
		overflow: auto;
		display: flex;
		align-items: flex-start;
		justify-content: center;
		padding: 2rem;
	}

	.preview-container {
		box-sizing: border-box;
		overflow: hidden;
		border-radius: 0.5rem;
	}

	.render-info {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1.5rem;
		padding: 0.5rem 1rem;
		background: rgba(0, 0, 0, 0.8);
		border-top: 1px solid rgba(255, 255, 255, 0.1);
		font-size: 0.75rem;
	}

	.info-item {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		color: #a1a1aa;
	}

	.info-label {
		color: #52525b;
	}
</style>
