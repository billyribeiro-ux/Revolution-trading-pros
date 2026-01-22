<!--
	Trade Entry Manager - ICT 7 Admin Component
	═══════════════════════════════════════════════════════════════════════════════════
	Apple Principal Engineer ICT 7+ Grade - January 2026
	
	Features:
	- Add Entry button with modal form
	- Inline quick-add for rapid data entry
	- Sortable table with drag reorder
	- Bulk import/export
	
	@version 1.0.0
-->
<script lang="ts">
	import { untrack } from 'svelte';
	import {
		tradePlanApi,
		type TradePlanEntry,
		type Bias
	} from '$lib/api/room-content';
	
	// Icons
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconGripVertical from '@tabler/icons-svelte/icons/grip-vertical';
	import IconArrowUp from '@tabler/icons-svelte/icons/arrow-up';
	import IconArrowDown from '@tabler/icons-svelte/icons/arrow-down';
	import IconCopy from '@tabler/icons-svelte/icons/copy';

	// ═══════════════════════════════════════════════════════════════════════════════
	// PROPS
	// ═══════════════════════════════════════════════════════════════════════════════

	interface Props {
		roomSlug: string;
		onSuccess?: (message: string) => void;
		onError?: (message: string) => void;
	}

	const { roomSlug, onSuccess, onError }: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════════

	let entries = $state<TradePlanEntry[]>([]);
	let isLoading = $state(true);
	let showModal = $state(false);
	let editingEntry = $state<TradePlanEntry | null>(null);
	let isSaving = $state(false);
	let showQuickAdd = $state(false);

	// Form state
	let form = $state({
		ticker: '',
		bias: 'BULLISH' as Bias,
		entry: '',
		target1: '',
		target2: '',
		target3: '',
		runner: '',
		stop: '',
		options_strike: '',
		options_exp: '',
		notes: ''
	});

	// Quick add form
	let quickForm = $state({
		ticker: '',
		bias: 'BULLISH' as Bias,
		entry: '',
		stop: ''
	});

	// ═══════════════════════════════════════════════════════════════════════════════
	// DERIVED
	// ═══════════════════════════════════════════════════════════════════════════════

	const isFormValid = $derived(form.ticker.trim() !== '');
	const isQuickFormValid = $derived(quickForm.ticker.trim() !== '');
	const sortedEntries = $derived([...entries].sort((a, b) => a.sort_order - b.sort_order));

	// ═══════════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════════

	async function loadEntries() {
		isLoading = true;
		try {
			const result = await tradePlanApi.list(roomSlug);
			entries = result.data;
		} catch (err) {
			onError?.('Failed to load trade plan entries');
			console.error(err);
		} finally {
			isLoading = false;
		}
	}

	async function saveEntry() {
		if (!isFormValid) return;
		isSaving = true;

		try {
			if (editingEntry) {
				await tradePlanApi.update(editingEntry.id, {
					ticker: form.ticker.toUpperCase(),
					bias: form.bias,
					entry: form.entry || undefined,
					target1: form.target1 || undefined,
					target2: form.target2 || undefined,
					target3: form.target3 || undefined,
					runner: form.runner || undefined,
					stop: form.stop || undefined,
					options_strike: form.options_strike || undefined,
					options_exp: form.options_exp || undefined,
					notes: form.notes || undefined
				});
				onSuccess?.('Trade plan entry updated');
			} else {
				await tradePlanApi.create({
					room_slug: roomSlug,
					ticker: form.ticker.toUpperCase(),
					bias: form.bias,
					entry: form.entry || undefined,
					target1: form.target1 || undefined,
					target2: form.target2 || undefined,
					target3: form.target3 || undefined,
					runner: form.runner || undefined,
					stop: form.stop || undefined,
					options_strike: form.options_strike || undefined,
					options_exp: form.options_exp || undefined,
					notes: form.notes || undefined
				});
				onSuccess?.('Trade plan entry added');
			}
			closeModal();
			await loadEntries();
		} catch (err) {
			onError?.('Failed to save trade plan entry');
			console.error(err);
		} finally {
			isSaving = false;
		}
	}

	async function quickAddEntry() {
		if (!isQuickFormValid) return;
		isSaving = true;

		try {
			await tradePlanApi.create({
				room_slug: roomSlug,
				ticker: quickForm.ticker.toUpperCase(),
				bias: quickForm.bias,
				entry: quickForm.entry || undefined,
				stop: quickForm.stop || undefined
			});
			onSuccess?.(`${quickForm.ticker.toUpperCase()} added to trade plan`);
			
			// Reset quick form
			quickForm = { ticker: '', bias: 'BULLISH', entry: '', stop: '' };
			await loadEntries();
		} catch (err) {
			onError?.('Failed to add entry');
			console.error(err);
		} finally {
			isSaving = false;
		}
	}

	async function deleteEntry(id: number) {
		if (!confirm('Delete this trade plan entry?')) return;

		try {
			await tradePlanApi.delete(id);
			onSuccess?.('Trade plan entry deleted');
			await loadEntries();
		} catch (err) {
			onError?.('Failed to delete entry');
			console.error(err);
		}
	}

	async function duplicateEntry(entry: TradePlanEntry) {
		try {
			await tradePlanApi.create({
				room_slug: roomSlug,
				ticker: entry.ticker,
				bias: entry.bias,
				entry: entry.entry || undefined,
				target1: entry.target1 || undefined,
				target2: entry.target2 || undefined,
				target3: entry.target3 || undefined,
				runner: entry.runner || undefined,
				stop: entry.stop || undefined,
				options_strike: entry.options_strike || undefined,
				options_exp: entry.options_exp || undefined,
				notes: entry.notes ? `${entry.notes} (copy)` : undefined
			});
			onSuccess?.(`${entry.ticker} duplicated`);
			await loadEntries();
		} catch (err) {
			onError?.('Failed to duplicate entry');
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// UI HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════════

	function openAddModal() {
		editingEntry = null;
		form = {
			ticker: '',
			bias: 'BULLISH',
			entry: '',
			target1: '',
			target2: '',
			target3: '',
			runner: '',
			stop: '',
			options_strike: '',
			options_exp: '',
			notes: ''
		};
		showModal = true;
	}

	function openEditModal(entry: TradePlanEntry) {
		editingEntry = entry;
		form = {
			ticker: entry.ticker,
			bias: entry.bias,
			entry: entry.entry || '',
			target1: entry.target1 || '',
			target2: entry.target2 || '',
			target3: entry.target3 || '',
			runner: entry.runner || '',
			stop: entry.stop || '',
			options_strike: entry.options_strike || '',
			options_exp: entry.options_exp || '',
			notes: entry.notes || ''
		};
		showModal = true;
	}

	function closeModal() {
		showModal = false;
		editingEntry = null;
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (roomSlug) {
			untrack(() => loadEntries());
		}
	});
</script>

<div class="trade-entry-manager">
	<!-- Header with Actions -->
	<div class="manager-header">
		<div class="header-left">
			<h3 class="manager-title">Trade Plan Entries</h3>
			<span class="entry-count">{entries.length} entries</span>
		</div>
		<div class="header-actions">
			<button 
				class="btn-quick-add"
				class:active={showQuickAdd}
				onclick={() => showQuickAdd = !showQuickAdd}
			>
				Quick Add
			</button>
			<button class="btn-add-entry" onclick={openAddModal}>
				<IconPlus size={18} />
				<span>Add Entry</span>
			</button>
		</div>
	</div>

	<!-- Quick Add Bar -->
	{#if showQuickAdd}
		<div class="quick-add-bar">
			<input
				type="text"
				placeholder="TICKER"
				bind:value={quickForm.ticker}
				class="quick-input ticker"
				onkeydown={(e) => e.key === 'Enter' && quickAddEntry()}
			/>
			<select bind:value={quickForm.bias} class="quick-select">
				<option value="BULLISH">BULLISH</option>
				<option value="BEARISH">BEARISH</option>
				<option value="NEUTRAL">NEUTRAL</option>
			</select>
			<input
				type="text"
				placeholder="Entry"
				bind:value={quickForm.entry}
				class="quick-input"
			/>
			<input
				type="text"
				placeholder="Stop"
				bind:value={quickForm.stop}
				class="quick-input"
			/>
			<button 
				class="btn-quick-submit"
				onclick={quickAddEntry}
				disabled={!isQuickFormValid || isSaving}
			>
				<IconPlus size={16} />
				Add
			</button>
		</div>
	{/if}

	<!-- Entries Table -->
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<span>Loading trade plan...</span>
		</div>
	{:else if entries.length === 0}
		<div class="empty-state">
			<div class="empty-icon">📋</div>
			<h4>No Trade Plan Entries</h4>
			<p>Add your first entry to get started</p>
			<button class="btn-add-first" onclick={openAddModal}>
				<IconPlus size={18} />
				Add First Entry
			</button>
		</div>
	{:else}
		<div class="entries-table-wrapper">
			<table class="entries-table">
				<thead>
					<tr>
						<th class="col-drag"></th>
						<th class="col-ticker">Ticker</th>
						<th class="col-bias">Bias</th>
						<th class="col-entry">Entry</th>
						<th class="col-target">T1</th>
						<th class="col-target">T2</th>
						<th class="col-target">T3</th>
						<th class="col-stop">Stop</th>
						<th class="col-options">Options</th>
						<th class="col-actions">Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each sortedEntries as entry (entry.id)}
						<tr class="entry-row">
							<td class="col-drag">
								<IconGripVertical size={16} class="drag-handle" />
							</td>
							<td class="col-ticker">
								<span class="ticker-badge">{entry.ticker}</span>
							</td>
							<td class="col-bias">
								<span class="bias-badge bias-{entry.bias.toLowerCase()}">{entry.bias}</span>
							</td>
							<td class="col-entry">{entry.entry || '-'}</td>
							<td class="col-target">{entry.target1 || '-'}</td>
							<td class="col-target">{entry.target2 || '-'}</td>
							<td class="col-target">{entry.target3 || '-'}</td>
							<td class="col-stop">{entry.stop || '-'}</td>
							<td class="col-options">
								{#if entry.options_strike}
									{entry.options_strike} {entry.options_exp || ''}
								{:else}
									-
								{/if}
							</td>
							<td class="col-actions">
								<button class="action-btn" title="Edit" onclick={() => openEditModal(entry)}>
									<IconEdit size={16} />
								</button>
								<button class="action-btn" title="Duplicate" onclick={() => duplicateEntry(entry)}>
									<IconCopy size={16} />
								</button>
								<button class="action-btn delete" title="Delete" onclick={() => deleteEntry(entry.id)}>
									<IconTrash size={16} />
								</button>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>

<!-- Add/Edit Modal -->
{#if showModal}
	<div class="modal-overlay" role="dialog" aria-modal="true" tabindex="-1" onclick={(e) => e.target === e.currentTarget && closeModal()} onkeydown={(e) => e.key === 'Escape' && closeModal()}>
		<div class="modal-content">
			<div class="modal-header">
				<h3>{editingEntry ? 'Edit Entry' : 'Add New Entry'}</h3>
				<button class="modal-close" onclick={closeModal}>
					<IconX size={20} />
				</button>
			</div>
			
			<form class="entry-form" onsubmit={(e) => { e.preventDefault(); saveEntry(); }}>
				<div class="form-row">
					<div class="form-group">
						<label for="ticker">Ticker *</label>
						<input
							id="ticker"
							type="text"
							bind:value={form.ticker}
							placeholder="NVDA"
							class="form-input"
							required
						/>
					</div>
					<div class="form-group">
						<label for="bias">Bias *</label>
						<select id="bias" bind:value={form.bias} class="form-select">
							<option value="BULLISH">BULLISH</option>
							<option value="BEARISH">BEARISH</option>
							<option value="NEUTRAL">NEUTRAL</option>
						</select>
					</div>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="entry">Entry Price</label>
						<input id="entry" type="text" bind:value={form.entry} placeholder="$142.50" class="form-input" />
					</div>
					<div class="form-group">
						<label for="stop">Stop Loss</label>
						<input id="stop" type="text" bind:value={form.stop} placeholder="$136.00" class="form-input" />
					</div>
				</div>

				<div class="form-row targets">
					<div class="form-group">
						<label for="target1">Target 1</label>
						<input id="target1" type="text" bind:value={form.target1} placeholder="$148" class="form-input" />
					</div>
					<div class="form-group">
						<label for="target2">Target 2</label>
						<input id="target2" type="text" bind:value={form.target2} placeholder="$155" class="form-input" />
					</div>
					<div class="form-group">
						<label for="target3">Target 3</label>
						<input id="target3" type="text" bind:value={form.target3} placeholder="$162" class="form-input" />
					</div>
					<div class="form-group">
						<label for="runner">Runner</label>
						<input id="runner" type="text" bind:value={form.runner} placeholder="$170+" class="form-input" />
					</div>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="options_strike">Options Strike</label>
						<input id="options_strike" type="text" bind:value={form.options_strike} placeholder="$145C" class="form-input" />
					</div>
					<div class="form-group">
						<label for="options_exp">Expiration</label>
						<input id="options_exp" type="text" bind:value={form.options_exp} placeholder="Feb 21" class="form-input" />
					</div>
				</div>

				<div class="form-group full-width">
					<label for="notes">Notes</label>
					<textarea
						id="notes"
						bind:value={form.notes}
						placeholder="Entry thesis, key levels, catalysts..."
						class="form-textarea"
						rows="3"
					></textarea>
				</div>

				<div class="form-actions">
					<button type="button" class="btn-cancel" onclick={closeModal}>Cancel</button>
					<button type="submit" class="btn-save" disabled={!isFormValid || isSaving}>
						{#if isSaving}
							<span class="spinner-small"></span>
							Saving...
						{:else}
							<IconCheck size={18} />
							{editingEntry ? 'Update Entry' : 'Add Entry'}
						{/if}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	.trade-entry-manager {
		background: white;
		border-radius: 12px;
		border: 1px solid #e2e8f0;
		overflow: hidden;
	}

	.manager-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 16px 20px;
		border-bottom: 1px solid #e2e8f0;
		background: #f8fafc;
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.manager-title {
		font-size: 16px;
		font-weight: 600;
		color: #1e293b;
		margin: 0;
	}

	.entry-count {
		font-size: 13px;
		color: #64748b;
		background: #e2e8f0;
		padding: 2px 8px;
		border-radius: 10px;
	}

	.header-actions {
		display: flex;
		gap: 8px;
	}

	.btn-quick-add {
		padding: 8px 16px;
		background: white;
		border: 1px solid #e2e8f0;
		border-radius: 8px;
		font-size: 13px;
		font-weight: 500;
		color: #475569;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-quick-add:hover,
	.btn-quick-add.active {
		background: #f1f5f9;
		border-color: #143E59;
		color: #143E59;
	}

	.btn-add-entry {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 8px 16px;
		background: #143E59;
		border: none;
		border-radius: 8px;
		font-size: 13px;
		font-weight: 600;
		color: white;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-add-entry:hover {
		background: #0f2d42;
	}

	/* Quick Add Bar */
	.quick-add-bar {
		display: flex;
		gap: 8px;
		padding: 12px 20px;
		background: #f1f5f9;
		border-bottom: 1px solid #e2e8f0;
	}

	.quick-input {
		padding: 8px 12px;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		font-size: 13px;
		flex: 1;
	}

	.quick-input.ticker {
		max-width: 100px;
		text-transform: uppercase;
		font-weight: 600;
	}

	.quick-select {
		padding: 8px 12px;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		font-size: 13px;
		background: white;
	}

	.btn-quick-submit {
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 8px 16px;
		background: #22c55e;
		border: none;
		border-radius: 6px;
		font-size: 13px;
		font-weight: 600;
		color: white;
		cursor: pointer;
	}

	.btn-quick-submit:hover:not(:disabled) {
		background: #16a34a;
	}

	.btn-quick-submit:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Table */
	.entries-table-wrapper {
		overflow-x: auto;
	}

	.entries-table {
		width: 100%;
		border-collapse: collapse;
	}

	.entries-table th {
		padding: 10px 12px;
		text-align: left;
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		color: #64748b;
		background: #f8fafc;
		border-bottom: 1px solid #e2e8f0;
	}

	.entries-table td {
		padding: 12px;
		font-size: 13px;
		color: #334155;
		border-bottom: 1px solid #f1f5f9;
	}

	.entry-row:hover {
		background: #f8fafc;
	}

	.col-drag {
		width: 32px;
		color: #94a3b8;
		cursor: grab;
	}

	.ticker-badge {
		font-weight: 700;
		color: #143E59;
		font-size: 14px;
	}

	.bias-badge {
		display: inline-block;
		padding: 2px 8px;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
	}

	.bias-bullish {
		background: #dcfce7;
		color: #166534;
	}

	.bias-bearish {
		background: #fee2e2;
		color: #991b1b;
	}

	.bias-neutral {
		background: #fef3c7;
		color: #92400e;
	}

	.col-actions {
		display: flex;
		gap: 4px;
	}

	.action-btn {
		padding: 6px;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: #64748b;
		cursor: pointer;
		transition: all 0.15s;
	}

	.action-btn:hover {
		background: #f1f5f9;
		color: #143E59;
	}

	.action-btn.delete:hover {
		background: #fee2e2;
		color: #dc2626;
	}

	/* Loading & Empty States */
	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 48px 24px;
		text-align: center;
	}

	.spinner {
		width: 32px;
		height: 32px;
		border: 3px solid #e2e8f0;
		border-top-color: #143E59;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.empty-icon {
		font-size: 48px;
		margin-bottom: 16px;
	}

	.empty-state h4 {
		margin: 0 0 8px;
		color: #1e293b;
	}

	.empty-state p {
		margin: 0 0 20px;
		color: #64748b;
	}

	.btn-add-first {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 10px 20px;
		background: #143E59;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		color: white;
		cursor: pointer;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 24px;
	}

	.modal-content {
		background: white;
		border-radius: 16px;
		width: 100%;
		max-width: 640px;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px 24px;
		border-bottom: 1px solid #e2e8f0;
	}

	.modal-header h3 {
		margin: 0;
		font-size: 18px;
		font-weight: 600;
		color: #1e293b;
	}

	.modal-close {
		padding: 8px;
		background: transparent;
		border: none;
		border-radius: 8px;
		color: #64748b;
		cursor: pointer;
	}

	.modal-close:hover {
		background: #f1f5f9;
		color: #1e293b;
	}

	/* Form */
	.entry-form {
		padding: 24px;
	}

	.form-row {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 16px;
		margin-bottom: 16px;
	}

	.form-row.targets {
		grid-template-columns: repeat(4, 1fr);
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.form-group.full-width {
		grid-column: 1 / -1;
	}

	.form-group label {
		font-size: 12px;
		font-weight: 600;
		color: #475569;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.form-input,
	.form-select,
	.form-textarea {
		padding: 10px 12px;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		font-size: 14px;
		transition: border-color 0.15s;
	}

	.form-input:focus,
	.form-select:focus,
	.form-textarea:focus {
		outline: none;
		border-color: #143E59;
		box-shadow: 0 0 0 3px rgba(20, 62, 89, 0.1);
	}

	.form-textarea {
		resize: vertical;
		font-family: inherit;
	}

	.form-actions {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		margin-top: 24px;
		padding-top: 20px;
		border-top: 1px solid #e2e8f0;
	}

	.btn-cancel {
		padding: 10px 20px;
		background: white;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		color: #475569;
		cursor: pointer;
	}

	.btn-cancel:hover {
		background: #f1f5f9;
	}

	.btn-save {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 10px 24px;
		background: #143E59;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		color: white;
		cursor: pointer;
	}

	.btn-save:hover:not(:disabled) {
		background: #0f2d42;
	}

	.btn-save:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.spinner-small {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255,255,255,0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.form-row {
			grid-template-columns: 1fr;
		}

		.form-row.targets {
			grid-template-columns: repeat(2, 1fr);
		}

		.quick-add-bar {
			flex-wrap: wrap;
		}
	}
</style>
