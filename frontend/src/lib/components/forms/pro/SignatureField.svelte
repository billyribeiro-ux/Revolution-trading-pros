<script lang="ts">
	/**
	 * SignatureField Component (FluentForms 6.1.5 - November 2025)
	 *
	 * Digital signature capture field with canvas drawing support.
	 * Works in both regular forms and conversational forms.
	 */

	interface Props {
		name: string;
		label?: string;
		required?: boolean;
		disabled?: boolean;
		width?: number;
		height?: number;
		penColor?: string;
		penWidth?: number;
		backgroundColor?: string;
		showClearButton?: boolean;
		showUndoButton?: boolean;
		error?: string;
		onchange?: (dataUrl: string | null) => void;
	}

	let {
		name,
		label = 'Signature',
		required = false,
		disabled = false,
		width = 400,
		height = 200,
		penColor = '#000000',
		penWidth = 2,
		backgroundColor = '#ffffff',
		showClearButton = true,
		showUndoButton = true,
		error = '',
		onchange
	}: Props = $props();

	let canvas: HTMLCanvasElement;
	let ctx: CanvasRenderingContext2D | null = null;
	let isDrawing = $state(false);
	let hasSignature = $state(false);
	let signatureData = $state<string | null>(null);
	let paths = $state<{ x: number; y: number }[][]>([]);
	let currentPath = $state<{ x: number; y: number }[]>([]);

	$effect(() => {
		if (canvas) {
			ctx = canvas.getContext('2d');
			if (ctx) {
				ctx.strokeStyle = penColor;
				ctx.lineWidth = penWidth;
				ctx.lineCap = 'round';
				ctx.lineJoin = 'round';
				clearCanvas();
			}
		}
	});

	function getPosition(e: MouseEvent | TouchEvent): { x: number; y: number } {
		const rect = canvas.getBoundingClientRect();
		const scaleX = canvas.width / rect.width;
		const scaleY = canvas.height / rect.height;

		if ('touches' in e) {
			const touch = e.touches[0];
			return {
				x: (touch.clientX - rect.left) * scaleX,
				y: (touch.clientY - rect.top) * scaleY
			};
		} else {
			return {
				x: (e.clientX - rect.left) * scaleX,
				y: (e.clientY - rect.top) * scaleY
			};
		}
	}

	function startDrawing(e: MouseEvent | TouchEvent) {
		if (disabled) return;
		e.preventDefault();

		isDrawing = true;
		currentPath = [];

		const pos = getPosition(e);
		currentPath.push(pos);

		if (ctx) {
			ctx.beginPath();
			ctx.moveTo(pos.x, pos.y);
		}
	}

	function draw(e: MouseEvent | TouchEvent) {
		if (!isDrawing || disabled || !ctx) return;
		e.preventDefault();

		const pos = getPosition(e);
		currentPath.push(pos);

		ctx.lineTo(pos.x, pos.y);
		ctx.stroke();
	}

	function stopDrawing() {
		if (!isDrawing) return;

		isDrawing = false;

		if (currentPath.length > 0) {
			paths = [...paths, currentPath];
			currentPath = [];
			hasSignature = true;
			saveSignature();
		}
	}

	function saveSignature() {
		if (canvas) {
			signatureData = canvas.toDataURL('image/png');
			if (onchange) {
				onchange(signatureData);
			}
		}
	}

	function clearCanvas() {
		if (ctx && canvas) {
			ctx.fillStyle = backgroundColor;
			ctx.fillRect(0, 0, canvas.width, canvas.height);
			ctx.strokeStyle = penColor;
			paths = [];
			currentPath = [];
			hasSignature = false;
			signatureData = null;
			if (onchange) {
				onchange(null);
			}
		}
	}

	function undoLastStroke() {
		if (paths.length === 0 || !ctx) return;

		paths = paths.slice(0, -1);
		redrawCanvas();

		if (paths.length === 0) {
			hasSignature = false;
			signatureData = null;
			if (onchange) {
				onchange(null);
			}
		} else {
			saveSignature();
		}
	}

	function redrawCanvas() {
		if (!ctx || !canvas) return;

		ctx.fillStyle = backgroundColor;
		ctx.fillRect(0, 0, canvas.width, canvas.height);
		ctx.strokeStyle = penColor;
		ctx.lineWidth = penWidth;

		for (const path of paths) {
			if (path.length === 0) continue;

			ctx.beginPath();
			ctx.moveTo(path[0].x, path[0].y);

			for (let i = 1; i < path.length; i++) {
				ctx.lineTo(path[i].x, path[i].y);
			}

			ctx.stroke();
		}
	}
</script>

<div class="signature-field" class:disabled class:has-error={error}>
	<label for="signature-{name}" class="signature-label">
		{label}
		{#if required}
			<span class="required-marker">*</span>
		{/if}
	</label>

	<div class="signature-pad-container" style="max-width: {width}px">
		<canvas
			bind:this={canvas}
			{width}
			{height}
			class="signature-canvas"
			class:has-signature={hasSignature}
			onmousedown={startDrawing}
			onmousemove={draw}
			onmouseup={stopDrawing}
			onmouseleave={stopDrawing}
			ontouchstart={startDrawing}
			ontouchmove={draw}
			ontouchend={stopDrawing}
		></canvas>

		{#if !hasSignature && !disabled}
			<div class="signature-placeholder">
				<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
					<path d="M12 19l7-7 3 3-7 7-3-3z"></path>
					<path d="M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z"></path>
					<path d="M2 2l7.586 7.586"></path>
					<circle cx="11" cy="11" r="2"></circle>
				</svg>
				<span>Sign here</span>
			</div>
		{/if}

		<div class="signature-actions">
			{#if showUndoButton}
				<button
					type="button"
					class="action-btn undo"
					onclick={undoLastStroke}
					disabled={disabled || paths.length === 0}
					title="Undo"
					aria-label="Undo last stroke"
				>
					<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M3 7v6h6"></path>
						<path d="M21 17a9 9 0 00-9-9 9 9 0 00-6 2.3L3 13"></path>
					</svg>
				</button>
			{/if}
			{#if showClearButton}
				<button
					type="button"
					class="action-btn clear"
					onclick={clearCanvas}
					disabled={disabled || !hasSignature}
					title="Clear"
					aria-label="Clear signature"
				>
					<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<polyline points="3 6 5 6 21 6"></polyline>
						<path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
					</svg>
				</button>
			{/if}
		</div>
	</div>

	<input id="signature-{name}" type="hidden" {name} value={signatureData || ''} />

	{#if error}
		<div class="error-message">
			<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<circle cx="12" cy="12" r="10"></circle>
				<line x1="12" y1="8" x2="12" y2="12"></line>
				<line x1="12" y1="16" x2="12.01" y2="16"></line>
			</svg>
			{error}
		</div>
	{/if}

	<p class="signature-hint">Draw your signature using mouse or touch</p>
</div>

<style>
	.signature-field {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.signature-label {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #374151;
	}

	.required-marker {
		color: #ef4444;
		margin-left: 0.25rem;
	}

	.signature-pad-container {
		position: relative;
		width: 100%;
	}

	.signature-canvas {
		display: block;
		width: 100%;
		height: auto;
		border: 2px dashed #d1d5db;
		border-radius: 0.5rem;
		background-color: #ffffff;
		cursor: crosshair;
		touch-action: none;
	}

	.signature-canvas.has-signature {
		border-style: solid;
		border-color: #3b82f6;
	}

	.signature-placeholder {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		color: #9ca3af;
		pointer-events: none;
	}

	.signature-placeholder span {
		font-size: 0.875rem;
	}

	.signature-actions {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		display: flex;
		gap: 0.25rem;
	}

	.action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background-color: white;
		border: 1px solid #e5e7eb;
		border-radius: 0.375rem;
		color: #6b7280;
		cursor: pointer;
		transition: all 0.15s;
	}

	.action-btn:hover:not(:disabled) {
		background-color: #f3f4f6;
		color: #374151;
	}

	.action-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.action-btn.clear:hover:not(:disabled) {
		background-color: #fef2f2;
		border-color: #fca5a5;
		color: #dc2626;
	}

	.signature-hint {
		margin: 0;
		font-size: 0.75rem;
		color: #9ca3af;
	}

	/* Disabled State */
	.disabled .signature-canvas {
		cursor: not-allowed;
		opacity: 0.6;
	}

	/* Error State */
	.has-error .signature-canvas {
		border-color: #fca5a5;
	}

	.error-message {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #dc2626;
		font-size: 0.8125rem;
	}

	/* Responsive */
	@media (max-width: 480px) {
		.signature-canvas {
			height: 150px;
		}
	}
</style>
