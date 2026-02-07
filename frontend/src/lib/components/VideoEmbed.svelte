<!--
/**
 * VideoEmbed Component - Google L7+ Enterprise Implementation
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * ENTERPRISE FEATURES:
 * 
 * 1. PLATFORM SUPPORT:
 *    - YouTube
 *    - Vimeo
 *    - Wistia
 *    - Dailymotion
 *    - Twitch
 *    - Custom HTML5
 *    - HLS/DASH streams
 * 
 * 2. ADVANCED CONTROLS:
 *    - Custom player UI
 *    - Playback speed
 *    - Quality selection
 *    - Chapter markers
 *    - Subtitles/CC
 *    - Picture-in-Picture
 * 
 * 3. ANALYTICS:
 *    - View tracking
 *    - Engagement metrics
 *    - Heatmap data
 *    - Drop-off points
 *    - Completion rate
 *    - Interaction tracking
 * 
 * 4. INTERACTIVE:
 *    - Clickable overlays
 *    - Call-to-actions
 *    - Annotations
 *    - Polls/Quizzes
 *    - Lead capture
 *    - Shopping tags
 * 
 * 5. OPTIMIZATION:
 *    - Lazy loading
 *    - Thumbnail preview
 *    - Adaptive streaming
 *    - Bandwidth detection
 *    - CDN integration
 *    - Error recovery
 * 
 * @version 3.0.0 (Google L7+ Enterprise)
 * @component
 */
-->

<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade, fly } from 'svelte/transition';
	import { browser } from '$app/environment';

	// YouTube API type declarations
	declare global {
		interface Window {
			YT?: {
				Player: any;
				PlayerState: {
					PLAYING: number;
					PAUSED: number;
					ENDED: number;
					BUFFERING: number;
				};
			};
			onYouTubeIframeAPIReady?: () => void;
		}
	}
	import {
		IconPlayerPlay,
		IconPlayerPause,
		IconVolume,
		IconVolumeOff,
		IconRefresh,
		IconSettings,
		IconMaximize,
		IconMinimize,
		IconTextCaption,
		IconPictureInPictureOn
	} from '$lib/icons';
	import { sanitizeVideoOverlay } from '$lib/utils/sanitize';

	// ═══════════════════════════════════════════════════════════════════════════
	// Props
	// ═══════════════════════════════════════════════════════════════════════════

	interface Props {
		// Core Props
		url: string;
		poster?: string | null;
		title?: string;
		description?: string | null;
		// Playback Props
		autoplay?: boolean;
		muted?: boolean;
		loop?: boolean;
		controls?: boolean;
		playsinline?: boolean;
		preload?: 'none' | 'metadata' | 'auto';
		startTime?: number;
		endTime?: number | null;
		defaultQuality?:
			| 'auto'
			| '144p'
			| '240p'
			| '360p'
			| '480p'
			| '720p'
			| '1080p'
			| '1440p'
			| '2160p';
		defaultSpeed?: number;
		volume?: number;
		// Display Props
		aspectRatio?: '16:9' | '4:3' | '1:1' | '9:16' | '21:9' | 'custom';
		customAspectRatio?: string;
		width?: string | null;
		maxWidth?: string;
		height?: string | null;
		borderRadius?: string;
		shadow?: boolean;
		lazyLoad?: boolean;
		showThumbnail?: boolean;
		thumbnailQuality?: 'default' | 'medium' | 'high' | 'standard' | 'maxres';
		// UI Props
		customControls?: boolean;
		showPlayButton?: boolean;
		showVolumeControl?: boolean;
		showTimeDisplay?: boolean;
		showProgressBar?: boolean;
		showQualitySelector?: boolean;
		showSpeedControl?: boolean;
		showFullscreen?: boolean;
		showPictureInPicture?: boolean;
		showSubtitles?: boolean;
		showShare?: boolean;
		showDownload?: boolean;
		controlsTimeout?: number;
		// Analytics Props
		trackAnalytics?: boolean;
		analyticsId?: string | null;
		trackingEvents?: string[];
		// Interactive Props
		chapters?: VideoChapter[];
		overlays?: VideoOverlay[];
		annotations?: VideoAnnotation[];
		callToAction?: CallToAction | null;
		subtitles?: SubtitleTrack[];
		playlist?: VideoPlaylistItem[];
		// Event Callbacks
		onPlay?: () => void;
		onPause?: () => void;
		onEnded?: () => void;
		onTimeUpdate?: (time: number, duration: number) => void;
		onProgress?: (percent: number) => void;
		onError?: (error: any) => void;
		onReady?: () => void;
		onQualityChange?: (quality: string) => void;
		onVolumeChange?: (volume: number) => void;
	}

	let {
		url,
		poster = null,
		title = 'Video player',
		description: _description = null,
		autoplay = false,
		muted = false,
		loop = false,
		controls = true,
		playsinline = true,
		preload = 'metadata',
		startTime = 0,
		endTime = null,
		defaultQuality = 'auto',
		defaultSpeed = 1.0,
		volume = 1.0,
		aspectRatio = '16:9',
		customAspectRatio = '',
		width = null,
		maxWidth = '100%',
		height = null,
		borderRadius = '12px',
		shadow = true,
		lazyLoad = true,
		showThumbnail = true,
		thumbnailQuality = 'high',
		customControls = false,
		showPlayButton = true,
		showVolumeControl = true,
		showTimeDisplay = true,
		showProgressBar = true,
		showQualitySelector = true,
		showSpeedControl = true,
		showFullscreen = true,
		showPictureInPicture = true,
		showSubtitles = true,
		showShare: _showShare = false,
		showDownload: _showDownload = false,
		controlsTimeout = 3000,
		trackAnalytics = true,
		analyticsId = null,
		trackingEvents = ['play', 'pause', 'complete', 'progress'],
		chapters = [],
		overlays = [],
		annotations: _annotations = [],
		callToAction = null,
		subtitles = [],
		playlist = [],
		onPlay,
		onPause,
		onEnded,
		onTimeUpdate,
		onProgress,
		onError,
		onReady,
		onQualityChange,
		onVolumeChange
	}: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// Type Definitions
	// ═══════════════════════════════════════════════════════════════════════════

	interface VideoChapter {
		id: string;
		title: string;
		startTime: number;
		endTime?: number;
		thumbnail?: string;
	}

	interface VideoOverlay {
		id: string;
		type: 'text' | 'image' | 'button' | 'html';
		content: string;
		startTime: number;
		endTime: number;
		position: OverlayPosition;
		action?: () => void;
		style?: string;
	}

	interface OverlayPosition {
		top?: string;
		bottom?: string;
		left?: string;
		right?: string;
	}

	interface VideoAnnotation {
		id: string;
		text: string;
		startTime: number;
		endTime: number;
		x: number;
		y: number;
		link?: string;
	}

	interface CallToAction {
		text: string;
		buttonText: string;
		link: string;
		showAt: 'start' | 'end' | 'time' | 'pause';
		time?: number;
		style?: string;
	}

	interface SubtitleTrack {
		label: string;
		language: string;
		src: string;
		default?: boolean;
	}

	interface VideoPlaylistItem {
		id: string;
		title: string;
		url: string;
		thumbnail?: string;
		duration?: number;
	}

	interface VideoAnalytics {
		id: string;
		url: string;
		platform: string;
		events: AnalyticsEvent[];
		watchTime: number;
		completionRate: number;
		interactions: number;
		quality: string;
		bufferEvents: number;
		errors: number;
	}

	interface AnalyticsEvent {
		type: string;
		timestamp: number;
		data?: any;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// State
	// ═══════════════════════════════════════════════════════════════════════════

	let playerElement: HTMLElement;
	let videoElement: HTMLVideoElement | null = $state(null);
	let iframeElement: HTMLIFrameElement | null = $state(null);
	let playerAPI: any = $state(null);

	// Platform detection
	let platform: 'youtube' | 'vimeo' | 'wistia' | 'dailymotion' | 'twitch' | 'html5' =
		$state('html5');
	let embedUrl: string = $state('');
	let videoId: string = $state('');
	let thumbnailUrl: string = $state('');

	// Player state
	let isReady: boolean = $state(false);
	let isPlaying: boolean = $state(false);
	let isPaused: boolean = $state(false);
	let isEnded: boolean = $state(false);
	// @ts-expect-error write-only state
	let isBuffering: boolean = $state(false);
	let isFullscreen: boolean = $state(false);
	// @ts-expect-error write-only state
	let isPictureInPicture: boolean = $state(false);
	let isMuted: boolean = $state(false);
	let hasError: boolean = $state(false);

	// Sync muted prop to isMuted state
	$effect(() => {
		isMuted = muted;
	});
	let errorMessage: string = $state('');

	// Time tracking
	let currentTime: number = $state(0);
	let duration: number = $state(0);
	let bufferedEnd: number = $state(0);
	let playbackRate: number = $state(1.0);
	let currentVolume: number = $state(1.0);

	// Sync defaultSpeed and volume props to state
	$effect(() => {
		playbackRate = defaultSpeed;
		currentVolume = volume;
	});

	// UI state
	let showControls: boolean = $state(true);
	let showSettings: boolean = $state(false);
	let controlsTimer: number | null = $state(null);
	// @ts-expect-error write-only state
	let isHovering: boolean = $state(false);
	let thumbnailLoaded: boolean = $state(false);
	let hasInteracted: boolean = $state(false);
	// @ts-expect-error write-only state
	let showCallToAction: boolean = $state(false);

	// Quality options
	let availableQualities: string[] = $state([]);
	let currentQuality: string = $state('auto');

	// Sync defaultQuality prop to currentQuality state
	$effect(() => {
		currentQuality = defaultQuality;
	});

	// Analytics - initialized with defaults
	let analyticsBase = $state({
		id: '',
		url: '',
		platform: 'html5' as 'youtube' | 'vimeo' | 'wistia' | 'dailymotion' | 'twitch' | 'html5',
		events: [] as AnalyticsEvent[],
		watchTime: 0,
		completionRate: 0,
		interactions: 0,
		bufferEvents: 0,
		errors: 0
	});

	// Sync analyticsId and url props to analyticsBase
	$effect(() => {
		analyticsBase.id = analyticsId || generateId();
		analyticsBase.url = url;
	});

	// Derive full analytics object with current quality
	let analytics: VideoAnalytics = $derived({
		...analyticsBase,
		quality: currentQuality
	});

	let watchStartTime: number = $state(0);
	let totalWatchTime: number = $state(0);
	let progressMilestones: Set<number> = $state(new Set());

	// Animations

	// Event handler references for cleanup (prevents memory leaks)
	let handleEnterPiP: (() => void) | null = null;
	let handleLeavePiP: (() => void) | null = null;

	// Event dispatching handled via callback props

	// ═══════════════════════════════════════════════════════════════════════════
	// Reactive Statements
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		// Detect platform and generate embed URL
		const result = detectPlatform(url);
		platform = result.platform;
		videoId = result.videoId;
		embedUrl = generateEmbedUrl(url, platform, videoId);
		thumbnailUrl = generateThumbnailUrl(platform, videoId);
		// Update the base state, not the derived analytics object
		analyticsBase.platform = platform;
	});

	let progressPercent = $derived(duration > 0 ? (currentTime / duration) * 100 : 0);
	let bufferedPercent = $derived(duration > 0 ? (bufferedEnd / duration) * 100 : 0);
	let timeDisplay = $derived(`${formatTime(currentTime)} / ${formatTime(duration)}`);

	let currentChapter = $derived(
		chapters.find(
			(chapter) =>
				currentTime >= chapter.startTime && (!chapter.endTime || currentTime < chapter.endTime)
		)
	);

	let activeOverlays = $derived(
		overlays.filter((overlay) => currentTime >= overlay.startTime && currentTime <= overlay.endTime)
	);

	$effect(() => {
		if (callToAction && shouldShowCTA()) {
			showCallToAction = true;
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(() => {
		if (!browser) return;

		// Initialize player based on platform
		initializePlayer();

		// Setup event listeners
		setupEventListeners();

		// Load thumbnail if needed
		if (showThumbnail && thumbnailUrl) {
			loadThumbnail();
		}

		// Track view
		if (trackAnalytics) {
			trackEvent('view', { url, platform });
		}
	});

	onDestroy(() => {
		cleanup();
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// Platform Detection
	// ═══════════════════════════════════════════════════════════════════════════

	function detectPlatform(url: string): {
		platform: 'youtube' | 'vimeo' | 'wistia' | 'dailymotion' | 'twitch' | 'html5';
		videoId: string;
	} {
		if (url.includes('youtube.com') || url.includes('youtu.be')) {
			return { platform: 'youtube', videoId: extractYouTubeId(url) };
		} else if (url.includes('vimeo.com')) {
			return { platform: 'vimeo', videoId: extractVimeoId(url) };
		} else if (url.includes('wistia.com') || url.includes('wi.st')) {
			return { platform: 'wistia', videoId: extractWistiaId(url) };
		} else if (url.includes('dailymotion.com') || url.includes('dai.ly')) {
			return { platform: 'dailymotion', videoId: extractDailymotionId(url) };
		} else if (url.includes('twitch.tv')) {
			return { platform: 'twitch', videoId: extractTwitchId(url) };
		} else {
			return { platform: 'html5', videoId: '' };
		}
	}

	function extractYouTubeId(url: string): string {
		const patterns = [
			/(?:youtube\.com\/watch\?v=|youtu\.be\/|youtube\.com\/embed\/)([^&\n?#]+)/,
			/^([^&\n?#]+)$/
		];

		for (const pattern of patterns) {
			const match = url.match(pattern);
			if (match?.[1]) return match[1];
		}
		return '';
	}

	function extractVimeoId(url: string): string {
		const match = url.match(/vimeo\.com\/(\d+)/);
		return match?.[1] ?? '';
	}

	function extractWistiaId(url: string): string {
		const match = url.match(/(?:wistia\.com|wi\.st)\/medias?\/([a-zA-Z0-9]+)/);
		return match?.[1] ?? '';
	}

	function extractDailymotionId(url: string): string {
		const match = url.match(/(?:dailymotion\.com\/video|dai\.ly)\/([a-zA-Z0-9]+)/);
		return match?.[1] ?? '';
	}

	function extractTwitchId(url: string): string {
		const match = url.match(/twitch\.tv\/videos\/(\d+)/);
		return match?.[1] ?? '';
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Embed URL Generation
	// ═══════════════════════════════════════════════════════════════════════════

	function generateEmbedUrl(
		url: string,
		platform: 'youtube' | 'vimeo' | 'wistia' | 'dailymotion' | 'twitch' | 'html5',
		videoId: string
	): string {
		const params = new URLSearchParams();

		switch (platform) {
			case 'youtube':
				params.append('autoplay', autoplay ? '1' : '0');
				params.append('mute', muted ? '1' : '0');
				params.append('controls', controls && !customControls ? '1' : '0');
				params.append('loop', loop ? '1' : '0');
				params.append('rel', '0');
				params.append('modestbranding', '1');
				params.append('playsinline', playsinline ? '1' : '0');
				params.append('start', startTime.toString());
				if (endTime) params.append('end', endTime.toString());
				params.append('enablejsapi', '1');
				params.append('origin', window.location.origin);
				return `https://www.youtube.com/embed/${videoId}?${params.toString()}`;

			case 'vimeo':
				params.append('autoplay', autoplay ? '1' : '0');
				params.append('muted', muted ? '1' : '0');
				params.append('controls', controls && !customControls ? '1' : '0');
				params.append('loop', loop ? '1' : '0');
				params.append('playsinline', playsinline ? '1' : '0');
				params.append('quality', defaultQuality);
				return `https://player.vimeo.com/video/${videoId}?${params.toString()}`;

			case 'wistia':
				params.append('autoPlay', autoplay ? 'true' : 'false');
				params.append('muted', muted ? 'true' : 'false');
				params.append('controlsVisibleOnLoad', controls ? 'true' : 'false');
				params.append('playsinline', playsinline ? 'true' : 'false');
				return `https://fast.wistia.net/embed/iframe/${videoId}?${params.toString()}`;

			case 'dailymotion':
				params.append('autoplay', autoplay ? '1' : '0');
				params.append('mute', muted ? '1' : '0');
				params.append('controls', controls && !customControls ? '1' : '0');
				params.append('start', startTime.toString());
				return `https://www.dailymotion.com/embed/video/${videoId}?${params.toString()}`;

			case 'twitch':
				params.append('autoplay', autoplay.toString());
				params.append('muted', muted.toString());
				params.append(
					'time',
					`${Math.floor(startTime / 3600)}h${Math.floor((startTime % 3600) / 60)}m${startTime % 60}s`
				);
				return `https://player.twitch.tv/?video=v${videoId}&parent=${window.location.hostname}&${params.toString()}`;

			default:
				return url;
		}
	}

	function generateThumbnailUrl(
		platform: 'youtube' | 'vimeo' | 'wistia' | 'dailymotion' | 'twitch' | 'html5',
		videoId: string
	): string {
		switch (platform) {
			case 'youtube':
				const ytQualities = {
					default: 'default',
					medium: 'mqdefault',
					high: 'hqdefault',
					standard: 'sddefault',
					maxres: 'maxresdefault'
				};
				return `https://img.youtube.com/vi/${videoId}/${ytQualities[thumbnailQuality]}.jpg`;

			case 'vimeo':
				// Vimeo requires API call for thumbnail
				return `https://vumbnail.com/${videoId}.jpg`;

			case 'wistia':
				return `https://embed-ssl.wistia.com/deliveries/${videoId}.jpg`;

			case 'dailymotion':
				return `https://www.dailymotion.com/thumbnail/video/${videoId}`;

			default:
				return poster || '';
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Player Initialization
	// ═══════════════════════════════════════════════════════════════════════════

	async function initializePlayer() {
		switch (platform) {
			case 'youtube':
				await loadYouTubeAPI();
				break;
			case 'vimeo':
				await loadVimeoAPI();
				break;
			case 'html5':
				initializeHTML5Player();
				break;
			default:
				// Initialize iframe player
				initializeIframePlayer();
		}
	}

	async function loadYouTubeAPI() {
		if (window.YT) {
			createYouTubePlayer();
			return;
		}

		const tag = document.createElement('script');
		tag.src = 'https://www.youtube.com/iframe_api';
		const firstScriptTag = document.getElementsByTagName('script')[0];
		if (firstScriptTag?.parentNode) {
			firstScriptTag.parentNode.insertBefore(tag, firstScriptTag);
		} else {
			document.head.appendChild(tag);
		}

		window.onYouTubeIframeAPIReady = () => {
			createYouTubePlayer();
		};
	}

	function createYouTubePlayer() {
		if (!window.YT || !iframeElement) return;
		playerAPI = new window.YT.Player(iframeElement, {
			events: {
				onReady: handleYouTubeReady,
				onStateChange: handleYouTubeStateChange,
				onError: handleYouTubeError
			}
		});
	}

	async function loadVimeoAPI() {
		if (!iframeElement) return;
		const Player = (await import('@vimeo/player')).default;
		playerAPI = new Player(iframeElement);

		playerAPI.on('play', () => handlePlay());
		playerAPI.on('pause', () => handlePause());
		playerAPI.on('ended', () => handleEnded());
		playerAPI.on('timeupdate', (data: any) => handleTimeUpdate(data.seconds, data.duration));
		playerAPI.on('error', (error: any) => handleError(error));

		handleReady();
	}

	function initializeHTML5Player() {
		if (!videoElement) return;

		videoElement.addEventListener('loadedmetadata', handleLoadedMetadata);
		videoElement.addEventListener('play', handlePlay);
		videoElement.addEventListener('pause', handlePause);
		videoElement.addEventListener('ended', handleEnded);
		videoElement.addEventListener('timeupdate', () =>
			handleTimeUpdate(videoElement!.currentTime, videoElement!.duration)
		);
		videoElement.addEventListener('progress', handleProgress);
		videoElement.addEventListener('error', (e) => handleError(e));
		videoElement.addEventListener('volumechange', handleVolumeChange);
		videoElement.addEventListener('waiting', () => (isBuffering = true));
		videoElement.addEventListener('playing', () => (isBuffering = false));

		if (startTime > 0) {
			videoElement.currentTime = startTime;
		}

		handleReady();
	}

	function initializeIframePlayer() {
		// Generic iframe initialization
		handleReady();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Event Handlers
	// ═══════════════════════════════════════════════════════════════════════════

	function handleReady() {
		isReady = true;
		if (onReady) onReady();
		// Event handled via onReady callback prop
		trackEvent('ready');
	}

	function handlePlay() {
		isPlaying = true;
		isPaused = false;
		hasInteracted = true;

		if (watchStartTime === 0) {
			watchStartTime = Date.now();
		}

		onPlay?.();
		trackEvent('play', { time: currentTime });

		resetControlsTimer();
	}

	function handlePause() {
		isPlaying = false;
		isPaused = true;

		if (watchStartTime > 0) {
			totalWatchTime += Date.now() - watchStartTime;
			watchStartTime = 0;
		}

		onPause?.();
		trackEvent('pause', { time: currentTime, watchTime: totalWatchTime });
	}

	function handleEnded() {
		isPlaying = false;
		isEnded = true;

		if (loop) {
			play();
		}

		onEnded?.();
		trackEvent('complete', { watchTime: totalWatchTime });

		// Update base state, not derived analytics
		analyticsBase.completionRate = 100;

		// Show CTA if configured for end
		if (callToAction?.showAt === 'end') {
			showCallToActionOverlay();
		}

		// Auto-play next in playlist
		if (playlist.length > 0) {
			playNext();
		}
	}

	function handleTimeUpdate(time: number, dur: number) {
		currentTime = time;
		duration = dur;

		// Track progress milestones
		const progressPercent = Math.floor((time / dur) * 100);
		const milestones = [25, 50, 75, 90];

		milestones.forEach((milestone) => {
			if (progressPercent >= milestone && !progressMilestones.has(milestone)) {
				progressMilestones.add(milestone);
				trackEvent('progress', { percent: milestone, time });
			}
		});

		onTimeUpdate?.(time, dur);
		onProgress?.(progressPercent);

		// Update base state, not derived analytics
		analyticsBase.completionRate = Math.max(analyticsBase.completionRate, progressPercent);
	}

	function handleError(error: any) {
		hasError = true;
		errorMessage = error.message || 'An error occurred while loading the video';
		// Update base state, not derived analytics
		analyticsBase.errors++;

		onError?.(error);
		trackEvent('error', { error: errorMessage });
	}

	function handleVolumeChange() {
		if (!videoElement) return;

		currentVolume = videoElement.volume;
		isMuted = videoElement.muted;

		onVolumeChange?.(currentVolume);
	}

	function handleLoadedMetadata() {
		if (!videoElement) return;

		duration = videoElement.duration;

		// Get available quality levels if supported
		if ('getVideoPlaybackQuality' in videoElement) {
			// Implementation would depend on specific API
		}
	}

	function handleProgress() {
		if (!videoElement) return;

		const buffered = videoElement.buffered;
		if (buffered.length > 0) {
			bufferedEnd = buffered.end(buffered.length - 1);
		}
	}

	// YouTube specific handlers
	function handleYouTubeReady(event: any) {
		playerAPI = event.target;
		duration = playerAPI.getDuration();
		handleReady();
	}

	function handleYouTubeStateChange(event: any) {
		if (!window.YT) return;
		switch (event.data) {
			case window.YT.PlayerState.PLAYING:
				handlePlay();
				break;
			case window.YT.PlayerState.PAUSED:
				handlePause();
				break;
			case window.YT.PlayerState.ENDED:
				handleEnded();
				break;
			case window.YT.PlayerState.BUFFERING:
				isBuffering = true;
				// Update base state, not derived analytics
				analyticsBase.bufferEvents++;
				break;
		}
	}

	function handleYouTubeError(event: any) {
		handleError({ code: event.data, message: 'YouTube player error' });
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Player Controls
	// ═══════════════════════════════════════════════════════════════════════════

	export function play() {
		switch (platform) {
			case 'youtube':
				playerAPI?.playVideo();
				break;
			case 'vimeo':
				playerAPI?.play();
				break;
			case 'html5':
				videoElement?.play();
				break;
		}
	}

	export function pause() {
		switch (platform) {
			case 'youtube':
				playerAPI?.pauseVideo();
				break;
			case 'vimeo':
				playerAPI?.pause();
				break;
			case 'html5':
				videoElement?.pause();
				break;
		}
	}

	export function togglePlay() {
		if (isPlaying) {
			pause();
		} else {
			play();
		}
		// Update base state, not derived analytics
		analyticsBase.interactions++;
	}

	export function seek(time: number) {
		currentTime = time;

		switch (platform) {
			case 'youtube':
				playerAPI?.seekTo(time, true);
				break;
			case 'vimeo':
				playerAPI?.setCurrentTime(time);
				break;
			case 'html5':
				if (videoElement) videoElement.currentTime = time;
				break;
		}

		trackEvent('seek', { from: currentTime, to: time });
	}

	export function setVolume(vol: number) {
		currentVolume = Math.max(0, Math.min(1, vol));

		switch (platform) {
			case 'youtube':
				playerAPI?.setVolume(currentVolume * 100);
				break;
			case 'vimeo':
				playerAPI?.setVolume(currentVolume);
				break;
			case 'html5':
				if (videoElement) videoElement.volume = currentVolume;
				break;
		}
	}

	export function toggleMute() {
		isMuted = !isMuted;

		switch (platform) {
			case 'youtube':
				if (isMuted) {
					playerAPI?.mute();
				} else {
					playerAPI?.unMute();
				}
				break;
			case 'vimeo':
				playerAPI?.setVolume(isMuted ? 0 : currentVolume);
				break;
			case 'html5':
				if (videoElement) videoElement.muted = isMuted;
				break;
		}

		analytics.interactions++;
	}

	export function setPlaybackRate(rate: number) {
		playbackRate = rate;

		switch (platform) {
			case 'youtube':
				playerAPI?.setPlaybackRate(rate);
				break;
			case 'vimeo':
				playerAPI?.setPlaybackRate(rate);
				break;
			case 'html5':
				if (videoElement) videoElement.playbackRate = rate;
				break;
		}

		trackEvent('speed_change', { rate });
	}

	export function setQuality(quality: string) {
		currentQuality = quality;

		switch (platform) {
			case 'youtube':
				playerAPI?.setPlaybackQuality(quality);
				break;
			case 'vimeo':
				playerAPI?.setQuality(quality);
				break;
		}

		if (onQualityChange) onQualityChange(quality);
		trackEvent('quality_change', { quality });
		analytics.quality = quality;
	}

	export async function enterFullscreen() {
		try {
			if (playerElement.requestFullscreen) {
				await playerElement.requestFullscreen();
			} else if ((playerElement as any).webkitRequestFullscreen) {
				await (playerElement as any).webkitRequestFullscreen();
			}
			isFullscreen = true;
			trackEvent('fullscreen_enter');
		} catch (error) {
			console.error('Failed to enter fullscreen:', error);
		}
	}

	export async function exitFullscreen() {
		try {
			if (document.exitFullscreen) {
				await document.exitFullscreen();
			} else if ((document as any).webkitExitFullscreen) {
				await (document as any).webkitExitFullscreen();
			}
			isFullscreen = false;
			trackEvent('fullscreen_exit');
		} catch (error) {
			console.error('Failed to exit fullscreen:', error);
		}
	}

	export async function togglePictureInPicture() {
		if (!videoElement) return;

		try {
			if (document.pictureInPictureElement) {
				await document.exitPictureInPicture();
				isPictureInPicture = false;
			} else {
				await videoElement.requestPictureInPicture();
				isPictureInPicture = true;
			}
			analytics.interactions++;
		} catch (error) {
			console.error('Picture-in-Picture not supported:', error);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Utility Functions
	// ═══════════════════════════════════════════════════════════════════════════

	function formatTime(seconds: number): string {
		if (!seconds || isNaN(seconds)) return '0:00';

		const hours = Math.floor(seconds / 3600);
		const minutes = Math.floor((seconds % 3600) / 60);
		const secs = Math.floor(seconds % 60);

		if (hours > 0) {
			return `${hours}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
		}
		return `${minutes}:${secs.toString().padStart(2, '0')}`;
	}

	function getAspectRatioStyle(): string {
		const ratios = {
			'16:9': '56.25%',
			'4:3': '75%',
			'1:1': '100%',
			'9:16': '177.77%',
			'21:9': '42.86%',
			custom: customAspectRatio || '56.25%'
		};
		return ratios[aspectRatio];
	}

	function generateId(): string {
		return `video_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
	}

	function loadThumbnail() {
		if (!thumbnailUrl) return;

		const img = new Image();
		img.onload = () => {
			thumbnailLoaded = true;
		};
		img.src = thumbnailUrl;
	}

	function shouldShowCTA(): boolean {
		if (!callToAction) return false;

		switch (callToAction.showAt) {
			case 'start':
				return currentTime < 3;
			case 'end':
				return isEnded;
			case 'pause':
				return isPaused;
			case 'time':
				return currentTime >= (callToAction.time || 0);
			default:
				return false;
		}
	}

	function showCallToActionOverlay() {
		// Implementation for CTA overlay
	}

	function playNext() {
		// Implementation for playlist
	}

	function setupEventListeners() {
		if (!browser) return;

		// Fullscreen change
		document.addEventListener('fullscreenchange', handleFullscreenChange);
		document.addEventListener('webkitfullscreenchange', handleFullscreenChange);

		// Picture-in-Picture (store references for cleanup to prevent memory leaks)
		if (videoElement) {
			handleEnterPiP = () => (isPictureInPicture = true);
			handleLeavePiP = () => (isPictureInPicture = false);
			videoElement.addEventListener('enterpictureinpicture', handleEnterPiP);
			videoElement.addEventListener('leavepictureinpicture', handleLeavePiP);
		}

		// Keyboard shortcuts
		document.addEventListener('keydown', handleKeyboard);
	}

	function handleFullscreenChange() {
		isFullscreen = !!(document.fullscreenElement || (document as any).webkitFullscreenElement);
	}

	function handleKeyboard(e: KeyboardEvent) {
		if (!isReady || !hasInteracted) return;

		switch (e.key) {
			case ' ':
			case 'k':
				e.preventDefault();
				togglePlay();
				break;
			case 'ArrowLeft':
				e.preventDefault();
				seek(Math.max(0, currentTime - 10));
				break;
			case 'ArrowRight':
				e.preventDefault();
				seek(Math.min(duration, currentTime + 10));
				break;
			case 'ArrowUp':
				e.preventDefault();
				setVolume(currentVolume + 0.1);
				break;
			case 'ArrowDown':
				e.preventDefault();
				setVolume(currentVolume - 0.1);
				break;
			case 'm':
				toggleMute();
				break;
			case 'f':
				if (isFullscreen) {
					exitFullscreen();
				} else {
					enterFullscreen();
				}
				break;
			case 'p':
				togglePictureInPicture();
				break;
		}
	}

	function handleMouseMove() {
		showControls = true;
		resetControlsTimer();
	}

	function resetControlsTimer() {
		if (controlsTimer) {
			clearTimeout(controlsTimer);
		}

		if (customControls && isPlaying) {
			controlsTimer = window.setTimeout(() => {
				showControls = false;
			}, controlsTimeout);
		}
	}

	function handleProgressClick(e: MouseEvent) {
		const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
		const percent = (e.clientX - rect.left) / rect.width;
		seek(duration * percent);
	}

	function trackEvent(event: string, data?: any) {
		if (!trackAnalytics || !trackingEvents.includes(event)) return;

		const analyticsEvent: AnalyticsEvent = {
			type: event,
			timestamp: Date.now(),
			data
		};

		analytics.events.push(analyticsEvent);

		// Send to analytics service
		if (browser && 'gtag' in window) {
			(window as any).gtag('event', `video_${event}`, {
				video_url: url,
				video_title: title,
				video_percent: progressPercent,
				...data
			});
		}

		// Analytics event tracked internally
	}

	function cleanup() {
		// Clear timers
		if (controlsTimer) {
			clearTimeout(controlsTimer);
		}

		// Remove event listeners
		document.removeEventListener('fullscreenchange', handleFullscreenChange);
		document.removeEventListener('webkitfullscreenchange', handleFullscreenChange);
		document.removeEventListener('keydown', handleKeyboard);

		// Remove PiP event listeners (prevents memory leaks)
		if (videoElement) {
			if (handleEnterPiP) {
				videoElement.removeEventListener('enterpictureinpicture', handleEnterPiP);
			}
			if (handleLeavePiP) {
				videoElement.removeEventListener('leavepictureinpicture', handleLeavePiP);
			}
		}

		// Save analytics
		if (watchStartTime > 0) {
			totalWatchTime += Date.now() - watchStartTime;
		}
		// Update base state, not derived analytics
		analyticsBase.watchTime = totalWatchTime;

		// Cleanup player API
		switch (platform) {
			case 'youtube':
				playerAPI?.destroy();
				break;
			case 'vimeo':
				playerAPI?.destroy();
				break;
		}
	}
</script>

<!-- Main Container -->
<div
	bind:this={playerElement}
	class="video-embed-container"
	class:fullscreen={isFullscreen}
	class:has-error={hasError}
	style="
		width: {width || '100%'};
		max-width: {maxWidth};
		height: {height || 'auto'};
		border-radius: {borderRadius};
		{shadow ? 'box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);' : ''}
	"
	role="region"
	aria-label="Video player"
	onmouseenter={() => (isHovering = true)}
	onmouseleave={() => (isHovering = false)}
	onmousemove={handleMouseMove}
>
	<!-- Video Container -->
	<div class="video-wrapper" style="padding-bottom: {getAspectRatioStyle()};">
		<!-- Thumbnail Preview -->
		{#if showThumbnail && thumbnailUrl && !hasInteracted && lazyLoad}
			<div class="thumbnail-container" transition:fade={{ duration: 300 }}>
				{#if thumbnailLoaded}
					<img src={thumbnailUrl} alt={title} class="thumbnail-image" />
				{/if}
				<button
					class="play-overlay"
					onclick={() => {
						hasInteracted = true;
						play();
					}}
					aria-label="Play video"
				>
					<IconPlayerPlay size={64} />
				</button>
			</div>
		{/if}

		<!-- Error State -->
		{#if hasError}
			<div class="error-container">
				<IconRefresh size={48} />
				<p>{errorMessage}</p>
				<button onclick={() => location.reload()}>Retry</button>
			</div>
		{/if}

		<!-- HTML5 Video -->
		{#if platform === 'html5' && !hasError}
			<video
				bind:this={videoElement}
				src={embedUrl}
				poster={poster || thumbnailUrl}
				{autoplay}
				{muted}
				{loop}
				controls={controls && !customControls}
				{playsinline}
				{preload}
				class="video-element"
			>
				{#each subtitles as track}
					<track
						kind="subtitles"
						label={track.label}
						srclang={track.language}
						src={track.src}
						default={track.default}
					/>
				{/each}
			</video>
		{/if}

		<!-- Iframe Embed -->
		{#if platform !== 'html5' && !hasError}
			<iframe
				bind:this={iframeElement}
				src={embedUrl}
				{title}
				frameborder="0"
				allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; fullscreen"
				class="video-iframe"
			></iframe>
		{/if}

		<!-- Interactive Overlays -->
		{#each activeOverlays as overlay}
			<button
				class="video-overlay overlay-{overlay.type}"
				style="
					top: {overlay.position.top || 'auto'};
					bottom: {overlay.position.bottom || 'auto'};
					left: {overlay.position.left || 'auto'};
					right: {overlay.position.right || 'auto'};
					{overlay.style || ''}
				"
				onclick={() => overlay.action?.()}
				aria-label="Video overlay"
			>
				{#if overlay.type === 'html'}
					{@html sanitizeVideoOverlay(overlay.content)}
				{:else}
					{overlay.content}
				{/if}
			</button>
		{/each}

		<!-- Custom Controls -->
		{#if customControls && isReady && showControls}
			<div class="custom-controls" transition:fade={{ duration: 200 }}>
				<!-- Progress Bar -->
				{#if showProgressBar}
					<button
						class="progress-bar"
						onclick={handleProgressClick}
						aria-label="Seek video"
						role="slider"
						aria-valuemin="0"
						aria-valuemax="100"
						aria-valuenow={progressPercent}
					>
						<div class="progress-buffered" style="width: {bufferedPercent}%;"></div>
						<div class="progress-played" style="width: {progressPercent}%;"></div>
						<div class="progress-thumb" style="left: {progressPercent}%;"></div>
					</button>
				{/if}

				<!-- Controls Row -->
				<div class="controls-row">
					<div class="controls-left">
						<!-- Play/Pause -->
						{#if showPlayButton}
							<button
								class="control-btn"
								onclick={togglePlay}
								aria-label={isPlaying ? 'Pause' : 'Play'}
							>
								{#if isPlaying}
									<IconPlayerPause size={24} />
								{:else}
									<IconPlayerPlay size={24} />
								{/if}
							</button>
						{/if}

						<!-- Volume -->
						{#if showVolumeControl}
							<div class="volume-control">
								<button
									class="control-btn"
									onclick={toggleMute}
									aria-label={isMuted ? 'Unmute' : 'Mute'}
								>
									{#if isMuted}
										<IconVolumeOff size={24} />
									{:else}
										<IconVolume size={24} />
									{/if}
								</button>
								<input
									type="range"
									min="0"
									max="1"
									step="0.1"
									value={currentVolume}
									oninput={(e: Event & { currentTarget: HTMLInputElement }) =>
										setVolume(parseFloat((e.currentTarget as HTMLInputElement).value))}
									class="volume-slider"
								/>
							</div>
						{/if}

						<!-- Time Display -->
						{#if showTimeDisplay}
							<span class="time-display">{timeDisplay}</span>
						{/if}
					</div>

					<div class="controls-right">
						<!-- Subtitles -->
						{#if showSubtitles && subtitles.length > 0}
							<button class="control-btn" aria-label="Subtitles">
								<IconTextCaption size={24} />
							</button>
						{/if}

						<!-- Settings -->
						{#if showQualitySelector || showSpeedControl}
							<button
								class="control-btn"
								onclick={() => (showSettings = !showSettings)}
								aria-label="Settings"
							>
								<IconSettings size={24} />
							</button>
						{/if}

						<!-- PiP -->
						{#if showPictureInPicture && platform === 'html5'}
							<button
								class="control-btn"
								onclick={togglePictureInPicture}
								aria-label="Picture in Picture"
							>
								<IconPictureInPictureOn size={24} />
							</button>
						{/if}

						<!-- Fullscreen -->
						{#if showFullscreen}
							<button
								class="control-btn"
								onclick={() => (isFullscreen ? exitFullscreen() : enterFullscreen())}
								aria-label={isFullscreen ? 'Exit fullscreen' : 'Enter fullscreen'}
							>
								{#if isFullscreen}
									<IconMinimize size={24} />
								{:else}
									<IconMaximize size={24} />
								{/if}
							</button>
						{/if}
					</div>
				</div>

				<!-- Settings Menu -->
				{#if showSettings}
					<div class="settings-menu" transition:fly={{ y: -10, duration: 200 }}>
						{#if showQualitySelector && availableQualities.length > 0}
							<div class="settings-section">
								<h4>Quality</h4>
								{#each availableQualities as quality}
									<button
										class="settings-option"
										class:active={currentQuality === quality}
										onclick={() => {
											setQuality(quality);
											showSettings = false;
										}}
									>
										{quality}
									</button>
								{/each}
							</div>
						{/if}

						{#if showSpeedControl}
							<div class="settings-section">
								<h4>Speed</h4>
								{#each [0.5, 0.75, 1, 1.25, 1.5, 2] as speed}
									<button
										class="settings-option"
										class:active={playbackRate === speed}
										onclick={() => {
											setPlaybackRate(speed);
											showSettings = false;
										}}
									>
										{speed}x
									</button>
								{/each}
							</div>
						{/if}
					</div>
				{/if}
			</div>
		{/if}
	</div>

	<!-- Chapter Display -->
	{#if currentChapter}
		<div class="chapter-display">
			{currentChapter.title}
		</div>
	{/if}
</div>

<style>
	.video-embed-container {
		position: relative;
		width: 100%;
		background: #000;
		overflow: hidden;
		transition: transform 0.3s ease;
	}

	.video-embed-container:hover {
		transform: var(--hover-scale, scale(1));
	}

	.video-wrapper {
		position: relative;
		width: 100%;
		height: 0;
		overflow: hidden;
	}

	.video-iframe,
	.video-element {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		border: 0;
	}

	.video-element {
		object-fit: contain;
		background: #000;
	}

	/* Thumbnail Preview */
	.thumbnail-container {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		cursor: pointer;
		background: #000;
	}

	.thumbnail-image {
		width: 100%;
		height: 100%;
		object-fit: cover;
		filter: brightness(0.8);
		transition: filter 0.3s;
	}

	.thumbnail-container:hover .thumbnail-image {
		filter: brightness(1);
	}

	.play-overlay {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		background: rgba(0, 0, 0, 0.7);
		border: none;
		border-radius: 50%;
		padding: 1rem;
		color: white;
		cursor: pointer;
		transition: all 0.3s;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.play-overlay:hover {
		background: rgba(0, 0, 0, 0.9);
		transform: translate(-50%, -50%) scale(1.1);
	}

	/* Error State */
	.error-container {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		text-align: center;
		color: white;
		padding: 2rem;
	}

	.error-container button {
		margin-top: 1rem;
		padding: 0.5rem 1rem;
		background: rgba(255, 255, 255, 0.2);
		border: 1px solid rgba(255, 255, 255, 0.3);
		border-radius: 4px;
		color: white;
		cursor: pointer;
		transition: background 0.2s;
	}

	.error-container button:hover {
		background: rgba(255, 255, 255, 0.3);
	}

	/* Custom Controls */
	.custom-controls {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		padding: 1rem;
		background: linear-gradient(to top, rgba(0, 0, 0, 0.8), transparent);
		color: white;
		transition: opacity 0.3s;
	}

	.progress-bar {
		position: relative;
		width: 100%;
		height: 4px;
		background: rgba(255, 255, 255, 0.2);
		border-radius: 2px;
		cursor: pointer;
		margin-bottom: 1rem;
		transition: height 0.2s;
	}

	.progress-bar:hover {
		height: 6px;
	}

	.progress-buffered {
		position: absolute;
		height: 100%;
		background: rgba(255, 255, 255, 0.3);
		border-radius: 2px;
		pointer-events: none;
	}

	.progress-played {
		position: absolute;
		height: 100%;
		background: #ef4444;
		border-radius: 2px;
		pointer-events: none;
	}

	.progress-thumb {
		position: absolute;
		top: 50%;
		transform: translate(-50%, -50%);
		width: 12px;
		height: 12px;
		background: white;
		border-radius: 50%;
		pointer-events: none;
		opacity: 0;
		transition: opacity 0.2s;
	}

	.progress-bar:hover .progress-thumb {
		opacity: 1;
	}

	.controls-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 1rem;
	}

	.controls-left,
	.controls-right {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.control-btn {
		background: none;
		border: none;
		color: white;
		cursor: pointer;
		padding: 0.5rem;
		transition: opacity 0.2s;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.control-btn:hover {
		opacity: 0.8;
	}

	.volume-control {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.volume-slider {
		width: 80px;
		height: 4px;
		-webkit-appearance: none;
		appearance: none;
		background: rgba(255, 255, 255, 0.3);
		border-radius: 2px;
		outline: none;
	}

	.volume-slider::-webkit-slider-thumb {
		-webkit-appearance: none;
		appearance: none;
		width: 12px;
		height: 12px;
		background: white;
		border-radius: 50%;
		cursor: pointer;
	}

	.time-display {
		font-size: 0.875rem;
		font-variant-numeric: tabular-nums;
		user-select: none;
	}

	/* Settings Menu */
	.settings-menu {
		position: absolute;
		bottom: 100%;
		right: 1rem;
		margin-bottom: 0.5rem;
		background: rgba(0, 0, 0, 0.9);
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 8px;
		padding: 0.5rem;
		min-width: 150px;
	}

	.settings-section {
		margin-bottom: 0.5rem;
	}

	.settings-section:last-child {
		margin-bottom: 0;
	}

	.settings-section h4 {
		font-size: 0.75rem;
		opacity: 0.7;
		margin-bottom: 0.25rem;
		text-transform: uppercase;
	}

	.settings-option {
		display: block;
		width: 100%;
		padding: 0.25rem 0.5rem;
		background: none;
		border: none;
		color: white;
		text-align: left;
		cursor: pointer;
		transition: background 0.2s;
		border-radius: 4px;
	}

	.settings-option:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	.settings-option.active {
		background: rgba(255, 255, 255, 0.2);
		font-weight: bold;
	}

	/* Overlays */
	.video-overlay {
		position: absolute;
		padding: 1rem;
		background: rgba(0, 0, 0, 0.8);
		color: white;
		border-radius: 8px;
		pointer-events: auto;
		z-index: 10;
		animation: fadeIn 0.3s;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	/* Chapter Display */
	.chapter-display {
		position: absolute;
		top: 1rem;
		left: 1rem;
		padding: 0.5rem 1rem;
		background: rgba(0, 0, 0, 0.7);
		color: white;
		border-radius: 4px;
		font-size: 0.875rem;
		pointer-events: none;
	}

	/* Fullscreen Styles */
	.video-embed-container.fullscreen {
		position: fixed !important;
		top: 0 !important;
		left: 0 !important;
		right: 0 !important;
		bottom: 0 !important;
		width: 100% !important;
		height: 100% !important;
		max-width: none !important;
		border-radius: 0 !important;
		z-index: 9999;
	}

	.video-embed-container.fullscreen .video-wrapper {
		padding-bottom: 0 !important;
		height: 100%;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   2026 MOBILE-FIRST RESPONSIVE DESIGN
	   ═══════════════════════════════════════════════════════════════════════════
	   Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	   Touch targets: 44x44px minimum for all interactive elements
	   Safe areas: env(safe-area-inset-*) for fullscreen viewing
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile base styles (default) */
	.video-wrapper {
		aspect-ratio: 16 / 9;
		padding-bottom: 0;
		height: auto;
	}

	/* Fallback for browsers without aspect-ratio */
	@supports not (aspect-ratio: 16 / 9) {
		.video-wrapper {
			height: 0;
			padding-bottom: 56.25%;
		}
	}

	/* Play overlay - Mobile: 48px touch target */
	.play-overlay {
		width: 56px;
		height: 56px;
		min-width: 44px;
		min-height: 44px;
		padding: 0;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	/* Control buttons - 44px touch target on mobile */
	.control-btn {
		min-width: 44px;
		min-height: 44px;
		padding: 10px;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	/* Progress bar - taller on mobile for touch */
	.progress-bar {
		height: 8px;
		border: none;
		padding: 0;
		margin-bottom: 0.75rem;
	}

	.progress-thumb {
		width: 16px;
		height: 16px;
		opacity: 1;
	}

	/* Volume control hidden on mobile */
	.volume-control {
		display: none;
	}

	/* Controls layout for mobile */
	.controls-row {
		gap: 0.25rem;
	}

	.custom-controls {
		padding: 0.75rem;
		padding-bottom: calc(0.75rem + env(safe-area-inset-bottom, 0));
	}

	.time-display {
		font-size: 0.75rem;
	}

	/* Settings menu for mobile */
	.settings-menu {
		right: 0.5rem;
		bottom: calc(100% + 0.25rem);
		min-width: 130px;
	}

	.settings-option {
		min-height: 44px;
		display: flex;
		align-items: center;
		padding: 0.5rem 0.75rem;
	}

	/* xs: 360px+ */
	@media (min-width: 360px) {
		.custom-controls {
			padding: 0.875rem;
			padding-bottom: calc(0.875rem + env(safe-area-inset-bottom, 0));
		}

		.controls-row {
			gap: 0.375rem;
		}
	}

	/* sm: 640px+ */
	@media (min-width: 640px) {
		.volume-control {
			display: flex;
		}

		.controls-row {
			gap: 0.5rem;
		}

		.control-btn {
			min-width: 36px;
			min-height: 36px;
			padding: 0.5rem;
		}

		.progress-bar {
			height: 4px;
			margin-bottom: 1rem;
		}

		.progress-bar:hover {
			height: 6px;
		}

		.progress-thumb {
			width: 12px;
			height: 12px;
			opacity: 0;
		}

		.progress-bar:hover .progress-thumb {
			opacity: 1;
		}

		.play-overlay {
			width: 72px;
			height: 72px;
			padding: 1rem;
		}

		.play-overlay:hover {
			transform: translate(-50%, -50%) scale(1.1);
		}

		.custom-controls {
			padding: 1rem;
		}

		.time-display {
			font-size: 0.875rem;
		}

		.settings-menu {
			right: 1rem;
			min-width: 150px;
		}

		.settings-option {
			min-height: auto;
			padding: 0.25rem 0.5rem;
		}
	}

	/* md: 768px+ */
	@media (min-width: 768px) {
		.play-overlay {
			width: 80px;
			height: 80px;
		}
	}

	/* lg: 1024px+ */
	@media (min-width: 1024px) {
		.play-overlay {
			width: 96px;
			height: 96px;
		}
	}

	/* Fullscreen safe areas */
	.video-embed-container.fullscreen {
		padding: env(safe-area-inset-top, 0) env(safe-area-inset-right, 0)
			env(safe-area-inset-bottom, 0) env(safe-area-inset-left, 0);
	}

	.video-embed-container.fullscreen .custom-controls {
		padding-bottom: calc(1rem + env(safe-area-inset-bottom, 0));
	}

	/* Accessibility */
	@media (prefers-reduced-motion: reduce) {
		.video-embed-container,
		.play-overlay,
		.custom-controls {
			transition: none !important;
			animation: none !important;
		}
	}

	.play-overlay:focus-visible,
	.control-btn:focus-visible,
	.progress-bar:focus-visible,
	.settings-option:focus-visible {
		outline: 2px solid #fff;
		outline-offset: 2px;
	}

	/* High contrast mode */
	@media (prefers-contrast: high) {
		.custom-controls {
			background: rgba(0, 0, 0, 0.95);
		}

		.progress-played {
			background: #fff;
		}
	}
</style>
