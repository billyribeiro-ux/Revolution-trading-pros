<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import {
		managerAPI,
		type ManagerRecord,
		type ManagerRoleRecord,
		type ManagerPage
	} from '#lib/api/crm-managers.js';
	import { useCrmAccess } from '#lib/crm/access.svelte.js';
	import ConfirmationModal from '#lib/components/admin/ConfirmationModal.svelte';
	const access = useCrmAccess();
	let tab = $state<'managers' | 'roles'>('managers'),
		search = $state(''),
		currentPage = $state(1),
		managers = $state<ManagerPage<ManagerRecord> | null>(null),
		roles = $state<ManagerPage<ManagerRoleRecord> | null>(null),
		loading = $state(true),
		busy = $state(false),
		error = $state(''),
		notice = $state('');
	let generation = 0,
		disposed = false;
	let pending = $state<{
		kind: 'managers' | 'manager-roles';
		id: string;
		version: number;
		label: string;
	} | null>(null);
	const meta = $derived(tab === 'managers' ? managers?.meta : roles?.meta);
	async function load() {
		const request = ++generation;
		loading = true;
		error = '';
		try {
			if (tab === 'managers') {
				if (!access.can('GET', '/managers'))
					throw new Error('CRM assignment records are not permitted');
				const result = await managerAPI.managers(currentPage, search);
				if (!disposed && request === generation) managers = result;
			} else {
				if (!access.can('GET', '/manager-roles'))
					throw new Error('CRM role records are not permitted');
				const result = await managerAPI.roles(currentPage, search);
				if (!disposed && request === generation) roles = result;
			}
		} catch (cause) {
			if (request === generation) {
				managers = null;
				roles = null;
				error =
					cause instanceof Error ? cause.message : 'Unable to load current CRM security records';
			}
		} finally {
			if (request === generation) loading = false;
		}
	}
	function changeTab(next: 'managers' | 'roles') {
		tab = next;
		currentPage = 1;
		search = '';
		void load();
	}
	async function remove() {
		const target = pending;
		if (!target || busy) return;
		pending = null;
		busy = true;
		error = '';
		notice = '';
		try {
			if (
				!access.can('DELETE', target.kind === 'managers' ? '/managers/{id}' : '/manager-roles/{id}')
			)
				throw new Error('A global administrator must change CRM security');
			await managerAPI.remove(target.kind, target.id, target.version);
			notice =
				target.kind === 'managers'
					? 'CRM assignment removed. The global account role is unchanged.'
					: 'CRM role deleted.';
			await load();
		} catch (cause) {
			error =
				cause instanceof Error
					? cause.message
					: 'Deletion was not confirmed. Reload before retrying.';
		} finally {
			busy = false;
		}
	}
	onMount(() => {
		tab = page.url.searchParams.get('tab') === 'roles' ? 'roles' : 'managers';
		void load();
		return () => {
			disposed = true;
			generation++;
		};
	});
</script>

<section>
	<h1>CRM managers and roles</h1>
	<p>
		Manage explicit CRM delegation. Security mutations and the active user roster require a global
		administrator; no default role is assigned automatically.
	</p>
	<nav aria-label="CRM security sections">
		<button onclick={() => changeTab('managers')} aria-pressed={tab === 'managers'}>Managers</button
		><button onclick={() => changeTab('roles')} aria-pressed={tab === 'roles'}>Roles</button><button
			disabled={loading || busy}
			onclick={() => void load()}>Refresh</button
		>{#if access.page('/admin/crm/managers/new')}<a href="/admin/crm/managers/new">Add Manager</a
			>{/if}{#if access.page('/admin/crm/managers/roles/new')}<a
				href="/admin/crm/managers/roles/new">Create Role</a
			>{/if}
	</nav>
	<form
		onsubmit={(event) => {
			event.preventDefault();
			currentPage = 1;
			void load();
		}}
	>
		<label>Search {tab}<input bind:value={search} maxlength="500" /></label><button disabled={busy}
			>Search</button
		>
	</form>
	{#if error}<p role="alert">{error}</p>{/if}{#if notice}<p role="status">
			{notice}
		</p>{/if}{#if loading}<p role="status">Loading current {tab}…</p>{:else if !error}<p>
			{meta?.total ?? 0} recorded {tab} · Page {meta?.current_page ?? 1} of {meta?.last_page ?? 1}.
		</p>
		{#if tab === 'managers'}<table>
				<thead
					><tr
						><th>Actual account</th><th>CRM role</th><th>Global role</th><th>Assignment</th><th
							>Actions</th
						></tr
					></thead
				><tbody
					>{#each managers?.data ?? [] as manager (manager.id)}<tr
							><td>{manager.email}<br />{manager.name ?? 'Name not recorded'}</td><td
								>{manager.role.name}</td
							><td>{manager.global_role ?? 'Not recorded'}</td><td
								>Version {manager.version} · {manager.is_active
									? 'Active account'
									: 'Inactive account'}</td
							><td
								>{#if access.page(`/admin/crm/managers/${manager.id}/edit`)}<a
										href="/admin/crm/managers/{manager.id}/edit">Edit assignment</a
									>{/if}<button
									disabled={busy || !access.can('DELETE', '/managers/{id}')}
									onclick={() => {
										pending = {
											kind: 'managers',
											id: manager.id,
											version: manager.version,
											label: manager.email
										};
									}}>Remove assignment</button
								></td
							></tr
						>{/each}</tbody
				>
			</table>{:else}<table>
				<thead
					><tr
						><th>Role</th><th>Assignments</th><th>Selection</th><th>Version</th><th>Actions</th></tr
					></thead
				><tbody
					>{#each roles?.data ?? [] as role (role.id)}<tr
							><td>{role.name}<br />{role.description ?? 'No description'}</td><td
								>{role.users_count}</td
							><td>{role.is_default ? 'Default selection' : 'Optional selection'}</td><td
								>{role.version}</td
							><td
								>{#if access.page(`/admin/crm/managers/roles/${role.id}/edit`)}<a
										href="/admin/crm/managers/roles/{role.id}/edit">Edit role</a
									>{/if}<button
									disabled={busy || !access.can('DELETE', '/manager-roles/{id}')}
									onclick={() => {
										pending = {
											kind: 'manager-roles',
											id: role.id,
											version: role.version,
											label: role.name
										};
									}}>Delete role</button
								></td
							></tr
						>{/each}</tbody
				>
			</table>{/if}
		<nav aria-label="CRM security pages">
			<button
				disabled={busy || currentPage <= 1}
				onclick={() => {
					currentPage--;
					void load();
				}}>Previous page</button
			><button
				disabled={busy || currentPage >= (meta?.last_page ?? 1)}
				onclick={() => {
					currentPage++;
					void load();
				}}>Next page</button
			>
		</nav>{/if}
</section>
<ConfirmationModal
	isOpen={pending !== null}
	title={pending?.kind === 'managers' ? 'Remove CRM assignment' : 'Delete CRM role'}
	message={`Confirm ${pending?.label ?? 'this record'} using saved version ${pending?.version ?? ''}. Assigned or default roles cannot be deleted.`}
	confirmText="Confirm security change"
	variant="danger"
	onConfirm={() => void remove()}
	onCancel={() => {
		pending = null;
	}}
/>

<style>
	section {
		padding: 2rem;
		color: #e2e8f0;
	}
	nav {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;
		margin: 1rem 0;
	}
	form,
	label {
		display: flex;
		gap: 1rem;
		align-items: center;
	}
	input,
	button {
		padding: 0.65rem;
		background: #0f172a;
		color: inherit;
		border: 1px solid #64748b;
		border-radius: 0.4rem;
	}
	table {
		width: 100%;
		border-collapse: collapse;
	}
	td,
	th {
		text-align: left;
		padding: 1rem;
		border-bottom: 1px solid #475569;
	}
	a {
		color: #7dd3fc;
	}
	[role='alert'] {
		color: #fca5a5;
	}
</style>
