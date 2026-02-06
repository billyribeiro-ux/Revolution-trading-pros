<!--
/**
 * Performance Monitoring Dashboard
 * ═══════════════════════════════════════════════════════════════════════════
 * Real-time performance metrics display
 */
-->

<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';

	interface Props {
		visible?: boolean;
		position?: 'bottom-right' | 'bottom-left' | 'top-right' | 'top-left';
	}

	let props: Props = $props();

	let visible = $derived(props.visible ?? true);
	let position = $derived(props.position ?? 'bottom-right');

	let fps = $state(0);
	let memory = $state({ used: 0, total: 0, percent: 0 });
	let renderTime = $state(0);
	let blockCount = $state(0);
	let cacheStats = $state({ hits: 0, misses: 0, hitRate: '0%' });

	let frameCount = 0;
	let lastTime = performance.now();
	let animationId: number;
	let updateInterval: ReturnType<typeof setInterval>;

	function measureFPS(): void {
		frameCount++;
		const currentTime = performance.now();

		if (currentTime - lastTime >= 1000) {
			fps = Math.round((frameCount * 1000) / (currentTime - lastTime));
			frameCount = 0;
			lastTime = currentTime;
		}

		animationId = requestAnimationFrame(measureFPS);
	}

	function updateMetrics(): void {
		if (!browser) return;

		// Memory (if available)
		const perfMemory = (performance as any).memory;
		if (perfMemory) {
			memory = {
				used: Math.round(perfMemory.usedJSHeapSize / 1024 / 1024),
				total: Math.round(perfMemory.jsHeapSizeLimit / 1024 / 1024),
				percent: Math.round((perfMemory.usedJSHeapSize / perfMemory.jsHeapSizeLimit) * 100)
			};
		}

		// Block count
		const blocks = document.querySelectorAll('[data-block-id]');
		blockCount = blocks.length;

		// Render time from performance entries
		const entries = performance.getEntriesByType('measure');
		const renderEntry = entries.find((e) => e.name.includes('render'));
		if (renderEntry) {
			renderTime = Math.round(renderEntry.duration * 100) / 100;
		}
	}

	onMount(() => {
		if (browser) {
			measureFPS();
			updateInterval = setInterval(updateMetrics, 1000);
			updateMetrics();
		}
	});

	onDestroy(() => {
		if (browser) {
			cancelAnimationFrame(animationId);
			clearInterval(updateInterval);
		}
	});

	function getStatusColor(value: number, thresholds: { good: number; warn: number }): string {
		if (value >= thresholds.good) return 'good';
		if (value >= thresholds.warn) return 'warn';
		return 'bad';
	}

	function getInverseStatusColor(
		value: number,
		thresholds: { good: number; warn: number }
	): string {
		if (value <= thresholds.good) return 'good';
		if (value <= thresholds.warn) return 'warn';
		return 'bad';
	}

	let positionClass = $derived(
		{
			'bottom-right': 'pos-br',
			'bottom-left': 'pos-bl',
			'top-right': 'pos-tr',
			'top-left': 'pos-tl'
		}[position]
	);
</script>

{#if visible}
	<div class="perf-dashboard {positionClass}" role="region" aria-label="Performance metrics">
		<div class="dashboard-header">
			<span class="dashboard-title">⚡ Performance</span>
		</div>

		<div class="metrics-grid">
			<div class="metric">
				<span class="metric-label">FPS</span>
				<span class="metric-value {getStatusColor(fps, { good: 55, warn: 30 })}">{fps}</span>
			</div>

			<div class="metric">
				<span class="metric-label">Render</span>
				<span class="metric-value {getInverseStatusColor(renderTime, { good: 8, warn: 16 })}"
					>{renderTime}ms</span
				>
			</div>

			<div class="metric">
				<span class="metric-label">Blocks</span>
				<span class="metric-value">{blockCount}</span>
			</div>

			{#if memory.total > 0}
				<div class="metric">
					<span class="metric-label">Memory</span>
					<span class="metric-value {getInverseStatusColor(memory.percent, { good: 50, warn: 75 })}"
						>{memory.used}MB</span
					>
				</div>
			{/if}

			<div class="metric">
				<span class="metric-label">Cache</span>
				<span class="metric-value">{cacheStats.hitRate}</span>
			</div>
		</div>

		<div class="dashboard-footer">
			<span class="timestamp">{new Date().toLocaleTimeString()}</span>
		</div>
	</div>
{/if}

<style>
	.perf-dashboard {
		position: fixed;
		padding: 0.75rem;
		background: rgba(15, 23, 42, 0.95);
		backdrop-filter: blur(8px);
		color: white;
		border-radius: 10px;
		font-family: 'SF Mono', 'Fira Code', monospace;
		font-size: 0.6875rem;
		z-index: 99999;
		min-width: 160px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
		border: 1px solid rgba(255, 255, 255, 0.1);
	}

	.pos-br {
		bottom: 1rem;
		right: 1rem;
	}
	.pos-bl {
		bottom: 1rem;
		left: 1rem;
	}
	.pos-tr {
		top: 1rem;
		right: 1rem;
	}
	.pos-tl {
		top: 1rem;
		left: 1rem;
	}

	.dashboard-header {
		margin-bottom: 0.5rem;
		padding-bottom: 0.5rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.dashboard-title {
		font-weight: 600;
		font-size: 0.75rem;
	}

	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 0.5rem;
	}

	.metric {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.metric-label {
		color: #94a3b8;
		font-size: 0.625rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.metric-value {
		font-weight: 600;
		font-size: 0.875rem;
		font-variant-numeric: tabular-nums;
	}

	.metric-value.good {
		color: #22c55e;
	}
	.metric-value.warn {
		color: #f59e0b;
	}
	.metric-value.bad {
		color: #ef4444;
	}

	.dashboard-footer {
		margin-top: 0.5rem;
		padding-top: 0.5rem;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
	}

	.timestamp {
		color: #64748b;
		font-size: 0.625rem;
	}
</style>
