<script lang="ts">
	/**
	 * Course Header Preview Component
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 */

	import type { CourseHeaderConfig } from '../../types';

	interface Props {
		config: CourseHeaderConfig;
		isPreview: boolean;
	}

	let { config, isPreview }: Props = $props();
</script>

<div
	class="course-header"
	style="background-color: {config.backgroundColor ?? '#143E59'}; color: {config.textColor ??
		'#FFFFFF'};"
>
	<div class="header-content">
		{#if config.subtitle}
			<p class="subtitle">{config.subtitle}</p>
		{/if}
		<h1 class="title">{config.title || 'Course Title'}</h1>
		{#if config.description}
			<p class="description">{config.description}</p>
		{/if}
		{#if config.showLoginButton}
			<button class="login-btn">
				{config.loginButtonText || 'LOGIN TO THE CLASSROOM'}
			</button>
		{/if}
	</div>
</div>

<style>
	.course-header {
		padding: 40px 24px;
		text-align: center;
		border-radius: 4px;
	}

	.header-content {
		max-width: 800px;
		margin: 0 auto;
	}

	.subtitle {
		font-size: 14px;
		text-transform: uppercase;
		letter-spacing: 1px;
		opacity: 0.8;
		margin: 0 0 8px 0;
	}

	.title {
		font-size: 2rem;
		font-weight: 600;
		margin: 0 0 16px 0;
		line-height: 1.2;
	}

	.description {
		font-size: 16px;
		opacity: 0.9;
		margin: 0 0 24px 0;
		line-height: 1.5;
	}

	.login-btn {
		display: inline-block;
		padding: 12px 32px;
		background: #e67e22;
		color: white;
		border: none;
		border-radius: 4px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: background 0.2s ease;
	}

	.login-btn:hover {
		background: #d35400;
	}
</style>
