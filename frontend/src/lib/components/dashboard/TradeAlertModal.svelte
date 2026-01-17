<script lang="ts">
	/**
	 * TradeAlertModal - Admin component for creating/editing trade alerts
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Features:
	 * - Create entry/exit alerts with TOS format preview
	 * - Support for both options and shares
	 * - Real-time TOS string generation
	 * - Edit existing alerts
	 *
	 * @version 1.0.0
	 * @requires Svelte 5.0+ / SvelteKit 2.0+
	 */

	import type {
		AlertType,
		TradeType,
		TradeAction,
		OptionType,
		OrderType,
		ContractType,
		RoomAlert,
		AlertCreateInput,
		AlertUpdateInput,
		TosStringParams
	} from '$lib/types/trading';
	import { buildTosString, validateTosParams } from '$lib/utils/tos-builder';

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS
	// ═══════════════════════════════════════════════════════════════════════════

	interface Props {
		isOpen: boolean;
		roomSlug: string;
		editAlert?: RoomAlert | null;
		entryAlerts?: RoomAlert[];
		onClose: () => void;
		onSave: (alert: AlertCreateInput | AlertUpdateInput, isEdit: boolean) => Promise<void>;
	}

	let {
		isOpen = $bindable(),
		roomSlug,
		editAlert = null,
		entryAlerts = [],
		onClose,
		onSave
	}: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let isSubmitting = $state(false);
	let errors = $state<string[]>([]);

	// Form fields
	let alertType = $state<AlertType>('ENTRY');
	let ticker = $state('');
	let title = $state('');
	let message = $state('');
	let notes = $state('');
	let tradeType = $state<TradeType>('shares');
	let action = $state<TradeAction>('BUY');
	let quantity = $state<number>(100);
	let optionType = $state<OptionType>('CALL');
	let strike = $state<number | null>(null);
	let expiration = $state<string>('');
	let contractType = $state<ContractType>('Weeklys');
	let orderType = $state<OrderType>('LMT');
	let limitPrice = $state<number | null>(null);
	let fillPrice = $state<number | null>(null);
	let entryAlertId = $state<number | null>(null);
	let isPinned = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const isEdit = $derived(editAlert !== null);

	const tosParams = $derived.by<TosStringParams | null>(() => {
		if (!tradeType || !action || !quantity || !ticker || !orderType) return null;

		return {
			trade_type: tradeType,
			action: action,
			quantity: quantity,
			ticker: ticker.toUpperCase(),
			option_type: tradeType === 'options' ? optionType : undefined,
			strike: tradeType === 'options' && strike ? strike : undefined,
			expiration: tradeType === 'options' && expiration ? expiration : undefined,
			contract_type: tradeType === 'options' ? contractType : undefined,
			order_type: orderType,
			limit_price: orderType === 'LMT' && limitPrice ? limitPrice : undefined
		};
	});

	const tosString = $derived.by(() => {
		const params = tosParams;
		if (!params) return '';
		const validationErrors = validateTosParams(params);
		if (validationErrors.length > 0) return '';
		return buildTosString(params);
	});

	const tosValidationErrors = $derived.by(() => {
		const params = tosParams;
		if (!params) return [];
		return validateTosParams(params);
	});

	// Get available entry alerts for EXIT linking
	const availableEntryAlerts = $derived(
		entryAlerts.filter((a) => a.alert_type === 'ENTRY' && a.ticker.toUpperCase() === ticker.toUpperCase())
	);

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	// Populate form when editing
	$effect(() => {
		if (editAlert) {
			alertType = editAlert.alert_type;
			ticker = editAlert.ticker;
			title = editAlert.title;
			message = editAlert.message;
			notes = editAlert.notes || '';
			tradeType = editAlert.trade_type || 'shares';
			action = editAlert.action || 'BUY';
			quantity = editAlert.quantity || 100;
			optionType = editAlert.option_type || 'CALL';
			strike = editAlert.strike;
			expiration = editAlert.expiration || '';
			contractType = editAlert.contract_type || 'Weeklys';
			orderType = editAlert.order_type || 'LMT';
			limitPrice = editAlert.limit_price;
			fillPrice = editAlert.fill_price;
			entryAlertId = editAlert.entry_alert_id;
			isPinned = editAlert.is_pinned;
		} else {
			resetForm();
		}
	});

	// Auto-generate title based on alert type and ticker
	$effect(() => {
		if (!isEdit && ticker && alertType && !title) {
			const actionWord = alertType === 'ENTRY' ? 'Opening' : alertType === 'EXIT' ? 'Closing' : 'Update:';
			const direction = action === 'BUY' ? 'Long' : 'Short';
			title = `${ticker.toUpperCase()} ${alertType === 'UPDATE' ? 'Update' : `${actionWord} ${direction} Position`}`;
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function resetForm() {
		alertType = 'ENTRY';
		ticker = '';
		title = '';
		message = '';
		notes = '';
		tradeType = 'shares';
		action = 'BUY';
		quantity = 100;
		optionType = 'CALL';
		strike = null;
		expiration = '';
		contractType = 'Weeklys';
		orderType = 'LMT';
		limitPrice = null;
		fillPrice = null;
		entryAlertId = null;
		isPinned = false;
		errors = [];
	}

	function validateForm(): boolean {
		const newErrors: string[] = [];

		if (!ticker.trim()) newErrors.push('Ticker is required');
		if (!title.trim()) newErrors.push('Title is required');
		if (!message.trim()) newErrors.push('Message is required');
		if (!quantity || quantity <= 0) newErrors.push('Quantity must be greater than 0');

		if (tradeType === 'options') {
			if (!strike || strike <= 0) newErrors.push('Strike price is required for options');
			if (!expiration) newErrors.push('Expiration date is required for options');
		}

		if (orderType === 'LMT' && (!limitPrice || limitPrice <= 0)) {
			newErrors.push('Limit price is required for limit orders');
		}

		if (alertType === 'EXIT' && !entryAlertId && availableEntryAlerts.length > 0) {
			newErrors.push('Please select the entry alert this exit is related to');
		}

		errors = newErrors;
		return newErrors.length === 0;
	}

	async function handleSubmit() {
		if (!validateForm()) return;

		isSubmitting = true;

		try {
			const alertData: AlertCreateInput | AlertUpdateInput = {
				...(isEdit ? {} : { room_slug: roomSlug }),
				alert_type: alertType,
				ticker: ticker.toUpperCase(),
				title,
				message,
				notes: notes || undefined,
				trade_type: tradeType,
				action,
				quantity,
				option_type: tradeType === 'options' ? optionType : undefined,
				strike: tradeType === 'options' ? strike ?? undefined : undefined,
				expiration: tradeType === 'options' ? expiration || undefined : undefined,
				contract_type: tradeType === 'options' ? contractType : undefined,
				order_type: orderType,
				limit_price: orderType === 'LMT' ? limitPrice ?? undefined : undefined,
				fill_price: fillPrice ?? undefined,
				entry_alert_id: alertType === 'EXIT' ? entryAlertId ?? undefined : undefined,
				is_pinned: isPinned
			};

			await onSave(alertData, isEdit);
			handleClose();
		} catch (err) {
			errors = [err instanceof Error ? err.message : 'Failed to save alert'];
		} finally {
			isSubmitting = false;
		}
	}

	function handleClose() {
		resetForm();
		onClose();
	}

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			handleClose();
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			handleClose();
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
	<div class="modal-backdrop" onclick={handleBackdropClick} role="dialog" aria-modal="true">
		<div class="modal-content">
			<div class="modal-header">
				<h2>{isEdit ? 'Edit Alert' : 'Create New Alert'}</h2>
				<button class="close-btn" onclick={handleClose} aria-label="Close">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="24" height="24">
						<path d="M6 18L18 6M6 6l12 12" />
					</svg>
				</button>
			</div>

			<form class="modal-body" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
				{#if errors.length > 0}
					<div class="error-banner">
						{#each errors as error}
							<p>{error}</p>
						{/each}
					</div>
				{/if}

				<!-- Alert Type & Ticker Row -->
				<div class="form-row">
					<div class="form-group">
						<label for="alertType">Alert Type</label>
						<select id="alertType" bind:value={alertType}>
							<option value="ENTRY">ENTRY</option>
							<option value="EXIT">EXIT</option>
							<option value="UPDATE">UPDATE</option>
						</select>
					</div>

					<div class="form-group">
						<label for="ticker">Ticker</label>
						<input
							type="text"
							id="ticker"
							bind:value={ticker}
							placeholder="AAPL"
							class="uppercase"
						/>
					</div>

					<div class="form-group checkbox-group">
						<label>
							<input type="checkbox" bind:checked={isPinned} />
							Pin Alert
						</label>
					</div>
				</div>

				<!-- Title -->
				<div class="form-group">
					<label for="title">Title</label>
					<input type="text" id="title" bind:value={title} placeholder="Alert title..." />
				</div>

				<!-- Trade Type Selection -->
				<div class="trade-type-selector">
					<button
						type="button"
						class="type-btn"
						class:active={tradeType === 'shares'}
						onclick={() => (tradeType = 'shares')}
					>
						Shares
					</button>
					<button
						type="button"
						class="type-btn"
						class:active={tradeType === 'options'}
						onclick={() => (tradeType = 'options')}
					>
						Options
					</button>
				</div>

				<!-- Trade Details Row -->
				<div class="form-row">
					<div class="form-group">
						<label for="action">Action</label>
						<select id="action" bind:value={action}>
							<option value="BUY">BUY</option>
							<option value="SELL">SELL</option>
						</select>
					</div>

					<div class="form-group">
						<label for="quantity">Quantity</label>
						<input
							type="number"
							id="quantity"
							bind:value={quantity}
							min="1"
							placeholder={tradeType === 'options' ? 'Contracts' : 'Shares'}
						/>
					</div>

					<div class="form-group">
						<label for="orderType">Order Type</label>
						<select id="orderType" bind:value={orderType}>
							<option value="LMT">LIMIT</option>
							<option value="MKT">MARKET</option>
						</select>
					</div>

					{#if orderType === 'LMT'}
						<div class="form-group">
							<label for="limitPrice">Limit Price</label>
							<input
								type="number"
								id="limitPrice"
								bind:value={limitPrice}
								step="0.01"
								min="0.01"
								placeholder="0.00"
							/>
						</div>
					{/if}
				</div>

				<!-- Options-specific fields -->
				{#if tradeType === 'options'}
					<div class="options-section">
						<h3>Options Details</h3>
						<div class="form-row">
							<div class="form-group">
								<label for="optionType">Option Type</label>
								<select id="optionType" bind:value={optionType}>
									<option value="CALL">CALL</option>
									<option value="PUT">PUT</option>
								</select>
							</div>

							<div class="form-group">
								<label for="strike">Strike</label>
								<input
									type="number"
									id="strike"
									bind:value={strike}
									step="0.5"
									min="0.5"
									placeholder="Strike price"
								/>
							</div>

							<div class="form-group">
								<label for="expiration">Expiration</label>
								<input type="date" id="expiration" bind:value={expiration} />
							</div>

							<div class="form-group">
								<label for="contractType">Contract Type</label>
								<select id="contractType" bind:value={contractType}>
									<option value="Weeklys">Weeklys</option>
									<option value="Monthly">Monthly</option>
									<option value="Standard">Standard</option>
									<option value="LEAPS">LEAPS</option>
								</select>
							</div>
						</div>
					</div>
				{/if}

				<!-- TOS String Preview -->
				<div class="tos-preview" class:valid={tosString && tosValidationErrors.length === 0}>
					<label>TOS String Preview</label>
					<div class="tos-string">
						{#if tosString}
							<code>{tosString}</code>
						{:else if tosValidationErrors.length > 0}
							<span class="tos-error">{tosValidationErrors[0]}</span>
						{:else}
							<span class="tos-placeholder">Fill in trade details to generate TOS string...</span>
						{/if}
					</div>
				</div>

				<!-- Fill Price (optional) -->
				<div class="form-group">
					<label for="fillPrice">Fill Price (Optional - actual execution price)</label>
					<input
						type="number"
						id="fillPrice"
						bind:value={fillPrice}
						step="0.01"
						min="0.01"
						placeholder="Leave blank if not filled yet"
					/>
				</div>

				<!-- Entry Alert Link (for EXIT alerts) -->
				{#if alertType === 'EXIT' && availableEntryAlerts.length > 0}
					<div class="form-group">
						<label for="entryAlertId">Link to Entry Alert</label>
						<select id="entryAlertId" bind:value={entryAlertId}>
							<option value={null}>-- Select Entry Alert --</option>
							{#each availableEntryAlerts as entry}
								<option value={entry.id}>{entry.title} ({entry.tos_string})</option>
							{/each}
						</select>
					</div>
				{/if}

				<!-- Message -->
				<div class="form-group">
					<label for="message">Message</label>
					<textarea
						id="message"
						bind:value={message}
						rows="3"
						placeholder="Detailed alert message for subscribers..."
					></textarea>
				</div>

				<!-- Notes -->
				<div class="form-group">
					<label for="notes">Internal Notes (Optional)</label>
					<textarea
						id="notes"
						bind:value={notes}
						rows="2"
						placeholder="Notes visible to admins only..."
					></textarea>
				</div>
			</form>

			<div class="modal-footer">
				<button type="button" class="btn btn-secondary" onclick={handleClose} disabled={isSubmitting}>
					Cancel
				</button>
				<button
					type="button"
					class="btn btn-primary"
					onclick={handleSubmit}
					disabled={isSubmitting || tosValidationErrors.length > 0}
				>
					{#if isSubmitting}
						Saving...
					{:else}
						{isEdit ? 'Update Alert' : 'Publish Alert'}
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.modal-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 20px;
		backdrop-filter: blur(4px);
	}

	.modal-content {
		background: #fff;
		border-radius: 16px;
		width: 100%;
		max-width: 700px;
		max-height: 90vh;
		overflow: hidden;
		display: flex;
		flex-direction: column;
		box-shadow: 0 25px 50px rgba(0, 0, 0, 0.25);
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 20px 24px;
		border-bottom: 1px solid #e5e7eb;
		background: linear-gradient(135deg, #143e59 0%, #1a5276 100%);
	}

	.modal-header h2 {
		font-size: 20px;
		font-weight: 700;
		margin: 0;
		color: #fff;
		font-family: 'Montserrat', sans-serif;
	}

	.close-btn {
		background: rgba(255, 255, 255, 0.15);
		border: none;
		padding: 8px;
		border-radius: 8px;
		cursor: pointer;
		color: #fff;
		transition: background 0.2s;
	}

	.close-btn:hover {
		background: rgba(255, 255, 255, 0.25);
	}

	.modal-body {
		padding: 24px;
		overflow-y: auto;
		flex: 1;
	}

	.error-banner {
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 8px;
		padding: 12px 16px;
		margin-bottom: 20px;
	}

	.error-banner p {
		color: #991b1b;
		margin: 0;
		font-size: 14px;
	}

	.error-banner p + p {
		margin-top: 4px;
	}

	.form-row {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
		gap: 16px;
		margin-bottom: 20px;
	}

	.form-group {
		margin-bottom: 16px;
	}

	.form-group label {
		display: block;
		font-size: 13px;
		font-weight: 600;
		color: #374151;
		margin-bottom: 6px;
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		width: 100%;
		padding: 10px 12px;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		font-size: 14px;
		transition: all 0.2s;
		background: #fff;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #143e59;
		box-shadow: 0 0 0 3px rgba(20, 62, 89, 0.1);
	}

	.form-group input.uppercase {
		text-transform: uppercase;
	}

	.checkbox-group {
		display: flex;
		align-items: center;
	}

	.checkbox-group label {
		display: flex;
		align-items: center;
		gap: 8px;
		cursor: pointer;
		margin-bottom: 0;
	}

	.checkbox-group input[type='checkbox'] {
		width: 18px;
		height: 18px;
		cursor: pointer;
	}

	.trade-type-selector {
		display: flex;
		gap: 8px;
		margin-bottom: 20px;
	}

	.type-btn {
		flex: 1;
		padding: 12px 20px;
		border: 2px solid #e5e7eb;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 600;
		background: #f9fafb;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.2s;
	}

	.type-btn:hover {
		border-color: #143e59;
		color: #143e59;
	}

	.type-btn.active {
		background: #143e59;
		border-color: #143e59;
		color: #fff;
	}

	.options-section {
		background: #f8fafc;
		border: 1px solid #e2e8f0;
		border-radius: 12px;
		padding: 20px;
		margin-bottom: 20px;
	}

	.options-section h3 {
		font-size: 14px;
		font-weight: 700;
		color: #143e59;
		margin: 0 0 16px 0;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.options-section .form-row {
		margin-bottom: 0;
	}

	.options-section .form-group {
		margin-bottom: 0;
	}

	.tos-preview {
		background: #1a1a2e;
		border-radius: 12px;
		padding: 16px;
		margin-bottom: 20px;
	}

	.tos-preview.valid {
		border: 2px solid #22c55e;
	}

	.tos-preview label {
		display: block;
		font-size: 11px;
		font-weight: 600;
		color: #94a3b8;
		margin-bottom: 8px;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.tos-string {
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
		min-height: 40px;
		display: flex;
		align-items: center;
	}

	.tos-string code {
		color: #22c55e;
		font-size: 15px;
		font-weight: 600;
		word-break: break-all;
	}

	.tos-string .tos-error {
		color: #ef4444;
		font-size: 13px;
	}

	.tos-string .tos-placeholder {
		color: #64748b;
		font-size: 13px;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		padding: 16px 24px;
		border-top: 1px solid #e5e7eb;
		background: #f9fafb;
	}

	.btn {
		padding: 12px 24px;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn-secondary {
		background: #fff;
		border: 1px solid #d1d5db;
		color: #374151;
	}

	.btn-secondary:hover:not(:disabled) {
		background: #f3f4f6;
	}

	.btn-primary {
		background: linear-gradient(135deg, #f69532 0%, #e8860d 100%);
		color: #fff;
		box-shadow: 0 4px 12px rgba(246, 149, 50, 0.3);
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 6px 16px rgba(246, 149, 50, 0.4);
	}

	.btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	@media (max-width: 640px) {
		.modal-content {
			max-height: 95vh;
		}

		.form-row {
			grid-template-columns: 1fr;
		}
	}
</style>
