<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import {
		managerAPI,
		type ManagerRecord,
		type ManagerRoleRecord,
		type ManagerUserOption,
		type ManagerPage
	} from '#lib/api/crm-managers.js';
	import { useCrmAccess } from '#lib/crm/access.svelte.js';
	let { id }: { id?: string } = $props();
	const access = useCrmAccess();
	let saved = $state<ManagerRecord | null>(null),
		roles = $state<ManagerPage<ManagerRoleRecord> | null>(null),
		users = $state<ManagerPage<ManagerUserOption> | null>(null),
		roleSearch = $state(''),
		userSearch = $state(''),
		rolePage = $state(1),
		userPage = $state(1),
		roleId = $state(''),
		userId = $state(''),
		loading = $state(true),
		busy = $state(false),
		error = $state('');
	let disposed = false;
	let selectedRole = $state<ManagerRoleRecord | null>(null),
		selectedUser = $state<ManagerUserOption | null>(null);
	const roleOptions = $derived(
		selectedRole && !roles?.data.some((role) => role.id === selectedRole?.id)
			? [selectedRole, ...(roles?.data ?? [])]
			: (roles?.data ?? [])
	);
	const userOptions = $derived(
		selectedUser && !users?.data.some((user) => user.id === selectedUser?.id)
			? [selectedUser, ...(users?.data ?? [])]
			: (users?.data ?? [])
	);
	async function search(kind: 'roles' | 'users', target = 1) {
		error = '';
		try {
			if (kind === 'roles') {
				const result = await managerAPI.roles(target, roleSearch);
				if (!disposed) {
					roles = result;
					rolePage = target;
				}
			} else {
				if (!access.can('GET', '/managers/users'))
					throw new Error('Only global administrators may search the active user roster');
				const result = await managerAPI.users(target, userSearch);
				if (!disposed) {
					users = result;
					userPage = target;
				}
			}
		} catch (cause) {
			if (!disposed)
				error = cause instanceof Error ? cause.message : 'Unable to load current choices';
		}
	}
	async function load() {
		loading = true;
		error = '';
		try {
			if (id) {
				const value = await managerAPI.manager(id);
				if (disposed) return;
				saved = value;
				userId = value.user_id;
				roleId = value.role_id;
				selectedRole = value.role;
			}
			await search('roles');
			if (!id) await search('users');
		} catch (cause) {
			error = cause instanceof Error ? cause.message : 'Unable to load manager assignment';
		} finally {
			loading = false;
		}
	}
	async function save(event: SubmitEvent) {
		event.preventDefault();
		if (busy) return;
		if (!access.can(id ? 'PUT' : 'POST', id ? '/managers/{id}' : '/managers')) {
			error = 'A global administrator must manage CRM assignments';
			return;
		}
		if (
			!roleOptions.some((role) => role.id === roleId) ||
			(!id && !userOptions.some((user) => user.id === userId))
		) {
			error = 'Choose actual saved user and role records';
			return;
		}
		busy = true;
		error = '';
		try {
			if (saved) await managerAPI.reassign(saved.id, roleId, saved.version);
			else await managerAPI.assign(userId, roleId);
			await goto('/admin/crm/managers', { invalidateAll: true });
		} catch (cause) {
			error =
				cause instanceof Error
					? cause.message
					: 'Assignment was not confirmed. Reload before retrying.';
		} finally {
			busy = false;
		}
	}
	onMount(() => {
		void load();
		return () => {
			disposed = true;
		};
	});
</script>

<section>
	<a href="/admin/crm/managers">← CRM managers</a>
	<h1>{id ? 'Edit CRM assignment' : 'Assign CRM manager'}</h1>
	<p>
		Assignments grant only the selected CRM capabilities. They do not change global account roles,
		provide member administration, or grant native email access.
	</p>
	{#if error}<p role="alert">{error}</p>
		<button onclick={() => void load()} disabled={busy}>Reload saved assignment</button
		>{/if}{#if loading}<p role="status">Loading current records…</p>{:else}
		{#if saved}<p>
				Account: {saved.email} · Global role: {saved.global_role ?? 'Not recorded'} · Assignment version
				{saved.version}.
			</p>{:else}<form
				onsubmit={(event) => {
					event.preventDefault();
					void search('users');
				}}
			>
				<label>Search active users<input bind:value={userSearch} maxlength="500" /></label><button
					>Search users</button
				>
			</form>
			<nav aria-label="User roster pages">
				<button disabled={userPage <= 1 || busy} onclick={() => void search('users', userPage - 1)}
					>Previous users</button
				><span>Page {userPage} of {users?.meta.last_page ?? 1}</span><button
					disabled={busy || userPage >= (users?.meta.last_page ?? 1)}
					onclick={() => void search('users', userPage + 1)}>Next users</button
				>
			</nav>{/if}
		<form
			onsubmit={(event) => {
				event.preventDefault();
				void search('roles');
			}}
		>
			<label>Search CRM roles<input bind:value={roleSearch} maxlength="500" /></label><button
				>Search roles</button
			>
		</form>
		<nav aria-label="Role picker pages">
			<button disabled={rolePage <= 1 || busy} onclick={() => void search('roles', rolePage - 1)}
				>Previous roles</button
			><span>Page {rolePage} of {roles?.meta.last_page ?? 1}</span><button
				disabled={busy || rolePage >= (roles?.meta.last_page ?? 1)}
				onclick={() => void search('roles', rolePage + 1)}>Next roles</button
			>
		</nav>
		<form onsubmit={(event) => void save(event)}>
			<fieldset disabled={busy}>
				<legend>Current assignment</legend>{#if !saved}<label
						>Active user<select
							bind:value={userId}
							required
							onchange={() => {
								selectedUser = userOptions.find((user) => user.id === userId) ?? null;
							}}
							><option value="">Choose an actual account</option
							>{#each userOptions as user (user.id)}<option
									value={user.id}
									disabled={user.manager_id !== null}
									>{user.email}{user.manager_id ? ' — already assigned' : ''}</option
								>{/each}</select
						></label
					>{/if}<label
					>CRM role<select
						bind:value={roleId}
						required
						onchange={() => {
							selectedRole = roleOptions.find((role) => role.id === roleId) ?? null;
						}}
						><option value="">Choose a saved role</option
						>{#each roleOptions as role (role.id)}<option value={role.id}
								>{role.name}{role.is_default ? ' (default selection)' : ''}</option
							>{/each}</select
					></label
				>
				<p>
					No role is assigned automatically. Review the role's module dependencies before saving.
				</p>
				<button disabled={!access.can(id ? 'PUT' : 'POST', id ? '/managers/{id}' : '/managers')}
					>{busy ? 'Saving…' : 'Save CRM assignment'}</button
				>
			</fieldset>
		</form>{/if}
</section>

<style>
	section {
		max-width: 64rem;
		padding: 2rem;
		color: #e2e8f0;
	}
	form,
	fieldset {
		display: grid;
		gap: 1rem;
		margin: 1rem 0;
	}
	fieldset {
		border: 1px solid #475569;
		padding: 1rem;
	}
	label {
		display: grid;
		gap: 0.5rem;
	}
	input,
	select,
	button {
		padding: 0.65rem;
		background: #0f172a;
		color: inherit;
		border: 1px solid #64748b;
		border-radius: 0.4rem;
	}
	nav {
		display: flex;
		gap: 1rem;
	}
	a {
		color: #7dd3fc;
	}
	[role='alert'] {
		color: #fca5a5;
	}
</style>
