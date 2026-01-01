<?php

declare(strict_types=1);

namespace Tests\Unit\ContentLake;

use App\Http\Requests\Api\ContentLake\GroqQueryRequest;
use App\Http\Requests\Api\ContentLake\WebhookRequest;
use App\Http\Requests\Api\ContentLake\SchemaRequest;
use App\Http\Requests\Api\ContentLake\ReleaseRequest;
use App\Http\Requests\Api\ContentLake\ImageCropRequest;
use Illuminate\Support\Facades\Validator;
use Tests\TestCase;

/**
 * FormRequest Validation Tests
 *
 * Tests enterprise-grade validation rules for ContentLake API requests.
 *
 * @level ICT11 Principal Engineer
 */
class FormRequestTest extends TestCase
{
    // ═══════════════════════════════════════════════════════════════════════════
    // GROQ QUERY REQUEST TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    public function test_groq_query_requires_query(): void
    {
        $request = new GroqQueryRequest();
        $rules = $request->rules();

        $validator = Validator::make([], $rules);

        $this->assertTrue($validator->fails());
        $this->assertArrayHasKey('query', $validator->errors()->toArray());
    }

    public function test_groq_query_accepts_valid_query(): void
    {
        $request = new GroqQueryRequest();
        $rules = $this->getBasicRules($request->rules());

        $validator = Validator::make([
            'query' => '*[_type == "post"]',
            'params' => ['limit' => 10],
            'useCache' => true,
        ], $rules);

        $this->assertFalse($validator->fails());
    }

    public function test_groq_query_rejects_oversized_query(): void
    {
        $request = new GroqQueryRequest();
        $rules = $this->getBasicRules($request->rules());

        $validator = Validator::make([
            'query' => str_repeat('a', 10001),
        ], $rules);

        $this->assertTrue($validator->fails());
        $this->assertArrayHasKey('query', $validator->errors()->toArray());
    }

    public function test_groq_query_validates_perspective(): void
    {
        $request = new GroqQueryRequest();
        $rules = $this->getBasicRules($request->rules());

        // Valid perspective
        $validator = Validator::make([
            'query' => '*[_type == "post"]',
            'perspective' => 'draft',
        ], $rules);
        $this->assertFalse($validator->fails());

        // Invalid perspective
        $validator = Validator::make([
            'query' => '*[_type == "post"]',
            'perspective' => 'invalid',
        ], $rules);
        $this->assertTrue($validator->fails());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // WEBHOOK REQUEST TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    public function test_webhook_requires_name_and_url(): void
    {
        $request = new WebhookRequest();
        $rules = $this->getBasicRules($request->rules());

        $validator = Validator::make([], $rules);

        $this->assertTrue($validator->fails());
        $errors = $validator->errors()->toArray();
        $this->assertArrayHasKey('name', $errors);
        $this->assertArrayHasKey('url', $errors);
    }

    public function test_webhook_accepts_valid_configuration(): void
    {
        $request = new WebhookRequest();
        $rules = $this->getBasicRules($request->rules());

        $validator = Validator::make([
            'name' => 'My Webhook',
            'url' => 'https://example.com/webhook',
            'events' => ['document.created', 'document.updated'],
            'is_active' => true,
        ], $rules);

        $this->assertFalse($validator->fails());
    }

    public function test_webhook_rejects_invalid_event_types(): void
    {
        $request = new WebhookRequest();
        $rules = $this->getBasicRules($request->rules());

        $validator = Validator::make([
            'name' => 'My Webhook',
            'url' => 'https://example.com/webhook',
            'events' => ['invalid.event'],
        ], $rules);

        $this->assertTrue($validator->fails());
    }

    public function test_webhook_validates_url_format(): void
    {
        $request = new WebhookRequest();
        $rules = $this->getBasicRules($request->rules());

        // Valid URL passes
        $validator = Validator::make([
            'name' => 'My Webhook',
            'url' => 'https://example.com/webhook',
            'events' => ['document.created'],
        ], $rules);

        $this->assertFalse($validator->fails());

        // URL too long should fail
        $validator = Validator::make([
            'name' => 'My Webhook',
            'url' => 'https://example.com/' . str_repeat('a', 2100),
            'events' => ['document.created'],
        ], $rules);

        $this->assertTrue($validator->fails());
        $this->assertArrayHasKey('url', $validator->errors()->toArray());
    }

    public function test_webhook_secret_minimum_length(): void
    {
        $request = new WebhookRequest();
        $rules = $this->getBasicRules($request->rules());

        // Too short secret
        $validator = Validator::make([
            'name' => 'My Webhook',
            'url' => 'https://example.com/webhook',
            'events' => ['document.created'],
            'secret' => 'short',
        ], $rules);

        $this->assertTrue($validator->fails());
        $this->assertArrayHasKey('secret', $validator->errors()->toArray());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SCHEMA REQUEST TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    public function test_schema_requires_name_and_fields(): void
    {
        $request = new SchemaRequest();
        $rules = $this->getBasicRules($request->rules());

        $validator = Validator::make([], $rules);

        $this->assertTrue($validator->fails());
        $errors = $validator->errors()->toArray();
        $this->assertArrayHasKey('name', $errors);
        $this->assertArrayHasKey('fields', $errors);
    }

    public function test_schema_accepts_valid_definition(): void
    {
        $request = new SchemaRequest();
        $rules = $this->getBasicRules($request->rules());

        $validator = Validator::make([
            'name' => 'blogPost',
            'title' => 'Blog Post',
            'fields' => [
                ['name' => 'title', 'type' => 'string'],
                ['name' => 'content', 'type' => 'text'],
            ],
        ], $rules);

        $this->assertFalse($validator->fails());
    }

    public function test_schema_name_must_start_with_letter(): void
    {
        $request = new SchemaRequest();
        $rules = $this->getBasicRules($request->rules());

        $validator = Validator::make([
            'name' => '123invalid',
            'fields' => [['name' => 'title', 'type' => 'string']],
        ], $rules);

        $this->assertTrue($validator->fails());
        $this->assertArrayHasKey('name', $validator->errors()->toArray());
    }

    public function test_schema_field_types_are_validated(): void
    {
        $request = new SchemaRequest();
        $rules = $this->getBasicRules($request->rules());

        // Valid field types
        $validTypes = ['string', 'text', 'number', 'boolean', 'datetime', 'image', 'reference'];
        foreach ($validTypes as $type) {
            $validator = Validator::make([
                'name' => 'testSchema',
                'fields' => [['name' => 'field', 'type' => $type]],
            ], $rules);
            $this->assertFalse($validator->fails(), "Type '{$type}' should be valid");
        }

        // Invalid field type
        $validator = Validator::make([
            'name' => 'testSchema',
            'fields' => [['name' => 'field', 'type' => 'invalid_type']],
        ], $rules);
        $this->assertTrue($validator->fails());
    }

    public function test_schema_limits_maximum_fields(): void
    {
        $request = new SchemaRequest();
        $rules = $this->getBasicRules($request->rules());

        $fields = [];
        for ($i = 0; $i < 101; $i++) {
            $fields[] = ['name' => "field{$i}", 'type' => 'string'];
        }

        $validator = Validator::make([
            'name' => 'testSchema',
            'fields' => $fields,
        ], $rules);

        $this->assertTrue($validator->fails());
        $this->assertArrayHasKey('fields', $validator->errors()->toArray());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // RELEASE REQUEST TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    public function test_release_requires_name(): void
    {
        $request = new ReleaseRequest();
        $rules = $this->getBasicRules($request->rules());

        $validator = Validator::make([], $rules);

        $this->assertTrue($validator->fails());
        $this->assertArrayHasKey('name', $validator->errors()->toArray());
    }

    public function test_release_accepts_valid_configuration(): void
    {
        $request = new ReleaseRequest();
        $rules = $this->getBasicRules($request->rules());

        $validator = Validator::make([
            'name' => 'Spring Release 2025',
            'description' => 'Major feature release',
            'metadata' => [
                'priority' => 'high',
                'tags' => ['feature', 'breaking'],
            ],
        ], $rules);

        $this->assertFalse($validator->fails());
    }

    public function test_release_name_minimum_length(): void
    {
        $request = new ReleaseRequest();
        $rules = $this->getBasicRules($request->rules());

        $validator = Validator::make([
            'name' => 'ab', // Too short
        ], $rules);

        $this->assertTrue($validator->fails());
        $this->assertArrayHasKey('name', $validator->errors()->toArray());
    }

    public function test_release_priority_values(): void
    {
        $request = new ReleaseRequest();
        $rules = $this->getBasicRules($request->rules());

        // Valid priorities
        foreach (['low', 'medium', 'high', 'critical'] as $priority) {
            $validator = Validator::make([
                'name' => 'Test Release',
                'metadata' => ['priority' => $priority],
            ], $rules);
            $this->assertFalse($validator->fails(), "Priority '{$priority}' should be valid");
        }

        // Invalid priority
        $validator = Validator::make([
            'name' => 'Test Release',
            'metadata' => ['priority' => 'urgent'],
        ], $rules);
        $this->assertTrue($validator->fails());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // IMAGE CROP REQUEST TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    public function test_crop_requires_all_edges(): void
    {
        $request = new ImageCropRequest();
        $rules = $this->getBasicRules($request->rules());

        $validator = Validator::make([
            'name' => 'thumbnail',
            'top' => 0.1,
            // Missing: left, bottom, right
        ], $rules);

        $this->assertTrue($validator->fails());
    }

    public function test_crop_accepts_valid_configuration(): void
    {
        $request = new ImageCropRequest();
        $rules = $this->getBasicRules($request->rules());

        $validator = Validator::make([
            'name' => 'thumbnail',
            'top' => 0.1,
            'left' => 0.1,
            'bottom' => 0.9,
            'right' => 0.9,
            'aspectRatio' => '16:9',
        ], $rules);

        $this->assertFalse($validator->fails());
    }

    public function test_crop_edge_values_in_range(): void
    {
        $request = new ImageCropRequest();
        $rules = $this->getBasicRules($request->rules());

        // Value out of range (> 1)
        $validator = Validator::make([
            'name' => 'thumbnail',
            'top' => 1.5,
            'left' => 0.1,
            'bottom' => 0.9,
            'right' => 0.9,
        ], $rules);

        $this->assertTrue($validator->fails());
        $this->assertArrayHasKey('top', $validator->errors()->toArray());
    }

    public function test_crop_name_format(): void
    {
        $request = new ImageCropRequest();
        $rules = $this->getBasicRules($request->rules());

        // Invalid name (starts with number)
        $validator = Validator::make([
            'name' => '123invalid',
            'top' => 0.1,
            'left' => 0.1,
            'bottom' => 0.9,
            'right' => 0.9,
        ], $rules);

        $this->assertTrue($validator->fails());
        $this->assertArrayHasKey('name', $validator->errors()->toArray());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // HELPER METHODS
    // ═══════════════════════════════════════════════════════════════════════════

    /**
     * Get basic rules without closures (for unit testing)
     */
    private function getBasicRules(array $rules): array
    {
        $basicRules = [];
        foreach ($rules as $field => $fieldRules) {
            if (is_array($fieldRules)) {
                $basicRules[$field] = array_filter($fieldRules, fn($rule) => !is_callable($rule));
            } else {
                $basicRules[$field] = $fieldRules;
            }
        }
        return $basicRules;
    }
}
