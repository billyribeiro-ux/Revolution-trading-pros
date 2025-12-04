<?php

declare(strict_types=1);

namespace App\Http\Requests\Api\ContentLake;

use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Contracts\Validation\Validator;
use Illuminate\Http\Exceptions\HttpResponseException;

/**
 * Webhook Request Validation
 *
 * Validates webhook configuration with security-focused URL validation
 * and event type restrictions.
 *
 * @level ICT11 Principal Engineer
 */
class WebhookRequest extends FormRequest
{
    /**
     * Allowed webhook event types
     */
    private const ALLOWED_EVENTS = [
        'document.created',
        'document.updated',
        'document.deleted',
        'document.published',
        'document.unpublished',
        'asset.created',
        'asset.updated',
        'asset.deleted',
        'release.published',
        'release.scheduled',
        'workflow.transition',
    ];

    /**
     * Blocked URL patterns for security
     */
    private const BLOCKED_PATTERNS = [
        '/^https?:\/\/localhost/i',
        '/^https?:\/\/127\./i',
        '/^https?:\/\/10\./i',
        '/^https?:\/\/172\.(1[6-9]|2[0-9]|3[0-1])\./i',
        '/^https?:\/\/192\.168\./i',
        '/^https?:\/\/\[::1\]/i',
        '/^https?:\/\/0\./i',
        '/^file:/i',
        '/^ftp:/i',
    ];

    public function authorize(): bool
    {
        return auth()->check();
    }

    public function rules(): array
    {
        $isUpdate = $this->isMethod('PATCH') || $this->isMethod('PUT');

        return [
            'name' => [$isUpdate ? 'nullable' : 'required', 'string', 'max:255'],
            'url' => [
                $isUpdate ? 'nullable' : 'required',
                'url',
                'max:2048',
                function ($attribute, $value, $fail) {
                    if ($value) {
                        $this->validateUrl($value, $fail);
                    }
                },
            ],
            'events' => [$isUpdate ? 'nullable' : 'required', 'array', 'min:1', 'max:20'],
            'events.*' => ['string', 'in:' . implode(',', self::ALLOWED_EVENTS)],
            'filter' => ['nullable', 'array'],
            'filter.documentTypes' => ['nullable', 'array'],
            'filter.documentTypes.*' => ['string', 'max:100'],
            'projection' => ['nullable', 'array'],
            'projection.*' => ['string', 'max:100'],
            'headers' => ['nullable', 'array', 'max:20'],
            'headers.*.name' => ['required_with:headers', 'string', 'max:100'],
            'headers.*.value' => ['required_with:headers', 'string', 'max:1000'],
            'is_active' => ['nullable', 'boolean'],
            'secret' => ['nullable', 'string', 'min:32', 'max:256'],
            'retry_config' => ['nullable', 'array'],
            'retry_config.max_attempts' => ['nullable', 'integer', 'min:0', 'max:10'],
            'retry_config.backoff_ms' => ['nullable', 'integer', 'min:100', 'max:60000'],
        ];
    }

    public function messages(): array
    {
        return [
            'name.required' => 'Webhook name is required.',
            'url.required' => 'Webhook URL is required.',
            'url.url' => 'Please provide a valid HTTPS URL.',
            'events.required' => 'At least one event type is required.',
            'events.*.in' => 'Invalid event type. Allowed: ' . implode(', ', self::ALLOWED_EVENTS),
            'secret.min' => 'Webhook secret must be at least 32 characters for security.',
        ];
    }

    /**
     * Validate webhook URL for security
     */
    private function validateUrl(string $url, callable $fail): void
    {
        // Must use HTTPS in production
        if (app()->environment('production') && !str_starts_with($url, 'https://')) {
            $fail('Webhook URLs must use HTTPS in production.');
            return;
        }

        // Block internal/private network addresses
        foreach (self::BLOCKED_PATTERNS as $pattern) {
            if (preg_match($pattern, $url)) {
                $fail('Webhook URLs cannot target internal or private network addresses.');
                return;
            }
        }

        // Validate URL is resolvable (optional, for stricter security)
        $host = parse_url($url, PHP_URL_HOST);
        if ($host && filter_var($host, FILTER_VALIDATE_IP)) {
            // If it's an IP, make sure it's not private
            if (!filter_var($host, FILTER_VALIDATE_IP, FILTER_FLAG_NO_PRIV_RANGE | FILTER_FLAG_NO_RES_RANGE)) {
                $fail('Webhook URLs cannot target private or reserved IP addresses.');
            }
        }
    }

    protected function failedValidation(Validator $validator): void
    {
        throw new HttpResponseException(
            response()->json([
                'success' => false,
                'message' => 'Invalid webhook configuration',
                'error_code' => 'VALIDATION_ERROR',
                'errors' => $validator->errors(),
            ], 422)
        );
    }
}
