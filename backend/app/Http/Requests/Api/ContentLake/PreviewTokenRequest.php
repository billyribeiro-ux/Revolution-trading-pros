<?php

declare(strict_types=1);

namespace App\Http\Requests\Api\ContentLake;

use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Contracts\Validation\Validator;
use Illuminate\Http\Exceptions\HttpResponseException;

/**
 * Preview Token Request Validation
 *
 * Validates preview token generation requests.
 *
 * @level ICT11 Principal Engineer
 */
class PreviewTokenRequest extends FormRequest
{
    /**
     * Minimum TTL: 1 minute
     */
    private const MIN_TTL = 60;

    /**
     * Maximum TTL: 7 days
     */
    private const MAX_TTL = 604800;

    public function authorize(): bool
    {
        return auth()->check();
    }

    public function rules(): array
    {
        return [
            'ttl' => [
                'nullable',
                'integer',
                'min:' . self::MIN_TTL,
                'max:' . self::MAX_TTL,
            ],
            'allowed_ips' => ['nullable', 'array', 'max:10'],
            'allowed_ips.*' => ['ip'],
            'single_use' => ['nullable', 'boolean'],
        ];
    }

    public function messages(): array
    {
        return [
            'ttl.min' => 'Token TTL must be at least 1 minute (60 seconds).',
            'ttl.max' => 'Token TTL cannot exceed 7 days (604800 seconds).',
            'allowed_ips.max' => 'Maximum of 10 allowed IP addresses.',
            'allowed_ips.*.ip' => 'Please provide valid IP addresses.',
        ];
    }

    protected function failedValidation(Validator $validator): void
    {
        throw new HttpResponseException(
            response()->json([
                'success' => false,
                'message' => 'Invalid preview token request',
                'error_code' => 'VALIDATION_ERROR',
                'errors' => $validator->errors(),
            ], 422)
        );
    }
}
