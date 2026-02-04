<!--
	/admin/crm/managers - Manager Permissions & Roles
	Apple Principal Engineer ICT 7 Grade - January 2026
	
	Features:
	- Manager user management with roles
	- Role CRUD with permission configuration
	- Tab-based navigation (Managers/Roles)
	- Permission badges and counts
	- Full Svelte 5 $state/$derived reactivity
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import IconUsers from '@tabler/icons-svelte/icons/users';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import IconShieldLock from '@tabler/icons-svelte/icons/shield-lock';
	import IconUser from '@tabler/icons-svelte/icons/user';
	import IconSettings from '@tabler/icons-svelte/icons/settings';
	import { crmAPI } from '$lib/api/crm';
	import type { ManagerRole, ManagerUser } from '$lib/crm/types';

	let roles = $state<ManagerRole[]>([]);
	let managers = $state<ManagerUser[]>([]);
	let isLoading = $state(true);
	let error = $state('');
	let activeTab = $state<'managers' | 'roles'>('managers');

	let stats = $state({
		totalManagers: 0,
		totalRoles: 0
	});

	async function loadData() {
		isLoading = true;
		error = '';

		try {
			const [rolesResponse, managersResponse] = await Promise.all([
				crmAPI.getManagerRoles(),
				crmAPI.getManagerUsers()
			]);

			roles = rolesResponse.data || [];
			managers = managersResponse.data || [];

			stats = {
				totalManagers: managers.length,
				totalRoles: roles.length
			};
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load data';
		} finally {
			isLoading = false;
		}
	}

	async function deleteRole(id: string) {
		if (
			!confirm('Are you sure you want to delete this role? Users with this role will lose access.')
		)
			return;

		try {
			await crmAPI.deleteManagerRole(id);
			await loadData();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete role';
		}
	}

	async function removeManager(id: string) {
		if (!confirm('Are you sure you want to remove this manager?')) return;

		try {
			await crmAPI.removeManager(id);
			await loadData();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to remove manager';
		}
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString();
	}

	$effect(() => {
		if (browser) {
			loadData();
		}
	});
</script>

<svelte:head>
	<title>Manager Permissions - FluentCRM Pro</title>
</svelte:head>

<div class="managers-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1>Manager Permissions</h1>
			<p class="page-description">Control access and permissions for CRM managers</p>
		</div>
		<div class="header-actions">
			<button class="btn-refresh" onclick={() => loadData()} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
			{#if activeTab === 'managers'}
				<a href="/admin/crm/managers/add" class="btn-primary">
					<IconPlus size={18} />
					Add Manager
				</a>
			{:else}
				<a href="/admin/crm/managers/roles/new" class="btn-primary">
					<IconPlus size={18} />
					New Role
				</a>
			{/if}
		</div>
	</div>

	<!-- Stats Cards -->
	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-icon blue">
				<IconUsers size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{stats.totalManagers}</span>
				<span class="stat-label">Managers</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon purple">
				<IconShieldLock size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{stats.totalRoles}</span>
				<span class="stat-label">Roles</span>
			</div>
		</div>
	</div>

	<!-- Tabs -->
	<div class="tabs">
		<button
			class="tab"
			class:active={activeTab === 'managers'}
			onclick={() => (activeTab = 'managers')}
		>
			<IconUsers size={18} />
			Managers
		</button>
		<button class="tab" class:active={activeTab === 'roles'} onclick={() => (activeTab = 'roles')}>
			<IconShieldLock size={18} />
			Roles
		</button>
	</div>

	<!-- Content -->
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button onclick={() => loadData()}>Try Again</button>
		</div>
	{:else if activeTab === 'managers'}
		<!-- Managers List -->
		{#if managers.length === 0}
			<div class="empty-state">
				<IconUsers size={48} />
				<h3>No managers added</h3>
				<p>Add users as CRM managers to give them access to specific features</p>
				<a href="/admin/crm/managers/add" class="btn-primary">
					<IconPlus size={18} />
					Add Manager
				</a>
			</div>
		{:else}
			<div class="managers-list">
				{#each managers as manager}
					<div class="manager-card">
						<div class="manager-avatar">
							{#if manager.avatar}
								<img src={manager.avatar} alt={manager.name} />
							{:else}
								<IconUser size={24} />
							{/if}
						</div>
						<div class="manager-info">
							<h3>{manager.name}</h3>
							<p class="manager-email">{manager.email || ''}</p>
						</div>
						<div class="manager-role">
							<span class="role-badge">
								<IconShieldLock size={14} />
								{manager.role?.name || 'No Role'}
							</span>
						</div>
						<div class="manager-meta">
							<span>Added {formatDate(manager.assigned_at)}</span>
						</div>
						<div class="manager-actions">
							<a href="/admin/crm/managers/{manager.id}/edit" class="btn-icon" title="Edit">
								<IconEdit size={16} />
							</a>
							<button
								class="btn-icon danger"
								title="Remove"
								onclick={() => removeManager(manager.id)}
							>
								<IconTrash size={16} />
							</button>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	{:else}
		<!-- Roles List -->
		{#if roles.length === 0}
			<div class="empty-state">
				<IconShieldLock size={48} />
				<h3>No roles defined</h3>
				<p>Create roles to define different permission levels for managers</p>
				<a href="/admin/crm/managers/roles/new" class="btn-primary">
					<IconPlus size={18} />
					Create Role
				</a>
			</div>
		{:else}
			<div class="roles-grid">
				{#each roles as role}
					<div class="role-card">
						<div class="role-header">
							<div class="role-icon">
								<IconShieldLock size={24} />
							</div>
							<div class="role-info">
								<h3>{role.name}</h3>
								{#if role.description}
									<p>{role.description}</p>
								{/if}
							</div>
							{#if role.is_default}
								<span class="default-badge">Default</span>
							{/if}
						</div>
						<div class="role-stats">
							<div class="stat">
								<IconUsers size={16} />
								<span>{role.users_count} users</span>
							</div>
							<div class="stat">
								<IconSettings size={16} />
								<span>{role.permissions.filter((p) => p.allowed).length} permissions</span>
							</div>
						</div>
						<div class="role-permissions">
							<span class="permissions-label">Permissions:</span>
							<div class="permissions-tags">
								{#each role.permissions.filter((p) => p.allowed).slice(0, 4) as perm}
									<span class="permission-tag">{perm.module} ({perm.action})</span>
								{/each}
								{#if role.permissions.filter((p) => p.allowed).length > 4}
									<span class="permission-more"
										>+{role.permissions.filter((p) => p.allowed).length - 4} more</span
									>
								{/if}
							</div>
						</div>
						<div class="role-actions">
							<a href="/admin/crm/managers/roles/{role.id}/edit" class="btn-secondary">
								<IconEdit size={16} />
								Edit
							</a>
							{#if !role.is_default}
								<button class="btn-icon danger" onclick={() => deleteRole(role.id)}>
									<IconTrash size={16} />
								</button>
							{/if}
						</div>
					</div>
				{/each}
			</div>
		{/if}
	{/if}
</div>

<style>
	.managers-page {
		max-width: 1200px;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.page-description {
		color: #64748b;
		margin: 0;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-refresh {
		width: 42px;
		height: 42px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: rgba(230, 184, 0, 0.2);
		color: var(--primary-500);
	}

	.btn-refresh :global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(230, 184, 0, 0.4);
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.2);
		color: var(--primary-500);
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
		font-size: 0.85rem;
	}

	.btn-secondary:hover {
		background: rgba(230, 184, 0, 0.2);
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
		max-width: 500px;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 14px;
	}

	.stat-icon {
		width: 52px;
		height: 52px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-icon.blue {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}
	.stat-icon.purple {
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-500);
	}

	.stat-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-label {
		font-size: 0.8rem;
		color: #64748b;
	}

	.tabs {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 2rem;
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
		padding-bottom: 1rem;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: transparent;
		border: 1px solid transparent;
		border-radius: 10px;
		color: #64748b;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.tab:hover {
		color: var(--primary-500);
	}

	.tab.active {
		background: rgba(230, 184, 0, 0.1);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-500);
	}

	.managers-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.manager-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 14px;
	}

	.manager-avatar {
		width: 48px;
		height: 48px;
		border-radius: 50%;
		background: rgba(230, 184, 0, 0.2);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--primary-500);
		overflow: hidden;
	}

	.manager-avatar img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.manager-info {
		flex: 1;
	}

	.manager-info h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.manager-email {
		font-size: 0.85rem;
		color: #64748b;
		margin: 0;
	}

	.role-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.35rem 0.75rem;
		background: rgba(230, 184, 0, 0.15);
		border-radius: 9999px;
		font-size: 0.8rem;
		font-weight: 600;
		color: var(--primary-500);
	}

	.manager-meta {
		font-size: 0.8rem;
		color: #64748b;
	}

	.manager-actions {
		display: flex;
		gap: 0.5rem;
	}

	.btn-icon {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-icon:hover {
		background: rgba(230, 184, 0, 0.1);
		color: var(--primary-500);
	}

	.btn-icon.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-color: rgba(239, 68, 68, 0.3);
	}

	.roles-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 1.5rem;
	}

	.role-card {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 14px;
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.role-header {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
	}

	.role-icon {
		width: 44px;
		height: 44px;
		border-radius: 10px;
		background: rgba(230, 184, 0, 0.15);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--primary-500);
		flex-shrink: 0;
	}

	.role-info {
		flex: 1;
	}

	.role-info h3 {
		font-size: 1.1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.role-info p {
		font-size: 0.85rem;
		color: #64748b;
		margin: 0;
	}

	.default-badge {
		padding: 0.25rem 0.5rem;
		background: rgba(34, 197, 94, 0.15);
		border-radius: 4px;
		font-size: 0.7rem;
		font-weight: 600;
		color: #4ade80;
	}

	.role-stats {
		display: flex;
		gap: 1.5rem;
		padding: 0.75rem 0;
		border-top: 1px solid rgba(230, 184, 0, 0.1);
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
	}

	.role-stats .stat {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.85rem;
		color: #94a3b8;
	}

	.role-permissions {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.permissions-label {
		font-size: 0.8rem;
		color: #64748b;
	}

	.permissions-tags {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.permission-tag {
		padding: 0.25rem 0.5rem;
		background: rgba(230, 184, 0, 0.1);
		border-radius: 4px;
		font-size: 0.7rem;
		color: var(--primary-500);
	}

	.permission-more {
		font-size: 0.75rem;
		color: #64748b;
	}

	.role-actions {
		display: flex;
		gap: 0.75rem;
		margin-top: auto;
	}

	.loading-state,
	.error-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: #64748b;
	}

	.empty-state :global(svg) {
		color: #475569;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		color: #e2e8f0;
		margin: 0 0 0.5rem 0;
	}

	.empty-state p {
		margin: 0 0 1.5rem 0;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(230, 184, 0, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}
</style>
