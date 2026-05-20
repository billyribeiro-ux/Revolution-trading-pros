<script lang="ts">
	import IconCalendar from '@tabler/icons-svelte-runes/icons/calendar';
	import type { PostState } from './types';

	type Props = {
		post: PostState;
	};

	let { post = $bindable() }: Props = $props();
</script>

<div class="sidebar-panel">
	<h3>Publish</h3>

	<div class="form-group">
		<label for="publish-date">
			<IconCalendar size={16} />
			Publish Date
		</label>
		<input
			id="publish-date"
			name="publish-date"
			type="datetime-local"
			bind:value={post.published_at}
		/>
	</div>

	<div class="form-group">
		<label>
			<input
				id="page-post-allow-comments"
				name="page-post-allow-comments"
				type="checkbox"
				bind:checked={post.allow_comments}
			/>
			Allow Comments
		</label>
	</div>
</div>

<style>
	.sidebar-panel {
		background: white;
		padding: 1.5rem;
		border-radius: 8px;
		border: 1px solid #e5e5e5;
	}

	.sidebar-panel h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0 0 1rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid #f0f0f0;
	}

	.sidebar-panel input[type='datetime-local'] {
		width: 100%;
		padding: 0.625rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.9rem;
		color: #1a1a1a;
		background: white;
	}

	.sidebar-panel input[type='checkbox'] {
		margin-right: 0.5rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-weight: 500;
		color: #1a1a1a;
		font-size: 0.95rem;
	}
</style>
