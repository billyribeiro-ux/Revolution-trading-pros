/**
 * Type declarations for lottie-web
 * Minimal type definitions for the lottie-web library
 */

declare module 'lottie-web/build/player/lottie_light.min.js' {
	export interface AnimationItem {
		play(): void;
		stop(): void;
		pause(): void;
		destroy(): void;
		setSpeed(speed: number): void;
		setDirection(direction: number): void;
		goToAndStop(value: number, isFrame?: boolean): void;
		goToAndPlay(value: number, isFrame?: boolean): void;
	}

	/** Lottie animation JSON data structure */
	export interface AnimationData {
		v?: string; // Version
		fr?: number; // Frame rate
		ip?: number; // In point
		op?: number; // Out point
		w?: number; // Width
		h?: number; // Height
		nm?: string; // Name
		ddd?: number; // 3D
		assets?: unknown[];
		layers?: unknown[];
		[key: string]: unknown;
	}

	/** Renderer-specific settings */
	export interface RendererSettings {
		preserveAspectRatio?: string;
		progressiveLoad?: boolean;
		hideOnTransparent?: boolean;
		className?: string;
		[key: string]: unknown;
	}

	export interface AnimationConfig {
		container: HTMLElement;
		renderer?: 'svg' | 'canvas' | 'html';
		loop?: boolean;
		autoplay?: boolean;
		/** Lottie animation JSON data */
		animationData?: AnimationData;
		/** Path to animation JSON file */
		path?: string;
		/** Renderer-specific configuration */
		rendererSettings?: RendererSettings;
	}

	export function loadAnimation(params: AnimationConfig): AnimationItem;
	export function destroy(name?: string): void;
	export function setQuality(quality: string | number): void;
}
