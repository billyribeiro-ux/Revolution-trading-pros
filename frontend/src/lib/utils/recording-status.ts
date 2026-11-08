import type { Attachment } from 'svelte/attachments';

/** Catch source failures, including errors that happened during SSR before hydration. */
export function recordingStatus(
	onChange: (unavailable: boolean) => void
): Attachment<HTMLVideoElement> {
	return (video) => {
		const failed = () => onChange(true);
		const ready = () => onChange(false);
		video.addEventListener('error', failed, true);
		video.addEventListener('loadeddata', ready);
		video.addEventListener('loadstart', ready);
		onChange(Boolean(video.error) || video.networkState === HTMLMediaElement.NETWORK_NO_SOURCE);
		return () => {
			video.removeEventListener('error', failed, true);
			video.removeEventListener('loadeddata', ready);
			video.removeEventListener('loadstart', ready);
		};
	};
}
