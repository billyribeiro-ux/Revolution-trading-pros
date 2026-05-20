<script lang="ts">
	/**
	 * R27-D extraction (2026-05-20): "Roles & Permissions" section of
	 * the admin user-edit form.
	 *
	 * FIX (incidental): the original markup gave BOTH role checkboxes
	 * the same `id="page-checkbox"` AND the same `name="page-checkbox"`
	 * — that's invalid HTML (duplicate `id` in the same DOM) and the
	 * `name` collision means form-data POSTs would have shipped only
	 * one value. After extraction each role gets a unique
	 * `role-{name}` id; checkboxes that toggle pure UI state don't
	 * need a `name` at all since the parent owns `formData.roles`.
	 */
	import { IconUser } from '$lib/icons';

	interface RoleOption {
		key: string;
		label: string;
		description: string;
		iconClass: 'admin' | 'super-admin';
	}

	interface Props {
		roles: string[];
		onToggleRole: (role: string) => void;
	}

	let { roles, onToggleRole }: Props = $props();

	// Bounded set — adding a new role here is the only place to touch.
	const options: RoleOption[] = [
		{
			key: 'admin',
			label: 'Admin',
			description: 'Full access to admin panel',
			iconClass: 'admin'
		},
		{
			key: 'super-admin',
			label: 'Super Admin',
			description: 'Complete system control',
			iconClass: 'super-admin'
		}
	];
</script>

<div class="form-section">
	<h2>Roles & Permissions</h2>

	<div class="roles-grid">
		{#each options as opt (opt.key)}
			<label class="role-checkbox">
				<input
					id={`role-${opt.key}`}
					type="checkbox"
					checked={roles.includes(opt.key)}
					onchange={() => onToggleRole(opt.key)}
				/>
				<div class="role-card">
					<div class="role-icon {opt.iconClass}">
						<IconUser size={24} />
					</div>
					<div class="role-info">
						<div class="role-name">{opt.label}</div>
						<div class="role-description">{opt.description}</div>
					</div>
				</div>
			</label>
		{/each}
	</div>
</div>

<style>
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
</style>
