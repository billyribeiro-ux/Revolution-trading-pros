import { readFileSync } from 'fs';
import { join } from 'path';

export async function GET() {
	try {
		// Read the HTML file from Implementation directory
		const htmlPath = join(process.cwd(), 'Implementation', 'tax-loss-harvest');
		const html = readFileSync(htmlPath, 'utf-8');
		
		// Serve the raw HTML exactly as-is with all CSS, styles, backgrounds, and scripts
		return new Response(html, {
			headers: {
				'Content-Type': 'text/html; charset=utf-8',
				'Cache-Control': 'public, max-age=3600'
			}
		});
	} catch (error) {
		console.error('Error loading HTML:', error);
		return new Response('Page not found', { status: 404 });
	}
}
