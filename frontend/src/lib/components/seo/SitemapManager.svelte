<script lang="ts">
	import { onMount } from 'svelte';
	import { authors } from '#lib/authors/authors.js';
	import {
		getSitemapAdmin,
		saveSitemapSettings,
		generateSitemap,
		approveVideo,
		removeVideoPublication,
		getVideoTraders,
		saveVideoSource,
		SITEMAP_VIDEO_PLATFORMS,
		type SitemapKind,
		type SitemapAdmin,
		type SitemapSettings,
		type VideoCandidate
	} from '#lib/api/seo-sitemaps.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	import ConfirmationModal from '#lib/components/admin/ConfirmationModal.svelte';
	let { kind }: { kind: SitemapKind } = $props();
	const uid = $props.id();
	let saved = $state<SitemapAdmin | null>(null),
		settings = $state<SitemapSettings | null>(null);
	let loading = $state(true),
		busy = $state(false),
		error = $state(''),
		notice = $state('');
	let page = $state(1),
		generation = 0,
		disposed = false;
	let approval = $state<VideoCandidate | null>(null),
		authorSlug = $state('');
	let removal = $state<VideoCandidate | null>(null);
	let showSource = $state(false),
		sourceId = $state<string | undefined>(undefined),
		sourceDate = '';
	let traders = $state<Array<{ id: string; name: string }>>([]),
		traderLoading = $state(false),
		traderError = $state('');
	let draft = $state({
		title: '',
		description: '',
		video_url: '',
		video_platform: 'self-hosted',
		thumbnail_url: '',
		duration: undefined as number | undefined,
		date: '',
		trader_id: ''
	});
	const title = $derived(kind === 'news' ? 'News Sitemap' : 'Video Sitemap');
	function modal(node: HTMLDialogElement) {
		node.showModal();
		return () => node.close();
	}
	async function load(target = page) {
		const request = ++generation;
		loading = true;
		error = '';
		try {
			const actual = await getSitemapAdmin(kind, target);
			if (disposed || request !== generation) return;
			if (actual.preview.meta.current_page > actual.preview.meta.last_page) {
				await load(actual.preview.meta.last_page);
				return;
			}
			saved = actual;
			settings = structuredClone(actual.settings);
			page = actual.preview.meta.current_page;
		} catch (cause) {
			if (!disposed && request === generation)
				error = apiErrorMessage(cause, 'Saved sitemap sources are unavailable');
		} finally {
			if (!disposed && request === generation) loading = false;
		}
	}
	async function save(event: SubmitEvent) {
		event.preventDefault();
		if (busy || !saved || !settings) return;
		busy = true;
		error = '';
		notice = '';
		generation += 1;
		try {
			await saveSitemapSettings(kind, settings, saved.settings_version);
			notice = 'Sitemap settings saved. Public XML reads the current eligible published sources.';
			await load();
		} catch (cause) {
			error = apiErrorMessage(
				cause,
				'Settings save was not confirmed; reload saved state before retrying'
			);
		} finally {
			if (!disposed) busy = false;
		}
	}
	async function regenerate() {
		if (busy || !saved) return;
		busy = true;
		error = '';
		notice = '';
		try {
			const actual = await generateSitemap(kind);
			notice = `Generation recorded from ${actual.entry_count} eligible source records using saved settings version ${actual.settings_version}.`;
			await load();
		} catch (cause) {
			error = apiErrorMessage(cause, 'Sitemap generation was not confirmed');
		} finally {
			if (!disposed) busy = false;
		}
	}
	function openApproval(video: VideoCandidate) {
		approval = video;
		authorSlug = authors.find((author) => author.name === video.author_name)?.slug ?? '';
	}
	async function approve() {
		if (busy || !approval) return;
		busy = true;
		error = '';
		notice = '';
		try {
			await approveVideo(approval, authorSlug);
			approval = null;
			notice =
				'Public watch page approved from the saved video metadata. Later metadata changes require review before inclusion.';
			await load();
		} catch (cause) {
			error = apiErrorMessage(cause, 'Public video approval was not confirmed');
		} finally {
			if (!disposed) busy = false;
		}
	}
	async function remove() {
		if (busy || !removal) return;
		busy = true;
		error = '';
		notice = '';
		try {
			await removeVideoPublication(removal.id);
			removal = null;
			notice =
				'Public watch page and sitemap listing removed. The source video remains in video management.';
			await load();
		} catch (cause) {
			error = apiErrorMessage(cause, 'Video publication removal was not confirmed');
		} finally {
			if (!disposed) busy = false;
		}
	}
	async function loadTraders() {
		traderLoading = true;
		traderError = '';
		try {
			const actual = await getVideoTraders();
			if (!disposed) traders = actual;
		} catch (cause) {
			if (!disposed) traderError = apiErrorMessage(cause, 'Saved video authors are unavailable');
		} finally {
			if (!disposed) traderLoading = false;
		}
	}
	function editSource(video?: VideoCandidate) {
		sourceId = video?.id;
		sourceDate = video?.published_at ?? '';
		draft = {
			title: video?.title ?? '',
			description: video?.description ?? '',
			video_url: video?.video_url ?? '',
			video_platform: video ? (video.video_platform ?? '') : 'self-hosted',
			thumbnail_url: video?.thumbnail_url ?? '',
			duration: video?.duration ?? undefined,
			date: sourceDate.slice(0, 19),
			trader_id: video?.trader_id ?? ''
		};
		showSource = true;
		error = '';
		void loadTraders();
	}
	async function saveSource(event: SubmitEvent) {
		event.preventDefault();
		if (busy || traderLoading || traderError) return;
		busy = true;
		error = '';
		notice = '';
		try {
			if (draft.duration === undefined) throw new Error('Provide the actual video duration');
			const published =
				sourceDate && draft.date === sourceDate.slice(0, 19)
					? sourceDate
					: new Date(`${draft.date}Z`).toISOString();
			const source = await saveVideoSource(
				{
					title: draft.title.trim(),
					description: draft.description.trim(),
					video_url: draft.video_url.trim(),
					video_platform: draft.video_platform,
					thumbnail_url: draft.thumbnail_url.trim(),
					duration: draft.duration,
					published_at: published,
					trader_id: draft.trader_id
				},
				sourceId
			);
			showSource = false;
			notice = 'Source video metadata saved. Review and explicitly approve its public watch page.';
			await load(1);
			openApproval(source);
		} catch (cause) {
			error = apiErrorMessage(
				cause,
				'Source save was not confirmed; refresh sources before retrying'
			);
		} finally {
			if (!disposed) busy = false;
		}
	}
	function formatDate(value: string | null) {
		return value
			? `${new Date(value).toLocaleString('en-US', { timeZone: 'UTC' })} UTC`
			: 'Unavailable';
	}
	onMount(() => {
		void load();
		return () => {
			disposed = true;
			generation += 1;
		};
	});
</script>

<section class="sitemap-manager">
	<header>
		<div>
			<h1>{title}</h1>
			<p>Saved source facts, publication eligibility and current XML output.</p>
		</div>
		<div class="actions">
			<a href="/{kind}-sitemap.xml" target="_blank" rel="noopener noreferrer">View Sitemap</a
			><button type="button" disabled={loading || busy || !saved} onclick={() => void regenerate()}
				>Regenerate</button
			><button type="button" disabled={loading || busy} onclick={() => void load()}
				>Refresh saved state</button
			>{#if kind === 'video'}<button type="button" disabled={busy} onclick={() => editSource()}
					>Add Video</button
				>{/if}
		</div>
	</header>
	{#if error}<p role="alert">{error}</p>{/if}{#if notice}<p role="status">{notice}</p>{/if}
	{#if loading}<p role="status">Loading saved sitemap sources…</p>{:else if saved && settings}
		<form onsubmit={(event) => void save(event)}>
			<fieldset disabled={busy}>
				<legend>Sitemap Settings · saved version {saved.settings_version}</legend>
				<label class="check"
					><input type="checkbox" bind:checked={settings.enabled} /> Enable {kind === 'news'
						? 'News'
						: 'Video'} Sitemap</label
				>
				{#if 'publication_name' in settings}
					<label for="{uid}-name">Publication Name</label><input
						id="{uid}-name"
						bind:value={settings.publication_name}
						required
						maxlength={200}
					/>
					<label for="{uid}-language">Language Code</label><input
						id="{uid}-language"
						bind:value={settings.language}
						required
						maxlength={5}
					/>
					<label for="{uid}-age">Max Article Age (hours)</label><input
						id="{uid}-age"
						type="number"
						bind:value={settings.max_age_hours}
						min={1}
						max={48}
						required
					/>
					<label for="{uid}-exclude">Excluded category slugs (comma separated)</label><input
						id="{uid}-exclude"
						value={settings.exclude_categories.join(', ')}
						onchange={(event) => {
							if (settings && 'exclude_categories' in settings)
								settings.exclude_categories = event.currentTarget.value
									.split(',')
									.map((value) => value.trim())
									.filter(Boolean);
						}}
					/>
					<p>
						Only actual published, indexable posts with a matching real author profile and original
						publication date can appear. Their original dates are never reset by generation.
					</p>
				{:else}
					<label class="check"
						><input type="checkbox" checked={settings.include_thumbnails} disabled /> Include video thumbnails
						(required by Google)</label
					>
					<fieldset>
						<legend>Supported platforms included in the sitemap</legend
						>{#each SITEMAP_VIDEO_PLATFORMS as platform (platform)}<label class="check"
								><input
									type="checkbox"
									checked={settings.platforms.includes(platform)}
									onchange={(event) => {
										if (settings && 'platforms' in settings)
											settings.platforms = event.currentTarget.checked
												? [...settings.platforms, platform]
												: settings.platforms.filter((value) => value !== platform);
									}}
								/>{platform}</label
							>{/each}
					</fieldset>
					<label class="check"
						><input type="checkbox" bind:checked={settings.include_uploader} /> Include actual uploader
						information</label
					><label class="check"
						><input type="checkbox" bind:checked={settings.detect_from_content} /> Find saved learning-center
						videos for publication review</label
					>
					<p>
						Thumbnails are required. Only explicitly approved public watch pages can appear.
						Room-assigned and private dashboard videos remain excluded. Turning the sitemap off does
						not revoke an approved public watch page.
					</p>
				{/if}<button type="submit">Save Settings</button>
			</fieldset>
		</form>
		<section class="generation" aria-label="Recorded generation">
			<h2>Recorded generation</h2>
			{#if saved.last_generation}<p>
					<time datetime={saved.last_generation.created_at}
						>{formatDate(saved.last_generation.created_at)}</time
					>
					· {saved.last_generation.entry_count} actual entries · saved settings version {saved
						.last_generation.settings_version}
				</p>{:else}<p>No generation has been recorded.</p>{/if}
			<p>
				Regenerate reads current eligible records and saves an audit snapshot. Public XML is
				generated from current source records, including expiration and publication changes.
			</p>
		</section>
		<section aria-label="Source eligibility">
			<h2>Source eligibility</h2>
			<p>
				{saved.preview.meta.total} saved source records · Page {page} of {saved.preview.meta
					.last_page}. {settings.enabled
					? 'Sitemap enabled.'
					: 'Sitemap disabled; XML intentionally contains no entries.'}
			</p>
			{#if !saved.preview.data.length}<p>No saved source records match this view.</p>{:else}<div
					class="table-scroll"
				>
					<table>
						<thead
							><tr
								><th>Source</th><th>Actual publication</th><th>Eligibility</th><th>Actions</th></tr
							></thead
						><tbody
							>{#each saved.preview.data as entry (entry.record.id)}<tr
									><td>{entry.record.title}<br /><code>{entry.record.path}</code></td><td
										>{formatDate(entry.record.published_at)}<br />{entry.record.author_name ??
											'Author unavailable'}</td
									><td>{entry.eligible ? 'Eligible' : entry.exclusion_reason}</td><td
										><div class="actions">
											{#if entry.eligible}<a
													href={entry.record.path}
													target="_blank"
													rel="noopener noreferrer">View public page</a
												>{/if}{#if kind === 'news'}<a href="/admin/blog/edit/{entry.record.id}"
													>Edit source article</a
												>{:else if 'source_hash' in entry.record}<button
													type="button"
													disabled={busy}
													onclick={() => openApproval(entry.record as VideoCandidate)}
													>Review publication</button
												><button
													type="button"
													disabled={busy}
													onclick={() => editSource(entry.record as VideoCandidate)}
													>Edit metadata</button
												>{#if entry.record.mapping_id}<button
														type="button"
														disabled={busy}
														onclick={() => {
															removal = entry.record as VideoCandidate;
														}}>Remove public listing</button
													>{/if}{/if}
										</div></td
									></tr
								>{/each}</tbody
						>
					</table>
				</div>{/if}
			<nav aria-label="Sitemap source pages">
				<button type="button" disabled={busy || page <= 1} onclick={() => void load(page - 1)}
					>Previous page</button
				><button
					type="button"
					disabled={busy || page >= saved.preview.meta.last_page}
					onclick={() => void load(page + 1)}>Next page</button
				>
			</nav>
		</section>
	{/if}
</section>

{#if approval}<dialog
		{@attach modal}
		aria-labelledby="{uid}-approval-title"
		oncancel={(event) => {
			event.preventDefault();
			if (!busy) approval = null;
		}}
	>
		<h2 id="{uid}-approval-title">Approve public watch page</h2>
		<p>{approval.title}</p>
		<p>
			This approval makes the saved learning-center video available on a public, indexable page at <code
				>{approval.path}</code
			>. Actual title, description, date, author, duration and media must be complete. Room-assigned
			videos cannot be approved.
		</p>
		<label for="{uid}-author">Actual public author profile</label><select
			id="{uid}-author"
			bind:value={authorSlug}
			disabled={busy}
			><option value="">Choose the matching author</option
			>{#each authors as author (author.slug)}<option value={author.slug}>{author.name}</option
				>{/each}</select
		>
		<p>
			Saved video author: {approval.author_name ?? 'Unavailable'}. Approval never changes the source
			author.
		</p>
		{#if error}<p role="alert">{error}</p>{/if}
		<div class="actions">
			<button type="button" disabled={busy || !authorSlug} onclick={() => void approve()}
				>Approve public watch page</button
			><button
				type="button"
				disabled={busy}
				onclick={() => {
					approval = null;
				}}>Cancel</button
			>
		</div>
	</dialog>{/if}
{#if showSource}<dialog
		{@attach modal}
		aria-labelledby="{uid}-source-title"
		oncancel={(event) => {
			event.preventDefault();
			if (!busy) showSource = false;
		}}
	>
		<h2 id="{uid}-source-title">
			{sourceId ? 'Edit saved video metadata' : 'Add public video source'}
		</h2>
		<p>
			Enter actual metadata from the video. Saving does not guess thumbnails, dates, duration or
			authors. Public page approval is a separate explicit action.
		</p>
		{#if error}<p role="alert">{error}</p>{/if}{#if traderError}<p role="alert">{traderError}</p>
			<button type="button" onclick={() => void loadTraders()}>Retry saved authors</button>{/if}
		<form onsubmit={(event) => void saveSource(event)}>
			<fieldset disabled={busy || traderLoading || !!traderError}>
				<label for="{uid}-video-title">Video title</label><input
					id="{uid}-video-title"
					bind:value={draft.title}
					required
					maxlength={100}
				/><label for="{uid}-description">Video description</label><textarea
					id="{uid}-description"
					bind:value={draft.description}
					required
					minlength={50}
					maxlength={2048}
					rows={4}></textarea>
				<p>
					Use an actual public video file on the configured Bunny/R2 media hosts, or a supported
					YouTube, Vimeo, Bunny, Dailymotion, TED or Wistia player URL. Unsupported hosts remain
					editable but cannot be approved.
				</p>
				<label for="{uid}-url">Public video or player URL</label><input
					id="{uid}-url"
					type="url"
					bind:value={draft.video_url}
					required
				/><label for="{uid}-platform">Video platform</label><select
					id="{uid}-platform"
					bind:value={draft.video_platform}
					required
					><option value="">Choose platform</option
					>{#each SITEMAP_VIDEO_PLATFORMS as platform (platform)}<option value={platform}
							>{platform}</option
						>{/each}</select
				><label for="{uid}-thumbnail">Actual thumbnail URL</label><input
					id="{uid}-thumbnail"
					type="url"
					bind:value={draft.thumbnail_url}
					required
				/><label for="{uid}-duration">Actual duration (seconds)</label><input
					id="{uid}-duration"
					type="number"
					bind:value={draft.duration}
					min={1}
					max={28800}
					step={1}
					required
				/><label for="{uid}-date">Original publication time (UTC)</label><input
					id="{uid}-date"
					type="datetime-local"
					step={1}
					bind:value={draft.date}
					required
				/><label for="{uid}-trader">Saved video author</label><select
					id="{uid}-trader"
					bind:value={draft.trader_id}
					required
					><option value="">Choose the actual saved author</option
					>{#each traders as trader (trader.id)}<option value={trader.id}>{trader.name}</option
						>{/each}</select
				><button type="submit">Save source video</button>
			</fieldset>
			<button
				type="button"
				disabled={busy}
				onclick={() => {
					showSource = false;
				}}>Cancel</button
			>
		</form>
	</dialog>{/if}
<ConfirmationModal
	isOpen={removal !== null}
	title="Remove Public Video Listing"
	message="Remove this approved public watch page and its sitemap entry? The source video remains in video management."
	confirmText="Remove listing"
	isLoading={busy}
	variant="danger"
	onConfirm={() => void remove()}
	onCancel={() => {
		if (!busy) removal = null;
	}}
/>

<style>
	.sitemap-manager {
		padding: 2rem;
		max-width: 1200px;
		margin: 0 auto;
		color: var(--text-primary, #e2e8f0);
	}
	header,
	.actions,
	nav {
		display: flex;
		gap: 0.75rem;
		flex-wrap: wrap;
		align-items: center;
		justify-content: space-between;
	}
	fieldset,
	.generation {
		display: grid;
		gap: 0.75rem;
		margin: 1.5rem 0;
		padding: 1.25rem;
		border: 1px solid #475569;
		border-radius: 0.75rem;
	}
	input,
	textarea,
	select,
	button {
		padding: 0.65rem;
		border-radius: 0.4rem;
		border: 1px solid #475569;
		background: #111827;
		color: inherit;
		font: inherit;
	}
	button {
		cursor: pointer;
	}
	button:disabled {
		cursor: wait;
		opacity: 0.6;
	}
	.check {
		display: flex;
		gap: 0.6rem;
		align-items: center;
	}
	.table-scroll {
		overflow-x: auto;
	}
	table {
		width: 100%;
		border-collapse: collapse;
	}
	th,
	td {
		padding: 0.8rem;
		text-align: left;
		border-bottom: 1px solid #475569;
		vertical-align: top;
	}
	code {
		overflow-wrap: anywhere;
	}
	[role='alert'] {
		color: #fca5a5;
	}
	[role='status'] {
		color: #a7f3d0;
	}
	dialog {
		color: #e2e8f0;
		background: #111827;
		border: 1px solid #475569;
		border-radius: 0.75rem;
		padding: 1.5rem;
		max-width: min(720px, calc(100vw - 3rem));
		max-height: calc(100vh - 3rem);
		overflow: auto;
	}
	dialog::backdrop {
		background: rgb(0 0 0 / 65%);
	}
	@media (max-width: 640px) {
		.sitemap-manager {
			padding: 1rem;
		}
	}
</style>
