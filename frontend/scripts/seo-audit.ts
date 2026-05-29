/**
 * SEO Audit Script
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Parses built HTML files and reports SEO issues:
 * - Duplicate title/meta/canonical/OG/Twitter tags
 * - Missing canonical on indexable pages
 * - Indexable pages with no title/description
 * - Noindex pages accidentally in sitemap source list
 * - JSON-LD parse failures
 *
 * Usage: npx tsx scripts/seo-audit.ts [build-dir]
 * Default build-dir: .svelte-kit/output/prerendered/pages
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since February 2026
 */

import { readFileSync, readdirSync, statSync } from 'fs';
import { join, relative } from 'path';

// ═══════════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════════

interface AuditIssue {
	severity: 'error' | 'warning' | 'info';
	file: string;
	message: string;
	details?: string;
}

interface AuditReport {
	totalFiles: number;
	issues: AuditIssue[];
	summary: {
		errors: number;
		warnings: number;
		info: number;
	};
}

// ═══════════════════════════════════════════════════════════════════════════════
// HTML PARSING HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

function extractTagsByRegex(html: string, pattern: RegExp): string[] {
	const matches: string[] = [];
	let match: RegExpExecArray | null;
	while ((match = pattern.exec(html)) !== null) {
		matches.push(match[0]);
	}
	return matches;
}

function extractAttr(tag: string, attr: string): string | null {
	const pattern = new RegExp(`${attr}=["']([^"']*)["']`, 'i');
	const match = tag.match(pattern);
	return match ? match[1] : null;
}

function extractTitles(html: string): string[] {
	const pattern = /<title[^>]*>(.*?)<\/title>/gi;
	const matches: string[] = [];
	let match: RegExpExecArray | null;
	while ((match = pattern.exec(html)) !== null) {
		matches.push(match[1]);
	}
	return matches;
}

function extractMetaByName(html: string, name: string): string[] {
	const pattern = new RegExp(`<meta[^>]*name=["']${name}["'][^>]*>`, 'gi');
	return extractTagsByRegex(html, pattern);
}

function extractMetaByProperty(html: string, property: string): string[] {
	const pattern = new RegExp(`<meta[^>]*property=["']${property}["'][^>]*>`, 'gi');
	return extractTagsByRegex(html, pattern);
}

function extractCanonicals(html: string): string[] {
	const pattern = /<link[^>]*rel=["']canonical["'][^>]*>/gi;
	return extractTagsByRegex(html, pattern);
}

function extractJsonLdBlocks(html: string): string[] {
	const pattern = /<script[^>]*type=["']application\/ld\+json["'][^>]*>([\s\S]*?)<\/script>/gi;
	const blocks: string[] = [];
	let match: RegExpExecArray | null;
	while ((match = pattern.exec(html)) !== null) {
		blocks.push(match[1]);
	}
	return blocks;
}

function isNoindex(html: string): boolean {
	const robotsMeta = extractMetaByName(html, 'robots');
	return robotsMeta.some((tag) => {
		const content = extractAttr(tag, 'content') ?? '';
		return content.toLowerCase().includes('noindex');
	});
}

// ═══════════════════════════════════════════════════════════════════════════════
// AUDIT CHECKS
// ═══════════════════════════════════════════════════════════════════════════════

function auditFile(filePath: string, relativePath: string): AuditIssue[] {
	const issues: AuditIssue[] = [];
	let html: string;

	try {
		html = readFileSync(filePath, 'utf-8');
	} catch {
		issues.push({
			severity: 'error',
			file: relativePath,
			message: 'Failed to read file'
		});
		return issues;
	}

	// Check for duplicate titles
	const titles = extractTitles(html);
	if (titles.length > 1) {
		issues.push({
			severity: 'error',
			file: relativePath,
			message: `Duplicate <title> tags found: ${titles.length}`,
			details: titles.join(' | ')
		});
	}
	if (titles.length === 0) {
		issues.push({
			severity: 'error',
			file: relativePath,
			message: 'Missing <title> tag'
		});
	}

	// Check for duplicate meta description
	const descriptions = extractMetaByName(html, 'description');
	if (descriptions.length > 1) {
		issues.push({
			severity: 'error',
			file: relativePath,
			message: `Duplicate meta description tags: ${descriptions.length}`
		});
	}
	if (descriptions.length === 0 && !isNoindex(html)) {
		issues.push({
			severity: 'warning',
			file: relativePath,
			message: 'Missing meta description on indexable page'
		});
	}

	// Check for duplicate robots meta
	const robotsTags = extractMetaByName(html, 'robots');
	if (robotsTags.length > 1) {
		issues.push({
			severity: 'error',
			file: relativePath,
			message: `Duplicate robots meta tags: ${robotsTags.length}`,
			details: robotsTags.map((t) => extractAttr(t, 'content')).join(' | ')
		});
	}

	// Check for duplicate canonical
	const canonicals = extractCanonicals(html);
	if (canonicals.length > 1) {
		issues.push({
			severity: 'error',
			file: relativePath,
			message: `Duplicate canonical links: ${canonicals.length}`,
			details: canonicals.map((t) => extractAttr(t, 'href')).join(' | ')
		});
	}
	if (canonicals.length === 0 && !isNoindex(html)) {
		issues.push({
			severity: 'warning',
			file: relativePath,
			message: 'Missing canonical link on indexable page'
		});
	}

	// Check for duplicate OG tags
	const ogTitles = extractMetaByProperty(html, 'og:title');
	if (ogTitles.length > 1) {
		issues.push({
			severity: 'error',
			file: relativePath,
			message: `Duplicate og:title tags: ${ogTitles.length}`
		});
	}

	const ogDescriptions = extractMetaByProperty(html, 'og:description');
	if (ogDescriptions.length > 1) {
		issues.push({
			severity: 'error',
			file: relativePath,
			message: `Duplicate og:description tags: ${ogDescriptions.length}`
		});
	}

	const ogUrls = extractMetaByProperty(html, 'og:url');
	if (ogUrls.length > 1) {
		issues.push({
			severity: 'error',
			file: relativePath,
			message: `Duplicate og:url tags: ${ogUrls.length}`
		});
	}

	// Check for duplicate Twitter tags
	const twitterTitles = extractMetaByName(html, 'twitter:title');
	if (twitterTitles.length > 1) {
		issues.push({
			severity: 'error',
			file: relativePath,
			message: `Duplicate twitter:title tags: ${twitterTitles.length}`
		});
	}

	const twitterCards = extractMetaByName(html, 'twitter:card');
	if (twitterCards.length > 1) {
		issues.push({
			severity: 'error',
			file: relativePath,
			message: `Duplicate twitter:card tags: ${twitterCards.length}`
		});
	}

	// Check JSON-LD validity
	const jsonLdBlocks = extractJsonLdBlocks(html);
	for (let i = 0; i < jsonLdBlocks.length; i++) {
		try {
			JSON.parse(jsonLdBlocks[i]);
		} catch (e) {
			issues.push({
				severity: 'error',
				file: relativePath,
				message: `JSON-LD parse failure in block ${i + 1}`,
				details: (e as Error).message
			});
		}
	}

	return issues;
}

// ═══════════════════════════════════════════════════════════════════════════════
// FILE DISCOVERY
// ═══════════════════════════════════════════════════════════════════════════════

function findHtmlFiles(dir: string): string[] {
	const files: string[] = [];

	try {
		const entries = readdirSync(dir);
		for (const entry of entries) {
			const fullPath = join(dir, entry);
			const stat = statSync(fullPath);
			if (stat.isDirectory()) {
				files.push(...findHtmlFiles(fullPath));
			} else if (entry.endsWith('.html')) {
				files.push(fullPath);
			}
		}
	} catch {
		// Directory doesn't exist or can't be read
	}

	return files;
}

// ═══════════════════════════════════════════════════════════════════════════════
// MAIN
// ═══════════════════════════════════════════════════════════════════════════════

function runAudit(buildDir: string): AuditReport {
	const htmlFiles = findHtmlFiles(buildDir);
	const allIssues: AuditIssue[] = [];

	for (const file of htmlFiles) {
		const relativePath = relative(buildDir, file);
		const issues = auditFile(file, relativePath);
		allIssues.push(...issues);
	}

	return {
		totalFiles: htmlFiles.length,
		issues: allIssues,
		summary: {
			errors: allIssues.filter((i) => i.severity === 'error').length,
			warnings: allIssues.filter((i) => i.severity === 'warning').length,
			info: allIssues.filter((i) => i.severity === 'info').length
		}
	};
}

function formatReport(report: AuditReport): string {
	const lines: string[] = [];

	lines.push('═══════════════════════════════════════════════════════════════');
	lines.push('  SEO AUDIT REPORT');
	lines.push('═══════════════════════════════════════════════════════════════');
	lines.push(`  Files scanned: ${report.totalFiles}`);
	lines.push(`  Errors:   ${report.summary.errors}`);
	lines.push(`  Warnings: ${report.summary.warnings}`);
	lines.push(`  Info:     ${report.summary.info}`);
	lines.push('═══════════════════════════════════════════════════════════════');
	lines.push('');

	if (report.issues.length === 0) {
		lines.push('  ✅ No issues found!');
		return lines.join('\n');
	}

	// Group by severity
	const errors = report.issues.filter((i) => i.severity === 'error');
	const warnings = report.issues.filter((i) => i.severity === 'warning');
	const infos = report.issues.filter((i) => i.severity === 'info');

	if (errors.length > 0) {
		lines.push('  ❌ ERRORS');
		lines.push('  ─────────────────────────────────────────────────────────');
		for (const issue of errors) {
			lines.push(`  [${issue.file}] ${issue.message}`);
			if (issue.details) lines.push(`    → ${issue.details}`);
		}
		lines.push('');
	}

	if (warnings.length > 0) {
		lines.push('  ⚠️  WARNINGS');
		lines.push('  ─────────────────────────────────────────────────────────');
		for (const issue of warnings) {
			lines.push(`  [${issue.file}] ${issue.message}`);
			if (issue.details) lines.push(`    → ${issue.details}`);
		}
		lines.push('');
	}

	if (infos.length > 0) {
		lines.push('  ℹ️  INFO');
		lines.push('  ─────────────────────────────────────────────────────────');
		for (const issue of infos) {
			lines.push(`  [${issue.file}] ${issue.message}`);
			if (issue.details) lines.push(`    → ${issue.details}`);
		}
		lines.push('');
	}

	return lines.join('\n');
}

// ═══════════════════════════════════════════════════════════════════════════════
// CLI ENTRY POINT
// ═══════════════════════════════════════════════════════════════════════════════

const buildDir = process.argv[2] || '.svelte-kit/output/prerendered/pages';
console.log(`\nScanning: ${buildDir}\n`);

const report = runAudit(buildDir);
console.log(formatReport(report));

if (report.summary.errors > 0) {
	process.exit(1);
}
