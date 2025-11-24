<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { apiClient } from '$lib/api/client';

	const dispatch = createEventDispatcher();

	export let widgetId: string;

	let isLoading = false;
	let shareEmail = '';
	let shareEmails: string[] = [];
	let permissions = {
		can_view: true,
		can_edit: false,
		can_delete: false
	};
	let shareLink: string | null = null;
	let linkExpireDays: number | null = 7;
	let sharedUsers: any[] = [];
	let error: string | null = null;
	let success: string | null = null;

	async function loadSharedUsers() {
		try {
			const response = (await apiClient.get(`/widgets/${widgetId}/shares`)) as { data: any[] };
			sharedUsers = response.data;
		} catch (err: any) {
			error = 'Failed to load shared users';
		}
	}

	async function handleShareWithUser() {
		if (!shareEmail) return;

		isLoading = true;
		error = null;
		success = null;

		try {
			await apiClient.post(`/widgets/${widgetId}/share`, {
				emails: [shareEmail],
				permissions
			});

			success = `Widget shared with ${shareEmail}`;
			shareEmail = '';
			await loadSharedUsers();
			dispatch('shared', { email: shareEmail });
		} catch (err: any) {
			error = err.message || 'Failed to share widget';
		} finally {
			isLoading = false;
		}
	}

	async function handleGenerateLink() {
		isLoading = true;
		error = null;
		success = null;

		try {
			const response = (await apiClient.post(`/widgets/${widgetId}/share-link`, {
				expires_in_days: linkExpireDays
			})) as { data: { link: string } };

			shareLink = response.data.link;
			success = 'Share link generated!';
		} catch (err: any) {
			error = err.message || 'Failed to generate share link';
		} finally {
			isLoading = false;
		}
	}

	async function handleRevokeShare(userId: number) {
		if (!confirm('Revoke access for this user?')) return;

		try {
			await apiClient.delete(`/widgets/${widgetId}/share/${userId}`);
			await loadSharedUsers();
			success = 'Access revoked';
		} catch (err: any) {
			error = 'Failed to revoke access';
		}
	}

	function copyLinkToClipboard() {
		if (shareLink) {
			navigator.clipboard.writeText(shareLink);
			success = 'Link copied to clipboard!';
		}
	}

	// Load shared users on mount
	loadSharedUsers();
</script>

<div class="widget-sharing">
	<div class="section">
		<h3>Share with Users</h3>

		<div class="form-group">
			<label for="share-email">Email Address</label>
			<div class="input-group">
				<input
					id="share-email"
					type="email"
					placeholder="user@example.com"
					bind:value={shareEmail}
				/>
				<button class="btn-add" on:click={handleShareWithUser} disabled={isLoading || !shareEmail}>
					Share
				</button>
			</div>
		</div>

		<div class="permissions-group">
			<label class="checkbox-label">
				<input type="checkbox" bind:checked={permissions.can_view} />
				<span>Can View</span>
			</label>
			<label class="checkbox-label">
				<input type="checkbox" bind:checked={permissions.can_edit} />
				<span>Can Edit</span>
			</label>
			<label class="checkbox-label">
				<input type="checkbox" bind:checked={permissions.can_delete} />
				<span>Can Delete</span>
			</label>
		</div>

		{#if sharedUsers.length > 0}
			<div class="shared-users-list">
				<h4>Shared With</h4>
				{#each sharedUsers as user}
					<div class="shared-user-item">
						<div class="user-info">
							<span class="user-name">{user.name}</span>
							<span class="user-email">{user.email}</span>
						</div>
						<div class="user-permissions">
							{#if user.can_view}<span class="badge">View</span>{/if}
							{#if user.can_edit}<span class="badge">Edit</span>{/if}
							{#if user.can_delete}<span class="badge">Delete</span>{/if}
						</div>
						<button class="btn-revoke" on:click={() => handleRevokeShare(user.id)}> Revoke </button>
					</div>
				{/each}
			</div>
		{/if}
	</div>

	<div class="divider"></div>

	<div class="section">
		<h3>Generate Share Link</h3>
		<p class="description">Create a public link to share this widget</p>

		<div class="form-group">
			<label for="expire-days">Link Expires In (days)</label>
			<input id="expire-days" type="number" min="1" max="365" bind:value={linkExpireDays} />
		</div>

		<button class="btn-primary" on:click={handleGenerateLink} disabled={isLoading}>
			{isLoading ? 'Generating...' : 'Generate Link'}
		</button>

		{#if shareLink}
			<div class="share-link-result">
				<input type="text" readonly value={shareLink} on:click={(e) => e.currentTarget.select()} />
				<button class="btn-copy" on:click={copyLinkToClipboard}> Copy </button>
			</div>
		{/if}
	</div>

	{#if error}
		<div class="alert alert-error">{error}</div>
	{/if}

	{#if success}
		<div class="alert alert-success">{success}</div>
	{/if}
</div>

<style>
	.widget-sharing {
		background: white;
		border-radius: 12px;
		padding: 2rem;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
	}

	.section {
		margin-bottom: 2rem;
	}

	h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 1rem 0;
	}

	h4 {
		font-size: 0.95rem;
		font-weight: 600;
		color: #374151;
		margin: 1.5rem 0 0.75rem 0;
	}

	.description {
		color: #6b7280;
		font-size: 0.875rem;
		margin: 0 0 1rem 0;
	}

	.form-group {
		margin-bottom: 1rem;
	}

	label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
		margin-bottom: 0.5rem;
	}

	.input-group {
		display: flex;
		gap: 0.5rem;
	}

	input[type='email'],
	input[type='number'],
	input[type='text'] {
		flex: 1;
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		font-size: 0.875rem;
	}

	.btn-add,
	.btn-primary {
		padding: 0.5rem 1rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-add:hover:not(:disabled),
	.btn-primary:hover:not(:disabled) {
		background: #2563eb;
	}

	.btn-add:disabled,
	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.permissions-group {
		display: flex;
		gap: 1.5rem;
		margin-bottom: 1rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		font-size: 0.875rem;
	}

	.shared-users-list {
		margin-top: 1.5rem;
	}

	.shared-user-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem;
		background: #f9fafb;
		border-radius: 8px;
		margin-bottom: 0.5rem;
	}

	.user-info {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.user-name {
		font-weight: 500;
		color: #1f2937;
		font-size: 0.875rem;
	}

	.user-email {
		color: #6b7280;
		font-size: 0.75rem;
	}

	.user-permissions {
		display: flex;
		gap: 0.5rem;
	}

	.badge {
		padding: 0.125rem 0.5rem;
		background: #dbeafe;
		color: #1e40af;
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 500;
	}

	.btn-revoke {
		padding: 0.25rem 0.75rem;
		background: #fee2e2;
		color: #991b1b;
		border: none;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 500;
		cursor: pointer;
	}

	.btn-revoke:hover {
		background: #fecaca;
	}

	.share-link-result {
		display: flex;
		gap: 0.5rem;
		margin-top: 1rem;
	}

	.share-link-result input {
		flex: 1;
		background: #f9fafb;
	}

	.btn-copy {
		padding: 0.5rem 1rem;
		background: #10b981;
		color: white;
		border: none;
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
	}

	.btn-copy:hover {
		background: #059669;
	}

	.divider {
		height: 1px;
		background: #e5e7eb;
		margin: 2rem 0;
	}

	.alert {
		padding: 1rem;
		border-radius: 8px;
		margin-top: 1rem;
		font-size: 0.875rem;
	}

	.alert-error {
		background: #fee2e2;
		color: #991b1b;
		border: 1px solid #fecaca;
	}

	.alert-success {
		background: #d1fae5;
		color: #065f46;
		border: 1px solid #a7f3d0;
	}
</style>
