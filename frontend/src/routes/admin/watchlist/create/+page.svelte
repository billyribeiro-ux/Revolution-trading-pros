<!--
	URL: /admin/watchlist/create
	
	Weekly Watchlist Admin - Create Entry
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Grade - January 2026
	
	Automated, easy-to-use system for creating watchlist entries
	Features:
	- Auto-generates slug from trader name and date
	- Dynamic date switcher table builder
	- Real-time preview
	- Validation
	
	@version 1.0.0
-->
<script lang="ts">
	import { goto } from '$app/navigation';
	import RtpIcon from '$lib/components/icons/RtpIcon.svelte';
	
	// Form state
	let trader = $state('');
	let weekOf = $state('');
	let videoUrl = $state('');
	let posterUrl = $state('');
	let spreadsheetUrl = $state('');
	let description = $state('');
	let status = $state<'published' | 'draft' | 'archived'>('draft');
	
	// Date switcher state
	let watchlistDates = $state<Array<{ date: string; spreadsheetUrl: string }>>([]);
	let newDate = $state('');
	let newDateSpreadsheet = $state('');
	
	// UI state
	let isSubmitting = $state(false);
	let error = $state('');
	let success = $state(false);
	
	// Auto-generate slug
	const slug = $derived(() => {
		if (!trader || !weekOf) return '';
		const date = new Date(weekOf);
		const month = String(date.getMonth() + 1).padStart(2, '0');
		const day = String(date.getDate()).padStart(2, '0');
		const year = date.getFullYear();
		const traderSlug = trader.toLowerCase().replace(/\s+/g, '-');
		return `${month}${day}${year}-${traderSlug}`;
	});
	
	// Auto-generate title
	const title = $derived(() => {
		if (!trader) return '';
		return `Weekly Watchlist with ${trader}`;
	});
	
	function addDateVersion() {
		if (!newDate || !newDateSpreadsheet) {
			error = 'Both date and spreadsheet URL are required';
			return;
		}
		
		watchlistDates = [...watchlistDates, { 
			date: newDate, 
			spreadsheetUrl: newDateSpreadsheet 
		}];
		
		newDate = '';
		newDateSpreadsheet = '';
		error = '';
	}
	
	function removeDateVersion(index: number) {
		watchlistDates = watchlistDates.filter((_, i) => i !== index);
	}
	
	async function handleSubmit(e: Event) {
		e.preventDefault();
		
		if (!trader || !weekOf || !videoUrl || !spreadsheetUrl) {
			error = 'Please fill in all required fields';
			return;
		}
		
		isSubmitting = true;
		error = '';
		
		try {
			const response = await fetch('/api/watchlist', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					'Authorization': 'Bearer mock-token' // TODO: Replace with real auth
				},
				body: JSON.stringify({
					title: title(),
					trader,
					weekOf,
					videoSrc: videoUrl,
					videoPoster: posterUrl || undefined,
					spreadsheetSrc: spreadsheetUrl,
					watchlistDates: watchlistDates.length > 0 ? watchlistDates : undefined,
					description: description || `Week starting on ${new Date(weekOf).toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' })}.`,
					status
				})
			});
			
			const data = await response.json();
			
			if (data.success) {
				success = true;
				setTimeout(() => {
					goto(`/watchlist/${data.data.slug}`);
				}, 1500);
			} else {
				error = data.message || 'Failed to create watchlist entry';
			}
		} catch (err) {
			error = 'Network error. Please try again.';
		} finally {
			isSubmitting = false;
		}
	}
</script>

<svelte:head>
	<title>Create Weekly Watchlist - Admin</title>
</svelte:head>

<div class="admin-page">
	<div class="admin-header">
		<h1>Create Weekly Watchlist Entry</h1>
		<a href="/admin/watchlist" class="btn-secondary">
			<RtpIcon name="arrow-left" size={16} />
			Back to List
		</a>
	</div>
	
	<div class="admin-content">
		<form onsubmit={handleSubmit} class="watchlist-form">
			<!-- Basic Info -->
			<section class="form-section">
				<h2>Basic Information</h2>
				
				<div class="form-group">
					<label for="trader">
						Trader Name <span class="required">*</span>
					</label>
					<input 
						type="text" 
						id="trader" 
						bind:value={trader}
						placeholder="e.g., Melissa Beegle"
						required
					/>
					<small>Full name of the trader presenting this watchlist</small>
				</div>
				
				<div class="form-group">
					<label for="weekOf">
						Week Of <span class="required">*</span>
					</label>
					<input 
						type="date" 
						id="weekOf" 
						bind:value={weekOf}
						required
					/>
					<small>Monday of the week this watchlist covers</small>
				</div>
				
				<div class="form-group">
					<div class="label-text">Auto-Generated Slug</div>
					<div class="slug-preview">
						<code>{slug() || 'Will be generated...'}</code>
					</div>
					<small>URL: /watchlist/{slug() || '[slug]'}</small>
				</div>
				
				<div class="form-group">
					<div class="label-text">Auto-Generated Title</div>
					<div class="slug-preview">
						{title() || 'Will be generated...'}
					</div>
				</div>
			</section>
			
			<!-- Video Info -->
			<section class="form-section">
				<h2>Video</h2>
				
				<div class="form-group">
					<label for="videoUrl">
						Video URL <span class="required">*</span>
					</label>
					<input 
						type="url" 
						id="videoUrl" 
						bind:value={videoUrl}
						placeholder="https://cloud-streaming.s3.amazonaws.com/..."
						required
					/>
					<small>S3 URL to the MP4 video file</small>
				</div>
				
				<div class="form-group">
					<label for="posterUrl">Poster Image URL</label>
					<input 
						type="url" 
						id="posterUrl" 
						bind:value={posterUrl}
						placeholder="https://cdn.simplertrading.com/..."
					/>
					<small>Thumbnail image shown before video plays (optional)</small>
				</div>
			</section>
			
			<!-- Spreadsheet Info -->
			<section class="form-section">
				<h2>Spreadsheet</h2>
				
				<div class="form-group">
					<label for="spreadsheetUrl">
						Default Spreadsheet URL <span class="required">*</span>
					</label>
					<input 
						type="url" 
						id="spreadsheetUrl" 
						bind:value={spreadsheetUrl}
						placeholder="https://docs.google.com/spreadsheets/..."
						required
					/>
					<small>Google Sheets embed URL (published to web)</small>
				</div>
			</section>
			
			<!-- Date Switcher -->
			<section class="form-section">
				<h2>Date Switcher (Optional)</h2>
				<p class="section-description">
					Add multiple date versions if this watchlist has historical data. 
					Users can switch between dates using tabs below the spreadsheet.
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
									class="btn-remove"
									onclick={() => removeDateVersion(index)}
								>
									<RtpIcon name="x" size={16} />
								</button>
							</div>
						{/each}
					</div>
				{/if}
				
				<div class="date-adder">
					<div class="form-row">
						<div class="form-group">
							<label for="newDate">Date Label</label>
							<input 
								type="text" 
								id="newDate" 
								bind:value={newDate}
								placeholder="e.g., 1/3/2026"
							/>
						</div>
						
						<div class="form-group">
							<label for="newDateSpreadsheet">Spreadsheet URL</label>
							<input 
								type="url" 
								id="newDateSpreadsheet" 
								bind:value={newDateSpreadsheet}
								placeholder="https://docs.google.com/..."
							/>
						</div>
						
						<button 
							type="button" 
							class="btn-add"
							onclick={addDateVersion}
						>
							<RtpIcon name="plus" size={16} />
							Add Date
						</button>
					</div>
				</div>
			</section>
			
			<!-- Additional Info -->
			<section class="form-section">
				<h2>Additional Information</h2>
				
				<div class="form-group">
					<label for="description">Description</label>
					<textarea 
						id="description" 
						bind:value={description}
						rows="3"
						placeholder="Optional description (auto-generated if left blank)"
					></textarea>
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
			</section>
			
			<!-- Error/Success Messages -->
			{#if error}
				<div class="alert alert-error">
					<RtpIcon name="alert-circle" size={20} />
					{error}
				</div>
			{/if}
			
			{#if success}
				<div class="alert alert-success">
					<RtpIcon name="check-circle" size={20} />
					Watchlist entry created successfully! Redirecting...
				</div>
			{/if}
			
			<!-- Submit -->
			<div class="form-actions">
				<button 
					type="submit" 
					class="btn-primary"
					disabled={isSubmitting}
				>
					{#if isSubmitting}
						Creating...
					{:else}
						Create Watchlist Entry
					{/if}
				</button>
				
				<a href="/admin/watchlist" class="btn-secondary">Cancel</a>
			</div>
		</form>
	</div>
</div>

<style>
	.admin-page {
		max-width: 900px;
		margin: 0 auto;
		padding: 40px 20px;
	}

	.admin-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 30px;
	}

	.admin-header h1 {
		font-size: 28px;
		font-weight: 700;
		color: #333;
		margin: 0;
	}

	.admin-content {
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		padding: 30px;
	}

	.watchlist-form {
		display: flex;
		flex-direction: column;
		gap: 30px;
	}

	.form-section {
		border-bottom: 1px solid #e5e5e5;
		padding-bottom: 30px;
	}

	.form-section:last-of-type {
		border-bottom: none;
		padding-bottom: 0;
	}

	.form-section h2 {
		font-size: 18px;
		font-weight: 600;
		color: #143E59;
		margin: 0 0 15px;
	}

	.section-description {
		color: #666;
		font-size: 14px;
		margin: 0 0 20px;
	}

	.form-group {
		margin-bottom: 20px;
	}

	.form-group label,
	.form-group .label-text {
		display: block;
		font-weight: 600;
		color: #333;
		margin-bottom: 8px;
		font-size: 14px;
	}

	.required {
		color: #e74c3c;
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		width: 100%;
		padding: 10px 12px;
		border: 1px solid #ddd;
		border-radius: 5px;
		font-size: 14px;
		font-family: inherit;
		transition: border-color 0.2s;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #143E59;
	}

	.form-group small {
		display: block;
		color: #666;
		font-size: 12px;
		margin-top: 5px;
	}

	.slug-preview {
		padding: 10px 12px;
		background: #f5f5f5;
		border: 1px solid #e5e5e5;
		border-radius: 5px;
		font-size: 14px;
		color: #333;
	}

	.slug-preview code {
		background: none;
		padding: 0;
		color: #143E59;
		font-weight: 600;
	}

	/* Date Switcher */
	.date-list {
		display: flex;
		flex-direction: column;
		gap: 10px;
		margin-bottom: 20px;
	}

	.date-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 12px 15px;
		background: #f9f9f9;
		border: 1px solid #e5e5e5;
		border-radius: 5px;
	}

	.date-info {
		display: flex;
		flex-direction: column;
		gap: 5px;
	}

	.date-info strong {
		color: #333;
		font-size: 14px;
	}

	.url-preview {
		color: #666;
		font-size: 12px;
		font-family: monospace;
	}

	.btn-remove {
		background: #e74c3c;
		color: #fff;
		border: none;
		border-radius: 5px;
		padding: 8px;
		cursor: pointer;
		transition: background 0.2s;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.btn-remove:hover {
		background: #c0392b;
	}

	.date-adder {
		background: #f9f9f9;
		padding: 20px;
		border-radius: 5px;
		border: 1px dashed #ddd;
	}

	.form-row {
		display: grid;
		grid-template-columns: 150px 1fr auto;
		gap: 15px;
		align-items: end;
	}

	.btn-add {
		background: #143E59;
		color: #fff;
		border: none;
		border-radius: 5px;
		padding: 10px 20px;
		cursor: pointer;
		font-weight: 600;
		font-size: 14px;
		display: flex;
		align-items: center;
		gap: 8px;
		transition: background 0.2s;
		white-space: nowrap;
	}

	.btn-add:hover {
		background: #0f2d41;
	}

	/* Alerts */
	.alert {
		padding: 15px 20px;
		border-radius: 5px;
		display: flex;
		align-items: center;
		gap: 12px;
		font-size: 14px;
	}

	.alert-error {
		background: #fee;
		color: #c0392b;
		border: 1px solid #e74c3c;
	}

	.alert-success {
		background: #efe;
		color: #27ae60;
		border: 1px solid #2ecc71;
	}

	/* Form Actions */
	.form-actions {
		display: flex;
		gap: 15px;
		padding-top: 20px;
		border-top: 1px solid #e5e5e5;
	}

	.btn-primary,
	.btn-secondary {
		padding: 12px 24px;
		border-radius: 5px;
		font-weight: 600;
		font-size: 14px;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
		display: inline-flex;
		align-items: center;
		gap: 8px;
	}

	.btn-primary {
		background: #143E59;
		color: #fff;
		border: none;
	}

	.btn-primary:hover:not(:disabled) {
		background: #0f2d41;
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: #f5f5f5;
		color: #333;
		border: 1px solid #ddd;
	}

	.btn-secondary:hover {
		background: #e8e8e8;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.admin-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 15px;
		}

		.admin-content {
			padding: 20px;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.form-actions {
			flex-direction: column;
		}

		.btn-primary,
		.btn-secondary {
			width: 100%;
			justify-content: center;
		}
	}
</style>
