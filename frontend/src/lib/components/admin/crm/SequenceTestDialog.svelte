<script lang="ts">
	import { useCrmAccess } from '#lib/crm/access.svelte.js';
	const access = useCrmAccess();
	import { onMount } from 'svelte';
	import { crmAPI } from '#lib/api/crm.js';
	import {
		getSequenceTestHistory,
		sendSequenceTest,
		type SequenceTestBatch,
		type SequenceTestIntent
	} from '#lib/api/sequence-test.js';
	import type { EmailSequence } from '#lib/crm/types.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	let { sequenceId, onclose }: { sequenceId: string; onclose: () => void } = $props();
	const uid = $props.id();
	let sequence = $state<EmailSequence | null>(null);
	let email = $state(''),
		emailId = $state('');
	let intent = $state<SequenceTestIntent | null>(null);
	let requests = $state<SequenceTestBatch[]>([]);
	let loading = $state(true),
		busy = $state(false),
		error = $state(''),
		historyError = $state('');
	let disposed = false;
	const unresolved = $derived(
		requests.some((request) =>
			request.submissions.some((receipt) => ['unknown', 'processing'].includes(receipt.status))
		)
	);
	const currentOutcome = $derived(
		requests.find((request) => request.request_id === intent?.request_id)
	);
	const canPrepare = $derived(!historyError && !unresolved && (!intent || !!currentOutcome));
	const labels = {
		accepted: 'Provider accepted',
		rejected: 'Provider rejected',
		unavailable: 'Provider unavailable',
		unknown: 'Acceptance unknown — reconcile before another test',
		processing: 'Submission in progress — check receipts'
	};
	function modal(node: HTMLDialogElement) {
		node.showModal();
		return () => node.close();
	}
	async function history() {
		historyError = '';
		if (!access.can('GET', '/sequences/{id}/tests')) {
			requests = [];
			historyError = 'Your role cannot read saved QA receipts.';
			return;
		}
		try {
			const saved = await getSequenceTestHistory(sequenceId);
			if (!disposed) requests = saved;
		} catch (cause) {
			if (!disposed)
				historyError = apiErrorMessage(cause, 'Saved sequence test receipts are unavailable');
		}
	}
	async function load() {
		if (busy) return;
		loading = true;
		error = '';
		try {
			const saved = await crmAPI.getSequence(sequenceId);
			if (disposed) return;
			sequence = saved.sequence;
			emailId = sequence.emails?.[0]?.id ?? '';
		} catch (cause) {
			if (!disposed) error = apiErrorMessage(cause, 'Saved sequence is unavailable');
		}
		await history();
		if (!disposed) loading = false;
	}
	async function send(replay?: SequenceTestBatch) {
		if (busy || !sequence) return;
		if (!access.can('POST', '/sequences/{id}/test')) {
			error = 'Sequence QA sending requires manage permission.';
			return;
		}
		busy = true;
		error = '';
		try {
			if (replay) {
				email = replay.submissions[0]?.recipient ?? '';
				emailId = replay.email_id;
				intent = {
					email,
					email_id: replay.email_id,
					request_id: replay.request_id,
					expected_version: replay.authoring_version
				};
			}
			intent ??= {
				email: email.trim().toLowerCase(),
				email_id: emailId,
				request_id: crypto.randomUUID(),
				expected_version: sequence.authoring_version
			};
			const result = await sendSequenceTest(sequenceId, intent);
			if (!disposed)
				requests = [
					result,
					...requests.filter((request) => request.request_id !== result.request_id)
				].slice(0, 20);
		} catch (cause) {
			if (!disposed)
				error = apiErrorMessage(
					cause,
					'Test outcome is unconfirmed; check saved receipts before retrying'
				);
		}
		await history();
		if (!disposed) busy = false;
	}
	async function check() {
		if (busy) return;
		busy = true;
		await history();
		if (!disposed) busy = false;
	}
	function another() {
		if (!busy && canPrepare) {
			intent = null;
			error = '';
		}
	}
	onMount(() => {
		void load();
		return () => {
			disposed = true;
		};
	});
</script>

<dialog
	{@attach modal}
	aria-labelledby="{uid}-title"
	oncancel={(event) => {
		event.preventDefault();
		if (!busy) onclose();
	}}
>
	<header>
		<h2 id="{uid}-title">Sequence test message</h2>
		<button type="button" disabled={busy} onclick={onclose} aria-label="Close sequence test message"
			>×</button
		>
	</header>
	<p>
		Send one saved email to an explicit test recipient. Provider acceptance does not establish inbox
		delivery. Tests do not enroll contacts or change sequence delivery metrics.
	</p>
	{#if loading}<p role="status">Loading saved sequence and test receipts…</p>
	{:else if sequence}
		<h3>{sequence.title}</h3>
		{#if sequence.definition_error}<p role="alert">
				Review the saved email definition before preparing a new test: {sequence.definition_error}
			</p>{/if}
		<form
			onsubmit={(event) => {
				event.preventDefault();
				void send();
			}}
		>
			<label for="{uid}-mail">Saved email</label><select
				id="{uid}-mail"
				bind:value={emailId}
				disabled={busy || intent !== null}
				>{#each sequence.emails ?? [] as mail (mail.id)}<option value={mail.id}
						>Email {mail.position + 1}: {mail.title} — {mail.email_subject}</option
					>{/each}</select
			>
			<label for="{uid}-recipient">Test recipient</label><input
				id="{uid}-recipient"
				type="email"
				required
				maxlength={255}
				bind:value={email}
				disabled={busy || intent !== null}
			/>
			{#if intent}<p>
					Request: <code>{intent.request_id}</code>. Replaying this saved request reads its existing
					outcome without another submission.
				</p>
				<div class="actions">
					<button type="submit" disabled={busy}>Retry same request</button><button
						type="button"
						disabled={busy || !canPrepare}
						onclick={another}>Prepare another test</button
					>
				</div>
				{#if !canPrepare}<p>
						Check saved receipts or retry the same request before preparing another test.
					</p>{/if}
			{:else}<button
					type="submit"
					disabled={busy ||
						!emailId ||
						!!sequence.definition_error ||
						unresolved ||
						!!historyError ||
						!access.can('POST', '/sequences/{id}/test') ||
						!access.can('GET', '/sequences/{id}/tests')}>Send test message</button
				>{/if}
		</form>
	{/if}
	{#if error}<p role="alert">{error}</p>
		{#if !sequence}<button type="button" disabled={busy || loading} onclick={() => void load()}
				>Retry loading sequence</button
			>{/if}{/if}
	{#if access.can('GET', '/sequences/{id}/tests')}<section
			aria-label="Saved sequence test receipts"
		>
			<div class="actions">
				<h3>Saved test receipts</h3>
				<button type="button" disabled={busy || loading} onclick={() => void check()}
					>Check receipts</button
				>
			</div>
			{#if historyError}<p role="alert">{historyError}</p>{:else if !loading && !requests.length}<p>
					No recorded sequence tests.
				</p>{/if}
			{#each requests as request (request.request_id)}<article>
					<p>
						Saved version {request.authoring_version} · email <code>{request.email_id}</code> ·
						<time datetime={request.created_at}>{request.created_at}</time>
					</p>
					{#each request.submissions as receipt (receipt.id)}<p>
							{receipt.recipient}: <strong>{labels[receipt.status]}</strong> ({receipt.provider})
						</p>
						{#if receipt.provider_message_id}<p>
								Provider receipt: <code>{receipt.provider_message_id}</code>
							</p>{/if}{#if receipt.error}<p>{receipt.error}</p>{/if}{/each}<button
						type="button"
						disabled={busy || loading || !sequence}
						onclick={() => void send(request)}>Read this request again</button
					>
				</article>{/each}
		</section>{:else}<p>Saved QA receipts require sequence management permission.</p>{/if}
</dialog>

<style>
	dialog {
		color: #e2e8f0;
		background: #111827;
		border: 1px solid #475569;
		border-radius: 16px;
		padding: 24px;
		width: min(720px, calc(100vw - 32px));
		max-height: calc(100vh - 48px);
		overflow: auto;
	}
	dialog::backdrop {
		background: rgb(0 0 0 / 65%);
	}
	header,
	.actions {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
		flex-wrap: wrap;
	}
	h2,
	h3 {
		margin: 0;
	}
	p {
		margin: 12px 0;
		overflow-wrap: anywhere;
	}
	form {
		display: grid;
		gap: 10px;
		margin: 20px 0;
	}
	input,
	select,
	button {
		font: inherit;
		color: inherit;
		background: #1e293b;
		border: 1px solid #64748b;
		border-radius: 8px;
		padding: 10px 12px;
	}
	button {
		cursor: pointer;
	}
	button:disabled {
		opacity: 0.6;
		cursor: wait;
	}
	[role='alert'] {
		color: #fca5a5;
	}
	article {
		border-top: 1px solid #334155;
		margin-top: 16px;
		padding-top: 8px;
	}
</style>
