<?php

namespace App\Services\Seo;

use App\Models\SeoAiSuggestion;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;

/**
 * AI Content Optimizer Service
 * 
 * Google L8 Enterprise-Grade AI Optimization Engine
 * Implements: GPT-4 Title/Meta Generation, Content Enhancement, Outline Generation
 * 
 * @author RevolutionSEO-L8-System
 * @version 1.0.0
 */
class AiContentOptimizerService
{
    private const CACHE_PREFIX = 'seo:ai';
    private const CACHE_TTL = 3600; // 1 hour
    
    private string $openaiApiKey;
    private string $anthropicApiKey;
    private string $provider;

    public function __construct()
    {
        $this->openaiApiKey = config('services.openai.api_key');
        $this->anthropicApiKey = config('services.anthropic.api_key');
        $this->provider = config('services.seo.ai_provider', 'openai');
    }

    /**
     * Generate optimized title suggestions.
     */
    public function generateTitleSuggestions(
        string $contentType,
        int $contentId,
        string $currentTitle,
        string $focusKeyword,
        string $contentSummary,
        int $count = 10
    ): array {
        $cacheKey = self::CACHE_PREFIX . ":title:{$contentType}:{$contentId}:" . md5($currentTitle . $focusKeyword);
        
        return Cache::remember($cacheKey, self::CACHE_TTL, function () use (
            $contentType, $contentId, $currentTitle, $focusKeyword, $contentSummary, $count
        ) {
            $prompt = $this->buildTitlePrompt($currentTitle, $focusKeyword, $contentSummary, $count);
            
            $response = $this->callAiApi($prompt, [
                'temperature' => 0.8,
                'max_tokens' => 500,
            ]);
            
            $suggestions = $this->parseTitleSuggestions($response);
            
            // Store suggestions in database
            foreach ($suggestions as $suggestion) {
                SeoAiSuggestion::create([
                    'content_type' => $contentType,
                    'content_id' => $contentId,
                    'suggestion_type' => 'title',
                    'original_text' => $currentTitle,
                    'suggested_text' => $suggestion['title'],
                    'reasoning' => $suggestion['reasoning'],
                    'impact_score' => $suggestion['impact_score'],
                    'confidence_score' => $suggestion['confidence_score'],
                    'status' => 'pending',
                    'metadata' => [
                        'model_used' => $this->provider,
                        'focus_keyword' => $focusKeyword,
                        'ctr_potential' => $suggestion['ctr_potential'] ?? 'medium',
                    ],
                ]);
            }
            
            return $suggestions;
        });
    }

    /**
     * Generate optimized meta description.
     */
    public function generateMetaDescription(
        string $contentType,
        int $contentId,
        string $currentMeta,
        string $focusKeyword,
        string $contentSummary,
        int $count = 5
    ): array {
        $cacheKey = self::CACHE_PREFIX . ":meta:{$contentType}:{$contentId}:" . md5($currentMeta . $focusKeyword);
        
        return Cache::remember($cacheKey, self::CACHE_TTL, function () use (
            $contentType, $contentId, $currentMeta, $focusKeyword, $contentSummary, $count
        ) {
            $prompt = $this->buildMetaPrompt($currentMeta, $focusKeyword, $contentSummary, $count);
            
            $response = $this->callAiApi($prompt, [
                'temperature' => 0.7,
                'max_tokens' => 400,
            ]);
            
            $suggestions = $this->parseMetaSuggestions($response);
            
            // Store suggestions
            foreach ($suggestions as $suggestion) {
                SeoAiSuggestion::create([
                    'content_type' => $contentType,
                    'content_id' => $contentId,
                    'suggestion_type' => 'meta',
                    'original_text' => $currentMeta,
                    'suggested_text' => $suggestion['description'],
                    'reasoning' => $suggestion['reasoning'],
                    'impact_score' => $suggestion['impact_score'],
                    'confidence_score' => $suggestion['confidence_score'],
                    'status' => 'pending',
                    'metadata' => [
                        'model_used' => $this->provider,
                        'focus_keyword' => $focusKeyword,
                        'has_cta' => $suggestion['has_cta'] ?? false,
                        'character_count' => strlen($suggestion['description']),
                    ],
                ]);
            }
            
            return $suggestions;
        });
    }

    /**
     * Generate content outline/brief.
     */
    public function generateOutline(
        string $topic,
        string $focusKeyword,
        array $secondaryKeywords = [],
        string $targetAudience = 'general',
        int $targetWordCount = 1500
    ): array {
        $cacheKey = self::CACHE_PREFIX . ":outline:" . md5($topic . $focusKeyword . json_encode($secondaryKeywords));
        
        return Cache::remember($cacheKey, self::CACHE_TTL * 2, function () use (
            $topic, $focusKeyword, $secondaryKeywords, $targetAudience, $targetWordCount
        ) {
            $prompt = $this->buildOutlinePrompt($topic, $focusKeyword, $secondaryKeywords, $targetAudience, $targetWordCount);
            
            $response = $this->callAiApi($prompt, [
                'temperature' => 0.6,
                'max_tokens' => 1500,
            ]);
            
            return $this->parseOutline($response);
        });
    }

    /**
     * Generate content improvement suggestions.
     */
    public function generateContentSuggestions(
        string $contentType,
        int $contentId,
        string $content,
        string $focusKeyword,
        array $missingEntities = [],
        array $missingTopics = []
    ): array {
        $cacheKey = self::CACHE_PREFIX . ":suggestions:{$contentType}:{$contentId}:" . md5($content);
        
        return Cache::remember($cacheKey, self::CACHE_TTL, function () use (
            $contentType, $contentId, $content, $focusKeyword, $missingEntities, $missingTopics
        ) {
            $prompt = $this->buildContentSuggestionsPrompt($content, $focusKeyword, $missingEntities, $missingTopics);
            
            $response = $this->callAiApi($prompt, [
                'temperature' => 0.5,
                'max_tokens' => 2000,
            ]);
            
            $suggestions = $this->parseContentSuggestions($response);
            
            // Store paragraph suggestions
            foreach ($suggestions as $suggestion) {
                if ($suggestion['type'] === 'paragraph') {
                    SeoAiSuggestion::create([
                        'content_type' => $contentType,
                        'content_id' => $contentId,
                        'suggestion_type' => 'paragraph',
                        'original_text' => $suggestion['original'] ?? '',
                        'suggested_text' => $suggestion['suggested'],
                        'reasoning' => $suggestion['reasoning'],
                        'impact_score' => $suggestion['impact_score'],
                        'confidence_score' => $suggestion['confidence_score'],
                        'status' => 'pending',
                        'metadata' => [
                            'model_used' => $this->provider,
                            'improvement_type' => $suggestion['improvement_type'] ?? 'general',
                        ],
                    ]);
                }
            }
            
            return $suggestions;
        });
    }

    /**
     * Enhance specific paragraph.
     */
    public function enhanceParagraph(
        string $paragraph,
        string $focusKeyword,
        string $improvementType = 'readability'
    ): string {
        $prompt = $this->buildParagraphEnhancementPrompt($paragraph, $focusKeyword, $improvementType);
        
        $response = $this->callAiApi($prompt, [
            'temperature' => 0.4,
            'max_tokens' => 500,
        ]);
        
        return trim($response);
    }

    /**
     * Call AI API (OpenAI or Anthropic).
     */
    private function callAiApi(string $prompt, array $options = []): string
    {
        try {
            if ($this->provider === 'openai') {
                return $this->callOpenAi($prompt, $options);
            } elseif ($this->provider === 'anthropic') {
                return $this->callAnthropic($prompt, $options);
            }
            
            throw new \Exception("Invalid AI provider: {$this->provider}");
            
        } catch (\Exception $e) {
            Log::error('AI API call failed', [
                'provider' => $this->provider,
                'error' => $e->getMessage(),
            ]);
            
            throw $e;
        }
    }

    /**
     * Call OpenAI GPT-4 API.
     */
    private function callOpenAi(string $prompt, array $options = []): string
    {
        if (!$this->openaiApiKey) {
            throw new \Exception('OpenAI API key not configured');
        }
        
        $response = Http::timeout(30)
            ->withHeaders([
                'Authorization' => 'Bearer ' . $this->openaiApiKey,
                'Content-Type' => 'application/json',
            ])
            ->post('https://api.openai.com/v1/chat/completions', [
                'model' => $options['model'] ?? 'gpt-4-turbo-preview',
                'messages' => [
                    [
                        'role' => 'system',
                        'content' => 'You are an expert SEO content optimizer with deep knowledge of search engine algorithms, user intent, and content marketing best practices.',
                    ],
                    [
                        'role' => 'user',
                        'content' => $prompt,
                    ],
                ],
                'temperature' => $options['temperature'] ?? 0.7,
                'max_tokens' => $options['max_tokens'] ?? 1000,
            ]);
        
        if (!$response->successful()) {
            throw new \Exception('OpenAI API request failed: ' . $response->body());
        }
        
        $data = $response->json();
        
        return $data['choices'][0]['message']['content'] ?? '';
    }

    /**
     * Call Anthropic Claude API.
     */
    private function callAnthropic(string $prompt, array $options = []): string
    {
        if (!$this->anthropicApiKey) {
            throw new \Exception('Anthropic API key not configured');
        }
        
        $response = Http::timeout(30)
            ->withHeaders([
                'x-api-key' => $this->anthropicApiKey,
                'anthropic-version' => '2023-06-01',
                'Content-Type' => 'application/json',
            ])
            ->post('https://api.anthropic.com/v1/messages', [
                'model' => $options['model'] ?? 'claude-3-opus-20240229',
                'max_tokens' => $options['max_tokens'] ?? 1000,
                'messages' => [
                    [
                        'role' => 'user',
                        'content' => $prompt,
                    ],
                ],
            ]);
        
        if (!$response->successful()) {
            throw new \Exception('Anthropic API request failed: ' . $response->body());
        }
        
        $data = $response->json();
        
        return $data['content'][0]['text'] ?? '';
    }

    /**
     * Build title generation prompt.
     */
    private function buildTitlePrompt(string $currentTitle, string $focusKeyword, string $summary, int $count): string
    {
        return <<<PROMPT
Generate {$count} SEO-optimized title variations for a piece of content.

Current Title: "{$currentTitle}"
Focus Keyword: "{$focusKeyword}"
Content Summary: {$summary}

Requirements:
- Include the focus keyword naturally
- Keep titles between 50-60 characters
- Make them compelling and click-worthy
- Optimize for both SEO and CTR
- Use power words and emotional triggers
- Ensure each title is unique and different in approach

For each title, provide:
1. The title text
2. Brief reasoning for why it's effective
3. Estimated impact score (0-100)
4. Confidence score (0-100)
5. CTR potential (low/medium/high)

Format as JSON array:
[
  {
    "title": "...",
    "reasoning": "...",
    "impact_score": 85,
    "confidence_score": 90,
    "ctr_potential": "high"
  }
]
PROMPT;
    }

    /**
     * Build meta description prompt.
     */
    private function buildMetaPrompt(string $currentMeta, string $focusKeyword, string $summary, int $count): string
    {
        return <<<PROMPT
Generate {$count} SEO-optimized meta description variations.

Current Meta: "{$currentMeta}"
Focus Keyword: "{$focusKeyword}"
Content Summary: {$summary}

Requirements:
- Include the focus keyword naturally
- Keep between 140-160 characters
- Include a clear call-to-action
- Make it compelling and informative
- Optimize for click-through rate
- Use active voice

For each description, provide:
1. The description text
2. Brief reasoning
3. Impact score (0-100)
4. Confidence score (0-100)
5. Whether it has a CTA (boolean)

Format as JSON array:
[
  {
    "description": "...",
    "reasoning": "...",
    "impact_score": 80,
    "confidence_score": 85,
    "has_cta": true
  }
]
PROMPT;
    }

    /**
     * Build outline generation prompt.
     */
    private function buildOutlinePrompt(
        string $topic,
        string $focusKeyword,
        array $secondaryKeywords,
        string $targetAudience,
        int $targetWordCount
    ): string {
        $secondaryKeywordsStr = implode(', ', $secondaryKeywords);
        
        return <<<PROMPT
Create a comprehensive content outline for an SEO-optimized article.

Topic: {$topic}
Focus Keyword: {$focusKeyword}
Secondary Keywords: {$secondaryKeywordsStr}
Target Audience: {$targetAudience}
Target Word Count: {$targetWordCount}

Create a detailed outline with:
1. Compelling H1 title
2. Introduction (what to cover)
3. Main sections with H2 headings
4. Subsections with H3 headings where appropriate
5. Key points to cover in each section
6. Suggested word count per section
7. Keywords to naturally incorporate
8. Questions to answer
9. Conclusion points

Format as JSON:
{
  "title": "...",
  "introduction": {
    "key_points": [...],
    "word_count": 150
  },
  "sections": [
    {
      "heading": "...",
      "level": "h2",
      "key_points": [...],
      "subsections": [...],
      "keywords": [...],
      "word_count": 300
    }
  ],
  "conclusion": {
    "key_points": [...],
    "word_count": 100
  }
}
PROMPT;
    }

    /**
     * Build content suggestions prompt.
     */
    private function buildContentSuggestionsPrompt(
        string $content,
        string $focusKeyword,
        array $missingEntities,
        array $missingTopics
    ): string {
        $missingEntitiesStr = implode(', ', $missingEntities);
        $missingTopicsStr = implode(', ', $missingTopics);
        
        $contentPreview = substr($content, 0, 2000);
        
        return <<<PROMPT
Analyze this content and provide specific improvement suggestions.

Content Preview: {$contentPreview}
Focus Keyword: {$focusKeyword}
Missing Entities: {$missingEntitiesStr}
Missing Topics: {$missingTopicsStr}

Provide suggestions for:
1. Adding missing entities naturally
2. Covering missing topics
3. Improving readability
4. Enhancing keyword usage
5. Adding relevant examples
6. Improving structure

For each suggestion, provide:
- Type (paragraph, heading, entity, topic)
- Specific suggested text or change
- Reasoning
- Impact score (0-100)
- Confidence score (0-100)
- Improvement type (readability, seo, engagement, etc.)

Format as JSON array.
PROMPT;
    }

    /**
     * Build paragraph enhancement prompt.
     */
    private function buildParagraphEnhancementPrompt(
        string $paragraph,
        string $focusKeyword,
        string $improvementType
    ): string {
        return <<<PROMPT
Improve this paragraph for {$improvementType}.

Original: {$paragraph}
Focus Keyword: {$focusKeyword}

Requirements:
- Maintain the core message
- Improve {$improvementType}
- Include the focus keyword naturally if relevant
- Keep similar length
- Use active voice
- Make it more engaging

Return only the improved paragraph, no explanation.
PROMPT;
    }

    /**
     * Parse title suggestions from AI response.
     */
    private function parseTitleSuggestions(string $response): array
    {
        try {
            $json = json_decode($response, true);
            if (json_last_error() === JSON_ERROR_NONE && is_array($json)) {
                return $json;
            }
        } catch (\Exception $e) {
            Log::warning('Failed to parse title suggestions as JSON', ['response' => $response]);
        }
        
        // Fallback: parse line by line
        $lines = explode("\n", $response);
        $suggestions = [];
        
        foreach ($lines as $line) {
            $line = trim($line);
            if (!empty($line) && !str_starts_with($line, '{') && !str_starts_with($line, '[')) {
                $suggestions[] = [
                    'title' => $line,
                    'reasoning' => 'AI-generated suggestion',
                    'impact_score' => 75,
                    'confidence_score' => 80,
                    'ctr_potential' => 'medium',
                ];
            }
        }
        
        return $suggestions;
    }

    /**
     * Parse meta suggestions from AI response.
     */
    private function parseMetaSuggestions(string $response): array
    {
        try {
            $json = json_decode($response, true);
            if (json_last_error() === JSON_ERROR_NONE && is_array($json)) {
                return $json;
            }
        } catch (\Exception $e) {
            Log::warning('Failed to parse meta suggestions as JSON', ['response' => $response]);
        }
        
        return [];
    }

    /**
     * Parse outline from AI response.
     */
    private function parseOutline(string $response): array
    {
        try {
            $json = json_decode($response, true);
            if (json_last_error() === JSON_ERROR_NONE && is_array($json)) {
                return $json;
            }
        } catch (\Exception $e) {
            Log::warning('Failed to parse outline as JSON', ['response' => $response]);
        }
        
        return [
            'title' => 'Generated Outline',
            'sections' => [],
        ];
    }

    /**
     * Parse content suggestions from AI response.
     */
    private function parseContentSuggestions(string $response): array
    {
        try {
            $json = json_decode($response, true);
            if (json_last_error() === JSON_ERROR_NONE && is_array($json)) {
                return $json;
            }
        } catch (\Exception $e) {
            Log::warning('Failed to parse content suggestions as JSON', ['response' => $response]);
        }
        
        return [];
    }
}
