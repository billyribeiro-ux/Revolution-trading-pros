<?php

declare(strict_types=1);

namespace Tests\Unit\ContentLake;

use App\Services\ContentLake\SchemaService;
use InvalidArgumentException;
use PHPUnit\Framework\TestCase;
use ReflectionClass;

/**
 * Unit tests for SchemaService - validation logic only (no DB)
 */
class SchemaServiceTest extends TestCase
{
    private SchemaService $service;

    protected function setUp(): void
    {
        parent::setUp();
        $this->service = new SchemaService();
    }

    // ═══════════════════════════════════════════════════════════════════════
    // SCHEMA DEFINITION VALIDATION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    public function test_schema_must_have_name(): void
    {
        $this->expectException(InvalidArgumentException::class);
        $this->expectExceptionMessage('Schema must have a name');

        $this->invokeValidateSchemaDefinition([
            'fields' => [['name' => 'title', 'type' => 'string']],
        ]);
    }

    public function test_schema_name_must_be_camel_case(): void
    {
        $this->expectException(InvalidArgumentException::class);
        $this->expectExceptionMessage('Schema name must be camelCase');

        $this->invokeValidateSchemaDefinition([
            'name' => 'Invalid-Name',
            'fields' => [['name' => 'title', 'type' => 'string']],
        ]);
    }

    public function test_schema_name_must_start_with_lowercase(): void
    {
        $this->expectException(InvalidArgumentException::class);
        $this->expectExceptionMessage('Schema name must be camelCase');

        $this->invokeValidateSchemaDefinition([
            'name' => 'InvalidName',
            'fields' => [['name' => 'title', 'type' => 'string']],
        ]);
    }

    public function test_schema_must_have_fields(): void
    {
        $this->expectException(InvalidArgumentException::class);
        $this->expectExceptionMessage('Schema must have fields array');

        $this->invokeValidateSchemaDefinition([
            'name' => 'validName',
        ]);
    }

    public function test_field_must_have_name(): void
    {
        $this->expectException(InvalidArgumentException::class);
        $this->expectExceptionMessage('Field at index 0 must have a name');

        $this->invokeValidateSchemaDefinition([
            'name' => 'validName',
            'fields' => [['type' => 'string']],
        ]);
    }

    public function test_field_must_have_type(): void
    {
        $this->expectException(InvalidArgumentException::class);
        $this->expectExceptionMessage("Field 'title' must have a type");

        $this->invokeValidateSchemaDefinition([
            'name' => 'validName',
            'fields' => [['name' => 'title']],
        ]);
    }

    public function test_field_type_must_be_valid(): void
    {
        $this->expectException(InvalidArgumentException::class);
        $this->expectExceptionMessage('Unknown field type: invalidType');

        $this->invokeValidateSchemaDefinition([
            'name' => 'validName',
            'fields' => [['name' => 'title', 'type' => 'invalidType']],
        ]);
    }

    public function test_valid_schema_passes_validation(): void
    {
        // This should not throw
        $result = $this->invokeValidateSchemaDefinition([
            'name' => 'blogPost',
            'title' => 'Blog Post',
            'fields' => [
                ['name' => 'title', 'type' => 'string'],
                ['name' => 'content', 'type' => 'portableText'],
                ['name' => 'published', 'type' => 'boolean'],
            ],
        ]);

        $this->assertNull($result); // void method returns null
    }

    // ═══════════════════════════════════════════════════════════════════════
    // FIELD TYPE VALIDATION
    // ═══════════════════════════════════════════════════════════════════════

    public function test_all_field_types_are_valid(): void
    {
        $validTypes = [
            'string', 'text', 'number', 'boolean', 'date', 'datetime',
            'url', 'email', 'slug', 'image', 'file', 'array', 'object',
            'reference', 'portableText', 'geopoint', 'color',
        ];

        foreach ($validTypes as $type) {
            $result = $this->invokeValidateSchemaDefinition([
                'name' => 'testSchema',
                'fields' => [['name' => 'testField', 'type' => $type]],
            ]);

            $this->assertNull($result, "Type '{$type}' should be valid");
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // VALIDATION RULES BUILDING TESTS
    // ═══════════════════════════════════════════════════════════════════════

    public function test_builds_required_rule(): void
    {
        $fields = [
            ['name' => 'title', 'type' => 'string', 'validation' => ['required' => true]],
        ];

        $rules = $this->invokeBuildValidationRules($fields);

        $this->assertStringContainsString('required', $rules['title']);
    }

    public function test_builds_nullable_when_not_required(): void
    {
        $fields = [
            ['name' => 'title', 'type' => 'string'],
        ];

        $rules = $this->invokeBuildValidationRules($fields);

        $this->assertStringContainsString('nullable', $rules['title']);
    }

    public function test_builds_string_rules(): void
    {
        $fields = [
            ['name' => 'name', 'type' => 'string', 'validation' => ['min' => 3, 'max' => 100]],
        ];

        $rules = $this->invokeBuildValidationRules($fields);

        $this->assertStringContainsString('string', $rules['name']);
        $this->assertStringContainsString('min:3', $rules['name']);
        $this->assertStringContainsString('max:100', $rules['name']);
    }

    public function test_builds_number_rules(): void
    {
        $fields = [
            ['name' => 'age', 'type' => 'number', 'validation' => ['min' => 0, 'max' => 150, 'integer' => true]],
        ];

        $rules = $this->invokeBuildValidationRules($fields);

        $this->assertStringContainsString('numeric', $rules['age']);
        $this->assertStringContainsString('min:0', $rules['age']);
        $this->assertStringContainsString('max:150', $rules['age']);
        $this->assertStringContainsString('integer', $rules['age']);
    }

    public function test_builds_boolean_rules(): void
    {
        $fields = [
            ['name' => 'active', 'type' => 'boolean'],
        ];

        $rules = $this->invokeBuildValidationRules($fields);

        $this->assertStringContainsString('boolean', $rules['active']);
    }

    public function test_builds_date_rules(): void
    {
        $fields = [
            ['name' => 'publishDate', 'type' => 'date'],
        ];

        $rules = $this->invokeBuildValidationRules($fields);

        $this->assertStringContainsString('date', $rules['publishDate']);
    }

    public function test_builds_url_rules(): void
    {
        $fields = [
            ['name' => 'website', 'type' => 'url'],
        ];

        $rules = $this->invokeBuildValidationRules($fields);

        $this->assertStringContainsString('url', $rules['website']);
    }

    public function test_builds_email_rules(): void
    {
        $fields = [
            ['name' => 'email', 'type' => 'email'],
        ];

        $rules = $this->invokeBuildValidationRules($fields);

        $this->assertStringContainsString('email', $rules['email']);
    }

    public function test_builds_slug_rules(): void
    {
        $fields = [
            ['name' => 'slug', 'type' => 'slug'],
        ];

        $rules = $this->invokeBuildValidationRules($fields);

        $this->assertStringContainsString('string', $rules['slug']);
        $this->assertStringContainsString('regex:/^[a-z0-9-]+$/', $rules['slug']);
    }

    public function test_builds_array_rules(): void
    {
        $fields = [
            ['name' => 'tags', 'type' => 'array', 'validation' => ['min' => 1, 'max' => 10]],
        ];

        $rules = $this->invokeBuildValidationRules($fields);

        $this->assertStringContainsString('array', $rules['tags']);
        $this->assertStringContainsString('min:1', $rules['tags']);
        $this->assertStringContainsString('max:10', $rules['tags']);
    }

    public function test_builds_nested_object_rules(): void
    {
        $fields = [
            [
                'name' => 'address',
                'type' => 'object',
                'fields' => [
                    ['name' => 'street', 'type' => 'string', 'validation' => ['required' => true]],
                    ['name' => 'city', 'type' => 'string'],
                ],
            ],
        ];

        $rules = $this->invokeBuildValidationRules($fields);

        $this->assertArrayHasKey('address', $rules);
        $this->assertArrayHasKey('address.street', $rules);
        $this->assertArrayHasKey('address.city', $rules);
        $this->assertStringContainsString('required', $rules['address.street']);
    }

    public function test_builds_regex_validation(): void
    {
        $fields = [
            ['name' => 'code', 'type' => 'string', 'validation' => ['regex' => '/^[A-Z]{3}$/']],
        ];

        $rules = $this->invokeBuildValidationRules($fields);

        $this->assertStringContainsString('regex:/^[A-Z]{3}$/', $rules['code']);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // TYPESCRIPT GENERATION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    public function test_string_type_to_typescript(): void
    {
        $result = $this->invokeFieldTypeToTypeScript(['type' => 'string']);
        $this->assertEquals('string', $result);
    }

    public function test_text_type_to_typescript(): void
    {
        $result = $this->invokeFieldTypeToTypeScript(['type' => 'text']);
        $this->assertEquals('string', $result);
    }

    public function test_number_type_to_typescript(): void
    {
        $result = $this->invokeFieldTypeToTypeScript(['type' => 'number']);
        $this->assertEquals('number', $result);
    }

    public function test_boolean_type_to_typescript(): void
    {
        $result = $this->invokeFieldTypeToTypeScript(['type' => 'boolean']);
        $this->assertEquals('boolean', $result);
    }

    public function test_date_type_to_typescript(): void
    {
        $result = $this->invokeFieldTypeToTypeScript(['type' => 'date']);
        $this->assertEquals('string', $result);
    }

    public function test_datetime_type_to_typescript(): void
    {
        $result = $this->invokeFieldTypeToTypeScript(['type' => 'datetime']);
        $this->assertEquals('string', $result);
    }

    public function test_url_type_to_typescript(): void
    {
        $result = $this->invokeFieldTypeToTypeScript(['type' => 'url']);
        $this->assertEquals('string', $result);
    }

    public function test_email_type_to_typescript(): void
    {
        $result = $this->invokeFieldTypeToTypeScript(['type' => 'email']);
        $this->assertEquals('string', $result);
    }

    public function test_slug_type_to_typescript(): void
    {
        $result = $this->invokeFieldTypeToTypeScript(['type' => 'slug']);
        $this->assertEquals('string', $result);
    }

    public function test_color_type_to_typescript(): void
    {
        $result = $this->invokeFieldTypeToTypeScript(['type' => 'color']);
        $this->assertEquals('string', $result);
    }

    public function test_image_type_to_typescript(): void
    {
        $result = $this->invokeFieldTypeToTypeScript(['type' => 'image']);
        $this->assertEquals('SanityImageAsset', $result);
    }

    public function test_file_type_to_typescript(): void
    {
        $result = $this->invokeFieldTypeToTypeScript(['type' => 'file']);
        $this->assertEquals('SanityImageAsset', $result);
    }

    public function test_reference_type_to_typescript(): void
    {
        $result = $this->invokeFieldTypeToTypeScript(['type' => 'reference']);
        $this->assertEquals('SanityReference', $result);
    }

    public function test_portable_text_type_to_typescript(): void
    {
        $result = $this->invokeFieldTypeToTypeScript(['type' => 'portableText']);
        $this->assertEquals('PortableTextBlock[]', $result);
    }

    public function test_geopoint_type_to_typescript(): void
    {
        $result = $this->invokeFieldTypeToTypeScript(['type' => 'geopoint']);
        $this->assertEquals('{ lat: number; lng: number }', $result);
    }

    public function test_array_type_to_typescript(): void
    {
        $result = $this->invokeFieldTypeToTypeScript([
            'type' => 'array',
            'of' => ['string'],
        ]);
        $this->assertEquals('(string)[]', $result);
    }

    public function test_array_of_references_to_typescript(): void
    {
        $result = $this->invokeFieldTypeToTypeScript([
            'type' => 'array',
            'of' => ['reference'],
        ]);
        $this->assertEquals('(SanityReference)[]', $result);
    }

    public function test_object_type_to_typescript(): void
    {
        $result = $this->invokeFieldTypeToTypeScript([
            'type' => 'object',
            'fields' => [
                ['name' => 'lat', 'type' => 'number', 'validation' => ['required' => true]],
                ['name' => 'lng', 'type' => 'number'],
            ],
        ]);
        $this->assertStringContainsString('lat: number', $result);
        $this->assertStringContainsString('lng?: number', $result);
    }

    public function test_empty_object_to_typescript(): void
    {
        $result = $this->invokeFieldTypeToTypeScript(['type' => 'object']);
        $this->assertEquals('Record<string, unknown>', $result);
    }

    public function test_empty_array_to_typescript(): void
    {
        $result = $this->invokeFieldTypeToTypeScript(['type' => 'array']);
        $this->assertEquals('unknown[]', $result);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // HELPER METHODS (use reflection to test private methods)
    // ═══════════════════════════════════════════════════════════════════════

    private function invokeValidateSchemaDefinition(array $schema): mixed
    {
        $reflection = new ReflectionClass($this->service);
        $method = $reflection->getMethod('validateSchemaDefinition');
        $method->setAccessible(true);
        return $method->invoke($this->service, $schema);
    }

    private function invokeBuildValidationRules(array $fields): array
    {
        $reflection = new ReflectionClass($this->service);
        $method = $reflection->getMethod('buildValidationRules');
        $method->setAccessible(true);
        return $method->invoke($this->service, $fields);
    }

    private function invokeFieldTypeToTypeScript(array $field): string
    {
        $reflection = new ReflectionClass($this->service);
        $method = $reflection->getMethod('fieldTypeToTypeScript');
        $method->setAccessible(true);
        return $method->invoke($this->service, $field);
    }
}
