import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { writeFile, mkdir } from 'fs/promises';
import { existsSync } from 'fs';
import path from 'path';
import crypto from 'crypto';
import { logger } from '$lib/utils/logger';

const UPLOAD_DIR = 'static/uploads/cms';
const MAX_FILE_SIZE = 10 * 1024 * 1024; // 10MB
const ALLOWED_TYPES = ['image/jpeg', 'image/png', 'image/gif', 'image/webp', 'image/avif'];

interface UploadResponse {
	success: boolean;
	url?: string;
	filename?: string;
	size?: number;
	error?: string;
}

export const POST: RequestHandler = async ({ request }) => {
	try {
		const formData = await request.formData();
		const file = formData.get('file') as File | null;

		if (!file) {
			return json({ success: false, error: 'No file provided' } as UploadResponse, { status: 400 });
		}

		// Validate file type
		if (!ALLOWED_TYPES.includes(file.type)) {
			return json(
				{
					success: false,
					error: `Invalid file type. Allowed: ${ALLOWED_TYPES.join(', ')}`
				} as UploadResponse,
				{ status: 400 }
			);
		}

		// Validate file size
		if (file.size > MAX_FILE_SIZE) {
			return json(
				{
					success: false,
					error: `File too large. Maximum size: ${MAX_FILE_SIZE / 1024 / 1024}MB`
				} as UploadResponse,
				{ status: 400 }
			);
		}

		// Generate unique filename
		const ext = path.extname(file.name) || '.jpg';
		const hash = crypto.randomBytes(16).toString('hex');
		const timestamp = Date.now();
		const filename = `${timestamp}-${hash}${ext}`;

		// Ensure upload directory exists
		const uploadPath = path.resolve(UPLOAD_DIR);
		if (!existsSync(uploadPath)) {
			await mkdir(uploadPath, { recursive: true });
		}

		// Write file
		const filePath = path.join(uploadPath, filename);
		const buffer = Buffer.from(await file.arrayBuffer());
		await writeFile(filePath, buffer);

		const response: UploadResponse = {
			success: true,
			url: `/uploads/cms/${filename}`,
			filename: filename,
			size: file.size
		};

		return json(response);
	} catch (error) {
		logger.error('[Image Upload] Error:', error);
		return json({ success: false, error: 'Failed to upload file' } as UploadResponse, {
			status: 500
		});
	}
};

// Also support DELETE for cleanup
export const DELETE: RequestHandler = async ({ url }) => {
	try {
		const filename = url.searchParams.get('filename');

		if (!filename) {
			return json({ success: false, error: 'Filename required' }, { status: 400 });
		}

		// Security: prevent directory traversal
		const sanitizedFilename = path.basename(filename);
		const filePath = path.join(path.resolve(UPLOAD_DIR), sanitizedFilename);

		const { unlink } = await import('fs/promises');
		await unlink(filePath);

		return json({ success: true });
	} catch (error) {
		logger.error('[Image Delete] Error:', error);
		return json({ success: false, error: 'Failed to delete file' }, { status: 500 });
	}
};
