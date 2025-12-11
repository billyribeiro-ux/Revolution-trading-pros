<!--
/**
 * TrainerCard Component - Learning Center
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Displays trainer/instructor information with photo, bio, and social links.
 *
 * @version 1.0.0 (December 2025)
 */
-->

<script lang="ts">
	import type { Trainer } from '$lib/types/learning-center';

	interface Props {
		trainer: Trainer;
		variant?: 'full' | 'compact' | 'inline';
		showBio?: boolean;
		showSpecialties?: boolean;
		showSocialLinks?: boolean;
		lessonCount?: number;
	}

	let {
		trainer,
		variant = 'full',
		showBio = true,
		showSpecialties = true,
		showSocialLinks = true,
		lessonCount
	}: Props = $props();
</script>

{#if variant === 'inline'}
	<!-- Inline variant - used in lesson headers -->
	<div class="trainer-inline">
		{#if trainer.thumbnailUrl || trainer.imageUrl}
			<img
				src={trainer.thumbnailUrl || trainer.imageUrl}
				alt={trainer.name}
				class="trainer-inline-avatar"
			/>
		{:else}
			<div class="trainer-inline-avatar placeholder">
				{trainer.firstName.charAt(0)}{trainer.lastName.charAt(0)}
			</div>
		{/if}
		<div class="trainer-inline-info">
			<span class="trainer-inline-name">{trainer.name}</span>
			<span class="trainer-inline-title">{trainer.title}</span>
		</div>
	</div>
{:else if variant === 'compact'}
	<!-- Compact variant - used in sidebars/grids -->
	<div class="trainer-compact">
		<div class="trainer-compact-avatar-wrap">
			{#if trainer.thumbnailUrl || trainer.imageUrl}
				<img
					src={trainer.thumbnailUrl || trainer.imageUrl}
					alt={trainer.name}
					class="trainer-compact-avatar"
				/>
			{:else}
				<div class="trainer-compact-avatar placeholder">
					{trainer.firstName.charAt(0)}{trainer.lastName.charAt(0)}
				</div>
			{/if}
		</div>
		<div class="trainer-compact-content">
			<h4 class="trainer-compact-name">{trainer.name}</h4>
			<p class="trainer-compact-title">{trainer.title}</p>
			{#if lessonCount !== undefined}
				<span class="trainer-compact-lessons">{lessonCount} lessons</span>
			{/if}
		</div>
	</div>
{:else}
	<!-- Full variant - detailed trainer card -->
	<div class="trainer-card">
		<div class="trainer-header">
			<div class="trainer-avatar-container">
				{#if trainer.imageUrl}
					<img
						src={trainer.imageUrl}
						alt={trainer.name}
						class="trainer-avatar"
					/>
				{:else}
					<div class="trainer-avatar placeholder">
						{trainer.firstName.charAt(0)}{trainer.lastName.charAt(0)}
					</div>
				{/if}
			</div>
			<div class="trainer-meta">
				<h3 class="trainer-name">{trainer.name}</h3>
				<p class="trainer-title">{trainer.title}</p>
				{#if lessonCount !== undefined}
					<span class="lesson-count">{lessonCount} lessons</span>
				{/if}
			</div>
		</div>

		{#if showBio && trainer.bio}
			<div class="trainer-bio">
				<p>{trainer.shortBio || trainer.bio}</p>
			</div>
		{/if}

		{#if showSpecialties && trainer.specialties?.length > 0}
			<div class="trainer-specialties">
				<h4>Specialties</h4>
				<div class="specialty-tags">
					{#each trainer.specialties as specialty}
						<span class="specialty-tag">{specialty}</span>
					{/each}
				</div>
			</div>
		{/if}

		{#if showSocialLinks && trainer.socialLinks}
			<div class="trainer-social">
				{#if trainer.socialLinks.twitter}
					<a
						href={trainer.socialLinks.twitter}
						target="_blank"
						rel="noopener noreferrer"
						class="social-link twitter"
						aria-label="Twitter"
					>
						<svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
							<path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z" />
						</svg>
					</a>
				{/if}
				{#if trainer.socialLinks.linkedin}
					<a
						href={trainer.socialLinks.linkedin}
						target="_blank"
						rel="noopener noreferrer"
						class="social-link linkedin"
						aria-label="LinkedIn"
					>
						<svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
							<path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z" />
						</svg>
					</a>
				{/if}
				{#if trainer.socialLinks.youtube}
					<a
						href={trainer.socialLinks.youtube}
						target="_blank"
						rel="noopener noreferrer"
						class="social-link youtube"
						aria-label="YouTube"
					>
						<svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
							<path d="M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z" />
						</svg>
					</a>
				{/if}
			</div>
		{/if}
	</div>
{/if}

<style>
	/* Full variant */
	.trainer-card {
		background: #1e293b;
		border-radius: 12px;
		padding: 1.5rem;
		border: 1px solid #334155;
	}

	.trainer-header {
		display: flex;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.trainer-avatar-container {
		flex-shrink: 0;
	}

	.trainer-avatar {
		width: 80px;
		height: 80px;
		border-radius: 50%;
		object-fit: cover;
		border: 3px solid #f97316;
	}

	.trainer-avatar.placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: white;
		font-size: 1.5rem;
		font-weight: 700;
	}

	.trainer-meta {
		display: flex;
		flex-direction: column;
		justify-content: center;
	}

	.trainer-name {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 700;
		color: white;
	}

	.trainer-title {
		margin: 0.25rem 0 0;
		font-size: 0.875rem;
		color: #94a3b8;
	}

	.lesson-count {
		margin-top: 0.5rem;
		font-size: 0.75rem;
		color: #f97316;
		font-weight: 600;
	}

	.trainer-bio {
		margin-bottom: 1rem;
	}

	.trainer-bio p {
		margin: 0;
		font-size: 0.875rem;
		color: #cbd5e1;
		line-height: 1.6;
	}

	.trainer-specialties h4 {
		margin: 0 0 0.5rem;
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.specialty-tags {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.specialty-tag {
		padding: 4px 10px;
		background: rgba(249, 115, 22, 0.1);
		border-radius: 9999px;
		font-size: 0.75rem;
		color: #f97316;
		font-weight: 500;
	}

	.trainer-social {
		display: flex;
		gap: 0.75rem;
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid #334155;
	}

	.social-link {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		border-radius: 8px;
		background: #334155;
		color: #94a3b8;
		transition: all 0.2s ease;
	}

	.social-link:hover {
		transform: translateY(-2px);
	}

	.social-link.twitter:hover {
		background: #1da1f2;
		color: white;
	}

	.social-link.linkedin:hover {
		background: #0077b5;
		color: white;
	}

	.social-link.youtube:hover {
		background: #ff0000;
		color: white;
	}

	/* Compact variant */
	.trainer-compact {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: #1e293b;
		border-radius: 8px;
		border: 1px solid #334155;
		transition: all 0.2s ease;
	}

	.trainer-compact:hover {
		border-color: rgba(249, 115, 22, 0.3);
	}

	.trainer-compact-avatar-wrap {
		flex-shrink: 0;
	}

	.trainer-compact-avatar {
		width: 48px;
		height: 48px;
		border-radius: 50%;
		object-fit: cover;
		border: 2px solid #f97316;
	}

	.trainer-compact-avatar.placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: white;
		font-size: 1rem;
		font-weight: 700;
	}

	.trainer-compact-content {
		flex: 1;
		min-width: 0;
	}

	.trainer-compact-name {
		margin: 0;
		font-size: 0.875rem;
		font-weight: 600;
		color: white;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.trainer-compact-title {
		margin: 0.125rem 0 0;
		font-size: 0.75rem;
		color: #94a3b8;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.trainer-compact-lessons {
		font-size: 0.7rem;
		color: #f97316;
		font-weight: 500;
	}

	/* Inline variant */
	.trainer-inline {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
	}

	.trainer-inline-avatar {
		width: 32px;
		height: 32px;
		border-radius: 50%;
		object-fit: cover;
		border: 2px solid #f97316;
	}

	.trainer-inline-avatar.placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: white;
		font-size: 0.75rem;
		font-weight: 700;
	}

	.trainer-inline-info {
		display: flex;
		flex-direction: column;
	}

	.trainer-inline-name {
		font-size: 0.875rem;
		font-weight: 600;
		color: white;
		line-height: 1.2;
	}

	.trainer-inline-title {
		font-size: 0.75rem;
		color: #94a3b8;
		line-height: 1.2;
	}

	/* Responsive */
	@media (max-width: 640px) {
		.trainer-header {
			flex-direction: column;
			align-items: center;
			text-align: center;
		}

		.trainer-avatar {
			width: 100px;
			height: 100px;
		}

		.specialty-tags {
			justify-content: center;
		}

		.trainer-social {
			justify-content: center;
		}
	}
</style>
