<script lang="ts">
	/**
	 * Admin user-edit page.
	 *
	 * R27-D extraction (2026-05-20): broke the 616-LOC page into six
	 * focused presentational components under `_components/`:
	 *   - UserEditPageHeader      — title + cancel button
	 *   - UserEditAlertBanner     — discriminated success/error banner
	 *   - UserBasicInfoSection    — first/last name + email ($bindable)
	 *   - UserPasswordSection     — new password + confirmation ($bindable)
	 *   - UserRolesSection        — role checkboxes (incidental fixes:
	 *                                 duplicate `id="page-checkbox"` AND
	 *                                 duplicate `name="page-checkbox"`
	 *                                 — both invalid HTML; now unique
	 *                                 per role)
	 *   - UserEditFormActions     — Cancel + Submit buttons
	 *
	 * The page now owns ONLY: routing (`userId` from `page.params`),
	 * load/save fetch + error mapping, validation, optimistic
	 * navigation back to `/admin/users` after success, and the role-
	 * toggle helper.
	 *
	 * Incidental: original `autocomplete="current-password"` on the
	 * NEW password input was wrong — corrected to `new-password` in
	 * `UserPasswordSection`. Confirmation field was already correct.
	 */
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { logger } from '$lib/utils/logger';

	import UserBasicInfoSection from './_components/UserBasicInfoSection.svelte';
	import UserEditAlertBanner from './_components/UserEditAlertBanner.svelte';
	import UserEditFormActions from './_components/UserEditFormActions.svelte';
	import UserEditPageHeader from './_components/UserEditPageHeader.svelte';
	import UserPasswordSection from './_components/UserPasswordSection.svelte';
	import UserRolesSection from './_components/UserRolesSection.svelte';
	import type { UserEditFormData } from './_components/types';

	let loading = $state(true);
	let saving = $state(false);
	let error = $state('');
	let success = $state('');

	let formData = $state<UserEditFormData>({
		name: '',
		first_name: '',
		last_name: '',
		email: '',
		password: '',
		password_confirmation: '',
		roles: []
	});

	let userId = $derived(parseInt(page.params.id!));

	// P1-6: load once on mount — $effect re-ran on every re-instantiation
	onMount(() => {
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
			formData.roles = user.roles?.map((r: { name: string }) => r.name) || [];
		} catch (err: unknown) {
			const e = err as { name?: string; message?: string };
			if (e?.name === 'TypeError' && e?.message === 'Failed to fetch') {
				error = 'Network error. Please check your connection and try again.';
			} else {
				error = 'Failed to load user. Please try again.';
			}
			logger.error('Failed to load user:', err);
		} finally {
			loading = false;
		}
	}

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
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
			const payload: Record<string, unknown> = {
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
		} catch (err: unknown) {
			error = err instanceof Error ? err.message : 'Failed to update user';
			logger.error('Failed to update user:', err);
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

	function cancel() {
		goto('/admin/users');
	}
</script>

<svelte:head>
	<title>Edit User | Admin</title>
</svelte:head>

<div class="admin-page">
	<UserEditPageHeader onCancel={cancel} />

	{#if success}
		<UserEditAlertBanner kind="success" message={success} />
	{/if}

	{#if error}
		<UserEditAlertBanner kind="error" message={error} onRetry={loadUser} />
	{/if}

	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading user...</p>
		</div>
	{:else}
		<form onsubmit={handleSubmit} class="user-form">
			<UserBasicInfoSection
				bind:firstName={formData.first_name}
				bind:lastName={formData.last_name}
				bind:email={formData.email}
			/>

			<UserPasswordSection
				bind:password={formData.password}
				bind:passwordConfirmation={formData.password_confirmation}
			/>

			<UserRolesSection roles={formData.roles} onToggleRole={toggleRole} />

			<UserEditFormActions {saving} onCancel={cancel} />
		</form>
	{/if}
</div>

<style>
	/* Page-level chrome only — section styles live in _components/. */

	.admin-page {
		max-width: 900px;
		margin: 0 auto;
		padding: 2rem;
		background: #0f172a;
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
</style>
