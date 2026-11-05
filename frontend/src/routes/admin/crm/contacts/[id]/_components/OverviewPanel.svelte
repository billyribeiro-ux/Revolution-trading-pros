<script lang="ts">
	import type {
		ContactRecord,
		ContactTag,
		ContactListMembership,
		ContactPage
	} from '#lib/crm/contact-types.js';
	import { formatDateTime } from './helpers';
	let {
		contact,
		tags,
		lists,
		tagsError,
		listsError,
		tagsLoading,
		listsLoading,
		busy,
		canReadTags, canReadLists, canAddTag, canRemoveTag, canAddList, canRemoveList,
		onAddTag,
		onRemoveTag,
		onAddList,
		onRemoveFromList,
		onRefreshTags,
		onRefreshLists,
		onListsPage
	}: {
		contact: ContactRecord;
		tags: ContactTag[];
		lists: ContactPage<ContactListMembership> | null;
		tagsError: string;
		listsError: string;
		tagsLoading: boolean;
		listsLoading: boolean;
		busy: boolean;
		canReadTags: boolean; canReadLists: boolean; canAddTag: boolean; canRemoveTag: boolean; canAddList: boolean; canRemoveList: boolean;
		onAddTag: () => void;
		onRemoveTag: (id: string) => void;
		onAddList: () => void;
		onRemoveFromList: (id: string) => void;
		onRefreshTags: () => void;
		onRefreshLists: () => void;
		onListsPage: (page: number) => void;
	} = $props();
</script>

<div class="overview">
	<section>
		<h2>Contact information</h2>
		<dl>
			<dt>Email</dt>
			<dd><a href={`mailto:${contact.email}`}>{contact.email}</a></dd>
			<dt>Phone</dt>
			<dd>
				{#if contact.phone}<a href={`tel:${contact.phone}`}>{contact.phone}</a>{:else}Unknown{/if}
			</dd>
			<dt>Company</dt>
			<dd>{contact.company || 'Unknown'}</dd>
			<dt>Job title</dt>
			<dd>{contact.job_title || 'Unknown'}</dd>
			<dt>Source</dt>
			<dd>{contact.source || 'Unknown'}</dd>
			<dt>Created</dt>
			<dd>{formatDateTime(contact.created_at)}</dd>
			<dt>Updated</dt>
			<dd>{formatDateTime(contact.updated_at)}</dd>
			<dt>Last recorded activity</dt>
			<dd>{formatDateTime(contact.last_activity_at)}</dd>
			<dt>Owner record</dt>
			<dd>{contact.owner_id ?? 'Unassigned'}</dd>
			<dt>Last contacted</dt>
			<dd>{formatDateTime(contact.last_contacted_at)}</dd>
		</dl>
	</section>
	<section>
		<header>
			<h2>Tags</h2>
			<button onclick={onAddTag} disabled={busy || !canAddTag}>Add tag</button>
		</header>
		{#if !canReadTags}<p>Tag assignments are not permitted by your current CRM access.</p>{:else if tagsLoading}<p role="status">Loading tags…</p>{:else if tagsError}<p role="alert">
				{tagsError}
			</p>
			<button onclick={onRefreshTags}>Retry tags</button>{:else if tags.length === 0}<p>
				No tags assigned.
			</p>{:else}<ul>
				{#each tags as tag (`${tag.id ?? 'legacy'}:${tag.name}`)}<li>
						<span style:color={tag.color ?? undefined}>{tag.name}</span>{#if tag.id}<button
								aria-label={`Remove tag ${tag.name}`}
								onclick={() => tag.id && onRemoveTag(tag.id)}
								disabled={busy || !canRemoveTag}>Remove</button
							>{:else}<span>Unregistered label; register a matching tag to manage it.</span>{/if}
					</li>{/each}
			</ul>{/if}
	</section>
	<section>
		<header>
			<h2>Lists</h2>
			<button onclick={onAddList} disabled={busy || !canAddList}>Add to list</button>
		</header>
		{#if !canReadLists}<p>List memberships are not permitted by your current CRM access.</p>{:else if listsLoading}<p role="status">Loading lists…</p>{:else if listsError}<p role="alert">
				{listsError}
			</p>
			<button onclick={onRefreshLists}>Retry lists</button>{:else if lists}<p>
				{lists.meta.total} recorded list memberships.
			</p>
			{#if lists.data.length === 0}<p>No list memberships on this page.</p>{:else}<ul>
					{#each lists.data as list (list.id)}<li>
							<span>{list.name}<small>Added {formatDateTime(list.subscribed_at)}</small></span
							><button
								aria-label={`Remove from list ${list.name}`}
								onclick={() => onRemoveFromList(list.id)}
								disabled={busy || !canRemoveList}>Remove</button
							>
						</li>{/each}
				</ul>{/if}
			<nav aria-label="List membership pages">
				<button
					disabled={lists.meta.current_page <= 1 || busy}
					onclick={() => onListsPage(lists.meta.current_page - 1)}>Previous</button
				><span>Page {lists.meta.current_page} of {lists.meta.total_pages}</span><button
					disabled={lists.meta.current_page >= lists.meta.total_pages || busy}
					onclick={() => onListsPage(lists.meta.current_page + 1)}>Next</button
				>
			</nav>{/if}
	</section>
	<section>
		<h2>Recorded linked facts</h2>
		{#if contact.facts}<p>
				Deals: {contact.facts.deals.total} total, {contact.facts.deals.open} open, {contact.facts
					.deals.won} won. Recorded deal activities: {contact.facts.deals.activities}; deal notes: {contact
					.facts.deals.notes}.
			</p>
			<p>CRM outbox provider acceptances: {contact.facts.email.accepted}.</p>
			{#if contact.facts.orders}<p>
					Linked member orders: {contact.facts.orders.total} total, {contact.facts.orders.completed} completed,
					{contact.facts.orders.refunded} refunded.
				</p>{:else}<p>Member orders: no recorded contact-to-member link.</p>{/if}{:else}<p>
				Linked facts unavailable.
			</p>{/if}
		<p>
			Behavior sessions, intent, friction, subscription MRR and lifetime value are not measured by
			this contact profile.
		</p>
	</section>
	<section>
		<h2>Custom fields</h2>
		{#if contact.custom_fields}<pre>{JSON.stringify(contact.custom_fields, null, 2)}</pre>{:else}<p>
				No custom fields recorded.
			</p>{/if}
	</section>
</div>

<style>
	.overview {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(min(100%, 22rem), 1fr));
		gap: 1rem;
	}
	section {
		padding: 1.2rem;
		border: 1px solid #334155;
		border-radius: 0.8rem;
		background: #0f172a;
	}
	h2 {
		font-size: 1rem;
	}
	header,
	nav,
	li {
		display: flex;
		gap: 0.8rem;
		justify-content: space-between;
		align-items: center;
	}
	ul {
		padding: 0;
		list-style: none;
		display: grid;
		gap: 0.8rem;
	}
	dd {
		margin: 0.2rem 0 1rem;
	}
	dt,
	p,
	small {
		color: #94a3b8;
	}
	small {
		display: block;
	}
	a {
		color: #7dd3fc;
	}
	button {
		padding: 0.5rem;
		border-radius: 0.4rem;
		color: #e2e8f0;
		border: 1px solid #475569;
		background: #1e293b;
	}
	[role='alert'] {
		color: #fca5a5;
	}
	pre {
		white-space: pre-wrap;
		overflow-wrap: anywhere;
	}
</style>
