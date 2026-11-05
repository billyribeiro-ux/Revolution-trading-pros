<script lang="ts">
	import { useCrmAccess } from '#lib/crm/access.svelte.js';
	const access = useCrmAccess();
	import { onMount } from 'svelte';
	import { crmAPI } from '#lib/api/crm.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	import type {
		SubscriberSettings,
		SubscriberFilter,
		ContactList,
		ContactTag,
		ContactSegment,
		CampaignAudiencePreview
	} from '#lib/crm/types.js';
	let {
		id,
		settings,
		editable,
		onSaved
	}: {
		id: string;
		settings?: SubscriberSettings;
		editable: boolean;
		onSaved: () => Promise<void>;
	} = $props();
	type Group = SubscriberFilter & { key: number };
	let nextKey = 0;
	let includes = $state<Group[]>([]),
		excludes = $state<Group[]>([]);
	let newsletterMode = $state(false),
		newsletterSegment = $state('confirmed');
	let segmentId = $state('');
	let advanced = $state<SubscriberSettings['advanced_filters']>([]);
	let lists = $state<ContactList[]>([]),
		tags = $state<ContactTag[]>([]),
		segments = $state<ContactSegment[]>([]);
	let listSearch = $state(''),
		tagSearch = $state(''),
		segmentSearch = $state('');
	let searching = $state<'lists' | 'tags' | 'segments' | null>(null),
		searchError = $state('');
	let searchedKinds = $state({ lists: false, tags: false, segments: false });
	let preview = $state<CampaignAudiencePreview | null>(null),
		loading = $state(true),
		saving = $state(false),
		error = $state('');
	let currentPage = $state(1),
		dirty = $state(false);
	async function loadPreview() {
		if (!access.can('GET', '/campaigns/{id}/recipients')) {
			preview = null;
			loading = false;
			error = 'Audience preview requires its campaign and contact read permissions.';
			return;
		}
		loading = true;
		error = '';
		try {
			preview = await crmAPI.getCampaignRecipients(id, { page: currentPage, per_page: 10 });
		} catch (cause) {
			error = apiErrorMessage(cause, 'Unable to load campaign audience');
		} finally {
			loading = false;
		}
	}
	async function search(kind: 'lists' | 'tags' | 'segments') {
		if (
			!access.can(
				'GET',
				kind === 'lists' ? '/lists' : kind === 'tags' ? '/contact-tags' : '/segments'
			)
		) {
			searchError = 'Your role cannot search these audience references.';
			return;
		}
		if (searching) return;
		searching = kind;
		searchError = '';
		try {
			if (kind === 'lists')
				lists = (await crmAPI.getContactLists({ search: listSearch, per_page: 25 })).data;
			else if (kind === 'tags')
				tags = (await crmAPI.getContactTags({ search: tagSearch, per_page: 25 })).data;
			else
				segments = (
					await crmAPI.getContactSegments({ search: segmentSearch, is_active: true, per_page: 25 })
				).data;
			searchedKinds[kind] = true;
		} catch (cause) {
			searchError = apiErrorMessage(cause, 'Unable to search audience records');
		} finally {
			searching = null;
		}
	}
	function group(filter: SubscriberFilter): Group {
		return {
			...filter,
			list: String(filter.list ?? ''),
			tag: String(filter.tag ?? ''),
			key: nextKey++
		};
	}
	function add(exclude: boolean) {
		const row = group({ list: '', tag: '' });
		if (exclude) excludes = [...excludes, row];
		else includes = [...includes, row];
		dirty = true;
	}
	function remove(key: number, exclude: boolean) {
		if (exclude) excludes = excludes.filter((row) => row.key !== key);
		else includes = includes.filter((row) => row.key !== key);
		dirty = true;
	}
	async function save(event: SubmitEvent) {
		event.preventDefault();
		if (saving) return;
		if (!access.can('PUT', '/campaigns/{id}')) {
			error = 'Your role cannot edit this audience.';
			return;
		}
		saving = true;
		error = '';
		try {
			const strip = ({ list, tag }: Group): SubscriberFilter => ({ list, tag });
			await crmAPI.updateCampaign(id, {
				subscriber_settings: newsletterMode
					? { newsletter_segment: newsletterSegment.trim(), eligibility: 'confirmed_only' }
					: {
							subscribers: includes.map(strip),
							excluded_subscribers: excludes.map(strip),
							dynamic_segment: segmentId,
							sending_filter: 'confirmed',
							advanced_filters: advanced ?? []
						}
			});
			dirty = false;
			currentPage = 1;
			await onSaved();
		} catch (cause) {
			error = apiErrorMessage(cause, 'Unable to save campaign audience');
		} finally {
			saving = false;
		}
	}
	function page(next: number) {
		currentPage = next;
		void loadPreview();
	}
	onMount(() => {
		includes = (settings?.subscribers ?? []).map(group);
		excludes = (settings?.excluded_subscribers ?? []).map(group);
		segmentId = settings?.dynamic_segment ?? '';
		newsletterMode = settings?.newsletter_segment !== undefined;
		newsletterSegment = settings?.newsletter_segment ?? 'confirmed';
		advanced = settings?.advanced_filters ?? [];
		void loadPreview();
	});
</script>

<section class="campaign-audience">
	<h2>Recipients</h2>
	<p>
		Only confirmed newsletter subscribers are eligible. Contacts without confirmation and
		unsubscribed contacts are excluded. The preview uses the saved audience.
	</p>
	{#if error}<p role="alert">{error}</p>
		<button type="button" onclick={() => void loadPreview()}>Retry preview</button>{/if}
	{#if editable && access.can('PUT', '/campaigns/{id}')}
		<form onsubmit={(event) => void save(event)}>
			<label for="audience-source">Recipient source</label>
			<select
				id="audience-source"
				bind:value={newsletterMode}
				onchange={() => {
					dirty = true;
				}}
			>
				<option value={false}>CRM contacts</option><option value={true}
					>Newsletter subscribers</option
				>
			</select>
			{#if newsletterMode}
				<label for="newsletter-segment">Newsletter segment (all, confirmed, or tag)</label>
				<input
					id="newsletter-segment"
					bind:value={newsletterSegment}
					required
					maxlength={255}
					oninput={() => {
						dirty = true;
					}}
				/>
				<p>
					Only subscribers with current confirmation are eligible, including when selecting all.
				</p>
			{:else}
				<fieldset>
					<legend>Find audience records</legend>
					<label for="audience-list-search">Search lists</label>
					<div class="search">
						<input id="audience-list-search" bind:value={listSearch} /><button
							type="button"
							disabled={searching !== null}
							onclick={() => void search('lists')}>Find lists</button
						>
					</div>
					<label for="audience-tag-search">Search tags</label>
					<div class="search">
						<input id="audience-tag-search" bind:value={tagSearch} /><button
							type="button"
							disabled={searching !== null}
							onclick={() => void search('tags')}>Find tags</button
						>
					</div>
					<label for="audience-segment-search">Search dynamic segments</label>
					<div class="search">
						<input id="audience-segment-search" bind:value={segmentSearch} /><button
							type="button"
							disabled={searching !== null}
							onclick={() => void search('segments')}>Find segments</button
						>
					</div>
					{#if searchedKinds.lists && !lists.length}<p>
							No matching lists.
						</p>{/if}{#if searchedKinds.tags && !tags.length}<p>
							No matching tags.
						</p>{/if}{#if searchedKinds.segments && !segments.length}<p>
							No matching active segments.
						</p>{/if}
					<small
						>Each search returns up to 25 matches. Refine the name to find another record.</small
					>{#if searching}<p role="status">Searching {searching}…</p>{/if}{#if searchError}<p
							role="alert"
						>
							{searchError}
						</p>{/if}
				</fieldset>
				<h3>Include matching contacts</h3>
				<p>
					{includes.length
						? 'Include contacts matching any group. A group with both a list and tag requires both.'
						: 'All confirmed contacts are included before exclusions and segment rules.'}
				</p>
				{#each includes as row, index (row.key)}
					<fieldset>
						<legend>Include group {index + 1}</legend>
						<label for={`include-list-${row.key}`}>List</label><select
							id={`include-list-${row.key}`}
							bind:value={row.list}
							onchange={() => {
								dirty = true;
							}}
							required={!row.tag}
							><option value="">Any list</option
							>{#if row.list && !lists.some((item) => String(item.id) === row.list)}<option
									value={row.list}>List #{row.list} (selected)</option
								>{/if}{#each lists as list (list.id)}<option value={String(list.id)}
									>{list.title}</option
								>{/each}</select
						>
						<label for={`include-tag-${row.key}`}>Tag</label><select
							id={`include-tag-${row.key}`}
							bind:value={row.tag}
							onchange={() => {
								dirty = true;
							}}
							required={!row.list}
							><option value="">Any tag</option
							>{#if row.tag && !tags.some((item) => String(item.id) === row.tag)}<option
									value={row.tag}>Tag #{row.tag} (selected)</option
								>{/if}{#each tags as tag (tag.id)}<option value={String(tag.id)}>{tag.title}</option
								>{/each}</select
						>
						<button type="button" onclick={() => remove(row.key, false)}
							>Remove include group {index + 1}</button
						>
					</fieldset>
				{/each}<button type="button" onclick={() => add(false)}>Add include group</button>
				<h3>Exclude matching contacts</h3>
				<p>Any matching exclusion takes priority over inclusion.</p>
				{#each excludes as row, index (row.key)}
					<fieldset>
						<legend>Exclude group {index + 1}</legend>
						<label for={`exclude-list-${row.key}`}>List</label><select
							id={`exclude-list-${row.key}`}
							bind:value={row.list}
							onchange={() => {
								dirty = true;
							}}
							required={!row.tag}
							><option value="">Any list</option
							>{#if row.list && !lists.some((item) => String(item.id) === row.list)}<option
									value={row.list}>List #{row.list} (selected)</option
								>{/if}{#each lists as list (list.id)}<option value={String(list.id)}
									>{list.title}</option
								>{/each}</select
						>
						<label for={`exclude-tag-${row.key}`}>Tag</label><select
							id={`exclude-tag-${row.key}`}
							bind:value={row.tag}
							onchange={() => {
								dirty = true;
							}}
							required={!row.list}
							><option value="">Any tag</option
							>{#if row.tag && !tags.some((item) => String(item.id) === row.tag)}<option
									value={row.tag}>Tag #{row.tag} (selected)</option
								>{/if}{#each tags as tag (tag.id)}<option value={String(tag.id)}>{tag.title}</option
								>{/each}</select
						>
						<button type="button" onclick={() => remove(row.key, true)}
							>Remove exclude group {index + 1}</button
						>
					</fieldset>
				{/each}<button type="button" onclick={() => add(true)}>Add exclude group</button>
				<label for="audience-segment">Also match a dynamic segment</label><select
					id="audience-segment"
					bind:value={segmentId}
					onchange={() => {
						dirty = true;
					}}
					><option value="">No additional segment</option
					>{#if segmentId && !segments.some((item) => String(item.id) === segmentId)}<option
							value={segmentId}>Segment #{segmentId} (selected)</option
						>{/if}{#each segments as segment (segment.id)}<option value={String(segment.id)}
							>{segment.title}</option
						>{/each}</select
				>
				{#if advanced?.length}<details>
						<summary>Additional saved rules ({advanced.length})</summary>
						<ul>
							{#each advanced as rule, index (index)}<li>
									{rule.field}
									{rule.operator}
									{JSON.stringify(rule.value)}
								</li>{/each}
						</ul>
						<button
							type="button"
							onclick={() => {
								advanced = [];
								dirty = true;
							}}>Clear additional rules</button
						>
					</details>{/if}
				<p>
					<a href="/admin/crm/segments/new">Create a dynamic segment</a> to combine additional contact
					rules.
				</p>
				{#if dirty}<p role="status">Save the audience to update the recipient preview.</p>{/if}
			{/if}
			<button type="submit" disabled={saving}>{saving ? 'Saving…' : 'Save Audience'}</button>
		</form>{/if}
	{#if loading}<p role="status">
			Loading recipient preview…
		</p>{:else if preview && access.can('GET', '/campaigns/{id}/recipients')}
		<p>
			{preview.audience.eligible_recipients} eligible recipients from {preview.audience
				.matching_contacts} matching {preview.audience.source === 'newsletter'
				? 'newsletter subscribers'
				: 'contacts'}. {preview.audience.excluded_without_confirmation} excluded without an active confirmation.
		</p>
		{#if !preview.data.length}<p>No eligible recipients match the saved audience.</p>{:else}<table>
				<thead><tr><th>Name</th><th>Email</th></tr></thead><tbody
					>{#each preview.data as contact (contact.id)}<tr
							><td>{contact.full_name}</td><td>{contact.email}</td></tr
						>{/each}</tbody
				>
			</table>{/if}
		<nav aria-label="Campaign recipient pages">
			<button
				type="button"
				disabled={currentPage <= 1 || loading}
				onclick={() => page(currentPage - 1)}>Previous</button
			><span>Page {currentPage} of {preview.meta.total_pages}</span><button
				type="button"
				disabled={currentPage >= preview.meta.total_pages || loading}
				onclick={() => page(currentPage + 1)}>Next</button
			>
		</nav>
	{/if}
</section>

<style>
	.campaign-audience {
		margin: 2rem 0;
		padding: 1rem;
		border: 1px solid var(--border-color, #475569);
		border-radius: 0.5rem;
	}
	h2 {
		font-size: 1.25rem;
	}
	h3 {
		font-size: 1.05rem;
		margin-top: 1rem;
	}
	form {
		display: grid;
		gap: 0.75rem;
	}
	fieldset,
	details {
		padding: 1rem;
		border: 1px solid var(--border-color, #475569);
		border-radius: 0.5rem;
	}
	label {
		display: block;
		font-weight: 600;
		margin-top: 0.5rem;
	}
	input,
	select,
	button {
		padding: 0.65rem;
		border: 1px solid var(--border-color, #475569);
		border-radius: 0.5rem;
		background: var(--bg-secondary, #18181b);
		color: inherit;
	}
	input,
	select {
		width: 100%;
	}
	button {
		cursor: pointer;
	}
	.search,
	nav {
		display: flex;
		align-items: center;
		gap: 1rem;
	}
	.search input {
		flex: 1;
		min-width: 0;
	}
	.search button {
		flex-shrink: 0;
	}
	small {
		color: var(--text-secondary);
	}
	[role='alert'] {
		color: #fca5a5;
	}
	table {
		width: 100%;
		text-align: left;
		margin: 1rem 0;
	}
	td,
	th {
		padding: 0.5rem;
		border-bottom: 1px solid var(--border-color, #475569);
	}
	nav {
		margin-top: 1rem;
	}
	a {
		color: var(--accent-color, #7dd3fc);
	}
</style>
