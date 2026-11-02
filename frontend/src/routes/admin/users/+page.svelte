<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { usersApi, AdminApiError } from '#lib/api/admin.js';
	import { IconPlus, IconUser, IconEdit, IconTrash, IconShield } from '#lib/icons/index.js';
	import ConfirmationModal from '#lib/components/admin/ConfirmationModal.svelte';
	import SkeletonLoader from '#lib/components/SkeletonLoader.svelte';

	let loading = $state(true);
	interface AdminUser {
		id: number;
		email: string;
		name?: string;
		first_name?: string;
		last_name?: string;
		roles?: Array<{ id: number | string; name: string }>;
		created_at: string;
	}
	let users = $state<AdminUser[]>([]);
	let error = $state('');
	let showDeleteModal = $state(false);
	let pendingDeleteId = $state<number | null>(null);

	// P1-6: load once on mount — $effect re-ran on every re-instantiation
	onMount(() => {
		loadUsers();
	});

	async function loadUsers() {
		loading = true;
		error = '';
		try {
			const response = await usersApi.list();
			// API returns roles as {id,name} objects (richer than the auth User type);
			// model that locally via AdminUser.
			users = (response.data ?? []) as unknown as AdminUser[];
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

	function deleteUser(id: number) {
		pendingDeleteId = id;
		showDeleteModal = true;
	}

	async function confirmDeleteUser() {
		if (!pendingDeleteId) return;
		showDeleteModal = false;
		const id = pendingDeleteId;
		pendingDeleteId = null;
		try {
			await usersApi.delete(id);
			await loadUsers();
		} catch (err) {
			error = 'Failed to delete user';
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

<div class="admin-users">
	<div class="admin-page-container">
		

		<!-- Header - Centered Style -->
		<header class="page-header">
			<h1>Users Management</h1>
			<p class="subtitle">Manage admin users and permissions</p>
			<div class="header-actions">
				<button class="btn-primary" onclick={() => goto('/admin/users/create')}>
					<IconPlus size={18} />
					Add Admin User
				</button>
			</div>
		</header>

		{#if loading}
			<!-- Skeleton rows reserve the table footprint to avoid CLS -->
			<div class="users-table">
				<SkeletonLoader variant="table-row" count={8} />
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
						{#each users as user (user.id)}
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
								<td class="email">{user.email || ''}</td>
								<td>
									<div class="roles">
										{#if user.roles && user.roles.length > 0}
											{#each user.roles as role (role.id)}
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
	<!-- End admin-page-container -->
</div>

<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete User"
	message="Are you sure you want to delete this user? This action cannot be undone."
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteUser}
	onCancel={() => {
		showDeleteModal = false;
		pendingDeleteId = null;
	}}
/>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * ADMIN USERS - Analytics Dashboard Layout Pattern
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.admin-users {
		background: var(--bg-base);
		color: var(--text-primary);
		position: relative;
		overflow: hidden;
	}

	.admin-page-container {
		position: relative;
		z-index: 10;
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	

	/* Page Header - CENTERED */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary);
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		color: var(--text-tertiary);
		font-size: 0.875rem;
		margin: 0 0 1rem 0;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0.75rem;
		flex-wrap: wrap;
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
		background: var(--primary-500);
		color: var(--bg-base);
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(230, 184, 0, 0.3);
	}

	.btn-secondary {
		background: var(--bg-surface, #1c2128);
		color: var(--text-secondary, #8b949e);
		border: 1px solid var(--border-default, #30363d);
	}

	.btn-secondary:hover {
		background: var(--bg-hover, #252b33);
	}

	.error-state,
	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
		color: var(--text-secondary, #8b949e);
	}

	.empty-state {
		color: var(--text-tertiary, #6e7681);
	}

	.empty-state :global(svg) {
		color: var(--text-tertiary, #6e7681);
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		color: var(--text-primary, #f0f6fc);
		margin-bottom: 0.5rem;
		font-size: 1.5rem;
	}

	.empty-state p {
		margin-bottom: 1.5rem;
		font-family: 'Open Sans Pro', 'Open Sans', sans-serif;
	}

	.error-state p {
		color: var(--error-emphasis, #f85149);
		margin-bottom: 1rem;
		font-size: 1.125rem;
	}

	.users-table {
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 12px;
		overflow: hidden;
	}

	table {
		width: 100%;
		border-collapse: collapse;
	}

	thead {
		background: var(--bg-base, #0d1117);
	}

	th {
		padding: 1rem 1.5rem;
		text-align: left;
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-secondary, #8b949e);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	tbody tr {
		border-top: 1px solid var(--border-muted, #21262d);
		transition: background 0.2s;
	}

	tbody tr:hover {
		background: rgba(230, 184, 0, 0.05);
	}

	td {
		padding: 1rem 1.5rem;
		color: var(--text-secondary, #8b949e);
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
		background: var(--primary-500);
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		font-weight: 700;
		font-size: 1.125rem;
	}

	.user-name {
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
	}

	.user-full-name {
		font-size: 0.875rem;
		color: var(--text-secondary, #8b949e);
	}

	.email {
		color: var(--text-secondary, #8b949e);
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
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.3);
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--primary-500);
		text-transform: uppercase;
	}

	.role-badge.user {
		background: var(--bg-surface, #1c2128);
		border-color: var(--border-default, #30363d);
		color: var(--text-secondary, #8b949e);
	}

	.date {
		color: var(--text-secondary, #8b949e);
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
		background: var(--bg-surface, #1c2128);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 6px;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: var(--bg-hover, #252b33);
	}

	.action-btn.edit:hover {
		background: rgba(230, 184, 0, 0.1);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-500);
	}

	.action-btn.delete:hover {
		background: rgba(248, 81, 73, 0.1);
		border-color: rgba(248, 81, 73, 0.3);
		color: var(--error-emphasis, #f85149);
	}

	@media (max-width: 1023.98px) {
		.users-table {
			overflow-x: auto;
		}

		table {
			min-width: 800px;
		}
	}

	@media (max-width: 639.98px) {
		.page-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 1rem;
		}
	}
</style>
