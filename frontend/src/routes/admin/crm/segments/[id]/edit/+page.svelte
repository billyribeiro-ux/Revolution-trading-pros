<!--
	Stub page — audit 06-crm.md P0 #1 (orphan-link epidemic).
	Backend wiring tracked in /docs/audits/admin-2026-04-26/06-crm-DEFERRED.md.
-->
<script lang="ts">
	import IconArrowLeft from '@tabler/icons-svelte-runes/icons/arrow-left';
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';
	import { page } from '$app/state';

	let recordId = $derived(page.params.id ?? '');
</script>

<svelte:head>
	<title>Edit Segment — Coming Soon | Admin</title>
</svelte:head>

<div class="stub-page">
	<header class="page-header">
		<a href="/admin/crm/segments" class="back-link">
			<IconArrowLeft size={18} />
			<span>Back to Segments</span>
		</a>
		<h1>Edit Segment</h1>
		<p class="page-description">Record ID: <code>{recordId}</code></p>
	</header>

	<section class="stub-card">
		<div class="stub-icon">
			<IconAlertCircle size={32} />
		</div>
		<h2>Coming soon — backend pending</h2>
		<p>
			This form is not yet connected to the backend. See <code
				>docs/audits/admin-2026-04-26/06-crm-DEFERRED.md</code
			>
			for the integration plan.
		</p>
		<a href="/admin/crm/segments" class="btn-back">Return to Segments</a>
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
