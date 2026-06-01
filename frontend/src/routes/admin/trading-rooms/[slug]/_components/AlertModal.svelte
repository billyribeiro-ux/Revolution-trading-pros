<!--
	AlertModal — Add/Edit trading alert modal with optional TOS-format fields.
	Extracted from +page.svelte (R8-C) — 1 $bindable form prop, 2 callback props.
-->
<script lang="ts">
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import type { AlertType, RoomAlert } from '$lib/api/room-content';

	export interface AlertFormData {
		alert_type: AlertType;
		ticker: string;
		title: string;
		message: string;
		notes: string;
		trade_type: 'options' | 'shares' | '';
		action: 'BUY' | 'SELL' | '';
		quantity: string;
		option_type: 'CALL' | 'PUT' | '';
		strike: string;
		expiration: string;
		contract_type: 'Weeklys' | 'Monthly' | 'LEAPS' | '';
		order_type: 'MKT' | 'LMT' | '';
		limit_price: string;
		fill_price: string;
		tos_string: string;
		is_new: boolean;
		is_published: boolean;
	}

	interface Props {
		open: boolean;
		editingAlert: RoomAlert | null;
		form: AlertFormData;
		isSaving: boolean;
		isValid: boolean;
		onSave: () => void;
		onClose: () => void;
	}

	let {
		open,
		editingAlert,
		form = $bindable(),
		isSaving,
		isValid,
		onSave,
		onClose
	}: Props = $props();
</script>

{#if open}
	<div
		class="modal-overlay"
		onclick={onClose}
		onkeydown={(e) => e.key === 'Escape' && onClose()}
		aria-hidden="true"
	>
		<div
			class="modal"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			aria-labelledby="alert-modal-title"
			tabindex="-1"
		>
			<div class="modal-header">
				<h2 id="alert-modal-title">{editingAlert ? 'Edit' : 'Create'} Alert</h2>
				<button class="close-btn" onclick={onClose}>
					<IconX size={24} />
				</button>
			</div>
			<form
				class="modal-body"
				onsubmit={(e) => {
					e.preventDefault();
					onSave();
				}}
			>
				<div class="form-row">
					<div class="form-group">
						<label for="alert_type">Type *</label>
						<select id="alert_type" bind:value={form.alert_type}>
							<option value="ENTRY">ENTRY</option>
							<option value="EXIT">EXIT</option>
							<option value="UPDATE">UPDATE</option>
						</select>
					</div>
					<div class="form-group">
						<label for="alert_ticker">Ticker *</label>
						<input
							id="alert_ticker"
							name="alert_ticker"
							type="text"
							bind:value={form.ticker}
							placeholder="NVDA"
							style="text-transform: uppercase"
						/>
					</div>
				</div>

				<div class="form-group full-width">
					<label for="alert_title">Title *</label>
					<input
						id="alert_title"
						name="alert_title"
						type="text"
						bind:value={form.title}
						placeholder="Opening NVDA Swing Position"
					/>
				</div>

				<div class="form-group full-width">
					<label for="alert_message">Message *</label>
					<textarea
						id="alert_message"
						bind:value={form.message}
						placeholder="Entering NVDA at $142.50. First target $148, stop at $136..."
						rows="3"
					></textarea>
				</div>

				<div class="form-group full-width">
					<label for="alert_notes">Detailed Notes (for dropdown)</label>
					<textarea
						id="alert_notes"
						bind:value={form.notes}
						placeholder="Entry based on breakout above $142 resistance with strong volume confirmation..."
						rows="4"
					></textarea>
				</div>

				<!-- TOS Format Section -->
				{#if form.alert_type === 'ENTRY' || form.alert_type === 'EXIT'}
					<div class="tos-section">
						<h4 class="section-title">TOS Format (Optional)</h4>
						<div class="form-row">
							<div class="form-group">
								<label for="trade_type">Trade Type</label>
								<select id="trade_type" bind:value={form.trade_type}>
									<option value="">Select...</option>
									<option value="options">Options</option>
									<option value="shares">Shares</option>
								</select>
							</div>
							<div class="form-group">
								<label for="action">Action</label>
								<select id="action" bind:value={form.action}>
									<option value="">Select...</option>
									<option value="BUY">BUY</option>
									<option value="SELL">SELL</option>
								</select>
							</div>
							<div class="form-group">
								<label for="quantity">Quantity</label>
								<input
									id="quantity"
									name="quantity"
									type="number"
									bind:value={form.quantity}
									placeholder="10"
								/>
							</div>
						</div>

						{#if form.trade_type === 'options'}
							<div class="form-row">
								<div class="form-group">
									<label for="option_type">Option Type</label>
									<select id="option_type" bind:value={form.option_type}>
										<option value="">Select...</option>
										<option value="CALL">CALL</option>
										<option value="PUT">PUT</option>
									</select>
								</div>
								<div class="form-group">
									<label for="strike">Strike</label>
									<input
										id="strike"
										name="strike"
										type="number"
										step="0.5"
										bind:value={form.strike}
										placeholder="145"
									/>
								</div>
								<div class="form-group">
									<label for="expiration">Expiration</label>
									<input
										id="expiration"
										name="expiration"
										type="date"
										bind:value={form.expiration}
									/>
								</div>
							</div>
							<div class="form-row">
								<div class="form-group">
									<label for="contract_type">Contract Type</label>
									<select id="contract_type" bind:value={form.contract_type}>
										<option value="">Select...</option>
										<option value="Weeklys">Weeklys</option>
										<option value="Monthly">Monthly</option>
										<option value="LEAPS">LEAPS</option>
									</select>
								</div>
								<div class="form-group">
									<label for="order_type">Order Type</label>
									<select id="order_type" bind:value={form.order_type}>
										<option value="">Select...</option>
										<option value="MKT">Market (MKT)</option>
										<option value="LMT">Limit (LMT)</option>
									</select>
								</div>
							</div>
						{/if}

						<div class="form-row">
							<div class="form-group">
								<label for="limit_price">Limit Price</label>
								<input
									id="limit_price"
									name="limit_price"
									type="number"
									step="0.01"
									bind:value={form.limit_price}
									placeholder="2.50"
								/>
							</div>
							<div class="form-group">
								<label for="fill_price">Fill Price</label>
								<input
									id="fill_price"
									name="fill_price"
									type="number"
									step="0.01"
									bind:value={form.fill_price}
									placeholder="2.48"
								/>
							</div>
						</div>

						<div class="form-group full-width">
							<label for="tos_string">TOS String (auto-generated or manual)</label>
							<input
								id="tos_string"
								name="tos_string"
								type="text"
								bind:value={form.tos_string}
								placeholder="BUY +10 NVDA 100 (Weeklys) 17 JAN 25 145 CALL @2.50 LMT"
							/>
						</div>
					</div>
				{/if}

				<div class="form-row checkboxes">
					<label class="checkbox-label">
						<input
							id="alert-is-new"
							name="alert-is-new"
							type="checkbox"
							bind:checked={form.is_new}
						/>
						<span>Mark as NEW</span>
					</label>
					<label class="checkbox-label">
						<input
							id="alert-is-published"
							name="alert-is-published"
							type="checkbox"
							bind:checked={form.is_published}
						/>
						<span>Published</span>
					</label>
				</div>

				<div class="modal-actions">
					<button type="button" class="btn-secondary" onclick={onClose}>Cancel</button>
					<button type="submit" class="btn-primary" disabled={isSaving || !isValid}>
						{isSaving ? 'Saving...' : editingAlert ? 'Update' : 'Create Alert'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	/* Modal */
	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: var(--z-modal, 400);
		padding: var(--space-2, 20px);
		isolation: isolate;
	}

	.modal {
		background: #fff;
		border-radius: 16px;
		width: 100%;
		max-width: 640px;
		max-height: 90vh;
		overflow-y: auto;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 24px;
		border-bottom: 1px solid #e2e8f0;
	}

	.modal-header h2 {
		font-size: 20px;
		font-weight: 700;
		color: #1e293b;
		margin: 0;
	}

	.close-btn {
		background: none;
		border: none;
		color: #64748b;
		cursor: pointer;
		padding: 4px;
	}

	.close-btn:hover {
		color: #1e293b;
	}

	.modal-body {
		padding: 24px;
	}

	.form-row {
		display: flex;
		gap: 16px;
		margin-bottom: 20px;
	}

	.form-group {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.form-group.full-width {
		margin-bottom: 20px;
	}

	.form-group label {
		font-size: 13px;
		font-weight: 600;
		color: #475569;
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		padding: 12px 14px;
		border: 1.5px solid #e2e8f0;
		border-radius: 10px;
		font-size: 14px;
		transition: border-color 0.2s;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #143e59;
	}

	.form-group textarea {
		resize: vertical;
	}

	.form-row.checkboxes {
		display: flex;
		gap: 24px;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 14px;
		color: #475569;
		cursor: pointer;
	}

	.checkbox-label input[type='checkbox'] {
		width: 18px;
		height: 18px;
		accent-color: #143e59;
	}

	.modal-actions {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		padding-top: 20px;
		border-top: 1px solid #e2e8f0;
	}

	/* Buttons */
	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		background: #143e59;
		color: #fff;
		border: none;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		background: #0f2d42;
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		background: #f1f5f9;
		color: #475569;
		border: none;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: #e2e8f0;
	}

	/* TOS Section */
	.tos-section {
		background: #f8fafc;
		border: 1px solid #e2e8f0;
		border-radius: 12px;
		padding: 20px;
		margin-bottom: 20px;
	}

	.section-title {
		font-size: 14px;
		font-weight: 600;
		color: #475569;
		margin: 0 0 16px 0;
		padding-bottom: 12px;
		border-bottom: 1px solid #e2e8f0;
	}

	@media (max-width: 767.98px) {
		.form-row {
			flex-direction: column;
		}
	}
</style>
