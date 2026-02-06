<!--
/**
 * VideoTranscript - Interactive Video Transcript Component
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT 7 Principal Engineer Grade - February 2026
 *
 * FEATURES:
 * 1. Displays video transcript with timestamps
 * 2. Syncs with video playback (highlights current line)
 * 3. Click to seek to transcript position
 * 4. Search within transcript
 * 5. Copy transcript functionality
 * 6. Mobile-first responsive design
 *
 * @version 1.0.0
 */
-->
<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';

	// ═══════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════

	interface TranscriptCue {
		id: string;
		startTime: number;
		endTime: number;
		text: string;
	}

	interface Props {
		/** VTT file URL */
		vttUrl?: string;
		/** Transcript cues array (alternative to VTT) */
		cues?: TranscriptCue[];
		/** Current video time in seconds */
		currentTime?: number;
		/** On seek callback (time in seconds) */
		onSeek?: (time: number) => void;
		/** Custom class */
		class?: string;
	}

	let { vttUrl, cues: propCues, currentTime = 0, onSeek, class: className = '' }: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════

	let transcriptCues = $state<TranscriptCue[]>([]);
	let searchQuery = $state('');
	let isLoading = $state(false);
	let error = $state<string | null>(null);
	let activeLineElement = $state<HTMLElement | null>(null);
	let containerElement = $state<HTMLElement | null>(null);
	let copied = $state(false);

	// ═══════════════════════════════════════════════════════════════════════
	// COMPUTED
	// ═══════════════════════════════════════════════════════════════════════

	let activeCueId = $derived.by(() => {
		for (const cue of transcriptCues) {
			if (currentTime >= cue.startTime && currentTime < cue.endTime) {
				return cue.id;
			}
		}
		return null;
	});

	let filteredCues = $derived.by(() => {
		if (!searchQuery.trim()) return transcriptCues;
		const query = searchQuery.toLowerCase();
		return transcriptCues.filter((cue) => cue.text.toLowerCase().includes(query));
	});

	// ═══════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════

	onMount(async () => {
		if (!browser) return;

		if (propCues && propCues.length > 0) {
			transcriptCues = propCues;
		} else if (vttUrl) {
			await loadVTT();
		}
	});

	// Auto-scroll to active line
	$effect(() => {
		const activeId = activeCueId;
		if (activeId && containerElement && !searchQuery) {
			const element = containerElement.querySelector(`[data-cue-id="${activeId}"]`);
			if (element) {
				element.scrollIntoView({ behavior: 'smooth', block: 'center' });
			}
		}
	});

	// ═══════════════════════════════════════════════════════════════════════
	// METHODS
	// ═══════════════════════════════════════════════════════════════════════

	async function loadVTT() {
		if (!vttUrl) return;

		isLoading = true;
		error = null;

		try {
			const response = await fetch(vttUrl);
			const vttText = await response.text();
			transcriptCues = parseVTT(vttText);
		} catch (e) {
			error = 'Failed to load transcript';
			console.error('Transcript load error:', e);
		} finally {
			isLoading = false;
		}
	}

	function parseVTT(vttText: string): TranscriptCue[] {
		const cues: TranscriptCue[] = [];
		const lines = vttText.split('\n');

		let currentCue: Partial<TranscriptCue> = {};
		let cueId = 0;

		for (let i = 0; i < lines.length; i++) {
			const line = lines[i].trim();

			// Skip WEBVTT header and empty lines
			if (line === 'WEBVTT' || line === '') {
				if (currentCue.text) {
					cues.push({
						id: String(cueId++),
						startTime: currentCue.startTime || 0,
						endTime: currentCue.endTime || 0,
						text: currentCue.text
					});
					currentCue = {};
				}
				continue;
			}

			// Parse timestamp line (00:00:00.000 --> 00:00:05.000)
			if (line.includes('-->')) {
				const [start, end] = line.split('-->').map((t) => parseTimestamp(t.trim()));
				currentCue.startTime = start;
				currentCue.endTime = end;
			} else if (currentCue.startTime !== undefined) {
				// Text line
				currentCue.text = currentCue.text ? `${currentCue.text} ${line}` : line;
			}
		}

		// Add last cue
		if (currentCue.text) {
			cues.push({
				id: String(cueId),
				startTime: currentCue.startTime || 0,
				endTime: currentCue.endTime || 0,
				text: currentCue.text
			});
		}

		return cues;
	}

	function parseTimestamp(timestamp: string): number {
		// Handle both HH:MM:SS.mmm and MM:SS.mmm formats
		const parts = timestamp.split(':');
		let hours = 0,
			minutes = 0,
			seconds = 0;

		if (parts.length === 3) {
			hours = parseFloat(parts[0]);
			minutes = parseFloat(parts[1]);
			seconds = parseFloat(parts[2]);
		} else if (parts.length === 2) {
			minutes = parseFloat(parts[0]);
			seconds = parseFloat(parts[1]);
		}

		return hours * 3600 + minutes * 60 + seconds;
	}

	function formatTimestamp(seconds: number): string {
		const hours = Math.floor(seconds / 3600);
		const mins = Math.floor((seconds % 3600) / 60);
		const secs = Math.floor(seconds % 60);

		if (hours > 0) {
			return `${hours}:${String(mins).padStart(2, '0')}:${String(secs).padStart(2, '0')}`;
		}
		return `${mins}:${String(secs).padStart(2, '0')}`;
	}

	function handleCueClick(cue: TranscriptCue) {
		onSeek?.(cue.startTime);
	}

	async function copyTranscript() {
		const fullText = transcriptCues.map((cue) => cue.text).join('\n\n');

		try {
			await navigator.clipboard.writeText(fullText);
			copied = true;
			setTimeout(() => {
				copied = false;
			}, 2000);
		} catch (e) {
			console.error('Failed to copy transcript:', e);
		}
	}

	function highlightSearch(text: string): string {
		if (!searchQuery.trim()) return text;
		const regex = new RegExp(`(${searchQuery.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi');
		return text.replace(regex, '<mark>$1</mark>');
	}
</script>

<div class="transcript {className}" bind:this={containerElement}>
	<!-- Header with search and copy -->
	<div class="transcript__header">
		<h3 class="transcript__title">Transcript</h3>
		<div class="transcript__actions">
			<div class="transcript__search">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" width="16" height="16">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
					/>
				</svg>
				<input
					type="text"
					bind:value={searchQuery}
					placeholder="Search transcript..."
					class="transcript__search-input"
				/>
			</div>
			<button
				class="transcript__copy-btn"
				onclick={copyTranscript}
				title="Copy transcript"
				disabled={transcriptCues.length === 0}
			>
				{#if copied}
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" width="18" height="18">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M5 13l4 4L19 7"
						/>
					</svg>
				{:else}
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" width="18" height="18">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
						/>
					</svg>
				{/if}
			</button>
		</div>
	</div>

	<!-- Content -->
	<div class="transcript__content">
		{#if isLoading}
			<div class="transcript__loading">
				<div class="transcript__spinner"></div>
				<span>Loading transcript...</span>
			</div>
		{:else if error}
			<div class="transcript__error">{error}</div>
		{:else if filteredCues.length === 0}
			<div class="transcript__empty">
				{#if searchQuery}
					No results found for "{searchQuery}"
				{:else}
					No transcript available
				{/if}
			</div>
		{:else}
			<div class="transcript__list">
				{#each filteredCues as cue (cue.id)}
					<button
						class="transcript__cue"
						class:transcript__cue--active={activeCueId === cue.id}
						data-cue-id={cue.id}
						onclick={() => handleCueClick(cue)}
					>
						<span class="transcript__timestamp">{formatTimestamp(cue.startTime)}</span>
						<span class="transcript__text">
							{@html highlightSearch(cue.text)}
						</span>
					</button>
				{/each}
			</div>
		{/if}
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   TRANSCRIPT - 2026 Mobile-First Responsive Design
	   ═══════════════════════════════════════════════════════════════════════════ */

	.transcript {
		display: flex;
		flex-direction: column;
		background: var(--color-surface, #fff);
		border-radius: 8px;
		overflow: hidden;
		height: 100%;
		max-height: 500px;
	}

	.transcript__header {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		padding: 1rem;
		border-bottom: 1px solid var(--color-border, #e5e7eb);
	}

	.transcript__title {
		font-size: 1rem;
		font-weight: 600;
		color: var(--color-text, #1f2937);
		margin: 0;
	}

	.transcript__actions {
		display: flex;
		gap: 0.5rem;
		align-items: center;
	}

	.transcript__search {
		display: flex;
		align-items: center;
		flex: 1;
		background: var(--color-bg, #f3f4f6);
		border-radius: 6px;
		padding: 0.5rem 0.75rem;
		gap: 0.5rem;
		color: var(--color-text-muted, #6b7280);
	}

	.transcript__search-input {
		flex: 1;
		border: none;
		background: transparent;
		font-size: 0.875rem;
		color: var(--color-text, #1f2937);
		outline: none;
	}

	.transcript__search-input::placeholder {
		color: var(--color-text-muted, #9ca3af);
	}

	.transcript__copy-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		border: none;
		background: var(--color-bg, #f3f4f6);
		border-radius: 6px;
		color: var(--color-text-muted, #6b7280);
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.transcript__copy-btn:hover:not(:disabled) {
		background: var(--color-primary, #2563eb);
		color: #fff;
	}

	.transcript__copy-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Content */
	.transcript__content {
		flex: 1;
		overflow-y: auto;
		scroll-behavior: smooth;
	}

	.transcript__list {
		display: flex;
		flex-direction: column;
	}

	/* Cue button */
	.transcript__cue {
		display: flex;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		border: none;
		background: transparent;
		text-align: left;
		cursor: pointer;
		transition: background 0.15s ease;
		border-bottom: 1px solid var(--color-border, #f3f4f6);
	}

	.transcript__cue:hover {
		background: var(--color-bg-hover, #f9fafb);
	}

	.transcript__cue--active {
		background: var(--color-primary-light, #eff6ff);
		border-left: 3px solid var(--color-primary, #2563eb);
	}

	.transcript__timestamp {
		flex-shrink: 0;
		font-size: 0.75rem;
		font-weight: 500;
		color: var(--color-primary, #2563eb);
		font-variant-numeric: tabular-nums;
		min-width: 45px;
	}

	.transcript__text {
		font-size: 0.875rem;
		color: var(--color-text, #374151);
		line-height: 1.5;
	}

	.transcript__text :global(mark) {
		background: #fef08a;
		color: inherit;
		padding: 0 2px;
		border-radius: 2px;
	}

	/* Loading & Empty states */
	.transcript__loading,
	.transcript__empty,
	.transcript__error {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		color: var(--color-text-muted, #6b7280);
		text-align: center;
		gap: 0.75rem;
	}

	.transcript__error {
		color: var(--color-error, #ef4444);
	}

	.transcript__spinner {
		width: 24px;
		height: 24px;
		border: 2px solid var(--color-border, #e5e7eb);
		border-top-color: var(--color-primary, #2563eb);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (min-width: 640px) {
		.transcript__header {
			flex-direction: row;
			justify-content: space-between;
			align-items: center;
		}

		.transcript__actions {
			flex-shrink: 0;
		}

		.transcript__search {
			width: 200px;
		}
	}

	/* Dark mode */
	@media (prefers-color-scheme: dark) {
		.transcript {
			background: #1f2937;
		}

		.transcript__title {
			color: #f9fafb;
		}

		.transcript__search {
			background: #374151;
		}

		.transcript__search-input {
			color: #f9fafb;
		}

		.transcript__copy-btn {
			background: #374151;
			color: #9ca3af;
		}

		.transcript__cue:hover {
			background: #374151;
		}

		.transcript__cue--active {
			background: rgba(37, 99, 235, 0.2);
		}

		.transcript__text {
			color: #d1d5db;
		}
	}
</style>
