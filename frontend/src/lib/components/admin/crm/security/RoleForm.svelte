<script lang="ts">
	import { onMount } from 'svelte';
	import { currentCrmPolicy } from '#lib/config/crm-policy.js';
	const policy = currentCrmPolicy().policy;
	import { goto } from '$app/navigation';
	import {
		managerAPI,
		type ManagerRoleRecord,
		type RolePermission,
		type RolePatch
	} from '#lib/api/crm-managers.js';
	import { useCrmAccess } from '#lib/crm/access.svelte.js';
	import { crmActions, type CrmAction } from '#lib/crm/policy.js';
	let { id }: { id?: string } = $props();
	const access = useCrmAccess();
	let saved = $state<ManagerRoleRecord | null>(null),
		name = $state(''),
		slug = $state(''),
		description = $state(''),
		isDefault = $state(false),
		permissions = $state<RolePermission[]>([]),
		modules = $state<Awaited<ReturnType<typeof managerAPI.modules>>>([]),
		loading = $state(true),
		busy = $state(false),
		error = $state('');
	let disposed = false;
	function enabled(module: string, action: CrmAction) {
		return permissions.some(
			(item) => item.module === module && item.action === action && item.allowed
		);
	}
	function toggle(module: string, action: CrmAction, allowed: boolean) {
		permissions = [
			...permissions.filter((item) => item.module !== module || item.action !== action),
			{ module, action, allowed }
		];
	}
	async function load() {
		loading = true;
		error = '';
		try {
			const [catalog, role] = await Promise.all([
				managerAPI.modules(),
				id ? managerAPI.role(id) : Promise.resolve(null)
			]);
			if (disposed) return;
			modules = catalog;
			saved = role;
			if (role) {
				name = role.name;
				slug = role.slug;
				description = role.description ?? '';
				isDefault = role.is_default;
				permissions = role.permissions.map((item) => ({ ...item }));
			}
		} catch (cause) {
			if (!disposed) error = cause instanceof Error ? cause.message : 'Unable to load role';
		} finally {
			if (!disposed) loading = false;
		}
	}
	async function save(event: SubmitEvent) {
		event.preventDefault();
		if (busy) return;
		if (!access.can(id ? 'PUT' : 'POST', id ? '/manager-roles/{id}' : '/manager-roles')) {
			error = 'A global administrator with CRM security permission must save roles.';
			return;
		}
		busy = true;
		error = '';
		try {
			const fields: RolePatch = {};
			if (!saved || name.trim() !== saved.name) fields.name = name.trim();
			if ((!saved && slug.trim()) || (saved && slug.trim() !== saved.slug))
				fields.slug = slug.trim();
			const nextDescription = description.trim() || null;
			if (!saved || nextDescription !== saved.description) fields.description = nextDescription;
			if (!saved || isDefault !== saved.is_default) fields.is_default = isDefault;
			if (!saved || JSON.stringify(permissions) !== JSON.stringify(saved.permissions))
				fields.permissions = permissions;
			if (saved) {
				if (Object.keys(fields).length === 0) {
					error = 'No changes to save.';
					return;
				}
				fields.expected_version = saved.version;
			}
			await managerAPI.saveRole(id, fields);
			await goto('/admin/crm/managers?tab=roles', { invalidateAll: true });
		} catch (cause) {
			error =
				cause instanceof Error
					? cause.message
					: 'Role save was not confirmed. Reload before retrying.';
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

<section class="security-form">
	<a href="/admin/crm/managers?tab=roles">← CRM roles</a>
	<h1>{id ? 'Edit CRM role' : 'Create CRM role'}</h1>
	<p>
		CRM grants never change a user's global role or grant native member/email administration. A
		default role is a selection preference; new users are never assigned automatically.
	</p>
	{#if error}<p role="alert">{error}</p>
		<button onclick={() => void load()} disabled={busy}>Reload saved role</button
		>{/if}{#if loading}<p role="status">Loading current security records…</p>{:else}<form
			onsubmit={(event) => void save(event)}
		>
			<fieldset disabled={busy}>
				<legend>{saved ? `Saved version ${saved.version}` : 'New role'}</legend><label
					>Role name<input bind:value={name} required maxlength="100" /></label
				><label
					>Slug<input
						bind:value={slug}
						maxlength="100"
						pattern="[a-z0-9]+(?:-[a-z0-9]+)*"
						placeholder="Generated from name when empty"
					/></label
				><label>Description<textarea bind:value={description} maxlength="2000"></textarea></label
				><label><input type="checkbox" bind:checked={isDefault} /> Default selection</label
				>{#each modules as module (module.id)}<fieldset>
						<legend>{module.name}</legend>{#each module.notes as note (note)}<p>{note}</p>{/each}
						<p>
							Manage includes view, create, edit and delete within this module. Dependencies listed
							above still require their separate grants.
						</p>
						<details>
							<summary>Operation requirements</summary>
							<p>
								Every listed grant is required together. Conditional requirements apply to the
								actual saved source, request body or retained history.
							</p>
							<ul>
								{#each policy.routes.filter((route) => route.all[0]?.module === module.id) as route (`${route.methods.join(',')}:${route.path}:${JSON.stringify(route.query_equals)}`)}<li
									>
										<code
											>{route.methods.filter((method) => method !== 'HEAD').join(' / ')}
											{route.path}</code
										>: {route.all
											.map((grant) => `${grant.module}.${grant.action}`)
											.join(' + ')}{route.global_admin_only
											? ' · Global administrator required'
											: ''}{#if route.conditional.length}<ul>
												{#each route.conditional as factId (factId)}{@const fact =
														policy.facts.find((item) => item.id === factId)}{#if fact}<li>
															{fact.description}: {fact.all
																.map((grant) => `${grant.module}.${grant.action}`)
																.join(' + ')}{fact.global_admin_only
																? ' Global administrator required.'
																: ''}
														</li>{/if}{/each}
											</ul>{/if}
									</li>{/each}
							</ul>
						</details>
						<div class="permissions">
							{#each crmActions as action (action)}<label
									><input
										type="checkbox"
										checked={enabled(module.id, action) ||
											(action !== 'manage' && enabled(module.id, 'manage'))}
										disabled={action !== 'manage' && enabled(module.id, 'manage')}
										onchange={(event) => toggle(module.id, action, event.currentTarget.checked)}
									/>{action}</label
								>{/each}
						</div>
					</fieldset>{/each}<button
					type="submit"
					disabled={!access.can(id ? 'PUT' : 'POST', id ? '/manager-roles/{id}' : '/manager-roles')}
					>{busy ? 'Saving…' : 'Save CRM role'}</button
				>
			</fieldset>
		</form>{/if}
</section>

<style>
	.security-form {
		max-width: 70rem;
		padding: 2rem;
		color: #e2e8f0;
	}
	form,
	fieldset {
		display: grid;
		gap: 1rem;
	}
	fieldset {
		border: 1px solid #475569;
		border-radius: 0.5rem;
		padding: 1rem;
	}
	label {
		display: grid;
		gap: 0.5rem;
	}
	input,
	textarea,
	button {
		padding: 0.65rem;
		background: #0f172a;
		color: inherit;
		border: 1px solid #64748b;
		border-radius: 0.4rem;
	}
	.permissions {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;
	}
	.permissions label {
		display: flex;
		align-items: center;
	}
	a {
		color: #7dd3fc;
	}
	[role='alert'] {
		color: #fca5a5;
	}
</style>
