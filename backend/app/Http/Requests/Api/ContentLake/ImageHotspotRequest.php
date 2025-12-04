<?php

declare(strict_types=1);

namespace App\Http\Requests\Api\ContentLake;

use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Contracts\Validation\Validator;
use Illuminate\Http\Exceptions\HttpResponseException;

/**
 * Image Hotspot Request Validation
 *
 * Validates hotspot and crop configurations for images.
 *
 * @level ICT11 Principal Engineer
 */
class ImageHotspotRequest extends FormRequest
{
    public function authorize(): bool
    {
        return true;
    }

    public function rules(): array
    {
        return [
            'x' => ['required', 'numeric', 'min:0', 'max:1'],
            'y' => ['required', 'numeric', 'min:0', 'max:1'],
            'width' => ['nullable', 'numeric', 'min:0.01', 'max:1'],
            'height' => ['nullable', 'numeric', 'min:0.01', 'max:1'],
            'name' => ['nullable', 'string', 'max:100', 'regex:/^[a-zA-Z][a-zA-Z0-9_-]*$/'],
        ];
    }

    public function messages(): array
    {
        return [
            'x.required' => 'Hotspot X coordinate is required.',
            'x.numeric' => 'X coordinate must be a number.',
            'x.min' => 'X coordinate must be between 0 and 1.',
            'x.max' => 'X coordinate must be between 0 and 1.',
            'y.required' => 'Hotspot Y coordinate is required.',
            'y.numeric' => 'Y coordinate must be a number.',
            'y.min' => 'Y coordinate must be between 0 and 1.',
            'y.max' => 'Y coordinate must be between 0 and 1.',
            'name.regex' => 'Hotspot name must start with a letter and contain only letters, numbers, underscores, and hyphens.',
        ];
    }

    protected function failedValidation(Validator $validator): void
    {
        throw new HttpResponseException(
            response()->json([
                'success' => false,
                'message' => 'Invalid hotspot configuration',
                'error_code' => 'VALIDATION_ERROR',
                'errors' => $validator->errors(),
            ], 422)
        );
    }
}
