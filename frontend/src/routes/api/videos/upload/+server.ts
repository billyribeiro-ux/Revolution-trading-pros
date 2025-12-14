/**
 * Video Upload API - Proxy to Backend
 *
 * Handles video file uploads by proxying to the Laravel backend.
 * The backend handles storage to Cloudflare R2.
 *
 * @version 2.0.0 - Serverless Compatible (Cloudflare)
 */

import { json, error, type RequestEvent } from '@sveltejs/kit';

// API Base URL - use environment variable or fallback
const getApiUrl = () => {
	if (typeof process !== 'undefined' && process.env?.VITE_API_URL) {
		return process.env.VITE_API_URL;
	}
	return 'http://localhost:8000/api';
};

// API Base URL
const API_URL = getApiUrl();

// Upload configuration
const MAX_FILE_SIZE = 500 * 1024 * 1024; // 500MB
const ALLOWED_VIDEO_TYPES = ['video/mp4', 'video/webm', 'video/quicktime', 'video/x-msvideo'];
const ALLOWED_IMAGE_TYPES = ['image/jpeg', 'image/png', 'image/webp', 'image/gif'];

// In-memory upload tracking (for session-based uploads)
interface UploadSession {
	id: string;
	filename: string;
	size: number;
	uploaded: number;
	status: 'pending' | 'uploading' | 'processing' | 'complete' | 'error';
	video_url?: string;
	thumbnail_url?: string;
	error?: string;
	created_at: string;
}

const uploadSessions: Map<string, UploadSession> = new Map();

// POST - Handle file upload or request presigned URL
export const POST = async ({ request, url, cookies }: RequestEvent) => {
	const action = url.searchParams.get('action');

	// Get auth token from cookies
	const token = cookies.get('auth_token');

	// Request presigned URL for cloud upload
	if (action === 'presign') {
		return handlePresignRequest(request, token);
	}

	// Request upload session
	if (action === 'init') {
		return handleUploadInit(request);
	}

	// Direct file upload - proxy to backend
	return handleDirectUpload(request, token);
};

// GET - Check upload status
export const GET = async ({ url }: RequestEvent) => {
	const sessionId = url.searchParams.get('session_id');

	if (sessionId) {
		const session = uploadSessions.get(sessionId);
		if (!session) {
			throw error(404, 'Upload session not found');
		}
		return json({
			success: true,
			data: session
		});
	}

	// List recent uploads
	const sessions = Array.from(uploadSessions.values())
		.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
		.slice(0, 20);

	return json({
		success: true,
		data: sessions
	});
};

// Handle presigned URL request (for S3-compatible storage like R2)
async function handlePresignRequest(request: Request, token?: string) {
	try {
		const body = await request.json();
		const { filename, content_type, size } = body;

		if (!filename || !content_type) {
			throw error(400, 'Filename and content_type are required');
		}

		if (size && size > MAX_FILE_SIZE) {
			throw error(400, `File size exceeds maximum of ${MAX_FILE_SIZE / 1024 / 1024}MB`);
		}

		// Request presigned URL from backend
		const response = await fetch(`${API_URL}/videos/presign`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				...(token ? { Authorization: `Bearer ${token}` } : {})
			},
			body: JSON.stringify({ filename, content_type, size })
		});

		if (!response.ok) {
			const errorData = await response.json().catch(() => ({}));
			throw error(response.status, errorData.message || 'Failed to get presigned URL');
		}

		const data = await response.json();

		// Create local session for tracking
		const uploadId = data.data?.upload_id || `upload_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
		const session: UploadSession = {
			id: uploadId,
			filename,
			size: size || 0,
			uploaded: 0,
			status: 'pending',
			created_at: new Date().toISOString()
		};
		uploadSessions.set(uploadId, session);

		return json(data);
	} catch (err) {
		if (err instanceof Error && 'status' in err) {
			throw err;
		}
		console.error('Presign error:', err);
		throw error(500, 'Failed to generate presigned URL');
	}
}

// Handle upload initialization
async function handleUploadInit(request: Request) {
	try {
		const body = await request.json();
		const { filename, size, content_type } = body;

		if (!filename) {
			throw error(400, 'Filename is required');
		}

		// Validate file type
		const isVideo = ALLOWED_VIDEO_TYPES.includes(content_type);
		const isImage = ALLOWED_IMAGE_TYPES.includes(content_type);

		if (!isVideo && !isImage) {
			throw error(400, 'File type not allowed');
		}

		if (size && size > MAX_FILE_SIZE) {
			throw error(400, `File size exceeds maximum of ${MAX_FILE_SIZE / 1024 / 1024}MB`);
		}

		const uploadId = `upload_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;

		const session: UploadSession = {
			id: uploadId,
			filename,
			size: size || 0,
			uploaded: 0,
			status: 'pending',
			created_at: new Date().toISOString()
		};
		uploadSessions.set(uploadId, session);

		return json({
			success: true,
			data: {
				upload_id: uploadId,
				upload_url: `/api/videos/upload?session_id=${uploadId}`,
				chunk_size: 5 * 1024 * 1024 // 5MB chunks
			}
		});
	} catch (err) {
		if (err instanceof Error && 'status' in err) {
			throw err;
		}
		throw error(500, 'Failed to initialize upload');
	}
}

// Handle direct file upload - proxy to backend
async function handleDirectUpload(request: Request, token?: string) {
	try {
		const formData = await request.formData();
		const file = formData.get('file') as File | null;
		const type = formData.get('type') as string || 'video';
		const sessionId = formData.get('session_id') as string | null;

		if (!file) {
			throw error(400, 'No file provided');
		}

		// Validate file type
		const allowedTypes = type === 'thumbnail' ? ALLOWED_IMAGE_TYPES : ALLOWED_VIDEO_TYPES;
		if (!allowedTypes.includes(file.type)) {
			throw error(400, `Invalid file type: ${file.type}`);
		}

		// Validate file size
		if (file.size > MAX_FILE_SIZE) {
			throw error(400, `File size exceeds maximum of ${MAX_FILE_SIZE / 1024 / 1024}MB`);
		}

		// Update session status
		if (sessionId) {
			const session = uploadSessions.get(sessionId);
			if (session) {
				session.status = 'uploading';
				uploadSessions.set(sessionId, session);
			}
		}

		// Proxy upload to backend
		const backendFormData = new FormData();
		backendFormData.append('file', file);
		backendFormData.append('type', type);
		if (sessionId) {
			backendFormData.append('session_id', sessionId);
		}

		const response = await fetch(`${API_URL}/videos/upload`, {
			method: 'POST',
			headers: {
				...(token ? { Authorization: `Bearer ${token}` } : {})
			},
			body: backendFormData
		});

		if (!response.ok) {
			const errorData = await response.json().catch(() => ({}));

			// Update session with error
			if (sessionId) {
				const session = uploadSessions.get(sessionId);
				if (session) {
					session.status = 'error';
					session.error = errorData.message || 'Upload failed';
					uploadSessions.set(sessionId, session);
				}
			}

			throw error(response.status, errorData.message || 'Upload failed');
		}

		const data = await response.json();

		// Update session with success
		if (sessionId) {
			const session = uploadSessions.get(sessionId);
			if (session) {
				session.status = 'complete';
				session.uploaded = file.size;
				if (type === 'thumbnail') {
					session.thumbnail_url = data.data?.url;
				} else {
					session.video_url = data.data?.url;
				}
				uploadSessions.set(sessionId, session);
			}
		}

		return json(data);
	} catch (err) {
		if (err instanceof Error && 'status' in err) {
			throw err;
		}
		console.error('Upload error:', err);
		throw error(500, 'Failed to upload file');
	}
}
