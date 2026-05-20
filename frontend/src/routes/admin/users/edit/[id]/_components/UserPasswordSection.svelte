<script lang="ts">
	/**
	 * R27-D extraction (2026-05-20): "Change Password" section of the
	 * admin user-edit form. Both password fields are blank by default;
	 * the parent's submit handler only sends them on the wire when
	 * `password` is truthy (leave blank to keep current password).
	 *
	 * FIX (incidental): the original page used
	 *   autocomplete="current-password" on the NEW password field —
	 *   the canonical token for a change-password flow is
	 *   "new-password". The confirmation field was already correct.
	 */
	interface Props {
		password: string;
		passwordConfirmation: string;
	}

	let {
		password = $bindable(),
		passwordConfirmation = $bindable()
	}: Props = $props();
</script>

<div class="form-section">
	<h2>Change Password</h2>
	<p class="help-text">Leave blank to keep current password</p>

	<div class="form-row">
		<div class="form-group">
			<label for="password">New Password</label>
			<input
				type="password"
				id="password"
				name="password"
				autocomplete="new-password"
				bind:value={password}
				class="input"
				placeholder="••••••••"
			/>
		</div>

		<div class="form-group">
			<label for="password_confirmation">Confirm Password</label>
			<input
				type="password"
				id="password_confirmation"
				name="password_confirmation"
				autocomplete="new-password"
				bind:value={passwordConfirmation}
				class="input"
				placeholder="••••••••"
			/>
		</div>
	</div>
</div>

<style>
	.form-section {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		padding: 2rem;
	}

	.form-section h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 1.5rem 0;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-group:last-child {
		margin-bottom: 0;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1.5rem;
	}

	label {
		display: block;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
		font-size: 0.95rem;
	}

	.input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.95rem;
		transition: all 0.2s;
		font-family: 'Open Sans Pro', 'Open Sans', sans-serif;
	}

	.input:focus {
		outline: none;
		border-color: rgba(230, 184, 0, 0.5);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.1);
	}

	.help-text {
		color: #94a3b8;
		font-size: 0.875rem;
		margin: 0 0 1rem 0;
		font-family: 'Open Sans Pro', 'Open Sans', sans-serif;
	}

	@media (max-width: 768px) {
		.form-row {
			grid-template-columns: 1fr;
		}
	}
</style>
