<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { crmAPI } from '#lib/api/crm.js';
	import { sequenceConditions } from '#lib/api/email-sequence-catalog.js';
	import { sequenceMailer, type SequenceMailWrite, type SequenceWrite } from '#lib/api/crm-sequence-contract.js';
	import type { EmailSequence, SequenceStats } from '#lib/crm/types.js';
	import type { JsonValue } from '#lib/api/_types.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	let { id }: { id?: string } = $props();
	type DraftMail = SequenceMailWrite & { id: string; conditionSource: string };
	let saved = $state<EmailSequence | null>(null);
	let stats = $state<SequenceStats | null>(null);
	let title = $state('');
	let description = $state('');
	let format = $state('raw_html');
	let sender = $state(sequenceMailer(undefined));
	let audience = $state('{}');
	let trigger = $state<EmailSequence['trigger_type']>('manual');
	let triggerConfig = $state('{}');
	let mails = $state<DraftMail[]>([]);
	let loading = $state(true);
	let ready = $state(false);
	let busy = $state(false);
	let error = $state('');
	let notice = $state('');
	let createSlug = '';
	let pendingCreateTitle = '';
	const active = $derived(saved?.status === 'active');
	function apply(sequence: EmailSequence) {
		saved = sequence; title = sequence.title; description = sequence.description ?? ''; format = sequence.design_template;
		sender = { ...(sequence.settings?.mailer_settings ?? sequenceMailer(undefined)) };
		audience = JSON.stringify(sequence.subscriber_settings ?? {}, null, 2);
		trigger = sequence.trigger_type; triggerConfig = JSON.stringify(sequence.trigger_config, null, 2);
		mails = (sequence.emails ?? []).map(mail => ({ id: mail.id, title: mail.title, email_subject: mail.email_subject, email_pre_header: mail.email_pre_header ?? '', email_body: mail.email_body, text_body: mail.text_body ?? '', delay_value: mail.delay_value, delay_unit: mail.delay_unit, status: mail.status, settings: { ...(mail.settings ?? sequenceMailer(undefined)) }, conditions: mail.conditions, conditionSource: JSON.stringify(mail.conditions, null, 2) }));
	}
	async function load() {
		if (busy) return;
		loading = true; ready = false; error = ''; notice = '';
		try {
			if (id) { const result = await crmAPI.getSequence(id); apply(result.sequence); stats = result.stats; }
			else if (!createSlug) createSlug = `sequence-${crypto.randomUUID()}`;
			ready = true;
		} catch (cause) { error = apiErrorMessage(cause, 'Unable to load sequence'); }
		finally { loading = false; }
	}
	function configuration(source: string, label: string): Record<string, JsonValue> {
		const value: unknown = JSON.parse(source);
		if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error(`${label} must be an object`);
		return value as Record<string, JsonValue>;
	}
	function addMail() {
		mails = [...mails, { id: crypto.randomUUID(), title: `Email ${mails.length + 1}`, email_subject: '', email_pre_header: '', email_body: '', text_body: '', delay_value: 0, delay_unit: 'days', status: 'active', settings: sequenceMailer(undefined), conditions: [], conditionSource: '[]' }];
	}
	function move(index: number, by: number) {
		const next = [...mails], item = next[index], target = index + by;
		if (!item || target < 0 || target >= next.length) return;
		next.splice(index, 1); next.splice(target, 0, item); mails = next;
	}
	async function recoverDraft(): Promise<EmailSequence | null> {
		let page = 1;
		while (true) {
			const result = await crmAPI.getSequences({ search: pendingCreateTitle, page, per_page: 100 });
			const existing = result.data.find(sequence => sequence.slug === createSlug);
			if (existing) return existing;
			if (page >= (result.meta.total_pages ?? 1)) return null;
			page += 1;
		}
	}
	async function save(event: SubmitEvent) {
		event.preventDefault();
		if (busy || !ready || active) return;
		busy = true; error = ''; notice = '';
		try {
			if (!saved && pendingCreateTitle) {
				const existing = await recoverDraft();
				if (existing) { apply(existing); await goto(`/admin/crm/sequences/${existing.id}/edit`); return; }
			}
			const fields: SequenceWrite = { title, description, design_template: format, settings: { mailer_settings: sender }, subscriber_settings: configuration(audience, 'Audience filters'), trigger_type: trigger, trigger_config: configuration(triggerConfig, 'Trigger settings'), emails: mails.map(({ conditionSource, ...mail }) => ({ ...mail, conditions: sequenceConditions(JSON.parse(conditionSource)) })) };
			let result: EmailSequence;
			if (saved) result = await crmAPI.updateSequence(saved.id, { ...fields, expected_version: saved.authoring_version });
			else { pendingCreateTitle = title; result = await crmAPI.createSequence({ ...fields, slug: createSlug }); }
			apply(result); notice = 'Sequence saved. Existing enrollments retain their saved email revision.';
			if (!id) await goto(`/admin/crm/sequences/${result.id}/edit`);
			else stats = (await crmAPI.getSequence(result.id)).stats;
		} catch (cause) { error = apiErrorMessage(cause, 'Save was not confirmed. Check saved state before retrying.'); }
		finally { busy = false; }
	}
	async function status() {
		if (busy || !saved) return;
		busy = true; error = ''; notice = '';
		try {
			const result = await crmAPI.updateSequence(saved.id, { status: active ? 'paused' : 'active', expected_version: saved.authoring_version });
			// Retain unsaved edits while refreshing the server version and status.
			saved = result; stats = (await crmAPI.getSequence(result.id)).stats;
			notice = result.status === 'active' ? 'Saved sequence activated.' : 'Sequence paused. An in-flight submission retains its actual provider receipt.';
		} catch (cause) { error = apiErrorMessage(cause, 'Status change was not confirmed; refresh saved state.'); }
		finally { busy = false; }
	}
	onMount(() => { void load(); });
</script>

<section class="sequence-form">
	<a href="/admin/crm/sequences">← Sequences</a>
	<header><h1>{id ? 'Edit Sequence' : 'New Sequence'}</h1><button type="button" onclick={() => void load()} disabled={busy || loading}>Reload saved state</button></header>
	{#if error}<p role="alert">{error}</p>{/if}
	{#if notice}<p role="status">{notice}</p>{/if}
	{#if loading}<p role="status">Loading sequence…</p>{:else if ready}
		{#if saved}
			<div class="saved"><span>Saved status: <strong>{saved.status}</strong> · Version {saved.authoring_version}</span><button type="button" onclick={() => void status()} disabled={busy}>{active ? 'Pause sequence' : 'Activate saved sequence'}</button><a href="/admin/crm/sequences/{saved.id}">Analytics</a><a href="/admin/crm/sequences/{saved.id}/subscribers">Subscribers</a></div>
			{#if saved.last_error}<p role="alert">{saved.last_error}</p>{/if}
			{#if saved.definition_error}<p role="alert">Stored emails need review: {saved.definition_error}. Saving reviewed emails preserves the original revision.</p><details><summary>Original stored emails</summary><pre>{JSON.stringify(saved.historical_steps, null, 2)}</pre></details>{/if}
			{#if stats}<p>Recorded receipts: {stats.total_sent} accepted · {stats.total_delivered} delivered · {stats.failed} failed · {stats.unknown} awaiting reconciliation · {stats.queued} queued · {stats.processing} processing</p>{/if}
		{/if}
		{#if active}<p>Pause the sequence before editing. Enrolled contacts continue with their saved revision.</p>{/if}
		<form onsubmit={(event) => void save(event)}>
			<fieldset disabled={busy || active}>
				<legend>Sequence details</legend>
				<label for="sequence-title">Sequence title</label><input id="sequence-title" bind:value={title} required maxlength={255} />
				<label for="sequence-description">Description</label><textarea id="sequence-description" bind:value={description} maxlength={4000} rows={3}></textarea>
				<label for="sequence-format">Content format</label><select id="sequence-format" bind:value={format}><option value="raw_html">HTML with text alternative</option><option value="plain_text">Plain text</option></select>
				<label for="sequence-trigger">Enrollment trigger</label><select id="sequence-trigger" bind:value={trigger}><option value="manual">Manual</option><option value="signup">Signup</option><option value="purchase">Purchase</option><option value="tag_added">Tag added</option><option value="custom_event">Custom event</option></select>
				<label for="sequence-trigger-config">Trigger settings (JSON)</label><textarea id="sequence-trigger-config" bind:value={triggerConfig} rows={3} maxlength={65536}></textarea>
				<label for="sequence-audience">Audience filters (JSON)</label><textarea id="sequence-audience" bind:value={audience} rows={4} maxlength={65536}></textarea><p>Only confirmed contacts matching the saved filters can enroll. Consent is checked again before each submission.</p>
			</fieldset>
			<fieldset disabled={busy || active}><legend>Default sender</legend><label class="check"><input type="checkbox" bind:checked={sender.is_custom} /> Use a custom sender</label><label for="sequence-from-name">From name</label><input id="sequence-from-name" bind:value={sender.from_name} maxlength={255} /><label for="sequence-from-email">From email</label><input id="sequence-from-email" type="email" bind:value={sender.from_email} required={sender.is_custom} /><label for="sequence-reply-name">Reply-to name</label><input id="sequence-reply-name" bind:value={sender.reply_to_name} maxlength={255} /><label for="sequence-reply-email">Reply-to email</label><input id="sequence-reply-email" type="email" bind:value={sender.reply_to_email} /></fieldset>
			{#each mails as mail, index (mail.id)}
				<fieldset disabled={busy || active} data-sequence-step={index + 1}>
					<legend>Email {index + 1}</legend>
					<div class="step-actions"><button type="button" onclick={() => move(index, -1)} disabled={index === 0}>Move up</button><button type="button" onclick={() => move(index, 1)} disabled={index === mails.length - 1}>Move down</button><button type="button" onclick={() => { mails = mails.filter(other => other.id !== mail.id); }}>Remove email</button></div>
					<label for="mail-title-{mail.id}">Email title</label><input id="mail-title-{mail.id}" bind:value={mail.title} required maxlength={255} />
					<label for="mail-subject-{mail.id}">Subject</label><input id="mail-subject-{mail.id}" bind:value={mail.email_subject} required maxlength={500} />
					<label for="mail-preheader-{mail.id}">Preheader</label><input id="mail-preheader-{mail.id}" bind:value={mail.email_pre_header} maxlength={500} />
					<label for="mail-body-{mail.id}">{format === 'plain_text' ? 'Email text' : 'Email HTML'}</label><textarea id="mail-body-{mail.id}" bind:value={mail.email_body} rows={12} required maxlength={1000000}></textarea>
					<label for="mail-text-{mail.id}">Plain text alternative</label><textarea id="mail-text-{mail.id}" bind:value={mail.text_body} rows={5} maxlength={1000000}></textarea>
					<label for="mail-delay-{mail.id}">Delay after the previous accepted email (first email: after enrollment)</label><input id="mail-delay-{mail.id}" type="number" min={0} max={527040} step={1} bind:value={mail.delay_value} required /><select aria-label="Delay unit for email {index + 1}" bind:value={mail.delay_unit}><option value="minutes">Minutes</option><option value="hours">Hours</option><option value="days">Days</option><option value="weeks">Weeks</option></select>
					<label for="mail-status-{mail.id}">Step status</label><select id="mail-status-{mail.id}" bind:value={mail.status}><option value="active">Active</option><option value="draft">Draft</option><option value="paused">Paused</option></select>
					<details><summary>Sender override and conditions</summary><label class="check"><input type="checkbox" bind:checked={mail.settings.is_custom} /> Override sender for this email</label><label for="mail-from-name-{mail.id}">From name</label><input id="mail-from-name-{mail.id}" bind:value={mail.settings.from_name} maxlength={255} /><label for="mail-from-email-{mail.id}">From email</label><input id="mail-from-email-{mail.id}" type="email" bind:value={mail.settings.from_email} required={mail.settings.is_custom} /><label for="mail-reply-name-{mail.id}">Reply-to name</label><input id="mail-reply-name-{mail.id}" bind:value={mail.settings.reply_to_name} maxlength={255} /><label for="mail-reply-email-{mail.id}">Reply-to email</label><input id="mail-reply-email-{mail.id}" type="email" bind:value={mail.settings.reply_to_email} /><p>Email ID: <code>{mail.id}</code>. Open and click conditions may reference an earlier email ID.</p><label for="mail-conditions-{mail.id}">Conditions (JSON array)</label><textarea id="mail-conditions-{mail.id}" bind:value={mail.conditionSource} rows={5} maxlength={65536}></textarea><p>Supported types: opened, clicked, not_opened, not_clicked, tag and custom. All conditions must match.</p></details>
				</fieldset>
			{/each}
			<div class="step-actions"><button type="button" onclick={addMail} disabled={busy || active || mails.length >= 50}>Add email</button><button type="submit" disabled={busy || active}>{busy ? 'Working…' : saved ? 'Save sequence' : 'Create draft'}</button></div>
		</form>
	{/if}
</section>

<style>
	.sequence-form { max-width: 76rem; padding: 2rem; color: #e2e8f0; }
	header, .saved, .step-actions { display: flex; align-items: center; flex-wrap: wrap; gap: 1rem; }
	header { justify-content: space-between; }
	form, fieldset, details[open] { display: grid; gap: .75rem; }
	fieldset { margin: 1rem 0; border: 1px solid #475569; border-radius: .75rem; padding: 1.25rem; min-width: 0; }
	input, select, textarea, button { padding: .75rem; border: 1px solid #475569; border-radius: .5rem; background: #0f172a; color: inherit; max-width: 100%; }
	textarea, pre { font-family: monospace; white-space: pre-wrap; overflow-wrap: anywhere; }
	.check { display: flex; gap: .5rem; align-items: center; }
	[role='alert'] { color: #fca5a5; }
	[role='status'] { color: #86efac; }
	a { color: #7dd3fc; }
	button:disabled { opacity: .55; }
</style>
