<?php

declare(strict_types=1);

namespace Tests\Feature\ContentLake;

use App\Models\Post;
use App\Models\User;
use App\Services\ContentLake\DocumentHistoryService;
use App\Services\ContentLake\DocumentPerspectiveService;
use App\Services\ContentLake\GroqQueryService;
use App\Services\ContentLake\ImageHotspotService;
use App\Services\ContentLake\PortableTextService;
use App\Services\ContentLake\ReleaseBundleService;
use App\Services\ContentLake\SchemaService;
use App\Services\ContentLake\WebhookService;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;
use Tests\TestCase;

/**
 * Comprehensive tests for Sanity-equivalent Content Lake features.
 */
class ContentLakeTest extends TestCase
{
    use RefreshDatabase;

    private User $adminUser;
    private PortableTextService $portableTextService;
    private GroqQueryService $groqService;
    private DocumentPerspectiveService $perspectiveService;
    private DocumentHistoryService $historyService;
    private WebhookService $webhookService;
    private SchemaService $schemaService;

    protected function setUp(): void
    {
        parent::setUp();

        // Create admin user
        $this->adminUser = User::factory()->create([
            'email' => 'admin@test.com',
        ]);
        $this->adminUser->assignRole('admin');

        // Initialize services
        $this->portableTextService = app(PortableTextService::class);
        $this->groqService = app(GroqQueryService::class);
        $this->perspectiveService = app(DocumentPerspectiveService::class);
        $this->historyService = app(DocumentHistoryService::class);
        $this->webhookService = app(WebhookService::class);
        $this->schemaService = app(SchemaService::class);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PORTABLE TEXT TESTS
    // ═══════════════════════════════════════════════════════════════════════

    /** @test */
    public function it_can_parse_and_render_portable_text(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                '_key' => 'key1',
                'style' => 'h1',
                'children' => [
                    ['_type' => 'span', '_key' => 's1', 'text' => 'Hello World', 'marks' => []],
                ],
                'markDefs' => [],
            ],
            [
                '_type' => 'block',
                '_key' => 'key2',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', '_key' => 's2', 'text' => 'This is ', 'marks' => []],
                    ['_type' => 'span', '_key' => 's3', 'text' => 'bold', 'marks' => ['strong']],
                    ['_type' => 'span', '_key' => 's4', 'text' => ' text.', 'marks' => []],
                ],
                'markDefs' => [],
            ],
        ];

        $parsed = $this->portableTextService->parse($blocks);
        $html = $this->portableTextService->toHtml($parsed);

        $this->assertStringContainsString('<h1>Hello World</h1>', $html);
        $this->assertStringContainsString('<strong>bold</strong>', $html);
        $this->assertStringContainsString('<p>This is', $html);
    }

    /** @test */
    public function it_can_render_links_in_portable_text(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                '_key' => 'key1',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', '_key' => 's1', 'text' => 'Click ', 'marks' => []],
                    ['_type' => 'span', '_key' => 's2', 'text' => 'here', 'marks' => ['link1']],
                ],
                'markDefs' => [
                    ['_type' => 'link', '_key' => 'link1', 'href' => 'https://example.com', 'blank' => true],
                ],
            ],
        ];

        $html = $this->portableTextService->toHtml($blocks);

        $this->assertStringContainsString('href="https://example.com"', $html);
        $this->assertStringContainsString('target="_blank"', $html);
        $this->assertStringContainsString('rel="noopener noreferrer"', $html);
    }

    /** @test */
    public function it_can_render_lists_in_portable_text(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                '_key' => 'key1',
                'style' => 'normal',
                'listItem' => 'bullet',
                'level' => 1,
                'children' => [['_type' => 'span', 'text' => 'Item 1', 'marks' => []]],
                'markDefs' => [],
            ],
            [
                '_type' => 'block',
                '_key' => 'key2',
                'style' => 'normal',
                'listItem' => 'bullet',
                'level' => 1,
                'children' => [['_type' => 'span', 'text' => 'Item 2', 'marks' => []]],
                'markDefs' => [],
            ],
        ];

        $html = $this->portableTextService->toHtml($blocks);

        $this->assertStringContainsString('<ul>', $html);
        $this->assertStringContainsString('<li>Item 1</li>', $html);
        $this->assertStringContainsString('<li>Item 2</li>', $html);
        $this->assertStringContainsString('</ul>', $html);
    }

    /** @test */
    public function it_can_render_code_blocks(): void
    {
        $blocks = [
            [
                '_type' => 'code',
                '_key' => 'key1',
                'code' => 'const x = 1;',
                'language' => 'javascript',
                'filename' => 'example.js',
            ],
        ];

        $html = $this->portableTextService->toHtml($blocks);

        $this->assertStringContainsString('class="language-javascript"', $html);
        $this->assertStringContainsString('const x = 1;', $html);
        $this->assertStringContainsString('example.js', $html);
    }

    /** @test */
    public function it_calculates_word_count_and_reading_time(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                '_key' => 'key1',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', 'text' => 'This is a test paragraph with ten words in it.', 'marks' => []],
                ],
                'markDefs' => [],
            ],
        ];

        $wordCount = $this->portableTextService->wordCount($blocks);
        $readingTime = $this->portableTextService->readingTime($blocks);

        $this->assertEquals(10, $wordCount);
        $this->assertEquals(1, $readingTime); // Less than 200 words = 1 minute
    }

    /** @test */
    public function it_validates_portable_text_structure(): void
    {
        $validBlocks = [
            [
                '_type' => 'block',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', 'text' => 'Hello', 'marks' => []],
                ],
                'markDefs' => [],
            ],
        ];

        $result = $this->portableTextService->validate($validBlocks);

        $this->assertTrue($result->isValid);
        $this->assertEmpty($result->errors);
    }

    /** @test */
    public function it_can_build_documents_with_fluent_api(): void
    {
        $document = $this->portableTextService->createDocument()
            ->heading(1, 'My Title')
            ->paragraph('This is a paragraph.')
            ->bulletList(['Item 1', 'Item 2', 'Item 3'])
            ->code('console.log("hello")', 'javascript')
            ->blockquote('A famous quote');

        $blocks = $document->toArray();
        $html = $document->toHtml();

        $this->assertCount(7, $blocks); // 1 heading + 1 para + 3 list items + 1 code + 1 quote
        $this->assertStringContainsString('<h1>My Title</h1>', $html);
        $this->assertStringContainsString('<ul>', $html);
        $this->assertStringContainsString('<blockquote>', $html);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // GROQ QUERY TESTS
    // ═══════════════════════════════════════════════════════════════════════

    /** @test */
    public function it_can_execute_basic_groq_query(): void
    {
        // Create test posts
        Post::factory()->count(5)->create(['status' => 'published']);

        $result = $this->groqService->query(
            '*[_type == "post"] | order(created_at desc) [0...3]',
            [],
            false
        );

        $this->assertIsArray($result->data);
        $this->assertLessThanOrEqual(3, count($result->data));
        $this->assertFalse($result->fromCache);
    }

    /** @test */
    public function it_caches_groq_queries(): void
    {
        Post::factory()->count(3)->create(['status' => 'published']);

        // First query - not cached
        $result1 = $this->groqService->query('*[_type == "post"]');
        $this->assertFalse($result1->fromCache);

        // Second query - should be cached
        $result2 = $this->groqService->query('*[_type == "post"]');
        $this->assertTrue($result2->fromCache);
    }

    /** @test */
    public function it_can_filter_groq_queries(): void
    {
        Post::factory()->create(['status' => 'published', 'title' => 'Test Post']);
        Post::factory()->create(['status' => 'draft', 'title' => 'Draft Post']);

        $result = $this->groqService->query(
            '*[_type == "post" && status == "published"]',
            [],
            false
        );

        $this->assertCount(1, $result->data);
        $this->assertEquals('Test Post', $result->data[0]['title']);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // DOCUMENT PERSPECTIVE TESTS
    // ═══════════════════════════════════════════════════════════════════════

    /** @test */
    public function it_can_create_document_perspective(): void
    {
        $post = Post::factory()->create([
            'status' => 'draft',
            'title' => 'Test Post',
        ]);

        $perspective = $this->perspectiveService->getOrCreatePerspective($post);

        $this->assertNotNull($perspective['document_id']);
        $this->assertEquals('post', $perspective['document_type']);
        $this->assertTrue((bool) $perspective['has_unpublished_changes']);
    }

    /** @test */
    public function it_can_update_draft_content(): void
    {
        $post = Post::factory()->create(['status' => 'draft']);
        $perspective = $this->perspectiveService->getOrCreatePerspective($post);
        $documentId = $perspective['document_id'];

        $newContent = [
            'title' => 'Updated Title',
            'content' => 'Updated content',
        ];

        $this->actingAs($this->adminUser);
        $updated = $this->perspectiveService->updateDraft($documentId, $newContent);

        $this->assertEquals('Updated Title', $updated['title']);
        $this->assertTrue($updated['_hasUnpublishedChanges']);
    }

    /** @test */
    public function it_can_publish_and_unpublish_documents(): void
    {
        $post = Post::factory()->create(['status' => 'draft', 'title' => 'My Post']);
        $perspective = $this->perspectiveService->getOrCreatePerspective($post);
        $documentId = $perspective['document_id'];

        $this->actingAs($this->adminUser);

        // Publish
        $published = $this->perspectiveService->publish($documentId);
        $this->assertFalse($published['_hasUnpublishedChanges']);

        // Unpublish
        $unpublished = $this->perspectiveService->unpublish($documentId);
        $this->assertTrue($unpublished['_hasUnpublishedChanges']);
    }

    /** @test */
    public function it_can_generate_and_validate_preview_tokens(): void
    {
        $post = Post::factory()->create(['status' => 'draft']);
        $perspective = $this->perspectiveService->getOrCreatePerspective($post);
        $documentId = $perspective['document_id'];

        $this->actingAs($this->adminUser);
        $token = $this->perspectiveService->createPreviewToken($documentId);

        $this->assertNotEmpty($token);
        $this->assertEquals(64, strlen($token));

        // Validate token
        $validation = $this->perspectiveService->validatePreviewToken($token);
        $this->assertNotNull($validation);
        $this->assertEquals($documentId, $validation['documentId']);

        // Get document by token
        $document = $this->perspectiveService->getByPreviewToken($token);
        $this->assertNotNull($document);
    }

    /** @test */
    public function it_can_calculate_diff_between_draft_and_published(): void
    {
        $post = Post::factory()->create(['status' => 'draft', 'title' => 'Original']);
        $perspective = $this->perspectiveService->getOrCreatePerspective($post);
        $documentId = $perspective['document_id'];

        $this->actingAs($this->adminUser);

        // Publish first
        $this->perspectiveService->publish($documentId);

        // Update draft
        $this->perspectiveService->updateDraft($documentId, [
            'title' => 'Modified',
            'new_field' => 'Added',
        ]);

        $diff = $this->perspectiveService->getDiff($documentId);

        $this->assertIsArray($diff);
        $this->assertNotEmpty($diff);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // DOCUMENT HISTORY TESTS
    // ═══════════════════════════════════════════════════════════════════════

    /** @test */
    public function it_records_document_history(): void
    {
        $documentId = Str::uuid()->toString();

        $this->historyService->recordChange(
            $documentId,
            'post',
            ['title' => 'First Version'],
            'create',
            $this->adminUser->id
        );

        $this->historyService->recordChange(
            $documentId,
            'post',
            ['title' => 'Second Version'],
            'update',
            $this->adminUser->id
        );

        $history = $this->historyService->getHistory($documentId);

        $this->assertEquals(2, $history['total']);
        $this->assertCount(2, $history['items']);
    }

    /** @test */
    public function it_can_get_specific_revision(): void
    {
        $documentId = Str::uuid()->toString();

        $this->historyService->recordChange(
            $documentId,
            'post',
            ['title' => 'Version 1', 'content' => 'Content 1'],
            'create'
        );

        $revision = $this->historyService->getRevision($documentId, 1);

        $this->assertNotNull($revision);
        $this->assertEquals(1, $revision['revision']);
        $this->assertEquals('Version 1', $revision['content']['title']);
    }

    /** @test */
    public function it_can_compare_revisions(): void
    {
        $documentId = Str::uuid()->toString();

        $this->historyService->recordChange(
            $documentId,
            'post',
            ['title' => 'Old Title', 'content' => 'Same content'],
            'create'
        );

        $this->historyService->recordChange(
            $documentId,
            'post',
            ['title' => 'New Title', 'content' => 'Same content'],
            'update'
        );

        $comparison = $this->historyService->compareRevisions($documentId, 1, 2);

        $this->assertArrayHasKey('from', $comparison);
        $this->assertArrayHasKey('to', $comparison);
        $this->assertArrayHasKey('patches', $comparison);
        $this->assertArrayHasKey('diff', $comparison);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // WEBHOOK TESTS
    // ═══════════════════════════════════════════════════════════════════════

    /** @test */
    public function it_can_create_webhooks(): void
    {
        $this->actingAs($this->adminUser);

        $webhook = $this->webhookService->create([
            'name' => 'Test Webhook',
            'url' => 'https://example.com/webhook',
            'events' => ['document.published', 'document.updated'],
        ]);

        $this->assertArrayHasKey('webhook_id', $webhook);
        $this->assertArrayHasKey('secret', $webhook);
        $this->assertNotEmpty($webhook['secret']);
    }

    /** @test */
    public function it_can_list_webhooks(): void
    {
        $this->actingAs($this->adminUser);

        $this->webhookService->create([
            'name' => 'Webhook 1',
            'url' => 'https://example.com/hook1',
            'events' => ['*'],
        ]);

        $this->webhookService->create([
            'name' => 'Webhook 2',
            'url' => 'https://example.com/hook2',
            'events' => ['*'],
        ]);

        $list = $this->webhookService->list();

        $this->assertEquals(2, $list['total']);
        $this->assertCount(2, $list['items']);
    }

    /** @test */
    public function it_generates_correct_webhook_signature(): void
    {
        $payload = '{"test": true}';
        $secret = 'my-secret-key';

        $signature = hash_hmac('sha256', $payload, $secret);
        $isValid = $this->webhookService->verifySignature($payload, $signature, $secret);

        $this->assertTrue($isValid);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // SCHEMA TESTS
    // ═══════════════════════════════════════════════════════════════════════

    /** @test */
    public function it_can_register_schema(): void
    {
        $schema = $this->schemaService->register([
            'name' => 'article',
            'title' => 'Article',
            'fields' => [
                ['name' => 'title', 'type' => 'string', 'validation' => ['required' => true]],
                ['name' => 'content', 'type' => 'portableText'],
                ['name' => 'publishedAt', 'type' => 'datetime'],
            ],
        ]);

        $this->assertEquals('article', $schema['name']);
        $this->assertCount(3, $schema['fields']);
    }

    /** @test */
    public function it_can_validate_document_against_schema(): void
    {
        $this->schemaService->register([
            'name' => 'testDoc',
            'fields' => [
                ['name' => 'title', 'type' => 'string', 'validation' => ['required' => true, 'max' => 100]],
                ['name' => 'count', 'type' => 'number', 'validation' => ['min' => 0]],
            ],
        ]);

        // Valid document
        $result = $this->schemaService->validate('testDoc', [
            'title' => 'My Title',
            'count' => 5,
        ]);

        $this->assertTrue($result['valid']);

        // Invalid document
        $result = $this->schemaService->validate('testDoc', [
            'title' => '', // Required field empty
            'count' => -1, // Below min
        ]);

        $this->assertFalse($result['valid']);
    }

    /** @test */
    public function it_can_generate_typescript_from_schema(): void
    {
        $this->schemaService->register([
            'name' => 'blogPost',
            'fields' => [
                ['name' => 'title', 'type' => 'string', 'validation' => ['required' => true]],
                ['name' => 'published', 'type' => 'boolean'],
                ['name' => 'views', 'type' => 'number'],
            ],
        ]);

        $typescript = $this->schemaService->generateTypeScript('blogPost');

        $this->assertStringContainsString('export interface BlogPost', $typescript);
        $this->assertStringContainsString('title: string', $typescript);
        $this->assertStringContainsString('published?: boolean', $typescript);
        $this->assertStringContainsString('views?: number', $typescript);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // IMAGE HOTSPOT TESTS
    // ═══════════════════════════════════════════════════════════════════════

    /** @test */
    public function it_can_set_and_get_hotspot(): void
    {
        // Create a media record
        $mediaId = DB::table('media')->insertGetId([
            'filename' => 'test.jpg',
            'path' => 'images/test.jpg',
            'mime_type' => 'image/jpeg',
            'size' => 1024,
            'width' => 800,
            'height' => 600,
            'created_at' => now(),
            'updated_at' => now(),
        ]);

        $hotspotService = app(ImageHotspotService::class);

        $hotspot = $hotspotService->setHotspot($mediaId, 0.3, 0.7);

        $this->assertEquals(0.3, $hotspot['x']);
        $this->assertEquals(0.7, $hotspot['y']);
        $this->assertTrue($hotspot['isDefault']);

        // Retrieve hotspot
        $retrieved = $hotspotService->getHotspot($mediaId);
        $this->assertEquals(0.3, $retrieved['x']);
    }

    /** @test */
    public function it_can_set_and_get_crop(): void
    {
        $mediaId = DB::table('media')->insertGetId([
            'filename' => 'test.jpg',
            'path' => 'images/test.jpg',
            'mime_type' => 'image/jpeg',
            'size' => 1024,
            'created_at' => now(),
            'updated_at' => now(),
        ]);

        $hotspotService = app(ImageHotspotService::class);

        $crop = $hotspotService->setCrop($mediaId, 'square', 0.1, 0.1, 0.9, 0.9, '1:1');

        $this->assertEquals('square', $crop['name']);
        $this->assertEquals('1:1', $crop['aspectRatio']);
        $this->assertEquals(0.1, $crop['top']);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // RELEASE BUNDLE TESTS
    // ═══════════════════════════════════════════════════════════════════════

    /** @test */
    public function it_can_create_release_bundle(): void
    {
        $this->actingAs($this->adminUser);

        $releaseService = app(ReleaseBundleService::class);

        $release = $releaseService->create([
            'name' => 'Q1 2025 Release',
            'description' => 'First quarter content release',
        ]);

        $this->assertNotNull($release['id']);
        $this->assertEquals('Q1 2025 Release', $release['name']);
        $this->assertEquals('draft', $release['status']);
    }

    /** @test */
    public function it_can_add_documents_to_release(): void
    {
        $this->actingAs($this->adminUser);

        $releaseService = app(ReleaseBundleService::class);

        $release = $releaseService->create(['name' => 'Test Release']);

        $post = Post::factory()->create(['status' => 'draft']);
        $perspective = $this->perspectiveService->getOrCreatePerspective($post);

        $releaseService->addDocument(
            $release['id'],
            $perspective['document_id'],
            'post',
            'update'
        );

        $updated = $releaseService->get($release['id']);

        $this->assertEquals(1, $updated['documentCount']);
    }

    /** @test */
    public function it_can_schedule_release(): void
    {
        $this->actingAs($this->adminUser);

        $releaseService = app(ReleaseBundleService::class);

        $release = $releaseService->create(['name' => 'Scheduled Release']);
        $scheduledAt = now()->addDays(7)->toDateTimeString();

        $scheduled = $releaseService->schedule($release['id'], $scheduledAt);

        $this->assertEquals('scheduled', $scheduled['status']);
        $this->assertNotNull($scheduled['scheduledAt']);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // API ENDPOINT TESTS
    // ═══════════════════════════════════════════════════════════════════════

    /** @test */
    public function api_can_execute_groq_query(): void
    {
        Post::factory()->count(3)->create(['status' => 'published']);

        $response = $this->actingAs($this->adminUser)
            ->postJson('/api/content-lake/query', [
                'query' => '*[_type == "post"]',
            ]);

        $response->assertOk();
        $response->assertJsonStructure(['data', 'ms', 'cached']);
    }

    /** @test */
    public function api_can_render_portable_text(): void
    {
        $response = $this->actingAs($this->adminUser)
            ->postJson('/api/content-lake/portable-text/render', [
                'blocks' => [
                    [
                        '_type' => 'block',
                        'style' => 'normal',
                        'children' => [
                            ['_type' => 'span', 'text' => 'Hello', 'marks' => []],
                        ],
                        'markDefs' => [],
                    ],
                ],
            ]);

        $response->assertOk();
        $response->assertJsonStructure(['html', 'plainText', 'wordCount', 'readingTime']);
    }

    /** @test */
    public function api_can_manage_webhooks(): void
    {
        // Create
        $response = $this->actingAs($this->adminUser)
            ->postJson('/api/admin/content-lake/webhooks', [
                'name' => 'API Test Webhook',
                'url' => 'https://api.example.com/hook',
                'events' => ['document.published'],
            ]);

        $response->assertCreated();
        $webhookId = $response->json('data.webhook_id');

        // List
        $response = $this->actingAs($this->adminUser)
            ->getJson('/api/admin/content-lake/webhooks');

        $response->assertOk();
        $this->assertEquals(1, $response->json('total'));

        // Delete
        $response = $this->actingAs($this->adminUser)
            ->deleteJson("/api/admin/content-lake/webhooks/{$webhookId}");

        $response->assertOk();
    }

    /** @test */
    public function api_can_manage_schemas(): void
    {
        // Register
        $response = $this->actingAs($this->adminUser)
            ->postJson('/api/admin/content-lake/schemas', [
                'name' => 'apiTest',
                'title' => 'API Test Schema',
                'fields' => [
                    ['name' => 'name', 'type' => 'string'],
                ],
            ]);

        $response->assertCreated();

        // List
        $response = $this->actingAs($this->adminUser)
            ->getJson('/api/admin/content-lake/schemas');

        $response->assertOk();

        // Get
        $response = $this->actingAs($this->adminUser)
            ->getJson('/api/admin/content-lake/schemas/apiTest');

        $response->assertOk();
        $this->assertEquals('apiTest', $response->json('data.name'));
    }

    /** @test */
    public function api_can_manage_releases(): void
    {
        // Create
        $response = $this->actingAs($this->adminUser)
            ->postJson('/api/admin/content-lake/releases', [
                'name' => 'API Test Release',
            ]);

        $response->assertCreated();
        $bundleId = $response->json('data.id');

        // Get
        $response = $this->actingAs($this->adminUser)
            ->getJson("/api/admin/content-lake/releases/{$bundleId}");

        $response->assertOk();

        // List
        $response = $this->actingAs($this->adminUser)
            ->getJson('/api/admin/content-lake/releases');

        $response->assertOk();
        $this->assertEquals(1, $response->json('total'));
    }

    /** @test */
    public function preview_token_endpoint_is_public(): void
    {
        $post = Post::factory()->create(['status' => 'draft']);
        $perspective = $this->perspectiveService->getOrCreatePerspective($post);

        $this->actingAs($this->adminUser);
        $token = $this->perspectiveService->createPreviewToken($perspective['document_id']);

        // Access without authentication
        $response = $this->getJson("/api/preview?token={$token}");

        $response->assertOk();
        $response->assertJsonStructure(['data']);
    }

    /** @test */
    public function invalid_preview_token_returns_404(): void
    {
        $response = $this->getJson('/api/preview?token=invalid-token-here');

        $response->assertNotFound();
    }
}
