<!--
	URL: /crm/contacts/[id]
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { crmAPI } from '$lib/api/crm';
	import type { Contact, TimelineEvent } from '$lib/crm/types';
	import { IconArrowLeft, IconMail, IconPhone, IconActivity, IconUserCircle } from '$lib/icons';

	let contact = $state<Contact | null>(null);
	let timeline = $state<TimelineEvent[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);

	let contactId = $derived(page.params.id as string);

	onMount(async () => {
		await loadContact();
	});

	async function loadContact() {
		loading = true;
		error = null;
		try {
			contact = await crmAPI.getContact(contactId);
			timeline = await crmAPI.getContactTimeline(contactId);
		} catch (e) {
			console.error('Failed to load contact', e);
			error = 'Failed to load contact. Please try again.';
		} finally {
			loading = false;
		}
	}

	function goBack() {
		goto('/crm/contacts');
	}
</script>

<svelte:head>
	<title>{contact ? `${contact.full_name} | CRM Contact` : 'Contact | CRM'}</title>
</svelte:head>

<div class="contact-detail-page">
	<div class="contact-detail-shell">
		<button class="back-button" onclick={goBack}>
			<IconArrowLeft size={16} />
			Back to contacts
		</button>

		{#if loading && !contact}
			<div class="center-state">Loading contact…</div>
		{:else if error}
			<div class="error-panel">
				{error}
			</div>
		{:else if contact}
			<div class="detail-layout">
				<!-- Left: Profile & Timeline -->
				<div class="primary-column">
					<!-- Profile Card -->
					<div class="panel profile-panel">
						<div class="profile-header">
							<div class="avatar-icon">
								<IconUserCircle size={32} />
							</div>
							<div class="profile-copy">
								<h1 class="contact-name">{contact.full_name}</h1>
								<p class="contact-title">{contact.job_title}</p>
								<div class="profile-badges">
									<span class="profile-badge">{contact.status}</span>
									<span class="profile-badge">{contact.lifecycle_stage}</span>
									{#if contact.subscription_status !== 'none'}
										<span class="profile-badge profile-badge--revenue">
											{contact.subscription_status} · ${contact.subscription_mrr.toFixed(0)} MRR
										</span>
									{/if}
								</div>
							</div>
						</div>

						<div class="contact-methods">
							<div class="contact-method">
								<span class="method-icon">
									<IconMail size={14} />
								</span>
								<a href={`mailto:${contact.email || ''}`} class="method-link"
									>{contact.email || ''}</a
								>
							</div>
							{#if contact.phone}
								<div class="contact-method">
									<span class="method-icon">
										<IconPhone size={14} />
									</span>
									<a href={`tel:${contact.phone}`} class="method-link">{contact.phone}</a>
								</div>
							{/if}
						</div>

						<div class="score-grid">
							<div class="score-block">
								<p class="metric-label">Lead Score</p>
								<p class="metric-value">{contact.lead_score}</p>
							</div>
							<div class="score-block">
								<p class="metric-label">Health</p>
								<p class="metric-value">{contact.health_score}</p>
							</div>
							<div class="score-block">
								<p class="metric-label">Engagement</p>
								<p class="metric-value">{contact.engagement_score}</p>
							</div>
						</div>
					</div>

					<!-- Timeline -->
					<div class="panel timeline-panel">
						<div class="section-heading">
							<span class="section-icon">
								<IconActivity size={18} />
							</span>
							Activity Timeline
						</div>

						{#if !timeline.length}
							<p class="empty-message">No timeline events yet.</p>
						{:else}
							<ol class="timeline-list">
								{#each timeline as event (event.id)}
									<li class="timeline-item">
										<div class="timeline-dot"></div>
										<div class="timeline-content">
											<div class="timeline-header">
												<p class="timeline-title">{event.title}</p>
												<span class="timeline-date">
													{new Date(event.occurred_at).toLocaleString()}
												</span>
											</div>
											{#if event.description}
												<p class="timeline-description">{event.description}</p>
											{/if}
											{#if event.created_by}
												<p class="timeline-meta">by {event.created_by.name}</p>
											{/if}
										</div>
									</li>
								{/each}
							</ol>
						{/if}
					</div>
				</div>

				<!-- Right: Metrics -->
				<div class="side-column">
					<div class="panel side-panel">
						<p class="metric-label">Email Engagement</p>
						<div class="metric-grid metric-grid--two">
							<div>
								<p class="submetric-label">Opens</p>
								<p class="metric-value">{contact.email_opens}</p>
							</div>
							<div>
								<p class="submetric-label">Clicks</p>
								<p class="metric-value">{contact.email_clicks}</p>
							</div>
						</div>
					</div>

					<div class="panel side-panel">
						<p class="metric-label">Behavior Signals</p>
						<div class="metric-list">
							<p>Avg Engagement Score: {contact.avg_engagement_score.toFixed(1)}</p>
							<p>Avg Intent Score: {contact.avg_intent_score.toFixed(1)}</p>
							<p>Friction Events: {contact.friction_events_count}</p>
							<p>Total Sessions: {contact.total_sessions}</p>
						</div>
					</div>

					<div class="panel side-panel">
						<p class="metric-label">Value & Revenue</p>
						<div class="metric-list">
							<p>MRR: ${contact.subscription_mrr.toFixed(0)}</p>
							<p>LTV: ${contact.lifetime_value.toFixed(0)}</p>
							<p>Deals: {contact.deals_count}</p>
						</div>
					</div>
				</div>
			</div>
		{:else}
			<div class="center-state">Contact not found.</div>
		{/if}
	</div>
</div>

<style>
	.contact-detail-page {
		min-height: 100%;
		background: rgba(2, 6, 23, 0.95);
		color: #f8fafc;
	}

	.contact-detail-shell {
		width: min(100%, 72rem);
		margin: 0 auto;
		padding: 1.5rem;
	}

	.back-button,
	.center-state,
	.profile-header,
	.avatar-icon,
	.profile-badges,
	.contact-method,
	.section-heading,
	.section-icon,
	.timeline-item,
	.timeline-header {
		display: flex;
	}

	.back-button {
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 1rem;
		border: 0;
		background: transparent;
		color: #94a3b8;
		padding: 0;
		font-size: 0.75rem;
		transition: color 0.2s ease;
	}

	.back-button:hover,
	.back-button:focus-visible {
		color: #e2e8f0;
	}

	.center-state {
		height: 16rem;
		align-items: center;
		justify-content: center;
		color: #94a3b8;
	}

	.error-panel {
		border: 1px solid rgba(190, 18, 60, 0.6);
		border-radius: 0.75rem;
		background: rgba(76, 5, 25, 0.4);
		color: #fecdd3;
		padding: 0.75rem 1rem;
		font-size: 0.875rem;
	}

	.detail-layout {
		display: grid;
		gap: 1.5rem;
	}

	.primary-column,
	.side-column {
		display: grid;
		align-content: start;
		gap: 1rem;
	}

	.panel {
		border: 1px solid rgba(30, 41, 59, 0.8);
		border-radius: 1rem;
		background: rgba(15, 23, 42, 0.7);
	}

	.profile-panel,
	.timeline-panel {
		padding: 1.25rem;
	}

	.profile-header {
		align-items: flex-start;
		gap: 1rem;
	}

	.avatar-icon {
		width: 3.5rem;
		height: 3.5rem;
		flex: 0 0 auto;
		align-items: center;
		justify-content: center;
		border-radius: 999px;
		background: rgba(30, 41, 59, 0.8);
		color: #a5b4fc;
	}

	.profile-copy {
		flex: 1;
		min-width: 0;
	}

	.contact-name {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 600;
		letter-spacing: 0;
		line-height: 1.2;
	}

	.contact-title {
		margin: 0.25rem 0 0;
		color: #94a3b8;
		font-size: 0.75rem;
	}

	.profile-badges {
		flex-wrap: wrap;
		align-items: center;
		gap: 0.75rem;
		margin-top: 0.5rem;
		color: #cbd5e1;
		font-size: 0.75rem;
	}

	.profile-badge {
		border-radius: 999px;
		background: rgba(30, 41, 59, 0.8);
		padding: 0.25rem 0.5rem;
	}

	.profile-badge--revenue {
		background: rgba(16, 185, 129, 0.1);
		color: #6ee7b7;
	}

	.contact-methods,
	.score-grid,
	.metric-grid {
		display: grid;
		gap: 0.75rem;
	}

	.contact-methods,
	.score-grid {
		margin-top: 1rem;
		color: #cbd5e1;
		font-size: 0.75rem;
	}

	.contact-method {
		align-items: center;
		gap: 0.5rem;
		min-width: 0;
	}

	.method-icon {
		display: inline-flex;
		flex: 0 0 auto;
		color: #94a3b8;
	}

	.method-link {
		overflow: hidden;
		color: inherit;
		text-overflow: ellipsis;
		white-space: nowrap;
		text-decoration: none;
	}

	.method-link:hover,
	.method-link:focus-visible {
		text-decoration: underline;
	}

	.score-grid {
		color: #94a3b8;
	}

	.metric-label {
		margin: 0;
		color: #64748b;
		font-size: 0.6875rem;
		letter-spacing: 0.04em;
		text-transform: uppercase;
	}

	.metric-value {
		margin: 0.25rem 0 0;
		color: #f8fafc;
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.2;
	}

	.section-heading {
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
		color: #f1f5f9;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.section-icon {
		color: #38bdf8;
	}

	.empty-message {
		margin: 0;
		padding: 2rem 0;
		color: #64748b;
		font-size: 0.75rem;
		text-align: center;
	}

	.timeline-list {
		display: grid;
		gap: 0.75rem;
		margin: 0;
		padding: 0;
		list-style: none;
		font-size: 0.75rem;
	}

	.timeline-item {
		gap: 0.75rem;
	}

	.timeline-dot {
		width: 0.5rem;
		height: 0.5rem;
		flex: 0 0 auto;
		margin-top: 0.25rem;
		border-radius: 999px;
		background: #38bdf8;
	}

	.timeline-content {
		flex: 1;
		min-width: 0;
		border-left: 1px solid #1e293b;
		padding-left: 0.75rem;
	}

	.timeline-header {
		align-items: center;
		justify-content: space-between;
		gap: 0.5rem;
	}

	.timeline-title {
		margin: 0;
		color: #f1f5f9;
		font-size: 0.8125rem;
		font-weight: 500;
	}

	.timeline-date,
	.timeline-meta {
		color: #64748b;
		font-size: 0.6875rem;
	}

	.timeline-date {
		white-space: nowrap;
	}

	.timeline-description,
	.timeline-meta {
		margin: 0.25rem 0 0;
	}

	.timeline-description {
		color: #cbd5e1;
		font-size: 0.6875rem;
	}

	.side-panel {
		padding: 1rem;
		font-size: 0.75rem;
	}

	.metric-grid,
	.metric-list {
		margin-top: 0.75rem;
	}

	.metric-grid--two {
		grid-template-columns: repeat(2, minmax(0, 1fr));
	}

	.submetric-label {
		margin: 0;
		color: #94a3b8;
		font-size: 0.6875rem;
	}

	.metric-list {
		display: grid;
		gap: 0.5rem;
		color: #cbd5e1;
		font-size: 0.6875rem;
	}

	.metric-list p {
		margin: 0;
	}

	@media (min-width: 768px) {
		.detail-layout {
			grid-template-columns: minmax(0, 2fr) minmax(0, 1.5fr);
		}

		.contact-methods {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}

		.score-grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}

	@media (max-width: 640px) {
		.contact-detail-shell {
			padding-right: 1rem;
			padding-left: 1rem;
		}

		.profile-header,
		.timeline-header {
			align-items: flex-start;
			flex-direction: column;
		}
	}
</style>
