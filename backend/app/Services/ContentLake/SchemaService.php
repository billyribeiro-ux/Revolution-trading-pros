<?php

declare(strict_types=1);

namespace App\Services\ContentLake;

use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Validator;
use Illuminate\Support\Str;
use InvalidArgumentException;

/**
 * Schema Service
 *
 * Implements Sanity-style schema-as-code for content types.
 *
 * Features:
 * - Define content types programmatically
 * - Field types (string, text, number, boolean, array, object, reference, etc.)
 * - Validation rules
 * - Field groups (fieldsets)
 * - Preview configuration
 * - Initial values
 */
class SchemaService
{
    /**
     * Built-in field types
     */
    private const FIELD_TYPES = [
        'string' => ['max' => 255],
        'text' => ['max' => 65535],
        'number' => ['min' => null, 'max' => null],
        'boolean' => [],
        'date' => ['format' => 'Y-m-d'],
        'datetime' => ['format' => 'Y-m-d H:i:s'],
        'url' => [],
        'email' => [],
        'slug' => ['source' => 'title'],
        'image' => ['accept' => 'image/*'],
        'file' => ['accept' => '*/*'],
        'array' => ['of' => []],
        'object' => ['fields' => []],
        'reference' => ['to' => []],
        'portableText' => [],
        'geopoint' => [],
        'color' => [],
    ];

    /**
     * Register a content schema
     */
    public function register(array $schema): array
    {
        $this->validateSchemaDefinition($schema);

        $name = $schema['name'];

        // Check if schema exists
        $existing = DB::table('content_schemas')
            ->where('name', $name)
            ->first();

        $data = [
            'name' => $name,
            'title' => $schema['title'] ?? Str::title(str_replace('_', ' ', $name)),
            'description' => $schema['description'] ?? null,
            'icon' => $schema['icon'] ?? null,
            'fields' => json_encode($schema['fields'] ?? []),
            'fieldsets' => isset($schema['fieldsets']) ? json_encode($schema['fieldsets']) : null,
            'preview' => isset($schema['preview']) ? json_encode($schema['preview']) : null,
            'orderings' => isset($schema['orderings']) ? json_encode($schema['orderings']) : null,
            'validation' => isset($schema['validation']) ? json_encode($schema['validation']) : null,
            'initial_value' => isset($schema['initialValue']) ? json_encode($schema['initialValue']) : null,
            'is_document' => $schema['type'] ?? 'document' === 'document',
            'is_singleton' => $schema['singleton'] ?? false,
            'is_system' => $schema['system'] ?? false,
            'updated_at' => now(),
        ];

        if ($existing) {
            $data['version'] = $existing->version + 1;
            DB::table('content_schemas')
                ->where('id', $existing->id)
                ->update($data);
            $id = $existing->id;
        } else {
            $data['version'] = 1;
            $data['created_at'] = now();
            $id = DB::table('content_schemas')->insertGetId($data);
        }

        return $this->get($name);
    }

    /**
     * Get schema by name
     */
    public function get(string $name): ?array
    {
        $schema = DB::table('content_schemas')
            ->where('name', $name)
            ->first();

        if (!$schema) {
            return null;
        }

        return $this->formatSchema($schema);
    }

    /**
     * List all schemas
     */
    public function list(array $options = []): array
    {
        $query = DB::table('content_schemas');

        if (isset($options['type'])) {
            $query->where('is_document', $options['type'] === 'document');
        }

        if (isset($options['system'])) {
            $query->where('is_system', $options['system']);
        }

        return $query->orderBy('name')
            ->get()
            ->map(fn($s) => $this->formatSchema($s))
            ->toArray();
    }

    /**
     * Delete schema
     */
    public function delete(string $name): bool
    {
        return DB::table('content_schemas')
            ->where('name', $name)
            ->where('is_system', false)
            ->delete() > 0;
    }

    /**
     * Validate document against schema
     */
    public function validate(string $schemaName, array $document): array
    {
        $schema = $this->get($schemaName);

        if (!$schema) {
            throw new InvalidArgumentException("Schema not found: {$schemaName}");
        }

        $rules = $this->buildValidationRules($schema['fields']);
        $validator = Validator::make($document, $rules);

        if ($validator->fails()) {
            return [
                'valid' => false,
                'errors' => $validator->errors()->toArray(),
            ];
        }

        // Run custom validation if defined
        if (!empty($schema['validation'])) {
            $customErrors = $this->runCustomValidation($document, $schema['validation']);
            if (!empty($customErrors)) {
                return [
                    'valid' => false,
                    'errors' => $customErrors,
                ];
            }
        }

        return ['valid' => true, 'errors' => []];
    }

    /**
     * Get initial values for schema
     */
    public function getInitialValue(string $schemaName): array
    {
        $schema = $this->get($schemaName);

        if (!$schema) {
            return [];
        }

        $initial = $schema['initialValue'] ?? [];

        // Add defaults from field definitions
        foreach ($schema['fields'] as $field) {
            $name = $field['name'];
            if (!isset($initial[$name]) && isset($field['initialValue'])) {
                $initial[$name] = $field['initialValue'];
            }
        }

        return $initial;
    }

    /**
     * Build validation rules from fields
     */
    private function buildValidationRules(array $fields): array
    {
        $rules = [];

        foreach ($fields as $field) {
            $fieldRules = [];
            $name = $field['name'];
            $type = $field['type'];

            // Required
            if ($field['validation']['required'] ?? false) {
                $fieldRules[] = 'required';
            } else {
                $fieldRules[] = 'nullable';
            }

            // Type-specific rules
            switch ($type) {
                case 'string':
                    $fieldRules[] = 'string';
                    if (isset($field['validation']['max'])) {
                        $fieldRules[] = 'max:' . $field['validation']['max'];
                    }
                    if (isset($field['validation']['min'])) {
                        $fieldRules[] = 'min:' . $field['validation']['min'];
                    }
                    if (isset($field['validation']['regex'])) {
                        $fieldRules[] = 'regex:' . $field['validation']['regex'];
                    }
                    break;

                case 'text':
                    $fieldRules[] = 'string';
                    break;

                case 'number':
                    $fieldRules[] = 'numeric';
                    if (isset($field['validation']['min'])) {
                        $fieldRules[] = 'min:' . $field['validation']['min'];
                    }
                    if (isset($field['validation']['max'])) {
                        $fieldRules[] = 'max:' . $field['validation']['max'];
                    }
                    if ($field['validation']['integer'] ?? false) {
                        $fieldRules[] = 'integer';
                    }
                    break;

                case 'boolean':
                    $fieldRules[] = 'boolean';
                    break;

                case 'date':
                    $fieldRules[] = 'date';
                    break;

                case 'datetime':
                    $fieldRules[] = 'date';
                    break;

                case 'url':
                    $fieldRules[] = 'url';
                    break;

                case 'email':
                    $fieldRules[] = 'email';
                    break;

                case 'slug':
                    $fieldRules[] = 'string';
                    $fieldRules[] = 'regex:/^[a-z0-9-]+$/';
                    break;

                case 'image':
                case 'file':
                    // Reference to media
                    $fieldRules[] = 'array';
                    break;

                case 'array':
                    $fieldRules[] = 'array';
                    if (isset($field['validation']['min'])) {
                        $fieldRules[] = 'min:' . $field['validation']['min'];
                    }
                    if (isset($field['validation']['max'])) {
                        $fieldRules[] = 'max:' . $field['validation']['max'];
                    }
                    break;

                case 'object':
                    $fieldRules[] = 'array';
                    break;

                case 'reference':
                    $fieldRules[] = 'array';
                    break;

                case 'portableText':
                    $fieldRules[] = 'array';
                    break;
            }

            $rules[$name] = implode('|', $fieldRules);

            // Handle nested object fields
            if ($type === 'object' && !empty($field['fields'])) {
                $nestedRules = $this->buildValidationRules($field['fields']);
                foreach ($nestedRules as $nestedName => $nestedRule) {
                    $rules["{$name}.{$nestedName}"] = $nestedRule;
                }
            }

            // Handle array item validation
            if ($type === 'array' && !empty($field['of'])) {
                foreach ($field['of'] as $itemType) {
                    if (is_array($itemType) && !empty($itemType['fields'])) {
                        $nestedRules = $this->buildValidationRules($itemType['fields']);
                        foreach ($nestedRules as $nestedName => $nestedRule) {
                            $rules["{$name}.*.{$nestedName}"] = $nestedRule;
                        }
                    }
                }
            }
        }

        return $rules;
    }

    /**
     * Run custom validation rules
     */
    private function runCustomValidation(array $document, array $validation): array
    {
        $errors = [];

        foreach ($validation as $rule) {
            $type = $rule['type'] ?? 'custom';

            switch ($type) {
                case 'unique':
                    // Check uniqueness
                    $field = $rule['field'];
                    $value = data_get($document, $field);
                    if ($value) {
                        $exists = DB::table($rule['table'] ?? 'posts')
                            ->where($field, $value)
                            ->when(isset($document['id']), fn($q) => $q->where('id', '!=', $document['id']))
                            ->exists();
                        if ($exists) {
                            $errors[$field][] = $rule['message'] ?? 'Value must be unique';
                        }
                    }
                    break;

                case 'conditional':
                    // Conditional required
                    $condition = $rule['condition'];
                    $conditionField = $condition['field'];
                    $conditionValue = $condition['value'] ?? true;
                    $requiredField = $rule['field'];

                    if (data_get($document, $conditionField) === $conditionValue) {
                        if (empty(data_get($document, $requiredField))) {
                            $errors[$requiredField][] = $rule['message'] ?? 'This field is required';
                        }
                    }
                    break;

                case 'custom':
                    // Custom callback (for PHP-defined schemas)
                    if (isset($rule['callback']) && is_callable($rule['callback'])) {
                        $result = call_user_func($rule['callback'], $document);
                        if ($result !== true) {
                            $field = $rule['field'] ?? '_custom';
                            $errors[$field][] = is_string($result) ? $result : 'Validation failed';
                        }
                    }
                    break;
            }
        }

        return $errors;
    }

    /**
     * Validate schema definition
     */
    private function validateSchemaDefinition(array $schema): void
    {
        if (empty($schema['name'])) {
            throw new InvalidArgumentException("Schema must have a name");
        }

        if (!preg_match('/^[a-z][a-zA-Z0-9]*$/', $schema['name'])) {
            throw new InvalidArgumentException("Schema name must be camelCase starting with lowercase letter");
        }

        if (empty($schema['fields']) || !is_array($schema['fields'])) {
            throw new InvalidArgumentException("Schema must have fields array");
        }

        foreach ($schema['fields'] as $index => $field) {
            if (empty($field['name'])) {
                throw new InvalidArgumentException("Field at index {$index} must have a name");
            }

            if (empty($field['type'])) {
                throw new InvalidArgumentException("Field '{$field['name']}' must have a type");
            }

            if (!array_key_exists($field['type'], self::FIELD_TYPES)) {
                throw new InvalidArgumentException("Unknown field type: {$field['type']}");
            }
        }
    }

    /**
     * Format schema from database
     */
    private function formatSchema(object $schema): array
    {
        return [
            'name' => $schema->name,
            'title' => $schema->title,
            'description' => $schema->description,
            'icon' => $schema->icon,
            'fields' => json_decode($schema->fields, true),
            'fieldsets' => $schema->fieldsets ? json_decode($schema->fieldsets, true) : null,
            'preview' => $schema->preview ? json_decode($schema->preview, true) : null,
            'orderings' => $schema->orderings ? json_decode($schema->orderings, true) : null,
            'validation' => $schema->validation ? json_decode($schema->validation, true) : null,
            'initialValue' => $schema->initial_value ? json_decode($schema->initial_value, true) : null,
            'type' => $schema->is_document ? 'document' : 'object',
            'singleton' => (bool) $schema->is_singleton,
            'system' => (bool) $schema->is_system,
            'version' => $schema->version,
        ];
    }

    /**
     * Generate TypeScript interface from schema
     */
    public function generateTypeScript(string $schemaName): string
    {
        $schema = $this->get($schemaName);

        if (!$schema) {
            throw new InvalidArgumentException("Schema not found: {$schemaName}");
        }

        $interfaceName = Str::studly($schema['name']);
        $lines = ["export interface {$interfaceName} {"];

        foreach ($schema['fields'] as $field) {
            $tsType = $this->fieldTypeToTypeScript($field);
            $optional = ($field['validation']['required'] ?? false) ? '' : '?';
            $comment = !empty($field['description']) ? "  /** {$field['description']} */\n" : '';
            $lines[] = $comment . "  {$field['name']}{$optional}: {$tsType};";
        }

        $lines[] = '}';
        $lines[] = '';

        return implode("\n", $lines);
    }

    /**
     * Convert field type to TypeScript
     */
    private function fieldTypeToTypeScript(array $field): string
    {
        return match ($field['type']) {
            'string', 'text', 'slug', 'url', 'email', 'color' => 'string',
            'number' => 'number',
            'boolean' => 'boolean',
            'date', 'datetime' => 'string',
            'image', 'file' => 'SanityImageAsset',
            'reference' => 'SanityReference',
            'array' => $this->arrayTypeToTypeScript($field),
            'object' => $this->objectTypeToTypeScript($field),
            'portableText' => 'PortableTextBlock[]',
            'geopoint' => '{ lat: number; lng: number }',
            default => 'unknown',
        };
    }

    /**
     * Convert array field to TypeScript
     */
    private function arrayTypeToTypeScript(array $field): string
    {
        if (empty($field['of'])) {
            return 'unknown[]';
        }

        $types = array_map(function ($itemType) {
            if (is_string($itemType)) {
                return match ($itemType) {
                    'string' => 'string',
                    'number' => 'number',
                    'reference' => 'SanityReference',
                    default => 'unknown',
                };
            }
            return $this->fieldTypeToTypeScript($itemType);
        }, $field['of']);

        $union = implode(' | ', array_unique($types));
        return "({$union})[]";
    }

    /**
     * Convert object field to TypeScript
     */
    private function objectTypeToTypeScript(array $field): string
    {
        if (empty($field['fields'])) {
            return 'Record<string, unknown>';
        }

        $props = [];
        foreach ($field['fields'] as $subField) {
            $tsType = $this->fieldTypeToTypeScript($subField);
            $optional = ($subField['validation']['required'] ?? false) ? '' : '?';
            $props[] = "{$subField['name']}{$optional}: {$tsType}";
        }

        return '{ ' . implode('; ', $props) . ' }';
    }

    /**
     * Register default schemas
     */
    public function registerDefaults(): void
    {
        // Post schema
        $this->register([
            'name' => 'post',
            'title' => 'Post',
            'icon' => 'document',
            'fields' => [
                ['name' => 'title', 'type' => 'string', 'title' => 'Title', 'validation' => ['required' => true]],
                ['name' => 'slug', 'type' => 'slug', 'title' => 'Slug', 'options' => ['source' => 'title']],
                ['name' => 'excerpt', 'type' => 'text', 'title' => 'Excerpt', 'validation' => ['max' => 500]],
                ['name' => 'content', 'type' => 'portableText', 'title' => 'Content'],
                ['name' => 'featuredImage', 'type' => 'image', 'title' => 'Featured Image'],
                ['name' => 'author', 'type' => 'reference', 'title' => 'Author', 'to' => ['user']],
                ['name' => 'categories', 'type' => 'array', 'title' => 'Categories', 'of' => [['type' => 'reference', 'to' => ['category']]]],
                ['name' => 'tags', 'type' => 'array', 'title' => 'Tags', 'of' => ['string']],
                ['name' => 'publishedAt', 'type' => 'datetime', 'title' => 'Published At'],
            ],
            'preview' => [
                'select' => ['title' => 'title', 'author' => 'author.name', 'media' => 'featuredImage'],
            ],
            'orderings' => [
                ['title' => 'Published Date, New', 'by' => [['field' => 'publishedAt', 'direction' => 'desc']]],
                ['title' => 'Title', 'by' => [['field' => 'title', 'direction' => 'asc']]],
            ],
        ]);

        // Category schema
        $this->register([
            'name' => 'category',
            'title' => 'Category',
            'icon' => 'tag',
            'fields' => [
                ['name' => 'name', 'type' => 'string', 'title' => 'Name', 'validation' => ['required' => true]],
                ['name' => 'slug', 'type' => 'slug', 'title' => 'Slug', 'options' => ['source' => 'name']],
                ['name' => 'description', 'type' => 'text', 'title' => 'Description'],
                ['name' => 'color', 'type' => 'color', 'title' => 'Color'],
                ['name' => 'parent', 'type' => 'reference', 'title' => 'Parent Category', 'to' => ['category']],
            ],
        ]);
    }
}
