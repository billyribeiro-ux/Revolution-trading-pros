<?php

declare(strict_types=1);

namespace App\Services\ContentLake;

use Illuminate\Support\Str;
use InvalidArgumentException;

/**
 * Portable Text Service
 *
 * Implements Sanity's Portable Text specification for rich text content.
 * https://portabletext.org/
 *
 * Features:
 * - Block-level content (paragraphs, headings, lists, custom blocks)
 * - Span-level decorators (bold, italic, underline, code, etc.)
 * - Annotations (links, internal references, custom annotations)
 * - Custom block types
 * - Validation
 * - HTML/React serialization
 */
class PortableTextService
{
    /**
     * Default block styles
     */
    private const DEFAULT_STYLES = [
        'normal',
        'h1', 'h2', 'h3', 'h4', 'h5', 'h6',
        'blockquote',
    ];

    /**
     * Default decorators (marks)
     */
    private const DEFAULT_DECORATORS = [
        'strong',
        'em',
        'underline',
        'strike',
        'code',
        'sup',
        'sub',
    ];

    /**
     * Default list types
     */
    private const DEFAULT_LIST_TYPES = [
        'bullet',
        'number',
    ];

    /**
     * Built-in annotation types
     */
    private array $annotationTypes = [];

    /**
     * Custom block types
     */
    private array $blockTypes = [];

    /**
     * Custom serializers
     */
    private array $serializers = [];

    public function __construct()
    {
        $this->registerDefaultAnnotations();
        $this->registerDefaultBlockTypes();
        $this->registerDefaultSerializers();
    }

    /**
     * Create a new Portable Text document
     */
    public function createDocument(): PortableTextDocument
    {
        return new PortableTextDocument($this);
    }

    /**
     * Parse raw Portable Text array into validated structure
     */
    public function parse(array $blocks): array
    {
        $parsed = [];

        foreach ($blocks as $block) {
            $parsed[] = $this->parseBlock($block);
        }

        return $parsed;
    }

    /**
     * Parse single block
     */
    private function parseBlock(array $block): array
    {
        // Validate required fields
        if (!isset($block['_type'])) {
            throw new InvalidArgumentException('Block must have _type');
        }

        // Generate key if missing
        if (!isset($block['_key'])) {
            $block['_key'] = $this->generateKey();
        }

        // Handle standard block type
        if ($block['_type'] === 'block') {
            return $this->parseTextBlock($block);
        }

        // Handle custom block type
        if (isset($this->blockTypes[$block['_type']])) {
            return $this->parseCustomBlock($block);
        }

        // Unknown block type - pass through with validation
        return $block;
    }

    /**
     * Parse text block
     */
    private function parseTextBlock(array $block): array
    {
        $parsed = [
            '_type' => 'block',
            '_key' => $block['_key'] ?? $this->generateKey(),
            'style' => $block['style'] ?? 'normal',
            'children' => [],
            'markDefs' => $block['markDefs'] ?? [],
        ];

        // Validate style
        if (!in_array($parsed['style'], self::DEFAULT_STYLES, true)) {
            throw new InvalidArgumentException("Invalid block style: {$parsed['style']}");
        }

        // Handle list item
        if (isset($block['listItem'])) {
            if (!in_array($block['listItem'], self::DEFAULT_LIST_TYPES, true)) {
                throw new InvalidArgumentException("Invalid list type: {$block['listItem']}");
            }
            $parsed['listItem'] = $block['listItem'];
            $parsed['level'] = $block['level'] ?? 1;
        }

        // Parse children (spans)
        foreach ($block['children'] ?? [] as $child) {
            $parsed['children'][] = $this->parseSpan($child, $parsed['markDefs']);
        }

        return $parsed;
    }

    /**
     * Parse span (inline text)
     */
    private function parseSpan(array $span, array $markDefs): array
    {
        if (!isset($span['_type'])) {
            $span['_type'] = 'span';
        }

        if ($span['_type'] !== 'span') {
            // Inline object
            return $span;
        }

        $parsed = [
            '_type' => 'span',
            '_key' => $span['_key'] ?? $this->generateKey(),
            'text' => $span['text'] ?? '',
            'marks' => [],
        ];

        // Parse marks
        foreach ($span['marks'] ?? [] as $mark) {
            // Check if it's a decorator
            if (in_array($mark, self::DEFAULT_DECORATORS, true)) {
                $parsed['marks'][] = $mark;
                continue;
            }

            // Check if it's an annotation reference
            $markDef = collect($markDefs)->firstWhere('_key', $mark);
            if ($markDef) {
                $parsed['marks'][] = $mark;
            }
        }

        return $parsed;
    }

    /**
     * Parse custom block
     */
    private function parseCustomBlock(array $block): array
    {
        $type = $block['_type'];
        $definition = $this->blockTypes[$type];

        $parsed = [
            '_type' => $type,
            '_key' => $block['_key'] ?? $this->generateKey(),
        ];

        // Validate and copy defined fields
        foreach ($definition['fields'] ?? [] as $fieldName => $fieldConfig) {
            if (isset($block[$fieldName])) {
                $parsed[$fieldName] = $block[$fieldName];
            } elseif ($fieldConfig['required'] ?? false) {
                throw new InvalidArgumentException("Required field '{$fieldName}' missing in {$type} block");
            }
        }

        return $parsed;
    }

    /**
     * Serialize Portable Text to HTML
     */
    public function toHtml(array $blocks): string
    {
        $html = '';
        $listStack = [];

        foreach ($blocks as $index => $block) {
            // Handle list items
            if (isset($block['listItem'])) {
                $listType = $block['listItem'];
                $level = $block['level'] ?? 1;
                $tag = $listType === 'number' ? 'ol' : 'ul';

                // Open new lists as needed
                while (count($listStack) < $level) {
                    $html .= "<{$tag}>";
                    $listStack[] = $tag;
                }

                // Close lists if going back levels
                while (count($listStack) > $level) {
                    $closingTag = array_pop($listStack);
                    $html .= "</{$closingTag}>";
                }

                $html .= '<li>' . $this->serializeBlock($block, false) . '</li>';

                // Check if next block continues list
                $nextBlock = $blocks[$index + 1] ?? null;
                if (!$nextBlock || !isset($nextBlock['listItem'])) {
                    while (!empty($listStack)) {
                        $closingTag = array_pop($listStack);
                        $html .= "</{$closingTag}>";
                    }
                }

                continue;
            }

            // Close any open lists
            while (!empty($listStack)) {
                $closingTag = array_pop($listStack);
                $html .= "</{$closingTag}>";
            }

            $html .= $this->serializeBlock($block);
        }

        return $html;
    }

    /**
     * Serialize single block to HTML
     */
    private function serializeBlock(array $block, bool $wrapInTag = true): string
    {
        $type = $block['_type'];

        // Use custom serializer if available
        if (isset($this->serializers[$type])) {
            return call_user_func($this->serializers[$type], $block, $this);
        }

        // Handle standard block
        if ($type === 'block') {
            return $this->serializeTextBlock($block, $wrapInTag);
        }

        // Handle built-in block types
        return match ($type) {
            'image' => $this->serializeImageBlock($block),
            'code' => $this->serializeCodeBlock($block),
            default => $this->serializeUnknownBlock($block),
        };
    }

    /**
     * Serialize text block
     */
    private function serializeTextBlock(array $block, bool $wrapInTag = true): string
    {
        $content = $this->serializeChildren($block['children'] ?? [], $block['markDefs'] ?? []);

        if (!$wrapInTag) {
            return $content;
        }

        $tag = match ($block['style'] ?? 'normal') {
            'h1' => 'h1',
            'h2' => 'h2',
            'h3' => 'h3',
            'h4' => 'h4',
            'h5' => 'h5',
            'h6' => 'h6',
            'blockquote' => 'blockquote',
            default => 'p',
        };

        return "<{$tag}>{$content}</{$tag}>";
    }

    /**
     * Serialize children (spans)
     */
    private function serializeChildren(array $children, array $markDefs): string
    {
        $html = '';

        foreach ($children as $child) {
            if ($child['_type'] === 'span') {
                $html .= $this->serializeSpan($child, $markDefs);
            } else {
                // Inline object
                $html .= $this->serializeInlineObject($child);
            }
        }

        return $html;
    }

    /**
     * Serialize span
     */
    private function serializeSpan(array $span, array $markDefs): string
    {
        $text = htmlspecialchars($span['text'] ?? '', ENT_QUOTES, 'UTF-8');

        // Apply marks in reverse order (innermost first)
        $marks = array_reverse($span['marks'] ?? []);

        foreach ($marks as $mark) {
            // Check if it's a decorator
            if (in_array($mark, self::DEFAULT_DECORATORS, true)) {
                $tag = match ($mark) {
                    'strong' => 'strong',
                    'em' => 'em',
                    'underline' => 'u',
                    'strike' => 's',
                    'code' => 'code',
                    'sup' => 'sup',
                    'sub' => 'sub',
                    default => 'span',
                };
                $text = "<{$tag}>{$text}</{$tag}>";
                continue;
            }

            // Check if it's an annotation
            $markDef = collect($markDefs)->firstWhere('_key', $mark);
            if ($markDef) {
                $text = $this->serializeAnnotation($markDef, $text);
            }
        }

        return $text;
    }

    /**
     * Serialize annotation
     */
    private function serializeAnnotation(array $markDef, string $text): string
    {
        $type = $markDef['_type'];

        // Use custom annotation serializer if available
        if (isset($this->annotationTypes[$type]['serializer'])) {
            return call_user_func($this->annotationTypes[$type]['serializer'], $markDef, $text);
        }

        return match ($type) {
            'link' => sprintf(
                '<a href="%s"%s>%s</a>',
                htmlspecialchars($markDef['href'] ?? '#', ENT_QUOTES),
                !empty($markDef['blank']) ? ' target="_blank" rel="noopener noreferrer"' : '',
                $text
            ),
            'internalLink' => sprintf(
                '<a href="/document/%s" data-reference="%s">%s</a>',
                htmlspecialchars($markDef['reference']['_ref'] ?? '', ENT_QUOTES),
                htmlspecialchars($markDef['reference']['_ref'] ?? '', ENT_QUOTES),
                $text
            ),
            default => $text,
        };
    }

    /**
     * Serialize inline object
     */
    private function serializeInlineObject(array $object): string
    {
        $type = $object['_type'];

        if (isset($this->serializers[$type])) {
            return call_user_func($this->serializers[$type], $object, $this);
        }

        return '';
    }

    /**
     * Serialize image block
     */
    private function serializeImageBlock(array $block): string
    {
        $alt = htmlspecialchars($block['alt'] ?? '', ENT_QUOTES);
        $caption = $block['caption'] ?? null;
        $asset = $block['asset'] ?? [];

        // Build image URL
        $url = $asset['url'] ?? $asset['_ref'] ?? '';
        if (isset($asset['_ref']) && !str_starts_with($asset['_ref'], 'http')) {
            // Resolve asset reference
            $url = "/api/media/{$asset['_ref']}";
        }

        $img = sprintf('<img src="%s" alt="%s" loading="lazy">', htmlspecialchars($url, ENT_QUOTES), $alt);

        if ($caption) {
            return sprintf(
                '<figure>%s<figcaption>%s</figcaption></figure>',
                $img,
                htmlspecialchars($caption, ENT_QUOTES)
            );
        }

        return $img;
    }

    /**
     * Serialize code block
     */
    private function serializeCodeBlock(array $block): string
    {
        $code = htmlspecialchars($block['code'] ?? '', ENT_QUOTES);
        $language = htmlspecialchars($block['language'] ?? 'text', ENT_QUOTES);
        $filename = $block['filename'] ?? null;
        $highlightedLines = $block['highlightedLines'] ?? [];

        $html = "<pre><code class=\"language-{$language}\"";

        if (!empty($highlightedLines)) {
            $html .= sprintf(' data-highlight-lines="%s"', implode(',', $highlightedLines));
        }

        $html .= ">{$code}</code></pre>";

        if ($filename) {
            $html = "<div class=\"code-block\"><div class=\"code-filename\">{$filename}</div>{$html}</div>";
        }

        return $html;
    }

    /**
     * Serialize unknown block type
     */
    private function serializeUnknownBlock(array $block): string
    {
        return sprintf(
            '<!-- Unknown block type: %s -->',
            htmlspecialchars($block['_type'], ENT_QUOTES)
        );
    }

    /**
     * Extract plain text from Portable Text
     */
    public function toPlainText(array $blocks): string
    {
        $text = '';

        foreach ($blocks as $block) {
            if ($block['_type'] === 'block') {
                foreach ($block['children'] ?? [] as $child) {
                    if ($child['_type'] === 'span') {
                        $text .= $child['text'] ?? '';
                    }
                }
                $text .= "\n";
            }
        }

        return trim($text);
    }

    /**
     * Calculate word count
     */
    public function wordCount(array $blocks): int
    {
        $text = $this->toPlainText($blocks);
        return str_word_count($text);
    }

    /**
     * Calculate reading time in minutes
     */
    public function readingTime(array $blocks, int $wordsPerMinute = 200): int
    {
        $wordCount = $this->wordCount($blocks);
        return (int) ceil($wordCount / $wordsPerMinute);
    }

    /**
     * Validate Portable Text structure
     */
    public function validate(array $blocks): ValidationResult
    {
        $errors = [];
        $warnings = [];

        foreach ($blocks as $index => $block) {
            $blockErrors = $this->validateBlock($block, $index);
            $errors = array_merge($errors, $blockErrors['errors']);
            $warnings = array_merge($warnings, $blockErrors['warnings']);
        }

        return new ValidationResult(empty($errors), $errors, $warnings);
    }

    /**
     * Validate single block
     */
    private function validateBlock(array $block, int $index): array
    {
        $errors = [];
        $warnings = [];
        $path = "blocks[{$index}]";

        if (!isset($block['_type'])) {
            $errors[] = "{$path}: Missing required field '_type'";
            return ['errors' => $errors, 'warnings' => $warnings];
        }

        if (!isset($block['_key'])) {
            $warnings[] = "{$path}: Missing '_key' (will be auto-generated)";
        }

        if ($block['_type'] === 'block') {
            // Validate text block
            if (!isset($block['children']) || !is_array($block['children'])) {
                $errors[] = "{$path}: Text block must have 'children' array";
            } else {
                foreach ($block['children'] as $childIndex => $child) {
                    if (($child['_type'] ?? 'span') === 'span' && !isset($child['text'])) {
                        $errors[] = "{$path}.children[{$childIndex}]: Span must have 'text'";
                    }
                }
            }

            // Validate marks reference valid markDefs
            $markDefKeys = collect($block['markDefs'] ?? [])->pluck('_key')->toArray();
            foreach ($block['children'] ?? [] as $child) {
                foreach ($child['marks'] ?? [] as $mark) {
                    if (!in_array($mark, self::DEFAULT_DECORATORS, true) &&
                        !in_array($mark, $markDefKeys, true)) {
                        $errors[] = "{$path}: Mark '{$mark}' references non-existent markDef";
                    }
                }
            }
        }

        return ['errors' => $errors, 'warnings' => $warnings];
    }

    /**
     * Register annotation type
     */
    public function registerAnnotation(string $name, array $config): self
    {
        $this->annotationTypes[$name] = $config;
        return $this;
    }

    /**
     * Register block type
     */
    public function registerBlockType(string $name, array $config): self
    {
        $this->blockTypes[$name] = $config;
        return $this;
    }

    /**
     * Register custom serializer
     */
    public function registerSerializer(string $type, callable $serializer): self
    {
        $this->serializers[$type] = $serializer;
        return $this;
    }

    /**
     * Generate unique key
     */
    public function generateKey(): string
    {
        return Str::random(12);
    }

    /**
     * Register default annotations
     */
    private function registerDefaultAnnotations(): void
    {
        $this->annotationTypes = [
            'link' => [
                'title' => 'Link',
                'fields' => [
                    'href' => ['type' => 'url', 'required' => true],
                    'blank' => ['type' => 'boolean', 'default' => false],
                ],
            ],
            'internalLink' => [
                'title' => 'Internal Link',
                'fields' => [
                    'reference' => ['type' => 'reference', 'required' => true],
                ],
            ],
            'footnote' => [
                'title' => 'Footnote',
                'fields' => [
                    'note' => ['type' => 'text', 'required' => true],
                ],
            ],
        ];
    }

    /**
     * Register default block types
     */
    private function registerDefaultBlockTypes(): void
    {
        $this->blockTypes = [
            'image' => [
                'title' => 'Image',
                'fields' => [
                    'asset' => ['type' => 'reference', 'required' => true],
                    'alt' => ['type' => 'string'],
                    'caption' => ['type' => 'string'],
                    'hotspot' => ['type' => 'object'],
                    'crop' => ['type' => 'object'],
                ],
            ],
            'code' => [
                'title' => 'Code Block',
                'fields' => [
                    'code' => ['type' => 'text', 'required' => true],
                    'language' => ['type' => 'string'],
                    'filename' => ['type' => 'string'],
                    'highlightedLines' => ['type' => 'array'],
                ],
            ],
            'callout' => [
                'title' => 'Callout',
                'fields' => [
                    'type' => ['type' => 'string', 'options' => ['info', 'warning', 'error', 'success']],
                    'title' => ['type' => 'string'],
                    'body' => ['type' => 'portableText'],
                ],
            ],
            'embed' => [
                'title' => 'Embed',
                'fields' => [
                    'url' => ['type' => 'url', 'required' => true],
                    'provider' => ['type' => 'string'],
                ],
            ],
            'table' => [
                'title' => 'Table',
                'fields' => [
                    'rows' => ['type' => 'array', 'required' => true],
                ],
            ],
        ];
    }

    /**
     * Register default serializers
     */
    private function registerDefaultSerializers(): void
    {
        // Callout serializer
        $this->serializers['callout'] = function (array $block, self $service): string {
            $type = $block['type'] ?? 'info';
            $title = htmlspecialchars($block['title'] ?? '', ENT_QUOTES);
            $body = isset($block['body']) ? $service->toHtml($block['body']) : '';

            return sprintf(
                '<div class="callout callout-%s">%s%s</div>',
                $type,
                $title ? "<div class=\"callout-title\">{$title}</div>" : '',
                "<div class=\"callout-body\">{$body}</div>"
            );
        };

        // Embed serializer
        $this->serializers['embed'] = function (array $block): string {
            $url = htmlspecialchars($block['url'] ?? '', ENT_QUOTES);
            $provider = $block['provider'] ?? $this->detectProvider($url);

            return match ($provider) {
                'youtube' => $this->serializeYouTubeEmbed($url),
                'vimeo' => $this->serializeVimeoEmbed($url),
                'twitter' => "<blockquote class=\"twitter-tweet\"><a href=\"{$url}\"></a></blockquote>",
                default => "<a href=\"{$url}\" target=\"_blank\" rel=\"noopener\">{$url}</a>",
            };
        };

        // Table serializer
        $this->serializers['table'] = function (array $block): string {
            $html = '<table>';

            foreach ($block['rows'] ?? [] as $rowIndex => $row) {
                $html .= '<tr>';
                foreach ($row['cells'] ?? [] as $cell) {
                    $tag = $rowIndex === 0 ? 'th' : 'td';
                    $content = htmlspecialchars($cell ?? '', ENT_QUOTES);
                    $html .= "<{$tag}>{$content}</{$tag}>";
                }
                $html .= '</tr>';
            }

            $html .= '</table>';
            return $html;
        };
    }

    /**
     * Detect embed provider from URL
     */
    private function detectProvider(string $url): string
    {
        if (str_contains($url, 'youtube.com') || str_contains($url, 'youtu.be')) {
            return 'youtube';
        }
        if (str_contains($url, 'vimeo.com')) {
            return 'vimeo';
        }
        if (str_contains($url, 'twitter.com') || str_contains($url, 'x.com')) {
            return 'twitter';
        }
        return 'unknown';
    }

    /**
     * Serialize YouTube embed
     */
    private function serializeYouTubeEmbed(string $url): string
    {
        preg_match('/(?:youtube\.com\/watch\?v=|youtu\.be\/)([^&\s]+)/', $url, $matches);
        $videoId = $matches[1] ?? '';

        return sprintf(
            '<div class="video-embed"><iframe src="https://www.youtube.com/embed/%s" frameborder="0" allowfullscreen></iframe></div>',
            htmlspecialchars($videoId, ENT_QUOTES)
        );
    }

    /**
     * Serialize Vimeo embed
     */
    private function serializeVimeoEmbed(string $url): string
    {
        preg_match('/vimeo\.com\/(\d+)/', $url, $matches);
        $videoId = $matches[1] ?? '';

        return sprintf(
            '<div class="video-embed"><iframe src="https://player.vimeo.com/video/%s" frameborder="0" allowfullscreen></iframe></div>',
            htmlspecialchars($videoId, ENT_QUOTES)
        );
    }
}

/**
 * Portable Text Document Builder
 */
class PortableTextDocument
{
    private array $blocks = [];

    public function __construct(
        private readonly PortableTextService $service
    ) {}

    /**
     * Add paragraph
     */
    public function paragraph(string $text, array $marks = []): self
    {
        $this->blocks[] = [
            '_type' => 'block',
            '_key' => $this->service->generateKey(),
            'style' => 'normal',
            'children' => [
                [
                    '_type' => 'span',
                    '_key' => $this->service->generateKey(),
                    'text' => $text,
                    'marks' => $marks,
                ],
            ],
            'markDefs' => [],
        ];
        return $this;
    }

    /**
     * Add heading
     */
    public function heading(int $level, string $text): self
    {
        $this->blocks[] = [
            '_type' => 'block',
            '_key' => $this->service->generateKey(),
            'style' => "h{$level}",
            'children' => [
                [
                    '_type' => 'span',
                    '_key' => $this->service->generateKey(),
                    'text' => $text,
                    'marks' => [],
                ],
            ],
            'markDefs' => [],
        ];
        return $this;
    }

    /**
     * Add image
     */
    public function image(string $assetRef, ?string $alt = null, ?string $caption = null): self
    {
        $this->blocks[] = [
            '_type' => 'image',
            '_key' => $this->service->generateKey(),
            'asset' => ['_ref' => $assetRef, '_type' => 'reference'],
            'alt' => $alt,
            'caption' => $caption,
        ];
        return $this;
    }

    /**
     * Add code block
     */
    public function code(string $code, string $language = 'text', ?string $filename = null): self
    {
        $this->blocks[] = [
            '_type' => 'code',
            '_key' => $this->service->generateKey(),
            'code' => $code,
            'language' => $language,
            'filename' => $filename,
        ];
        return $this;
    }

    /**
     * Add blockquote
     */
    public function blockquote(string $text): self
    {
        $this->blocks[] = [
            '_type' => 'block',
            '_key' => $this->service->generateKey(),
            'style' => 'blockquote',
            'children' => [
                [
                    '_type' => 'span',
                    '_key' => $this->service->generateKey(),
                    'text' => $text,
                    'marks' => [],
                ],
            ],
            'markDefs' => [],
        ];
        return $this;
    }

    /**
     * Add bullet list
     */
    public function bulletList(array $items): self
    {
        foreach ($items as $item) {
            $this->blocks[] = [
                '_type' => 'block',
                '_key' => $this->service->generateKey(),
                'style' => 'normal',
                'listItem' => 'bullet',
                'level' => 1,
                'children' => [
                    [
                        '_type' => 'span',
                        '_key' => $this->service->generateKey(),
                        'text' => $item,
                        'marks' => [],
                    ],
                ],
                'markDefs' => [],
            ];
        }
        return $this;
    }

    /**
     * Add numbered list
     */
    public function numberedList(array $items): self
    {
        foreach ($items as $item) {
            $this->blocks[] = [
                '_type' => 'block',
                '_key' => $this->service->generateKey(),
                'style' => 'normal',
                'listItem' => 'number',
                'level' => 1,
                'children' => [
                    [
                        '_type' => 'span',
                        '_key' => $this->service->generateKey(),
                        'text' => $item,
                        'marks' => [],
                    ],
                ],
                'markDefs' => [],
            ];
        }
        return $this;
    }

    /**
     * Add callout
     */
    public function callout(string $type, string $title, string $body): self
    {
        $this->blocks[] = [
            '_type' => 'callout',
            '_key' => $this->service->generateKey(),
            'type' => $type,
            'title' => $title,
            'body' => [
                [
                    '_type' => 'block',
                    '_key' => $this->service->generateKey(),
                    'style' => 'normal',
                    'children' => [
                        [
                            '_type' => 'span',
                            '_key' => $this->service->generateKey(),
                            'text' => $body,
                            'marks' => [],
                        ],
                    ],
                    'markDefs' => [],
                ],
            ],
        ];
        return $this;
    }

    /**
     * Add custom block
     */
    public function customBlock(string $type, array $data): self
    {
        $this->blocks[] = array_merge([
            '_type' => $type,
            '_key' => $this->service->generateKey(),
        ], $data);
        return $this;
    }

    /**
     * Get blocks array
     */
    public function toArray(): array
    {
        return $this->blocks;
    }

    /**
     * Convert to HTML
     */
    public function toHtml(): string
    {
        return $this->service->toHtml($this->blocks);
    }

    /**
     * Convert to plain text
     */
    public function toPlainText(): string
    {
        return $this->service->toPlainText($this->blocks);
    }
}

/**
 * Validation Result
 */
class ValidationResult
{
    public function __construct(
        public readonly bool $isValid,
        public readonly array $errors,
        public readonly array $warnings,
    ) {}

    public function toArray(): array
    {
        return [
            'valid' => $this->isValid,
            'errors' => $this->errors,
            'warnings' => $this->warnings,
        ];
    }
}
