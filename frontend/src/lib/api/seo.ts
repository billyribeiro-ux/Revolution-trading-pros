/**
 * SEO Management Service - Google L7+ Enterprise Implementation
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * ENTERPRISE FEATURES:
 *
 * 1. AI-POWERED ANALYSIS:
 *    - Content optimization
 *    - Keyword research
 *    - Competitor analysis
 *    - SERP tracking
 *    - Topic clustering
 *    - Intent matching
 *
 * 2. TECHNICAL SEO:
 *    - Site auditing
 *    - Core Web Vitals
 *    - Schema markup
 *    - XML sitemaps
 *    - Robots.txt management
 *    - Crawl optimization
 *
 * 3. CONTENT OPTIMIZATION:
 *    - Readability analysis
 *    - Keyword density
 *    - Semantic SEO
 *    - Internal linking
 *    - Content gap analysis
 *    - E-A-T scoring
 *
 * 4. MONITORING & ALERTS:
 *    - Rank tracking
 *    - Backlink monitoring
 *    - 404 detection
 *    - Redirect chains
 *    - Performance metrics
 *    - Algorithm updates
 *
 * 5. REPORTING & INSIGHTS:
 *    - Custom dashboards
 *    - White-label reports
 *    - ROI tracking
 *    - Competitor benchmarking
 *    - Forecast modeling
 *    - Executive summaries
 *
 * @version 3.0.0 (Google L7+ Enterprise)
 * @license MIT
 */

import { writable, derived, get } from 'svelte/store';
import { browser } from '$app/environment';
import { api, type ApiResponse } from './client';

// ═══════════════════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════════════════

// Production fallbacks - NEVER use localhost in production
// NOTE: No /api suffix - endpoints already include /api prefix
const PROD_WS = 'wss://revolution-trading-pros-api.fly.dev';
const PROD_AI = 'https://revolution-trading-pros-api.fly.dev/ai';

const WS_BASE = import.meta.env['VITE_WS_URL'] || PROD_WS;
const AI_API = import.meta.env['VITE_AI_API_URL'] || PROD_AI;

const CACHE_TTL = 300000; // 5 minutes
const ANALYSIS_DEBOUNCE = 2000; // 2 seconds
const RANK_CHECK_INTERVAL = 3600000; // 1 hour
const ALERT_CHECK_INTERVAL = 300000; // 5 minutes

// ICT 7 Principal Engineer Pattern: Feature flag for SEO service
// Set VITE_SEO_ENABLED=true when backend endpoints are ready
const SEO_ENABLED = import.meta.env['VITE_SEO_ENABLED'] === 'true';

// ═══════════════════════════════════════════════════════════════════════════
// Enhanced Type Definitions
// ═══════════════════════════════════════════════════════════════════════════

export interface EnhancedSeoAnalysis extends SeoAnalysis {
	// Core metrics
	overall_score: number;
	technical_score: number;
	content_score: number;
	user_experience_score: number;

	// Analysis data (for compatibility)
	analysis?: {
		overall_score: number;
		technical_score: number;
		content_score: number;
		user_experience_score: number;
		suggestions: Suggestion[];
	};

	// Advanced analysis
	competitor_comparison?: CompetitorAnalysis;
	keyword_opportunities?: KeywordOpportunity[];
	content_gaps?: ContentGap[];
	link_opportunities?: LinkOpportunity[];
	schema_suggestions?: SchemaMarkup[];

	// Performance metrics
	page_speed?: PageSpeedMetrics;
	core_web_vitals?: CoreWebVitals;
	mobile_usability?: MobileUsability;

	// Content quality
	readability?: ReadabilityMetrics;
	semantic_analysis?: SemanticAnalysis;
	topic_relevance?: TopicRelevance;
	eat_score?: EATScore;

	// Technical issues
	technical_issues?: TechnicalIssue[];
	accessibility_issues?: AccessibilityIssue[];
	security_issues?: SecurityIssue[];
}

export interface SeoAnalysis {
	id: number;
	analyzable_type: string;
	analyzable_id: number;
	focus_keyword?: string;
	seo_score: number;
	analysis_results: AnalysisResult[];
	suggestions: Suggestion[];
	keyword_density: number;
	readability_score: number;
	has_meta_title: boolean;
	has_meta_description: boolean;
	created_at: string;
	updated_at?: string;
}

export interface AnalysisResult {
	category: 'technical' | 'content' | 'performance' | 'accessibility';
	type: string;
	status: 'pass' | 'warning' | 'fail';
	message: string;
	impact: 'low' | 'medium' | 'high' | 'critical';
	details?: any;
}

export interface Suggestion {
	priority: 'low' | 'medium' | 'high' | 'critical';
	category: string;
	title: string;
	description: string;
	implementation?: string;
	estimated_impact?: number;
	effort?: 'low' | 'medium' | 'high';
	automated?: boolean;
}

export interface CompetitorAnalysis {
	competitors: Competitor[];
	comparison: ComparisonMetrics;
	opportunities: string[];
	threats: string[];
}

export interface PageMetrics {
	url: string;
	title: string;
	traffic: number;
	keywords: number;
	backlinks: number;
}

export interface Competitor {
	domain: string;
	seo_score: number;
	domain_authority: number;
	traffic_estimate: number;
	ranking_keywords: number;
	backlinks: number;
	top_pages: PageMetrics[];
}

export interface ComparisonMetrics {
	your_score: number;
	average_competitor_score: number;
	percentile: number;
	strengths: string[];
	weaknesses: string[];
}

export interface KeywordOpportunity {
	keyword: string;
	search_volume: number;
	difficulty: number;
	opportunity_score: number;
	current_position?: number;
	potential_traffic: number;
	cpc?: number;
	intent: 'informational' | 'navigational' | 'commercial' | 'transactional';
	serp_features: string[];
}

export interface ContentGap {
	topic: string;
	competitor_coverage: number;
	search_volume: number;
	relevance_score: number;
	suggested_keywords: string[];
	content_outline?: ContentOutline;
}

export interface ContentOutline {
	title: string;
	headings: HeadingStructure[];
	word_count_target: number;
	keywords_to_include: string[];
	questions_to_answer: string[];
}

export interface HeadingStructure {
	level: 1 | 2 | 3 | 4;
	text: string;
	keywords: string[];
}

export interface LinkOpportunity {
	type: 'internal' | 'external';
	source_url?: string;
	target_url: string;
	anchor_text: string;
	relevance_score: number;
	authority_score?: number;
	reason: string;
}

export interface SchemaMarkup {
	type: string;
	properties: Record<string, any>;
	required: string[];
	recommended: string[];
	implementation: string;
	testing_tool_url: string;
}

export interface PageSpeedMetrics {
	score: number;
	fcp: number; // First Contentful Paint
	lcp: number; // Largest Contentful Paint
	fid: number; // First Input Delay
	cls: number; // Cumulative Layout Shift
	ttfb: number; // Time to First Byte
	tti: number; // Time to Interactive
	tbt: number; // Total Blocking Time
	opportunities: SpeedOpportunity[];
}

export interface SpeedOpportunity {
	title: string;
	savings_ms: number;
	savings_bytes?: number;
	implementation: string;
}

export interface CoreWebVitals {
	lcp_status: 'good' | 'needs-improvement' | 'poor';
	fid_status: 'good' | 'needs-improvement' | 'poor';
	cls_status: 'good' | 'needs-improvement' | 'poor';
	overall_status: 'passing' | 'failing';
	field_data?: FieldData;
	lab_data?: LabData;
}

export interface FieldData {
	origin: string;
	data_points: number;
	percentile_75: CoreWebVitalMetrics;
}

export interface LabData {
	device: 'mobile' | 'desktop';
	metrics: CoreWebVitalMetrics;
}

export interface CoreWebVitalMetrics {
	lcp: number;
	fid: number;
	cls: number;
}

export interface MobileUsability {
	is_mobile_friendly: boolean;
	viewport_configured: boolean;
	text_readable: boolean;
	targets_sized: boolean;
	issues: MobileIssue[];
}

export interface MobileIssue {
	type: string;
	severity: 'low' | 'medium' | 'high';
	elements_affected: number;
	fix: string;
}

export interface ReadabilityMetrics {
	flesch_score: number;
	flesch_grade: string;
	gunning_fog: number;
	average_sentence_length: number;
	average_word_length: number;
	complex_words_percentage: number;
	passive_voice_percentage: number;
	transition_words_percentage: number;
}

export interface SemanticAnalysis {
	main_entities: Entity[];
	topics: Topic[];
	sentiment: Sentiment;
	related_searches: string[];
	people_also_ask: string[];
}

export interface Entity {
	name: string;
	type: string;
	salience: number;
	wikipedia_url?: string;
}

export interface Topic {
	name: string;
	relevance: number;
	keywords: string[];
}

export interface Sentiment {
	score: number;
	magnitude: number;
	label: 'positive' | 'neutral' | 'negative';
}

export interface TopicRelevance {
	main_topic: string;
	relevance_score: number;
	supporting_topics: string[];
	missing_topics: string[];
	topic_coverage: number;
}

export interface EATScore {
	expertise: number;
	authoritativeness: number;
	trustworthiness: number;
	overall: number;
	factors: EATFactor[];
	improvements: string[];
}

export interface EATFactor {
	name: string;
	score: number;
	weight: number;
	status: 'good' | 'needs-improvement' | 'poor';
}

export interface TechnicalIssue {
	type: string;
	severity: 'low' | 'medium' | 'high' | 'critical';
	affected_pages: number;
	description: string;
	solution: string;
	effort: 'low' | 'medium' | 'high';
}

export interface AccessibilityIssue {
	wcag_criteria: string;
	level: 'A' | 'AA' | 'AAA';
	type: string;
	elements_affected: number;
	impact: string;
	fix: string;
}

export interface SecurityIssue {
	type: string;
	severity: 'low' | 'medium' | 'high' | 'critical';
	description: string;
	recommendation: string;
	cve?: string;
}

export interface Redirect {
	id: number;
	from_path: string;
	to_path: string;
	status_code: 301 | 302 | 307 | 308;
	is_active: boolean;
	type: 'manual' | 'automatic' | 'imported';
	hit_count: number;
	last_accessed?: string;
	chain?: RedirectChain;
	created_at: string;
	updated_at?: string;
}

export interface RedirectChain {
	length: number;
	paths: string[];
	total_latency: number;
	has_loop: boolean;
}

export interface Error404 {
	id: number;
	url: string;
	hit_count: number;
	is_resolved: boolean;
	referrers: string[];
	user_agents: Record<string, number>;
	suggested_redirect?: string;
	first_seen_at: string;
	last_seen_at: string;
}

export interface SitemapConfig {
	id: string;
	type: 'pages' | 'posts' | 'products' | 'categories' | 'custom';
	enabled: boolean;
	frequency: 'always' | 'hourly' | 'daily' | 'weekly' | 'monthly' | 'yearly' | 'never';
	priority: number;
	include_images: boolean;
	include_videos: boolean;
	max_entries: number;
	conditions?: SitemapCondition[];
}

export interface SitemapCondition {
	field: string;
	operator: 'equals' | 'not_equals' | 'contains' | 'greater_than' | 'less_than';
	value: any;
}

export interface LocalBusiness {
	name: string;
	description: string;
	address: Address;
	phone: string;
	email: string;
	website: string;
	hours: BusinessHours[];
	category: string;
	price_range?: string;
	images: string[];
	social_profiles: SocialProfile[];
	attributes: Record<string, any>;
}

export interface Address {
	street: string;
	city: string;
	state: string;
	postal_code: string;
	country: string;
	latitude?: number;
	longitude?: number;
}

export interface BusinessHours {
	day: string;
	open: string;
	close: string;
	is_closed: boolean;
}

export interface SocialProfile {
	platform: string;
	url: string;
	username?: string;
}

export interface RankTracking {
	keyword: string;
	position: number;
	previous_position?: number;
	change: number;
	url: string;
	search_volume: number;
	device: 'desktop' | 'mobile';
	location: string;
	serp_features: string[];
	competitors: CompetitorRank[];
	history: RankHistory[];
}

export interface CompetitorRank {
	domain: string;
	position: number;
	url: string;
}

export interface RankHistory {
	date: string;
	position: number;
	url: string;
}

export interface BacklinkProfile {
	total_backlinks: number;
	referring_domains: number;
	domain_authority: number;
	trust_flow: number;
	citation_flow: number;
	new_backlinks_30d: number;
	lost_backlinks_30d: number;
	toxic_backlinks: number;
	anchor_text_distribution: Record<string, number>;
	top_referring_domains: ReferringDomain[];
	recent_backlinks: Backlink[];
}

export interface ReferringDomain {
	domain: string;
	authority: number;
	backlinks: number;
	first_seen: string;
	last_seen: string;
}

export interface Backlink {
	url: string;
	anchor_text: string;
	domain_authority: number;
	is_follow: boolean;
	is_image: boolean;
	first_seen: string;
	last_seen: string;
	status: 'active' | 'lost';
}

// ═══════════════════════════════════════════════════════════════════════════
// Core Service Class
// ═══════════════════════════════════════════════════════════════════════════

class SeoManagementService {
	private static instance: SeoManagementService;
	private cache = new Map<string, { data: any; expiry: number }>();
	private wsConnection?: WebSocket;
	private analysisDebounceTimers = new Map<string, number>();
	private rankCheckInterval?: number;
	private alertCheckInterval?: number;
	private pendingAnalyses = new Map<string, Promise<any>>();

	// Stores
	public analyses = writable<EnhancedSeoAnalysis[]>([]);
	public currentAnalysis = writable<EnhancedSeoAnalysis | null>(null);
	public redirects = writable<Redirect[]>([]);
	public errors404 = writable<Error404[]>([]);
	public rankings = writable<RankTracking[]>([]);
	public backlinks = writable<BacklinkProfile | null>(null);
	public competitors = writable<Competitor[]>([]);
	public alerts = writable<SeoAlert[]>([]);
	public isLoading = writable(false);
	public error = writable<string | null>(null);

	// Derived stores
	public criticalIssues = derived(
		this.currentAnalysis,
		($analysis) => $analysis?.technical_issues?.filter((i) => i.severity === 'critical') || []
	);

	public topOpportunities = derived(
		this.currentAnalysis,
		($analysis) =>
			$analysis?.keyword_opportunities
				?.sort((a, b) => b.opportunity_score - a.opportunity_score)
				.slice(0, 10) || []
	);

	public overallScore = derived(this.currentAnalysis, ($analysis) => $analysis?.overall_score || 0);

	private constructor() {
		this.initialize();
	}

	static getInstance(): SeoManagementService {
		if (!SeoManagementService.instance) {
			SeoManagementService.instance = new SeoManagementService();
		}
		return SeoManagementService.instance;
	}

	/**
	 * Initialize service
	 * ICT 7 Principal Engineer Pattern: Feature-flagged initialization
	 * Set VITE_SEO_ENABLED=true in .env when backend endpoints are ready
	 */
	private initialize(): void {
		if (!browser) return;

		if (!SEO_ENABLED) {
			console.debug('[SeoService] Disabled (set VITE_SEO_ENABLED=true to enable)');
			return;
		}

		// Full initialization when feature is enabled
		this.setupWebSocket();
		this.startRankTracking();
		this.startAlertMonitoring();
		this.loadInitialData();

		console.debug('[SeoService] Initialized');
	}

	/**
	 * WebSocket setup for real-time SEO monitoring
	 * ICT 7 FIX: Only attempt WebSocket if VITE_WS_URL is explicitly configured
	 */
	private setupWebSocket(): void {
		if (!browser) return;

		// ICT 7 FIX: Only attempt WebSocket if VITE_WS_URL is explicitly configured
		const configuredWsUrl = import.meta.env['VITE_WS_URL'];
		if (!configuredWsUrl) {
			// Silently skip - WebSocket is optional
			return;
		}

		try {
			this.wsConnection = new WebSocket(`${configuredWsUrl}/seo`);

			this.wsConnection.onopen = () => {
				console.debug('[SeoService] WebSocket connected');
				this.subscribeToUpdates();
			};

			this.wsConnection.onmessage = (event) => {
				this.handleWebSocketMessage(event);
			};

			this.wsConnection.onerror = () => {
				// Silently handle - WebSocket is optional
			};

			this.wsConnection.onclose = () => {
				// Don't auto-reconnect - WebSocket is optional
			};
		} catch (error) {
			// Silently handle - WebSocket is optional
		}
	}

	private subscribeToUpdates(): void {
		this.wsConnection?.send(
			JSON.stringify({
				type: 'subscribe',
				channels: ['rankings', 'crawl', 'alerts', 'competitors']
			})
		);
	}

	private handleWebSocketMessage(event: MessageEvent): void {
		try {
			const message = JSON.parse(event.data);

			switch (message.type) {
				case 'ranking_update':
					this.handleRankingUpdate(message.data);
					break;
				case 'crawl_complete':
					this.handleCrawlComplete(message.data);
					break;
				case 'new_404':
					this.handleNew404(message.data);
					break;
				case 'competitor_update':
					this.handleCompetitorUpdate(message.data);
					break;
				case 'seo_alert':
					this.handleSeoAlert(message.data);
					break;
			}
		} catch (error) {
			console.error('[SeoService] Failed to handle WebSocket message:', error);
		}
	}

	private handleRankingUpdate(data: RankTracking): void {
		this.rankings.update((rankings) => {
			const index = rankings.findIndex((r) => r.keyword === data.keyword);
			if (index >= 0) {
				rankings[index] = data;
			} else {
				rankings.push(data);
			}
			return rankings;
		});

		// Show notification for significant changes
		if (Math.abs(data.change) >= 5) {
			this.showNotification(
				`Ranking ${data.change > 0 ? 'improved' : 'dropped'} for "${data.keyword}": ${data.change > 0 ? '+' : ''}${data.change}`,
				data.change > 0 ? 'success' : 'warning'
			);
		}
	}

	private handleCrawlComplete(_data: any): void {
		// Refresh technical issues
		this.loadTechnicalIssues();
	}

	private handleNew404(error404: Error404): void {
		this.errors404.update((errors) => [...errors, error404]);
		this.showNotification(`New 404 error detected: ${error404.url}`, 'warning');
	}

	private handleCompetitorUpdate(competitor: Competitor): void {
		this.competitors.update((comps) => {
			const index = comps.findIndex((c) => c.domain === competitor.domain);
			if (index >= 0) {
				comps[index] = competitor;
			} else {
				comps.push(competitor);
			}
			return comps;
		});
	}

	private handleSeoAlert(alert: SeoAlert): void {
		this.alerts.update((alerts) => [alert, ...alerts]);
		this.showNotification(alert.message, alert.severity);
	}

	/**
	 * Load initial data
	 */
	private async loadInitialData(): Promise<void> {
		try {
			await Promise.all([
				this.loadRedirects(),
				this.load404Errors(),
				this.loadRankings(),
				this.loadBacklinks()
			]);
		} catch (error) {
			console.error('[SeoService] Failed to load initial data:', error);
		}
	}

	/**
	 * Start rank tracking
	 */
	private startRankTracking(): void {
		if (!browser) return;

		// Initial check
		this.checkRankings();

		// Periodic checks
		this.rankCheckInterval = window.setInterval(() => {
			this.checkRankings();
		}, RANK_CHECK_INTERVAL);
	}

	private async checkRankings(): Promise<void> {
		try {
			const rankings = await api.get<{ rankings: RankTracking[] }>('/admin/seo/rankings/check');
			this.rankings.set(rankings.rankings);
		} catch (error) {
			console.error('[SeoService] Failed to check rankings:', error);
		}
	}

	/**
	 * Start alert monitoring
	 */
	private startAlertMonitoring(): void {
		if (!browser) return;

		// Periodic checks
		this.alertCheckInterval = window.setInterval(() => {
			this.checkAlerts();
		}, ALERT_CHECK_INTERVAL);
	}

	private async checkAlerts(): Promise<void> {
		try {
			const alerts = await api.get<{ alerts: SeoAlert[] }>('/admin/seo/alerts/check');

			// Process new alerts
			alerts.alerts.forEach((alert) => {
				if (alert.is_new) {
					this.handleSeoAlert(alert);
				}
			});
		} catch (error) {
			console.error('[SeoService] Failed to check alerts:', error);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Public API Methods - Analysis
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Analyze content for SEO
	 */
	async analyze(
		contentType: string,
		contentId: number,
		focusKeyword?: string,
		options: AnalysisOptions = {}
	): Promise<EnhancedSeoAnalysis> {
		this.isLoading.set(true);
		this.error.set(null);

		const analysisKey = `${contentType}_${contentId}`;

		// Debounce analysis
		if (this.analysisDebounceTimers.has(analysisKey)) {
			clearTimeout(this.analysisDebounceTimers.get(analysisKey));
		}

		return new Promise((resolve, reject) => {
			const timer = window.setTimeout(
				async () => {
					try {
						// Check if analysis is already pending
						if (this.pendingAnalyses.has(analysisKey)) {
							const result = await this.pendingAnalyses.get(analysisKey);
							resolve(result);
							return;
						}

						// Create analysis promise
						const analysisPromise = this.performAnalysis(
							contentType,
							contentId,
							focusKeyword,
							options
						);

						// Store pending analysis
						this.pendingAnalyses.set(analysisKey, analysisPromise);

						const analysis = await analysisPromise;

						// Update stores
						this.currentAnalysis.set(analysis);
						this.analyses.update((analyses) => {
							const index = analyses.findIndex(
								(a) => a.analyzable_type === contentType && a.analyzable_id === contentId
							);
							if (index >= 0) {
								analyses[index] = analysis;
							} else {
								analyses.push(analysis);
							}
							return analyses;
						});

						resolve(analysis);
					} catch (error: any) {
						this.error.set(error.message);
						reject(error);
					} finally {
						this.pendingAnalyses.delete(analysisKey);
						this.isLoading.set(false);
					}
				},
				options.immediate ? 0 : ANALYSIS_DEBOUNCE
			);

			this.analysisDebounceTimers.set(analysisKey, timer);
		});
	}

	private async performAnalysis(
		contentType: string,
		contentId: number,
		focusKeyword?: string,
		options: AnalysisOptions = {}
	): Promise<EnhancedSeoAnalysis> {
		// Perform comprehensive analysis
		const [
			basicAnalysis,
			competitorAnalysis,
			keywordOpportunities,
			contentGaps,
			technicalAudit,
			performanceMetrics
		] = await Promise.all([
			this.getBasicAnalysis(contentType, contentId, focusKeyword),
			options.includeCompetitors ? this.analyzeCompetitors(focusKeyword) : null,
			options.includeKeywords ? this.findKeywordOpportunities(focusKeyword) : null,
			options.includeContentGaps ? this.analyzeContentGaps(focusKeyword) : null,
			options.includeTechnical ? this.performTechnicalAudit(contentType, contentId) : null,
			options.includePerformance ? this.getPerformanceMetrics(contentType, contentId) : null
		]);

		// Calculate overall score
		const overallScore = this.calculateOverallScore({
			basicAnalysis,
			technicalAudit,
			performanceMetrics
		});

		// Generate AI-powered suggestions
		const aiSuggestions = await this.generateAISuggestions({
			basicAnalysis,
			competitorAnalysis,
			keywordOpportunities,
			contentGaps
		});

		return {
			...basicAnalysis,
			overall_score: overallScore,
			technical_score: technicalAudit ? 100 - technicalAudit.issues.length * 5 : 100,
			content_score: basicAnalysis.seo_score,
			user_experience_score: performanceMetrics?.pageSpeed.score || 100,
			...(competitorAnalysis !== null && competitorAnalysis !== undefined && { competitor_comparison: competitorAnalysis }),
			...(keywordOpportunities !== null && keywordOpportunities !== undefined && { keyword_opportunities: keywordOpportunities }),
			...(contentGaps !== null && contentGaps !== undefined && { content_gaps: contentGaps }),
			...(technicalAudit?.issues !== undefined && { technical_issues: technicalAudit.issues }),
			...(performanceMetrics?.pageSpeed !== undefined && { page_speed: performanceMetrics.pageSpeed }),
			...(performanceMetrics?.coreWebVitals !== undefined && { core_web_vitals: performanceMetrics.coreWebVitals }),
			suggestions: [...basicAnalysis.suggestions, ...aiSuggestions]
		};
	}

	private async getBasicAnalysis(
		contentType: string,
		contentId: number,
		focusKeyword?: string
	): Promise<SeoAnalysis> {
		const response = await api.post<{ analysis: SeoAnalysis }>('/admin/seo/analyze', {
			content_type: contentType,
			content_id: contentId,
			focus_keyword: focusKeyword
		});
		return response.analysis;
	}

	private async analyzeCompetitors(keyword?: string): Promise<CompetitorAnalysis | null> {
		if (!keyword) return null;

		const response = await api.post<{ analysis: CompetitorAnalysis }>('/admin/seo/competitors/analyze', {
			keyword
		});
		return response.analysis;
	}

	private async findKeywordOpportunities(seed?: string): Promise<KeywordOpportunity[] | null> {
		const response = await api.post<{ opportunities: KeywordOpportunity[] }>(
			'/seo/keywords/opportunities',
			{ seed_keyword: seed }
		);
		return response.opportunities;
	}

	private async analyzeContentGaps(keyword?: string): Promise<ContentGap[] | null> {
		if (!keyword) return null;

		const response = await api.post<{ gaps: ContentGap[] }>('/admin/seo/content/gaps', { keyword });
		return response.gaps;
	}

	private async performTechnicalAudit(
		contentType: string,
		contentId: number
	): Promise<{ issues: TechnicalIssue[] } | null> {
		const response = await api.post<{ issues: TechnicalIssue[] }>('/admin/seo/technical/audit', {
			content_type: contentType,
			content_id: contentId
		});
		return { issues: response.issues };
	}

	private async getPerformanceMetrics(
		contentType: string,
		contentId: number
	): Promise<{ pageSpeed: PageSpeedMetrics; coreWebVitals: CoreWebVitals } | null> {
		const response = await api.post<{
			pageSpeed: PageSpeedMetrics;
			coreWebVitals: CoreWebVitals;
		}>('/seo/performance/analyze', {
			content_type: contentType,
			content_id: contentId
		});
		return response;
	}

	private calculateOverallScore(data: any): number {
		// Weighted scoring algorithm
		const weights = {
			technical: 0.3,
			content: 0.3,
			performance: 0.2,
			user_experience: 0.2
		};

		let score = 0;

		if (data.basicAnalysis) {
			score += data.basicAnalysis.seo_score * weights.content;
		}

		if (data.technicalAudit) {
			const technicalScore = 100 - data.technicalAudit.issues.length * 5;
			score += Math.max(0, technicalScore) * weights.technical;
		}

		if (data.performanceMetrics) {
			score += data.performanceMetrics.pageSpeed.score * weights.performance;
		}

		return Math.round(score);
	}

	private async generateAISuggestions(data: any): Promise<Suggestion[]> {
		try {
			const response = await api.post<{ suggestions: Suggestion[] }>(
				`${AI_API}/seo/suggestions`,
				data
			);
			return response.suggestions;
		} catch (error) {
			console.error('[SeoService] Failed to generate AI suggestions:', error);
			return [];
		}
	}

	/**
	 * Get cached or fresh analysis
	 */
	async getAnalysis(contentType: string, contentId: number): Promise<EnhancedSeoAnalysis> {
		const cacheKey = `analysis_${contentType}_${contentId}`;
		const cached = this.getFromCache(cacheKey);

		if (cached) {
			return cached;
		}

		const response = await api.get<{ analysis: EnhancedSeoAnalysis }>(
			`/seo/analyze/${contentType}/${contentId}`
		);

		this.setCache(cacheKey, response.analysis);
		return response.analysis;
	}

	/**
	 * Get SEO recommendations
	 */
	async getRecommendations(contentType: string, contentId: number): Promise<Suggestion[]> {
		const response = await api.get<{ recommendations: Suggestion[] }>(
			`/seo/analyze/${contentType}/${contentId}/recommendations`
		);
		return response.recommendations;
	}

	/**
	 * Auto-fix SEO issues
	 */
	async autoFix(
		contentType: string,
		contentId: number,
		issues: string[]
	): Promise<{ fixed: string[]; failed: string[] }> {
		const response = await api.post<{ fixed: string[]; failed: string[] }>('/admin/seo/auto-fix', {
			content_type: contentType,
			content_id: contentId,
			issues
		});

		// Refresh analysis after fixes
		await this.analyze(contentType, contentId, undefined, { immediate: true });

		return response;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Public API Methods - Redirects
	// ═══════════════════════════════════════════════════════════════════════════

	async loadRedirects(params?: any): Promise<Redirect[]> {
		try {
			const response = await api.get<ApiResponse<Redirect[]>>('/admin/redirects', { params });
			this.redirects.set(response.data);
			return response.data;
		} catch (error) {
			// ICT 7: Silently fail if endpoint doesn't exist
			console.debug('[SeoService] loadRedirects endpoint not available');
			return [];
		}
	}

	async createRedirect(data: Partial<Redirect>): Promise<Redirect> {
		const response = await api.post<{ redirect: Redirect }>('/admin/redirects', data);

		this.redirects.update((redirects) => [...redirects, response.redirect]);

		// Track redirect creation
		this.trackEvent('redirect_created', { from: data.from_path, to: data.to_path });

		return response.redirect;
	}

	async updateRedirect(id: number, data: Partial<Redirect>): Promise<Redirect> {
		const response = await api.put<{ redirect: Redirect }>(`/redirects/${id}`, data);

		this.redirects.update((redirects) => {
			const index = redirects.findIndex((r) => r.id === id);
			if (index >= 0) {
				redirects[index] = response.redirect;
			}
			return redirects;
		});

		return response.redirect;
	}

	async deleteRedirect(id: number): Promise<void> {
		await api.delete(`/redirects/${id}`);

		this.redirects.update((redirects) => redirects.filter((r) => r.id !== id));
	}

	async detectRedirectChains(): Promise<RedirectChain[]> {
		const response = await api.get<{ chains: RedirectChain[] }>('/admin/redirects/chains');
		return response.chains;
	}

	async importRedirects(file: File): Promise<{ imported: number; failed: number }> {
		const formData = new FormData();
		formData.append('file', file);

		const response = await api.post<{ imported: number; failed: number }>(
			'/redirects/import',
			formData,
			{
				headers: {
					'Content-Type': 'multipart/form-data'
				}
			}
		);

		// Reload redirects
		await this.loadRedirects();

		return response;
	}

	async exportRedirects(): Promise<Blob> {
		const response = await api.get<Blob>('/admin/redirects/export');
		return response;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Public API Methods - 404 Errors
	// ═══════════════════════════════════════════════════════════════════════════

	async load404Errors(params?: any): Promise<Error404[]> {
		try {
			const response = await api.get<ApiResponse<Error404[]>>('/admin/404-errors', { params });
			this.errors404.set(response.data);
			return response.data;
		} catch (error) {
			// ICT 7: Silently fail if endpoint doesn't exist
			console.debug('[SeoService] load404Errors endpoint not available');
			return [];
		}
	}

	async resolve404(id: number, redirectTo?: string): Promise<void> {
		if (redirectTo) {
			// Create redirect
			await this.createRedirect({
				from_path: (await this.get404(id)).url,
				to_path: redirectTo,
				status_code: 301,
				type: 'automatic'
			});
		}

		// Mark as resolved
		await api.put(`/404-errors/${id}/resolve`);

		this.errors404.update((errors) =>
			errors.map((e) => (e.id === id ? { ...e, is_resolved: true } : e))
		);
	}

	private async get404(id: number): Promise<Error404> {
		const errors = get(this.errors404);
		const error = errors.find((e) => e.id === id);

		if (error) return error;

		const response = await api.get<{ error: Error404 }>(`/404-errors/${id}`);
		return response.error;
	}

	async bulkDelete404s(
		resolvedOnly: boolean,
		olderThanDays?: number
	): Promise<{ deleted: number }> {
		const response = await api.post<{ deleted: number }>('/admin/404-errors/bulk-delete', {
			resolved_only: resolvedOnly,
			older_than_days: olderThanDays
		});

		// Reload 404 errors
		await this.load404Errors();

		return response;
	}

	async findSimilarPages(url: string): Promise<string[]> {
		const response = await api.post<{ suggestions: string[] }>('/admin/404-errors/find-similar', { url });
		return response.suggestions;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Public API Methods - Rankings
	// ═══════════════════════════════════════════════════════════════════════════

	async loadRankings(keywords?: string[]): Promise<RankTracking[]> {
		try {
			const response = await api.get<{ rankings: RankTracking[] }>('/admin/seo/rankings', {
				params: { keywords }
			});
			this.rankings.set(response.rankings);
			return response.rankings;
		} catch (error) {
			// ICT 7: Silently fail if endpoint doesn't exist
			console.debug('[SeoService] loadRankings endpoint not available');
			return [];
		}
	}

	async trackKeyword(
		keyword: string,
		url: string,
		options?: TrackingOptions
	): Promise<RankTracking> {
		const response = await api.post<{ tracking: RankTracking }>('/admin/seo/rankings/track', {
			keyword,
			url,
			...options
		});

		this.rankings.update((rankings) => [...rankings, response.tracking]);

		return response.tracking;
	}

	async updateRankings(): Promise<RankTracking[]> {
		const response = await api.post<{ rankings: RankTracking[] }>('/admin/seo/rankings/update');
		this.rankings.set(response.rankings);
		return response.rankings;
	}

	async getRankingHistory(keyword: string, days: number = 30): Promise<RankHistory[]> {
		const response = await api.get<{ history: RankHistory[] }>(
			`/seo/rankings/${encodeURIComponent(keyword)}/history`,
			{ params: { days } }
		);
		return response.history;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Public API Methods - Backlinks
	// ═══════════════════════════════════════════════════════════════════════════

	async loadBacklinks(): Promise<BacklinkProfile | null> {
		try {
			const response = await api.get<{ profile: BacklinkProfile }>('/admin/seo/backlinks');
			this.backlinks.set(response.profile);
			return response.profile;
		} catch (error) {
			// ICT 7: Silently fail if endpoint doesn't exist
			console.debug('[SeoService] loadBacklinks endpoint not available');
			return null;
		}
	}

	async checkNewBacklinks(): Promise<Backlink[]> {
		const response = await api.get<{ backlinks: Backlink[] }>('/admin/seo/backlinks/new');
		return response.backlinks;
	}

	async analyzeToxicBacklinks(): Promise<Backlink[]> {
		const response = await api.get<{ toxic: Backlink[] }>('/admin/seo/backlinks/toxic');
		return response.toxic;
	}

	async disavowBacklinks(domains: string[]): Promise<void> {
		await api.post('/seo/backlinks/disavow', { domains });
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Public API Methods - Settings & Configuration
	// ═══════════════════════════════════════════════════════════════════════════

	async getSettings(): Promise<SeoSettings> {
		const response = await api.get<SeoSettings>('/seo-settings');
		return response;
	}

	async updateSetting(key: string, value: any): Promise<void> {
		await api.put(`/seo-settings/${key}`, { value });
	}

	async getSitemapConfigs(): Promise<SitemapConfig[]> {
		const response = await api.get<{ configs: SitemapConfig[] }>('/seo-settings/sitemap-configs');
		return response.configs;
	}

	async updateSitemapConfig(config: SitemapConfig): Promise<void> {
		await api.put(`/seo-settings/sitemap-configs/${config.id}`, config);
	}

	async generateSitemap(): Promise<string> {
		const response = await api.post<{ url: string }>('/admin/seo/sitemap/generate');
		return response.url;
	}

	async getLocalBusiness(): Promise<LocalBusiness> {
		const response = await api.get<LocalBusiness>('/seo-settings/local-business');
		return response;
	}

	async updateLocalBusiness(data: Partial<LocalBusiness>): Promise<void> {
		await api.post('/seo-settings/local-business', data);
	}

	async generateSchema(type: string, data: any): Promise<string> {
		const response = await api.post<{ schema: string }>('/admin/seo/schema/generate', {
			type,
			data
		});
		return response.schema;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Utilities
	// ═══════════════════════════════════════════════════════════════════════════

	private getFromCache(key: string): any {
		const cached = this.cache.get(key);
		if (cached && Date.now() < cached.expiry) {
			return cached.data;
		}
		return null;
	}

	private setCache(key: string, data: any, ttl: number = CACHE_TTL): void {
		this.cache.set(key, {
			data,
			expiry: Date.now() + ttl
		});
	}


	private async loadTechnicalIssues(): Promise<void> {
		try {
			const response = await api.get<{ issues: TechnicalIssue[] }>('/admin/seo/technical/issues');

			// Update current analysis with new issues
			this.currentAnalysis.update((analysis) => {
				if (analysis) {
					analysis.technical_issues = response.issues;
				}
				return analysis;
			});
		} catch (error) {
			console.error('[SeoService] Failed to load technical issues:', error);
		}
	}

	private showNotification(
		message: string,
		type: 'info' | 'success' | 'warning' | 'error' = 'info'
	): void {
		// Implement notification system
		console.log(`[${type.toUpperCase()}] ${message}`);
	}

	private trackEvent(event: string, data: any): void {
		// Analytics tracking
		if (browser && 'gtag' in window) {
			(window as any).gtag('event', event, data);
		}
	}

	/**
	 * Cleanup
	 */
	destroy(): void {
		// Clear intervals
		if (this.rankCheckInterval) {
			clearInterval(this.rankCheckInterval);
		}
		if (this.alertCheckInterval) {
			clearInterval(this.alertCheckInterval);
		}

		// Close WebSocket
		this.wsConnection?.close();

		// Clear debounce timers
		this.analysisDebounceTimers.forEach((timer) => clearTimeout(timer));
		this.analysisDebounceTimers.clear();
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Type Definitions - Additional
// ═══════════════════════════════════════════════════════════════════════════

interface AnalysisOptions {
	immediate?: boolean;
	includeCompetitors?: boolean;
	includeKeywords?: boolean;
	includeContentGaps?: boolean;
	includeTechnical?: boolean;
	includePerformance?: boolean;
}

interface TrackingOptions {
	device?: 'desktop' | 'mobile';
	location?: string;
	language?: string;
	competitors?: string[];
}

interface SeoAlert {
	id: string;
	type: string;
	severity: 'info' | 'warning' | 'error';
	message: string;
	details?: any;
	is_new: boolean;
	created_at: string;
}

interface SeoSettings {
	auto_generate_meta: boolean;
	auto_generate_sitemap: boolean;
	enable_redirects: boolean;
	enable_404_tracking: boolean;
	enable_rank_tracking: boolean;
	crawl_frequency: string;
	robots_txt: string;
	default_meta_tags: Record<string, string>;
	social_meta_tags: Record<string, string>;
}

// ═══════════════════════════════════════════════════════════════════════════
// Export singleton instance and convenience functions
// ═══════════════════════════════════════════════════════════════════════════

const seoService = SeoManagementService.getInstance();

// Export stores
export const analyses = seoService.analyses;
export const currentAnalysis = seoService.currentAnalysis;
export const redirects = seoService.redirects;
export const errors404 = seoService.errors404;
export const rankings = seoService.rankings;
export const backlinks = seoService.backlinks;
export const competitors = seoService.competitors;
export const alerts = seoService.alerts;
export const criticalIssues = seoService.criticalIssues;
export const topOpportunities = seoService.topOpportunities;
export const overallScore = seoService.overallScore;
export const isLoading = seoService.isLoading;
export const error = seoService.error;

// Export main API object with all methods
export const seoApi = {
	// Analysis
	analyze: (
		contentType: string,
		contentId: number,
		focusKeyword?: string,
		options?: AnalysisOptions
	) => seoService.analyze(contentType, contentId, focusKeyword, options),
	getAnalysis: (contentType: string, contentId: number) =>
		seoService.getAnalysis(contentType, contentId),
	getRecommendations: (contentType: string, contentId: number) =>
		seoService.getRecommendations(contentType, contentId),
	autoFix: (contentType: string, contentId: number, issues: string[]) =>
		seoService.autoFix(contentType, contentId, issues),

	// Redirects
	listRedirects: (params?: any) => seoService.loadRedirects(params),
	createRedirect: (data: Partial<Redirect>) => seoService.createRedirect(data),
	updateRedirect: (id: number, data: Partial<Redirect>) => seoService.updateRedirect(id, data),
	deleteRedirect: (id: number) => seoService.deleteRedirect(id),
	detectRedirectChains: () => seoService.detectRedirectChains(),
	importRedirects: (file: File) => seoService.importRedirects(file),
	exportRedirects: () => seoService.exportRedirects(),
	getRedirectStats: () => api.get('/redirects/statistics'),

	// 404 Errors
	list404s: (params?: any) => seoService.load404Errors(params),
	resolve404: (id: number, redirectTo?: string) => seoService.resolve404(id, redirectTo),
	bulkDelete404s: (resolvedOnly: boolean, olderThanDays?: number) =>
		seoService.bulkDelete404s(resolvedOnly, olderThanDays),
	findSimilarPages: (url: string) => seoService.findSimilarPages(url),
	get404Stats: () => api.get('/404-errors/statistics'),

	// Rankings
	loadRankings: (keywords?: string[]) => seoService.loadRankings(keywords),
	trackKeyword: (keyword: string, url: string, options?: TrackingOptions) =>
		seoService.trackKeyword(keyword, url, options),
	updateRankings: () => seoService.updateRankings(),
	getRankingHistory: (keyword: string, days?: number) =>
		seoService.getRankingHistory(keyword, days),

	// Backlinks
	loadBacklinks: () => seoService.loadBacklinks(),
	checkNewBacklinks: () => seoService.checkNewBacklinks(),
	analyzeToxicBacklinks: () => seoService.analyzeToxicBacklinks(),
	disavowBacklinks: (domains: string[]) => seoService.disavowBacklinks(domains),

	// Settings
	getSettings: () => seoService.getSettings(),
	updateSetting: (key: string, value: any) => seoService.updateSetting(key, value),
	getSitemapConfigs: () => seoService.getSitemapConfigs(),
	updateSitemapConfig: (config: SitemapConfig) => seoService.updateSitemapConfig(config),
	generateSitemap: () => seoService.generateSitemap(),
	getLocalBusiness: () => seoService.getLocalBusiness(),
	updateLocalBusiness: (data: Partial<LocalBusiness>) => seoService.updateLocalBusiness(data),
	generateSchema: (type: string, data: any) => seoService.generateSchema(type, data)
};

export default seoService;
