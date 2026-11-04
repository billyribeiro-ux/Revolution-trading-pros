<script lang="ts">
	import {
		emailTemplatesApi,
		isEmailTemplateVariables,
		isEmailTemplate,
		AdminApiError,
		type EmailTemplate
	} from '#lib/api/admin.js';
	import { goto } from '$app/navigation';
	import { IconDeviceFloppy, IconX } from '#lib/icons/index.js';
	import { untrack } from 'svelte';

	/** The keyed edit route supplies a validated row; legacy data envelopes remain supported. */

	type TemplateEnvelope = { data: Partial<EmailTemplate> } & Partial<EmailTemplate>;

	interface Props {
		template?: Partial<EmailTemplate> | TemplateEnvelope | null;
		isEdit?: boolean;
		onsaved?: (detail: { id: number }) => void;
	}

	let props: Props = $props();

	// Unwrap a `{ data: ... }` envelope if the caller forgot to.
	function unwrap(t: Props['template']): Partial<EmailTemplate> | null {
		if (!t) return null;
		if (typeof t === 'object' && 'data' in t && t.data && typeof t.data === 'object') {
			return t.data as Partial<EmailTemplate>;
		}
		return t as Partial<EmailTemplate>;
	}

	// Destructure with defaults for internal use
	const template = $derived(unwrap(props.template));
	const isEdit = $derived(props.isEdit ?? false);
	const onsaved = $derived(props.onsaved);

	// Form fields — seeded once at mount.
	//
	// Both routes mount this form with a stable `template`: the edit page gates
	// it behind `{#if loading}…{:else}` so it only mounts once the row has
	// loaded (non-null), and the new page passes no template at all. Neither
	// swaps `template` on a live instance, so seeding once at mount replaces the
	// prop→state sync $effect without any behavioural change. `untrack` reads
	// the prop's initial value without registering a reactive dep (init-once
	// intent; avoids state_referenced_locally).
	const seed = untrack(() => template);

	let name = $state(seed?.name ?? '');
	let slug = $state(seed?.slug ?? '');
	let subject = $state(seed?.subject ?? '');
	let email_type = $state(seed?.email_type ?? 'transactional');
	let body_html = $state(seed?.body_html ?? seed?.body ?? '');
	let body_text = $state(seed?.body_text ?? '');
	let variables = $state(seed?.variables ? JSON.stringify(seed.variables, null, 2) : '');
	let is_active = $state(seed?.is_active ?? true);

	let loading = $state(false);
	let error = $state('');

	async function handleSubmit() {
		if (loading) return;
		loading = true;
		error = '';
		try {
			const parsed: unknown = variables ? JSON.parse(variables) : [];
			if (!isEmailTemplateVariables(parsed))
				throw new Error('Variables must be a JSON array of names or objects with a key.');
			const payload = {
				name,
				slug,
				subject,
				email_type,
				body_html,
				body_text,
				variables: parsed,
				is_active
			};
			if (isEdit) {
				const existingId = template?.id;
				if (existingId === undefined) {
					throw new Error('Cannot update template: missing id');
				}
				const response = await emailTemplatesApi.update(existingId, payload);
				if (!isEmailTemplate(response.data))
					throw new Error(
						'Invalid saved email template response. Reload the list before retrying.'
					);
				onsaved?.({ id: response.data.id });
			} else {
				const response = await emailTemplatesApi.create(payload);
				if (!isEmailTemplate(response.data))
					throw new Error(
						'Invalid saved email template response. Reload the list before retrying.'
					);
				onsaved?.({ id: response.data.id });
			}
			await goto('/admin/email/templates');
		} catch (e) {
			if (e instanceof AdminApiError) {
				error = e.message;
			} else {
				error = e instanceof Error ? e.message : 'Failed to save template';
			}
			console.error('Failed to save template:', e);
		} finally {
			loading = false;
		}
	}

	function cancel() {
		goto('/admin/email/templates');
	}
</script>

<div class="template-form">
	{#if error}
		<p class="alert alert-error" role="alert">{error}</p>
	{/if}
	<form
		onsubmit={(e: SubmitEvent) => {
			e.preventDefault();
			handleSubmit();
		}}
	>
		<div class="grid">
			<div class="field">
				<label for="name">Name</label>
				<input id="name" type="text" bind:value={name} required />
			</div>
			<div class="field">
				<label for="slug">Slug</label>
				<input id="slug" type="text" bind:value={slug} placeholder="auto-generated if empty" />
				<small
					>The slug identifies this template in application messages. Saving content updates those
					messages.</small
				>
			</div>
			<div class="field full">
				<label for="subject">Subject</label>
				<input id="subject" type="text" bind:value={subject} required />
			</div>
			<div class="field full">
				<label for="email_type">Email Type</label>
				<input
					id="email_type"
					type="text"
					bind:value={email_type}
					required
					placeholder="e.g. welcome, order_confirmation"
				/>
			</div>
			<div class="field full">
				<label for="body_html">HTML Body</label>
				<textarea id="body_html" rows="8" bind:value={body_html} required></textarea>
			</div>
			<div class="field full">
				<label for="body_text">Plain Text Body (optional)</label>
				<textarea id="body_text" rows="4" bind:value={body_text}></textarea>
			</div>
			<div class="field full">
				<label for="variables">Variables (JSON)</label>
				<textarea id="variables" rows="4" bind:value={variables} placeholder="Enter JSON variables"
				></textarea>
			</div>
			<div class="field">
				<label for="is-active" class="inline">
					<input id="is-active" type="checkbox" bind:checked={is_active} /> Active
				</label>
			</div>
		</div>
		<div class="actions">
			<button type="button" class="btn-secondary" onclick={cancel} disabled={loading} title="Cancel"
				><IconX size={18} /> Cancel</button
			>
			<button type="submit" class="btn-primary" disabled={loading} title="Save">
				{#if loading}
					Saving...
				{:else}
					<IconDeviceFloppy size={18} /> Save
				{/if}
			</button>
		</div>
	</form>
</div>

<style>
	.template-form {
		max-width: 900px;
		margin: 0 auto;
		padding: 2rem;
	}
	.grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}
	.field {
		display: flex;
		flex-direction: column;
	}
	.field.full {
		grid-column: 1 / -1;
	}
	label {
		font-weight: 600;
		margin-bottom: 0.25rem;
		color: var(--text-primary, #f0f6fc);
	}
	input,
	textarea {
		padding: 0.75rem;
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 6px;
		color: var(--text-primary, #f0f6fc);
	}
	input:focus,
	textarea:focus {
		outline: none;
		border-color: #e6b800;
		background: var(--bg-base, #0d1117);
	}
	.actions {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		margin-top: 1.5rem;
	}
	.btn-primary {
		background: #e6b800;
		color: #0d1117;
		border: none;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		cursor: pointer;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
	.btn-secondary {
		background: var(--bg-hover, #252b33);
		color: var(--text-secondary, #8b949e);
		border: 1px solid var(--border-default, #30363d);
		padding: 0.5rem 1rem;
		border-radius: 6px;
		cursor: pointer;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
	.btn-primary:hover,
	.btn-secondary:hover {
		opacity: 0.9;
	}
	.alert-error {
		background: rgba(248, 81, 73, 0.1);
		color: var(--error-emphasis, #f85149);
		padding: 0.75rem 1rem;
		border-radius: 6px;
		margin-bottom: 1rem;
	}
</style>
