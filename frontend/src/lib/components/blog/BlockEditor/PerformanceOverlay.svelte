<!--
/**
 * Revolution Trading Pros - Performance Overlay
 * ===============================================================================
 * Real-time performance monitoring overlay for the Block Editor
 *
 * Features:
 * - Toggle with Ctrl+Shift+P
 * - Core Web Vitals display with color-coded thresholds
 * - FPS counter with visual indicator
 * - Memory usage graph
 * - Editor-specific metrics
 * - Only visible in development mode
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { browser, dev } from '$app/environment';
	import {
		subscribeToMetrics,
		getWebVitalThresholds,
		type MetricsSnapshot,
		type WebVitalName
	} from './performance/metrics';

	// ==========================================================================
	// State
	// ==========================================================================

	let isVisible = $state(false);
	let isMinimized = $state(false);
	let snapshot = $state<MetricsSnapshot | null>(null);
	let memoryHistory = $state<number[]>([]);
	let fpsHistory = $state<number[]>([]);
	let position = $state({ x: 16, y: 16 });
	let isDragging = $state(false);
	let dragOffset = $state({ x: 0, y: 0 });

	const MAX_HISTORY_POINTS = 60;

	let unsubscribe: (() => void) | null = null;

	// ==========================================================================
	// Lifecycle
	// ==========================================================================

	onMount(() => {
		if (!browser || !dev) return;

		// Subscribe to metrics
		unsubscribe = subscribeToMetrics((newSnapshot) => {
			snapshot = newSnapshot;

			// Update memory history
			if (newSnapshot.memoryMetrics) {
				memoryHistory = [
					...memoryHistory.slice(-(MAX_HISTORY_POINTS - 1)),
					newSnapshot.memoryMetrics.usagePercentage
				];
			}

			// Update FPS history
			fpsHistory = [...fpsHistory.slice(-(MAX_HISTORY_POINTS - 1)), newSnapshot.fps];
		});

		// Listen for keyboard shortcut
		window.addEventListener('keydown', handleKeyDown);

		// Load saved position
		const savedPosition = localStorage.getItem('rtp_perf_overlay_position');
		if (savedPosition) {
			try {
				position = JSON.parse(savedPosition);
			} catch (e) {
				// Ignore
			}
		}
	});

	onDestroy(() => {
		if (unsubscribe) unsubscribe();
		if (browser) {
			window.removeEventListener('keydown', handleKeyDown);
		}
	});

	// ==========================================================================
	// Event Handlers
	// ==========================================================================

	function handleKeyDown(e: KeyboardEvent) {
		// Ctrl+Shift+P to toggle
		if (e.ctrlKey && e.shiftKey && e.key === 'P') {
			e.preventDefault();
			isVisible = !isVisible;
		}
	}

	function handleMouseDown(e: MouseEvent) {
		if ((e.target as HTMLElement).closest('.perf-drag-handle')) {
			isDragging = true;
			dragOffset = {
				x: e.clientX - position.x,
				y: e.clientY - position.y
			};
			e.preventDefault();
		}
	}

	function handleMouseMove(e: MouseEvent) {
		if (!isDragging) return;

		position = {
			x: Math.max(0, Math.min(window.innerWidth - 320, e.clientX - dragOffset.x)),
			y: Math.max(0, Math.min(window.innerHeight - 100, e.clientY - dragOffset.y))
		};
	}

	function handleMouseUp() {
		if (isDragging) {
			isDragging = false;
			localStorage.setItem('rtp_perf_overlay_position', JSON.stringify(position));
		}
	}

	function toggleMinimize() {
		isMinimized = !isMinimized;
	}

	function close() {
		isVisible = false;
	}

	// ==========================================================================
	// Helpers
	// ==========================================================================

	function getWebVitalColor(name: WebVitalName, value: number): string {
		const thresholds = getWebVitalThresholds(name);
		if (value <= thresholds.good) return '#10b981'; // green
		if (value <= thresholds.poor) return '#f59e0b'; // yellow
		return '#ef4444'; // red
	}

	function formatWebVitalValue(name: WebVitalName, value: number): string {
		if (name === 'CLS') return value.toFixed(3);
		return `${value.toFixed(0)}ms`;
	}

	function getFpsColor(fps: number): string {
		if (fps >= 55) return '#10b981'; // green
		if (fps >= 30) return '#f59e0b'; // yellow
		return '#ef4444'; // red
	}

	function getMemoryColor(percentage: number): string {
		if (percentage < 50) return '#10b981'; // green
		if (percentage < 80) return '#f59e0b'; // yellow
		return '#ef4444'; // red
	}

	function formatBytes(bytes: number): string {
		const mb = bytes / (1024 * 1024);
		return `${mb.toFixed(1)}MB`;
	}

	function getSparklinePath(
		values: number[],
		width: number,
		height: number,
		maxValue: number
	): string {
		if (values.length < 2) return '';

		const stepX = width / (MAX_HISTORY_POINTS - 1);
		const points = values.map((v, i) => {
			const x = i * stepX;
			const y = height - (v / maxValue) * height;
			return `${x},${y}`;
		});

		return `M${points.join(' L')}`;
	}
</script>

<svelte:window onmousemove={handleMouseMove} onmouseup={handleMouseUp} />

{#if dev && isVisible}
	<div
		class="perf-overlay"
		class:minimized={isMinimized}
		style="left: {position.x}px; top: {position.y}px;"
		role="dialog"
		aria-label="Performance Monitor"
		aria-modal="false"
		tabindex="-1"
		onmousedown={handleMouseDown}
	>
		<!-- Header -->
		<div class="perf-header perf-drag-handle">
			<div class="perf-title">
				<svg
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path d="M22 12h-4l-3 9L9 3l-3 9H2" />
				</svg>
				<span>Performance</span>
			</div>
			<div class="perf-controls">
				<button
					type="button"
					class="perf-btn"
					onclick={toggleMinimize}
					aria-label={isMinimized ? 'Expand' : 'Minimize'}
				>
					{#if isMinimized}
						<svg
							width="12"
							height="12"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<path d="M15 3h6v6M9 21H3v-6M21 3l-7 7M3 21l7-7" />
						</svg>
					{:else}
						<svg
							width="12"
							height="12"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<path d="M4 14h6v6M20 10h-6V4M14 10l7-7M3 21l7-7" />
						</svg>
					{/if}
				</button>
				<button type="button" class="perf-btn" onclick={close} aria-label="Close">
					<svg
						width="12"
						height="12"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M18 6L6 18M6 6l12 12" />
					</svg>
				</button>
			</div>
		</div>

		{#if !isMinimized && snapshot}
			<div class="perf-content">
				<!-- FPS Counter -->
				<div class="perf-section">
					<div class="perf-section-header">
						<span class="perf-section-title">FPS</span>
						<span class="perf-fps-value" style="color: {getFpsColor(snapshot.fps)}">
							{snapshot.fps}
						</span>
					</div>
					<div class="perf-sparkline">
						<svg width="100%" height="24" preserveAspectRatio="none">
							<path
								d={getSparklinePath(fpsHistory, 280, 24, 60)}
								fill="none"
								stroke={getFpsColor(snapshot.fps)}
								stroke-width="1.5"
								opacity="0.8"
							/>
						</svg>
					</div>
				</div>

				<!-- Core Web Vitals -->
				<div class="perf-section">
					<div class="perf-section-title">Core Web Vitals</div>
					<div class="perf-vitals-grid">
						{#each ['LCP', 'FCP', 'CLS', 'INP', 'TTFB'] as const as vital}
							{@const metric = snapshot.webVitals[vital]}
							<div class="perf-vital">
								<span class="perf-vital-name">{vital}</span>
								{#if metric}
									<span
										class="perf-vital-value"
										style="color: {getWebVitalColor(vital, metric.value)}"
									>
										{formatWebVitalValue(vital, metric.value)}
									</span>
								{:else}
									<span class="perf-vital-value perf-vital-pending">--</span>
								{/if}
							</div>
						{/each}
					</div>
				</div>

				<!-- Memory Usage -->
				{#if snapshot.memoryMetrics}
					<div class="perf-section">
						<div class="perf-section-header">
							<span class="perf-section-title">Memory</span>
							<span
								class="perf-memory-value"
								style="color: {getMemoryColor(snapshot.memoryMetrics.usagePercentage)}"
							>
								{formatBytes(snapshot.memoryMetrics.usedJSHeapSize)} / {formatBytes(
									snapshot.memoryMetrics.jsHeapSizeLimit
								)}
							</span>
						</div>
						<div class="perf-memory-bar">
							<div
								class="perf-memory-fill"
								style="
									width: {snapshot.memoryMetrics.usagePercentage}%;
									background-color: {getMemoryColor(snapshot.memoryMetrics.usagePercentage)}
								"
							></div>
						</div>
						<div class="perf-sparkline">
							<svg width="100%" height="24" preserveAspectRatio="none">
								<path
									d={getSparklinePath(memoryHistory, 280, 24, 100)}
									fill="none"
									stroke={getMemoryColor(snapshot.memoryMetrics.usagePercentage)}
									stroke-width="1.5"
									opacity="0.8"
								/>
							</svg>
						</div>
					</div>
				{/if}

				<!-- Editor Metrics -->
				<div class="perf-section">
					<div class="perf-section-title">Editor Metrics</div>
					<div class="perf-metrics-list">
						<div class="perf-metric-row">
							<span class="perf-metric-name">Block Count</span>
							<span class="perf-metric-value">{snapshot.totalBlockCount}</span>
						</div>

						{#each Object.entries(snapshot.editorMetrics).slice(0, 8) as [name, metric]}
							{@const avg = metric.count > 0 ? metric.sum / metric.count : 0}
							{@const color = avg < 16 ? '#10b981' : avg < 50 ? '#f59e0b' : '#ef4444'}
							<div class="perf-metric-row">
								<span class="perf-metric-name" title={name}>
									{name
										.replace(/_/g, ' ')
										.replace(/block render/i, '')
										.trim() || 'render'}
								</span>
								<span class="perf-metric-value" style="color: {color}">
									{avg.toFixed(1)}ms
									<span class="perf-metric-count">({metric.count})</span>
								</span>
							</div>
						{/each}
					</div>
				</div>

				<!-- Recent Operations -->
				{#if snapshot.operationMetrics.length > 0}
					<div class="perf-section">
						<div class="perf-section-title">Recent Operations</div>
						<div class="perf-ops-list">
							{#each snapshot.operationMetrics.slice(-5).reverse() as op}
								{@const color =
									op.duration < 100 ? '#10b981' : op.duration < 500 ? '#f59e0b' : '#ef4444'}
								<div class="perf-op-row">
									<span class="perf-op-name">{op.operation}</span>
									<span class="perf-op-duration" style="color: {color}">
										{op.duration.toFixed(0)}ms
									</span>
									<span
										class="perf-op-status"
										class:success={op.success}
										class:failure={!op.success}
									>
										{op.success ? 'OK' : 'FAIL'}
									</span>
								</div>
							{/each}
						</div>
					</div>
				{/if}
			</div>

			<!-- Footer -->
			<div class="perf-footer">
				<span class="perf-shortcut">Ctrl+Shift+P to toggle</span>
			</div>
		{/if}
	</div>
{/if}

<style>
	.perf-overlay {
		position: fixed;
		z-index: 99999;
		width: 320px;
		max-height: calc(100vh - 32px);
		background: rgba(17, 24, 39, 0.95);
		backdrop-filter: blur(8px);
		border: 1px solid rgba(75, 85, 99, 0.5);
		border-radius: 8px;
		font-family: ui-monospace, 'SF Mono', 'Monaco', 'Cascadia Code', monospace;
		font-size: 11px;
		color: #e5e7eb;
		box-shadow:
			0 20px 25px -5px rgba(0, 0, 0, 0.3),
			0 10px 10px -5px rgba(0, 0, 0, 0.2);
		overflow: hidden;
		user-select: none;
	}

	.perf-overlay.minimized {
		width: auto;
		min-width: 140px;
	}

	.perf-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 8px 12px;
		background: rgba(31, 41, 55, 0.8);
		border-bottom: 1px solid rgba(75, 85, 99, 0.3);
		cursor: grab;
	}

	.perf-header:active {
		cursor: grabbing;
	}

	.perf-title {
		display: flex;
		align-items: center;
		gap: 6px;
		font-weight: 600;
		color: #8b5cf6;
	}

	.perf-controls {
		display: flex;
		gap: 4px;
	}

	.perf-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 20px;
		height: 20px;
		padding: 0;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: #9ca3af;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.perf-btn:hover {
		background: rgba(75, 85, 99, 0.5);
		color: #e5e7eb;
	}

	.perf-content {
		max-height: calc(100vh - 120px);
		overflow-y: auto;
		scrollbar-width: thin;
		scrollbar-color: rgba(75, 85, 99, 0.5) transparent;
	}

	.perf-content::-webkit-scrollbar {
		width: 6px;
	}

	.perf-content::-webkit-scrollbar-track {
		background: transparent;
	}

	.perf-content::-webkit-scrollbar-thumb {
		background: rgba(75, 85, 99, 0.5);
		border-radius: 3px;
	}

	.perf-section {
		padding: 10px 12px;
		border-bottom: 1px solid rgba(75, 85, 99, 0.2);
	}

	.perf-section:last-child {
		border-bottom: none;
	}

	.perf-section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 6px;
	}

	.perf-section-title {
		font-size: 10px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		color: #9ca3af;
	}

	.perf-fps-value {
		font-size: 18px;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
	}

	.perf-sparkline {
		height: 24px;
		margin-top: 4px;
	}

	.perf-vitals-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 8px;
		margin-top: 8px;
	}

	.perf-vital {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 6px;
		background: rgba(31, 41, 55, 0.5);
		border-radius: 4px;
	}

	.perf-vital-name {
		font-size: 9px;
		font-weight: 600;
		color: #9ca3af;
		margin-bottom: 2px;
	}

	.perf-vital-value {
		font-size: 12px;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
	}

	.perf-vital-pending {
		color: #6b7280;
	}

	.perf-memory-value {
		font-size: 11px;
		font-weight: 600;
		font-variant-numeric: tabular-nums;
	}

	.perf-memory-bar {
		height: 4px;
		background: rgba(31, 41, 55, 0.8);
		border-radius: 2px;
		overflow: hidden;
		margin-top: 6px;
	}

	.perf-memory-fill {
		height: 100%;
		border-radius: 2px;
		transition:
			width 0.3s ease,
			background-color 0.3s ease;
	}

	.perf-metrics-list,
	.perf-ops-list {
		display: flex;
		flex-direction: column;
		gap: 4px;
		margin-top: 8px;
	}

	.perf-metric-row,
	.perf-op-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 4px 6px;
		background: rgba(31, 41, 55, 0.3);
		border-radius: 4px;
	}

	.perf-metric-name,
	.perf-op-name {
		flex: 1;
		font-size: 10px;
		color: #9ca3af;
		text-transform: capitalize;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.perf-metric-value,
	.perf-op-duration {
		font-size: 11px;
		font-weight: 600;
		font-variant-numeric: tabular-nums;
		margin-left: 8px;
	}

	.perf-metric-count {
		font-size: 9px;
		font-weight: 400;
		color: #6b7280;
		margin-left: 2px;
	}

	.perf-op-status {
		font-size: 9px;
		font-weight: 700;
		padding: 1px 4px;
		border-radius: 2px;
		margin-left: 6px;
	}

	.perf-op-status.success {
		background: rgba(16, 185, 129, 0.2);
		color: #10b981;
	}

	.perf-op-status.failure {
		background: rgba(239, 68, 68, 0.2);
		color: #ef4444;
	}

	.perf-footer {
		padding: 6px 12px;
		background: rgba(31, 41, 55, 0.5);
		border-top: 1px solid rgba(75, 85, 99, 0.2);
		text-align: center;
	}

	.perf-shortcut {
		font-size: 9px;
		color: #6b7280;
	}

	/* Animations */
	@keyframes fadeIn {
		from {
			opacity: 0;
			transform: translateY(-10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.perf-overlay {
		animation: fadeIn 0.2s ease-out;
	}
</style>
