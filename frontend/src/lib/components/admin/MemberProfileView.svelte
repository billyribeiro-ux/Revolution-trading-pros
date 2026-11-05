<script lang="ts">
	import { fromAction } from 'svelte/attachments';
	import {
		memberProfileApi,
		memberProfileRequest,
		MemberProfileRequestError,
		type MemberProfileRecord,
		type MemberMembership,
		type MemberOrderRecord
	} from '#lib/api/member-profile.js';
	import {
		memberProfileActions,
		memberNumericIdentity,
		operationalExpiry,
		extendedRecordedExpiry,
		type ProfileTag
	} from '#lib/api/member-profile-actions.js';
	import { downloadMemberOrders } from '#lib/api/member-order-export.js';
	import { membersApi, type MemberEditSeed } from '#lib/api/members.js';
	import { MemberProfileResource } from './member-profile-resource.svelte.js';
	import MemberProfileDialog from './MemberProfileDialog.svelte';
	import MemberFormModal from './MemberFormModal.svelte';
	import MemberHeader from '../../../routes/admin/members/[id]/_components/MemberHeader.svelte';
	import MemberStatsGrid from '../../../routes/admin/members/[id]/_components/MemberStatsGrid.svelte';
	import SubscriptionsTab from '../../../routes/admin/members/[id]/_components/SubscriptionsTab.svelte';
	import OrdersTab from '../../../routes/admin/members/[id]/_components/OrdersTab.svelte';
	import EmailsTab from '../../../routes/admin/members/[id]/_components/EmailsTab.svelte';
	import NotesTab from '../../../routes/admin/members/[id]/_components/NotesTab.svelte';
	import HistoryControls from '../../../routes/admin/members/[id]/_components/HistoryControls.svelte';
	import ExtendMembershipModal from '../../../routes/admin/members/[id]/_components/ExtendMembershipModal.svelte';
	import {
		recordedInstant,
		recordedWallClock,
		recordedUnits
	} from '../../../routes/admin/members/[id]/_components/profile-display.js';
	let {
		memberId,
		compact = false,
		onEdit,
		onEmail,
		onRefresh
	}: {
		memberId: string | number;
		compact?: boolean;
		onEdit?: (member: MemberProfileRecord) => void;
		onEmail?: (member: MemberProfileRecord) => void;
		onRefresh?: () => void;
	} = $props();
	const core = new MemberProfileResource((id, _query, signal) =>
		memberProfileApi.get(id, { signal })
	);
	const memberships = new MemberProfileResource((id, query, signal) =>
		memberProfileApi.memberships(id, query, { signal })
	);
	const orders = new MemberProfileResource((id, query, signal) =>
		memberProfileApi.orders(id, query, { signal })
	);
	const emails = new MemberProfileResource((id, query, signal) =>
		memberProfileApi.emails(id, query, { signal })
	);
	const notes = new MemberProfileResource((id, query, signal) =>
		memberProfileApi.notes(id, query, { signal })
	);
	const activity = new MemberProfileResource((id, query, signal) =>
		memberProfileApi.activity(id, query, { signal })
	);
	const timeline = new MemberProfileResource((id, query, signal) =>
		memberProfileApi.timeline(id, query, { signal })
	);
	type Tab = 'overview' | 'subscriptions' | 'orders' | 'emails' | 'notes' | 'activity';
	let activeTab = $state<Tab>('overview');
	const tabs: Tab[] = ['overview', 'subscriptions', 'orders', 'emails', 'notes', 'activity'];
	let generation = 0;
	let modal = $state<
		| ''
		| 'note'
		| 'tags'
		| 'grant'
		| 'revoke'
		| 'email'
		| 'ban'
		| 'suspend'
		| 'unban'
		| 'order'
		| 'delete-note'
	>('');
	let actionBusy = $state(false),
		actionError = $state(''),
		actionMessage = $state(''),
		actionUncertain = $state(false);
	let noteDraft = $state(''),
		noteError = $state(''),
		noteBusy = $state(false),
		noteUncertain = $state(false);
	let tagSearch = $state(''),
		newTag = $state(''),
		tagResult = $state<Awaited<ReturnType<typeof memberProfileActions.tags>> | null>(null),
		tagsLoading = $state(false),
		tagsError = $state(''),
		tagsBusy = $state(false),
		tagsUncertain = $state(false);
	let catalogGeneration = 0;
	let planSearch = $state(''),
		planResult = $state<Awaited<ReturnType<typeof membersApi.getServices>> | null>(null),
		plansLoading = $state(false),
		plansError = $state(''),
		selectedPlan = $state(''),
		grantExpiry = $state('');
	let selectedMembership = $state<MemberMembership | null>(null),
		extendDays = $state(30),
		newExpiry = $state('');
	let showExtend = $state(false),
		showEdit = $state(false);
	let pendingNote = $state(''),
		pendingMembership = $state(''),
		selectedOrder = $state<MemberOrderRecord | null>(null);
	let reason = $state(''),
		duration = $state<number | undefined>(undefined),
		emailSubject = $state(''),
		emailBody = $state('');
	let activityFilter = $state('');
	let editSeed = $derived<MemberEditSeed | null>(
		core.result
			? {
					id: core.result.member.id,
					name: core.result.member.name,
					email: core.result.member.email,
					first_name: core.result.member.first_name,
					last_name: core.result.member.last_name,
					role: core.result.member.role,
					avatar: core.result.member.avatar,
					tags: core.result.member.tags
				}
			: null
	);
	const current = (version: number) => version === generation;
	function message(cause: unknown) {
		return cause instanceof Error ? cause.message : 'The saved member state could not be confirmed';
	}
	function uncertain(cause: unknown) {
		return cause instanceof MemberProfileRequestError && cause.reconcile_history_before_retry;
	}
	function reset() {
		for (const resource of [core, memberships, orders, emails, notes, activity, timeline])
			resource.reset();
		modal = '';
		showExtend = false;
		showEdit = false;
		selectedMembership = null;
		selectedOrder = null;
		activeTab = 'overview';
		actionBusy = false;
		actionError = '';
		actionMessage = '';
		actionUncertain = false;
		noteDraft = '';
		noteBusy = false;
		noteError = '';
		noteUncertain = false;
		tagResult = null;
		tagsError = '';
		tagsBusy = false;
		tagsUncertain = false;
		tagsLoading = false;
		tagSearch = '';
		newTag = '';
		planResult = null;
		plansLoading = false;
		plansError = '';
		planSearch = '';
		emailSubject = '';
		emailBody = '';
		activityFilter = '';
		++catalogGeneration;
	}
	async function loadProfile(seed = true, id: string | number = memberId) {
		const version = generation;
		const result = await core.load(id);
		if (!result || !current(version)) return;
		if (seed) {
			memberships.seed(id, result.memberships);
			orders.seed(id, result.orders);
			emails.seed(id, result.emails);
			notes.seed(id, result.notes);
			activity.seed(id, result.activity);
			timeline.seed(id, result.timeline);
		}
		return result;
	}
	function connectProfile(_element: HTMLElement, id: string | number) {
		let active = id;
		function initialize(next: string | number) {
			++generation;
			reset();
			void loadProfile(true, next);
		}
		initialize(id);
		return {
			update(next: string | number) {
				if (next !== active) {
					active = next;
					initialize(next);
				}
			},
			destroy() {
				++generation;
				reset();
			}
		};
	}
	const profileAttachment = $derived(fromAction(connectProfile, () => memberId));
	async function refreshNotes() {
		const version = generation;
		const result = await notes.load(memberId);
		if (result && current(version)) {
			noteUncertain = false;
			if (noteError)
				noteError = 'History refreshed. Review the saved notes before explicitly submitting again.';
		}
	}
	async function saveNote() {
		if (noteBusy || noteUncertain || !noteDraft.trim()) return;
		const id = memberId,
			version = generation,
			content = noteDraft.trim();
		noteBusy = true;
		noteError = '';
		try {
			const saved = await memberProfileApi.addNote(id, content);
			if (!current(version)) return;
			noteDraft = '';
			modal = '';
			actionMessage = `Note #${saved.id} saved with its recorded author.`;
			await notes.load(id, { page: 1 });
			void timeline.load(id);
			onRefresh?.();
		} catch (cause) {
			if (current(version)) {
				noteError = message(cause);
				noteUncertain = uncertain(cause);
			}
		} finally {
			if (current(version)) noteBusy = false;
		}
	}
	async function deleteNote(id: string) {
		if (noteBusy || noteUncertain) return;
		const target = memberId,
			version = generation;
		noteBusy = true;
		noteError = '';
		try {
			const receipt = await memberProfileApi.removeNote(target, id);
			if (!current(version)) return;
			modal = '';
			actionMessage = receipt.removed ? `Note #${id} removed.` : `Note #${id} was already absent.`;
			await notes.load(target, { page: 1 });
			void timeline.load(target);
			onRefresh?.();
		} catch (cause) {
			if (current(version)) {
				noteError = message(cause);
				noteUncertain = uncertain(cause);
			}
		} finally {
			if (current(version)) noteBusy = false;
		}
	}
	async function loadTags(page = 1) {
		const version = generation,
			request = ++catalogGeneration;
		tagsLoading = true;
		tagsError = '';
		tagResult = null;
		try {
			const result = await memberProfileActions.tags(page, tagSearch);
			if (current(version) && request === catalogGeneration) {
				tagResult = result;
			}
		} catch (cause) {
			if (current(version) && request === catalogGeneration) tagsError = message(cause);
		} finally {
			if (current(version) && request === catalogGeneration) tagsLoading = false;
		}
	}
	async function reconcileTags() {
		const version = generation;
		const profile = await loadProfile(false);
		await loadTags();
		if (current(version) && profile && tagResult) {
			tagsUncertain = false;
			tagsError = 'Saved tags refreshed. Review current assignments before explicitly retrying.';
		}
	}
	function openTags() {
		modal = 'tags';
		void loadTags();
	}
	async function changeTag(tag: ProfileTag, remove = false) {
		if (tagsBusy || tagsUncertain) return;
		const id = String(memberId),
			version = generation;
		tagsBusy = true;
		tagsError = '';
		try {
			if (remove) await memberProfileActions.removeTag(id, tag.id);
			else await memberProfileActions.assignTag(id, tag.id);
			if (!current(version)) return;
			const profile = await loadProfile(false);
			if (!profile) return;
			if (profile.member.tag_records.some((t) => t.id === tag.id) === remove)
				throw new MemberProfileRequestError(
					'Tag receipt did not match the saved member. Refresh the profile before retrying.',
					200,
					{ outcome: 'unconfirmed', reconcile_history_before_retry: true }
				);
			actionMessage = remove ? `Tag ${tag.name} removed.` : `Tag ${tag.name} recorded.`;
			onRefresh?.();
		} catch (cause) {
			if (current(version)) {
				tagsError = message(cause);
				tagsUncertain = uncertain(cause);
			}
		} finally {
			if (current(version)) tagsBusy = false;
		}
	}
	async function createTag() {
		if (tagsBusy || tagsUncertain || !newTag.trim()) return;
		const version = generation;
		tagsBusy = true;
		tagsError = '';
		let created: ProfileTag;
		try {
			created = await memberProfileActions.createTag(newTag);
			if (!current(version)) return;
			newTag = '';
		} catch (cause) {
			if (current(version)) {
				tagsError = message(cause);
				tagsUncertain = uncertain(cause);
			}
			return;
		} finally {
			if (current(version)) tagsBusy = false;
		}
		if (current(version)) {
			await changeTag(created);
			await loadTags();
		}
	}
	async function loadPlans(page = 1) {
		const version = generation,
			request = ++catalogGeneration;
		plansLoading = true;
		plansError = '';
		try {
			const selected = selectedPlan ? memberNumericIdentity(selectedPlan) : undefined;
			const result = await membersApi.getServices({
				page,
				per_page: 25,
				search: planSearch,
				selected_id: selected
			});
			if (current(version) && request === catalogGeneration) planResult = result;
		} catch (cause) {
			if (current(version) && request === catalogGeneration) plansError = message(cause);
		} finally {
			if (current(version) && request === catalogGeneration) plansLoading = false;
		}
	}
	function openGrant() {
		modal = 'grant';
		actionError = '';
		selectedPlan = '';
		grantExpiry = '';
		void loadPlans();
	}
	function openExtend(sub: MemberMembership) {
		selectedMembership = sub;
		extendDays = 30;
		newExpiry = extendedRecordedExpiry(sub.expires_at, extendDays);
		actionError = '';
		showExtend = true;
	}
	function daysChanged(days: number) {
		extendDays = days;
		newExpiry = extendedRecordedExpiry(selectedMembership?.expires_at ?? null, days);
	}
	async function reloadAfterAction(id: string | number, version: number) {
		const saved = await loadProfile(false);
		if (!current(version) || !saved) return;
		await memberships.load(id);
		void activity.load(id);
		void timeline.load(id);
		onRefresh?.();
		return saved;
	}
	async function grant() {
		if (actionBusy || actionUncertain || !selectedPlan) return;
		const id = memberId,
			version = generation;
		actionBusy = true;
		actionError = '';
		try {
			const expiry = grantExpiry ? operationalExpiry(`${grantExpiry}T00:00:00`) : null;
			const receipt = await memberProfileActions.grant(String(id), selectedPlan, expiry);
			if (!current(version)) return;
			modal = '';
			actionMessage = `Membership #${receipt.id} granted; expiry is the recorded operational UTC value.`;
			await reloadAfterAction(id, version);
		} catch (cause) {
			if (current(version)) {
				actionError = message(cause);
				actionUncertain = uncertain(cause);
			}
		} finally {
			if (current(version)) actionBusy = false;
		}
	}
	async function extend() {
		if (actionBusy || actionUncertain || !selectedMembership) return;
		const id = memberId,
			version = generation,
			sub = selectedMembership;
		actionBusy = true;
		actionError = '';
		try {
			const receipt = await memberProfileActions.extend(
				String(id),
				sub.id,
				operationalExpiry(newExpiry)
			);
			if (!current(version)) return;
			showExtend = false;
			actionMessage = `Membership #${receipt.id} expiry saved.`;
			await reloadAfterAction(id, version);
		} catch (cause) {
			if (current(version)) {
				actionError = message(cause);
				actionUncertain = uncertain(cause);
			}
		} finally {
			if (current(version)) actionBusy = false;
		}
	}
	async function revoke() {
		if (actionBusy || actionUncertain || !pendingMembership) return;
		const id = memberId,
			version = generation,
			target = pendingMembership,
			oldTotal = memberships.result?.pagination.total;
		actionBusy = true;
		actionError = '';
		try {
			await memberProfileActions.revoke(target);
			if (!current(version)) return;
			const saved = await memberships.load(id, { page: 1 });
			if (
				!saved ||
				oldTotal === undefined ||
				saved.pagination.total !== oldTotal - 1 ||
				saved.memberships.some((row) => row.id === target)
			)
				throw new MemberProfileRequestError(
					'The revoke response needs a saved history review before retrying',
					200,
					{ outcome: 'unconfirmed', reconcile_history_before_retry: true }
				);
			modal = '';
			actionMessage = `Membership #${target} is absent from the updated history.`;
			await reloadAfterAction(id, version);
		} catch (cause) {
			if (current(version)) {
				actionError = message(cause);
				actionUncertain = uncertain(cause);
			}
		} finally {
			if (current(version)) actionBusy = false;
		}
	}
	async function reconcileActions() {
		const version = generation;
		const saved = await loadProfile(false);
		const history = await memberships.load(memberId);
		if (saved && history && current(version)) {
			actionUncertain = false;
			actionError = 'Saved state refreshed. Review it before explicitly retrying the action.';
		}
	}
	function openModeration(action: 'ban' | 'suspend' | 'unban') {
		modal = action;
		reason = '';
		duration = undefined;
		actionError = '';
	}
	async function moderate() {
		if (actionBusy || actionUncertain || !['ban', 'suspend', 'unban'].includes(modal)) return;
		const action = modal;
		if (action !== 'ban' && action !== 'suspend' && action !== 'unban') return;
		const id = memberId,
			version = generation;
		actionBusy = true;
		actionError = '';
		try {
			await memberProfileActions.moderate(String(id), action, reason, duration ?? null);
			if (!current(version)) return;
			const profile = await loadProfile(false);
			if (
				!profile ||
				profile.member.account.is_active !== (action === 'unban') ||
				profile.member.account.recorded_restriction !==
					(action === 'unban' ? 'active' : action === 'ban' ? 'banned' : 'suspended')
			)
				throw new MemberProfileRequestError(
					'The account response needs a saved profile review',
					200,
					{ outcome: 'unconfirmed', reconcile_history_before_retry: true }
				);
			modal = '';
			actionMessage = 'The recorded account status was confirmed by a fresh profile read.';
			void activity.load(id);
			void timeline.load(id);
			onRefresh?.();
		} catch (cause) {
			if (current(version)) {
				actionError = message(cause);
				actionUncertain = uncertain(cause);
			}
		} finally {
			if (current(version)) actionBusy = false;
		}
	}
	async function sendEmail() {
		if (actionBusy || actionUncertain || !emailSubject.trim() || !emailBody.trim()) return;
		const id = memberId,
			version = generation;
		actionBusy = true;
		actionError = '';
		try {
			await memberProfileRequest(
				`/api/admin/members/${memberNumericIdentity(id)}/send-email`,
				() => {
					throw new Error('A provider receipt is required to confirm this email');
				},
				{
					method: 'POST',
					body: { subject: emailSubject, body: emailBody, campaign_type: 'general' }
				}
			);
		} catch (cause) {
			if (current(version)) {
				actionError = message(cause);
				actionUncertain = uncertain(cause);
			}
		} finally {
			if (current(version)) actionBusy = false;
		}
	}
	function openEmail() {
		if (onEmail && core.result) onEmail(core.result.member);
		else {
			modal = 'email';
			actionError = '';
		}
	}
	function edit() {
		if (!core.result) return;
		if (onEdit) onEdit(core.result.member);
		else showEdit = true;
	}
	async function exportOrders() {
		if (actionBusy) return;
		const id = memberId,
			version = generation;
		actionBusy = true;
		actionError = '';
		try {
			const download = await downloadMemberOrders(id);
			if (!current(version)) return;
			const href = URL.createObjectURL(download.blob),
				anchor = document.createElement('a');
			anchor.href = href;
			anchor.download = download.filename;
			document.body.appendChild(anchor);
			anchor.click();
			anchor.remove();
			setTimeout(() => URL.revokeObjectURL(href), 1000);
			actionMessage = `Exported ${download.rows} recorded orders; SHA-256 ${download.sha256}.`;
		} catch (cause) {
			if (current(version)) actionError = message(cause);
		} finally {
			if (current(version)) actionBusy = false;
		}
	}
	function closeModal() {
		if (!actionBusy && !noteBusy && !tagsBusy) modal = '';
	}
</script>

<section
	class={['member-profile-view', { compact }]}
	aria-label="Member profile"
	{@attach profileAttachment}
>
	{#if core.loading}<p role="status">Loading recorded member profile…</p>{:else if core.error}<div
			role="alert"
		>
			<h2>Member profile unavailable</h2>
			<p>{core.error}</p>
			<button onclick={() => loadProfile()}>Retry member profile</button>
		</div>{:else if core.result}
		{@const profile = core.result}
		<MemberHeader
			member={profile.member}
			tagRecords={profile.member.tag_records}
			{tagsBusy}
			{tagsError}
			onRemoveTag={(id) => {
				const tag = profile.member.tag_records.find((t) => t.id === id);
				if (tag) void changeTag(tag, true);
			}}
			onOpenTagModal={openTags}
			onOpenNoteModal={() => (modal = 'note')}
			onOpenEmailModal={openEmail}
		/>
		<div class="account-actions">
			<button onclick={edit}>Edit Member</button><button
				disabled={actionBusy}
				onclick={() => openModeration('ban')}>Ban</button
			><button disabled={actionBusy} onclick={() => openModeration('suspend')}>Suspend</button
			><button disabled={actionBusy} onclick={() => openModeration('unban')}
				>Unban / Reactivate</button
			><button disabled={core.loading || actionBusy} onclick={() => loadProfile()}
				>Refresh profile</button
			>
		</div>
		<MemberStatsGrid {profile} />
		{#if actionMessage}<p role="status" class="receipt">{actionMessage}</p>{/if}
		{#if actionError}<p role="alert">{actionError}</p>{/if}
		{#if actionUncertain}<p>Outcome unconfirmed. Review saved state before another attempt.</p>
			<button
				disabled={actionBusy || core.loading || memberships.loading}
				onclick={reconcileActions}>Refresh saved state before retry</button
			>{/if}
		<nav class="profile-tabs" aria-label="Member profile sections">
			{#each tabs as tab (tab)}<button
					class={{ active: activeTab === tab }}
					aria-pressed={activeTab === tab}
					onclick={() => (activeTab = tab)}
					>{tab === 'overview'
						? 'Overview'
						: tab === 'subscriptions'
							? 'Subscriptions'
							: tab === 'orders'
								? 'Orders'
								: tab === 'emails'
									? 'Emails'
									: tab === 'notes'
										? 'Notes'
										: 'Activity'}</button
				>{/each}
		</nav>
		{#if activeTab === 'overview'}
			<section class="panel" aria-label="Recorded member details">
				<h3>Member Details</h3>
				<dl>
					<dt>Verified email</dt>
					<dd>{profile.member.email_verified ? 'Yes' : 'No'}</dd>
					<dt>Last recorded login</dt>
					<dd>{recordedWallClock(profile.member.last_login)}</dd>
					<dt>Joined</dt>
					<dd>{recordedWallClock(profile.member.joined_at)}</dd>
					<dt>Two-factor setting</dt>
					<dd>
						{profile.member.account.mfa_enabled === null
							? 'Not recorded'
							: profile.member.account.mfa_enabled
								? 'Enabled'
								: 'Disabled'}
					</dd>
				</dl>
				<p>{profile.timestamp_basis}</p>
				<p>{profile.billing_coverage}</p>
			</section>
			<section class="panel" aria-label="Recorded member timeline">
				<h3>Activity Timeline</h3>
				{#if timeline.result && !timeline.loading && !timeline.error}{#if timeline.result.data.length === 0}<p
						>
							No recorded timeline rows on this page.
						</p>{:else}<ol>
							{#each timeline.result.data as event (event.id)}<li>
									<strong>{event.title}</strong>
									<p>
										{event.timestamp_basis === 'recorded UTC instant'
											? recordedInstant(event.date)
											: recordedWallClock(event.date)}
									</p>
									<small>{event.type}</small>
								</li>{/each}
						</ol>{/if}
					<p>{timeline.result.coverage}</p>{/if}<HistoryControls
					label="timeline"
					pagination={timeline.result?.pagination ?? null}
					loading={timeline.loading}
					error={timeline.error}
					onPage={(page) => void timeline.load(memberId, { page })}
					onPerPage={(per_page) => void timeline.load(memberId, { page: 1, per_page })}
					onRefresh={() => void timeline.load(memberId)}
				/>
			</section>
		{:else if activeTab === 'subscriptions'}<SubscriptionsTab
				result={memberships.result}
				loading={memberships.loading}
				error={memberships.error}
				busy={actionBusy || actionUncertain}
				onPage={(page) => void memberships.load(memberId, { page })}
				onPerPage={(per_page) => void memberships.load(memberId, { page: 1, per_page })}
				onRefresh={() => void memberships.load(memberId)}
				onGrant={openGrant}
				onExtend={openExtend}
				onRevoke={(id) => {
					pendingMembership = id;
					modal = 'revoke';
					actionError = '';
				}}
			/>
		{:else if activeTab === 'orders'}<OrdersTab
				result={orders.result}
				loading={orders.loading}
				error={orders.error}
				busy={actionBusy}
				onPage={(page) => void orders.load(memberId, { page })}
				onPerPage={(per_page) => void orders.load(memberId, { page: 1, per_page })}
				onRefresh={() => void orders.load(memberId)}
				onView={(order) => {
					selectedOrder = order;
					modal = 'order';
				}}
				onExport={exportOrders}
			/>
		{:else if activeTab === 'emails'}<EmailsTab
				result={emails.result}
				loading={emails.loading}
				error={emails.error}
				busy={actionBusy}
				onPage={(page) => void emails.load(memberId, { page })}
				onPerPage={(per_page) => void emails.load(memberId, { page: 1, per_page })}
				onRefresh={() => void emails.load(memberId)}
				onSendEmail={openEmail}
			/>
		{:else if activeTab === 'notes'}<NotesTab
				result={notes.result}
				loading={notes.loading}
				error={notes.error}
				busy={noteBusy || noteUncertain}
				onPage={(page) => void notes.load(memberId, { page })}
				onPerPage={(per_page) => void notes.load(memberId, { page: 1, per_page })}
				onRefresh={refreshNotes}
				onAddNote={() => (modal = 'note')}
				onDeleteNote={(id) => {
					pendingNote = id;
					modal = 'delete-note';
				}}
			/>{#if noteError}<p role="alert">{noteError}</p>{/if}{#if noteUncertain}<button
					disabled={notes.loading || noteBusy}
					onclick={refreshNotes}>Refresh notes before retry</button
				>{/if}
		{:else if activeTab === 'activity'}<section class="panel" aria-label="Recorded member activity">
				<h3>Recorded Activity</h3>
				<form
					onsubmit={(event) => {
						event.preventDefault();
						void activity.load(memberId, { page: 1, action: activityFilter.trim() || undefined });
					}}
				>
					<label>Recorded action filter<input bind:value={activityFilter} maxlength="100" /></label
					><button disabled={activity.loading}>Filter activity</button>
				</form>
				{#if activity.result && !activity.loading && !activity.error}{#if activity.result.activity.length === 0}<p
						>
							No recorded activity rows on this page.
						</p>{:else}<ol>
							{#each activity.result.activity as event (event.id)}<li>
									<strong>{event.action}</strong>
									<p>{event.description ?? 'Description not recorded'}</p>
									<p>{recordedInstant(event.created_at)}</p>
								</li>{/each}
						</ol>{/if}
					<p>{activity.result.timestamp_basis}</p>{/if}<HistoryControls
					label="activity"
					pagination={activity.result?.pagination ?? null}
					loading={activity.loading}
					error={activity.error}
					onPage={(page) => void activity.load(memberId, { page })}
					onPerPage={(per_page) => void activity.load(memberId, { page: 1, per_page })}
					onRefresh={() => void activity.load(memberId)}
				/>
			</section>{/if}
	{/if}
</section>
{#if modal === 'note'}<MemberProfileDialog title="Add Note" busy={noteBusy} onClose={closeModal}
		><form
			onsubmit={(event) => {
				event.preventDefault();
				void saveNote();
			}}
		>
			<label
				>Note<textarea
					bind:value={noteDraft}
					rows="6"
					maxlength="10000"
					disabled={noteBusy || noteUncertain}
					required></textarea></label
			>{#if noteError}<p role="alert">{noteError}</p>{/if}{#if noteUncertain}<p>
					The draft is retained. Review saved notes before submitting again.
				</p>
				<button type="button" disabled={noteBusy || notes.loading} onclick={refreshNotes}
					>Refresh notes before retry</button
				>{/if}
			<footer>
				<button type="button" disabled={noteBusy} onclick={closeModal}>Cancel</button><button
					disabled={noteBusy || noteUncertain || !noteDraft.trim()}
					>{noteBusy ? 'Saving…' : 'Save Note'}</button
				>
			</footer>
		</form></MemberProfileDialog
	>
{:else if modal === 'delete-note'}<MemberProfileDialog
		title="Delete Note"
		busy={noteBusy}
		onClose={closeModal}
		><p>
			Delete saved note #{pendingNote}? This removes the recorded note and preserves the audit
			history.
		</p>
		{#if noteError}<p role="alert">{noteError}</p>{/if}
		<footer>
			<button disabled={noteBusy} onclick={closeModal}>Cancel</button><button
				disabled={noteBusy || noteUncertain}
				onclick={() => deleteNote(pendingNote)}>Delete Note</button
			>
		</footer>
		{#if noteUncertain}<button disabled={noteBusy || notes.loading} onclick={refreshNotes}
				>Refresh notes before retry</button
			>{/if}</MemberProfileDialog
	>
{:else if modal === 'tags'}<MemberProfileDialog
		title="Manage Tags"
		busy={tagsBusy}
		onClose={closeModal}
		><form
			onsubmit={(event) => {
				event.preventDefault();
				void loadTags();
			}}
		>
			<label>Search saved tags<input bind:value={tagSearch} maxlength="255" /></label><button
				disabled={tagsLoading || tagsBusy}>Search tags</button
			>
		</form>
		{#if tagsLoading}<p role="status">Loading saved tags…</p>{:else if tagsError}<p role="alert">
				{tagsError}
			</p>
			<button onclick={() => loadTags()}>Retry tags</button>{:else if tagResult}<div
				class="tag-options"
			>
				{#each tagResult.tags as tag (tag.id)}{@const assigned =
						core.result?.member.tag_records.some((t) => t.id === tag.id) ?? false}<button
						aria-pressed={assigned}
						disabled={tagsBusy || tagsUncertain}
						onclick={() => changeTag(tag, assigned)}
						>{assigned ? 'Remove' : 'Assign'} {tag.name}</button
					>{/each}
			</div>
			<nav aria-label="Saved tag pages">
				<button
					disabled={tagsBusy || tagResult.current_page <= 1}
					onclick={() => loadTags((tagResult?.current_page ?? 1) - 1)}>Previous tags</button
				><span
					>Page {tagResult.current_page} of {Math.max(1, tagResult.total_pages)} · {tagResult.total} saved
					tags</span
				><button
					disabled={tagsBusy || tagResult.current_page >= tagResult.total_pages}
					onclick={() => loadTags((tagResult?.current_page ?? 1) + 1)}>Next tags</button
				>
			</nav>{/if}
		<form
			onsubmit={(event) => {
				event.preventDefault();
				void createTag();
			}}
		>
			<label
				>New saved tag<input
					bind:value={newTag}
					maxlength="255"
					disabled={tagsBusy || tagsUncertain}
				/></label
			><button disabled={!newTag.trim() || tagsBusy || tagsUncertain}>Create and assign tag</button>
		</form>
		{#if tagsUncertain}<button onclick={reconcileTags}>Refresh saved tags before retry</button
			>{/if}</MemberProfileDialog
	>
{:else if modal === 'grant'}<MemberProfileDialog
		title="Grant Membership"
		busy={actionBusy}
		onClose={closeModal}
		><form
			onsubmit={(event) => {
				event.preventDefault();
				void loadPlans();
			}}
		>
			<label>Search membership plans<input bind:value={planSearch} /></label><button
				disabled={plansLoading}>Search plans</button
			>
		</form>
		{#if plansLoading}<p role="status">Loading plans…</p>{/if}{#if plansError}<p role="alert">
				{plansError}
			</p>
			<button onclick={() => loadPlans()}>Retry plans</button>{/if}{#if planResult}<label
				>Membership plan<select bind:value={selectedPlan} disabled={actionBusy}
					><option value="">Choose plan</option
					>{#if planResult.selected && !planResult.services.some((p) => String(p.id) === String(planResult?.selected?.id))}<option
							value={planResult.selected.id}>{planResult.selected.name}</option
						>{/if}{#each planResult.services as plan (plan.id)}<option value={plan.id}
							>{plan.name}</option
						>{/each}</select
				></label
			>
			<nav aria-label="Membership plan pages">
				<button
					disabled={actionBusy || planResult.pagination.current_page <= 1}
					onclick={() => loadPlans((planResult?.pagination.current_page ?? 1) - 1)}
					>Previous plans</button
				><span>Page {planResult.pagination.current_page} of {planResult.pagination.last_page}</span
				><button
					disabled={actionBusy ||
						planResult.pagination.current_page >= planResult.pagination.last_page}
					onclick={() => loadPlans((planResult?.pagination.current_page ?? 1) + 1)}
					>Next plans</button
				>
			</nav>{/if}<label
			>Expiration date (operational UTC, optional)<input
				type="date"
				bind:value={grantExpiry}
				disabled={actionBusy}
			/></label
		>
		<p>Leave empty for no recorded expiry. This grants access; it does not establish payment.</p>
		{#if actionError}<p role="alert">{actionError}</p>{/if}
		<footer>
			<button disabled={actionBusy} onclick={closeModal}>Cancel</button><button
				disabled={actionBusy || actionUncertain || !selectedPlan}
				onclick={grant}>Grant Membership</button
			>
		</footer></MemberProfileDialog
	>
{:else if modal === 'revoke'}<MemberProfileDialog
		title="Revoke Membership"
		busy={actionBusy}
		onClose={closeModal}
		><p>
			Remove membership #{pendingMembership} from this member? Existing order records remain separate.
		</p>
		{#if actionError}<p role="alert">{actionError}</p>{/if}
		<footer>
			<button disabled={actionBusy} onclick={closeModal}>Cancel</button><button
				disabled={actionBusy || actionUncertain}
				onclick={revoke}>Revoke Membership</button
			>
		</footer></MemberProfileDialog
	>
{:else if modal === 'ban' || modal === 'suspend' || modal === 'unban'}<MemberProfileDialog
		title={modal === 'ban'
			? 'Ban Member'
			: modal === 'suspend'
				? 'Suspend Member'
				: 'Reactivate Member'}
		busy={actionBusy}
		onClose={closeModal}
		><p>
			Change the recorded account restriction. Membership history is a separate source of facts.
		</p>
		{#if modal !== 'unban'}<label
				>Reason<textarea bind:value={reason} disabled={actionBusy}></textarea></label
			><label
				>Duration in days (optional)<input
					type="number"
					min="1"
					max="3650"
					bind:value={duration}
					disabled={actionBusy}
				/></label
			>{/if}{#if actionError}<p role="alert">{actionError}</p>{/if}
		<footer>
			<button disabled={actionBusy} onclick={closeModal}>Cancel</button><button
				disabled={actionBusy || actionUncertain}
				onclick={moderate}>Confirm account change</button
			>
		</footer></MemberProfileDialog
	>
{:else if modal === 'email'}<MemberProfileDialog
		title="Send Member Email"
		busy={actionBusy}
		onClose={closeModal}
		><p>To: {core.result?.member.email}</p>
		<form
			onsubmit={(event) => {
				event.preventDefault();
				void sendEmail();
			}}
		>
			<label
				>Subject<input
					bind:value={emailSubject}
					disabled={actionBusy || actionUncertain}
					required
				/></label
			><label
				>Message<textarea
					bind:value={emailBody}
					rows="8"
					disabled={actionBusy || actionUncertain}
					required></textarea></label
			>{#if actionError}<p role="alert">{actionError}</p>{/if}
			<footer>
				<button type="button" disabled={actionBusy} onclick={closeModal}>Cancel</button><button
					disabled={actionBusy || actionUncertain || !emailSubject.trim() || !emailBody.trim()}
					>Send Email</button
				>
			</footer>
		</form></MemberProfileDialog
	>
{:else if modal === 'order' && selectedOrder}<MemberProfileDialog
		title={`Recorded Order ${selectedOrder.order_number ?? selectedOrder.id}`}
		onClose={closeModal}
		><dl>
			<dt>Order identity</dt>
			<dd>{selectedOrder.id}</dd>
			<dt>Member identity</dt>
			<dd>{selectedOrder.user_id}</dd>
			<dt>Status</dt>
			<dd>{selectedOrder.status ?? 'Not recorded'}</dd>
			<dt>Total</dt>
			<dd>{recordedUnits(selectedOrder.total_units, selectedOrder.currency)}</dd>
			<dt>Recorded refund</dt>
			<dd>{recordedUnits(selectedOrder.refund_units, selectedOrder.currency)}</dd>
			<dt>Net recorded amount</dt>
			<dd>{recordedUnits(selectedOrder.net_recorded_units, selectedOrder.currency)}</dd>
			<dt>Recorded date</dt>
			<dd>{recordedWallClock(selectedOrder.created_at)}</dd>
			<dt>Recorded completed date</dt>
			<dd>{recordedWallClock(selectedOrder.completed_at)}</dd>
			<dt>Recorded refunded date</dt>
			<dd>{recordedWallClock(selectedOrder.refunded_at)}</dd>
		</dl></MemberProfileDialog
	>{/if}
<ExtendMembershipModal
	open={showExtend}
	subscription={selectedMembership}
	{newExpiry}
	onNewExpiryChange={(v) => (newExpiry = v)}
	error={actionError}
	{extendDays}
	extending={actionBusy || actionUncertain}
	onDaysChange={daysChanged}
	onClose={() => {
		if (!actionBusy) showExtend = false;
	}}
	onExtend={extend}
/>
{#if showEdit && editSeed}<MemberFormModal
		isOpen={showEdit}
		mode="edit"
		member={editSeed}
		onClose={() => (showEdit = false)}
		onSave={() => {
			showEdit = false;
			void loadProfile(false);
			onRefresh?.();
		}}
	/>{/if}

<style>
	.member-profile-view {
		color: var(--text-primary, #f0f6fc);
		min-width: 0;
	}
	.account-actions,
	.profile-tabs,
	.tag-options,
	nav,
	footer {
		display: flex;
		gap: 0.6rem;
		flex-wrap: wrap;
		align-items: center;
	}
	.account-actions {
		margin: -0.5rem 0 1.5rem;
	}
	.profile-tabs {
		border-bottom: 1px solid var(--border-default, #30363d);
		padding: 0.75rem 0;
		margin: 1rem 0;
	}
	.profile-tabs .active {
		background: var(--primary-500, #e6b800);
		color: #111;
	}
	.panel {
		border: 1px solid var(--border-default, #30363d);
		background: var(--bg-elevated, #161b22);
		border-radius: 14px;
		padding: 1rem;
		margin: 1rem 0;
		overflow-wrap: anywhere;
	}
	button,
	input,
	textarea,
	select {
		font: inherit;
		color: var(--text-primary, #f0f6fc);
		background: var(--bg-surface, #1c2128);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		padding: 0.65rem 0.85rem;
	}
	button {
		cursor: pointer;
	}
	button:disabled {
		opacity: 0.5;
		cursor: default;
	}
	input,
	textarea,
	select {
		width: 100%;
		box-sizing: border-box;
		min-width: 0;
	}
	label {
		display: block;
		margin: 0.75rem 0;
	}
	label input,
	label textarea,
	label select {
		display: block;
		margin-top: 0.4rem;
	}
	footer {
		justify-content: flex-end;
		margin-top: 1.5rem;
	}
	form + form {
		margin-top: 1.5rem;
	}
	p {
		line-height: 1.55;
		color: var(--text-secondary, #8b949e);
		overflow-wrap: anywhere;
	}
	[role='alert'] {
		color: var(--error-emphasis, #f85149);
	}
	.receipt {
		border-left: 3px solid #3fb950;
		padding-left: 0.75rem;
	}
	dt {
		color: var(--text-secondary, #8b949e);
		font-size: 0.85rem;
		margin-top: 0.75rem;
	}
	dd {
		margin: 0.25rem 0;
		overflow-wrap: anywhere;
	}
	li {
		margin-bottom: 1rem;
	}
	h3 {
		margin-top: 0;
	}
	.tag-options {
		margin: 1rem 0;
	}
	.compact {
		font-size: 0.95rem;
	}
	@media (max-width: 600px) {
		.profile-tabs button {
			flex: 1 1 28%;
		}
		.account-actions button {
			flex: 1 1 42%;
		}
	}
</style>
