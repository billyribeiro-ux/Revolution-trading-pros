<script lang="ts">
	/**
	 * SEO Analyzer - Enterprise-Grade SEO Analysis
	 * ==============================================
	 * Real-time SEO scoring with comprehensive analysis
	 * for title, meta, content, readability, and more.
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */

	import type { SEOAnalysis } from './types';

	interface Props {
		title: string;
		content: string;
		metaDescription?: string;
		focusKeyword?: string;
		slug?: string;
		onAnalysisComplete?: (analysis: SEOAnalysis) => void;
	}

	let {
		title,
		content,
		metaDescription = '',
		focusKeyword = '',
		slug = '',
		onAnalysisComplete
	}: Props = $props();

	// Analysis state
	let isAnalyzing = $state(false);
	let analysis = $state<SEOAnalysis | null>(null);
	let expandedCategories = $state<Set<string>>(new Set(['overall', 'title', 'content']));

	// Analyze content when inputs change
	$effect(() => {
		const timeout = setTimeout(() => {
			runAnalysis();
		}, 500);
		return () => clearTimeout(timeout);
	});

	// Run SEO analysis
	async function runAnalysis(): Promise<void> {
		isAnalyzing = true;

		// Simulate analysis delay for realistic UX
		await new Promise(resolve => setTimeout(resolve, 300));

		const issues: SEOAnalysis['issues'] = [];
		const suggestions: string[] = [];
		let overallScore = 100;

		// Extract plain text from content
		const plainText = stripHtml(content);
		const wordCount = countWords(plainText);
	const sentences = countSentences(plainText);

		// ===================
		// Title Analysis
		// ===================
		const titleScore = analyzeTitleSEO(title, focusKeyword, issues, suggestions);

		// ===================
		// Meta Description Analysis
		// ===================
		const metaScore = analyzeMetaSEO(metaDescription, focusKeyword, issues, suggestions);

		// ===================
		// Content Analysis
		// ===================
		const contentScore = analyzeContentSEO(plainText, wordCount, focusKeyword, issues, suggestions);

		// ===================
		// Readability Analysis
		// ===================
		const readabilityScore = analyzeReadability(plainText, sentences, wordCount, issues, suggestions);

		// ===================
		// Keyword Analysis
		// ===================
		const keywordScore = analyzeKeywords(plainText, title, metaDescription, focusKeyword, issues, suggestions);

		// ===================
		// Slug Analysis
		// ===================
		analyzeSlug(slug, focusKeyword, issues, suggestions);

		// ===================
		// Structure Analysis
		// ===================
		analyzeStructure(content, issues, suggestions);

		// Calculate overall score
		overallScore = Math.round(
			(titleScore * 0.2) +
			(metaScore * 0.15) +
			(contentScore * 0.25) +
			(readabilityScore * 0.2) +
			(keywordScore * 0.2)
		);

		// Ensure score is within bounds
		overallScore = Math.max(0, Math.min(100, overallScore));

		analysis = {
			score: overallScore,
			issues,
			suggestions,
			wordCount,
			readingTime: Math.ceil(wordCount / 200),
			keywordDensity: focusKeyword ? calculateKeywordDensity(plainText, focusKeyword) : 0,
			readabilityScore: getReadabilityGrade(readabilityScore),
			titleScore,
			metaScore,
			contentScore,
			readabilityGrade: readabilityScore
		};

		isAnalyzing = false;
		onAnalysisComplete?.(analysis);
	}

	// Strip HTML tags
	function stripHtml(html: string): string {
		return html.replace(/<[^>]*>/g, ' ').replace(/\s+/g, ' ').trim();
	}

	// Count words
	function countWords(text: string): number {
		return text.split(/\s+/).filter(word => word.length > 0).length;
	}

	// Count sentences
	function countSentences(text: string): number {
		return (text.match(/[.!?]+/g) || []).length || 1;
	}

	// Calculate keyword density
	function calculateKeywordDensity(text: string, keyword: string): number {
		if (!keyword) return 0;
		const words = countWords(text);
		const keywordRegex = new RegExp(keyword.toLowerCase(), 'gi');
		const matches = (text.toLowerCase().match(keywordRegex) || []).length;
		return words > 0 ? (matches / words) * 100 : 0;
	}

	// Analyze title SEO
	function analyzeTitleSEO(
		title: string,
		keyword: string,
		issues: SEOAnalysis['issues'],
		suggestions: string[]
	): number {
		let score = 100;
		const titleLength = title.length;

		// Title length check
		if (titleLength === 0) {
			issues.push({ type: 'error', message: 'Page title is missing', category: 'title' });
			score -= 40;
		} else if (titleLength < 30) {
			issues.push({ type: 'warning', message: `Title is too short (${titleLength} chars). Aim for 50-60 characters.`, category: 'title' });
			score -= 20;
		} else if (titleLength > 60) {
			issues.push({ type: 'warning', message: `Title is too long (${titleLength} chars). Keep under 60 characters.`, category: 'title' });
			score -= 15;
		} else {
			issues.push({ type: 'success', message: `Title length is optimal (${titleLength} chars)`, category: 'title' });
		}

		// Keyword in title
		if (keyword && title.toLowerCase().includes(keyword.toLowerCase())) {
			issues.push({ type: 'success', message: 'Focus keyword found in title', category: 'title' });
		} else if (keyword) {
			issues.push({ type: 'warning', message: 'Focus keyword not found in title', category: 'title' });
			suggestions.push('Add your focus keyword near the beginning of the title');
			score -= 15;
		}

		// Title starts with keyword
		if (keyword && title.toLowerCase().startsWith(keyword.toLowerCase())) {
			issues.push({ type: 'success', message: 'Title starts with focus keyword', category: 'title' });
		}

		// Power words in title
		const powerWords = ['ultimate', 'complete', 'essential', 'proven', 'best', 'top', 'guide', 'how to', 'secrets', 'tips'];
		const hasPowerWord = powerWords.some(word => title.toLowerCase().includes(word));
		if (hasPowerWord) {
			issues.push({ type: 'success', message: 'Title contains power words', category: 'title' });
		} else {
			suggestions.push('Consider adding power words to make your title more compelling');
		}

		return Math.max(0, score);
	}

	// Analyze meta description SEO
	function analyzeMetaSEO(
		meta: string,
		keyword: string,
		issues: SEOAnalysis['issues'],
		suggestions: string[]
	): number {
		let score = 100;
		const metaLength = meta.length;

		if (metaLength === 0) {
			issues.push({ type: 'error', message: 'Meta description is missing', category: 'meta' });
			score -= 40;
		} else if (metaLength < 120) {
			issues.push({ type: 'warning', message: `Meta description is too short (${metaLength} chars). Aim for 150-160 characters.`, category: 'meta' });
			score -= 20;
		} else if (metaLength > 160) {
			issues.push({ type: 'warning', message: `Meta description is too long (${metaLength} chars). Keep under 160 characters.`, category: 'meta' });
			score -= 10;
		} else {
			issues.push({ type: 'success', message: `Meta description length is optimal (${metaLength} chars)`, category: 'meta' });
		}

		// Keyword in meta
		if (keyword && meta.toLowerCase().includes(keyword.toLowerCase())) {
			issues.push({ type: 'success', message: 'Focus keyword found in meta description', category: 'meta' });
		} else if (keyword) {
			issues.push({ type: 'warning', message: 'Focus keyword not found in meta description', category: 'meta' });
			suggestions.push('Include your focus keyword in the meta description');
			score -= 15;
		}

		// Call to action
		const ctaWords = ['learn', 'discover', 'find out', 'get', 'start', 'try', 'read', 'click'];
		const hasCTA = ctaWords.some(word => meta.toLowerCase().includes(word));
		if (hasCTA) {
			issues.push({ type: 'success', message: 'Meta description contains a call to action', category: 'meta' });
		} else {
			suggestions.push('Add a call to action in your meta description');
		}

		return Math.max(0, score);
	}

	// Analyze content SEO
	function analyzeContentSEO(
		text: string,
		wordCount: number,
		keyword: string,
		issues: SEOAnalysis['issues'],
		suggestions: string[]
	): number {
		let score = 100;

		// Word count check
		if (wordCount < 300) {
			issues.push({ type: 'error', message: `Content is too short (${wordCount} words). Aim for at least 1000 words for better rankings.`, category: 'content' });
			score -= 30;
		} else if (wordCount < 1000) {
			issues.push({ type: 'warning', message: `Content is relatively short (${wordCount} words). Consider expanding to 1500+ words.`, category: 'content' });
			score -= 15;
		} else if (wordCount >= 1500) {
			issues.push({ type: 'success', message: `Excellent content length (${wordCount} words)`, category: 'content' });
		} else {
			issues.push({ type: 'success', message: `Good content length (${wordCount} words)`, category: 'content' });
		}

		// Keyword density
		if (keyword) {
			const density = calculateKeywordDensity(text, keyword);
			if (density === 0) {
				issues.push({ type: 'error', message: 'Focus keyword not found in content', category: 'content' });
				score -= 25;
			} else if (density < 0.5) {
				issues.push({ type: 'warning', message: `Keyword density is low (${density.toFixed(1)}%). Aim for 1-2%.`, category: 'content' });
				score -= 10;
			} else if (density > 3) {
				issues.push({ type: 'warning', message: `Keyword density is too high (${density.toFixed(1)}%). This may be seen as keyword stuffing.`, category: 'content' });
				score -= 15;
			} else {
				issues.push({ type: 'success', message: `Good keyword density (${density.toFixed(1)}%)`, category: 'content' });
			}
		}

		// First 100 words
		const first100Words = text.split(/\s+/).slice(0, 100).join(' ').toLowerCase();
		if (keyword && first100Words.includes(keyword.toLowerCase())) {
			issues.push({ type: 'success', message: 'Focus keyword appears in the first 100 words', category: 'content' });
		} else if (keyword) {
			issues.push({ type: 'warning', message: 'Focus keyword does not appear in the first 100 words', category: 'content' });
			suggestions.push('Add your focus keyword within the first paragraph');
			score -= 10;
		}

		return Math.max(0, score);
	}

	// Analyze readability
	function analyzeReadability(
		text: string,
		sentences: number,
		wordCount: number,
		issues: SEOAnalysis['issues'],
		suggestions: string[]
	): number {
		let score = 100;

		// Average sentence length
		const avgSentenceLength = sentences > 0 ? wordCount / sentences : 0;

		if (avgSentenceLength > 25) {
			issues.push({ type: 'warning', message: `Sentences are too long on average (${avgSentenceLength.toFixed(0)} words). Aim for under 20 words.`, category: 'readability' });
			suggestions.push('Break up long sentences for better readability');
			score -= 15;
		} else if (avgSentenceLength > 20) {
			issues.push({ type: 'info', message: `Average sentence length is slightly high (${avgSentenceLength.toFixed(0)} words)`, category: 'readability' });
			score -= 5;
		} else {
			issues.push({ type: 'success', message: `Good average sentence length (${avgSentenceLength.toFixed(0)} words)`, category: 'readability' });
		}

		// Passive voice (simplified check)
		const passivePatterns = /\b(was|were|is|are|been|being)\s+\w+ed\b/gi;
		const passiveCount = (text.match(passivePatterns) || []).length;
		const passivePercentage = sentences > 0 ? (passiveCount / sentences) * 100 : 0;

		if (passivePercentage > 20) {
			issues.push({ type: 'warning', message: `Too much passive voice detected (~${passivePercentage.toFixed(0)}%)`, category: 'readability' });
			suggestions.push('Use more active voice for engaging content');
			score -= 10;
		} else if (passivePercentage > 10) {
			issues.push({ type: 'info', message: `Some passive voice detected (~${passivePercentage.toFixed(0)}%)`, category: 'readability' });
		} else {
			issues.push({ type: 'success', message: 'Good use of active voice', category: 'readability' });
		}

		// Transition words (simplified)
		const transitionWords = ['however', 'therefore', 'furthermore', 'additionally', 'moreover', 'consequently', 'thus', 'hence', 'meanwhile', 'nevertheless', 'although', 'because', 'since', 'while', 'whereas'];
		const transitionCount = transitionWords.filter(word =>
			text.toLowerCase().includes(word)
		).length;

		if (transitionCount < 3 && wordCount > 300) {
			issues.push({ type: 'warning', message: 'Content lacks transition words', category: 'readability' });
			suggestions.push('Add transition words to improve content flow');
			score -= 10;
		} else {
			issues.push({ type: 'success', message: 'Good use of transition words', category: 'readability' });
		}

		return Math.max(0, score);
	}

	// Analyze keywords
	function analyzeKeywords(
		text: string,
		_title: string,
		_meta: string,
		keyword: string,
		issues: SEOAnalysis['issues'],
		suggestions: string[]
	): number {
		let score = 100;

		if (!keyword) {
			issues.push({ type: 'warning', message: 'No focus keyword set', category: 'keyword' });
			suggestions.push('Set a focus keyword to optimize your content');
			return 50;
		}

		// Check keyword variations
		const keywordWords = keyword.toLowerCase().split(/\s+/);
		let variationsFound = 0;

		keywordWords.forEach(word => {
			if (text.toLowerCase().includes(word)) {
				variationsFound++;
			}
		});

		if (variationsFound === keywordWords.length) {
			issues.push({ type: 'success', message: 'All keyword variations found in content', category: 'keyword' });
		} else {
			issues.push({ type: 'info', message: `${variationsFound}/${keywordWords.length} keyword variations found`, category: 'keyword' });
		}

		// Related keywords (LSI)
		suggestions.push('Consider adding related keywords and synonyms (LSI keywords)');

		// Keyword in subheadings check (would need HTML parsing)
		const hasKeywordInH2 = text.toLowerCase().includes(keyword.toLowerCase());
		if (!hasKeywordInH2) {
			suggestions.push('Add your focus keyword to at least one subheading (H2)');
			score -= 10;
		}

		return Math.max(0, score);
	}

	// Analyze URL slug
	function analyzeSlug(
		slug: string,
		keyword: string,
		issues: SEOAnalysis['issues'],
		suggestions: string[]
	): void {
		if (!slug) {
			issues.push({ type: 'info', message: 'URL slug not yet set', category: 'slug' });
			return;
		}

		// Slug length
		if (slug.length > 75) {
			issues.push({ type: 'warning', message: 'URL slug is too long. Keep under 75 characters.', category: 'slug' });
		} else {
			issues.push({ type: 'success', message: 'URL slug length is good', category: 'slug' });
		}

		// Keyword in slug
		if (keyword && slug.toLowerCase().includes(keyword.toLowerCase().replace(/\s+/g, '-'))) {
			issues.push({ type: 'success', message: 'Focus keyword found in URL slug', category: 'slug' });
		} else if (keyword) {
			issues.push({ type: 'warning', message: 'Focus keyword not found in URL slug', category: 'slug' });
			suggestions.push('Include your focus keyword in the URL slug');
		}

		// Stop words in slug
		const stopWords = ['a', 'an', 'the', 'and', 'or', 'but', 'in', 'on', 'at', 'to', 'for'];
		const hasStopWords = stopWords.some(word =>
			slug.split('-').includes(word)
		);
		if (hasStopWords) {
			suggestions.push('Consider removing stop words from your URL slug');
		}
	}

	// Analyze content structure
	function analyzeStructure(
		html: string,
		issues: SEOAnalysis['issues'],
		suggestions: string[]
	): void {
		// Headings hierarchy
		const h2Count = (html.match(/<h2[^>]*>/gi) || []).length;

		if (h2Count === 0) {
			issues.push({ type: 'warning', message: 'No H2 subheadings found', category: 'structure' });
			suggestions.push('Add H2 subheadings to improve content structure');
		} else if (h2Count < 3) {
			issues.push({ type: 'info', message: `Only ${h2Count} H2 subheading(s) found`, category: 'structure' });
		} else {
			issues.push({ type: 'success', message: `Good number of H2 subheadings (${h2Count})`, category: 'structure' });
		}

		// Images check
		const imgCount = (html.match(/<img[^>]*>/gi) || []).length;
		const altCount = (html.match(/<img[^>]*alt="[^"]+"/gi) || []).length;

		if (imgCount === 0) {
			issues.push({ type: 'warning', message: 'No images found in content', category: 'structure' });
			suggestions.push('Add relevant images to make content more engaging');
		} else if (altCount < imgCount) {
			issues.push({ type: 'warning', message: `${imgCount - altCount} image(s) missing alt text`, category: 'structure' });
			suggestions.push('Add descriptive alt text to all images');
		} else {
			issues.push({ type: 'success', message: 'All images have alt text', category: 'structure' });
		}

		// Links check
		const internalLinks = (html.match(/<a[^>]*href="\/[^"]*"/gi) || []).length;
		const externalLinks = (html.match(/<a[^>]*href="https?:\/\/[^"]*"/gi) || []).length;

		if (internalLinks === 0) {
			suggestions.push('Add internal links to other relevant content');
		} else {
			issues.push({ type: 'success', message: `${internalLinks} internal link(s) found`, category: 'structure' });
		}

		if (externalLinks === 0) {
			suggestions.push('Consider adding external links to authoritative sources');
		} else {
			issues.push({ type: 'success', message: `${externalLinks} external link(s) found`, category: 'structure' });
		}
	}

	// Get readability grade label
	function getReadabilityGrade(score: number): string {
		if (score >= 90) return 'Very Easy';
		if (score >= 80) return 'Easy';
		if (score >= 70) return 'Fairly Easy';
		if (score >= 60) return 'Standard';
		if (score >= 50) return 'Fairly Difficult';
		if (score >= 30) return 'Difficult';
		return 'Very Difficult';
	}

	// Get score color
	function getScoreColor(score: number): string {
		if (score >= 80) return '#10b981';
		if (score >= 60) return '#f59e0b';
		return '#ef4444';
	}

	// Toggle category expansion
	function toggleCategory(category: string): void {
		if (expandedCategories.has(category)) {
			expandedCategories.delete(category);
		} else {
			expandedCategories.add(category);
		}
		expandedCategories = new Set(expandedCategories);
	}

	// Get issues by category
	function getIssuesByCategory(category: string): SEOAnalysis['issues'] {
		return analysis?.issues.filter(i => i.category === category) || [];
	}

	// Get icon for issue type
	function getIssueIcon(type: string): string {
		switch (type) {
			case 'error': return '✗';
			case 'warning': return '⚠';
			case 'success': return '✓';
			case 'info': return 'ℹ';
			default: return '•';
		}
	}
</script>

<div class="seo-analyzer">
	{#if isAnalyzing}
		<div class="analyzing">
			<div class="spinner"></div>
			<span>Analyzing...</span>
		</div>
	{:else if analysis}
		<!-- Overall Score -->
		<div class="score-section">
			<div class="score-circle" style="--score-color: {getScoreColor(analysis.score)}">
				<svg viewBox="0 0 100 100">
					<circle
						class="score-bg"
						cx="50"
						cy="50"
						r="45"
					/>
					<circle
						class="score-progress"
						cx="50"
						cy="50"
						r="45"
						stroke-dasharray="{analysis.score * 2.83} 283"
						style="stroke: {getScoreColor(analysis.score)}"
					/>
				</svg>
				<div class="score-value">
					<span class="score-number">{analysis.score}</span>
					<span class="score-label">SEO Score</span>
				</div>
			</div>
			<div class="score-summary">
				<div class="summary-item">
					<span class="summary-label">Words</span>
					<span class="summary-value">{analysis.wordCount}</span>
				</div>
				<div class="summary-item">
					<span class="summary-label">Reading Time</span>
					<span class="summary-value">{analysis.readingTime} min</span>
				</div>
				<div class="summary-item">
					<span class="summary-label">Readability</span>
					<span class="summary-value">{analysis.readabilityScore}</span>
				</div>
				{#if focusKeyword}
					<div class="summary-item">
						<span class="summary-label">Keyword Density</span>
						<span class="summary-value">{analysis.keywordDensity.toFixed(1)}%</span>
					</div>
				{/if}
			</div>
		</div>

		<!-- Analysis Categories -->
		<div class="categories">
			<!-- Title Analysis -->
			<div class="category">
				<button
					class="category-header"
					onclick={() => toggleCategory('title')}
				>
					<div class="category-info">
						<span class="category-name">Title</span>
						<span class="category-score" style="color: {getScoreColor(analysis.titleScore || 0)}">
							{analysis.titleScore || 0}%
						</span>
					</div>
					<span class="chevron" class:expanded={expandedCategories.has('title')}>▼</span>
				</button>
				{#if expandedCategories.has('title')}
					<div class="category-content">
						{#each getIssuesByCategory('title') as issue}
							<div class="issue {issue.type}">
								<span class="issue-icon">{getIssueIcon(issue.type)}</span>
								<span class="issue-message">{issue.message}</span>
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<!-- Meta Description Analysis -->
			<div class="category">
				<button
					class="category-header"
					onclick={() => toggleCategory('meta')}
				>
					<div class="category-info">
						<span class="category-name">Meta Description</span>
						<span class="category-score" style="color: {getScoreColor(analysis.metaScore || 0)}">
							{analysis.metaScore || 0}%
						</span>
					</div>
					<span class="chevron" class:expanded={expandedCategories.has('meta')}>▼</span>
				</button>
				{#if expandedCategories.has('meta')}
					<div class="category-content">
						{#each getIssuesByCategory('meta') as issue}
							<div class="issue {issue.type}">
								<span class="issue-icon">{getIssueIcon(issue.type)}</span>
								<span class="issue-message">{issue.message}</span>
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<!-- Content Analysis -->
			<div class="category">
				<button
					class="category-header"
					onclick={() => toggleCategory('content')}
				>
					<div class="category-info">
						<span class="category-name">Content</span>
						<span class="category-score" style="color: {getScoreColor(analysis.contentScore || 0)}">
							{analysis.contentScore || 0}%
						</span>
					</div>
					<span class="chevron" class:expanded={expandedCategories.has('content')}>▼</span>
				</button>
				{#if expandedCategories.has('content')}
					<div class="category-content">
						{#each getIssuesByCategory('content') as issue}
							<div class="issue {issue.type}">
								<span class="issue-icon">{getIssueIcon(issue.type)}</span>
								<span class="issue-message">{issue.message}</span>
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<!-- Readability Analysis -->
			<div class="category">
				<button
					class="category-header"
					onclick={() => toggleCategory('readability')}
				>
					<div class="category-info">
						<span class="category-name">Readability</span>
						<span class="category-score" style="color: {getScoreColor(analysis.readabilityGrade || 0)}">
							{analysis.readabilityGrade || 0}%
						</span>
					</div>
					<span class="chevron" class:expanded={expandedCategories.has('readability')}>▼</span>
				</button>
				{#if expandedCategories.has('readability')}
					<div class="category-content">
						{#each getIssuesByCategory('readability') as issue}
							<div class="issue {issue.type}">
								<span class="issue-icon">{getIssueIcon(issue.type)}</span>
								<span class="issue-message">{issue.message}</span>
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<!-- Keyword Analysis -->
			<div class="category">
				<button
					class="category-header"
					onclick={() => toggleCategory('keyword')}
				>
					<div class="category-info">
						<span class="category-name">Keywords</span>
					</div>
					<span class="chevron" class:expanded={expandedCategories.has('keyword')}>▼</span>
				</button>
				{#if expandedCategories.has('keyword')}
					<div class="category-content">
						{#each getIssuesByCategory('keyword') as issue}
							<div class="issue {issue.type}">
								<span class="issue-icon">{getIssueIcon(issue.type)}</span>
								<span class="issue-message">{issue.message}</span>
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<!-- Structure Analysis -->
			<div class="category">
				<button
					class="category-header"
					onclick={() => toggleCategory('structure')}
				>
					<div class="category-info">
						<span class="category-name">Structure</span>
					</div>
					<span class="chevron" class:expanded={expandedCategories.has('structure')}>▼</span>
				</button>
				{#if expandedCategories.has('structure')}
					<div class="category-content">
						{#each getIssuesByCategory('structure') as issue}
							<div class="issue {issue.type}">
								<span class="issue-icon">{getIssueIcon(issue.type)}</span>
								<span class="issue-message">{issue.message}</span>
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<!-- URL Slug Analysis -->
			{#if slug}
				<div class="category">
					<button
						class="category-header"
						onclick={() => toggleCategory('slug')}
					>
						<div class="category-info">
							<span class="category-name">URL Slug</span>
						</div>
						<span class="chevron" class:expanded={expandedCategories.has('slug')}>▼</span>
					</button>
					{#if expandedCategories.has('slug')}
						<div class="category-content">
							{#each getIssuesByCategory('slug') as issue}
								<div class="issue {issue.type}">
									<span class="issue-icon">{getIssueIcon(issue.type)}</span>
									<span class="issue-message">{issue.message}</span>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			{/if}
		</div>

		<!-- Suggestions -->
		{#if analysis.suggestions.length > 0}
			<div class="suggestions">
				<h4>Improvement Suggestions</h4>
				<ul>
					{#each analysis.suggestions as suggestion}
						<li>{suggestion}</li>
					{/each}
				</ul>
			</div>
		{/if}
	{:else}
		<div class="empty-state">
			<p>Start writing to see SEO analysis</p>
		</div>
	{/if}
</div>

<style>
	.seo-analyzer {
		padding: 1rem;
	}

	.analyzing {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		padding: 2rem;
		color: var(--text-secondary, #6b7280);
	}

	.spinner {
		width: 20px;
		height: 20px;
		border: 2px solid var(--border-color, #e5e7eb);
		border-top-color: var(--primary, #3b82f6);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.score-section {
		display: flex;
		gap: 1.5rem;
		margin-bottom: 1.5rem;
		padding-bottom: 1.5rem;
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.score-circle {
		position: relative;
		width: 100px;
		height: 100px;
		flex-shrink: 0;
	}

	.score-circle svg {
		transform: rotate(-90deg);
	}

	.score-bg {
		fill: none;
		stroke: var(--border-color, #e5e7eb);
		stroke-width: 8;
	}

	.score-progress {
		fill: none;
		stroke-width: 8;
		stroke-linecap: round;
		transition: stroke-dasharray 0.5s ease;
	}

	.score-value {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		text-align: center;
	}

	.score-number {
		display: block;
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary, #1f2937);
	}

	.score-label {
		display: block;
		font-size: 0.625rem;
		color: var(--text-secondary, #6b7280);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.score-summary {
		flex: 1;
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 0.75rem;
	}

	.summary-item {
		display: flex;
		flex-direction: column;
	}

	.summary-label {
		font-size: 0.625rem;
		color: var(--text-secondary, #6b7280);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.summary-value {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
	}

	.categories {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.category {
		border: 1px solid var(--border-color, #e5e7eb);
		border-radius: 0.5rem;
		overflow: hidden;
	}

	.category-header {
		width: 100%;
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1rem;
		background: var(--bg-secondary, #f9fafb);
		border: none;
		cursor: pointer;
		transition: background 0.2s;
	}

	.category-header:hover {
		background: var(--bg-hover, #f3f4f6);
	}

	.category-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.category-name {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-primary, #1f2937);
	}

	.category-score {
		font-size: 0.75rem;
		font-weight: 600;
	}

	.chevron {
		font-size: 0.75rem;
		color: var(--text-secondary, #6b7280);
		transition: transform 0.2s;
	}

	.chevron.expanded {
		transform: rotate(180deg);
	}

	.category-content {
		padding: 0.75rem 1rem;
		border-top: 1px solid var(--border-color, #e5e7eb);
	}

	.issue {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
		padding: 0.5rem 0;
		font-size: 0.8125rem;
	}

	.issue:not(:last-child) {
		border-bottom: 1px solid var(--border-color, #e5e7eb);
	}

	.issue-icon {
		flex-shrink: 0;
		width: 16px;
		height: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 50%;
		font-size: 0.625rem;
		font-weight: 700;
	}

	.issue.error .issue-icon {
		background: #fef2f2;
		color: #dc2626;
	}

	.issue.warning .issue-icon {
		background: #fffbeb;
		color: #d97706;
	}

	.issue.success .issue-icon {
		background: #ecfdf5;
		color: #059669;
	}

	.issue.info .issue-icon {
		background: #eff6ff;
		color: #2563eb;
	}

	.issue-message {
		flex: 1;
		color: var(--text-secondary, #6b7280);
		line-height: 1.4;
	}

	.issue.error .issue-message {
		color: #dc2626;
	}

	.issue.warning .issue-message {
		color: #d97706;
	}

	.issue.success .issue-message {
		color: #059669;
	}

	.suggestions {
		margin-top: 1.5rem;
		padding: 1rem;
		background: #eff6ff;
		border-radius: 0.5rem;
	}

	.suggestions h4 {
		margin: 0 0 0.75rem;
		font-size: 0.875rem;
		font-weight: 600;
		color: #1e40af;
	}

	.suggestions ul {
		margin: 0;
		padding-left: 1.25rem;
	}

	.suggestions li {
		font-size: 0.8125rem;
		color: #1e40af;
		line-height: 1.5;
		margin-bottom: 0.375rem;
	}

	.suggestions li:last-child {
		margin-bottom: 0;
	}

	.empty-state {
		padding: 2rem;
		text-align: center;
		color: var(--text-tertiary, #9ca3af);
		font-size: 0.875rem;
	}
</style>
