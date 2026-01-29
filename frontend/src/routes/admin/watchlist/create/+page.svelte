<!--
	/admin/watchlist/create - Create Weekly Watchlist Entry
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- Auto-generates slug from trader name and date
	- Dynamic date switcher table builder
	- Real-time preview
	- Full validation with error handling
	- Room targeting support
	- Dark theme (RTP design system)
	- Full Svelte 5 $state/$derived reactivity
-->

<script lang="ts">
	/**
	 * Admin Weekly Watchlist - Create Entry
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Automated, easy-to-use system for creating watchlist entries.
	 * Uses centralized watchlistApi for all operations.
	 *
	 * @version 2.0.0 (January 2026) - Fixed Svelte 5 runes, API integration
	 */

	import { goto } from '$app/navigation';
	import { watchlistApi, type WatchlistDate } from '$lib/api/watchlist';
	import { ALL_ROOM_IDS } from '$lib/config/rooms';
	import RoomSelector from '$lib/components/admin/RoomSelector.svelte';
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconDeviceFloppy from '@tabler/icons-svelte/icons/device-floppy';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconTable from '@tabler/icons-svelte/icons/table';
	import IconCalendar from '@tabler/icons-svelte/icons/calendar';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconAlertCircle from '@tabler/icons-svelte/icons/alert-circle';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	// Form state
	let trader = $state('');
	let weekOf = $state('');
	let videoUrl = $state('');
	let posterUrl = $state('');
	let spreadsheetUrl = $state('');
	let description = $state('');
	let status = $state<'published' | 'draft' | 'archived'>('draft');
	let rooms = $state<string[]>([...ALL_ROOM_IDS]);

	// Date switcher state
	let watchlistDates = $state<WatchlistDate[]>([]);
	let newDate = $state('');
	let newDateSpreadsheet = $state('');

	// UI state
	let isSubmitting = $state(false);
	let error = $state('');
	let success = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE (Svelte 5 correct syntax)
	// ═══════════════════════════════════════════════════════════════════════════

	// Auto-generate slug from trader and weekOf
	const slug = $derived.by(() => {
		if (!trader || !weekOf) return '';
		const date = new Date(weekOf);
		if (isNaN(date.getTime())) return '';
		const month = String(date.getMonth() + 1).padStart(2, '0');
		const day = String(date.getDate()).padStart(2, '0');
		const year = date.getFullYear();
		const traderSlug = trader
			.toLowerCase()
			.replace(/[^a-z0-9]+/g, '-')
			.replace(/^-+|-+$/g, '');
		return `${month}${day}${year}-${traderSlug}`;
	});

	// Auto-generate title from trader
	const title = $derived.by(() => {
		if (!trader) return '';
		return `Weekly Watchlist with ${trader}`;
	});

	// Auto-generate description if not provided
	const autoDescription = $derived.by(() => {
		if (!weekOf) return '';
		const date = new Date(weekOf);
		if (isNaN(date.getTime())) return '';
		return `Week starting on ${date.toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' })}.`;
	});

	// Form validation
	const isFormValid = $derived.by(() => {
		return (
			trader.trim() !== '' &&
			weekOf !== '' &&
			videoUrl.trim() !== '' &&
			spreadsheetUrl.trim() !== '' &&
			rooms.length > 0
		);
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// EVENT HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════

	function addDateVersion() {
		if (!newDate || !newDateSpreadsheet) {
			error = 'Both date and spreadsheet URL are required';
			return;
		}

		watchlistDates = [
			...watchlistDates,
			{
				date: newDate,
				spreadsheetUrl: newDateSpreadsheet
			}
		];

		newDate = '';
		newDateSpreadsheet = '';
		error = '';
	}

	function removeDateVersion(index: number) {
		watchlistDates = watchlistDates.filter((_, i) => i !== index);
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();

		if (!isFormValid) {
			error = 'Please fill in all required fields and select at least one room';
			return;
		}

		isSubmitting = true;
		error = '';

		try {
			const response = await watchlistApi.create({
				title: title,
				trader,
				weekOf,
				slug: slug || undefined,
				videoSrc: videoUrl,
				videoPoster: posterUrl || undefined,
				spreadsheetSrc: spreadsheetUrl,
				watchlistDates: watchlistDates.length > 0 ? watchlistDates : undefined,
				description: description || autoDescription,
				status,
				rooms
			});

			if (response.success) {
				success = true;
				setTimeout(() => {
					goto(`/admin/watchlist/${response.data.slug}/edit`);
				}, 1500);
			} else {
				error = 'Failed to create watchlist entry';
			}
		} catch (err) {
			console.error('Create failed:', err);
			error = err instanceof Error ? err.message : 'Network error. Please try again.';
		} finally {
			isSubmitting = false;
		}
	}
</script>

<svelte:head>
	<title>Create Weekly Watchlist | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class="create-page">
	<!-- Header -->
	<header class="page-header">
		<div class="header-left">
			<a href="/admin/watchlist" class="back-btn">
				<IconArrowLeft size={20} />
				Back
			</a>
			<div class="header-titles">
				<h1>Create Weekly Watchlist</h1>
				<p class="subtitle">Add a new watchlist entry for members</p>
			</div>
		</div>
		<div class="header-actions">
			<button
				type="button"
				class="btn-primary"
				onclick={handleSubmit}
				disabled={isSubmitting || !isFormValid}
			>
				<IconDeviceFloppy size={18} />
				{isSubmitting ? 'Creating...' : 'Create Watchlist'}
			</button>
		</div>
	</header>

	<!-- Success Message -->
	{#if success}
		<div class="alert alert-success">
			<IconCheck size={20} />
			<span>Watchlist entry created successfully! Redirecting...</span>
		</div>
	{/if}

	<!-- Error Message -->
	{#if error}
		<div class="alert alert-error">
			<IconAlertCircle size={20} />
			<span>{error}</span>
		</div>
	{/if}

	<!-- Form Container -->
	<div class="form-container">
		<form onsubmit={handleSubmit}>
			<!-- Basic Info Section -->
			<section class="form-section">
				<h2 class="section-title">Basic Information</h2>
				<div class="form-grid">
					<div class="form-group">
						<label for="trader">Trader Name <span class="required">*</span></label>
						<input
							id="trader" name="trader"
							type="text"
							bind:value={trader}
							placeholder="e.g., TG Watkins"
							required
						/>
						<small>Full name of the trader presenting this watchlist</small>
					</div>

					<div class="form-group">
						<label for="weekOf">Week Of <span class="required">*</span></label>
						<input id="weekOf" name="weekOf" type="date" bind:value={weekOf} required />
						<small>Monday of the week this watchlist covers</small>
					</div>

					<div class="form-group">
						<!-- svelte-ignore a11y_label_has_associated_control -->
						<label>Auto-Generated Slug</label>
						<div class="preview-field">
							<code>{slug || 'Will be generated...'}</code>
						</div>
						<small>URL: /watchlist/{slug || '[slug]'}</small>
					</div>

					<div class="form-group">
						<!-- svelte-ignore a11y_label_has_associated_control -->
						<label>Auto-Generated Title</label>
						<div class="preview-field">
							{title || 'Will be generated...'}
						</div>
					</div>

					<div class="form-group">
						<label for="status">Status</label>
						<select id="status" bind:value={status}>
							<option value="draft">Draft</option>
							<option value="published">Published</option>
							<option value="archived">Archived</option>
						</select>
						<small>Draft entries won't appear on the frontend</small>
					</div>

					<div class="form-group full-width">
						<label for="description">Description</label>
						<textarea
							id="description"
							bind:value={description}
							placeholder={autoDescription || 'Optional description (auto-generated if left blank)'}
							rows="2"
						></textarea>
					</div>
				</div>
			</section>

			<!-- Video Section -->
			<section class="form-section">
				<div class="section-header">
					<IconVideo size={20} />
					<h2 class="section-title">Video</h2>
				</div>
				<div class="form-grid">
					<div class="form-group full-width">
						<label for="videoUrl">Video URL <span class="required">*</span></label>
						<input
							id="videoUrl" name="videoUrl"
							type="url"
							bind:value={videoUrl}
							placeholder="https://cloud-streaming.s3.amazonaws.com/..."
							required
						/>
						<small>S3 URL to the MP4 video file</small>
					</div>

					<div class="form-group full-width">
						<label for="posterUrl">Poster Image URL</label>
						<input
							id="posterUrl" name="posterUrl"
							type="url"
							bind:value={posterUrl}
							placeholder="https://cdn.simplertrading.com/..."
						/>
						<small>Thumbnail image shown before video plays (optional)</small>
					</div>
				</div>

				<!-- Video Preview -->
				{#if posterUrl}
					<div class="media-preview">
						<p class="preview-label">Video Poster Preview</p>
						<img src={posterUrl} alt="Video poster preview" />
					</div>
				{/if}
			</section>

			<!-- Spreadsheet Section -->
			<section class="form-section">
				<div class="section-header">
					<IconTable size={20} />
					<h2 class="section-title">Spreadsheet</h2>
				</div>
				<div class="form-grid">
					<div class="form-group full-width">
						<label for="spreadsheetUrl"
							>Default Spreadsheet URL <span class="required">*</span></label
						>
						<input
							id="spreadsheetUrl" name="spreadsheetUrl"
							type="url"
							bind:value={spreadsheetUrl}
							placeholder="https://docs.google.com/spreadsheets/..."
							required
						/>
						<small>Google Sheets embed URL (published to web)</small>
					</div>
				</div>
			</section>

			<!-- Date Switcher Section -->
			<section class="form-section">
				<div class="section-header">
					<IconCalendar size={20} />
					<h2 class="section-title">Date Switcher (Optional)</h2>
				</div>
				<p class="section-description">
					Add multiple date versions if this watchlist has historical data. Users can switch between
					dates using tabs below the spreadsheet.
				</p>

				{#if watchlistDates.length > 0}
					<div class="date-list">
						{#each watchlistDates as dateItem, index}
							<div class="date-item">
								<div class="date-info">
									<strong>{dateItem.date}</strong>
									<span class="url-preview">{dateItem.spreadsheetUrl.substring(0, 50)}...</span>
								</div>
								<button
									type="button"
									class="btn-icon-danger"
									onclick={() => removeDateVersion(index)}
									title="Remove date version"
								>
									<IconTrash size={16} />
								</button>
							</div>
						{/each}
					</div>
				{/if}

				<div class="date-adder">
					<div class="date-adder-row">
						<div class="form-group">
							<label for="newDate">Date Label</label>
							<input type="text" id="newDate" name="newDate" bind:value={newDate} placeholder="e.g., 1/3/2026" />
						</div>

						<div class="form-group flex-grow">
							<label for="newDateSpreadsheet">Spreadsheet URL</label>
							<input
								type="url"
								id="newDateSpreadsheet" name="newDateSpreadsheet"
								bind:value={newDateSpreadsheet}
								placeholder="https://docs.google.com/..."
							/>
						</div>

						<button type="button" class="btn-add" onclick={addDateVersion}>
							<IconPlus size={16} />
							Add Date
						</button>
					</div>
				</div>
			</section>

			<!-- Room Targeting Section -->
			<section class="form-section">
				<h2 class="section-title">Room Targeting</h2>
				<p class="section-description">
					Select which rooms and services can access this watchlist.
				</p>
				<RoomSelector bind:selectedRooms={rooms} />
			</section>
		</form>
	</div>
</div>

<style>
	.create-page {
		padding: 32px;
		max-width: 1000px;
		margin: 0 auto;
	}

	/* Header */
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 32px;
		gap: 20px;
	}

	.header-left {
		display: flex;
		align-items: flex-start;
		gap: 16px;
	}

	.back-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 8px 12px;
		background: #334155;
		border-radius: 8px;
		color: #94a3b8;
		text-decoration: none;
		font-size: 0.875rem;
		transition: all 0.2s;
	}

	.back-btn:hover {
		background: #475569;
		color: white;
	}

	.header-titles h1 {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 700;
		color: white;
	}

	.subtitle {
		margin: 4px 0 0;
		font-size: 0.875rem;
		color: #64748b;
	}

	.header-actions {
		display: flex;
		gap: 12px;
		flex-shrink: 0;
	}

	/* Buttons */
	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 600;
		text-decoration: none;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(249, 115, 22, 0.3);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
		transform: none;
	}

	.btn-add {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 10px 16px;
		background: #334155;
		border: 1px solid #475569;
		border-radius: 8px;
		color: white;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		white-space: nowrap;
		align-self: flex-end;
	}

	.btn-add:hover {
		background: #475569;
		border-color: #64748b;
	}

	.btn-icon-danger {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 6px;
		color: #ef4444;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon-danger:hover {
		background: rgba(239, 68, 68, 0.2);
		border-color: #ef4444;
	}

	/* Alerts */
	.alert {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 14px 18px;
		border-radius: 8px;
		margin-bottom: 24px;
		font-size: 0.875rem;
	}

	.alert-success {
		background: rgba(34, 197, 94, 0.1);
		border: 1px solid rgba(34, 197, 94, 0.3);
		color: #22c55e;
	}

	.alert-error {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #ef4444;
	}

	/* Form Container */
	.form-container {
		background: #1e293b;
		border-radius: 12px;
		border: 1px solid #334155;
		overflow: hidden;
	}

	.form-section {
		padding: 24px;
		border-bottom: 1px solid #334155;
	}

	.form-section:last-child {
		border-bottom: none;
	}

	.section-header {
		display: flex;
		align-items: center;
		gap: 10px;
		margin-bottom: 16px;
		color: #94a3b8;
	}

	.section-header .section-title {
		margin: 0;
	}

	.section-title {
		margin: 0 0 16px;
		font-size: 1rem;
		font-weight: 600;
		color: white;
	}

	.section-description {
		margin: -8px 0 16px;
		font-size: 0.875rem;
		color: #64748b;
	}

	.form-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 16px;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.form-group.full-width {
		grid-column: 1 / -1;
	}

	.form-group.flex-grow {
		flex: 1;
	}

	.form-group label {
		font-size: 0.8rem;
		font-weight: 500;
		color: #94a3b8;
	}

	.required {
		color: #ef4444;
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		padding: 10px 14px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 8px;
		color: white;
		font-size: 0.875rem;
		font-family: inherit;
		transition: border-color 0.2s;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #f97316;
	}

	.form-group textarea {
		resize: vertical;
	}

	.form-group small {
		font-size: 0.75rem;
		color: #64748b;
	}

	.preview-field {
		padding: 10px 14px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 8px;
		font-size: 0.875rem;
		color: #94a3b8;
	}

	.preview-field code {
		color: #f97316;
		font-weight: 600;
		background: none;
		padding: 0;
	}

	/* Media Preview */
	.media-preview {
		margin-top: 16px;
		padding: 16px;
		background: #0f172a;
		border-radius: 8px;
	}

	.preview-label {
		margin: 0 0 12px;
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
	}

	.media-preview img {
		max-width: 100%;
		max-height: 200px;
		border-radius: 6px;
	}

	/* Date Switcher */
	.date-list {
		display: flex;
		flex-direction: column;
		gap: 8px;
		margin-bottom: 16px;
	}

	.date-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 12px 16px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 8px;
	}

	.date-info {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.date-info strong {
		color: white;
		font-size: 0.875rem;
	}

	.url-preview {
		color: #64748b;
		font-size: 0.75rem;
		font-family: monospace;
	}

	.date-adder {
		background: #0f172a;
		padding: 16px;
		border-radius: 8px;
		border: 1px dashed #334155;
	}

	.date-adder-row {
		display: flex;
		gap: 12px;
		align-items: flex-end;
	}

	.date-adder-row .form-group:first-child {
		width: 150px;
		flex-shrink: 0;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.create-page {
			padding: 16px;
		}

		.page-header {
			flex-direction: column;
		}

		.header-actions {
			width: 100%;
		}

		.header-actions .btn-primary {
			width: 100%;
			justify-content: center;
		}

		.form-grid {
			grid-template-columns: 1fr;
		}

		.date-adder-row {
			flex-direction: column;
			align-items: stretch;
		}

		.date-adder-row .form-group:first-child {
			width: 100%;
		}

		.btn-add {
			width: 100%;
			justify-content: center;
		}
	}
</style>
