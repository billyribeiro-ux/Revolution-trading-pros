<?php

declare(strict_types=1);

namespace Tests\Unit\ContentLake;

use App\Services\ContentLake\PortableTextService;
use PHPUnit\Framework\TestCase;

/**
 * Unit tests for PortableTextService - no database required
 */
class PortableTextServiceTest extends TestCase
{
    private PortableTextService $service;

    protected function setUp(): void
    {
        parent::setUp();
        $this->service = new PortableTextService();
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PARSING TESTS
    // ═══════════════════════════════════════════════════════════════════════

    /** @test */
    public function test_it_can_parse_simple_block(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                '_key' => 'key1',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', '_key' => 's1', 'text' => 'Hello World', 'marks' => []],
                ],
                'markDefs' => [],
            ],
        ];

        $parsed = $this->service->parse($blocks);

        $this->assertCount(1, $parsed);
        $this->assertEquals('block', $parsed[0]['_type']);
        $this->assertEquals('normal', $parsed[0]['style']);
        $this->assertEquals('Hello World', $parsed[0]['children'][0]['text']);
    }

    /** @test */
    public function test_it_generates_keys_for_blocks_without_keys(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', 'text' => 'Test'],
                ],
            ],
        ];

        $parsed = $this->service->parse($blocks);

        $this->assertNotEmpty($parsed[0]['_key']);
        $this->assertNotEmpty($parsed[0]['children'][0]['_key']);
    }

    /** @test */
    public function test_it_parses_different_styles(): void
    {
        $styles = ['normal', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'blockquote'];

        foreach ($styles as $style) {
            $blocks = [
                [
                    '_type' => 'block',
                    'style' => $style,
                    'children' => [['_type' => 'span', 'text' => 'Test']],
                    'markDefs' => [],
                ],
            ];

            $parsed = $this->service->parse($blocks);
            $this->assertEquals($style, $parsed[0]['style']);
        }
    }

    /** @test */
    public function test_it_throws_on_invalid_style(): void
    {
        $this->expectException(\InvalidArgumentException::class);

        $blocks = [
            [
                '_type' => 'block',
                'style' => 'invalid-style',
                'children' => [['_type' => 'span', 'text' => 'Test']],
            ],
        ];

        $this->service->parse($blocks);
    }

    /** @test */
    public function test_it_parses_list_items(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                'style' => 'normal',
                'listItem' => 'bullet',
                'level' => 1,
                'children' => [['_type' => 'span', 'text' => 'Item 1']],
                'markDefs' => [],
            ],
        ];

        $parsed = $this->service->parse($blocks);

        $this->assertEquals('bullet', $parsed[0]['listItem']);
        $this->assertEquals(1, $parsed[0]['level']);
    }

    /** @test */
    public function test_it_parses_marks(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                '_key' => 'key1',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', 'text' => 'Bold', 'marks' => ['strong']],
                    ['_type' => 'span', 'text' => 'Italic', 'marks' => ['em']],
                    ['_type' => 'span', 'text' => 'Code', 'marks' => ['code']],
                ],
                'markDefs' => [],
            ],
        ];

        $parsed = $this->service->parse($blocks);

        $this->assertContains('strong', $parsed[0]['children'][0]['marks']);
        $this->assertContains('em', $parsed[0]['children'][1]['marks']);
        $this->assertContains('code', $parsed[0]['children'][2]['marks']);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // HTML RENDERING TESTS
    // ═══════════════════════════════════════════════════════════════════════

    /** @test */
    public function test_it_renders_paragraph_to_html(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', 'text' => 'Hello World', 'marks' => []],
                ],
                'markDefs' => [],
            ],
        ];

        $html = $this->service->toHtml($blocks);

        $this->assertEquals('<p>Hello World</p>', $html);
    }

    /** @test */
    public function test_it_renders_headings_to_html(): void
    {
        for ($i = 1; $i <= 6; $i++) {
            $blocks = [
                [
                    '_type' => 'block',
                    'style' => "h{$i}",
                    'children' => [
                        ['_type' => 'span', 'text' => "Heading {$i}", 'marks' => []],
                    ],
                    'markDefs' => [],
                ],
            ];

            $html = $this->service->toHtml($blocks);
            $this->assertEquals("<h{$i}>Heading {$i}</h{$i}>", $html);
        }
    }

    /** @test */
    public function test_it_renders_blockquote_to_html(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                'style' => 'blockquote',
                'children' => [
                    ['_type' => 'span', 'text' => 'A famous quote', 'marks' => []],
                ],
                'markDefs' => [],
            ],
        ];

        $html = $this->service->toHtml($blocks);

        $this->assertEquals('<blockquote>A famous quote</blockquote>', $html);
    }

    /** @test */
    public function test_it_renders_strong_text(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', 'text' => 'Bold text', 'marks' => ['strong']],
                ],
                'markDefs' => [],
            ],
        ];

        $html = $this->service->toHtml($blocks);

        $this->assertStringContainsString('<strong>Bold text</strong>', $html);
    }

    /** @test */
    public function test_it_renders_italic_text(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', 'text' => 'Italic text', 'marks' => ['em']],
                ],
                'markDefs' => [],
            ],
        ];

        $html = $this->service->toHtml($blocks);

        $this->assertStringContainsString('<em>Italic text</em>', $html);
    }

    /** @test */
    public function test_it_renders_inline_code(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', 'text' => 'code here', 'marks' => ['code']],
                ],
                'markDefs' => [],
            ],
        ];

        $html = $this->service->toHtml($blocks);

        $this->assertStringContainsString('<code>code here</code>', $html);
    }

    /** @test */
    public function test_it_renders_combined_marks(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', 'text' => 'Bold and italic', 'marks' => ['strong', 'em']],
                ],
                'markDefs' => [],
            ],
        ];

        $html = $this->service->toHtml($blocks);

        $this->assertStringContainsString('<strong>', $html);
        $this->assertStringContainsString('<em>', $html);
        $this->assertStringContainsString('Bold and italic', $html);
    }

    /** @test */
    public function test_it_renders_links(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', 'text' => 'Click here', 'marks' => ['link1']],
                ],
                'markDefs' => [
                    ['_type' => 'link', '_key' => 'link1', 'href' => 'https://example.com'],
                ],
            ],
        ];

        $html = $this->service->toHtml($blocks);

        $this->assertStringContainsString('href="https://example.com"', $html);
        $this->assertStringContainsString('Click here', $html);
    }

    /** @test */
    public function test_it_renders_links_with_blank_target(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', 'text' => 'External', 'marks' => ['link1']],
                ],
                'markDefs' => [
                    ['_type' => 'link', '_key' => 'link1', 'href' => 'https://external.com', 'blank' => true],
                ],
            ],
        ];

        $html = $this->service->toHtml($blocks);

        $this->assertStringContainsString('target="_blank"', $html);
        $this->assertStringContainsString('rel="noopener noreferrer"', $html);
    }

    /** @test */
    public function test_it_renders_bullet_list(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                'style' => 'normal',
                'listItem' => 'bullet',
                'level' => 1,
                'children' => [['_type' => 'span', 'text' => 'Item 1', 'marks' => []]],
                'markDefs' => [],
            ],
            [
                '_type' => 'block',
                'style' => 'normal',
                'listItem' => 'bullet',
                'level' => 1,
                'children' => [['_type' => 'span', 'text' => 'Item 2', 'marks' => []]],
                'markDefs' => [],
            ],
        ];

        $html = $this->service->toHtml($blocks);

        $this->assertStringContainsString('<ul>', $html);
        $this->assertStringContainsString('<li>Item 1</li>', $html);
        $this->assertStringContainsString('<li>Item 2</li>', $html);
        $this->assertStringContainsString('</ul>', $html);
    }

    /** @test */
    public function test_it_renders_numbered_list(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                'style' => 'normal',
                'listItem' => 'number',
                'level' => 1,
                'children' => [['_type' => 'span', 'text' => 'First', 'marks' => []]],
                'markDefs' => [],
            ],
            [
                '_type' => 'block',
                'style' => 'normal',
                'listItem' => 'number',
                'level' => 1,
                'children' => [['_type' => 'span', 'text' => 'Second', 'marks' => []]],
                'markDefs' => [],
            ],
        ];

        $html = $this->service->toHtml($blocks);

        $this->assertStringContainsString('<ol>', $html);
        $this->assertStringContainsString('<li>First</li>', $html);
        $this->assertStringContainsString('<li>Second</li>', $html);
        $this->assertStringContainsString('</ol>', $html);
    }

    /** @test */
    public function test_it_renders_code_block(): void
    {
        $blocks = [
            [
                '_type' => 'code',
                '_key' => 'code1',
                'code' => 'const x = 1;',
                'language' => 'javascript',
            ],
        ];

        $html = $this->service->toHtml($blocks);

        $this->assertStringContainsString('<pre><code', $html);
        $this->assertStringContainsString('class="language-javascript"', $html);
        $this->assertStringContainsString('const x = 1;', $html);
    }

    /** @test */
    public function test_it_renders_code_block_with_filename(): void
    {
        $blocks = [
            [
                '_type' => 'code',
                '_key' => 'code1',
                'code' => 'echo "hello"',
                'language' => 'bash',
                'filename' => 'script.sh',
            ],
        ];

        $html = $this->service->toHtml($blocks);

        $this->assertStringContainsString('script.sh', $html);
        $this->assertStringContainsString('class="language-bash"', $html);
    }

    /** @test */
    public function test_it_renders_image_block(): void
    {
        $blocks = [
            [
                '_type' => 'image',
                '_key' => 'img1',
                'asset' => ['_ref' => 'media-123'],
                'alt' => 'Test image',
            ],
        ];

        $html = $this->service->toHtml($blocks);

        $this->assertStringContainsString('<img', $html);
        $this->assertStringContainsString('alt="Test image"', $html);
        $this->assertStringContainsString('loading="lazy"', $html);
    }

    /** @test */
    public function test_it_renders_image_with_caption(): void
    {
        $blocks = [
            [
                '_type' => 'image',
                '_key' => 'img1',
                'asset' => ['url' => 'https://example.com/image.jpg'],
                'alt' => 'Alt text',
                'caption' => 'Image caption',
            ],
        ];

        $html = $this->service->toHtml($blocks);

        $this->assertStringContainsString('<figure>', $html);
        $this->assertStringContainsString('<figcaption>Image caption</figcaption>', $html);
    }

    /** @test */
    public function test_it_escapes_html_in_text(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', 'text' => '<script>alert("xss")</script>', 'marks' => []],
                ],
                'markDefs' => [],
            ],
        ];

        $html = $this->service->toHtml($blocks);

        $this->assertStringNotContainsString('<script>', $html);
        $this->assertStringContainsString('&lt;script&gt;', $html);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PLAIN TEXT EXTRACTION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    /** @test */
    public function test_it_extracts_plain_text(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', 'text' => 'Hello ', 'marks' => []],
                    ['_type' => 'span', 'text' => 'World', 'marks' => ['strong']],
                ],
                'markDefs' => [],
            ],
        ];

        $text = $this->service->toPlainText($blocks);

        $this->assertEquals('Hello World', $text);
    }

    /** @test */
    public function test_it_calculates_word_count(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', 'text' => 'This is a test sentence with eight words.', 'marks' => []],
                ],
                'markDefs' => [],
            ],
        ];

        $wordCount = $this->service->wordCount($blocks);

        $this->assertEquals(8, $wordCount);
    }

    /** @test */
    public function test_it_calculates_reading_time(): void
    {
        // Create 400 words (should be 2 minutes at 200 wpm)
        $text = str_repeat('word ', 400);
        $blocks = [
            [
                '_type' => 'block',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', 'text' => trim($text), 'marks' => []],
                ],
                'markDefs' => [],
            ],
        ];

        $readingTime = $this->service->readingTime($blocks);

        $this->assertEquals(2, $readingTime);
    }

    /** @test */
    public function test_it_returns_minimum_one_minute_reading_time(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', 'text' => 'Short text', 'marks' => []],
                ],
                'markDefs' => [],
            ],
        ];

        $readingTime = $this->service->readingTime($blocks);

        $this->assertEquals(1, $readingTime);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // VALIDATION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    /** @test */
    public function test_it_validates_correct_structure(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                '_key' => 'key1',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', '_key' => 's1', 'text' => 'Valid', 'marks' => []],
                ],
                'markDefs' => [],
            ],
        ];

        $result = $this->service->validate($blocks);

        $this->assertTrue($result->isValid);
        $this->assertEmpty($result->errors);
    }

    /** @test */
    public function test_it_detects_missing_type(): void
    {
        $blocks = [
            [
                '_key' => 'key1',
                'style' => 'normal',
                'children' => [],
            ],
        ];

        $result = $this->service->validate($blocks);

        $this->assertFalse($result->isValid);
        $this->assertNotEmpty($result->errors);
        $this->assertStringContainsString("Missing required field '_type'", $result->errors[0]);
    }

    /** @test */
    public function test_it_detects_missing_children(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                '_key' => 'key1',
                'style' => 'normal',
                'markDefs' => [],
            ],
        ];

        $result = $this->service->validate($blocks);

        $this->assertFalse($result->isValid);
        $this->assertStringContainsString("must have 'children'", $result->errors[0]);
    }

    /** @test */
    public function test_it_detects_missing_key_as_warning(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', 'text' => 'Text', 'marks' => []],
                ],
                'markDefs' => [],
            ],
        ];

        $result = $this->service->validate($blocks);

        $this->assertTrue($result->isValid);
        $this->assertNotEmpty($result->warnings);
    }

    /** @test */
    public function test_it_detects_invalid_mark_reference(): void
    {
        $blocks = [
            [
                '_type' => 'block',
                '_key' => 'key1',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', '_key' => 's1', 'text' => 'Text', 'marks' => ['nonexistent']],
                ],
                'markDefs' => [],
            ],
        ];

        $result = $this->service->validate($blocks);

        $this->assertFalse($result->isValid);
        $this->assertStringContainsString("references non-existent markDef", $result->errors[0]);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // DOCUMENT BUILDER TESTS
    // ═══════════════════════════════════════════════════════════════════════

    /** @test */
    public function test_it_can_build_document_with_fluent_api(): void
    {
        $document = $this->service->createDocument()
            ->heading(1, 'Title')
            ->paragraph('A paragraph.');

        $blocks = $document->toArray();

        $this->assertCount(2, $blocks);
        $this->assertEquals('h1', $blocks[0]['style']);
        $this->assertEquals('normal', $blocks[1]['style']);
    }

    /** @test */
    public function test_it_builds_bullet_list(): void
    {
        $document = $this->service->createDocument()
            ->bulletList(['Item A', 'Item B', 'Item C']);

        $blocks = $document->toArray();

        $this->assertCount(3, $blocks);
        $this->assertEquals('bullet', $blocks[0]['listItem']);
        $this->assertEquals('Item A', $blocks[0]['children'][0]['text']);
    }

    /** @test */
    public function test_it_builds_numbered_list(): void
    {
        $document = $this->service->createDocument()
            ->numberedList(['First', 'Second']);

        $blocks = $document->toArray();

        $this->assertCount(2, $blocks);
        $this->assertEquals('number', $blocks[0]['listItem']);
    }

    /** @test */
    public function test_it_builds_code_block(): void
    {
        $document = $this->service->createDocument()
            ->code('function test() {}', 'javascript', 'test.js');

        $blocks = $document->toArray();

        $this->assertCount(1, $blocks);
        $this->assertEquals('code', $blocks[0]['_type']);
        $this->assertEquals('javascript', $blocks[0]['language']);
        $this->assertEquals('test.js', $blocks[0]['filename']);
    }

    /** @test */
    public function test_it_builds_blockquote(): void
    {
        $document = $this->service->createDocument()
            ->blockquote('Famous quote here');

        $blocks = $document->toArray();

        $this->assertCount(1, $blocks);
        $this->assertEquals('blockquote', $blocks[0]['style']);
    }

    /** @test */
    public function test_it_builds_image(): void
    {
        $document = $this->service->createDocument()
            ->image('media-123', 'Alt text', 'Caption');

        $blocks = $document->toArray();

        $this->assertCount(1, $blocks);
        $this->assertEquals('image', $blocks[0]['_type']);
        $this->assertEquals('Alt text', $blocks[0]['alt']);
        $this->assertEquals('Caption', $blocks[0]['caption']);
    }

    /** @test */
    public function test_it_builds_callout(): void
    {
        $document = $this->service->createDocument()
            ->callout('warning', 'Warning Title', 'Warning message');

        $blocks = $document->toArray();

        $this->assertCount(1, $blocks);
        $this->assertEquals('callout', $blocks[0]['_type']);
        $this->assertEquals('warning', $blocks[0]['type']);
        $this->assertEquals('Warning Title', $blocks[0]['title']);
    }

    /** @test */
    public function test_it_converts_document_to_html(): void
    {
        $html = $this->service->createDocument()
            ->heading(1, 'Hello')
            ->paragraph('World')
            ->toHtml();

        $this->assertStringContainsString('<h1>Hello</h1>', $html);
        $this->assertStringContainsString('<p>World</p>', $html);
    }

    /** @test */
    public function test_it_converts_document_to_plain_text(): void
    {
        $text = $this->service->createDocument()
            ->heading(1, 'Hello')
            ->paragraph('World')
            ->toPlainText();

        $this->assertStringContainsString('Hello', $text);
        $this->assertStringContainsString('World', $text);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // CUSTOM SERIALIZER TESTS
    // ═══════════════════════════════════════════════════════════════════════

    /** @test */
    public function test_it_can_register_custom_serializer(): void
    {
        $this->service->registerSerializer('customBlock', function (array $block): string {
            return '<div class="custom">' . ($block['content'] ?? '') . '</div>';
        });

        $blocks = [
            [
                '_type' => 'customBlock',
                '_key' => 'custom1',
                'content' => 'Custom content',
            ],
        ];

        $html = $this->service->toHtml($blocks);

        $this->assertStringContainsString('<div class="custom">Custom content</div>', $html);
    }

    /** @test */
    public function test_it_can_register_custom_annotation(): void
    {
        $this->service->registerAnnotation('highlight', [
            'title' => 'Highlight',
            'serializer' => function (array $markDef, string $text): string {
                $color = $markDef['color'] ?? 'yellow';
                return "<mark style=\"background:{$color}\">{$text}</mark>";
            },
        ]);

        $blocks = [
            [
                '_type' => 'block',
                'style' => 'normal',
                'children' => [
                    ['_type' => 'span', 'text' => 'Highlighted', 'marks' => ['hl1']],
                ],
                'markDefs' => [
                    ['_type' => 'highlight', '_key' => 'hl1', 'color' => 'pink'],
                ],
            ],
        ];

        $html = $this->service->toHtml($blocks);

        $this->assertStringContainsString('<mark style="background:pink">', $html);
    }

    /** @test */
    public function test_it_can_register_custom_block_type(): void
    {
        $this->service->registerBlockType('video', [
            'title' => 'Video',
            'fields' => [
                'url' => ['type' => 'url', 'required' => true],
                'title' => ['type' => 'string'],
            ],
        ]);

        $this->service->registerSerializer('video', function (array $block): string {
            return '<video src="' . htmlspecialchars($block['url']) . '"></video>';
        });

        $blocks = [
            [
                '_type' => 'video',
                '_key' => 'v1',
                'url' => 'https://example.com/video.mp4',
            ],
        ];

        $parsed = $this->service->parse($blocks);
        $this->assertEquals('video', $parsed[0]['_type']);

        $html = $this->service->toHtml($blocks);
        $this->assertStringContainsString('<video', $html);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // CALLOUT AND EMBED SERIALIZER TESTS
    // ═══════════════════════════════════════════════════════════════════════

    /** @test */
    public function test_it_renders_callout_block(): void
    {
        $blocks = [
            [
                '_type' => 'callout',
                '_key' => 'c1',
                'type' => 'info',
                'title' => 'Note',
                'body' => [
                    [
                        '_type' => 'block',
                        'style' => 'normal',
                        'children' => [['_type' => 'span', 'text' => 'Important info', 'marks' => []]],
                        'markDefs' => [],
                    ],
                ],
            ],
        ];

        $html = $this->service->toHtml($blocks);

        $this->assertStringContainsString('class="callout callout-info"', $html);
        $this->assertStringContainsString('class="callout-title">Note</div>', $html);
        $this->assertStringContainsString('Important info', $html);
    }

    /** @test */
    public function test_it_renders_table_block(): void
    {
        $blocks = [
            [
                '_type' => 'table',
                '_key' => 't1',
                'rows' => [
                    ['cells' => ['Header 1', 'Header 2']],
                    ['cells' => ['Cell 1', 'Cell 2']],
                ],
            ],
        ];

        $html = $this->service->toHtml($blocks);

        $this->assertStringContainsString('<table>', $html);
        $this->assertStringContainsString('<th>Header 1</th>', $html);
        $this->assertStringContainsString('<td>Cell 1</td>', $html);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // KEY GENERATION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    /** @test */
    public function test_it_generates_unique_keys(): void
    {
        $keys = [];
        for ($i = 0; $i < 100; $i++) {
            $keys[] = $this->service->generateKey();
        }

        $uniqueKeys = array_unique($keys);
        $this->assertCount(100, $uniqueKeys);
    }

    /** @test */
    public function generated_keys_have_correct_length(): void
    {
        $key = $this->service->generateKey();
        $this->assertEquals(12, strlen($key));
    }
}
