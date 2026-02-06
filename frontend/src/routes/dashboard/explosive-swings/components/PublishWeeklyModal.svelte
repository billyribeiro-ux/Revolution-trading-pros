<!--
	═══════════════════════════════════════════════════════════════════════════════════
	PublishWeeklyModal - Unified Weekly Video + Trade Plan Upload
	═══════════════════════════════════════════════════════════════════════════════════
	
	@description Combined workflow for publishing weekly breakdown with video and trade plan
	             entries in a single, streamlined modal. Features side-by-side layout on
	             desktop and stacked accordion on mobile.
	@version 1.0.0 - ICT 7+ Grade: Unified workflow, responsive design
	@requires Svelte 5.0+ (January 2026 syntax)
	@standards Apple Principal Engineer ICT 7+ Standards | WCAG 2.1 AA
	
	Features:
	  - Side-by-side layout (desktop 1024px+)
	  - Stacked accordion (tablet/mobile)
	  - Copy from last week functionality
	  - Batch trade plan entry creation
	  - Real-time validation
	  - Keyboard shortcuts (Cmd/Ctrl+Enter to publish)
	
	@example
	<PublishWeeklyModal
	  isOpen={showModal}
	  roomSlug="explosive-swings"
	  onClose={() => showModal = false}
	  onSuccess={() => refreshData()}
	/>
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import {
		weeklyVideoApi,
		tradePlanApi,
		type Bias,
		type TradePlanEntry
	} from '$lib/api/room-content';

	// ═══════════════════════════════════════════════════════════════════════════════════
	// TYPE DEFINITIONS
	// ═══════════════════════════════════════════════════════════════════════════════════

	interface TradePlanRow {
		id: string; // Temporary client ID for tracking
		ticker: string;
		bias: Bias;
		entry: string;
		target1: string;
		target2: string;
		target3: string;
		runner: string;
		stop: string;
		optionsStrike: string;
		optionsExp: string;
		notes: string;
		isExpanded: boolean;
	}

	interface Props {
		isOpen: boolean;
		roomSlug: string;
		onClose: () => void;
		onSuccess?: () => void;
	}

	// ═══════════════════════════════════════════════════════════════════════════════════
	// PROPS & STATE
	// ═══════════════════════════════════════════════════════════════════════════════════

	const { isOpen, roomSlug, onClose, onSuccess }: Props = $props();

	// Modal refs
	let modalRef = $state<HTMLDivElement | null>(null);

	// Current step (for mobile accordion)
	let activeSection = $state<'video' | 'trades' | 'review'>('video');

	// Video form state
	let videoForm = $state({
		week_of: getNextMonday(),
		week_title: '',
		video_title: '',
		video_url: '',
		thumbnail_url: '',
		duration: '',
		description: ''
	});

	// Trade plan entries
	let tradePlanRows = $state<TradePlanRow[]>([createEmptyRow()]);

	// UI state
	let isPublishing = $state(false);
	let errorMessage = $state('');
	let successMessage = $state('');
	let isLoadingLastWeek = $state(false);

	// A11y announcements
	let statusAnnouncement = $state('');

	// ═══════════════════════════════════════════════════════════════════════════════════
	// DERIVED VALUES
	// ═══════════════════════════════════════════════════════════════════════════════════

	const isVideoValid = $derived(
		isValidBunnyUrl(videoForm.video_url) &&
			videoForm.video_title.trim().length >= 3 &&
			videoForm.week_of !== ''
	);

	const validTradePlanCount = $derived(
		tradePlanRows.filter((row) => isValidTicker(row.ticker)).length
	);

	const hasTradePlanEntries = $derived(validTradePlanCount > 0);

	const canPublish = $derived(isVideoValid && !isPublishing);

	const canPreviewVideo = $derived(isValidBunnyUrl(videoForm.video_url));

	// ═══════════════════════════════════════════════════════════════════════════════════
	// UTILITY FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════════════

	function getNextMonday(): string {
		const today = new Date();
		const dayOfWeek = today.getDay();
		const daysUntilMonday = dayOfWeek === 0 ? 1 : (8 - dayOfWeek) % 7 || 7;
		const nextMonday = new Date(today);
		nextMonday.setDate(today.getDate() + daysUntilMonday);
		return nextMonday.toISOString().split('T')[0];
	}

	function formatWeekDisplay(dateStr: string): string {
		if (!dateStr) return '';
		try {
			const date = new Date(dateStr + 'T00:00:00');
			return date.toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' });
		} catch {
			return dateStr;
		}
	}

	function isValidBunnyUrl(url: string): boolean {
		if (!url || typeof url !== 'string') return false;
		try {
			const parsed = new URL(url.trim());
			return parsed.protocol === 'https:' && parsed.hostname === 'iframe.mediadelivery.net';
		} catch {
			return false;
		}
	}

	function isValidTicker(ticker: string): boolean {
		if (!ticker) return false;
		const cleaned = ticker.trim().toUpperCase();
		return /^[A-Z]{1,5}$/.test(cleaned);
	}

	function isValidPrice(price: string): boolean {
		if (!price || price.trim() === '') return true;
		const cleaned = price.replace(/[$,]/g, '');
		const num = parseFloat(cleaned);
		return !isNaN(num) && isFinite(num) && num >= 0;
	}

	function createEmptyRow(): TradePlanRow {
		return {
			id: crypto.randomUUID(),
			ticker: '',
			bias: 'BULLISH',
			entry: '',
			target1: '',
			target2: '',
			target3: '',
			runner: '',
			stop: '',
			optionsStrike: '',
			optionsExp: '',
			notes: '',
			isExpanded: false
		};
	}

	function generateId(): string {
		return crypto.randomUUID();
	}

	// ═══════════════════════════════════════════════════════════════════════════════════
	// TRADE PLAN ROW ACTIONS
	// ═══════════════════════════════════════════════════════════════════════════════════

	function addRow() {
		tradePlanRows = [...tradePlanRows, createEmptyRow()];
		statusAnnouncement = 'New trade plan row added';
	}

	function removeRow(id: string) {
		if (tradePlanRows.length <= 1) return;
		tradePlanRows = tradePlanRows.filter((row) => row.id !== id);
		statusAnnouncement = 'Trade plan row removed';
	}

	function toggleRowNotes(id: string) {
		tradePlanRows = tradePlanRows.map((row) =>
			row.id === id ? { ...row, isExpanded: !row.isExpanded } : row
		);
	}

	function updateRow(id: string, field: keyof TradePlanRow, value: string) {
		tradePlanRows = tradePlanRows.map((row) => (row.id === id ? { ...row, [field]: value } : row));
	}

	// ═══════════════════════════════════════════════════════════════════════════════════
	// COPY FROM LAST WEEK
	// ═══════════════════════════════════════════════════════════════════════════════════

	async function copyFromLastWeek() {
		isLoadingLastWeek = true;
		errorMessage = '';

		try {
			const response = await tradePlanApi.list(roomSlug);
			if (response.data && response.data.length > 0) {
				// Convert API entries to our row format
				const previousEntries: TradePlanRow[] = response.data.map((entry: TradePlanEntry) => ({
					id: generateId(),
					ticker: entry.ticker || '',
					bias: (entry.bias as Bias) || 'BULLISH',
					entry: entry.entry || '',
					target1: entry.target1 || '',
					target2: entry.target2 || '',
					target3: entry.target3 || '',
					runner: entry.runner || '',
					stop: entry.stop || '',
					optionsStrike: entry.options_strike || '',
					optionsExp: '', // Reset expiration for new week
					notes: entry.notes || '',
					isExpanded: false
				}));

				tradePlanRows = previousEntries.length > 0 ? previousEntries : [createEmptyRow()];
				statusAnnouncement = `Copied ${previousEntries.length} entries from last week`;
			} else {
				errorMessage = 'No previous trade plan entries found';
			}
		} catch (err) {
			console.error('Failed to copy from last week:', err);
			errorMessage = 'Failed to load previous entries';
		} finally {
			isLoadingLastWeek = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════════════
	// PUBLISH WORKFLOW
	// ═══════════════════════════════════════════════════════════════════════════════════

	async function handlePublish() {
		if (!canPublish) return;

		isPublishing = true;
		errorMessage = '';
		successMessage = '';

		try {
			// Step 1: Create the weekly video
			const videoResponse = await weeklyVideoApi.create({
				room_slug: roomSlug,
				week_of: videoForm.week_of,
				week_title: videoForm.week_title || `Week of ${formatWeekDisplay(videoForm.week_of)}`,
				video_title: videoForm.video_title,
				video_url: videoForm.video_url,
				video_platform: 'bunny',
				thumbnail_url: videoForm.thumbnail_url || undefined,
				duration: videoForm.duration || undefined,
				description: videoForm.description || undefined
			});

			if (!videoResponse) {
				throw new Error('Failed to create weekly video');
			}

			// Step 2: Create trade plan entries (if any valid entries)
			const validEntries = tradePlanRows.filter((row) => isValidTicker(row.ticker));

			if (validEntries.length > 0) {
				// Create entries in parallel
				const entryPromises = validEntries.map((row, index) =>
					tradePlanApi.create({
						room_slug: roomSlug,
						week_of: videoForm.week_of,
						ticker: row.ticker.toUpperCase(),
						bias: row.bias,
						entry: row.entry || undefined,
						target1: row.target1 || undefined,
						target2: row.target2 || undefined,
						target3: row.target3 || undefined,
						runner: row.runner || undefined,
						stop: row.stop || undefined,
						options_strike: row.optionsStrike || undefined,
						options_exp: row.optionsExp || undefined,
						notes: row.notes || undefined,
						sort_order: index
					})
				);

				await Promise.all(entryPromises);
			}

			successMessage = `Published weekly breakdown with ${validEntries.length} trade plan entries`;
			statusAnnouncement = successMessage;

			// Notify parent and close after delay
			setTimeout(() => {
				onSuccess?.();
				handleClose();
			}, 1500);
		} catch (err) {
			console.error('Publish failed:', err);
			errorMessage = err instanceof Error ? err.message : 'Failed to publish weekly breakdown';
		} finally {
			isPublishing = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════════════
	// MODAL LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════════════

	function handleClose() {
		resetForm();
		onClose();
	}

	function resetForm() {
		videoForm = {
			week_of: getNextMonday(),
			week_title: '',
			video_title: '',
			video_url: '',
			thumbnail_url: '',
			duration: '',
			description: ''
		};
		tradePlanRows = [createEmptyRow()];
		errorMessage = '';
		successMessage = '';
		activeSection = 'video';
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			handleClose();
		}
		// Cmd/Ctrl + Enter to publish
		if ((e.metaKey || e.ctrlKey) && e.key === 'Enter' && canPublish) {
			handlePublish();
		}
	}

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			handleClose();
		}
	}

	// Lock body scroll when modal is open
	$effect(() => {
		if (!isOpen) return;

		const previousOverflow = document.body.style.overflow;
		document.body.style.overflow = 'hidden';

		requestAnimationFrame(() => modalRef?.focus());

		return () => {
			document.body.style.overflow = previousOverflow;
		};
	});
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
	<!-- A11y status announcements -->
	<div class="sr-only" role="status" aria-live="polite" aria-atomic="true">
		{statusAnnouncement}
	</div>

	<!-- Modal Backdrop -->
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		class="modal-backdrop"
		role="dialog"
		aria-modal="true"
		aria-labelledby="publish-modal-title"
		tabindex="-1"
		onclick={handleBackdropClick}
		onkeydown={handleKeydown}
	>
		<!-- Modal Container -->
		<div class="modal-container" bind:this={modalRef} tabindex="-1">
			<!-- Modal Header -->
			<header class="modal-header">
				<h2 id="publish-modal-title">Publish Weekly Breakdown</h2>
				<button type="button" class="close-btn" onclick={handleClose} aria-label="Close modal">
					<svg
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						width="24"
						height="24"
					>
						<path d="M18 6L6 18M6 6l12 12" />
					</svg>
				</button>
			</header>

			<!-- Error/Success Messages -->
			{#if errorMessage}
				<div class="message message--error" role="alert">
					<svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
						<path
							d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"
						/>
					</svg>
					{errorMessage}
				</div>
			{/if}

			{#if successMessage}
				<div class="message message--success" role="status">
					<svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
						<path
							d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"
						/>
					</svg>
					{successMessage}
				</div>
			{/if}

			<!-- Modal Body - Split Layout -->
			<div class="modal-body">
				<!-- LEFT PANEL: Video Upload -->
				<section class="panel panel--video">
					<div class="panel-header">
						<h3>
							<svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
								<path d="M8 5v14l11-7z" />
							</svg>
							Video Details
						</h3>
						{#if isVideoValid}
							<span class="status-badge status-badge--success">✓ Ready</span>
						{/if}
					</div>

					<div class="panel-content">
						<!-- Week Selection -->
						<div class="form-group">
							<label for="week_of">Week Of</label>
							<input type="date" id="week_of" bind:value={videoForm.week_of} required />
						</div>

						<!-- Video URL -->
						<div class="form-group">
							<label for="video_url">Bunny.net Embed URL <span class="required">*</span></label>
							<input
								type="url"
								id="video_url"
								bind:value={videoForm.video_url}
								placeholder="https://iframe.mediadelivery.net/embed/..."
								class:invalid={videoForm.video_url && !isValidBunnyUrl(videoForm.video_url)}
								required
							/>
							{#if videoForm.video_url && !isValidBunnyUrl(videoForm.video_url)}
								<span class="field-error">Must be a valid Bunny.net embed URL</span>
							{/if}
						</div>

						<!-- Video Title -->
						<div class="form-group">
							<label for="video_title">Video Title <span class="required">*</span></label>
							<input
								type="text"
								id="video_title"
								bind:value={videoForm.video_title}
								placeholder="Weekly Breakdown: Top Swing Setups"
								minlength="3"
								required
							/>
						</div>

						<!-- Week Title (Optional) -->
						<div class="form-group">
							<label for="week_title">Week Title (Optional)</label>
							<input
								type="text"
								id="week_title"
								bind:value={videoForm.week_title}
								placeholder="Week of {formatWeekDisplay(videoForm.week_of)}"
							/>
						</div>

						<!-- Description (Optional) -->
						<div class="form-group">
							<label for="description">Description (Optional)</label>
							<textarea
								id="description"
								bind:value={videoForm.description}
								placeholder="Complete breakdown of this week's top swing trade opportunities..."
								rows="3"
							></textarea>
						</div>

						<!-- Video Preview -->
						{#if canPreviewVideo}
							<div class="video-preview">
								<h4>Preview</h4>
								<div class="video-frame">
									<iframe
										src={videoForm.video_url}
										title="Video preview"
										frameborder="0"
										allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
									></iframe>
								</div>
							</div>
						{/if}
					</div>
				</section>

				<!-- RIGHT PANEL: Trade Plan Entries -->
				<section class="panel panel--trades">
					<div class="panel-header">
						<h3>
							<svg
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								width="20"
								height="20"
							>
								<rect x="3" y="3" width="18" height="18" rx="2" />
								<path d="M3 9h18M9 21V9" />
							</svg>
							Trade Plan Entries
						</h3>
						<div class="panel-actions">
							<button
								type="button"
								class="action-btn action-btn--secondary"
								onclick={copyFromLastWeek}
								disabled={isLoadingLastWeek}
							>
								{#if isLoadingLastWeek}
									<span class="spinner-small"></span>
								{:else}
									<svg
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="2"
										width="16"
										height="16"
									>
										<rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
										<path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1" />
									</svg>
									Copy Last Week
								{/if}
							</button>
							<span class="entry-count">{validTradePlanCount} entries</span>
						</div>
					</div>

					<div class="panel-content">
						<!-- Trade Plan Table -->
						<div class="trade-sheet-wrapper">
							<table class="trade-sheet">
								<thead>
									<tr>
										<th>Ticker</th>
										<th>Bias</th>
										<th>Entry</th>
										<th>T1</th>
										<th>T2</th>
										<th>T3</th>
										<th>Runner</th>
										<th>Stop</th>
										<th>Options</th>
										<th>Exp</th>
										<th class="notes-th">Notes</th>
										<th class="actions-th"></th>
									</tr>
								</thead>
								<tbody>
									{#each tradePlanRows as row (row.id)}
										<tr class:has-notes-open={row.isExpanded}>
											<td class="ticker-cell">
												<input
													type="text"
													value={row.ticker}
													oninput={(e) =>
														updateRow(row.id, 'ticker', e.currentTarget.value.toUpperCase())}
													placeholder="NVDA"
													maxlength="5"
													class="cell-input cell-input--ticker"
													class:invalid={row.ticker && !isValidTicker(row.ticker)}
												/>
											</td>
											<td>
												<select
													value={row.bias}
													onchange={(e) => updateRow(row.id, 'bias', e.currentTarget.value as Bias)}
													class="cell-select bias-select bias-select--{row.bias.toLowerCase()}"
												>
													<option value="BULLISH">Bullish</option>
													<option value="BEARISH">Bearish</option>
													<option value="NEUTRAL">Neutral</option>
												</select>
											</td>
											<td>
												<input
													type="text"
													value={row.entry}
													oninput={(e) => updateRow(row.id, 'entry', e.currentTarget.value)}
													placeholder="142.50"
													class="cell-input cell-input--price"
												/>
											</td>
											<td>
												<input
													type="text"
													value={row.target1}
													oninput={(e) => updateRow(row.id, 'target1', e.currentTarget.value)}
													placeholder="145"
													class="cell-input cell-input--price"
												/>
											</td>
											<td>
												<input
													type="text"
													value={row.target2}
													oninput={(e) => updateRow(row.id, 'target2', e.currentTarget.value)}
													placeholder="148"
													class="cell-input cell-input--price"
												/>
											</td>
											<td>
												<input
													type="text"
													value={row.target3}
													oninput={(e) => updateRow(row.id, 'target3', e.currentTarget.value)}
													placeholder="152"
													class="cell-input cell-input--price"
												/>
											</td>
											<td>
												<input
													type="text"
													value={row.runner}
													oninput={(e) => updateRow(row.id, 'runner', e.currentTarget.value)}
													placeholder="155+"
													class="cell-input cell-input--price"
												/>
											</td>
											<td>
												<input
													type="text"
													value={row.stop}
													oninput={(e) => updateRow(row.id, 'stop', e.currentTarget.value)}
													placeholder="140"
													class="cell-input cell-input--stop"
												/>
											</td>
											<td>
												<input
													type="text"
													value={row.optionsStrike}
													oninput={(e) => updateRow(row.id, 'optionsStrike', e.currentTarget.value)}
													placeholder="145c"
													class="cell-input cell-input--options"
												/>
											</td>
											<td>
												<input
													type="text"
													value={row.optionsExp}
													oninput={(e) => updateRow(row.id, 'optionsExp', e.currentTarget.value)}
													placeholder="2/7"
													class="cell-input cell-input--exp"
												/>
											</td>
											<td class="notes-toggle-cell">
												<button
													type="button"
													class="table-notes-btn"
													class:expanded={row.isExpanded}
													onclick={() => toggleRowNotes(row.id)}
													aria-label="Toggle notes"
												>
													<svg
														class="chevron-icon"
														viewBox="0 0 24 24"
														fill="none"
														stroke="currentColor"
														stroke-width="2.5"
														width="16"
														height="16"
													>
														<path d="M19 9l-7 7-7-7" />
													</svg>
												</button>
											</td>
											<td class="actions-cell">
												<button
													type="button"
													class="remove-row-btn"
													onclick={() => removeRow(row.id)}
													disabled={tradePlanRows.length <= 1}
													aria-label="Remove row"
												>
													<svg
														viewBox="0 0 24 24"
														fill="none"
														stroke="currentColor"
														stroke-width="2"
														width="16"
														height="16"
													>
														<path d="M18 6L6 18M6 6l12 12" />
													</svg>
												</button>
											</td>
										</tr>
										{#if row.isExpanded}
											<tr class="notes-row expanded">
												<td colspan="12">
													<div class="trade-notes-panel">
														<div class="trade-notes-badge">{row.ticker || 'Notes'}</div>
														<textarea
															value={row.notes}
															oninput={(e) => updateRow(row.id, 'notes', e.currentTarget.value)}
															placeholder="Add trade notes, analysis, or key levels..."
															rows="2"
															class="notes-textarea"
														></textarea>
													</div>
												</td>
											</tr>
										{/if}
									{/each}
								</tbody>
							</table>
						</div>

						<!-- Add Row Button -->
						<button type="button" class="add-row-btn" onclick={addRow}>
							<svg
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								width="18"
								height="18"
							>
								<path d="M12 5v14M5 12h14" />
							</svg>
							Add Entry
						</button>
					</div>
				</section>
			</div>

			<!-- Modal Footer -->
			<footer class="modal-footer">
				<div class="footer-info">
					<span class="week-display">
						Week of {formatWeekDisplay(videoForm.week_of)}
					</span>
					<span class="entries-count">
						{validTradePlanCount} trade plan {validTradePlanCount === 1 ? 'entry' : 'entries'}
					</span>
				</div>
				<div class="footer-actions">
					<button type="button" class="btn btn--secondary" onclick={handleClose}> Cancel </button>
					<button
						type="button"
						class="btn btn--primary"
						onclick={handlePublish}
						disabled={!canPublish}
					>
						{#if isPublishing}
							<span class="spinner-small"></span>
							Publishing...
						{:else}
							<svg
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								width="18"
								height="18"
							>
								<path d="M22 2L11 13M22 2l-7 20-4-9-9-4 20-7z" />
							</svg>
							Publish Weekly Breakdown
						{/if}
					</button>
				</div>
			</footer>
		</div>
	</div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════════════
	   MODAL BACKDROP - Full screen overlay with blur
	   ═══════════════════════════════════════════════════════════════════════════════════ */
	.modal-backdrop {
		position: fixed;
		inset: 0;
		z-index: 10000;
		isolation: isolate;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-2);
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
		animation: fadeIn 0.2s ease;
		overscroll-behavior: contain;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════════
	   MODAL CONTAINER - Main modal box
	   ═══════════════════════════════════════════════════════════════════════════════════ */
	.modal-container {
		position: relative;
		width: 100%;
		max-width: 1200px;
		max-height: calc(100vh - var(--space-4));
		max-height: calc(100dvh - var(--space-4));
		background: linear-gradient(135deg, #f69532 0%, #e8850d 50%, #d4790a 100%);
		border-radius: 16px;
		box-shadow: 0 32px 64px rgba(0, 0, 0, 0.4);
		display: flex;
		flex-direction: column;
		overflow: hidden;
		animation: slideUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
		overscroll-behavior: contain;
	}

	@keyframes slideUp {
		from {
			opacity: 0;
			transform: translateY(20px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════════
	   MODAL HEADER
	   ═══════════════════════════════════════════════════════════════════════════════════ */
	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 20px 24px;
		border-bottom: 1px solid rgba(255, 255, 255, 0.15);
	}

	.modal-header h2 {
		font-family:
			'Montserrat',
			-apple-system,
			BlinkMacSystemFont,
			sans-serif;
		font-size: 20px;
		font-weight: 700;
		color: #fff;
		margin: 0;
	}

	.close-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: var(--touch-target-min);
		height: var(--touch-target-min);
		background: rgba(255, 255, 255, 0.15);
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 10px;
		color: #fff;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.close-btn:hover {
		background: rgba(255, 255, 255, 0.25);
	}

	.close-btn:focus-visible {
		outline: 3px solid rgba(255, 255, 255, 0.8);
		outline-offset: 2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════════
	   MESSAGES - Error & Success
	   ═══════════════════════════════════════════════════════════════════════════════════ */
	.message {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 12px 24px;
		font-size: 14px;
		font-weight: 500;
	}

	.message--error {
		background: rgba(239, 68, 68, 0.95);
		color: #fff;
	}

	.message--success {
		background: rgba(34, 197, 94, 0.95);
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════════
	   MODAL BODY - Split Layout
	   ═══════════════════════════════════════════════════════════════════════════════════ */
	.modal-body {
		display: grid;
		grid-template-columns: 1fr;
		gap: 0;
		flex: 1;
		overflow: hidden;
	}

	@media (min-width: 1024px) {
		.modal-body {
			grid-template-columns: 380px 1fr;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════════
	   PANELS - Video & Trade Plan sections
	   ═══════════════════════════════════════════════════════════════════════════════════ */
	.panel {
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.panel--video {
		background: rgba(255, 255, 255, 0.1);
		border-right: 1px solid rgba(255, 255, 255, 0.1);
	}

	.panel--trades {
		background: #fff;
	}

	.panel-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 20px;
		border-bottom: 1px solid rgba(0, 0, 0, 0.1);
	}

	.panel--video .panel-header {
		border-bottom-color: rgba(255, 255, 255, 0.15);
	}

	.panel-header h3 {
		display: flex;
		align-items: center;
		gap: 10px;
		font-size: 15px;
		font-weight: 700;
		margin: 0;
		color: #333;
	}

	.panel--video .panel-header h3 {
		color: #fff;
	}

	.panel-actions {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.panel-content {
		flex: 1;
		overflow-y: auto;
		padding: 20px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════════
	   FORM ELEMENTS - Video Panel
	   ═══════════════════════════════════════════════════════════════════════════════════ */
	.form-group {
		margin-bottom: 16px;
	}

	.form-group label {
		display: block;
		font-size: 13px;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.9);
		margin-bottom: 6px;
	}

	.required {
		color: #fbbf24;
	}

	.form-group input,
	.form-group textarea {
		width: 100%;
		padding: var(--space-1-5) var(--space-2);
		min-height: var(--touch-target-min);
		background: rgba(255, 255, 255, 0.95);
		border: 2px solid transparent;
		border-radius: 8px;
		font-size: var(--text-base);
		color: #333;
		transition: all 0.2s ease;
	}

	.form-group input:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #143e59;
		box-shadow: 0 0 0 3px rgba(20, 62, 89, 0.2);
	}

	.form-group input.invalid {
		border-color: #ef4444;
		background: #fef2f2;
	}

	.field-error {
		display: block;
		font-size: 12px;
		color: #fef2f2;
		margin-top: 4px;
	}

	.form-group textarea {
		resize: vertical;
		min-height: 80px;
	}

	/* Video Preview */
	.video-preview {
		margin-top: 20px;
	}

	.video-preview h4 {
		font-size: 13px;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.9);
		margin: 0 0 10px 0;
	}

	.video-frame {
		position: relative;
		width: 100%;
		padding-bottom: 56.25%;
		background: #143e59;
		border-radius: 8px;
		overflow: hidden;
	}

	.video-frame iframe {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		border: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════════
	   TRADE SHEET TABLE - Matches WeeklyHero design
	   ═══════════════════════════════════════════════════════════════════════════════════ */
	.trade-sheet-wrapper {
		overflow-x: auto;
		border-radius: 10px;
		border: 1px solid #e5e7eb;
	}

	.trade-sheet {
		width: 100%;
		min-width: 900px;
		border-collapse: collapse;
		font-size: 13px;
	}

	.trade-sheet thead {
		background: #f8fafc;
	}

	.trade-sheet th {
		padding: 12px 8px;
		font-weight: 600;
		color: #64748b;
		text-align: left;
		border-bottom: 2px solid #e5e7eb;
		white-space: nowrap;
	}

	.trade-sheet th:first-child {
		padding-left: 16px;
	}

	.trade-sheet td {
		padding: 6px 4px;
		border-bottom: 1px solid #f1f5f9;
		vertical-align: middle;
	}

	.trade-sheet td:first-child {
		padding-left: 12px;
	}

	.trade-sheet tbody tr:hover {
		background: #f8fafc;
	}

	.trade-sheet tbody tr.has-notes-open {
		background: #fef3c7;
	}

	/* Cell Inputs */
	.cell-input {
		width: 100%;
		padding: 8px 10px;
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		font-size: 13px;
		font-weight: 500;
		color: #333;
		transition: all 0.15s ease;
	}

	.cell-input:focus {
		outline: none;
		border-color: #f69532;
		box-shadow: 0 0 0 3px rgba(246, 149, 50, 0.15);
	}

	.cell-input.invalid {
		border-color: #ef4444;
		background: #fef2f2;
	}

	.cell-input--ticker {
		width: 70px;
		text-transform: uppercase;
		font-weight: 700;
	}

	.cell-input--price {
		width: 70px;
		text-align: right;
	}

	.cell-input--stop {
		width: 70px;
		text-align: right;
		color: #ef4444;
	}

	.cell-input--options {
		width: 65px;
	}

	.cell-input--exp {
		width: 55px;
	}

	/* Bias Select */
	.cell-select {
		padding: 8px 10px;
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		font-size: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.bias-select--bullish {
		color: #16a34a;
		background: #f0fdf4;
		border-color: #bbf7d0;
	}

	.bias-select--bearish {
		color: #dc2626;
		background: #fef2f2;
		border-color: #fecaca;
	}

	.bias-select--neutral {
		color: #6b7280;
		background: #f9fafb;
		border-color: #e5e7eb;
	}

	/* Notes Toggle Button */
	.notes-toggle-cell {
		text-align: center;
	}

	.table-notes-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: #f8fafc;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.table-notes-btn:hover {
		background: #f1f5f9;
		border-color: #cbd5e1;
	}

	.table-notes-btn .chevron-icon {
		transition: transform 0.2s ease;
		color: #64748b;
	}

	.table-notes-btn.expanded .chevron-icon {
		transform: rotate(180deg);
	}

	/* Actions Cell */
	.actions-cell {
		text-align: center;
		width: 40px;
	}

	.remove-row-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.remove-row-btn:hover:not(:disabled) {
		background: #fef2f2;
		color: #ef4444;
	}

	.remove-row-btn:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}

	/* Notes Row */
	.notes-row {
		background: #fffbeb !important;
	}

	.trade-notes-panel {
		padding: 12px 16px;
	}

	.trade-notes-badge {
		display: inline-block;
		padding: 4px 10px;
		background: #f69532;
		color: #fff;
		font-size: 11px;
		font-weight: 700;
		border-radius: 4px;
		margin-bottom: 8px;
	}

	.notes-textarea {
		width: 100%;
		padding: 10px 12px;
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		font-size: 13px;
		color: #333;
		resize: vertical;
		min-height: 60px;
	}

	.notes-textarea:focus {
		outline: none;
		border-color: #f69532;
		box-shadow: 0 0 0 3px rgba(246, 149, 50, 0.15);
	}

	/* Add Row Button */
	.add-row-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		width: 100%;
		padding: 12px;
		margin-top: 16px;
		background: #f8fafc;
		border: 2px dashed #e5e7eb;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.add-row-btn:hover {
		background: #f1f5f9;
		border-color: #cbd5e1;
		color: #475569;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════════
	   ACTION BUTTONS & BADGES
	   ═══════════════════════════════════════════════════════════════════════════════════ */
	.action-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 8px 14px;
		border-radius: 6px;
		font-size: 13px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.action-btn--secondary {
		background: #f8fafc;
		border: 1px solid #e5e7eb;
		color: #475569;
	}

	.action-btn--secondary:hover:not(:disabled) {
		background: #f1f5f9;
		border-color: #cbd5e1;
	}

	.action-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.status-badge {
		padding: 4px 10px;
		border-radius: 9999px;
		font-size: 12px;
		font-weight: 600;
	}

	.status-badge--success {
		background: rgba(34, 197, 94, 0.2);
		color: #16a34a;
	}

	.entry-count {
		font-size: 13px;
		font-weight: 600;
		color: #64748b;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════════
	   MODAL FOOTER
	   ═══════════════════════════════════════════════════════════════════════════════════ */
	.modal-footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 24px;
		background: rgba(0, 0, 0, 0.1);
		border-top: 1px solid rgba(255, 255, 255, 0.1);
	}

	.footer-info {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.week-display {
		font-size: 14px;
		font-weight: 600;
		color: #fff;
	}

	.entries-count {
		font-size: 12px;
		color: rgba(255, 255, 255, 0.8);
	}

	.footer-actions {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-1);
		padding: var(--space-1-5) var(--space-3);
		min-height: var(--touch-target-min);
		border-radius: 10px;
		font-size: var(--text-base);
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn--secondary {
		background: rgba(255, 255, 255, 0.15);
		border: 2px solid rgba(255, 255, 255, 0.3);
		color: #fff;
	}

	.btn--secondary:hover {
		background: rgba(255, 255, 255, 0.25);
		border-color: rgba(255, 255, 255, 0.4);
	}

	.btn--primary {
		background: #143e59;
		border: 2px solid #143e59;
		color: #fff;
	}

	.btn--primary:hover:not(:disabled) {
		background: #0f2d42;
		border-color: #0f2d42;
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.3);
	}

	.btn--primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn:focus-visible {
		outline: 3px solid rgba(255, 255, 255, 0.8);
		outline-offset: 2px;
	}

	/* Spinner */
	.spinner-small {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: #fff;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════════════ */
	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS
	   ═══════════════════════════════════════════════════════════════════════════════════ */

	/* ═══════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS - Mobile-first using design tokens
	   ═══════════════════════════════════════════════════════════════════════ */

	/* Mobile: Bottom sheet style modal */
	@media (max-width: 767px) {
		.modal-backdrop {
			padding: var(--space-1);
			align-items: flex-end;
		}

		.modal-container {
			max-height: calc(100vh - var(--space-2));
			max-height: calc(100dvh - var(--space-2));
			border-radius: 16px 16px 0 0;
		}

		.modal-header {
			padding: var(--space-2);
		}

		.modal-header h2 {
			font-size: var(--text-lg);
		}

		.panel-header {
			padding: var(--space-2);
			flex-wrap: wrap;
			gap: var(--space-1);
		}

		.panel-header h3 {
			font-size: var(--text-sm);
		}

		.panel-actions {
			width: 100%;
			justify-content: space-between;
		}

		.panel-content {
			padding: var(--space-2);
			max-height: 250px;
		}

		.trade-sheet {
			font-size: var(--text-xs);
		}

		.trade-sheet th,
		.trade-sheet td {
			padding: var(--space-1) var(--space-0-5);
		}

		.cell-input {
			padding: var(--space-1);
			font-size: var(--text-xs);
		}

		.cell-input--ticker {
			width: 60px;
		}

		.cell-input--price {
			width: 55px;
		}

		.modal-footer {
			padding: var(--space-2);
			flex-direction: column;
			gap: var(--space-2);
		}

		.footer-info {
			align-items: center;
			text-align: center;
		}

		.footer-actions {
			width: 100%;
			flex-direction: column;
			gap: var(--space-1);
		}

		.footer-actions .btn {
			width: 100%;
		}
	}

	/* Tablet (768px - 1023px) */
	@media (min-width: 768px) and (max-width: 1023px) {
		.modal-body {
			grid-template-columns: 1fr;
		}

		.panel--video {
			border-right: none;
			border-bottom: 1px solid rgba(255, 255, 255, 0.15);
		}

		.panel-content {
			max-height: 300px;
		}

		.panel--trades .panel-content {
			max-height: 400px;
		}

		.modal-footer {
			flex-direction: column;
			gap: var(--space-2);
			text-align: center;
		}

		.footer-info {
			align-items: center;
		}

		.footer-actions {
			width: 100%;
			justify-content: center;
		}
	}

	/* Reduced motion preference */
	@media (prefers-reduced-motion: reduce) {
		.modal-backdrop,
		.modal-container,
		.cell-input,
		.btn,
		.table-notes-btn .chevron-icon {
			animation: none;
			transition: none;
		}
	}
</style>
