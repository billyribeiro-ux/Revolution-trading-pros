/**
 * AI Content Assistant
 * ═══════════════════════════════════════════════════════════════════════════
 * Real-time AI-powered writing assistance
 */

export interface SuggestionResult {
	suggestions: string[];
	confidence: number;
}

export interface ImprovementResult {
	improved: string;
	changes: Array<{ original: string; replacement: string; reason: string }>;
}

export interface OutlineResult {
	outline: string[];
	estimatedWordCount: number;
}

export interface GrammarCheck {
	issues: Array<{
		text: string;
		suggestion: string;
		type: 'grammar' | 'spelling' | 'style' | 'clarity';
		position: { start: number; end: number };
	}>;
	score: number;
}

export interface ContentAssistantConfig {
	apiEndpoint?: string;
	apiKey?: string;
	model?: string;
	maxTokens?: number;
}

export class ContentAssistant {
	private config: ContentAssistantConfig;

	constructor(config: ContentAssistantConfig = {}) {
		this.config = {
			apiEndpoint: config.apiEndpoint ?? '/api/ai',
			model: config.model ?? 'gpt-4',
			maxTokens: config.maxTokens ?? 500,
			...config
		};
	}

	async getSuggestions(content: string, context?: string): Promise<SuggestionResult> {
		try {
			const response = await fetch(`${this.config.apiEndpoint}/suggestions`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					...(this.config.apiKey && { Authorization: `Bearer ${this.config.apiKey}` })
				},
				body: JSON.stringify({
					content,
					context,
					model: this.config.model,
					maxTokens: this.config.maxTokens
				})
			});

			if (!response.ok) {
				throw new Error(`API error: ${response.status}`);
			}

			return await response.json();
		} catch (error) {
			console.error('AI suggestions error:', error);
			return { suggestions: [], confidence: 0 };
		}
	}

	async improveWriting(text: string): Promise<ImprovementResult> {
		try {
			const response = await fetch(`${this.config.apiEndpoint}/improve`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					...(this.config.apiKey && { Authorization: `Bearer ${this.config.apiKey}` })
				},
				body: JSON.stringify({
					text,
					model: this.config.model
				})
			});

			if (!response.ok) {
				throw new Error(`API error: ${response.status}`);
			}

			return await response.json();
		} catch (error) {
			console.error('AI improve error:', error);
			return { improved: text, changes: [] };
		}
	}

	async generateOutline(topic: string, depth = 2): Promise<OutlineResult> {
		try {
			const response = await fetch(`${this.config.apiEndpoint}/outline`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					...(this.config.apiKey && { Authorization: `Bearer ${this.config.apiKey}` })
				},
				body: JSON.stringify({
					topic,
					depth,
					model: this.config.model
				})
			});

			if (!response.ok) {
				throw new Error(`API error: ${response.status}`);
			}

			return await response.json();
		} catch (error) {
			console.error('AI outline error:', error);
			return { outline: [], estimatedWordCount: 0 };
		}
	}

	async checkGrammar(text: string): Promise<GrammarCheck> {
		try {
			const response = await fetch(`${this.config.apiEndpoint}/grammar`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					...(this.config.apiKey && { Authorization: `Bearer ${this.config.apiKey}` })
				},
				body: JSON.stringify({
					text,
					model: this.config.model
				})
			});

			if (!response.ok) {
				throw new Error(`API error: ${response.status}`);
			}

			return await response.json();
		} catch (error) {
			console.error('AI grammar check error:', error);
			return { issues: [], score: 100 };
		}
	}

	async summarize(text: string, length: 'short' | 'medium' | 'long' = 'medium'): Promise<string> {
		try {
			const response = await fetch(`${this.config.apiEndpoint}/summarize`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					...(this.config.apiKey && { Authorization: `Bearer ${this.config.apiKey}` })
				},
				body: JSON.stringify({
					text,
					length,
					model: this.config.model
				})
			});

			if (!response.ok) {
				throw new Error(`API error: ${response.status}`);
			}

			const data = await response.json();
			return data.summary;
		} catch (error) {
			console.error('AI summarize error:', error);
			return '';
		}
	}

	async expandContent(text: string, style?: string): Promise<string> {
		try {
			const response = await fetch(`${this.config.apiEndpoint}/expand`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					...(this.config.apiKey && { Authorization: `Bearer ${this.config.apiKey}` })
				},
				body: JSON.stringify({
					text,
					style,
					model: this.config.model
				})
			});

			if (!response.ok) {
				throw new Error(`API error: ${response.status}`);
			}

			const data = await response.json();
			return data.expanded;
		} catch (error) {
			console.error('AI expand error:', error);
			return text;
		}
	}

	async generateTitle(content: string, count = 5): Promise<string[]> {
		try {
			const response = await fetch(`${this.config.apiEndpoint}/titles`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					...(this.config.apiKey && { Authorization: `Bearer ${this.config.apiKey}` })
				},
				body: JSON.stringify({
					content,
					count,
					model: this.config.model
				})
			});

			if (!response.ok) {
				throw new Error(`API error: ${response.status}`);
			}

			const data = await response.json();
			return data.titles;
		} catch (error) {
			console.error('AI titles error:', error);
			return [];
		}
	}

	async translateText(text: string, targetLanguage: string): Promise<string> {
		try {
			const response = await fetch(`${this.config.apiEndpoint}/translate`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					...(this.config.apiKey && { Authorization: `Bearer ${this.config.apiKey}` })
				},
				body: JSON.stringify({
					text,
					targetLanguage,
					model: this.config.model
				})
			});

			if (!response.ok) {
				throw new Error(`API error: ${response.status}`);
			}

			const data = await response.json();
			return data.translated;
		} catch (error) {
			console.error('AI translate error:', error);
			return text;
		}
	}
}

let contentAssistant: ContentAssistant | null = null;

export function getContentAssistant(config?: ContentAssistantConfig): ContentAssistant {
	if (!contentAssistant) {
		contentAssistant = new ContentAssistant(config);
	}
	return contentAssistant;
}
