/**
 * Video Upload API - Real File Upload Handler
 *
 * Handles video file uploads with support for:
 * - Direct uploads
 * - Presigned URL generation for S3/cloud storage
 * - Thumbnail generation
 * - Progress tracking
 *
 * @version 1.0.0 - December 2025
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { writeFile, mkdir } from 'fs/promises';
import { existsSync } from 'fs';
import path from 'path';

// Upload configuration
const UPLOAD_DIR = 'static/uploads/videos';
const THUMBNAIL_DIR = 'static/uploads/thumbnails';
const MAX_FILE_SIZE = 500 * 1024 * 1024; // 500MB
const ALLOWED_VIDEO_TYPES = ['video/mp4', 'video/webm', 'video/quicktime', 'video/x-msvideo'];
const ALLOWED_IMAGE_TYPES = ['image/jpeg', 'image/png', 'image/webp', 'image/gif'];

// Upload tracking
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
export const POST: RequestHandler = async ({ request, url }) => {
	const action = url.searchParams.get('action');

	// Request presigned URL for cloud upload
	if (action === 'presign') {
		return handlePresignRequest(request);
	}

	// Request upload session
	if (action === 'init') {
		return handleUploadInit(request);
	}

	// Direct file upload
	return handleDirectUpload(request);
};

// GET - Check upload status
export const GET: RequestHandler = async ({ url }) => {
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

// Handle presigned URL request (for S3-compatible storage)
async function handlePresignRequest(request: Request) {
	try {
		const body = await request.json();
		const { filename, content_type, size } = body;

		if (!filename || !content_type) {
			throw error(400, 'Filename and content_type are required');
		}

		if (size && size > MAX_FILE_SIZE) {
			throw error(400, `File size exceeds maximum of ${MAX_FILE_SIZE / 1024 / 1024}MB`);
		}

		// In production, generate actual presigned URL from S3/R2/etc
		// For now, return a mock presigned URL structure
		const uploadId = `upload_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
		const key = `videos/${uploadId}/${filename}`;

		// Create upload session
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
				presigned_url: `/api/videos/upload?session_id=${uploadId}`,
				key,
				fields: {
					'Content-Type': content_type
				},
				expires_in: 3600 // 1 hour
			}
		});
	} catch (err) {
		if (err instanceof Error && 'status' in err) {
			throw err;
		}
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

// Handle direct file upload
async function handleDirectUpload(request: Request) {
	try {
		const formData = await request.formData();
		const file = formData.get('file') as File | null;
		const type = formData.get('type') as string || 'video'; // 'video' or 'thumbnail'
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

		// Determine upload directory
		const uploadDir = type === 'thumbnail' ? THUMBNAIL_DIR : UPLOAD_DIR;

		// Ensure directory exists
		if (!existsSync(uploadDir)) {
			await mkdir(uploadDir, { recursive: true });
		}

		// Generate unique filename
		const ext = path.extname(file.name);
		const uploadId = sessionId || `${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
		const filename = `${uploadId}${ext}`;
		const filePath = path.join(uploadDir, filename);

		// Update session status
		if (sessionId) {
			const session = uploadSessions.get(sessionId);
			if (session) {
				session.status = 'uploading';
				uploadSessions.set(sessionId, session);
			}
		}

		// Convert file to buffer and write
		const arrayBuffer = await file.arrayBuffer();
		const buffer = Buffer.from(arrayBuffer);
		await writeFile(filePath, buffer);

		// Generate public URL
		const publicUrl = `/${uploadDir}/${filename}`;

		// Update session
		if (sessionId) {
			const session = uploadSessions.get(sessionId);
			if (session) {
				session.status = 'complete';
				session.uploaded = file.size;
				if (type === 'thumbnail') {
					session.thumbnail_url = publicUrl;
				} else {
					session.video_url = publicUrl;
				}
				uploadSessions.set(sessionId, session);
			}
		}

		return json({
			success: true,
			data: {
				upload_id: uploadId,
				filename,
				size: file.size,
				type: file.type,
				url: publicUrl,
				status: 'complete'
			}
		});
	} catch (err) {
		if (err instanceof Error && 'status' in err) {
			throw err;
		}
		console.error('Upload error:', err);
		throw error(500, 'Failed to upload file');
	}
}
