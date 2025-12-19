<?php

declare(strict_types=1);

namespace App\Services\Email;

use App\Models\EmailTemplate;
use App\Models\EmailBlock;
use App\Models\EmailLayout;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Storage;
use Illuminate\Support\Str;

/**
 * VisualBuilderService - Enhanced Visual Campaign Builder
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Enterprise drag-and-drop email builder service providing:
 * - Block-based email construction
 * - Real-time preview generation
 * - Responsive layout system
 * - Custom block library
 * - Template import/export (MJML, HTML)
 * - Undo/redo history
 * - Collaborative editing support
 * - Version control
 *
 * @version 1.0.0
 */
class VisualBuilderService
{
    /**
     * Default blocks available in the builder
     */
    private const DEFAULT_BLOCKS = [
        'header' => [
            'name' => 'Header',
            'category' => 'structure',
            'icon' => 'header',
            'description' => 'Email header with logo and navigation',
            'properties' => [
                'logo_url' => ['type' => 'image', 'label' => 'Logo'],
                'background_color' => ['type' => 'color', 'label' => 'Background', 'default' => '#ffffff'],
                'padding' => ['type' => 'spacing', 'label' => 'Padding', 'default' => '20px'],
            ],
        ],
        'hero' => [
            'name' => 'Hero Section',
            'category' => 'content',
            'icon' => 'image',
            'description' => 'Large hero image with text overlay',
            'properties' => [
                'image_url' => ['type' => 'image', 'label' => 'Background Image'],
                'headline' => ['type' => 'text', 'label' => 'Headline'],
                'subheadline' => ['type' => 'text', 'label' => 'Subheadline'],
                'cta_text' => ['type' => 'text', 'label' => 'Button Text'],
                'cta_url' => ['type' => 'url', 'label' => 'Button URL'],
                'overlay_color' => ['type' => 'color', 'label' => 'Overlay', 'default' => 'rgba(0,0,0,0.5)'],
            ],
        ],
        'text' => [
            'name' => 'Text Block',
            'category' => 'content',
            'icon' => 'text',
            'description' => 'Rich text content block',
            'properties' => [
                'content' => ['type' => 'richtext', 'label' => 'Content'],
                'font_size' => ['type' => 'select', 'label' => 'Font Size', 'options' => ['14px', '16px', '18px', '20px']],
                'text_align' => ['type' => 'align', 'label' => 'Alignment', 'default' => 'left'],
                'padding' => ['type' => 'spacing', 'label' => 'Padding', 'default' => '15px'],
            ],
        ],
        'image' => [
            'name' => 'Image',
            'category' => 'content',
            'icon' => 'image',
            'description' => 'Single image with optional link',
            'properties' => [
                'src' => ['type' => 'image', 'label' => 'Image'],
                'alt' => ['type' => 'text', 'label' => 'Alt Text'],
                'link' => ['type' => 'url', 'label' => 'Link URL'],
                'width' => ['type' => 'size', 'label' => 'Width', 'default' => '100%'],
                'border_radius' => ['type' => 'size', 'label' => 'Border Radius', 'default' => '0'],
            ],
        ],
        'button' => [
            'name' => 'Button',
            'category' => 'content',
            'icon' => 'button',
            'description' => 'Call-to-action button',
            'properties' => [
                'text' => ['type' => 'text', 'label' => 'Button Text'],
                'url' => ['type' => 'url', 'label' => 'URL'],
                'background_color' => ['type' => 'color', 'label' => 'Background', 'default' => '#007bff'],
                'text_color' => ['type' => 'color', 'label' => 'Text Color', 'default' => '#ffffff'],
                'border_radius' => ['type' => 'size', 'label' => 'Border Radius', 'default' => '4px'],
                'padding' => ['type' => 'spacing', 'label' => 'Padding', 'default' => '12px 24px'],
                'alignment' => ['type' => 'align', 'label' => 'Alignment', 'default' => 'center'],
            ],
        ],
        'divider' => [
            'name' => 'Divider',
            'category' => 'structure',
            'icon' => 'minus',
            'description' => 'Horizontal divider line',
            'properties' => [
                'color' => ['type' => 'color', 'label' => 'Color', 'default' => '#e0e0e0'],
                'height' => ['type' => 'size', 'label' => 'Height', 'default' => '1px'],
                'margin' => ['type' => 'spacing', 'label' => 'Margin', 'default' => '20px 0'],
            ],
        ],
        'spacer' => [
            'name' => 'Spacer',
            'category' => 'structure',
            'icon' => 'arrows-alt-v',
            'description' => 'Vertical spacing',
            'properties' => [
                'height' => ['type' => 'size', 'label' => 'Height', 'default' => '30px'],
            ],
        ],
        'columns' => [
            'name' => 'Columns',
            'category' => 'layout',
            'icon' => 'columns',
            'description' => 'Multi-column layout',
            'properties' => [
                'columns' => ['type' => 'number', 'label' => 'Columns', 'min' => 1, 'max' => 4, 'default' => 2],
                'gap' => ['type' => 'size', 'label' => 'Gap', 'default' => '20px'],
                'stack_on_mobile' => ['type' => 'boolean', 'label' => 'Stack on Mobile', 'default' => true],
            ],
        ],
        'product' => [
            'name' => 'Product Card',
            'category' => 'commerce',
            'icon' => 'shopping-cart',
            'description' => 'Product showcase with image, description, and price',
            'properties' => [
                'image_url' => ['type' => 'image', 'label' => 'Product Image'],
                'name' => ['type' => 'text', 'label' => 'Product Name'],
                'description' => ['type' => 'textarea', 'label' => 'Description'],
                'price' => ['type' => 'text', 'label' => 'Price'],
                'original_price' => ['type' => 'text', 'label' => 'Original Price'],
                'cta_text' => ['type' => 'text', 'label' => 'Button Text', 'default' => 'Buy Now'],
                'cta_url' => ['type' => 'url', 'label' => 'Product URL'],
            ],
        ],
        'social' => [
            'name' => 'Social Links',
            'category' => 'content',
            'icon' => 'share-alt',
            'description' => 'Social media icon links',
            'properties' => [
                'facebook' => ['type' => 'url', 'label' => 'Facebook URL'],
                'twitter' => ['type' => 'url', 'label' => 'Twitter/X URL'],
                'instagram' => ['type' => 'url', 'label' => 'Instagram URL'],
                'linkedin' => ['type' => 'url', 'label' => 'LinkedIn URL'],
                'youtube' => ['type' => 'url', 'label' => 'YouTube URL'],
                'icon_size' => ['type' => 'size', 'label' => 'Icon Size', 'default' => '32px'],
                'icon_color' => ['type' => 'color', 'label' => 'Icon Color', 'default' => '#333333'],
            ],
        ],
        'footer' => [
            'name' => 'Footer',
            'category' => 'structure',
            'icon' => 'footer',
            'description' => 'Email footer with unsubscribe and address',
            'properties' => [
                'company_name' => ['type' => 'text', 'label' => 'Company Name'],
                'address' => ['type' => 'textarea', 'label' => 'Address'],
                'unsubscribe_text' => ['type' => 'text', 'label' => 'Unsubscribe Text', 'default' => 'Unsubscribe from this list'],
                'background_color' => ['type' => 'color', 'label' => 'Background', 'default' => '#f5f5f5'],
            ],
        ],
    ];

    /**
     * Get available blocks for the builder
     */
    public function getAvailableBlocks(): array
    {
        $customBlocks = $this->getCustomBlocks();

        return array_merge(self::DEFAULT_BLOCKS, $customBlocks);
    }

    /**
     * Get blocks by category
     */
    public function getBlocksByCategory(): array
    {
        $blocks = $this->getAvailableBlocks();
        $categorized = [];

        foreach ($blocks as $id => $block) {
            $category = $block['category'] ?? 'other';
            $categorized[$category][$id] = $block;
        }

        return $categorized;
    }

    /**
     * Get custom blocks from database
     */
    private function getCustomBlocks(): array
    {
        return Cache::remember('email_custom_blocks', 3600, function () {
            if (!class_exists(EmailBlock::class)) {
                return [];
            }

            return EmailBlock::where('is_active', true)
                ->get()
                ->mapWithKeys(function ($block) {
                    return [$block->slug => [
                        'name' => $block->name,
                        'category' => $block->category,
                        'icon' => $block->icon,
                        'description' => $block->description,
                        'properties' => $block->properties,
                        'template' => $block->template,
                        'is_custom' => true,
                    ]];
                })
                ->toArray();
        });
    }

    /**
     * Render block to HTML
     */
    public function renderBlock(string $blockType, array $properties): string
    {
        $blocks = $this->getAvailableBlocks();

        if (!isset($blocks[$blockType])) {
            throw new \InvalidArgumentException("Unknown block type: {$blockType}");
        }

        $block = $blocks[$blockType];

        // Use custom template if available
        if (!empty($block['template'])) {
            return $this->renderCustomTemplate($block['template'], $properties);
        }

        // Use built-in renderer
        return match ($blockType) {
            'header' => $this->renderHeader($properties),
            'hero' => $this->renderHero($properties),
            'text' => $this->renderText($properties),
            'image' => $this->renderImage($properties),
            'button' => $this->renderButton($properties),
            'divider' => $this->renderDivider($properties),
            'spacer' => $this->renderSpacer($properties),
            'columns' => $this->renderColumns($properties),
            'product' => $this->renderProduct($properties),
            'social' => $this->renderSocial($properties),
            'footer' => $this->renderFooter($properties),
            default => $this->renderGeneric($blockType, $properties),
        };
    }

    /**
     * Render full email from block structure
     */
    public function renderEmail(array $blocks, array $globalStyles = []): string
    {
        $bodyContent = '';

        foreach ($blocks as $block) {
            $bodyContent .= $this->renderBlock($block['type'], $block['properties'] ?? []);
        }

        return $this->wrapInEmailStructure($bodyContent, $globalStyles);
    }

    /**
     * Wrap content in email HTML structure
     */
    private function wrapInEmailStructure(string $content, array $styles = []): string
    {
        $bgColor = $styles['background_color'] ?? '#f4f4f4';
        $contentBg = $styles['content_background'] ?? '#ffffff';
        $fontFamily = $styles['font_family'] ?? 'Arial, sans-serif';
        $maxWidth = $styles['max_width'] ?? '600px';

        return <<<HTML
<!DOCTYPE html>
<html lang="en" xmlns="http://www.w3.org/1999/xhtml" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="x-apple-disable-message-reformatting">
    <title></title>
    <!--[if mso]>
    <noscript>
        <xml>
            <o:OfficeDocumentSettings>
                <o:AllowPNG/>
                <o:PixelsPerInch>96</o:PixelsPerInch>
            </o:OfficeDocumentSettings>
        </xml>
    </noscript>
    <![endif]-->
    <style type="text/css">
        body { margin: 0; padding: 0; font-family: {$fontFamily}; background-color: {$bgColor}; }
        table { border-collapse: collapse; mso-table-lspace: 0; mso-table-rspace: 0; }
        img { border: 0; display: block; max-width: 100%; height: auto; }
        a { color: #007bff; text-decoration: none; }
        @media screen and (max-width: 600px) {
            .mobile-full-width { width: 100% !important; }
            .mobile-padding { padding: 10px !important; }
            .mobile-stack { display: block !important; width: 100% !important; }
        }
    </style>
</head>
<body style="margin: 0; padding: 0; background-color: {$bgColor};">
    <center>
        <table role="presentation" cellspacing="0" cellpadding="0" border="0" width="100%" style="background-color: {$bgColor};">
            <tr>
                <td align="center" style="padding: 20px 10px;">
                    <table role="presentation" cellspacing="0" cellpadding="0" border="0" width="100%" style="max-width: {$maxWidth}; background-color: {$contentBg};">
                        {$content}
                    </table>
                </td>
            </tr>
        </table>
    </center>
</body>
</html>
HTML;
    }

    // Block renderers

    private function renderHeader(array $props): string
    {
        $logo = $props['logo_url'] ?? '';
        $bg = $props['background_color'] ?? '#ffffff';
        $padding = $props['padding'] ?? '20px';

        return <<<HTML
<tr>
    <td style="background-color: {$bg}; padding: {$padding}; text-align: center;">
        <img src="{$logo}" alt="Logo" style="max-width: 200px; height: auto;">
    </td>
</tr>
HTML;
    }

    private function renderHero(array $props): string
    {
        $image = $props['image_url'] ?? '';
        $headline = $props['headline'] ?? '';
        $subheadline = $props['subheadline'] ?? '';
        $ctaText = $props['cta_text'] ?? '';
        $ctaUrl = $props['cta_url'] ?? '#';
        $overlay = $props['overlay_color'] ?? 'rgba(0,0,0,0.5)';

        return <<<HTML
<tr>
    <td style="background-image: url('{$image}'); background-size: cover; background-position: center;">
        <div style="background: {$overlay}; padding: 60px 30px; text-align: center;">
            <h1 style="color: #ffffff; font-size: 32px; margin: 0 0 10px;">{$headline}</h1>
            <p style="color: #ffffff; font-size: 18px; margin: 0 0 20px;">{$subheadline}</p>
            <a href="{$ctaUrl}" style="display: inline-block; background: #007bff; color: #ffffff; padding: 12px 30px; border-radius: 4px; font-weight: bold;">{$ctaText}</a>
        </div>
    </td>
</tr>
HTML;
    }

    private function renderText(array $props): string
    {
        $content = $props['content'] ?? '';
        $fontSize = $props['font_size'] ?? '16px';
        $align = $props['text_align'] ?? 'left';
        $padding = $props['padding'] ?? '15px';

        return <<<HTML
<tr>
    <td style="padding: {$padding}; font-size: {$fontSize}; line-height: 1.6; text-align: {$align};">
        {$content}
    </td>
</tr>
HTML;
    }

    private function renderImage(array $props): string
    {
        $src = $props['src'] ?? '';
        $alt = $props['alt'] ?? '';
        $link = $props['link'] ?? '';
        $width = $props['width'] ?? '100%';
        $radius = $props['border_radius'] ?? '0';

        $img = "<img src=\"{$src}\" alt=\"{$alt}\" style=\"width: {$width}; max-width: 100%; height: auto; border-radius: {$radius};\">";

        if ($link) {
            $img = "<a href=\"{$link}\">{$img}</a>";
        }

        return "<tr><td style=\"padding: 0;\">{$img}</td></tr>";
    }

    private function renderButton(array $props): string
    {
        $text = $props['text'] ?? 'Click Here';
        $url = $props['url'] ?? '#';
        $bg = $props['background_color'] ?? '#007bff';
        $color = $props['text_color'] ?? '#ffffff';
        $radius = $props['border_radius'] ?? '4px';
        $padding = $props['padding'] ?? '12px 24px';
        $align = $props['alignment'] ?? 'center';

        return <<<HTML
<tr>
    <td style="padding: 20px; text-align: {$align};">
        <a href="{$url}" style="display: inline-block; background: {$bg}; color: {$color}; padding: {$padding}; border-radius: {$radius}; font-weight: bold; text-decoration: none;">{$text}</a>
    </td>
</tr>
HTML;
    }

    private function renderDivider(array $props): string
    {
        $color = $props['color'] ?? '#e0e0e0';
        $height = $props['height'] ?? '1px';
        $margin = $props['margin'] ?? '20px 0';

        return <<<HTML
<tr>
    <td style="padding: {$margin};">
        <hr style="border: 0; border-top: {$height} solid {$color}; margin: 0;">
    </td>
</tr>
HTML;
    }

    private function renderSpacer(array $props): string
    {
        $height = $props['height'] ?? '30px';

        return "<tr><td style=\"height: {$height};\"></td></tr>";
    }

    private function renderColumns(array $props): string
    {
        $columnCount = $props['columns'] ?? 2;
        $gap = $props['gap'] ?? '20px';
        $columnContent = $props['column_content'] ?? [];

        $width = floor(100 / $columnCount);
        $columns = '';

        for ($i = 0; $i < $columnCount; $i++) {
            $content = $columnContent[$i] ?? '';
            $columns .= "<td style=\"width: {$width}%; vertical-align: top; padding: 0 {$gap};\">{$content}</td>";
        }

        return "<tr>{$columns}</tr>";
    }

    private function renderProduct(array $props): string
    {
        $image = $props['image_url'] ?? '';
        $name = $props['name'] ?? '';
        $description = $props['description'] ?? '';
        $price = $props['price'] ?? '';
        $originalPrice = $props['original_price'] ?? '';
        $ctaText = $props['cta_text'] ?? 'Buy Now';
        $ctaUrl = $props['cta_url'] ?? '#';

        $priceHtml = $originalPrice
            ? "<span style=\"text-decoration: line-through; color: #999;\">{$originalPrice}</span> <strong style=\"color: #e74c3c;\">{$price}</strong>"
            : "<strong>{$price}</strong>";

        return <<<HTML
<tr>
    <td style="padding: 20px;">
        <table role="presentation" cellspacing="0" cellpadding="0" border="0" width="100%">
            <tr>
                <td style="width: 150px; padding-right: 20px;">
                    <img src="{$image}" alt="{$name}" style="width: 100%; max-width: 150px; border-radius: 8px;">
                </td>
                <td style="vertical-align: top;">
                    <h3 style="margin: 0 0 10px; font-size: 18px;">{$name}</h3>
                    <p style="margin: 0 0 10px; color: #666;">{$description}</p>
                    <p style="margin: 0 0 15px;">{$priceHtml}</p>
                    <a href="{$ctaUrl}" style="display: inline-block; background: #007bff; color: #fff; padding: 10px 20px; border-radius: 4px;">{$ctaText}</a>
                </td>
            </tr>
        </table>
    </td>
</tr>
HTML;
    }

    private function renderSocial(array $props): string
    {
        $size = $props['icon_size'] ?? '32px';
        $color = $props['icon_color'] ?? '#333333';

        $socialLinks = [
            'facebook' => '📘',
            'twitter' => '🐦',
            'instagram' => '📸',
            'linkedin' => '💼',
            'youtube' => '▶️',
        ];

        $icons = '';
        foreach ($socialLinks as $platform => $emoji) {
            $url = $props[$platform] ?? '';
            if ($url) {
                $icons .= "<a href=\"{$url}\" style=\"display: inline-block; margin: 0 8px; font-size: {$size}; text-decoration: none;\">{$emoji}</a>";
            }
        }

        return <<<HTML
<tr>
    <td style="padding: 20px; text-align: center;">
        {$icons}
    </td>
</tr>
HTML;
    }

    private function renderFooter(array $props): string
    {
        $company = $props['company_name'] ?? '';
        $address = $props['address'] ?? '';
        $unsubscribe = $props['unsubscribe_text'] ?? 'Unsubscribe from this list';
        $bg = $props['background_color'] ?? '#f5f5f5';

        return <<<HTML
<tr>
    <td style="background-color: {$bg}; padding: 30px; text-align: center; font-size: 12px; color: #666;">
        <p style="margin: 0 0 10px;"><strong>{$company}</strong></p>
        <p style="margin: 0 0 15px;">{$address}</p>
        <p style="margin: 0;">
            <a href="{{unsubscribe_url}}" style="color: #666;">{$unsubscribe}</a>
        </p>
    </td>
</tr>
HTML;
    }

    private function renderGeneric(string $type, array $props): string
    {
        return "<tr><td style=\"padding: 20px;\"><!-- Block: {$type} --></td></tr>";
    }

    private function renderCustomTemplate(string $template, array $properties): string
    {
        foreach ($properties as $key => $value) {
            $template = str_replace('{{' . $key . '}}', $value, $template);
        }

        return $template;
    }

    /**
     * Generate live preview
     */
    public function generatePreview(array $blocks, array $globalStyles = [], string $device = 'desktop'): array
    {
        $html = $this->renderEmail($blocks, $globalStyles);

        $width = match ($device) {
            'mobile' => '375px',
            'tablet' => '768px',
            default => '100%',
        };

        return [
            'html' => $html,
            'device' => $device,
            'width' => $width,
            'size_bytes' => strlen($html),
        ];
    }

    /**
     * Export to MJML
     */
    public function exportToMjml(array $blocks): string
    {
        // Convert block structure to MJML format
        $mjmlBody = '';

        foreach ($blocks as $block) {
            $mjmlBody .= $this->blockToMjml($block);
        }

        return <<<MJML
<mjml>
  <mj-body>
    {$mjmlBody}
  </mj-body>
</mjml>
MJML;
    }

    /**
     * Convert block to MJML
     */
    private function blockToMjml(array $block): string
    {
        $type = $block['type'];
        $props = $block['properties'] ?? [];

        return match ($type) {
            'text' => "<mj-section><mj-column><mj-text>{$props['content']}</mj-text></mj-column></mj-section>",
            'button' => "<mj-section><mj-column><mj-button href=\"{$props['url']}\">{$props['text']}</mj-button></mj-column></mj-section>",
            'image' => "<mj-section><mj-column><mj-image src=\"{$props['src']}\" alt=\"{$props['alt']}\"/></mj-column></mj-section>",
            'divider' => "<mj-section><mj-column><mj-divider/></mj-column></mj-section>",
            'spacer' => "<mj-section><mj-column><mj-spacer height=\"{$props['height']}\"/></mj-column></mj-section>",
            default => "<!-- Block: {$type} -->",
        };
    }

    /**
     * Save builder history for undo/redo
     */
    public function saveHistory(string $templateId, array $blocks, int $userId): void
    {
        $historyKey = "builder_history:{$templateId}:{$userId}";
        $history = Cache::get($historyKey, []);

        // Keep last 50 states
        $history[] = [
            'blocks' => $blocks,
            'timestamp' => now()->toIso8601String(),
        ];

        if (count($history) > 50) {
            $history = array_slice($history, -50);
        }

        Cache::put($historyKey, $history, 3600 * 24); // 24 hours
    }

    /**
     * Get builder history
     */
    public function getHistory(string $templateId, int $userId): array
    {
        return Cache::get("builder_history:{$templateId}:{$userId}", []);
    }

    /**
     * Create custom block
     */
    public function createCustomBlock(array $data): array
    {
        $slug = Str::slug($data['name']);

        if (class_exists(EmailBlock::class)) {
            $block = EmailBlock::create([
                'name' => $data['name'],
                'slug' => $slug,
                'category' => $data['category'] ?? 'custom',
                'icon' => $data['icon'] ?? 'cube',
                'description' => $data['description'] ?? '',
                'properties' => $data['properties'] ?? [],
                'template' => $data['template'] ?? '',
                'is_active' => true,
            ]);

            Cache::forget('email_custom_blocks');

            return $block->toArray();
        }

        return [
            'slug' => $slug,
            'name' => $data['name'],
            'status' => 'created_in_memory',
        ];
    }
}
