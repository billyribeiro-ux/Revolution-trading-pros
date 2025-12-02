<?php

declare(strict_types=1);

namespace App\Http\Requests\Api\ContentLake;

use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Contracts\Validation\Validator;
use Illuminate\Http\Exceptions\HttpResponseException;

/**
 * Update Draft Request Validation
 *
 * Validates document draft update requests with content sanitization.
 *
 * @level ICT11 Principal Engineer
 */
class UpdateDraftRequest extends FormRequest
{
    /**
     * Maximum content size in characters (5MB text)
     */
    private const MAX_CONTENT_SIZE = 5242880;

    public function authorize(): bool
    {
        return auth()->check();
    }

    public function rules(): array
    {
        return [
            'content' => [
                'required',
                'array',
                function ($attribute, $value, $fail) {
                    $this->validateContentSize($value, $fail);
                },
            ],
            'content.title' => ['nullable', 'string', 'max:500'],
            'content.slug' => ['nullable', 'string', 'max:255', 'regex:/^[a-z0-9]+(?:-[a-z0-9]+)*$/'],
            'content.excerpt' => ['nullable', 'string', 'max:2000'],
            'content.body' => ['nullable', 'array'],
            'content.meta' => ['nullable', 'array'],
            'content.meta.title' => ['nullable', 'string', 'max:70'],
            'content.meta.description' => ['nullable', 'string', 'max:160'],
            'autoSave' => ['nullable', 'boolean'],
        ];
    }

    public function messages(): array
    {
        return [
            'content.required' => 'Document content is required.',
            'content.slug.regex' => 'Slug must contain only lowercase letters, numbers, and hyphens.',
            'content.meta.title.max' => 'Meta title should not exceed 70 characters for SEO.',
            'content.meta.description.max' => 'Meta description should not exceed 160 characters for SEO.',
        ];
    }

    /**
     * Validate total content size
     */
    private function validateContentSize(array $content, callable $fail): void
    {
        $serialized = json_encode($content);

        if ($serialized === false) {
            $fail('Content contains invalid data that cannot be serialized.');
            return;
        }

        if (strlen($serialized) > self::MAX_CONTENT_SIZE) {
            $fail('Content exceeds maximum allowed size of 5MB.');
        }
    }

    protected function failedValidation(Validator $validator): void
    {
        throw new HttpResponseException(
            response()->json([
                'success' => false,
                'message' => 'Invalid draft content',
                'error_code' => 'VALIDATION_ERROR',
                'errors' => $validator->errors(),
            ], 422)
        );
    }
}
