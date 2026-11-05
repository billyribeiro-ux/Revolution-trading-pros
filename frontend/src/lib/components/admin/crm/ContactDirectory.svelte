<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { crmAPI } from '#lib/api/crm.js';
	import {
		contactStatuses,
		lifecycleStages,
		type ContactRecord,
		type ContactPage
	} from '#lib/crm/contact-types.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	let {
		dashboard = false,
		detailBase = '/admin/crm/contacts',
		title = 'Contacts'
	}: { dashboard?: boolean; detailBase?: string; title?: string } = $props();
	let result = $state<ContactPage<ContactRecord> | null>(null),
		loading = $state(true),
		error = $state(''),
		search = $state(''),
		status = $state(''),
		lifecycle = $state(''),
		selected = $state<string[]>([]);
	let stats = $state<{
			total: number;
			newThisMonth: number;
			activeDeals: number;
			dealValue: number;
			highScores: number;
			measured: number;
			unknown: number;
		} | null>(null),
		statsError = $state('');
	let disposed = false,
		generation = 0,
		statsGeneration = 0;
	const modules = [
		['Contact segments', '/admin/crm/segments'],
		['Deals', '/admin/crm/deals'],
		['Companies', '/admin/crm/companies'],
		['Campaigns', '/admin/crm/campaigns'],
		['Email sequences', '/admin/crm/sequences'],
		['Recurring campaigns', '/admin/crm/recurring-campaigns'],
		['Automations', '/admin/crm/automations'],
		['Smart links', '/admin/crm/smart-links'],
		['Templates', '/admin/crm/templates'],
		['Lists', '/admin/crm/lists'],
		['Tags', '/admin/crm/tags'],
		['Abandoned carts', '/admin/crm/abandoned-carts'],
		['Webhooks', '/admin/crm/webhooks'],
		['Import / export', '/admin/crm/import-export'],
		['Logs', '/admin/crm/logs'],
		['Settings', '/admin/crm/settings'],
		['Video tutorials', '/tutorials']
	];
	function count(value: unknown): number {
		if (typeof value !== 'number' || !Number.isSafeInteger(value) || value < 0)
			throw new Error('Invalid CRM statistics');
		return value;
	}
	async function loadStats() {
		if (!dashboard) return;
		const current = ++statsGeneration;
		statsError = '';
		try {
			const response = await fetch('/api/admin/crm/stats', {
				cache: 'no-store',
				credentials: 'same-origin',
				redirect: 'error',
				signal: AbortSignal.timeout(30000)
			});
			if (!response.ok) throw new Error(`CRM statistics request failed (${response.status})`);
			const value: unknown = await response.json();
			if (!value || typeof value !== 'object' || Array.isArray(value))
				throw new Error('Invalid CRM statistics');
			const row =
				'data' in value &&
				value.data &&
				typeof value.data === 'object' &&
				!Array.isArray(value.data)
					? value.data
					: value;
			if (
				!('total_contacts' in row) ||
				!('new_this_month' in row) ||
				!('active_deals' in row) ||
				!('deal_value' in row) ||
				!('high_score_contacts' in row) ||
				!('measured_score_contacts' in row) ||
				!('unknown_score_contacts' in row)
			)
				throw new Error('Incomplete CRM statistics');
			const next = {
				total: count(row.total_contacts),
				newThisMonth: count(row.new_this_month),
				activeDeals: count(row.active_deals),
				dealValue: row.deal_value,
				highScores: count(row.high_score_contacts),
				measured: count(row.measured_score_contacts),
				unknown: count(row.unknown_score_contacts)
			};
			if (
				typeof next.dealValue !== 'number' ||
				!Number.isFinite(next.dealValue) ||
				next.measured + next.unknown !== next.total ||
				next.highScores > next.measured
			)
				throw new Error('Inconsistent CRM statistics');
			if (!disposed && current === statsGeneration) stats = { ...next, dealValue: next.dealValue };
		} catch (cause) {
			if (!disposed && current === statsGeneration)
				statsError = apiErrorMessage(cause, 'Unable to load CRM statistics');
		}
	}
	async function load(page = 1, syncUrl = true) {
		const applied = { search, status, lifecycle };
		const current = ++generation;
		loading = true;
		error = '';
		try {
			const response = await crmAPI.getContacts({
				page,
				per_page: 25,
				search: applied.search || undefined,
				status: applied.status || undefined,
				lifecycle_stage: applied.lifecycle || undefined
			});
			if (disposed || current !== generation) return;
			result = response;
			selected = [];
			if (syncUrl) {
				const url = new URL(location.href);
				for (const [key, value] of [
					['search', applied.search],
					['status', applied.status],
					['lifecycle_stage', applied.lifecycle],
					['page', String(page)]
				]) {
					if (value && !(key === 'page' && value === '1')) url.searchParams.set(key, value);
					else url.searchParams.delete(key);
				}
				await goto(`${url.pathname}${url.search}`, { replace: true, reset: false });
			}
		} catch (cause) {
			if (!disposed && current === generation)
				error = apiErrorMessage(cause, 'Unable to load contacts');
		} finally {
			if (!disposed && current === generation) loading = false;
		}
	}
	function toggle(id: string, checked: boolean) {
		selected = checked ? [...new Set([...selected, id])] : selected.filter((value) => value !== id);
	}
	function date(value: string | null): string {
		return value ? new Date(value).toLocaleString() : 'Unknown';
	}
	onMount(() => {
		const url = new URL(location.href);
		search = url.searchParams.get('search') ?? '';
		status = url.searchParams.get('status') ?? '';
		lifecycle = url.searchParams.get('lifecycle_stage') ?? url.searchParams.get('stage') ?? '';
		const page = Number(url.searchParams.get('page') ?? '1');
		if (!Number.isSafeInteger(page) || page < 1) {
			error = 'Invalid contact page number';
			loading = false;
		} else void load(page, false);
		void loadStats();
		return () => {
			disposed = true;
		};
	});
</script>

<section class="directory">
	<header>
		<div>
			<h1>{title}</h1>
			<p>Saved contact records, current score evidence and recorded relationship facts.</p>
		</div>
		<div class="actions">
			<button
				disabled={loading}
				onclick={() => {
					void load(result?.meta.current_page ?? 1);
					void loadStats();
				}}>Refresh</button
			><a class="button" href="/admin/crm/contacts/new">Add Contact</a>
		</div>
	</header>
	{#if dashboard}<nav class="modules" aria-label="CRM modules">
			{#each modules as [label, href] (href)}<a {href}>{label}</a>{/each}<span
				>Documentation: unavailable</span
			>
		</nav>
		{#if statsError}<p role="alert">{statsError}</p>
			<button onclick={() => void loadStats()}>Retry statistics</button>{:else if stats}<div
				class="stats"
			>
				<div><strong>{stats.total}</strong><span>All contacts</span></div>
				<div><strong>{stats.newThisMonth}</strong><span>Created this month</span></div>
				<div><strong>{stats.highScores}</strong><span>Current measured scores ≥ 75</span></div>
				<div>
					<strong>{stats.measured} / {stats.unknown}</strong><span>Measured / unknown scores</span>
				</div>
				<div><strong>{stats.activeDeals}</strong><span>Open deals</span></div>
				<div>
					<strong
						>{new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(
							stats.dealValue
						)}</strong
					><span>Open deal value (USD only)</span>
				</div>
			</div>{/if}{/if}
	<form
		class="filters"
		onsubmit={(event) => {
			event.preventDefault();
			void load();
		}}
	>
		<label
			>Search contacts<input name="search" placeholder="Name or email" bind:value={search} /></label
		><label
			>Contact status<select bind:value={status}
				><option value="">All statuses</option>{#each contactStatuses as value (value)}<option
						{value}>{value}</option
					>{/each}</select
			></label
		><label
			>Lifecycle stage<select bind:value={lifecycle}
				><option value="">All lifecycle stages</option
				>{#each lifecycleStages as value (value)}<option {value}>{value}</option>{/each}</select
			></label
		><button disabled={loading}>Apply filters</button>
	</form>
	{#if loading}<p role="status">Loading contacts…</p>{:else if error}<p role="alert">{error}</p>
		<button onclick={() => void load(result?.meta.current_page ?? 1)}>Retry contacts</button
		>{:else if result}<p>
			{result.meta.total} contacts match the current filters. {selected.length} selected on this page.
			{#if selected.length}<button
					onclick={() => {
						selected = [];
					}}>Clear selection</button
				>{/if}
		</p>
		<p>Health, risk and lifetime value are not measured by this directory.</p>
		<div class="table">
			<table>
				<thead
					><tr
						><th
							><input
								type="checkbox"
								aria-label="Select all contacts on this page"
								checked={result.data.length > 0 && selected.length === result.data.length}
								onchange={(event) => {
									selected = event.currentTarget.checked
										? (result?.data.map((row) => row.id) ?? [])
										: [];
								}}
							/></th
						><th>Contact</th><th>Company / job title</th><th>Status / lifecycle</th><th
							>Lead score</th
						><th>Tags / lists</th><th>Last recorded activity</th><th>Actions</th></tr
					></thead
				><tbody
					>{#each result.data as contact (contact.id)}<tr
							><td
								><input
									type="checkbox"
									aria-label={`Select ${contact.email}`}
									checked={selected.includes(contact.id)}
									onchange={(event) => toggle(contact.id, event.currentTarget.checked)}
								/></td
							><td
								><a href={`${detailBase}/${contact.id}`}>{contact.full_name}</a><small
									>{contact.email}</small
								>{#if contact.phone}<a href={`tel:${contact.phone}`}>{contact.phone}</a>{/if}</td
							><td>{contact.company || 'Unknown'}<small>{contact.job_title || 'Unknown'}</small></td
							><td
								>{contact.status ?? 'Unknown'}<small
									>{contact.lifecycle_stage ?? 'Unknown lifecycle'}</small
								></td
							><td
								>{contact.lead_score === null
									? 'Not measured'
									: `${contact.lead_score} / ${contact.scoring.max_score}`}<small
									>{contact.scoring.status.replaceAll('_', ' ')}</small
								></td
							><td
								>{contact.tags === null
									? 'Tags unknown'
									: contact.tags.length
										? contact.tags.join(', ')
										: 'No tags'}<small
									><a href={`${detailBase}/${contact.id}`}>View list memberships</a></small
								></td
							><td>{date(contact.last_activity_at)}</td><td
								><a href={`${detailBase}/${contact.id}`}>View</a>
								<a href={`/admin/crm/contacts/${contact.id}/edit`}>Edit</a>
								<details>
									<summary>More</summary><a href={`${detailBase}/${contact.id}`}
										>Notes, tags, lists and email</a
									>
								</details></td
							></tr
						>{/each}</tbody
				>
			</table>
		</div>
		{#if result.data.length === 0}<p>No contacts found on this page.</p>{/if}
		<nav class="pagination" aria-label="Contact pages">
			<button
				disabled={result.meta.current_page <= 1}
				onclick={() => void load((result?.meta.current_page ?? 1) - 1)}>Previous</button
			><span
				>Page {result.meta.current_page} of {result.meta.total_pages}; {result.meta.total} contacts</span
			><button
				disabled={result.meta.current_page >= result.meta.total_pages}
				onclick={() => void load((result?.meta.current_page ?? 1) + 1)}>Next</button
			>
		</nav>{/if}
</section>

<style>
	.directory {
		max-width: 100rem;
		padding: 1.5rem;
		margin: auto;
		color: #e2e8f0;
	}
	header,
	.actions,
	.filters,
	.modules,
	.pagination {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
		align-items: center;
		justify-content: space-between;
	}
	header h1 {
		margin-bottom: 0.5rem;
	}
	p,
	small,
	.stats span,
	.modules span {
		color: #94a3b8;
	}
	.modules {
		justify-content: flex-start;
		padding: 1rem 0;
	}
	.stats {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(10rem, 1fr));
		gap: 1rem;
		margin: 1rem 0;
	}
	.stats div {
		padding: 1rem;
		border: 1px solid #334155;
		border-radius: 0.7rem;
		display: grid;
		gap: 0.5rem;
	}
	.stats strong {
		font-size: 1.4rem;
	}
	.filters {
		align-items: end;
		justify-content: flex-start;
		padding: 1rem 0;
	}
	label {
		display: grid;
		gap: 0.5rem;
	}
	input,
	select,
	button,
	.button {
		padding: 0.65rem;
		background: #0f172a;
		border: 1px solid #475569;
		border-radius: 0.4rem;
		color: inherit;
	}
	a {
		color: #7dd3fc;
	}
	.button {
		text-decoration: none;
	}
	.table {
		overflow-x: auto;
	}
	table {
		width: 100%;
		border-collapse: collapse;
	}
	th,
	td {
		text-align: left;
		padding: 0.9rem;
		border-bottom: 1px solid #334155;
	}
	small {
		display: block;
		margin: 0.3rem 0;
	}
	.pagination {
		margin-top: 1rem;
	}
	[role='alert'] {
		color: #fca5a5;
	}
</style>
