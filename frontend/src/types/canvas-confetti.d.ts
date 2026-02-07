declare module 'canvas-confetti' {
	interface Options {
		particleCount?: number;
		angle?: number;
		spread?: number;
		startVelocity?: number;
		ticks?: number;
		origin?: {
			x?: number;
			y?: number;
		};
		colors?: string[];
		shapes?: string[];
		scalar?: number;
		zIndex?: number;
		disableForReducedMotion?: boolean;
		useWorker?: boolean;
		resize?: boolean;
		canvas?: HTMLCanvasElement;
	}

	interface CreateTypes {
		(options?: Options): Promise<void>;
		reset(): void;
	}

	function _create(options?: Options): Promise<void>;
	function _create(canvas: HTMLCanvasElement, options?: Options): Promise<void>;
	function _reset(): void;

	const confetti: CreateTypes;
	export = confetti;
}
