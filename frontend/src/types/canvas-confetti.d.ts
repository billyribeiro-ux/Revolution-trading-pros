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

	function create(options?: Options): Promise<void>;
	function create(canvas: HTMLCanvasElement, options?: Options): Promise<void>;
	function reset(): void;

	const confetti: CreateTypes;
	export = confetti;
}
