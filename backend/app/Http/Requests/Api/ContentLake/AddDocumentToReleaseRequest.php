<?php

declare(strict_types=1);

namespace App\Http\Requests\Api\ContentLake;

use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Contracts\Validation\Validator;
use Illuminate\Http\Exceptions\HttpResponseException;

/**
 * Add Document to Release Request Validation
 *
 * Validates document addition to release bundles.
 *
 * @level ICT11 Principal Engineer
 */
class AddDocumentToReleaseRequest extends FormRequest
{
    private const ALLOWED_ACTIONS = ['create', 'update', 'delete'];

    public function authorize(): bool
    {
        return auth()->check();
    }

    public function rules(): array
    {
        return [
            'documentId' => [
                'required',
                'string',
                'max:255',
                'regex:/^[a-zA-Z0-9_-]+$/',
            ],
            'documentType' => [
                'required',
                'string',
                'max:100',
                'regex:/^[a-zA-Z][a-zA-Z0-9_]*$/',
            ],
            'action' => [
                'nullable',
                'string',
                'in:' . implode(',', self::ALLOWED_ACTIONS),
            ],
        ];
    }

    public function messages(): array
    {
        return [
            'documentId.required' => 'Document ID is required.',
            'documentId.regex' => 'Document ID contains invalid characters.',
            'documentType.required' => 'Document type is required.',
            'documentType.regex' => 'Document type must start with a letter and contain only letters, numbers, and underscores.',
            'action.in' => 'Action must be one of: create, update, delete.',
        ];
    }

    protected function failedValidation(Validator $validator): void
    {
        throw new HttpResponseException(
            response()->json([
                'success' => false,
                'message' => 'Invalid document addition request',
                'error_code' => 'VALIDATION_ERROR',
                'errors' => $validator->errors(),
            ], 422)
        );
    }
}
