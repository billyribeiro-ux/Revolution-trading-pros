<?php

declare(strict_types=1);

namespace App\Http\Requests\Api\ContentLake;

use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Contracts\Validation\Validator;
use Illuminate\Http\Exceptions\HttpResponseException;

/**
 * Compare Revisions Request Validation
 *
 * Validates revision comparison requests.
 *
 * @level ICT11 Principal Engineer
 */
class CompareRevisionsRequest extends FormRequest
{
    public function authorize(): bool
    {
        return auth()->check();
    }

    public function rules(): array
    {
        return [
            'from' => ['required', 'integer', 'min:1', 'max:999999'],
            'to' => ['required', 'integer', 'min:1', 'max:999999', 'different:from'],
        ];
    }

    public function messages(): array
    {
        return [
            'from.required' => 'Source revision number is required.',
            'to.required' => 'Target revision number is required.',
            'from.min' => 'Revision number must be at least 1.',
            'to.min' => 'Revision number must be at least 1.',
            'to.different' => 'Source and target revisions must be different.',
        ];
    }

    protected function failedValidation(Validator $validator): void
    {
        throw new HttpResponseException(
            response()->json([
                'success' => false,
                'message' => 'Invalid revision comparison request',
                'error_code' => 'VALIDATION_ERROR',
                'errors' => $validator->errors(),
            ], 422)
        );
    }
}
