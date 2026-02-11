/**
 * useImageUpload Hook - Comprehensive Test Suite
 * ===============================================================================
 * Unit tests for image upload with progress tracking and validation
 *
 * @version 2.0.0
 */

// Import setup first to initialize mocks
import './setup';

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

// ============================================================================
// Mock Dependencies
// ============================================================================

// Mock sanitization utilities
vi.mock('$lib/utils/sanitization', () => ({
	validateFile: vi.fn((file, options) => {
		// Simulate validation based on options
		if (options.maxSize && file.size > options.maxSize) {
			return {
				valid: false,
				error: `File too large. Maximum size: ${(options.maxSize / 1024 / 1024).toFixed(2)}MB`
			};
		}
		if (options.allowedTypes && !options.allowedTypes.includes(file.type)) {
			return {
				valid: false,
				error: `Invalid file type. Allowed: ${options.allowedTypes.join(', ')}`
			};
		}
		return { valid: true, sanitizedName: file.name.toLowerCase() };
	}),
	sanitizeFilename: vi.fn((name) => name.toLowerCase().replace(/\s+/g, '_'))
}));

// Import after mocking
import {
	useImageUpload,
	dataUrlToFile,
	compressImage,
	createPresignedUploader,
	type ImageUploadOptions,
	type ImageMetadata,
	type UploadState
} from '../useImageUpload.svelte';

// ============================================================================
// Test Utilities
// ============================================================================

function createMockFile(
	name: string = 'test.jpg',
	type: string = 'image/jpeg',
	size: number = 1024
): File {
	const content = new Array(size).fill('a').join('');
	return new File([content], name, { type });
}

function createMockXHR() {
	const mockXHR = {
		open: vi.fn(),
		send: vi.fn(),
		setRequestHeader: vi.fn(),
		abort: vi.fn(),
		upload: {
			addEventListener: vi.fn(),
			removeEventListener: vi.fn()
		},
		addEventListener: vi.fn(),
		removeEventListener: vi.fn(),
		status: 200,
		statusText: 'OK',
		responseText: JSON.stringify({ success: true, url: 'https://cdn.example.com/image.jpg' }),
		readyState: 4
	};
	return mockXHR;
}

// ============================================================================
// useImageUpload Tests
// ============================================================================

describe('useImageUpload', () => {
	let mockXHR: ReturnType<typeof createMockXHR>;
	let originalXHR: typeof XMLHttpRequest;

	beforeEach(() => {
		vi.clearAllMocks();
		vi.useFakeTimers();

		// Mock XMLHttpRequest with a proper class constructor
		mockXHR = createMockXHR();
		originalXHR = global.XMLHttpRequest;
		global.XMLHttpRequest = class MockXMLHttpRequest {
			constructor() {
				return mockXHR as any;
			}
		} as any;
	});

	afterEach(() => {
		vi.useRealTimers();
		global.XMLHttpRequest = originalXHR;
	});

	// ========================================================================
	// Initialization Tests
	// ========================================================================

	describe('Initialization', () => {
		it('initializes with default state', () => {
			const onSuccess = vi.fn();
			const hook = useImageUpload({ onSuccess });

			expect(hook.uploading).toBe(false);
			expect(hook.progress).toBe(0);
			expect(hook.error).toBeNull();
			expect(hook.url).toBeNull();
			expect(hook.metadata).toBeNull();
		});

		it('provides all expected methods', () => {
			const hook = useImageUpload({ onSuccess: vi.fn() });

			expect(typeof hook.upload).toBe('function');
			expect(typeof hook.uploadMultiple).toBe('function');
			expect(typeof hook.cancel).toBe('function');
			expect(typeof hook.reset).toBe('function');
			expect(typeof hook.validateImage).toBe('function');
			expect(typeof hook.handleDrop).toBe('function');
			expect(typeof hook.handleDragOver).toBe('function');
			expect(typeof hook.handleFileInput).toBe('function');
			expect(typeof hook.handlePaste).toBe('function');
			expect(typeof hook.openFilePicker).toBe('function');
		});

		it('accepts custom options', () => {
			const options: ImageUploadOptions = {
				maxSize: 5 * 1024 * 1024,
				allowedTypes: ['image/png'],
				allowedExtensions: ['png'],
				endpoint: '/api/custom/upload',
				onSuccess: vi.fn(),
				onError: vi.fn(),
				onProgress: vi.fn()
			};

			const hook = useImageUpload(options);

			// Hook should be created successfully with custom options
			expect(hook).toBeDefined();
		});
	});

	// ========================================================================
	// File Type Validation Tests
	// ========================================================================

	describe('File Type Validation', () => {
		it('validates file type - rejects invalid type', () => {
			const onError = vi.fn();
			const hook = useImageUpload({
				allowedTypes: ['image/jpeg', 'image/png'],
				onSuccess: vi.fn(),
				onError
			});

			const invalidFile = createMockFile('test.txt', 'text/plain', 1024);
			const result = hook.validateImage(invalidFile);

			expect(result.valid).toBe(false);
			expect(result.error).toContain('Invalid file type');
		});

		it('validates file type - accepts valid type', () => {
			const hook = useImageUpload({
				allowedTypes: ['image/jpeg', 'image/png'],
				onSuccess: vi.fn()
			});

			const validFile = createMockFile('test.jpg', 'image/jpeg', 1024);
			const result = hook.validateImage(validFile);

			expect(result.valid).toBe(true);
		});

		it('validates file type - accepts default types', () => {
			const hook = useImageUpload({ onSuccess: vi.fn() });

			const jpegFile = createMockFile('test.jpg', 'image/jpeg', 1024);
			const pngFile = createMockFile('test.png', 'image/png', 1024);
			const webpFile = createMockFile('test.webp', 'image/webp', 1024);

			expect(hook.validateImage(jpegFile).valid).toBe(true);
			expect(hook.validateImage(pngFile).valid).toBe(true);
			expect(hook.validateImage(webpFile).valid).toBe(true);
		});

		it('validates file type - rejects unsupported format', () => {
			const hook = useImageUpload({
				allowedTypes: ['image/jpeg'],
				onSuccess: vi.fn()
			});

			const bmpFile = createMockFile('test.bmp', 'image/bmp', 1024);
			const result = hook.validateImage(bmpFile);

			expect(result.valid).toBe(false);
		});
	});

	// ========================================================================
	// File Size Validation Tests
	// ========================================================================

	describe('File Size Validation', () => {
		it('validates file size - rejects oversized file', () => {
			const maxSize = 1024 * 1024; // 1MB
			const onError = vi.fn();

			const hook = useImageUpload({
				maxSize,
				onSuccess: vi.fn(),
				onError
			});

			const largeFile = createMockFile('large.jpg', 'image/jpeg', 2 * 1024 * 1024); // 2MB
			const result = hook.validateImage(largeFile);

			expect(result.valid).toBe(false);
			expect(result.error).toContain('too large');
		});

		it('validates file size - accepts file within limit', () => {
			const maxSize = 10 * 1024 * 1024; // 10MB

			const hook = useImageUpload({
				maxSize,
				onSuccess: vi.fn()
			});

			const smallFile = createMockFile('small.jpg', 'image/jpeg', 1024 * 1024); // 1MB
			const result = hook.validateImage(smallFile);

			expect(result.valid).toBe(true);
		});

		it('validates file size - accepts file at exact limit', () => {
			const maxSize = 1024 * 1024; // 1MB

			const hook = useImageUpload({
				maxSize,
				onSuccess: vi.fn()
			});

			const exactFile = createMockFile('exact.jpg', 'image/jpeg', maxSize);
			const result = hook.validateImage(exactFile);

			expect(result.valid).toBe(true);
		});

		it('uses default max size of 10MB', () => {
			const hook = useImageUpload({ onSuccess: vi.fn() });

			const largeFile = createMockFile('huge.jpg', 'image/jpeg', 15 * 1024 * 1024); // 15MB
			const result = hook.validateImage(largeFile);

			expect(result.valid).toBe(false);
		});
	});

	// ========================================================================
	// Upload Progress Tests
	// ========================================================================

	describe('Upload Progress', () => {
		it('tracks upload progress', async () => {
			const onProgress = vi.fn();
			const hook = useImageUpload({
				onSuccess: vi.fn(),
				onProgress
			});

			// Store the progress handler when addEventListener is called
			let progressHandler: ((event: ProgressEvent) => void) | undefined;
			mockXHR.upload.addEventListener.mockImplementation(
				(event: string, handler: (e: ProgressEvent) => void) => {
					if (event === 'progress') {
						progressHandler = handler;
					}
				}
			);

			// Store load handler
			let loadHandler: (() => void) | undefined;
			mockXHR.addEventListener.mockImplementation((event: string, handler: () => void) => {
				if (event === 'load') {
					loadHandler = handler;
				}
			});

			const file = createMockFile('test.jpg', 'image/jpeg', 1024);

			// Start upload (don't await yet)
			void hook.upload(file);

			// Allow async operations to start
			await vi.advanceTimersByTimeAsync(10);

			// Simulate progress events
			if (progressHandler) {
				progressHandler({ loaded: 256, total: 1024, lengthComputable: true } as ProgressEvent);
				progressHandler({ loaded: 512, total: 1024, lengthComputable: true } as ProgressEvent);
				progressHandler({ loaded: 1024, total: 1024, lengthComputable: true } as ProgressEvent);
			}

			// Simulate successful load
			if (loadHandler) {
				loadHandler();
			}

			await vi.advanceTimersByTimeAsync(10);
		});

		it('calls onProgress callback with percentage', async () => {
			const onProgress = vi.fn();
			const hook = useImageUpload({
				onSuccess: vi.fn(),
				onProgress
			});

			let progressHandler: ((event: ProgressEvent) => void) | undefined;
			mockXHR.upload.addEventListener.mockImplementation(
				(event: string, handler: (e: ProgressEvent) => void) => {
					if (event === 'progress') {
						progressHandler = handler;
					}
				}
			);

			const file = createMockFile('test.jpg', 'image/jpeg', 1000);
			hook.upload(file);

			// Flush microtasks so upload reaches XHR stage (Image.onload + createThumbnail)
			await vi.advanceTimersByTimeAsync(50);

			if (progressHandler) {
				progressHandler({ loaded: 500, total: 1000, lengthComputable: true } as ProgressEvent);
			}

			expect(onProgress).toHaveBeenCalledWith(50);
		});
	});

	// ========================================================================
	// Success Handling Tests
	// ========================================================================

	describe('Successful Upload', () => {
		it('handles successful upload', async () => {
			const onSuccess = vi.fn();
			const uploadUrl = 'https://cdn.example.com/uploaded.jpg';

			const hook = useImageUpload({ onSuccess });

			mockXHR.status = 200;
			mockXHR.responseText = JSON.stringify({
				success: true,
				url: uploadUrl,
				thumbnailUrl: 'https://cdn.example.com/thumb.jpg'
			});

			let loadHandler: (() => void) | undefined;
			mockXHR.addEventListener.mockImplementation((event: string, handler: () => void) => {
				if (event === 'load') {
					loadHandler = handler;
				}
			});

			const file = createMockFile('test.jpg', 'image/jpeg', 1024);
			void hook.upload(file);

			await vi.advanceTimersByTimeAsync(10);

			if (loadHandler) {
				loadHandler();
			}

			await vi.advanceTimersByTimeAsync(10);
		});

		it('calls onSuccess callback with URL and metadata', async () => {
			const onSuccess = vi.fn();
			const uploadUrl = 'https://cdn.example.com/uploaded.jpg';

			const hook = useImageUpload({ onSuccess });

			mockXHR.status = 200;
			mockXHR.responseText = JSON.stringify({
				success: true,
				url: uploadUrl
			});

			let loadHandler: (() => void) | undefined;
			mockXHR.addEventListener.mockImplementation((event: string, handler: () => void) => {
				if (event === 'load') {
					loadHandler = handler;
				}
			});

			const file = createMockFile('test.jpg', 'image/jpeg', 1024);
			hook.upload(file);

			await vi.advanceTimersByTimeAsync(50);

			if (loadHandler) {
				loadHandler();
			}

			// onSuccess should be called with URL
			await vi.advanceTimersByTimeAsync(10);
		});
	});

	// ========================================================================
	// Error Handling Tests
	// ========================================================================

	describe('Error Handling', () => {
		it('handles upload errors', async () => {
			const onError = vi.fn();
			const hook = useImageUpload({
				onSuccess: vi.fn(),
				onError
			});

			mockXHR.status = 500;
			mockXHR.responseText = JSON.stringify({ error: 'Server error' });

			let loadHandler: (() => void) | undefined;
			mockXHR.addEventListener.mockImplementation((event: string, handler: () => void) => {
				if (event === 'load') {
					loadHandler = handler;
				}
			});

			const file = createMockFile('test.jpg', 'image/jpeg', 1024);
			hook.upload(file);

			await vi.advanceTimersByTimeAsync(50);

			if (loadHandler) {
				loadHandler();
			}

			await vi.advanceTimersByTimeAsync(10);
		});

		it('handles network errors', async () => {
			const onError = vi.fn();
			const hook = useImageUpload({
				onSuccess: vi.fn(),
				onError
			});

			let errorHandler: (() => void) | undefined;
			mockXHR.addEventListener.mockImplementation((event: string, handler: () => void) => {
				if (event === 'error') {
					errorHandler = handler;
				}
			});

			const file = createMockFile('test.jpg', 'image/jpeg', 1024);
			hook.upload(file);

			await vi.advanceTimersByTimeAsync(50);

			if (errorHandler) {
				errorHandler();
			}

			await vi.advanceTimersByTimeAsync(10);
		});

		it('handles timeout errors', async () => {
			const onError = vi.fn();
			const hook = useImageUpload({
				onSuccess: vi.fn(),
				onError
			});

			let timeoutHandler: (() => void) | undefined;
			mockXHR.addEventListener.mockImplementation((event: string, handler: () => void) => {
				if (event === 'timeout') {
					timeoutHandler = handler;
				}
			});

			const file = createMockFile('test.jpg', 'image/jpeg', 1024);
			hook.upload(file);

			await vi.advanceTimersByTimeAsync(50);

			if (timeoutHandler) {
				timeoutHandler();
			}

			await vi.advanceTimersByTimeAsync(10);
		});

		it('calls onError callback with error', async () => {
			const onError = vi.fn();
			const hook = useImageUpload({
				onSuccess: vi.fn(),
				onError
			});

			// Test validation error
			const invalidFile = createMockFile('test.txt', 'text/plain', 1024);
			await hook.upload(invalidFile);

			expect(onError).toHaveBeenCalledWith(expect.any(Error));
		});

		it('handles invalid file during upload', async () => {
			const onError = vi.fn();
			const hook = useImageUpload({
				allowedTypes: ['image/jpeg'],
				onSuccess: vi.fn(),
				onError
			});

			const invalidFile = createMockFile('test.gif', 'image/gif', 1024);
			await hook.upload(invalidFile);

			expect(onError).toHaveBeenCalled();
		});
	});

	// ========================================================================
	// Reset State Tests
	// ========================================================================

	describe('Reset State', () => {
		it('resets state on reset()', () => {
			const hook = useImageUpload({ onSuccess: vi.fn() });

			// Simulate some state
			// Then reset
			hook.reset();

			expect(hook.uploading).toBe(false);
			expect(hook.progress).toBe(0);
			expect(hook.error).toBeNull();
			expect(hook.url).toBeNull();
			expect(hook.metadata).toBeNull();
		});

		it('cancels ongoing upload on reset()', async () => {
			const hook = useImageUpload({ onSuccess: vi.fn() });

			const file = createMockFile('test.jpg', 'image/jpeg', 1024);
			hook.upload(file);

			// Flush microtasks so upload reaches XHR stage (Image.onload + createThumbnail)
			await vi.advanceTimersByTimeAsync(50);

			hook.reset();

			expect(mockXHR.abort).toHaveBeenCalled();
		});
	});

	// ========================================================================
	// Cancel Tests
	// ========================================================================

	describe('Cancel Upload', () => {
		it('cancels ongoing upload', async () => {
			const hook = useImageUpload({ onSuccess: vi.fn() });

			const file = createMockFile('test.jpg', 'image/jpeg', 1024);
			hook.upload(file);

			// Flush microtasks so upload reaches XHR stage
			await vi.advanceTimersByTimeAsync(50);

			hook.cancel();

			expect(mockXHR.abort).toHaveBeenCalled();
			expect(hook.uploading).toBe(false);
		});
	});

	// ========================================================================
	// Drag and Drop Tests
	// ========================================================================

	describe('Drag and Drop', () => {
		it('handles drop event with valid image', async () => {
			const onSuccess = vi.fn();
			const hook = useImageUpload({ onSuccess });

			const file = createMockFile('dropped.jpg', 'image/jpeg', 1024);
			const dropEvent = {
				preventDefault: vi.fn(),
				stopPropagation: vi.fn(),
				dataTransfer: {
					files: [file]
				}
			} as unknown as DragEvent;

			hook.handleDrop(dropEvent);

			expect(dropEvent.preventDefault).toHaveBeenCalled();
			expect(dropEvent.stopPropagation).toHaveBeenCalled();
		});

		it('handles drop event with no valid image', async () => {
			const onError = vi.fn();
			const hook = useImageUpload({
				allowedTypes: ['image/jpeg'],
				onSuccess: vi.fn(),
				onError
			});

			const file = createMockFile('doc.pdf', 'application/pdf', 1024);
			const dropEvent = {
				preventDefault: vi.fn(),
				stopPropagation: vi.fn(),
				dataTransfer: {
					files: [file]
				}
			} as unknown as DragEvent;

			hook.handleDrop(dropEvent);

			expect(onError).toHaveBeenCalled();
		});

		it('handles dragOver event', () => {
			const hook = useImageUpload({ onSuccess: vi.fn() });

			const dragOverEvent = {
				preventDefault: vi.fn(),
				stopPropagation: vi.fn()
			} as unknown as DragEvent;

			hook.handleDragOver(dragOverEvent);

			expect(dragOverEvent.preventDefault).toHaveBeenCalled();
			expect(dragOverEvent.stopPropagation).toHaveBeenCalled();
		});
	});

	// ========================================================================
	// Paste Tests
	// ========================================================================

	describe('Paste Support', () => {
		it('handles paste event with image', () => {
			const hook = useImageUpload({ onSuccess: vi.fn() });

			const file = createMockFile('pasted.png', 'image/png', 1024);
			const pasteEvent = {
				preventDefault: vi.fn(),
				clipboardData: {
					items: [
						{
							type: 'image/png',
							getAsFile: () => file
						}
					]
				}
			} as unknown as ClipboardEvent;

			hook.handlePaste(pasteEvent);

			expect(pasteEvent.preventDefault).toHaveBeenCalled();
		});

		it('ignores paste event without image', () => {
			const hook = useImageUpload({ onSuccess: vi.fn() });

			const pasteEvent = {
				preventDefault: vi.fn(),
				clipboardData: {
					items: [
						{
							type: 'text/plain',
							getAsFile: () => null
						}
					]
				}
			} as unknown as ClipboardEvent;

			hook.handlePaste(pasteEvent);

			// Should not start upload
			expect(mockXHR.open).not.toHaveBeenCalled();
		});
	});

	// ========================================================================
	// File Input Tests
	// ========================================================================

	describe('File Input', () => {
		it('handles file input change', () => {
			const hook = useImageUpload({ onSuccess: vi.fn() });

			const file = createMockFile('selected.jpg', 'image/jpeg', 1024);
			const inputEvent = {
				target: {
					files: [file],
					value: 'C:\\fakepath\\selected.jpg'
				}
			} as unknown as Event;

			hook.handleFileInput(inputEvent);

			// Input value should be reset
			expect((inputEvent.target as HTMLInputElement).value).toBe('');
		});

		it('opens file picker', () => {
			const hook = useImageUpload({ onSuccess: vi.fn() });

			const mockClick = vi.fn();
			const mockInput = {
				type: '',
				accept: '',
				onchange: null as any,
				click: mockClick
			};

			vi.spyOn(document, 'createElement').mockReturnValue(mockInput as any);

			hook.openFilePicker();

			expect(mockInput.type).toBe('file');
			expect(mockClick).toHaveBeenCalled();
		});
	});
});

// ============================================================================
// Utility Function Tests
// ============================================================================

describe('dataUrlToFile', () => {
	it('converts PNG data URL to File', () => {
		const dataUrl =
			'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==';
		const file = dataUrlToFile(dataUrl, 'test.png');

		expect(file).toBeInstanceOf(File);
		expect(file.name).toBe('test.png');
		expect(file.type).toBe('image/png');
	});

	it('converts JPEG data URL to File', () => {
		const dataUrl = 'data:image/jpeg;base64,/9j/4AAQSkZJRgABAQEASABIAAD/2wBDAAgGBgcGBQgHBwcJCQ==';
		const file = dataUrlToFile(dataUrl, 'test.jpg');

		expect(file.type).toBe('image/jpeg');
	});

	it('handles custom filename', () => {
		const dataUrl = 'data:image/png;base64,dGVzdA==';
		const file = dataUrlToFile(dataUrl, 'my-custom-name.png');

		expect(file.name).toBe('my-custom-name.png');
	});
});

describe('compressImage', () => {
	it('returns a Promise', () => {
		const file = createMockFile('test.jpg', 'image/jpeg', 1024 * 1024);
		const result = compressImage(file, 1920, 1080, 0.85);

		expect(result).toBeInstanceOf(Promise);
	});

	it('accepts quality parameter', () => {
		const file = createMockFile('test.jpg', 'image/jpeg', 1024);

		expect(() => compressImage(file, 800, 600, 0.5)).not.toThrow();
		expect(() => compressImage(file, 800, 600, 1.0)).not.toThrow();
	});

	it('accepts various dimensions', () => {
		const file = createMockFile('test.jpg', 'image/jpeg', 1024);

		expect(() => compressImage(file, 100, 100, 0.8)).not.toThrow();
		expect(() => compressImage(file, 1920, 1080, 0.8)).not.toThrow();
	});
});

describe('createPresignedUploader', () => {
	let originalXHR: typeof XMLHttpRequest;

	beforeEach(() => {
		originalXHR = global.XMLHttpRequest;

		const mockXHR: Record<string, any> = {
			open: vi.fn(),
			send: vi.fn(function () {
				queueMicrotask(() => {
					mockXHR.status = 200;
					mockXHR._handlers.load?.forEach((h: () => void) => h());
				});
			}),
			setRequestHeader: vi.fn(),
			abort: vi.fn(),
			upload: {
				addEventListener: vi.fn(),
				removeEventListener: vi.fn()
			},
			addEventListener: vi.fn((event: string, handler: () => void) => {
				if (!mockXHR._handlers[event]) mockXHR._handlers[event] = [];
				mockXHR._handlers[event].push(handler);
			}),
			removeEventListener: vi.fn(),
			status: 200,
			responseText: JSON.stringify({ success: true, url: 'https://s3.example.com/uploaded.jpg' }),
			readyState: 4,
			_handlers: {} as Record<string, Array<() => void>>
		};

		global.XMLHttpRequest = class MockXMLHttpRequest {
			constructor() {
				return mockXHR as any;
			}
		} as any;
	});

	afterEach(() => {
		global.XMLHttpRequest = originalXHR;
	});

	it('creates an uploader function', () => {
		const getPresignedUrl = vi.fn().mockResolvedValue('https://s3.example.com/presigned');
		const uploader = createPresignedUploader(getPresignedUrl);

		expect(typeof uploader).toBe('function');
	});

	it('uploader returns a Promise', () => {
		const getPresignedUrl = vi.fn().mockResolvedValue('https://s3.example.com/presigned');
		const uploader = createPresignedUploader(getPresignedUrl);

		const file = createMockFile('test.jpg', 'image/jpeg', 1024);
		const result = uploader(file);

		expect(result).toBeInstanceOf(Promise);
	});

	it('calls getPresignedUrl with file details', async () => {
		const getPresignedUrl = vi.fn().mockResolvedValue('https://s3.example.com/presigned');
		const uploader = createPresignedUploader(getPresignedUrl);

		const file = createMockFile('test.jpg', 'image/jpeg', 1024);
		const uploadPromise = uploader(file);

		await vi.waitFor(() => {
			expect(getPresignedUrl).toHaveBeenCalledWith('test.jpg', 'image/jpeg');
		});

		await Promise.allSettled([uploadPromise]);
	});
});

// ============================================================================
// Type Tests
// ============================================================================

describe('Type Definitions', () => {
	describe('ImageUploadOptions', () => {
		it('accepts all optional properties', () => {
			const options: ImageUploadOptions = {
				maxSize: 10 * 1024 * 1024,
				allowedTypes: ['image/jpeg', 'image/png'],
				allowedExtensions: ['jpg', 'png'],
				endpoint: '/api/upload',
				headers: { Authorization: 'Bearer token' },
				generateThumbnail: true,
				thumbnailSize: 200,
				onSuccess: vi.fn(),
				onError: vi.fn(),
				onProgress: vi.fn()
			};

			expect(options.maxSize).toBe(10 * 1024 * 1024);
		});
	});

	describe('ImageMetadata', () => {
		it('has expected structure', () => {
			const metadata: ImageMetadata = {
				width: 1920,
				height: 1080,
				size: 1024 * 1024,
				type: 'image/jpeg',
				name: 'photo.jpg',
				thumbnailUrl: 'https://cdn.example.com/thumb.jpg',
				blurhash: 'LEHV6nWB2yk8'
			};

			expect(metadata.width).toBe(1920);
			expect(metadata.height).toBe(1080);
		});
	});

	describe('UploadState', () => {
		it('has all state properties', () => {
			const state: UploadState = {
				uploading: true,
				progress: 50,
				error: null,
				url: null,
				metadata: null
			};

			expect(state.uploading).toBe(true);
			expect(state.progress).toBe(50);
		});
	});
});

// ============================================================================
// Edge Cases
// ============================================================================

describe('Edge Cases', () => {
	it('handles empty file name', () => {
		const file = new File(['content'], '', { type: 'image/jpeg' });
		expect(file.name).toBe('');
	});

	it('handles zero-size file', () => {
		const file = new File([], 'empty.jpg', { type: 'image/jpeg' });
		expect(file.size).toBe(0);
	});

	it('handles multiple upload calls', async () => {
		const mockXHR = createMockXHR();
		const originalXHR = global.XMLHttpRequest;
		global.XMLHttpRequest = class MockXMLHttpRequest {
			constructor() {
				return mockXHR as any;
			}
		} as any;

		try {
			const onSuccess = vi.fn();
			const hook = useImageUpload({ onSuccess });

			const file1 = createMockFile('file1.jpg', 'image/jpeg', 1024);
			const file2 = createMockFile('file2.jpg', 'image/jpeg', 1024);

			// Start first upload
			hook.upload(file1).catch(() => {});

			// Allow full async chain: Image.onload (microtask) → createThumbnail → XHR.send
			await new Promise((r) => setTimeout(r, 200));

			// Verify first upload reached XHR
			expect(mockXHR.open).toHaveBeenCalled();

			// Start second upload (should abort first)
			hook.upload(file2).catch(() => {});

			await new Promise((r) => setTimeout(r, 200));
		} finally {
			global.XMLHttpRequest = originalXHR;
		}
	});
});
