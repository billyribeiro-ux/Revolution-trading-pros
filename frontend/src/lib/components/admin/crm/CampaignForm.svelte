<script lang="ts">
	import { useCrmAccess } from '#lib/crm/access.svelte.js';
	const access = useCrmAccess();
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { crmAPI } from '#lib/api/crm.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	import { sanitizeHtml } from '#lib/utils/sanitize.js';
	import type {
		CampaignType,
		CampaignWrite,
		EmailTemplate,
		SequenceSettings,
		UtmSettings
	} from '#lib/crm/types.js';
	let { id }: { id?: string } = $props();
	let loading = $state(true),
		loaded = $state(false),
		saving = $state(false),
		error = $state('');
	let title = $state(''),
		subject = $state(''),
		preheader = $state(''),
		body = $state('');
	let textBody = $state('');
	let type = $state<CampaignType>('regular');
	let design = $state('raw');
	let templateId = $state<string | null>(null);
	let templateConfig = $state<Record<string, unknown> | null>(null);
	let templateSearch = $state(''),
		searching = $state(false),
		searched = $state(false);
	let templates = $state<EmailTemplate[]>([]);
	let status = $state('draft');
	let customSender = $state(false),
		fromName = $state(''),
		fromEmail = $state(''),
		replyName = $state(''),
		replyEmail = $state('');
	let originalSettings = $state<SequenceSettings | null>(null);
	let utm = $state<UtmSettings>({});
	const preview = $derived(sanitizeHtml(body));
	async function load() {
		loading = true;
		loaded = false;
		error = '';
		try {
			if (id) {
				const campaign = await crmAPI.getCampaign(id);
				title = campaign.title;
				subject = campaign.email_subject ?? '';
				preheader = campaign.email_pre_header ?? '';
				body = campaign.email_body ?? '';
				textBody = campaign.text_body ?? '';
				type = campaign.type;
				design = campaign.design_template;
				templateId = campaign.template_id ?? null;
				templateConfig = campaign.template_config ?? null;
				status = campaign.status;
				originalSettings = campaign.settings ?? null;
				const sender = campaign.settings?.mailer_settings;
				customSender = sender?.is_custom ?? false;
				fromName = sender?.from_name ?? '';
				fromEmail = sender?.from_email ?? '';
				replyName = sender?.reply_to_name ?? '';
				replyEmail = sender?.reply_to_email ?? '';
				utm = { ...campaign.utm_settings };
			}
			loaded = true;
		} catch (cause) {
			error = apiErrorMessage(cause, 'Unable to load campaign');
		} finally {
			loading = false;
		}
	}
	async function findTemplates() {
		if (!access.can('GET', '/templates')) {
			templates = [];
			error = 'Your role cannot choose CRM templates. You can compose an original email.';
			return;
		}
		if (searching) return;
		searching = true;
		error = '';
		try {
			templates = (await crmAPI.getEmailTemplates({ search: templateSearch, per_page: 25 })).data;
			searched = true;
		} catch (cause) {
			error = apiErrorMessage(cause, 'Unable to search templates');
		} finally {
			searching = false;
		}
	}
	function applyTemplate(template: EmailTemplate) {
		templateId = template.id;
		templateConfig = template.template_config ?? null;
		design = template.design_template;
		subject = template.subject ?? '';
		body = template.content;
		templates = [];
		searched = false;
	}
	async function save(event: SubmitEvent) {
		event.preventDefault();
		if (saving || !loaded) return;
		if (!access.can(id ? 'PUT' : 'POST', id ? '/campaigns/{id}' : '/campaigns')) {
			error = 'Your role cannot save this campaign.';
			return;
		}
		saving = true;
		error = '';
		try {
			const numericTemplate = templateId ? Number(templateId) : null;
			if (
				numericTemplate !== null &&
				(!Number.isSafeInteger(numericTemplate) || numericTemplate < 1)
			)
				throw new Error('Template ID is outside the supported range.');
			const fields: CampaignWrite = {
				title: title.trim(),
				email_subject: subject,
				email_pre_header: preheader,
				email_body: body,
				text_body: textBody || null,
				type,
				design_template: design,
				template_id: numericTemplate,
				template_config: templateConfig,
				settings: {
					...originalSettings,
					mailer_settings: {
						from_name: fromName,
						from_email: fromEmail,
						reply_to_name: replyName,
						reply_to_email: replyEmail,
						is_custom: customSender
					}
				},
				utm_settings: utm
			};
			const saved = id
				? await crmAPI.updateCampaign(id, fields)
				: await crmAPI.createCampaign(fields);
			await goto(`/admin/crm/campaigns/${saved.id}`, { invalidateAll: true });
		} catch (cause) {
			error = apiErrorMessage(cause, 'Unable to save campaign');
		} finally {
			saving = false;
		}
	}
	onMount(() => {
		void load();
	});
</script>

<section class="campaign-form">
	<a href="/admin/crm/campaigns">← Campaigns</a>
	<h1>{id ? 'Edit Campaign' : 'New Campaign'}</h1>
	{#if error}<p role="alert">{error}</p>{/if}
	{#if loading}<p role="status">Loading campaign…</p>{:else if !loaded}<button
			type="button"
			onclick={() => void load()}>Retry loading</button
		>{:else}
		<p>Save the email before choosing recipients and scheduling delivery.</p>
		{#if status !== 'draft'}<p>
				Current status: {status}. Sending or sent campaigns cannot be edited.
			</p>{/if}
		<form onsubmit={(event) => void save(event)}>
			<label for="campaign-title">Campaign title</label><input
				id="campaign-title"
				bind:value={title}
				required
				maxlength={255}
			/>
			<label for="campaign-subject">Email subject</label><input
				id="campaign-subject"
				bind:value={subject}
				maxlength={500}
			/>
			<label for="campaign-preheader">Preheader</label><input
				id="campaign-preheader"
				bind:value={preheader}
				maxlength={500}
			/>
			<label for="campaign-type">Email format</label><select id="campaign-type" bind:value={type}
				><option value="regular">HTML email</option><option value="plain_text">Plain text</option
				><option value="raw_html">Custom HTML</option></select
			>
			<fieldset>
				<legend>Start from an email template</legend><label for="campaign-template-search"
					>Find template</label
				>
				<div class="search">
					<input id="campaign-template-search" bind:value={templateSearch} /><button
						type="button"
						disabled={searching}
						onclick={() => void findTemplates()}
						>{searching ? 'Searching…' : 'Search templates'}</button
					>
				</div>
				{#if searched && !templates.length}<p>No matching templates.</p>{/if}
				<ul>
					{#each templates as template (template.id)}<li>
							<button type="button" onclick={() => applyTemplate(template)}
								>Use {template.title}</button
							>
						</li>{/each}
				</ul>
				{#if templateId}<p>
						Based on template #{templateId}. Applying another template replaces the subject and
						body.
					</p>
					<button
						type="button"
						onclick={() => {
							templateId = null;
							templateConfig = null;
							design = 'raw';
						}}>Unlink template</button
					>{/if}
			</fieldset>
			<label for="campaign-body">{type === 'plain_text' ? 'Email text' : 'Email HTML'}</label
			><textarea id="campaign-body" bind:value={body} maxlength={1000000} rows={18}></textarea>
			{#if type !== 'plain_text'}
				<label for="campaign-text-alternative">Plain-text alternative (optional)</label>
				<textarea id="campaign-text-alternative" bind:value={textBody} maxlength={1000000} rows={5}
				></textarea>
			{/if}
			<details>
				<summary>Preview email</summary>{#if type === 'plain_text'}<pre>{body}</pre>{:else}<iframe
						title="Campaign email preview"
						sandbox=""
						srcdoc={preview}
					></iframe>{/if}
			</details>
			<fieldset>
				<legend>Sender</legend><label class="check"
					><input type="checkbox" bind:checked={customSender} />Use a custom sender</label
				>
				<p>
					{customSender
						? 'The email provider must authorize this sender before delivery.'
						: 'Use the sender configured for the email provider.'}
				</p>
				{#if customSender}<label for="campaign-from-name">From name</label><input
						id="campaign-from-name"
						bind:value={fromName}
						maxlength={255}
					/><label for="campaign-from-email">From email</label><input
						id="campaign-from-email"
						type="email"
						bind:value={fromEmail}
						required
						maxlength={255}
					/>{/if}
				<label for="campaign-reply-name">Reply-to name</label><input
					id="campaign-reply-name"
					bind:value={replyName}
					maxlength={255}
				/><label for="campaign-reply-email">Reply-to email</label><input
					id="campaign-reply-email"
					type="email"
					bind:value={replyEmail}
					maxlength={255}
				/>
			</fieldset>
			<details>
				<summary>Campaign link tracking</summary><label for="campaign-utm-source">UTM source</label
				><input id="campaign-utm-source" bind:value={utm.utm_source} maxlength={255} /><label
					for="campaign-utm-medium">UTM medium</label
				><input id="campaign-utm-medium" bind:value={utm.utm_medium} maxlength={255} /><label
					for="campaign-utm-campaign">UTM campaign</label
				><input id="campaign-utm-campaign" bind:value={utm.utm_campaign} maxlength={255} /><label
					for="campaign-utm-term">UTM term</label
				><input id="campaign-utm-term" bind:value={utm.utm_term} maxlength={255} /><label
					for="campaign-utm-content">UTM content</label
				><input id="campaign-utm-content" bind:value={utm.utm_content} maxlength={255} />
			</details>
			<div class="actions">
				<button type="submit" disabled={saving}>{saving ? 'Saving…' : 'Save Campaign'}</button><a
					href={id ? `/admin/crm/campaigns/${id}` : '/admin/crm/campaigns'}>Cancel</a
				>
			</div>
		</form>{/if}
</section>

<style>
	.campaign-form {
		max-width: 64rem;
		margin: auto;
		padding: 2rem;
		color: var(--text-primary);
	}
	h1 {
		font-size: 1.75rem;
		margin: 1rem 0;
	}
	form {
		display: grid;
		gap: 0.75rem;
	}
	label {
		display: block;
		font-weight: 600;
		margin-top: 0.5rem;
	}
	input,
	select,
	textarea,
	button {
		padding: 0.75rem;
		border: 1px solid var(--border-color, #475569);
		border-radius: 0.5rem;
		background: var(--bg-secondary, #0f172a);
		color: inherit;
	}
	input,
	select,
	textarea {
		width: 100%;
	}
	textarea,
	pre {
		font-family: monospace;
	}
	pre {
		white-space: pre-wrap;
	}
	button {
		cursor: pointer;
	}
	.check,
	.actions,
	.search {
		display: flex;
		align-items: center;
		gap: 1rem;
	}
	.check input {
		width: auto;
	}
	.search input {
		flex: 1;
		min-width: 0;
	}
	.search button {
		flex-shrink: 0;
	}
	fieldset,
	details {
		padding: 1rem;
		border: 1px solid var(--border-color, #475569);
		border-radius: 0.5rem;
	}
	summary {
		cursor: pointer;
		font-weight: 600;
	}
	iframe {
		display: block;
		width: 100%;
		min-height: 24rem;
		background: white;
		border: 0;
		margin-top: 1rem;
	}
	[role='alert'] {
		color: #fca5a5;
	}
	ul {
		list-style: none;
		padding: 0;
	}
	li {
		margin: 0.5rem 0;
	}
	a {
		color: var(--accent-color, #7dd3fc);
	}
</style>
