<script lang="ts">
	/**
	 * R27-D extraction (2026-05-20): "Basic Information" section of the
	 * admin user-edit form (first_name / last_name / email). Uses
	 * `$bindable` on first_name/last_name/email so parent runes stay
	 * the single source of truth.
	 */
	interface Props {
		firstName: string;
		lastName: string;
		email: string;
	}

	let {
		firstName = $bindable(),
		lastName = $bindable(),
		email = $bindable()
	}: Props = $props();
</script>

<div class="form-section">
	<h2>Basic Information</h2>

	<div class="form-row">
		<div class="form-group">
			<label for="first_name">First Name</label>
			<input
				type="text"
				id="first_name"
				name="first_name"
				bind:value={firstName}
				class="input"
				placeholder="John"
			/>
		</div>

		<div class="form-group">
			<label for="last_name">Last Name</label>
			<input
				type="text"
				id="last_name"
				name="last_name"
				bind:value={lastName}
				class="input"
				placeholder="Doe"
			/>
		</div>
	</div>

	<div class="form-group">
		<label for="email">Email Address *</label>
		<input
			type="email"
			id="email"
			name="email"
			autocomplete="email"
			bind:value={email}
			class="input"
			required
			placeholder="user@example.com"
		/>
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

	@media (max-width: 768px) {
		.form-row {
			grid-template-columns: 1fr;
		}
	}
</style>
