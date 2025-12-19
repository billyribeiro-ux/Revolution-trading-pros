<?php

declare(strict_types=1);

namespace App\Http\Requests\Api\ContentLake;

use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Contracts\Validation\Validator;
use Illuminate\Http\Exceptions\HttpResponseException;

/**
 * Schema Request Validation
 *
 * Validates content schema definitions with comprehensive field type checking.
 *
 * @level ICT11 Principal Engineer
 */
class SchemaRequest extends FormRequest
{
    /**
     * Allowed field types
     */
    private const ALLOWED_TYPES = [
        'string',
        'text',
        'number',
        'boolean',
        'datetime',
        'date',
        'url',
        'slug',
        'image',
        'file',
        'reference',
        'array',
        'object',
        'block',
        'geopoint',
        'color',
        'code',
        'markdown',
    ];

    /**
     * Reserved schema names
     */
    private const RESERVED_NAMES = [
        'system',
        'user',
        'asset',
        'reference',
        'document',
        'block',
        '_type',
        '_id',
        '_key',
        '_ref',
        '_rev',
        '_createdAt',
        '_updatedAt',
    ];

    public function authorize(): bool
    {
        return auth()->check();
    }

    public function rules(): array
    {
        return [
            'name' => [
                'required',
                'string',
                'max:100',
                'regex:/^[a-zA-Z][a-zA-Z0-9_]*$/',
                function ($attribute, $value, $fail) {
                    if (in_array(strtolower($value), array_map('strtolower', self::RESERVED_NAMES))) {
                        $fail('Schema name is reserved and cannot be used.');
                    }
                },
            ],
            'title' => ['nullable', 'string', 'max:255'],
            'description' => ['nullable', 'string', 'max:1000'],
            'fields' => ['required', 'array', 'min:1', 'max:100'],
            'fields.*.name' => [
                'required',
                'string',
                'max:100',
                'regex:/^[a-zA-Z][a-zA-Z0-9_]*$/',
            ],
            'fields.*.type' => [
                'required',
                'string',
                'in:' . implode(',', self::ALLOWED_TYPES),
            ],
            'fields.*.title' => ['nullable', 'string', 'max:255'],
            'fields.*.description' => ['nullable', 'string', 'max:500'],
            'fields.*.required' => ['nullable', 'boolean'],
            'fields.*.hidden' => ['nullable', 'boolean'],
            'fields.*.readOnly' => ['nullable', 'boolean'],
            'fields.*.options' => ['nullable', 'array'],
            'fields.*.validation' => ['nullable', 'array'],
            'fields.*.of' => ['nullable', 'array'], // For array types
            'fields.*.to' => ['nullable', 'array'], // For reference types
            'fieldsets' => ['nullable', 'array'],
            'fieldsets.*.name' => ['required_with:fieldsets', 'string', 'max:100'],
            'fieldsets.*.title' => ['nullable', 'string', 'max:255'],
            'fieldsets.*.options' => ['nullable', 'array'],
            'preview' => ['nullable', 'array'],
            'preview.select' => ['nullable', 'array'],
            'preview.prepare' => ['nullable', 'string'],
            'orderings' => ['nullable', 'array'],
            'orderings.*.name' => ['required_with:orderings', 'string', 'max:100'],
            'orderings.*.title' => ['nullable', 'string', 'max:255'],
            'orderings.*.by' => ['required_with:orderings', 'array'],
            'validation' => ['nullable', 'array'],
            'initialValue' => ['nullable', 'array'],
            'icon' => ['nullable', 'string', 'max:100'],
            'liveEdit' => ['nullable', 'boolean'],
        ];
    }

    public function messages(): array
    {
        return [
            'name.required' => 'Schema name is required.',
            'name.regex' => 'Schema name must start with a letter and contain only letters, numbers, and underscores.',
            'fields.required' => 'At least one field is required.',
            'fields.min' => 'At least one field is required.',
            'fields.max' => 'Maximum of 100 fields allowed per schema.',
            'fields.*.name.regex' => 'Field names must start with a letter and contain only letters, numbers, and underscores.',
            'fields.*.type.in' => 'Invalid field type. Allowed: ' . implode(', ', self::ALLOWED_TYPES),
        ];
    }

    /**
     * Prepare the data for validation.
     */
    protected function prepareForValidation(): void
    {
        // Ensure field names are unique
        if ($this->has('fields')) {
            $fields = $this->input('fields');
            $names = [];

            foreach ($fields as $field) {
                if (isset($field['name'])) {
                    $names[] = $field['name'];
                }
            }

            if (count($names) !== count(array_unique($names))) {
                // Will be caught by additional validation
            }
        }
    }

    /**
     * Configure the validator instance.
     */
    public function withValidator(Validator $validator): void
    {
        $validator->after(function (Validator $validator) {
            $this->validateUniqueFieldNames($validator);
            $this->validateReferenceTypes($validator);
        });
    }

    /**
     * Validate that all field names are unique
     */
    private function validateUniqueFieldNames(Validator $validator): void
    {
        $fields = $this->input('fields', []);
        $names = [];

        foreach ($fields as $index => $field) {
            $name = $field['name'] ?? null;
            if ($name && in_array($name, $names)) {
                $validator->errors()->add(
                    "fields.{$index}.name",
                    "Duplicate field name '{$name}'. Field names must be unique."
                );
            }
            $names[] = $name;
        }
    }

    /**
     * Validate reference type configurations
     */
    private function validateReferenceTypes(Validator $validator): void
    {
        $fields = $this->input('fields', []);

        foreach ($fields as $index => $field) {
            if (($field['type'] ?? '') === 'reference' && empty($field['to'])) {
                $validator->errors()->add(
                    "fields.{$index}.to",
                    "Reference fields must specify target types in 'to' property."
                );
            }

            if (($field['type'] ?? '') === 'array' && empty($field['of'])) {
                $validator->errors()->add(
                    "fields.{$index}.of",
                    "Array fields must specify item types in 'of' property."
                );
            }
        }
    }

    protected function failedValidation(Validator $validator): void
    {
        throw new HttpResponseException(
            response()->json([
                'success' => false,
                'message' => 'Invalid schema definition',
                'error_code' => 'VALIDATION_ERROR',
                'errors' => $validator->errors(),
            ], 422)
        );
    }
}
