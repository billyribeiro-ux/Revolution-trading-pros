<!--
	ComingSoonStub — shared placeholder for admin CRM "new"/"edit" pages whose
	backend is deferred. Consolidates 21 near-identical stub pages.
	Audit: 06-crm.md P0 #1 (orphan-link epidemic).
	Backend wiring tracked in /docs/audits/admin-2026-04-26/06-crm-DEFERRED.md.
-->
<script lang="ts">
	import IconArrowLeft from '@tabler/icons-svelte-runes/icons/arrow-left';
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';

	interface Props {
		/** Heading shown in <h1>; also the default browser-tab title. */
		entity: string;
		/** href for the back link and the "Return to…" button. */
		backHref: string;
		/** Noun used in "Back to {backLabel}" and "Return to {backLabel}". */
		backLabel: string;
		/** Sentence under the heading on "new" pages (ignored when `id` is set). */
		description?: string;
		/** Record id for [id]/edit pages; when set, shows "{idLabel}: {id}". */
		id?: string | undefined;
		/** Label for the id line on edit pages. */
		idLabel?: string;
		/** Override for the <title> text (defaults to `entity`). */
		title?: string;
		/** Lead sentence of the card body. */
		bodyLead?: string;
	}

	let {
		entity,
		backHref,
		backLabel,
		description = '',
		id = undefined,
		idLabel = 'Record ID',
		title = undefined,
		bodyLead = 'This form is not yet connected to the backend.'
	}: Props = $props();
</script>

<svelte:head>
	<title>{title ?? entity} — Coming Soon | Admin</title>
</svelte:head>

<div class="stub-page">
	<header class="page-header">
		<a href={backHref} class="back-link">
			<IconArrowLeft size={18} />
			<span>Back to {backLabel}</span>
		</a>
		<h1>{entity}</h1>
		{#if id !== undefined}
			<p class="page-description">{idLabel}: <code>{id}</code></p>
		{:else}
			<p class="page-description">{description}</p>
		{/if}
	</header>

	<section class="stub-card">
		<div class="stub-icon">
			<IconAlertCircle size={32} />
		</div>
		<h2>Coming soon — backend pending</h2>
		<p>
			{bodyLead} See <code>docs/audits/admin-2026-04-26/06-crm-DEFERRED.md</code>
			for the integration plan.
		</p>
		<a href={backHref} class="btn-back">Return to {backLabel}</a>
	</section>
</div>

<style>
	.stub-page {
		max-width: 720px;
		padding: 1rem;
	}
	.page-header {
		margin-bottom: 1.5rem;
	}
	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		color: #94a3b8;
		text-decoration: none;
		font-size: 0.9rem;
		margin-bottom: 0.75rem;
		transition: color 0.2s;
	}
	.back-link:hover {
		color: #c7d2fe;
	}
	.page-header h1 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}
	.page-description {
		color: #64748b;
		margin: 0;
	}
	.page-description code {
		background: rgba(99, 102, 241, 0.12);
		color: #c7d2fe;
		padding: 0.1rem 0.35rem;
		border-radius: 4px;
		font-size: 0.8rem;
	}
	.stub-card {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.15);
		border-radius: 14px;
		padding: 2.5rem 1.5rem;
		text-align: center;
	}
	.stub-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 64px;
		height: 64px;
		border-radius: 50%;
		background: rgba(245, 158, 11, 0.12);
		color: #fbbf24;
		margin-bottom: 1rem;
	}
	.stub-card h2 {
		color: #f1f5f9;
		font-size: 1.15rem;
		margin: 0 0 0.5rem 0;
	}
	.stub-card p {
		color: #94a3b8;
		font-size: 0.9rem;
		margin: 0 0 1.5rem 0;
		line-height: 1.55;
	}
	.stub-card code {
		background: rgba(99, 102, 241, 0.12);
		color: #c7d2fe;
		padding: 0.1rem 0.35rem;
		border-radius: 4px;
		font-size: 0.8rem;
	}
	.btn-back {
		display: inline-block;
		padding: 0.65rem 1.25rem;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		border-radius: 10px;
		color: white;
		text-decoration: none;
		font-size: 0.9rem;
		font-weight: 600;
		transition: transform 0.2s;
	}
	.btn-back:hover {
		transform: translateY(-1px);
	}
</style>
