<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Form;
use App\Models\FormField;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Str;

/**
 * Form AI Service - AI-powered form generation and optimization
 *
 * Features:
 * - Natural language form generation
 * - Form field suggestions
 * - Smart field type detection
 * - Form optimization recommendations
 * - Conversion optimization
 * - A/B test suggestions
 * - Content generation
 * - Translation
 * - Spam detection
 * - Sentiment analysis
 *
 * @version 1.0.0
 */
class FormAIService
{
    /**
     * AI provider configuration
     */
    private string $provider;
    private string $apiKey;
    private string $model;

    /**
     * Field type detection patterns
     */
    private const FIELD_PATTERNS = [
        'email' => [
            'keywords' => ['email', 'e-mail', 'mail'],
            'type' => 'email',
        ],
        'phone' => [
            'keywords' => ['phone', 'telephone', 'mobile', 'cell', 'contact number'],
            'type' => 'phone',
        ],
        'name' => [
            'keywords' => ['name', 'full name', 'your name'],
            'type' => 'text',
            'attributes' => ['autocomplete' => 'name'],
        ],
        'first_name' => [
            'keywords' => ['first name', 'given name', 'forename'],
            'type' => 'text',
            'attributes' => ['autocomplete' => 'given-name'],
        ],
        'last_name' => [
            'keywords' => ['last name', 'surname', 'family name'],
            'type' => 'text',
            'attributes' => ['autocomplete' => 'family-name'],
        ],
        'address' => [
            'keywords' => ['address', 'street', 'location'],
            'type' => 'address',
        ],
        'date' => [
            'keywords' => ['date', 'birthday', 'dob', 'when'],
            'type' => 'date',
        ],
        'time' => [
            'keywords' => ['time', 'hour', 'schedule'],
            'type' => 'time',
        ],
        'number' => [
            'keywords' => ['number', 'amount', 'quantity', 'count', 'how many'],
            'type' => 'number',
        ],
        'currency' => [
            'keywords' => ['price', 'cost', 'budget', 'salary', 'income', 'payment'],
            'type' => 'number',
            'attributes' => ['format' => 'currency'],
        ],
        'url' => [
            'keywords' => ['website', 'url', 'link', 'homepage'],
            'type' => 'url',
        ],
        'textarea' => [
            'keywords' => ['message', 'description', 'comments', 'feedback', 'details', 'tell us', 'explain'],
            'type' => 'textarea',
        ],
        'file' => [
            'keywords' => ['upload', 'file', 'attachment', 'document', 'resume', 'cv', 'photo', 'image'],
            'type' => 'file',
        ],
        'rating' => [
            'keywords' => ['rating', 'rate', 'stars', 'score', 'satisfaction'],
            'type' => 'rating',
        ],
        'signature' => [
            'keywords' => ['signature', 'sign', 'autograph'],
            'type' => 'signature',
        ],
    ];

    public function __construct()
    {
        $this->provider = config('services.ai.provider', 'openai');
        $this->apiKey = config('services.ai.api_key', config('services.openai.api_key'));
        $this->model = config('services.ai.model', 'gpt-4');
    }

    // =========================================================================
    // FORM GENERATION
    // =========================================================================

    /**
     * Generate form from natural language description
     */
    public function generateFormFromDescription(string $description, array $options = []): array
    {
        $prompt = $this->buildFormGenerationPrompt($description, $options);
        $response = $this->callAI($prompt);

        if (!$response['success']) {
            return $response;
        }

        $formData = $this->parseFormResponse($response['content']);

        return [
            'success' => true,
            'form' => $formData,
            'suggestions' => $this->getFormSuggestions($formData),
        ];
    }

    /**
     * Build form generation prompt
     */
    private function buildFormGenerationPrompt(string $description, array $options): string
    {
        $style = $options['style'] ?? 'professional';
        $industry = $options['industry'] ?? null;

        return <<<PROMPT
You are a form design expert. Create a form based on this description:

"{$description}"

Style: {$style}
Industry: {$industry}

Return a JSON object with this structure:
{
    "title": "Form title",
    "description": "Brief description",
    "fields": [
        {
            "type": "text|email|phone|textarea|select|radio|checkbox|date|time|number|file|rating|signature|address",
            "label": "Field label",
            "name": "field_name",
            "placeholder": "Placeholder text",
            "help_text": "Help text if needed",
            "required": true|false,
            "validation": {},
            "attributes": {
                "options": [] // for select/radio/checkbox
            }
        }
    ],
    "settings": {
        "submit_button_text": "Submit",
        "success_message": "Thank you message"
    }
}

Guidelines:
- Use appropriate field types
- Include helpful placeholders
- Mark essential fields as required
- Add validation where appropriate
- Keep it concise but complete
- Use clear, professional labels

Return only valid JSON, no explanation.
PROMPT;
    }

    /**
     * Parse AI response to form structure
     */
    private function parseFormResponse(string $content): array
    {
        // Extract JSON from response
        $content = trim($content);

        // Remove markdown code blocks if present
        if (str_starts_with($content, '```')) {
            $content = preg_replace('/^```(?:json)?\s*/', '', $content);
            $content = preg_replace('/\s*```$/', '', $content);
        }

        $data = json_decode($content, true);

        if (json_last_error() !== JSON_ERROR_NONE) {
            return [
                'title' => 'New Form',
                'description' => '',
                'fields' => [],
                'error' => 'Failed to parse AI response',
            ];
        }

        // Normalize field names
        foreach ($data['fields'] ?? [] as $index => &$field) {
            if (empty($field['name'])) {
                $field['name'] = Str::snake($field['label'] ?? 'field_' . ($index + 1));
            }
            $field['order'] = $index;
        }

        return $data;
    }

    // =========================================================================
    // FIELD SUGGESTIONS
    // =========================================================================

    /**
     * Suggest fields based on form title/context
     */
    public function suggestFields(string $context, array $existingFields = []): array
    {
        $prompt = <<<PROMPT
Based on this form context: "{$context}"

Existing fields: {$this->formatExistingFields($existingFields)}

Suggest 3-5 additional fields that would improve this form.

Return JSON array:
[
    {
        "type": "field_type",
        "label": "Field Label",
        "name": "field_name",
        "reason": "Why this field is useful"
    }
]

Return only valid JSON.
PROMPT;

        $response = $this->callAI($prompt);

        if (!$response['success']) {
            return [];
        }

        return $this->parseJsonResponse($response['content']);
    }

    /**
     * Detect field type from label
     */
    public function detectFieldType(string $label): array
    {
        $labelLower = strtolower($label);

        foreach (self::FIELD_PATTERNS as $pattern => $config) {
            foreach ($config['keywords'] as $keyword) {
                if (str_contains($labelLower, $keyword)) {
                    return [
                        'type' => $config['type'],
                        'attributes' => $config['attributes'] ?? [],
                        'confidence' => 0.9,
                        'pattern' => $pattern,
                    ];
                }
            }
        }

        // Default to text with low confidence
        return [
            'type' => 'text',
            'attributes' => [],
            'confidence' => 0.5,
            'pattern' => null,
        ];
    }

    /**
     * Suggest options for select/radio fields
     */
    public function suggestOptions(string $fieldLabel, string $context = ''): array
    {
        $prompt = <<<PROMPT
Suggest options for a form field with label: "{$fieldLabel}"
Context: {$context}

Return JSON array of 4-8 common options:
["Option 1", "Option 2", "Option 3"]

Return only valid JSON array.
PROMPT;

        $response = $this->callAI($prompt);

        if (!$response['success']) {
            return [];
        }

        return $this->parseJsonResponse($response['content']);
    }

    // =========================================================================
    // FORM OPTIMIZATION
    // =========================================================================

    /**
     * Analyze form and provide optimization recommendations
     */
    public function analyzeForm(Form $form): array
    {
        $fields = $form->fields()->orderBy('order')->get();
        $fieldCount = $fields->count();

        $analysis = [
            'score' => 100,
            'issues' => [],
            'recommendations' => [],
            'field_analysis' => [],
        ];

        // Check field count
        if ($fieldCount > 15) {
            $analysis['score'] -= 10;
            $analysis['issues'][] = [
                'severity' => 'warning',
                'message' => 'Form has many fields (' . $fieldCount . '). Consider reducing for better conversion.',
            ];
        }

        // Check for required fields ratio
        $requiredCount = $fields->where('required', true)->count();
        $requiredRatio = $fieldCount > 0 ? $requiredCount / $fieldCount : 0;

        if ($requiredRatio > 0.8) {
            $analysis['score'] -= 5;
            $analysis['issues'][] = [
                'severity' => 'info',
                'message' => 'Most fields are required. Consider making some optional.',
            ];
        }

        // Check for email field
        $hasEmail = $fields->where('type', 'email')->isNotEmpty();
        if (!$hasEmail) {
            $analysis['recommendations'][] = [
                'type' => 'add_field',
                'message' => 'Consider adding an email field for follow-up.',
                'field' => ['type' => 'email', 'label' => 'Email'],
            ];
        }

        // Check for help text
        $fieldsWithHelp = $fields->filter(fn($f) => !empty($f->help_text))->count();
        if ($fieldsWithHelp < $fieldCount * 0.3) {
            $analysis['recommendations'][] = [
                'type' => 'add_help',
                'message' => 'Add help text to guide users through complex fields.',
            ];
        }

        // Check for placeholders
        $fieldsWithPlaceholder = $fields->filter(fn($f) => !empty($f->placeholder))->count();
        if ($fieldsWithPlaceholder < $fieldCount * 0.5) {
            $analysis['score'] -= 3;
            $analysis['recommendations'][] = [
                'type' => 'add_placeholders',
                'message' => 'Add placeholder text to improve field clarity.',
            ];
        }

        // AI-powered content analysis
        if ($this->apiKey) {
            $aiAnalysis = $this->getAIFormAnalysis($form, $fields);
            $analysis['ai_suggestions'] = $aiAnalysis;
        }

        return $analysis;
    }

    /**
     * Get AI-powered form analysis
     */
    private function getAIFormAnalysis(Form $form, $fields): array
    {
        $fieldsJson = $fields->map(fn($f) => [
            'type' => $f->type,
            'label' => $f->label,
            'required' => $f->required,
        ])->toJson();

        $prompt = <<<PROMPT
Analyze this form for conversion optimization:

Title: {$form->title}
Description: {$form->description}
Fields: {$fieldsJson}

Provide 3-5 specific, actionable recommendations to improve:
1. Conversion rate
2. User experience
3. Data quality

Return JSON:
{
    "recommendations": [
        {
            "category": "conversion|ux|data",
            "priority": "high|medium|low",
            "suggestion": "Specific suggestion",
            "impact": "Expected impact"
        }
    ]
}
PROMPT;

        $response = $this->callAI($prompt);

        if (!$response['success']) {
            return [];
        }

        $data = $this->parseJsonResponse($response['content']);
        return $data['recommendations'] ?? [];
    }

    // =========================================================================
    // CONTENT GENERATION
    // =========================================================================

    /**
     * Generate form content (labels, help text, etc.)
     */
    public function generateContent(string $type, array $context): array
    {
        $prompts = [
            'labels' => "Generate clear, professional field labels for a {$context['form_type']} form.",
            'help_text' => "Generate helpful guidance text for a field labeled '{$context['field_label']}'.",
            'success_message' => "Generate a friendly thank-you message for a {$context['form_type']} form.",
            'error_messages' => "Generate user-friendly validation error messages for common form errors.",
            'placeholder' => "Generate a helpful placeholder for a {$context['field_type']} field labeled '{$context['field_label']}'.",
        ];

        $prompt = $prompts[$type] ?? $prompts['labels'];

        $response = $this->callAI($prompt . "\n\nReturn only the text, no JSON or formatting.");

        return [
            'success' => $response['success'],
            'content' => $response['content'] ?? '',
        ];
    }

    /**
     * Translate form content
     */
    public function translateForm(Form $form, string $targetLanguage): array
    {
        $fields = $form->fields()->get();
        $content = [
            'title' => $form->title,
            'description' => $form->description,
            'fields' => $fields->map(fn($f) => [
                'id' => $f->id,
                'label' => $f->label,
                'placeholder' => $f->placeholder,
                'help_text' => $f->help_text,
            ])->toArray(),
        ];

        $prompt = <<<PROMPT
Translate this form content to {$targetLanguage}:

{$this->formatForTranslation($content)}

Return JSON with same structure but translated values.
Keep field IDs unchanged.
PROMPT;

        $response = $this->callAI($prompt);

        if (!$response['success']) {
            return ['success' => false, 'error' => $response['error'] ?? 'Translation failed'];
        }

        return [
            'success' => true,
            'translations' => $this->parseJsonResponse($response['content']),
            'target_language' => $targetLanguage,
        ];
    }

    // =========================================================================
    // SPAM DETECTION
    // =========================================================================

    /**
     * Analyze submission for spam
     */
    public function analyzeSubmissionForSpam(array $data, array $metadata = []): array
    {
        $score = 0;
        $reasons = [];

        // Check for URLs in text fields
        foreach ($data as $key => $value) {
            if (is_string($value)) {
                $urlCount = preg_match_all('/https?:\/\/[^\s]+/', $value);
                if ($urlCount > 2) {
                    $score += 30;
                    $reasons[] = 'Multiple URLs detected';
                }

                // Check for common spam patterns
                $spamPatterns = [
                    '/\b(viagra|cialis|casino|lottery|winner|prize|free money)\b/i',
                    '/\b(click here|buy now|limited offer|act now)\b/i',
                    '/<[^>]+>/', // HTML tags
                ];

                foreach ($spamPatterns as $pattern) {
                    if (preg_match($pattern, $value)) {
                        $score += 20;
                        $reasons[] = 'Spam pattern detected';
                    }
                }

                // Check for excessive caps
                $capsRatio = strlen(preg_replace('/[^A-Z]/', '', $value)) / max(strlen($value), 1);
                if ($capsRatio > 0.5 && strlen($value) > 20) {
                    $score += 10;
                    $reasons[] = 'Excessive capitalization';
                }
            }
        }

        // Check submission speed (if timestamp provided)
        if (!empty($metadata['form_load_time']) && !empty($metadata['submit_time'])) {
            $duration = $metadata['submit_time'] - $metadata['form_load_time'];
            if ($duration < 3) { // Less than 3 seconds
                $score += 40;
                $reasons[] = 'Submitted too quickly (bot behavior)';
            }
        }

        // AI-powered spam detection for edge cases
        if ($score < 50 && $score > 20 && $this->apiKey) {
            $aiResult = $this->aiSpamCheck($data);
            if ($aiResult['is_spam']) {
                $score += 30;
                $reasons = array_merge($reasons, $aiResult['reasons'] ?? []);
            }
        }

        return [
            'score' => min($score, 100),
            'is_spam' => $score >= 50,
            'confidence' => $score >= 70 ? 'high' : ($score >= 50 ? 'medium' : 'low'),
            'reasons' => array_unique($reasons),
        ];
    }

    /**
     * AI-powered spam check
     */
    private function aiSpamCheck(array $data): array
    {
        $prompt = <<<PROMPT
Analyze this form submission for spam indicators:

{$this->formatSubmissionData($data)}

Return JSON:
{
    "is_spam": true|false,
    "confidence": 0.0-1.0,
    "reasons": ["reason1", "reason2"]
}
PROMPT;

        $response = $this->callAI($prompt);

        if (!$response['success']) {
            return ['is_spam' => false];
        }

        return $this->parseJsonResponse($response['content']);
    }

    // =========================================================================
    // HELPER METHODS
    // =========================================================================

    /**
     * Call AI API
     */
    private function callAI(string $prompt): array
    {
        if (!$this->apiKey) {
            return [
                'success' => false,
                'error' => 'AI API key not configured',
            ];
        }

        $cacheKey = 'ai_response:' . md5($prompt);
        $cached = Cache::get($cacheKey);

        if ($cached) {
            return ['success' => true, 'content' => $cached, 'cached' => true];
        }

        try {
            $response = match ($this->provider) {
                'openai' => $this->callOpenAI($prompt),
                'anthropic' => $this->callAnthropic($prompt),
                default => $this->callOpenAI($prompt),
            };

            if ($response['success']) {
                Cache::put($cacheKey, $response['content'], now()->addHours(24));
            }

            return $response;
        } catch (\Exception $e) {
            return [
                'success' => false,
                'error' => $e->getMessage(),
            ];
        }
    }

    /**
     * Call OpenAI API
     */
    private function callOpenAI(string $prompt): array
    {
        $response = Http::withToken($this->apiKey)
            ->timeout(60)
            ->post('https://api.openai.com/v1/chat/completions', [
                'model' => $this->model,
                'messages' => [
                    ['role' => 'system', 'content' => 'You are a form design and optimization expert. Always respond with valid JSON when requested.'],
                    ['role' => 'user', 'content' => $prompt],
                ],
                'temperature' => 0.7,
                'max_tokens' => 2000,
            ]);

        if ($response->successful()) {
            return [
                'success' => true,
                'content' => $response->json('choices.0.message.content'),
            ];
        }

        return [
            'success' => false,
            'error' => $response->json('error.message', 'API request failed'),
        ];
    }

    /**
     * Call Anthropic API
     */
    private function callAnthropic(string $prompt): array
    {
        $response = Http::withHeaders([
            'x-api-key' => $this->apiKey,
            'anthropic-version' => '2023-06-01',
        ])
            ->timeout(60)
            ->post('https://api.anthropic.com/v1/messages', [
                'model' => $this->model,
                'max_tokens' => 2000,
                'messages' => [
                    ['role' => 'user', 'content' => $prompt],
                ],
            ]);

        if ($response->successful()) {
            return [
                'success' => true,
                'content' => $response->json('content.0.text'),
            ];
        }

        return [
            'success' => false,
            'error' => $response->json('error.message', 'API request failed'),
        ];
    }

    /**
     * Parse JSON response
     */
    private function parseJsonResponse(string $content): array
    {
        $content = trim($content);

        // Remove markdown code blocks
        if (str_starts_with($content, '```')) {
            $content = preg_replace('/^```(?:json)?\s*/', '', $content);
            $content = preg_replace('/\s*```$/', '', $content);
        }

        $data = json_decode($content, true);

        return is_array($data) ? $data : [];
    }

    /**
     * Format existing fields for prompt
     */
    private function formatExistingFields(array $fields): string
    {
        if (empty($fields)) {
            return 'None';
        }

        return implode(', ', array_map(fn($f) => $f['label'] ?? $f['name'] ?? 'Unknown', $fields));
    }

    /**
     * Format content for translation
     */
    private function formatForTranslation(array $content): string
    {
        return json_encode($content, JSON_PRETTY_PRINT | JSON_UNESCAPED_UNICODE);
    }

    /**
     * Format submission data for analysis
     */
    private function formatSubmissionData(array $data): string
    {
        return collect($data)->map(fn($v, $k) => "{$k}: " . (is_array($v) ? json_encode($v) : $v))->implode("\n");
    }

    /**
     * Get form suggestions
     */
    private function getFormSuggestions(array $formData): array
    {
        return [
            'consider_adding' => $this->suggestMissingFields($formData['fields'] ?? []),
            'optimization_tips' => [
                'Keep the form as short as possible',
                'Use clear, action-oriented submit button text',
                'Add progress indicators for multi-step forms',
            ],
        ];
    }

    /**
     * Suggest missing common fields
     */
    private function suggestMissingFields(array $fields): array
    {
        $fieldTypes = array_column($fields, 'type');
        $suggestions = [];

        $commonFields = [
            'email' => 'Email field for follow-up communication',
            'phone' => 'Phone field for urgent contact',
            'consent' => 'Privacy consent checkbox for GDPR compliance',
        ];

        foreach ($commonFields as $type => $reason) {
            if (!in_array($type, $fieldTypes)) {
                $suggestions[] = [
                    'type' => $type,
                    'reason' => $reason,
                ];
            }
        }

        return $suggestions;
    }
}
