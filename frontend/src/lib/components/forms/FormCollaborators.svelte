<script lang="ts">
	/**
	 * Form Collaborators - Team collaboration management
	 *
	 * Features:
	 * - Add/remove collaborators
	 * - Role management
	 * - Real-time presence indicators
	 * - Activity feed
	 * - Comment threads
	 *
	 * @version 1.0.0
	 */

	import { onMount, onDestroy } from 'svelte';
	import { getAuthToken } from '$lib/stores/auth.svelte';

	interface Props {
		formId: number;
	}

	interface Collaborator {
		id: number;
		name: string;
		email: string;
		role: string;
		invited_at: string;
		last_active_at: string | null;
	}

	interface Activity {
		id: number;
		user_name: string;
		action: string;
		description: string;
		created_at: string;
	}

	let { formId }: Props = $props();

	// State
	let collaborators = $state<Collaborator[]>([]);
	let activeUsers = $state<{ user_id: number; name: string; avatar_color: string }[]>([]);
	let activities = $state<Activity[]>([]);
	let loading = $state(true);
	let inviteEmail = $state('');
	let inviteRole = $state('editor');
	let inviting = $state(false);
	let showInviteForm = $state(false);
	let pollingInterval: ReturnType<typeof setInterval> | null = null;

	const roles = [
		{ value: 'editor', label: 'Editor', description: 'Can edit form fields and settings' },
		{ value: 'viewer', label: 'Viewer', description: 'Can view form and submissions' },
		{ value: 'commenter', label: 'Commenter', description: 'Can view and add comments' }
	];

	// Fetch collaborators
	async function fetchCollaborators() {
		try {
			const token = getAuthToken();
			const response = await fetch(`/api/forms/${formId}/collaborators`, {
				headers: { Authorization: `Bearer ${token}` }
			});

			if (response.ok) {
				collaborators = await response.json();
			}
		} catch (error) {
			console.error('Failed to fetch collaborators:', error);
		}
	}

	// Fetch active users (presence)
	async function fetchActiveUsers() {
		try {
			const token = getAuthToken();
			const response = await fetch(`/api/forms/${formId}/presence`, {
				headers: { Authorization: `Bearer ${token}` }
			});

			if (response.ok) {
				activeUsers = await response.json();
			}
		} catch (error) {
			console.error('Failed to fetch active users:', error);
		}
	}

	// Fetch activity feed
	async function fetchActivities() {
		try {
			const token = getAuthToken();
			const response = await fetch(`/api/forms/${formId}/activity`, {
				headers: { Authorization: `Bearer ${token}` }
			});

			if (response.ok) {
				activities = await response.json();
			}
		} catch (error) {
			console.error('Failed to fetch activities:', error);
		}
	}

	// Invite collaborator
	async function inviteCollaborator() {
		if (!inviteEmail.trim()) return;

		inviting = true;
		try {
			const token = getAuthToken();
			const response = await fetch(`/api/forms/${formId}/collaborators`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${token}`
				},
				body: JSON.stringify({
					email: inviteEmail,
					role: inviteRole
				})
			});

			if (response.ok) {
				await fetchCollaborators();
				inviteEmail = '';
				showInviteForm = false;
			}
		} catch (error) {
			console.error('Failed to invite collaborator:', error);
		}
		inviting = false;
	}

	// Update collaborator role
	async function updateRole(userId: number, newRole: string) {
		try {
			const token = getAuthToken();
			await fetch(`/api/forms/${formId}/collaborators/${userId}`, {
				method: 'PATCH',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${token}`
				},
				body: JSON.stringify({ role: newRole })
			});

			await fetchCollaborators();
		} catch (error) {
			console.error('Failed to update role:', error);
		}
	}

	// Remove collaborator
	async function removeCollaborator(userId: number) {
		if (!confirm('Are you sure you want to remove this collaborator?')) return;

		try {
			const token = getAuthToken();
			await fetch(`/api/forms/${formId}/collaborators/${userId}`, {
				method: 'DELETE',
				headers: { Authorization: `Bearer ${token}` }
			});

			await fetchCollaborators();
		} catch (error) {
			console.error('Failed to remove collaborator:', error);
		}
	}

	// Format relative time
	function formatRelativeTime(dateString: string | null): string {
		if (!dateString) return 'Never';

		const date = new Date(dateString);
		const now = new Date();
		const diffMs = now.getTime() - date.getTime();
		const diffMins = Math.floor(diffMs / 60000);

		if (diffMins < 1) return 'Just now';
		if (diffMins < 60) return `${diffMins}m ago`;

		const diffHours = Math.floor(diffMins / 60);
		if (diffHours < 24) return `${diffHours}h ago`;

		const diffDays = Math.floor(diffHours / 24);
		if (diffDays < 7) return `${diffDays}d ago`;

		return date.toLocaleDateString();
	}

	// Check if user is online
	function isOnline(userId: number): boolean {
		return activeUsers.some((u) => u.user_id === userId);
	}

	// Get user avatar color
	function getAvatarColor(userId: number): string {
		const user = activeUsers.find((u) => u.user_id === userId);
		return user?.avatar_color || '#6b7280';
	}

	// Initial load
	onMount(async () => {
		loading = true;
		await Promise.all([fetchCollaborators(), fetchActiveUsers(), fetchActivities()]);
		loading = false;

		// Poll for presence updates
		pollingInterval = setInterval(fetchActiveUsers, 10000);
	});

	onDestroy(() => {
		if (pollingInterval) {
			clearInterval(pollingInterval);
		}
	});
</script>

<div class="collaborators-panel">
	<div class="panel-header">
		<h3>Team Collaboration</h3>
		<button class="btn-invite" onclick={() => (showInviteForm = !showInviteForm)}>
			{showInviteForm ? 'Cancel' : 'Invite'}
		</button>
	</div>

	{#if showInviteForm}
		<div class="invite-form">
			<div class="form-row">
				<input
					type="email"
					bind:value={inviteEmail}
					placeholder="Email address"
					class="email-input"
				/>
				<select bind:value={inviteRole} class="role-select">
					{#each roles as role}
						<option value={role.value}>{role.label}</option>
					{/each}
				</select>
			</div>
			<button
				class="btn-send-invite"
				onclick={inviteCollaborator}
				disabled={inviting || !inviteEmail}
			>
				{inviting ? 'Inviting...' : 'Send Invite'}
			</button>
		</div>
	{/if}

	{#if loading}
		<div class="loading">Loading...</div>
	{:else}
		<!-- Active Users -->
		{#if activeUsers.length > 0}
			<div class="section">
				<div class="section-header">
					<span class="online-dot"></span>
					<span>{activeUsers.length} online now</span>
				</div>
				<div class="active-users">
					{#each activeUsers as user}
						<div
							class="user-avatar"
							style="background-color: {user.avatar_color}"
							title={user.name}
						>
							{user.name.charAt(0).toUpperCase()}
						</div>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Collaborators List -->
		<div class="section">
			<div class="section-header">Collaborators ({collaborators.length})</div>
			<div class="collaborators-list">
				{#each collaborators as collab}
					<div class="collaborator-item">
						<div
							class="collab-avatar"
							class:online={isOnline(collab.id)}
							style="background-color: {getAvatarColor(collab.id)}"
						>
							{(collab.name || '?').charAt(0).toUpperCase()}
						</div>
						<div class="collab-info">
							<div class="collab-name">{collab.name || ''}</div>
							<div class="collab-email">{collab.email || ''}</div>
							<div class="collab-meta">
								{#if isOnline(collab.id)}
									<span class="status online">Online</span>
								{:else}
									<span class="status">Last seen {formatRelativeTime(collab.last_active_at)}</span>
								{/if}
							</div>
						</div>
						<div class="collab-actions">
							<select
								value={collab.role}
								onchange={(e: Event) =>
									updateRole(collab.id, (e.target as HTMLSelectElement).value)}
								class="role-badge"
							>
								{#each roles as role}
									<option value={role.value}>{role.label}</option>
								{/each}
							</select>
							<button
								class="btn-remove"
								onclick={() => removeCollaborator(collab.id)}
								title="Remove collaborator"
							>
								&times;
							</button>
						</div>
					</div>
				{/each}

				{#if collaborators.length === 0}
					<div class="empty-state">
						<p>No collaborators yet</p>
						<button class="btn-invite-first" onclick={() => (showInviteForm = true)}>
							Invite your first team member
						</button>
					</div>
				{/if}
			</div>
		</div>

		<!-- Activity Feed -->
		<div class="section">
			<div class="section-header">Recent Activity</div>
			<div class="activity-feed">
				{#each activities.slice(0, 10) as activity}
					<div class="activity-item">
						<div class="activity-icon">
							{#if activity.action.includes('comment')}
								💬
							{:else if activity.action.includes('field')}
								📝
							{:else if activity.action.includes('publish')}
								🚀
							{:else}
								📋
							{/if}
						</div>
						<div class="activity-content">
							<div class="activity-description">{activity.description}</div>
							<div class="activity-time">{formatRelativeTime(activity.created_at)}</div>
						</div>
					</div>
				{/each}

				{#if activities.length === 0}
					<div class="empty-activity">No recent activity</div>
				{/if}
			</div>
		</div>
	{/if}
</div>

<style>
	.collaborators-panel {
		background: white;
		border-radius: 0.75rem;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		overflow: hidden;
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.panel-header h3 {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: #111827;
	}

	.btn-invite {
		padding: 0.375rem 0.75rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 0.375rem;
		font-size: 0.75rem;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.2s;
	}

	.btn-invite:hover {
		background: #2563eb;
	}

	.invite-form {
		padding: 1rem 1.25rem;
		background: #f9fafb;
		border-bottom: 1px solid #e5e7eb;
	}

	.form-row {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
	}

	.email-input {
		flex: 1;
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 0.875rem;
	}

	.email-input:focus {
		outline: none;
		border-color: #3b82f6;
	}

	.role-select {
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		background: white;
	}

	.btn-send-invite {
		width: 100%;
		padding: 0.5rem;
		background: #10b981;
		color: white;
		border: none;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.2s;
	}

	.btn-send-invite:hover:not(:disabled) {
		background: #059669;
	}

	.btn-send-invite:disabled {
		background: #9ca3af;
		cursor: not-allowed;
	}

	.loading {
		padding: 2rem;
		text-align: center;
		color: #6b7280;
	}

	.section {
		padding: 1rem 1.25rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.section:last-child {
		border-bottom: none;
	}

	.section-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.75rem;
		font-weight: 600;
		color: #6b7280;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.75rem;
	}

	.online-dot {
		width: 8px;
		height: 8px;
		background: #10b981;
		border-radius: 50%;
		animation: pulse 2s infinite;
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	.active-users {
		display: flex;
		gap: 0.5rem;
	}

	.user-avatar {
		width: 32px;
		height: 32px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: default;
	}

	.collaborators-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.collaborator-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.5rem;
		border-radius: 0.5rem;
		transition: background 0.2s;
	}

	.collaborator-item:hover {
		background: #f9fafb;
	}

	.collab-avatar {
		width: 36px;
		height: 36px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		font-weight: 600;
		position: relative;
	}

	.collab-avatar.online::after {
		content: '';
		position: absolute;
		bottom: 0;
		right: 0;
		width: 10px;
		height: 10px;
		background: #10b981;
		border: 2px solid white;
		border-radius: 50%;
	}

	.collab-info {
		flex: 1;
		min-width: 0;
	}

	.collab-name {
		font-size: 0.875rem;
		font-weight: 500;
		color: #111827;
	}

	.collab-email {
		font-size: 0.75rem;
		color: #6b7280;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.collab-meta {
		font-size: 0.625rem;
		margin-top: 0.125rem;
	}

	.status {
		color: #9ca3af;
	}

	.status.online {
		color: #10b981;
	}

	.collab-actions {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.role-badge {
		padding: 0.25rem 0.5rem;
		font-size: 0.75rem;
		border: 1px solid #e5e7eb;
		border-radius: 0.25rem;
		background: white;
		cursor: pointer;
	}

	.btn-remove {
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: none;
		border: none;
		color: #9ca3af;
		font-size: 1.25rem;
		cursor: pointer;
		border-radius: 0.25rem;
		transition: all 0.2s;
	}

	.btn-remove:hover {
		background: #fee2e2;
		color: #dc2626;
	}

	.empty-state {
		text-align: center;
		padding: 1.5rem;
	}

	.empty-state p {
		color: #6b7280;
		margin: 0 0 1rem 0;
	}

	.btn-invite-first {
		padding: 0.5rem 1rem;
		background: #eff6ff;
		color: #3b82f6;
		border: 1px solid #3b82f6;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-invite-first:hover {
		background: #3b82f6;
		color: white;
	}

	.activity-feed {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		max-height: 300px;
		overflow-y: auto;
	}

	.activity-item {
		display: flex;
		gap: 0.75rem;
	}

	.activity-icon {
		font-size: 1rem;
		flex-shrink: 0;
	}

	.activity-content {
		flex: 1;
		min-width: 0;
	}

	.activity-description {
		font-size: 0.8125rem;
		color: #374151;
	}

	.activity-time {
		font-size: 0.6875rem;
		color: #9ca3af;
		margin-top: 0.125rem;
	}

	.empty-activity {
		text-align: center;
		color: #9ca3af;
		font-size: 0.875rem;
		padding: 1rem;
	}
</style>
