<script lang="ts">
	/**
	 * CourseReviews Component
	 * Apple Principal Engineer ICT 7 Grade - February 2026
	 * Display and submit course reviews with rating distribution
	 */

	import { apiFetch } from '$lib/api/config';

	interface Review {
		id: number;
		user_id: number;
		rating: number;
		title?: string;
		content?: string;
		is_verified_purchase: boolean;
		created_at: string;
	}

	interface ReviewSummary {
		avg_rating: number;
		total_reviews: number;
		rating_distribution: Record<string, number>;
	}

	interface ReviewsResponse {
		success: boolean;
		data?: {
			reviews: Review[];
			summary: ReviewSummary;
		};
		error?: string;
	}

	interface ApiResponse {
		success: boolean;
		error?: string;
	}

	interface Props {
		courseSlug: string;
		isEnrolled?: boolean;
		allowSubmit?: boolean;
	}

	let props: Props = $props();

	// Props with defaults
	const isEnrolled = $derived(props.isEnrolled ?? false);
	const allowSubmit = $derived(props.allowSubmit ?? true);

	let reviews: Review[] = $state([]);
	let summary: ReviewSummary | null = $state(null);
	let isLoading = $state(true);
	let showForm = $state(false);
	let isSubmitting = $state(false);

	// Form state
	let rating = $state(5);
	let title = $state('');
	let content = $state('');
	let error = $state('');
	let successMessage = $state('');

	const ratingLabels = ['Terrible', 'Poor', 'Average', 'Good', 'Excellent'];

	async function loadReviews() {
		try {
			isLoading = true;
			const res = await apiFetch<ReviewsResponse>(`/courses/${props.courseSlug}/reviews`);
			if (res.success && res.data) {
				reviews = res.data.reviews || [];
				summary = res.data.summary || null;
			}
		} catch (e) {
			console.error('Failed to load reviews:', e);
		} finally {
			isLoading = false;
		}
	}

	async function submitReview() {
		try {
			isSubmitting = true;
			error = '';
			successMessage = '';

			const res = await apiFetch<ApiResponse>(`/member/courses/${props.courseSlug}/reviews`, {
				method: 'POST',
				body: JSON.stringify({ rating, title, content })
			});

			if (res.success) {
				successMessage = 'Review submitted successfully!';
				showForm = false;
				rating = 5;
				title = '';
				content = '';
				await loadReviews();
			} else {
				error = res.error || 'Failed to submit review';
			}
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to submit review';
		} finally {
			isSubmitting = false;
		}
	}

	async function _deleteReview() {
		if (!confirm('Are you sure you want to delete your review?')) return;

		try {
			const res = await apiFetch<ApiResponse>(`/member/courses/${props.courseSlug}/reviews`, {
				method: 'DELETE'
			});
			if (res.success) {
				successMessage = 'Review deleted';
				await loadReviews();
			}
		} catch (e) {
			console.error('Failed to delete review:', e);
		}
	}

	const formatDate = (dateStr: string): string => {
		return new Date(dateStr).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	};

	$effect(() => {
		loadReviews();
	});
</script>

<div class="reviews-section">
	<div class="reviews-header">
		<h2>Student Reviews</h2>
		{#if allowSubmit && isEnrolled}
			<button class="btn-write-review" onclick={() => (showForm = !showForm)}>
				{showForm ? 'Cancel' : 'Write a Review'}
			</button>
		{/if}
	</div>

	{#if successMessage}
		<div class="alert success">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="20"
				height="20"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" /><polyline points="22 4 12 14.01 9 11.01" />
			</svg>
			{successMessage}
		</div>
	{/if}

	{#if showForm}
		<div class="review-form">
			<h3>Share Your Experience</h3>

			<div class="rating-input">
				<span class="label">Your Rating</span>
				<div class="stars-input">
					{#each [1, 2, 3, 4, 5] as star}
						<button
							type="button"
							class="star-btn"
							class:active={star <= rating}
							onclick={() => (rating = star)}
							aria-label="Rate {star} stars"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								width="28"
								height="28"
								viewBox="0 0 24 24"
								fill={star <= rating ? 'currentColor' : 'none'}
								stroke="currentColor"
								stroke-width="2"
							>
								<polygon
									points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"
								/>
							</svg>
						</button>
					{/each}
				</div>
				<span class="rating-label">{ratingLabels[rating - 1]}</span>
			</div>

			<div class="form-field">
				<label for="review-title">Title (optional)</label>
				<input
					type="text"
					id="review-title"
					bind:value={title}
					placeholder="Summarize your experience"
					maxlength="100"
				/>
			</div>

			<div class="form-field">
				<label for="review-content">Your Review (optional)</label>
				<textarea
					id="review-content"
					bind:value={content}
					placeholder="Tell others what you thought about this course..."
					rows="4"
					maxlength="2000"
				></textarea>
			</div>

			{#if error}
				<div class="alert error">{error}</div>
			{/if}

			<div class="form-actions">
				<button class="btn secondary" onclick={() => (showForm = false)}>Cancel</button>
				<button class="btn primary" onclick={submitReview} disabled={isSubmitting}>
					{isSubmitting ? 'Submitting...' : 'Submit Review'}
				</button>
			</div>
		</div>
	{/if}

	{#if isLoading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading reviews...</p>
		</div>
	{:else if summary}
		<div class="reviews-summary">
			<div class="avg-rating">
				<span class="rating-value">{summary.avg_rating.toFixed(1)}</span>
				<div class="rating-stars">
					{#each [1, 2, 3, 4, 5] as star}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="20"
							height="20"
							viewBox="0 0 24 24"
							fill={star <= Math.round(summary.avg_rating) ? '#fbbf24' : 'none'}
							stroke="#fbbf24"
							stroke-width="2"
						>
							<polygon
								points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"
							/>
						</svg>
					{/each}
				</div>
				<span class="total-reviews">{summary.total_reviews} reviews</span>
			</div>

			<div class="rating-distribution">
				{#each ['5', '4', '3', '2', '1'] as starNum}
					{@const count = summary.rating_distribution[starNum] || 0}
					{@const percent = summary.total_reviews > 0 ? (count / summary.total_reviews) * 100 : 0}
					<div class="distribution-row">
						<span class="star-label">{starNum} star</span>
						<div class="bar-container">
							<div class="bar-fill" style="width: {percent}%"></div>
						</div>
						<span class="count">{count}</span>
					</div>
				{/each}
			</div>
		</div>

		{#if reviews.length > 0}
			<div class="reviews-list">
				{#each reviews as review}
					<div class="review-card">
						<div class="review-header">
							<div class="user-info">
								<div class="avatar">
									<svg
										xmlns="http://www.w3.org/2000/svg"
										width="24"
										height="24"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="2"
									>
										<path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" /><circle
											cx="12"
											cy="7"
											r="4"
										/>
									</svg>
								</div>
								<div class="user-meta">
									<span class="username">Student</span>
									{#if review.is_verified_purchase}
										<span class="verified">Verified Purchase</span>
									{/if}
								</div>
							</div>
							<span class="review-date">{formatDate(review.created_at)}</span>
						</div>
						<div class="review-rating">
							{#each [1, 2, 3, 4, 5] as star}
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="16"
									height="16"
									viewBox="0 0 24 24"
									fill={star <= review.rating ? '#fbbf24' : 'none'}
									stroke="#fbbf24"
									stroke-width="2"
								>
									<polygon
										points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"
									/>
								</svg>
							{/each}
						</div>
						{#if review.title}
							<h4 class="review-title">{review.title}</h4>
						{/if}
						{#if review.content}
							<p class="review-content">{review.content}</p>
						{/if}
					</div>
				{/each}
			</div>
		{:else}
			<div class="no-reviews">
				<p>No reviews yet. Be the first to share your experience!</p>
			</div>
		{/if}
	{/if}
</div>

<style>
	.reviews-section {
		padding: 24px 0;
	}

	.reviews-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 24px;
		flex-wrap: wrap;
		gap: 16px;
	}

	.reviews-header h2 {
		font-size: 24px;
		font-weight: 600;
		color: #1f2937;
		margin: 0;
	}

	.btn-write-review {
		padding: 10px 20px;
		background: #143e59;
		color: #fff;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: background 0.2s;
	}

	.btn-write-review:hover {
		background: #0f2f45;
	}

	.alert {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 16px;
		border-radius: 8px;
		margin-bottom: 24px;
	}

	.alert.success {
		background: #d1fae5;
		color: #065f46;
	}

	.alert.error {
		background: #fee2e2;
		color: #991b1b;
	}

	.review-form {
		background: #f9fafb;
		border-radius: 12px;
		padding: 24px;
		margin-bottom: 32px;
	}

	.review-form h3 {
		margin: 0 0 20px;
		font-size: 18px;
		color: #1f2937;
	}

	.rating-input {
		margin-bottom: 20px;
	}

	.rating-input .label {
		display: block;
		font-size: 14px;
		font-weight: 500;
		color: #374151;
		margin-bottom: 8px;
	}

	.stars-input {
		display: flex;
		gap: 4px;
		margin-bottom: 4px;
	}

	.star-btn {
		background: none;
		border: none;
		padding: 4px;
		cursor: pointer;
		color: #d1d5db;
		transition: color 0.2s;
	}

	.star-btn.active,
	.star-btn:hover {
		color: #fbbf24;
	}

	.rating-label {
		font-size: 14px;
		color: #6b7280;
	}

	.form-field {
		margin-bottom: 16px;
	}

	.form-field label {
		display: block;
		font-size: 14px;
		font-weight: 500;
		color: #374151;
		margin-bottom: 6px;
	}

	.form-field input,
	.form-field textarea {
		width: 100%;
		padding: 12px;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		font-size: 14px;
		resize: vertical;
	}

	.form-field input:focus,
	.form-field textarea:focus {
		outline: none;
		border-color: #143e59;
		box-shadow: 0 0 0 3px rgba(20, 62, 89, 0.1);
	}

	.form-actions {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		margin-top: 20px;
	}

	.btn {
		padding: 12px 24px;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn.primary {
		background: #143e59;
		color: #fff;
	}
	.btn.primary:hover:not(:disabled) {
		background: #0f2f45;
	}
	.btn.primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
	.btn.secondary {
		background: #fff;
		color: #374151;
		border: 1px solid #d1d5db;
	}
	.btn.secondary:hover {
		background: #f9fafb;
	}

	.loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 12px;
		padding: 40px 0;
		color: #6b7280;
	}

	.spinner {
		width: 32px;
		height: 32px;
		border: 3px solid #e5e7eb;
		border-top-color: #143e59;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.reviews-summary {
		display: grid;
		grid-template-columns: auto 1fr;
		gap: 32px;
		padding: 24px;
		background: #f9fafb;
		border-radius: 12px;
		margin-bottom: 32px;
	}

	.avg-rating {
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
	}

	.rating-value {
		font-size: 48px;
		font-weight: 700;
		color: #1f2937;
		line-height: 1;
	}

	.rating-stars {
		display: flex;
		gap: 2px;
		margin: 8px 0;
	}

	.total-reviews {
		font-size: 14px;
		color: #6b7280;
	}

	.rating-distribution {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.distribution-row {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.star-label {
		font-size: 13px;
		color: #6b7280;
		width: 50px;
	}

	.bar-container {
		flex: 1;
		height: 8px;
		background: #e5e7eb;
		border-radius: 4px;
		overflow: hidden;
	}

	.bar-fill {
		height: 100%;
		background: #fbbf24;
		border-radius: 4px;
	}

	.count {
		font-size: 13px;
		color: #6b7280;
		width: 30px;
		text-align: right;
	}

	.reviews-list {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.review-card {
		padding: 20px;
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
	}

	.review-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 12px;
	}

	.user-info {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.avatar {
		width: 40px;
		height: 40px;
		background: #e5e7eb;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #6b7280;
	}

	.user-meta {
		display: flex;
		flex-direction: column;
	}

	.username {
		font-size: 14px;
		font-weight: 600;
		color: #1f2937;
	}

	.verified {
		font-size: 12px;
		color: #10b981;
	}

	.review-date {
		font-size: 13px;
		color: #9ca3af;
	}

	.review-rating {
		display: flex;
		gap: 2px;
		margin-bottom: 12px;
	}

	.review-title {
		font-size: 16px;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 8px;
	}

	.review-content {
		font-size: 14px;
		color: #4b5563;
		line-height: 1.6;
		margin: 0;
	}

	.no-reviews {
		text-align: center;
		padding: 40px 20px;
		color: #6b7280;
	}

	/* Responsive */
	@media (max-width: 639px) {
		.reviews-summary {
			grid-template-columns: 1fr;
			text-align: center;
		}

		.review-header {
			flex-direction: column;
			gap: 8px;
		}

		.form-actions {
			flex-direction: column;
		}

		.form-actions .btn {
			width: 100%;
		}
	}
</style>
