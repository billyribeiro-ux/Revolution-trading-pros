<?php

declare(strict_types=1);

namespace App\Http\Requests\Api\ContentLake;

use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Contracts\Validation\Validator;
use Illuminate\Http\Exceptions\HttpResponseException;

/**
 * Image Crop Request Validation
 *
 * Validates image crop configurations.
 *
 * @level ICT11 Principal Engineer
 */
class ImageCropRequest extends FormRequest
{
    /**
     * Common aspect ratios
     */
    private const VALID_ASPECT_RATIOS = [
        '1:1',
        '4:3',
        '3:2',
        '16:9',
        '21:9',
        '2:3',
        '3:4',
        '9:16',
        'free',
    ];

    public function authorize(): bool
    {
        return true;
    }

    public function rules(): array
    {
        return [
            'name' => [
                'required',
                'string',
                'max:100',
                'regex:/^[a-zA-Z][a-zA-Z0-9_-]*$/',
            ],
            'top' => ['required', 'numeric', 'min:0', 'max:1'],
            'left' => ['required', 'numeric', 'min:0', 'max:1'],
            'bottom' => ['required', 'numeric', 'min:0', 'max:1'],
            'right' => ['required', 'numeric', 'min:0', 'max:1'],
            'aspectRatio' => [
                'nullable',
                'string',
                function ($attribute, $value, $fail) {
                    if ($value && !$this->isValidAspectRatio($value)) {
                        $fail('Invalid aspect ratio format. Use format like "16:9" or "free".');
                    }
                },
            ],
        ];
    }

    public function messages(): array
    {
        return [
            'name.required' => 'Crop preset name is required.',
            'name.regex' => 'Crop name must start with a letter and contain only letters, numbers, underscores, and hyphens.',
            'top.required' => 'Top edge value is required.',
            'left.required' => 'Left edge value is required.',
            'bottom.required' => 'Bottom edge value is required.',
            'right.required' => 'Right edge value is required.',
            '*.min' => 'Edge values must be between 0 and 1.',
            '*.max' => 'Edge values must be between 0 and 1.',
        ];
    }

    /**
     * Configure the validator instance.
     */
    public function withValidator(Validator $validator): void
    {
        $validator->after(function (Validator $validator) {
            $this->validateCropBounds($validator);
        });
    }

    /**
     * Validate that crop bounds are logically consistent
     */
    private function validateCropBounds(Validator $validator): void
    {
        $top = $this->input('top');
        $left = $this->input('left');
        $bottom = $this->input('bottom');
        $right = $this->input('right');

        if ($top !== null && $bottom !== null && $top >= $bottom) {
            $validator->errors()->add('top', 'Top edge must be less than bottom edge.');
        }

        if ($left !== null && $right !== null && $left >= $right) {
            $validator->errors()->add('left', 'Left edge must be less than right edge.');
        }

        // Ensure crop area is at least 1% of the image
        if ($top !== null && $bottom !== null && $left !== null && $right !== null) {
            $width = $right - $left;
            $height = $bottom - $top;

            if ($width * $height < 0.01) {
                $validator->errors()->add('crop', 'Crop area must be at least 1% of the image.');
            }
        }
    }

    /**
     * Validate aspect ratio format
     */
    private function isValidAspectRatio(string $ratio): bool
    {
        if (in_array($ratio, self::VALID_ASPECT_RATIOS)) {
            return true;
        }

        // Check custom ratio format (e.g., "5:4")
        return preg_match('/^\d+:\d+$/', $ratio) === 1;
    }

    protected function failedValidation(Validator $validator): void
    {
        throw new HttpResponseException(
            response()->json([
                'success' => false,
                'message' => 'Invalid crop configuration',
                'error_code' => 'VALIDATION_ERROR',
                'errors' => $validator->errors(),
            ], 422)
        );
    }
}
