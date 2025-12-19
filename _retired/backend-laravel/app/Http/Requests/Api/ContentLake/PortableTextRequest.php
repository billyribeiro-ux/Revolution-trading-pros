<?php

declare(strict_types=1);

namespace App\Http\Requests\Api\ContentLake;

use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Contracts\Validation\Validator;
use Illuminate\Http\Exceptions\HttpResponseException;

/**
 * Portable Text Request Validation
 *
 * Validates Portable Text block structures for rendering and validation.
 *
 * @level ICT11 Principal Engineer
 */
class PortableTextRequest extends FormRequest
{
    /**
     * Maximum allowed blocks to prevent DoS
     */
    private const MAX_BLOCKS = 1000;

    /**
     * Maximum nesting depth for blocks
     */
    private const MAX_NESTING_DEPTH = 10;

    public function authorize(): bool
    {
        return true;
    }

    public function rules(): array
    {
        return [
            'blocks' => [
                'required',
                'array',
                'min:1',
                'max:' . self::MAX_BLOCKS,
                function ($attribute, $value, $fail) {
                    $this->validateBlockStructure($value, $fail);
                },
            ],
            'blocks.*._type' => ['required_with:blocks', 'string', 'max:100'],
            'blocks.*._key' => ['nullable', 'string', 'max:50'],
            'options' => ['nullable', 'array'],
            'options.serializers' => ['nullable', 'array'],
        ];
    }

    public function messages(): array
    {
        return [
            'blocks.required' => 'Portable Text blocks are required.',
            'blocks.min' => 'At least one block is required.',
            'blocks.max' => 'Too many blocks. Maximum is ' . self::MAX_BLOCKS . '.',
            'blocks.*._type.required_with' => 'Each block must have a _type field.',
        ];
    }

    /**
     * Validate block structure for security and integrity
     */
    private function validateBlockStructure(array $blocks, callable $fail): void
    {
        $depth = $this->calculateMaxDepth($blocks);

        if ($depth > self::MAX_NESTING_DEPTH) {
            $fail('Block nesting exceeds maximum depth of ' . self::MAX_NESTING_DEPTH . '.');
            return;
        }

        // Validate each block has required structure
        foreach ($blocks as $index => $block) {
            if (!is_array($block)) {
                $fail("Block at index {$index} must be an array.");
                return;
            }

            if (!isset($block['_type'])) {
                $fail("Block at index {$index} is missing required '_type' field.");
                return;
            }
        }
    }

    /**
     * Calculate maximum nesting depth of blocks
     */
    private function calculateMaxDepth(array $data, int $currentDepth = 0): int
    {
        $maxDepth = $currentDepth;

        foreach ($data as $item) {
            if (is_array($item)) {
                $depth = $this->calculateMaxDepth($item, $currentDepth + 1);
                $maxDepth = max($maxDepth, $depth);
            }
        }

        return $maxDepth;
    }

    protected function failedValidation(Validator $validator): void
    {
        throw new HttpResponseException(
            response()->json([
                'success' => false,
                'message' => 'Invalid Portable Text structure',
                'error_code' => 'VALIDATION_ERROR',
                'errors' => $validator->errors(),
            ], 422)
        );
    }
}
