/**
 * Media Controls Hook
 * ═══════════════════════════════════════════════════════════════════════════
 * Reusable media player logic for audio/video blocks
 * Provides playback, time, and volume controls with Svelte 5 runes
 */

import { getBlockStateManager, type BlockId } from '$lib/stores/blockState.svelte';
import { logger } from '$lib/utils/logger';

// ============================================================================
// Types
// ============================================================================

export interface MediaControlsOptions {
	blockId: BlockId;
	mediaElement?: HTMLAudioElement | HTMLVideoElement | null;
	onEnded?: () => void;
	onError?: (error: Error) => void;
}

export interface MediaControlsReturn {
	// State getters
	readonly playing: boolean;
	readonly progress: number;
	readonly volume: number;
	readonly muted: boolean;
	readonly duration: number;
	readonly currentTime: number;

	// Playback controls
	togglePlay: () => void;
	play: () => void;
	pause: () => void;

	// Time controls
	seek: (percent: number) => void;
	handleSeekClick: (event: MouseEvent) => void;
	handleTimeUpdate: () => void;

	// Volume controls
	setVolume: (volume: number) => void;
	toggleMute: () => void;

	// Event handlers
	handleEnded: () => void;
	handleError: (event: Event) => void;
	handleLoadedMetadata: () => void;

	// Utilities
	formatTime: (seconds: number) => string;
	cleanup: () => void;
	setMediaElement: (element: HTMLAudioElement | HTMLVideoElement | null) => void;
}

// ============================================================================
// Hook Implementation
// ============================================================================

export function useMediaControls(options: MediaControlsOptions): MediaControlsReturn {
	const stateManager = getBlockStateManager();
	const { blockId, onEnded, onError } = options;

	// Reactive state from manager
	const state = $derived(stateManager.getAudioState(blockId));

	// Media element reference using $state
	let mediaElement = $state<HTMLAudioElement | HTMLVideoElement | null>(
		options.mediaElement ?? null
	);

	// ========================================================================
	// Playback Controls
	// ========================================================================

	function togglePlay(): void {
		if (!mediaElement) return;

		try {
			if (state.playing) {
				mediaElement.pause();
				stateManager.setAudioState(blockId, { playing: false });
			} else {
				const playPromise = mediaElement.play();
				if (playPromise !== undefined) {
					playPromise
						.then(() => {
							stateManager.setAudioState(blockId, { playing: true });
						})
						.catch((error) => {
							logger.error('Play failed:', error);
							onError?.(error);
							stateManager.setAudioState(blockId, { playing: false });
						});
				} else {
					stateManager.setAudioState(blockId, { playing: true });
				}
			}
		} catch (error) {
			onError?.(error instanceof Error ? error : new Error(String(error)));
		}
	}

	function play(): void {
		if (!mediaElement) return;

		const playPromise = mediaElement.play();
		if (playPromise !== undefined) {
			playPromise
				.then(() => {
					stateManager.setAudioState(blockId, { playing: true });
				})
				.catch((error) => {
					logger.error('Play failed:', error);
					onError?.(error);
				});
		} else {
			stateManager.setAudioState(blockId, { playing: true });
		}
	}

	function pause(): void {
		if (!mediaElement) return;

		mediaElement.pause();
		stateManager.setAudioState(blockId, { playing: false });
	}

	// ========================================================================
	// Time Controls
	// ========================================================================

	function seek(percent: number): void {
		if (!mediaElement || isNaN(mediaElement.duration)) return;

		const clampedPercent = Math.max(0, Math.min(100, percent));
		const time = (clampedPercent / 100) * mediaElement.duration;

		mediaElement.currentTime = time;
		stateManager.setAudioState(blockId, {
			currentTime: time,
			progress: clampedPercent
		});
	}

	function handleSeekClick(event: MouseEvent): void {
		if (!mediaElement) return;

		const container = event.currentTarget as HTMLElement;
		const rect = container.getBoundingClientRect();
		const percent = ((event.clientX - rect.left) / rect.width) * 100;

		seek(percent);
	}

	function handleTimeUpdate(): void {
		if (!mediaElement) return;

		const currentTime = mediaElement.currentTime;
		const duration = mediaElement.duration;

		if (isNaN(duration) || duration === 0) return;

		const progress = (currentTime / duration) * 100;

		stateManager.setAudioState(blockId, {
			currentTime,
			duration,
			progress
		});
	}

	// ========================================================================
	// Volume Controls
	// ========================================================================

	function setVolume(volume: number): void {
		if (!mediaElement) return;

		const clampedVolume = Math.max(0, Math.min(1, volume));
		mediaElement.volume = clampedVolume;

		stateManager.setAudioState(blockId, {
			volume: clampedVolume,
			muted: clampedVolume === 0
		});
	}

	function toggleMute(): void {
		if (!mediaElement) return;

		const newMuted = !state.muted;
		mediaElement.muted = newMuted;

		stateManager.setAudioState(blockId, { muted: newMuted });
	}

	// ========================================================================
	// Event Handlers
	// ========================================================================

	function handleEnded(): void {
		stateManager.setAudioState(blockId, {
			playing: false,
			progress: 0,
			currentTime: 0
		});

		if (mediaElement) {
			mediaElement.currentTime = 0;
		}

		onEnded?.();
	}

	function handleError(event: Event): void {
		const mediaError = (event.target as HTMLMediaElement).error;
		const errorMessage = mediaError?.message || 'Unknown media playback error';
		const error = new Error(`Media playback error: ${errorMessage}`);

		logger.error(error);
		onError?.(error);

		stateManager.setAudioState(blockId, { playing: false });
	}

	function handleLoadedMetadata(): void {
		if (!mediaElement) return;

		stateManager.setAudioState(blockId, {
			duration: mediaElement.duration
		});
	}

	// ========================================================================
	// Utility Functions
	// ========================================================================

	function formatTime(seconds: number): string {
		if (!seconds || isNaN(seconds) || !isFinite(seconds)) {
			return '0:00';
		}

		const totalSeconds = Math.floor(seconds);
		const mins = Math.floor(totalSeconds / 60);
		const secs = totalSeconds % 60;

		return `${mins}:${secs.toString().padStart(2, '0')}`;
	}

	function cleanup(): void {
		if (mediaElement) {
			mediaElement.pause();
			mediaElement.src = '';
			mediaElement.load();
		}

		stateManager.setAudioState(blockId, {
			playing: false,
			progress: 0,
			currentTime: 0,
			duration: 0
		});
	}

	function setMediaElement(element: HTMLAudioElement | HTMLVideoElement | null): void {
		mediaElement = element;
	}

	// Register cleanup with state manager
	stateManager.registerCleanup(blockId, cleanup);

	// ========================================================================
	// Return API
	// ========================================================================

	return {
		// State getters (reactive via $derived)
		get playing() {
			return state.playing;
		},
		get progress() {
			return state.progress;
		},
		get volume() {
			return state.volume;
		},
		get muted() {
			return state.muted;
		},
		get duration() {
			return state.duration;
		},
		get currentTime() {
			return state.currentTime;
		},

		// Playback controls
		togglePlay,
		play,
		pause,

		// Time controls
		seek,
		handleSeekClick,
		handleTimeUpdate,

		// Volume controls
		setVolume,
		toggleMute,

		// Event handlers
		handleEnded,
		handleError,
		handleLoadedMetadata,

		// Utilities
		formatTime,
		cleanup,
		setMediaElement
	};
}
