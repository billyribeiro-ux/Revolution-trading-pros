<?php

declare(strict_types=1);

namespace App\Http\Requests\Api\ContentLake;

use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Contracts\Validation\Validator;
use Illuminate\Http\Exceptions\HttpResponseException;

/**
 * GROQ Query Request Validation
 *
 * Validates GROQ query requests with security-focused rules
 * to prevent injection attacks and resource exhaustion.
 *
 * @level ICT11 Principal Engineer
 */
class GroqQueryRequest extends FormRequest
{
    /**
     * Determine if the user is authorized to make this request.
     */
    public function authorize(): bool
    {
        return true; // Authorization handled by middleware
    }

    /**
     * Get the validation rules that apply to the request.
     *
     * @return array<string, \Illuminate\Contracts\Validation\ValidationRule|array|string>
     */
    public function rules(): array
    {
        return [
            'query' => [
                'required',
                'string',
                'max:10000',
                function ($attribute, $value, $fail) {
                    $this->validateGroqSyntax($value, $fail);
                },
            ],
            'params' => ['nullable', 'array', 'max:50'],
            'params.*' => ['nullable'],
            'useCache' => ['nullable', 'boolean'],
            'perspective' => ['nullable', 'string', 'in:draft,published,previewDrafts'],
        ];
    }

    /**
     * Get custom messages for validator errors.
     */
    public function messages(): array
    {
        return [
            'query.required' => 'A GROQ query is required.',
            'query.max' => 'The GROQ query exceeds the maximum allowed length of 10,000 characters.',
            'params.max' => 'Too many parameters provided. Maximum is 50.',
            'perspective.in' => 'Invalid perspective. Must be one of: draft, published, previewDrafts.',
        ];
    }

    /**
     * Validate GROQ query syntax for security
     */
    private function validateGroqSyntax(string $query, callable $fail): void
    {
        // Prevent extremely nested queries (DoS prevention)
        $nestingLevel = max(
            substr_count($query, '['),
            substr_count($query, '{'),
            substr_count($query, '(')
        );

        if ($nestingLevel > 20) {
            $fail('Query nesting level exceeds maximum allowed depth.');
            return;
        }

        // Detect potentially dangerous operations
        $dangerousPatterns = [
            '/\beval\s*\(/i',
            '/\bexec\s*\(/i',
            '/\bsystem\s*\(/i',
            '/--/',  // SQL comment attempt
            '/;.*;/', // Multiple statement attempt
        ];

        foreach ($dangerousPatterns as $pattern) {
            if (preg_match($pattern, $query)) {
                $fail('Query contains potentially dangerous patterns.');
                return;
            }
        }
    }

    /**
     * Handle a failed validation attempt.
     */
    protected function failedValidation(Validator $validator): void
    {
        throw new HttpResponseException(
            response()->json([
                'success' => false,
                'message' => 'Validation failed',
                'error_code' => 'VALIDATION_ERROR',
                'errors' => $validator->errors(),
            ], 422)
        );
    }
}
