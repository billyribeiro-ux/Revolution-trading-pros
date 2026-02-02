/**
 * Media Controls Hook
 * ═══════════════════════════════════════════════════════════════════════════
 * Composable logic for audio/video player controls
 */

import { getBlockStateManager, type BlockId } from '$lib/stores/blockState.svelte';

export interface MediaControlsOptions {
	blockId: BlockId;
	type: 'audio' | 'video';
	onError?: (error: Error) => void;
	onEnded?: () => void;
}

export function useMediaControls(options: MediaControlsOptions) {
	const stateManager = getBlockStateManager();
	const { blockId, type, onError, onEnded } = options;

	// Reactive state from manager
	const state = $derived(stateManager.getAudioState(blockId));

	// Element reference
	let elementRef = $state<HTMLMediaElement | null>(null);

	// ========================================================================
	// Playback Controls
	// ========================================================================

	function togglePlay(): void {
		if (!elementRef) return;

		try {
			if (state.playing) {
				elementRef.pause();
			} else {
				const playPromise = elementRef.play();
				if (playPromise !== undefined) {
					playPromise.catch((error) => {
						console.error('Play failed:', error);
						onError?.(error);
						stateManager.setAudioState(blockId, { playing: false });
					});
				}
			}
			stateManager.setAudioState(blockId, { playing: !state.playing });
		} catch (error) {
			onError?.(error instanceof Error ? error : new Error(String(error)));
		}
	}

	function pause(): void {
		if (!elementRef) return;
		elementRef.pause();
		stateManager.setAudioState(blockId, { playing: false });
	}

	function play(): void {
		if (!elementRef) return;
		const playPromise = elementRef.play();
		if (playPromise !== undefined) {
			playPromise
				.then(() => {
					stateManager.setAudioState(blockId, { playing: true });
				})
				.catch((error) => {
					console.error('Play failed:', error);
					onError?.(error);
				});
		}
	}

	// ========================================================================
	// Time Controls
	// ========================================================================

	function handleTimeUpdate(): void {
		if (!elementRef) return;

		const currentTime = elementRef.currentTime;
		const duration = elementRef.duration;

		if (isNaN(duration) || duration === 0) return;

		const progress = (currentTime / duration) * 100;

		stateManager.setAudioState(blockId, {
			currentTime,
			duration,
			progress
		});
	}

	function seek(percent: number): void {
		if (!elementRef || isNaN(elementRef.duration)) return;

		const time = (percent / 100) * elementRef.duration;
		elementRef.currentTime = time;
		stateManager.setAudioState(blockId, {
			currentTime: time,
			progress: percent
		});
	}

	function handleSeekClick(event: MouseEvent): void {
		if (!elementRef) return;

		const container = event.currentTarget as HTMLElement;
		const rect = container.getBoundingClientRect();
		const percent = ((event.clientX - rect.left) / rect.width) * 100;

		seek(Math.max(0, Math.min(100, percent)));
	}

	// ========================================================================
	// Volume Controls
	// ========================================================================

	function setVolume(volume: number): void {
		if (!elementRef) return;

		const clampedVolume = Math.max(0, Math.min(1, volume));
		elementRef.volume = clampedVolume;

		stateManager.setAudioState(blockId, {
			volume: clampedVolume,
			muted: clampedVolume === 0
		});
	}

	function toggleMute(): void {
		if (!elementRef) return;

		const newMuted = !state.muted;
		elementRef.muted = newMuted;

		stateManager.setAudioState(blockId, { muted: newMuted });
	}

	function handleVolumeChange(event: Event): void {
		const value = parseFloat((event.target as HTMLInputElement).value);
		setVolume(value);
	}

	// ========================================================================
	// Event Handlers
	// ========================================================================

	function handleEnded(): void {
		stateManager.setAudioState(blockId, { playing: false, progress: 0 });
		if (elementRef) {
			elementRef.currentTime = 0;
		}
		onEnded?.();
	}

	function handleError(event: Event): void {
		const error = new Error(
			`${type} playback error: ${(event.target as HTMLMediaElement).error?.message || 'Unknown error'}`
		);
		console.error(error);
		onError?.(error);
		stateManager.setAudioState(blockId, { playing: false });
	}

	function handleLoadedMetadata(): void {
		if (!elementRef) return;
		stateManager.setAudioState(blockId, {
			duration: elementRef.duration
		});
	}

	// ========================================================================
	// Utility Functions
	// ========================================================================

	function formatTime(seconds: number): string {
		if (!seconds || isNaN(seconds)) return '0:00';
		const mins = Math.floor(seconds / 60);
		const secs = Math.floor(seconds % 60);
		return `${mins}:${secs.toString().padStart(2, '0')}`;
	}

	// ========================================================================
	// Cleanup
	// ========================================================================

	function cleanup(): void {
		if (elementRef) {
			elementRef.pause();
			elementRef.src = '';
			elementRef.load();
		}
	}

	stateManager.registerCleanup(blockId, cleanup);

	// ========================================================================
	// Return API
	// ========================================================================

	return {
		// State (reactive)
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

		// Element ref
		get elementRef() {
			return elementRef;
		},
		set elementRef(value: HTMLMediaElement | null) {
			elementRef = value;
		},

		// Playback
		togglePlay,
		play,
		pause,

		// Time
		seek,
		handleSeekClick,
		handleTimeUpdate,

		// Volume
		setVolume,
		toggleMute,
		handleVolumeChange,

		// Events
		handleEnded,
		handleError,
		handleLoadedMetadata,

		// Utils
		formatTime,
		cleanup
	};
}
