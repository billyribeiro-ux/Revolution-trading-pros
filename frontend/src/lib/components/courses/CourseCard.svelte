<script lang="ts">
	/**
	 * CourseCard Component
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 */

	interface Course {
		id: string;
		title: string;
		slug: string;
		card_image_url?: string;
		card_description?: string;
		card_badge?: string;
		card_badge_color?: string;
		instructor_name?: string;
		instructor_avatar_url?: string;
		level?: string;
		price_cents: number;
		is_free?: boolean;
		lesson_count?: number;
		total_duration_minutes?: number;
		enrollment_count?: number;
		avg_rating?: number;
		review_count?: number;
	}

	interface Props {
		course: Course;
		variant?: 'default' | 'compact' | 'featured' | 'admin';
		showProgress?: boolean;
		progress?: number;
		href?: string;
		onclick?: () => void;
	}

	let {
		course,
		variant = 'default',
		showProgress = false,
		progress = 0,
		href,
		onclick
	}: Props = $props();

	const formatDuration = (minutes: number | undefined): string => {
		if (!minutes) return '';
		const hours = Math.floor(minutes / 60);
		const mins = minutes % 60;
		if (hours > 0) return mins > 0 ? `${hours}h ${mins}m` : `${hours}h`;
		return `${mins}m`;
	};

	const formatPrice = (cents: number, isFree?: boolean): string => {
		if (isFree || cents === 0) return 'Free';
		return `$${(cents / 100).toFixed(2)}`;
	};

	const defaultImage = '/logos/revolution-logo.png';
	const imageUrl = $derived(course.card_image_url || defaultImage);
	const linkHref = $derived(href || `/classes/${course.slug}`);
</script>

{#if variant === 'compact'}
	<a href={linkHref} class="card compact" onclick={(e) => { if (onclick) { e.preventDefault(); onclick(); } }}>
		<img src={imageUrl} alt={course.title} class="thumb" loading="lazy" />
		<div class="info">
			<h4 class="title">{course.title}</h4>
			<div class="meta">
				{#if course.lesson_count}<span>{course.lesson_count} lessons</span>{/if}
				{#if course.total_duration_minutes}<span>• {formatDuration(course.total_duration_minutes)}</span>{/if}
			</div>
		</div>
		{#if showProgress}
			<div class="progress-ring">
				<svg viewBox="0 0 36 36">
					<path d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" fill="none" stroke="#e5e7eb" stroke-width="3" />
					<path d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" fill="none" stroke="#10b981" stroke-width="3" stroke-dasharray="{progress}, 100" />
				</svg>
				<span>{progress}%</span>
			</div>
		{/if}
	</a>
{:else}
	<a href={linkHref} class="card {variant}" onclick={(e) => { if (onclick) { e.preventDefault(); onclick(); } }}>
		<div class="image-wrap">
			<img src={imageUrl} alt={course.title} loading="lazy" />
			{#if course.card_badge}
				<span class="badge" style="background-color: {course.card_badge_color || '#10b981'}">{course.card_badge}</span>
			{/if}
		</div>
		<div class="content">
			{#if course.level}
				<span class="level level--{course.level?.toLowerCase()}">{course.level}</span>
			{/if}
			<h3 class="title">{course.title}</h3>
			{#if course.card_description}
				<p class="desc">{course.card_description}</p>
			{/if}
			<div class="meta">
				{#if course.instructor_name}<span class="instructor">{course.instructor_name}</span>{/if}
				<div class="stats">
					{#if course.lesson_count}<span>{course.lesson_count} lessons</span>{/if}
					{#if course.total_duration_minutes}<span>• {formatDuration(course.total_duration_minutes)}</span>{/if}
				</div>
			</div>
			<div class="footer">
				{#if course.avg_rating}
					<div class="rating">
						<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="currentColor"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>
						<span>{course.avg_rating.toFixed(1)}</span>
						{#if course.review_count}<span class="count">({course.review_count})</span>{/if}
					</div>
				{/if}
				<span class="price">{formatPrice(course.price_cents, course.is_free)}</span>
			</div>
			{#if showProgress}
				<div class="progress"><div class="bar" style="width: {progress}%"></div></div>
			{/if}
		</div>
	</a>
{/if}

<style>
	.card { display: flex; flex-direction: column; background: #fff; border-radius: 12px; overflow: hidden; box-shadow: 0 1px 3px rgba(0,0,0,0.1); transition: all 0.2s; text-decoration: none; color: inherit; }
	.card:hover { box-shadow: 0 4px 12px rgba(0,0,0,0.15); transform: translateY(-2px); }
	.image-wrap { position: relative; aspect-ratio: 16/9; overflow: hidden; }
	.image-wrap img { width: 100%; height: 100%; object-fit: cover; }
	.badge { position: absolute; top: 12px; left: 12px; padding: 4px 10px; border-radius: 4px; font-size: 11px; font-weight: 600; text-transform: uppercase; color: #fff; }
	.content { padding: 16px; display: flex; flex-direction: column; gap: 8px; flex: 1; }
	.level { font-size: 11px; font-weight: 600; text-transform: uppercase; color: #6b7280; }
	.level--beginner { color: #10b981; }
	.level--intermediate { color: #f59e0b; }
	.level--advanced { color: #ef4444; }
	.title { font-size: 16px; font-weight: 600; color: #1f2937; margin: 0; line-height: 1.3; display: -webkit-box; -webkit-line-clamp: 2; line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; }
	.desc { font-size: 13px; color: #6b7280; margin: 0; line-height: 1.5; display: -webkit-box; -webkit-line-clamp: 2; line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; }
	.meta { display: flex; flex-direction: column; gap: 4px; margin-top: auto; }
	.instructor { font-size: 13px; color: #4b5563; }
	.stats { display: flex; gap: 6px; font-size: 12px; color: #9ca3af; }
	.footer { display: flex; align-items: center; justify-content: space-between; padding-top: 12px; border-top: 1px solid #f3f4f6; margin-top: 8px; }
	.rating { display: flex; align-items: center; gap: 4px; font-size: 13px; color: #1f2937; font-weight: 500; }
	.rating svg { color: #fbbf24; }
	.count { color: #9ca3af; font-weight: 400; }
	.price { font-size: 16px; font-weight: 700; color: #143e59; }
	.progress { height: 4px; background: #e5e7eb; border-radius: 2px; overflow: hidden; margin-top: 8px; }
	.bar { height: 100%; background: #10b981; border-radius: 2px; }
	.compact { flex-direction: row; padding: 12px; gap: 12px; align-items: center; }
	.compact .thumb { width: 80px; height: 60px; border-radius: 6px; flex-shrink: 0; object-fit: cover; }
	.compact .info { padding: 0; gap: 4px; display: flex; flex-direction: column; }
	.compact .title { font-size: 14px; }
	.compact .meta { flex-direction: row; font-size: 12px; color: #9ca3af; }
	.progress-ring { width: 40px; height: 40px; position: relative; flex-shrink: 0; }
	.progress-ring svg { width: 100%; height: 100%; transform: rotate(-90deg); }
	.progress-ring span { position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); font-size: 10px; font-weight: 600; }
	.featured { flex-direction: row; max-width: 800px; }
	.featured .image-wrap { width: 320px; flex-shrink: 0; aspect-ratio: auto; height: 200px; }
	.featured .content { padding: 24px; }
	.featured .title { font-size: 20px; }
	@media (max-width: 768px) { .featured { flex-direction: column; } .featured .image-wrap { width: 100%; height: auto; aspect-ratio: 16/9; } }
</style>
