/** Missing recorded video metadata must stay distinct from measured zero values. */
export function formatVideoViews(views: number | null | undefined): string {
	if (views == null || !Number.isFinite(views) || views < 0) return 'Views unavailable';
	if (views >= 1_000_000) return `${(views / 1_000_000).toFixed(1)}M views`;
	if (views >= 1_000) return `${(views / 1_000).toFixed(1)}K views`;
	return `${views} views`;
}

export function recordedVideoDate(value: string | null | undefined): string | null {
	if (!value) return null;
	const date = value.split('T')[0];
	if (!date || !/^\d{4}-\d{2}-\d{2}$/.test(date)) return null;
	const parsed = new Date(`${date}T00:00:00.000Z`);
	return Number.isFinite(parsed.getTime()) && parsed.toISOString().slice(0, 10) === date
		? date
		: null;
}

export function videoDateSlug(value: string | null | undefined): string | null {
	const date = recordedVideoDate(value);
	return date ? `${date.slice(5, 7)}${date.slice(8, 10)}${date.slice(0, 4)}` : null;
}

export function formatRecordedVideoDate(value: string | null | undefined): string | null {
	const date = recordedVideoDate(value);
	return date
		? new Date(`${date}T00:00:00.000Z`).toLocaleDateString('en-US', {
				year: 'numeric',
				month: 'long',
				day: 'numeric',
				timeZone: 'UTC'
			})
		: null;
}

export function formatRecordedVideoDuration(seconds: number | null | undefined): string | null {
	if (seconds == null || !Number.isFinite(seconds) || seconds < 0) return null;
	const wholeSeconds = Math.floor(seconds);
	const hours = Math.floor(wholeSeconds / 3600);
	const minutes = Math.floor((wholeSeconds % 3600) / 60);
	const remainder = String(wholeSeconds % 60).padStart(2, '0');
	return hours > 0
		? `${hours}:${String(minutes).padStart(2, '0')}:${remainder}`
		: `${minutes}:${remainder}`;
}

export function videoPlaybackUnavailable(video: {
	video_url?: string | null;
	embed_url?: string | null;
}): boolean {
	// Partial legacy card props may omit both properties; an explicit null comes from the API.
	return (
		(video.video_url === null || video.embed_url === null) && !video.video_url && !video.embed_url
	);
}
