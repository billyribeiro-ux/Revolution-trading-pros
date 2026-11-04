<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	let { id }: { id: string } = $props();
	import { emailTemplatesApi, type JsonValue } from '#lib/api/admin.js';
	import { sanitizeHtml } from '#lib/utils/sanitize.js';
	let loading = $state(true),
		error = $state(''),
		sample = $state('{}');
	let preview = $state<{ subject: string; html: string; text: string | null } | null>(null);
	let generation = 0;
	async function load() {
		const current = ++generation;
		loading = true;
		error = '';
		try {
			const key = Number(id);
			if (!Number.isSafeInteger(key) || key <= 0) throw new Error('Invalid template identifier');
			const data: unknown = JSON.parse(sample);
			if (!data || typeof data !== 'object' || Array.isArray(data))
				throw new Error('Preview data must be a JSON object');
			const response = await emailTemplatesApi.preview(key, data as Record<string, JsonValue>);
			if (
				!response.data ||
				typeof response.data.subject !== 'string' ||
				typeof response.data.html !== 'string'
			)
				throw new Error('Invalid template preview response');
			if (current === generation) preview = response.data;
		} catch (cause) {
			if (current === generation) {
				preview = null;
				error = cause instanceof Error ? cause.message : 'Unable to load preview';
			}
		} finally {
			if (current === generation) loading = false;
		}
	}
	onMount(() => {
		void load();
	});
	onDestroy(() => {
		generation++;
	});
</script>

<div class="preview-page">
	<h1>Email template preview</h1>
	<form
		onsubmit={(event) => {
			event.preventDefault();
			void load();
		}}
	>
		<label for="sample-data">Preview data (JSON)</label>
		<textarea id="sample-data" rows="6" bind:value={sample}></textarea>
		<p>Provide values for the template variables. Preview does not send an email.</p>
		<button type="submit" class="btn-secondary" disabled={loading}>Preview template</button>
	</form>
	{#if loading}<p class="text-muted" role="status">Loading preview...</p>{/if}
	{#if error}<p class="alert alert-error" role="alert">{error}</p>{/if}
	{#if preview}<h2 class="subject">{preview.subject}</h2>
		<iframe
			class="body"
			title="Email template HTML preview"
			sandbox=""
			srcdoc={sanitizeHtml(preview.html, 'rich')}
		></iframe>
		{#if preview.text}<h3>Plain text alternative</h3>
			<pre>{preview.text}</pre>{/if}
	{/if}
	<a class="btn-secondary" href="/admin/email/templates">Back to List</a>
</div>

<style>
	.preview-page {
		max-width: 900px;
		margin: 0 auto;
		padding: 2rem;
	}
	.subject {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		margin-bottom: 1rem;
	}
	.body {
		width: 100%;
		min-height: 25rem;
		background: var(--bg-elevated, #161b22);
		padding: 1.5rem;
		border-radius: 8px;
		border: 1px solid rgba(230, 184, 0, 0.1);
	}
	.text-muted {
		color: var(--text-secondary, #8b949e);
	}
	.alert-error {
		background: rgba(248, 81, 73, 0.1);
		color: var(--error-emphasis, #f85149);
		padding: 0.75rem 1rem;
		border-radius: 6px;
		margin-bottom: 1rem;
	}
	.btn-secondary {
		background: var(--bg-hover, #252b33);
		color: var(--text-secondary, #8b949e);
		border: 1px solid var(--border-default, #30363d);
		padding: 0.5rem 1rem;
		border-radius: 6px;
		cursor: pointer;
		margin-top: 1rem;
	}
	.btn-secondary:hover {
		background: var(--bg-hover, #252b33);
	}
</style>
