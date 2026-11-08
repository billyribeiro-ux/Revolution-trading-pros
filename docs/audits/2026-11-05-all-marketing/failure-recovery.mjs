import { chromium } from '../../../frontend/node_modules/playwright/index.mjs';
import { execFileSync } from 'node:child_process';
import { writeFile } from 'node:fs/promises';
const origin = process.env.AUDIT_ORIGIN;
if (origin !== 'http://127.0.0.1:4185' || process.env.MARKETING_PUBLIC_FIXTURE !== 'published')
	throw Error('Owned fixture and preview required');
const pid = Number(
	execFileSync('lsof', ['-t', '-iTCP:19085', '-sTCP:LISTEN'], { encoding: 'utf8' }).trim()
);
const command = execFileSync('ps', ['-p', String(pid), '-o', 'comm='], { encoding: 'utf8' }).trim();
if (!command.endsWith('/marketing_public_fixture'))
	throw Error('Refuse to pause any other service');
const browser = await chromium.launch();
const page = await browser.newPage();
const result = {};
let stopped = false;
try {
	process.kill(pid, 'SIGSTOP');
	stopped = true;
	await page.goto(origin + '/blog', { timeout: 20000 });
	result.blogUnavailable = await page.locator('.error').textContent();
	if (!result.blogUnavailable.includes('temporarily unavailable'))
		throw Error('Blog concealed outage');
	const response = await page.goto(origin + '/tutorials', { timeout: 30000 });
	result.tutorialStatus = response.status();
	if (response.status() !== 503) throw Error('Video catalog concealed outage');
	process.kill(pid, 'SIGCONT');
	stopped = false;
	await page.goto(origin + '/blog');
	await page.locator('.post-card').first().waitFor();
	result.recoveredPosts = await page.locator('.post-card').count();
	await page.goto(origin + '/tutorials');
	await page
		.getByRole('link', { name: /Marketing fixture educational video/ })
		.first()
		.waitFor();
	result.recoveredVideo = true;
	const missing = await page.goto(origin + '/learning-center/time-frames-in-tos');
	result.removedPlaceholderStatus = missing.status();
	if (missing.status() !== 404) throw Error('Unknown video did not return 404');
	await writeFile(
		new URL('./artifacts/failure-recovery.json', import.meta.url),
		JSON.stringify(result, null, 2)
	);
	console.log(result);
} finally {
	if (stopped) process.kill(pid, 'SIGCONT');
	await browser.close();
}
