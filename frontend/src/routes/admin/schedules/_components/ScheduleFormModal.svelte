<!--
	ScheduleFormModal — create / edit modal dialog.
	Extracted from /admin/schedules in R21-C.

	Parent owns formData ($state) and binds it via $bindable so per-keystroke
	updates don't round-trip an `onChange` callback.
-->
<script lang="ts">
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import type { ScheduleForm } from './types';

	interface TzOption {
		value: string;
		label: string;
	}

	interface RoomTypeOption {
		value: 'live' | 'recorded' | 'hybrid';
		label: string;
	}

	interface Props {
		open: boolean;
		formData: ScheduleForm;
		days: readonly string[];
		timezones: readonly TzOption[];
		roomTypes: readonly RoomTypeOption[];
		saving: boolean;
		isEdit: boolean;
		onsubmit: () => void;
		onclose: () => void;
	}

	let {
		open,
		formData = $bindable(),
		days,
		timezones,
		roomTypes,
		saving,
		isEdit,
		onsubmit,
		onclose
	}: Props = $props();

	function handleOverlayClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			onclose();
		}
	}

	function handleOverlayKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			onclose();
		}
	}
</script>

{#if open}
	<div
		class="modal-overlay"
		role="dialog"
		aria-modal="true"
		aria-labelledby="modal-title"
		tabindex="-1"
		onclick={handleOverlayClick}
		onkeydown={handleOverlayKeydown}
	>
		<div class="modal" role="document">
			<div class="modal-header">
				<h3 id="modal-title">
					{isEdit ? 'Edit Schedule' : 'Create Schedule'}
				</h3>
				<button class="modal-close" onclick={onclose} aria-label="Close">
					<IconX size={24} />
				</button>
			</div>

			<form
				class="modal-form"
				onsubmit={(e) => {
					e.preventDefault();
					onsubmit();
				}}
			>
				<div class="form-group">
					<label for="title">Event Title <span class="required">*</span></label>
					<input
						type="text"
						id="title"
						name="title"
						bind:value={formData.title}
						placeholder="e.g., Morning Market Analysis"
						required
					/>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="trader_name">Trader/Host</label>
						<input
							type="text"
							id="trader_name"
							name="trader_name"
							bind:value={formData.trader_name}
							placeholder="e.g., Taylor Horton"
						/>
					</div>

					<div class="form-group">
						<label for="room_type">Session Type</label>
						<select id="room_type" bind:value={formData.room_type}>
							{#each roomTypes as type (type.value)}
								<option value={type.value}>{type.label}</option>
							{/each}
						</select>
					</div>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="day_of_week">Day of Week <span class="required">*</span></label>
						<select id="day_of_week" bind:value={formData.day_of_week} required>
							{#each days as day, index (index)}
								<option value={index}>{day}</option>
							{/each}
						</select>
					</div>

					<div class="form-group">
						<label for="recurrence">Recurrence</label>
						<select id="recurrence" bind:value={formData.recurrence}>
							<option value="weekly">Weekly</option>
							<option value="biweekly">Bi-weekly</option>
							<option value="monthly">Monthly</option>
							<!-- FIX-2026-04-26 (P3-3): <option value={null}> silently coerces to "" — use explicit empty string. -->
							<option value="">One-time</option>
						</select>
					</div>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="start_time">Start Time <span class="required">*</span></label>
						<input
							type="time"
							id="start_time"
							name="start_time"
							bind:value={formData.start_time}
							required
						/>
					</div>

					<div class="form-group">
						<label for="end_time">End Time <span class="required">*</span></label>
						<input
							type="time"
							id="end_time"
							name="end_time"
							bind:value={formData.end_time}
							required
						/>
					</div>
				</div>

				<div class="form-group">
					<label for="timezone">Timezone</label>
					<select id="timezone" bind:value={formData.timezone}>
						{#each timezones as tz (tz.value)}
							<option value={tz.value}>{tz.label}</option>
						{/each}
					</select>
				</div>

				<div class="form-group">
					<label for="description">Description</label>
					<textarea
						id="description"
						bind:value={formData.description}
						placeholder="Optional description of the session..."
						rows="3"
					></textarea>
				</div>

				<div class="form-group form-checkbox">
					<label>
						<input
							id="page-formdata-is-active"
							name="page-formdata-is-active"
							type="checkbox"
							bind:checked={formData.is_active}
						/>
						<span>Active (visible to members)</span>
					</label>
				</div>

				<div class="modal-actions">
					<button type="button" class="btn btn-secondary" onclick={onclose}>Cancel</button>
					<button type="submit" class="btn btn-primary" disabled={saving}>
						{#if saving}
							<span class="spinner-sm"></span>
							Saving...
						{:else}
							{isEdit ? 'Update Schedule' : 'Create Schedule'}
						{/if}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
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
		z-index: 1000;
		padding: 20px;
	}

	.modal {
		background: #fff;
		border-radius: 16px;
		width: 100%;
		max-width: 560px;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow: 0 24px 48px rgba(0, 0, 0, 0.2);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px 24px;
		border-bottom: 1px solid #e2e8f0;
	}

	.modal-header h3 {
		font-size: 20px;
		font-weight: 700;
		color: #1e293b;
		margin: 0;
	}

	.modal-close {
		background: none;
		border: none;
		cursor: pointer;
		color: #64748b;
		padding: 4px;
		display: flex;
		border-radius: 6px;
	}

	.modal-close:hover {
		background: #f1f5f9;
		color: #1e293b;
	}

	.modal-form {
		padding: 24px;
	}

	.form-group {
		margin-bottom: 20px;
	}

	.form-group label {
		display: block;
		font-size: 13px;
		font-weight: 600;
		color: #475569;
		margin-bottom: 6px;
	}

	.required {
		color: #dc2626;
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		width: 100%;
		padding: 10px 14px;
		font-size: 14px;
		border: 1px solid #e2e8f0;
		border-radius: 8px;
		transition:
			border-color 0.2s,
			box-shadow 0.2s;
		background: #fff;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #143e59;
		box-shadow: 0 0 0 3px rgba(20, 62, 89, 0.1);
	}

	.form-group textarea {
		resize: vertical;
		min-height: 80px;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 16px;
	}

	.form-checkbox {
		display: flex;
		align-items: center;
	}

	.form-checkbox label {
		display: flex;
		align-items: center;
		gap: 10px;
		cursor: pointer;
		margin: 0;
	}

	.form-checkbox input[type='checkbox'] {
		width: 18px;
		height: 18px;
		cursor: pointer;
	}

	.form-checkbox span {
		font-weight: 400;
		color: #1e293b;
	}

	.modal-actions {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		margin-top: 24px;
		padding-top: 20px;
		border-top: 1px solid #e2e8f0;
	}

	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s ease;
		border: none;
	}

	.btn-primary {
		background: #143e59;
		color: #fff;
	}

	.btn-primary:hover {
		background: #0d2a3d;
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: #f1f5f9;
		color: #475569;
	}

	.btn-secondary:hover {
		background: #e2e8f0;
	}

	.spinner-sm {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: #fff;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
		display: inline-block;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@media (max-width: 767.98px) {
		.form-row {
			grid-template-columns: 1fr;
		}
	}
</style>
