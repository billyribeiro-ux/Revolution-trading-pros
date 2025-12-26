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

	export interface AnimationConfig {
		container: HTMLElement;
		renderer?: 'svg' | 'canvas' | 'html';
		loop?: boolean;
		autoplay?: boolean;
		animationData?: any;
		path?: string;
		rendererSettings?: any;
	}

	export function loadAnimation(params: AnimationConfig): AnimationItem;
	export function destroy(name?: string): void;
	export function setQuality(quality: string | number): void;
}
