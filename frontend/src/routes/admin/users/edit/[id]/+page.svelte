<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { IconCheck, IconX, IconUser } from '$lib/icons';

	let loading = $state(true);
	let saving = $state(false);
	let error = $state('');
	let success = $state('');

	let formData = $state({
		name: '',
		first_name: '',
		last_name: '',
		email: '',
		password: '',
		password_confirmation: '',
		roles: [] as string[]
	});

	let userId = $derived(parseInt(page.params.id!));

	// Load user on mount
	$effect(() => {
		loadUser();
	});

	async function loadUser() {
		loading = true;
		error = '';
		try {
			// Use SvelteKit proxy endpoint
			const response = await fetch(`/api/admin/users/${userId}`);

			if (!response.ok) {
				if (response.status === 401) {
					goto('/login');
					return;
				}
				if (response.status === 404) {
					error = 'User not found. The user may have been deleted.';
					return;
				}
				if (response.status === 403) {
					error = 'Permission denied. You do not have access to edit this user.';
					return;
				}
				error = 'Failed to load user data.';
				return;
			}

			const result = await response.json();
			const user = result.data;

			if (!user) {
				error = 'User not found. Please check the user ID and try again.';
				return;
			}

			formData.name = user.name || '';
			formData.first_name = user.first_name || '';
			formData.last_name = user.last_name || '';
			formData.email = user.email || '';
			formData.roles = user.roles?.map((r: any) => r.name) || [];
		} catch (err: any) {
			if (err?.name === 'TypeError' && err?.message === 'Failed to fetch') {
				error = 'Network error. Please check your connection and try again.';
			} else {
				error = 'Failed to load user. Please try again.';
			}
			console.error('Failed to load user:', err);
		} finally {
			loading = false;
		}
	}

	async function handleSubmit() {
		error = '';
		success = '';

		// Validation
		if (!formData.email.trim()) {
			error = 'Email is required';
			return;
		}

		if (formData.password && formData.password !== formData.password_confirmation) {
			error = 'Passwords do not match';
			return;
		}

		saving = true;
		try {
			const payload: any = {
				name: formData.name || `${formData.first_name} ${formData.last_name}`.trim(),
				first_name: formData.first_name,
				last_name: formData.last_name,
				email: formData.email,
				roles: formData.roles
			};

			// Only include password if it's being changed
			if (formData.password) {
				payload.password = formData.password;
				payload.password_confirmation = formData.password_confirmation;
			}

			// Use SvelteKit proxy endpoint
			const response = await fetch(`/api/admin/users/${userId}`, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(payload)
			});

			if (!response.ok) {
				const result = await response.json().catch(() => ({}));
				throw new Error(result.message || 'Failed to update user');
			}

			success = 'User updated successfully!';
			setTimeout(() => {
				goto('/admin/users');
			}, 1500);
		} catch (err: any) {
			error = err.message || 'Failed to update user';
			console.error('Failed to update user:', err);
		} finally {
			saving = false;
		}
	}

	function toggleRole(role: string) {
		if (formData.roles.includes(role)) {
			formData.roles = formData.roles.filter((r) => r !== role);
		} else {
			formData.roles = [...formData.roles, role];
		}
	}
</script>

<svelte:head>
	<title>Edit User | Admin</title>
</svelte:head>

<div class="admin-page">
	<div class="page-header">
		<div>
			<h1>Edit User</h1>
			<p>Update user information and roles</p>
		</div>
		<button class="btn-secondary" onclick={() => goto('/admin/users')}>
			<IconX size={18} />
			Cancel
		</button>
	</div>

	{#if success}
		<div class="alert success">
			<IconCheck size={20} />
			{success}
		</div>
	{/if}

	{#if error}
		<div class="alert error">
			<IconX size={20} />
			<span class="error-message">{error}</span>
			<button class="btn-retry" onclick={loadUser}>Retry</button>
		</div>
	{/if}

	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading user...</p>
		</div>
	{:else}
		<form onsubmit={handleSubmit} class="user-form">
			<div class="form-section">
				<h2>Basic Information</h2>

				<div class="form-row">
					<div class="form-group">
						<label for="first_name">First Name</label>
						<input
							type="text"
							id="first_name"
							bind:value={formData.first_name}
							class="input"
							placeholder="John"
						/>
					</div>

					<div class="form-group">
						<label for="last_name">Last Name</label>
						<input
							type="text"
							id="last_name"
							bind:value={formData.last_name}
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
						bind:value={formData.email}
						class="input"
						required
						placeholder="user@example.com"
					/>
				</div>
			</div>

			<div class="form-section">
				<h2>Change Password</h2>
				<p class="help-text">Leave blank to keep current password</p>

				<div class="form-row">
					<div class="form-group">
						<label for="password">New Password</label>
						<input
							type="password"
							id="password"
							bind:value={formData.password}
							class="input"
							placeholder="••••••••"
						/>
					</div>

					<div class="form-group">
						<label for="password_confirmation">Confirm Password</label>
						<input
							type="password"
							id="password_confirmation"
							bind:value={formData.password_confirmation}
							class="input"
							placeholder="••••••••"
						/>
					</div>
				</div>
			</div>

			<div class="form-section">
				<h2>Roles & Permissions</h2>

				<div class="roles-grid">
					<label class="role-checkbox">
						<input
							id="page-checkbox" name="page-checkbox" type="checkbox"
							checked={formData.roles.includes('admin')}
							onchange={() => toggleRole('admin')}
						/>
						<div class="role-card">
							<div class="role-icon admin">
								<IconUser size={24} />
							</div>
							<div class="role-info">
								<div class="role-name">Admin</div>
								<div class="role-description">Full access to admin panel</div>
							</div>
						</div>
					</label>

					<label class="role-checkbox">
						<input
							id="page-checkbox" name="page-checkbox" type="checkbox"
							checked={formData.roles.includes('super-admin')}
							onchange={() => toggleRole('super-admin')}
						/>
						<div class="role-card">
							<div class="role-icon super-admin">
								<IconUser size={24} />
							</div>
							<div class="role-info">
								<div class="role-name">Super Admin</div>
								<div class="role-description">Complete system control</div>
							</div>
						</div>
					</label>
				</div>
			</div>

			<div class="form-actions">
				<button type="button" class="btn-secondary" onclick={() => goto('/admin/users')}>
					Cancel
				</button>
				<button type="submit" class="btn-primary" disabled={saving}>
					{#if saving}
						Saving...
					{:else}
						<IconCheck size={18} />
						Update User
					{/if}
				</button>
			</div>
		</form>
	{/if}
</div>

<style>
	.admin-page {
		max-width: 900px;
		margin: 0 auto;
		padding: 2rem;
		background: #0f172a;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: #94a3b8;
		font-size: 0.95rem;
		font-family: 'Open Sans Pro', 'Open Sans', sans-serif;
	}

	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 8px;
		font-weight: 600;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(230, 184, 0, 0.3);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: rgba(148, 163, 184, 0.1);
		color: #cbd5e1;
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	.btn-secondary:hover {
		background: rgba(148, 163, 184, 0.2);
	}

	.alert {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		border-radius: 8px;
		margin-bottom: 1.5rem;
		font-weight: 500;
	}

	.alert.success {
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.3);
		color: #34d399;
	}

	.alert.error {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
		justify-content: flex-start;
	}

	.error-message {
		flex: 1;
	}

	.btn-retry {
		padding: 0.5rem 1rem;
		background: rgba(239, 68, 68, 0.2);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 6px;
		color: #f87171;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		white-space: nowrap;
	}

	.btn-retry:hover {
		background: rgba(239, 68, 68, 0.3);
	}

	.loading {
		text-align: center;
		padding: 4rem 2rem;
		color: #94a3b8;
	}

	.loading .spinner {
		width: 48px;
		height: 48px;
		border: 4px solid rgba(148, 163, 184, 0.1);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin: 0 auto 1rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.user-form {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

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

	.roles-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1rem;
	}

	.role-checkbox {
		position: relative;
		cursor: pointer;
		margin-bottom: 0;
	}

	.role-checkbox input[type='checkbox'] {
		position: absolute;
		opacity: 0;
		cursor: pointer;
	}

	.role-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.5rem;
		background: rgba(15, 23, 42, 0.8);
		border: 2px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		transition: all 0.2s;
	}

	.role-checkbox input[type='checkbox']:checked + .role-card {
		border-color: var(--primary-500);
		background: rgba(230, 184, 0, 0.1);
	}

	.role-checkbox:hover .role-card {
		border-color: rgba(148, 163, 184, 0.4);
	}

	.role-icon {
		width: 48px;
		height: 48px;
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.role-icon.admin {
		background: rgba(230, 184, 0, 0.2);
		color: var(--primary-500);
	}

	.role-icon.super-admin {
		background: rgba(179, 143, 0, 0.2);
		color: var(--primary-600);
	}

	.role-info {
		flex: 1;
	}

	.role-name {
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 0.25rem;
	}

	.role-description {
		font-size: 0.875rem;
		color: #94a3b8;
		font-family: 'Open Sans Pro', 'Open Sans', sans-serif;
	}

	.form-actions {
		display: flex;
		justify-content: flex-end;
		gap: 1rem;
		padding-top: 1rem;
	}

	@media (max-width: 768px) {
		.page-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 1rem;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.form-actions {
			flex-direction: column-reverse;
		}

		.form-actions button {
			width: 100%;
		}
	}
</style>
