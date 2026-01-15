<script lang="ts">
	import { emailTemplatesApi, AdminApiError } from '$lib/api/admin';
	import { goto } from '$app/navigation';
	import { IconDeviceFloppy, IconX } from '$lib/icons';

	interface Props {
		template?: any;
		isEdit?: boolean;
		onsaved?: (detail: { id: number }) => void;
	}

	let {
		template = null,
		isEdit = false,
		onsaved
	}: Props = $props();

	// Form fields
	let name = $state('');
	let slug = $state('');
	let subject = $state('');
	let email_type = $state('');
	let body_html = $state('');
	let body_text = $state('');
	let variables = $state('');
	let is_active = $state(true);

	// Sync with prop changes
	$effect(() => {
		if (template) {
			name = template.name ?? '';
			slug = template.slug ?? '';
			subject = template.subject ?? '';
			email_type = template.email_type ?? '';
			body_html = template.body_html ?? '';
			body_text = template.body_text ?? '';
			variables = template.variables ? JSON.stringify(template.variables, null, 2) : '';
			is_active = template.is_active ?? true;
		}
	});

	let loading = $state(false);
	let error = $state('');

	async function handleSubmit() {
		loading = true;
		error = '';
		const payload = {
			name,
			slug,
			subject,
			email_type,
			body_html,
			body_text: body_text || undefined,
			variables: variables ? JSON.parse(variables) : [],
			is_active
		};
		try {
			if (isEdit) {
				await emailTemplatesApi.update((template as any).id, payload);
				onsaved?.({ id: (template as any).id });
			} else {
				const created = await emailTemplatesApi.create(payload);
				onsaved?.({ id: (created as any).id });
			}
			goto('/admin/email/templates');
		} catch (e) {
			if (e instanceof AdminApiError) {
				error = e.message;
			} else {
				error = 'Failed to save template';
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

<svelte:head>
	<title>{isEdit ? 'Edit' : 'New'} Email Template | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class="template-form">
	{#if error}
		<p class="alert alert-error">{error}</p>
	{/if}
	<form onsubmit={(e: SubmitEvent) => { e.preventDefault(); handleSubmit(); }}>
		<div class="grid">
			<div class="field">
				<label for="name">Name</label>
				<input id="name" type="text" bind:value={name} required />
			</div>
			<div class="field">
				<label for="slug">Slug</label>
				<input id="slug" type="text" bind:value={slug} placeholder="auto-generated if empty" />
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
			<button
				type="button"
				class="btn-secondary"
				onclick={cancel}
				disabled={loading}
				title="Cancel"><IconX size={18} /> Cancel</button
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
		color: #e2e8f0;
	}
	input,
	textarea {
		padding: 0.75rem;
		background: rgba(15, 23, 42, 0.5);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 6px;
		color: #f1f5f9;
	}
	input:focus,
	textarea:focus {
		outline: none;
		border-color: #E6B800;
		background: rgba(15, 23, 42, 0.7);
	}
	.actions {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		margin-top: 1.5rem;
	}
	.btn-primary {
		background: linear-gradient(135deg, #E6B800, #B38F00);
		color: #0D1117;
		border: none;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		cursor: pointer;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
	.btn-secondary {
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		border: 1px solid rgba(100, 116, 139, 0.3);
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
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
		padding: 0.75rem 1rem;
		border-radius: 6px;
		margin-bottom: 1rem;
	}
</style>
