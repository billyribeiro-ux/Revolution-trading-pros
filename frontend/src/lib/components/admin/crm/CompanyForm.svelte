<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { crmAPI } from '#lib/api/crm.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	import type { CompanyIndustry, CompanySize } from '#lib/crm/types.js';

	let { id }: { id?: string } = $props();
	let loading = $state(true);
	let saving = $state(false);
	let error = $state('');
	let name = $state('');
	let website = $state('');
	let industry = $state<CompanyIndustry | ''>('');
	let size = $state<CompanySize | ''>('');
	let logoUrl = $state('');
	let description = $state('');
	const industries: CompanyIndustry[] = [
		'technology',
		'finance',
		'healthcare',
		'education',
		'retail',
		'manufacturing',
		'real_estate',
		'consulting',
		'marketing',
		'media',
		'nonprofit',
		'government',
		'other'
	];
	const sizes: CompanySize[] = [
		'1-10',
		'11-50',
		'51-200',
		'201-500',
		'501-1000',
		'1001-5000',
		'5001+'
	];

	async function load() {
		if (!id) {
			loading = false;
			return;
		}
		loading = true;
		error = '';
		try {
			const company = await crmAPI.getCompany(id);
			name = company.name;
			website = company.website ?? '';
			industry = company.industry ?? '';
			size = company.size ?? '';
			logoUrl = company.logo_url ?? '';
			description = company.description ?? '';
		} catch (cause) {
			error = apiErrorMessage(cause, 'Unable to load company');
		} finally {
			loading = false;
		}
	}

	async function save(event: SubmitEvent) {
		event.preventDefault();
		if (saving) return;
		saving = true;
		error = '';
		try {
			const fields = {
				name: name.trim(),
				website,
				industry: industry || undefined,
				size: size || undefined,
				logo_url: logoUrl,
				description
			};
			if (id) await crmAPI.updateCompany(id, fields);
			else await crmAPI.createCompany(fields);
			await goto('/admin/crm/companies', { invalidateAll: true });
		} catch (cause) {
			error = apiErrorMessage(cause, 'Unable to save company');
		} finally {
			saving = false;
		}
	}

	onMount(() => {
		void load();
	});
</script>

<section class="company-editor">
	<a href="/admin/crm/companies">← Companies</a>
	<h1>{id ? 'Edit Company' : 'New Company'}</h1>
	{#if error}
		<div role="alert" class="error">{error}</div>
		{#if id}<button type="button" onclick={() => void load()}>Retry loading</button>{/if}
	{/if}
	{#if loading}
		<p role="status">Loading company…</p>
	{:else}
		<form onsubmit={(event) => void save(event)}>
			<label for="company-name">Company name</label>
			<input
				id="company-name"
				autocomplete="organization"
				bind:value={name}
				required
				maxlength={255}
			/>
			<label for="company-website">Website</label>
			<input
				id="company-website"
				type="url"
				autocomplete="url"
				bind:value={website}
				maxlength={500}
			/>
			<label for="company-industry">Industry</label>
			<select id="company-industry" bind:value={industry}>
				<option value="">Not specified</option>
				{#each industries as option (option)}<option value={option}
						>{option.replaceAll('_', ' ')}</option
					>{/each}
			</select>
			<label for="company-size">Company size</label>
			<select id="company-size" bind:value={size}>
				<option value="">Not specified</option>
				{#each sizes as option (option)}<option value={option}>{option} employees</option>{/each}
			</select>
			<label for="company-logo">Logo URL</label>
			<input id="company-logo" type="url" autocomplete="off" bind:value={logoUrl} maxlength={500} />
			<label for="company-description">Description</label>
			<textarea id="company-description" bind:value={description} maxlength={50000} rows={6}
			></textarea>
			<div class="actions">
				<button type="submit" disabled={saving}>{saving ? 'Saving…' : 'Save Company'}</button>
				<a href="/admin/crm/companies">Cancel</a>
			</div>
		</form>
	{/if}
</section>

<style>
	.company-editor {
		max-width: 48rem;
		margin: 0 auto;
		padding: 2rem;
		color: var(--text-primary);
	}
	h1 {
		font-size: 1.75rem;
		margin: 1rem 0 1.5rem;
	}
	form {
		display: grid;
		gap: 0.75rem;
	}
	label {
		font-weight: 600;
		margin-top: 0.5rem;
	}
	input,
	select,
	textarea {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid var(--border-color, #52525b);
		border-radius: 0.5rem;
		background: var(--bg-elevated, #18181b);
		color: inherit;
	}
	.actions {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-top: 1rem;
	}
	button {
		padding: 0.75rem 1rem;
		border-radius: 0.5rem;
		background: var(--accent-primary, #6366f1);
		color: white;
		font-weight: 600;
	}
	.error {
		margin: 1rem 0;
		padding: 1rem;
		border: 1px solid #ef4444;
		border-radius: 0.5rem;
	}
</style>
