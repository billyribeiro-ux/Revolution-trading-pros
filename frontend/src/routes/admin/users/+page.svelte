<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { usersApi, AdminApiError } from '$lib/api/admin';
	import { IconPlus, IconUser, IconEdit, IconTrash, IconShield } from '$lib/icons';

	let loading = true;
	let users: any[] = [];
	let error = '';

	onMount(async () => {
		await loadUsers();
	});

	async function loadUsers() {
		loading = true;
		error = '';
		try {
			const response = await usersApi.list();
			users = response.data || [];
		} catch (err) {
			if (err instanceof AdminApiError) {
				if (err.status === 401) {
					goto('/login');
					return;
				} else if (err.status === 403) {
					error = 'You are not authorized to view users.';
				} else {
					error = err.message;
				}
			} else {
				error = 'Error connecting to server';
			}
			console.error('Failed to load users:', err);
		} finally {
			loading = false;
		}
	}

	async function deleteUser(id: number) {
		if (!confirm('Are you sure you want to delete this user?')) return;
		try {
			await usersApi.delete(id);
			await loadUsers();
		} catch (err) {
			alert('Failed to delete user');
			console.error(err);
		}
	}

	function formatDate(dateString: string) {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}
</script>

<svelte:head>
	<title>Users Management | Admin</title>
</svelte:head>

<div class="admin-page">
	<div class="page-header">
		<div>
			<h1>Users Management</h1>
			<p>Manage admin users and permissions</p>
		</div>
		<button class="btn-primary" onclick={() => goto('/admin/users/create')}>
			<IconPlus size={18} />
			Add Admin User
		</button>
	</div>

	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading users...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button class="btn-secondary" onclick={loadUsers}>Try Again</button>
		</div>
	{:else if users.length === 0}
		<div class="empty-state">
			<IconUser size={64} stroke={1} />
			<h3>No users found</h3>
			<p>Add your first admin user to get started</p>
			<button class="btn-primary" onclick={() => goto('/admin/users/create')}>
				<IconPlus size={18} />
				Add Admin User
			</button>
		</div>
	{:else}
		<div class="users-table">
			<table>
				<thead>
					<tr>
						<th>Name</th>
						<th>Email</th>
						<th>Roles</th>
						<th>Created</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each users as user}
						<tr>
							<td>
								<div class="user-info">
									<div class="user-avatar">
										{user.name?.charAt(0).toUpperCase() || 'U'}
									</div>
									<div>
										<div class="user-name">{user.name || 'Unnamed User'}</div>
										{#if user.first_name || user.last_name}
											<div class="user-full-name">
												{user.first_name}
												{user.last_name}
											</div>
										{/if}
									</div>
								</div>
							</td>
							<td class="email">{user.email}</td>
							<td>
								<div class="roles">
									{#if user.roles && user.roles.length > 0}
										{#each user.roles as role}
											<span class="role-badge">
												<IconShield size={14} />
												{role.name}
											</span>
										{/each}
									{:else}
										<span class="role-badge user">User</span>
									{/if}
								</div>
							</td>
							<td class="date">{formatDate(user.created_at)}</td>
							<td>
								<div class="actions">
									<button
										class="action-btn edit"
										onclick={() => goto(`/admin/users/edit/${user.id}`)}
										title="Edit user"
									>
										<IconEdit size={16} />
									</button>
									<button
										class="action-btn delete"
										onclick={() => deleteUser(user.id)}
										title="Delete user"
									>
										<IconTrash size={16} />
									</button>
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>

<style>
	.admin-page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
		background: #0f172a;
		min-height: 100vh;
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
		background: linear-gradient(135deg, #3b82f6, #8b5cf6);
		color: white;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(59, 130, 246, 0.3);
	}

	.btn-secondary {
		background: rgba(148, 163, 184, 0.1);
		color: #cbd5e1;
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	.btn-secondary:hover {
		background: rgba(148, 163, 184, 0.2);
	}

	.loading,
	.error-state,
	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
		color: #94a3b8;
	}

	.loading .spinner {
		width: 48px;
		height: 48px;
		border: 4px solid rgba(148, 163, 184, 0.1);
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin: 0 auto 1rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.empty-state {
		color: #64748b;
	}

	.empty-state :global(svg) {
		color: #475569;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		color: #f1f5f9;
		margin-bottom: 0.5rem;
		font-size: 1.5rem;
	}

	.empty-state p {
		margin-bottom: 1.5rem;
		font-family: 'Open Sans Pro', 'Open Sans', sans-serif;
	}

	.error-state p {
		color: #f87171;
		margin-bottom: 1rem;
		font-size: 1.125rem;
	}

	.users-table {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		overflow: hidden;
	}

	table {
		width: 100%;
		border-collapse: collapse;
	}

	thead {
		background: rgba(15, 23, 42, 0.8);
	}

	th {
		padding: 1rem 1.5rem;
		text-align: left;
		font-size: 0.875rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	tbody tr {
		border-top: 1px solid rgba(148, 163, 184, 0.1);
		transition: background 0.2s;
	}

	tbody tr:hover {
		background: rgba(59, 130, 246, 0.05);
	}

	td {
		padding: 1rem 1.5rem;
		color: #cbd5e1;
	}

	.user-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.user-avatar {
		width: 40px;
		height: 40px;
		border-radius: 50%;
		background: linear-gradient(135deg, #3b82f6, #8b5cf6);
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		font-weight: 700;
		font-size: 1.125rem;
	}

	.user-name {
		font-weight: 600;
		color: #f1f5f9;
	}

	.user-full-name {
		font-size: 0.875rem;
		color: #94a3b8;
	}

	.email {
		color: #94a3b8;
		font-family: 'Open Sans Pro', 'Open Sans', monospace;
	}

	.roles {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.role-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.75rem;
		background: rgba(139, 92, 246, 0.1);
		border: 1px solid rgba(139, 92, 246, 0.3);
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 600;
		color: #a78bfa;
		text-transform: uppercase;
	}

	.role-badge.user {
		background: rgba(148, 163, 184, 0.1);
		border-color: rgba(148, 163, 184, 0.3);
		color: #94a3b8;
	}

	.date {
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.actions {
		display: flex;
		gap: 0.5rem;
	}

	.action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #cbd5e1;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: rgba(148, 163, 184, 0.2);
	}

	.action-btn.edit:hover {
		background: rgba(59, 130, 246, 0.1);
		border-color: rgba(59, 130, 246, 0.3);
		color: #60a5fa;
	}

	.action-btn.delete:hover {
		background: rgba(239, 68, 68, 0.1);
		border-color: rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	@media (max-width: 1024px) {
		.users-table {
			overflow-x: auto;
		}

		table {
			min-width: 800px;
		}
	}

	@media (max-width: 640px) {
		.page-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 1rem;
		}
	}
</style>
