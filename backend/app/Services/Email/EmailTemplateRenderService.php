<?php

namespace App\Services\Email;

use App\Models\EmailTemplate;
use App\Models\EmailLayout;
use App\Models\EmailBlock;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Blade;
use TijsVerkoyen\CssToInlineStyles\CssToInlineStyles;

/**
 * Email Template Rendering Service
 * RevolutionEmailTemplates-L8-System
 * 
 * Renders email templates with blocks, variables, and inline CSS
 */
class EmailTemplateRenderService
{
    private VariableResolverService $variableResolver;
    private CssToInlineStyles $cssInliner;

    public function __construct(VariableResolverService $variableResolver)
    {
        $this->variableResolver = $variableResolver;
        $this->cssInliner = new CssToInlineStyles();
    }

    /**
     * Render complete email template
     */
    public function render(EmailTemplate $template, array $context = []): array
    {
        $cacheKey = "email_render:{$template->id}:" . md5(json_encode($context));
        
        return Cache::remember($cacheKey, 300, function () use ($template, $context) {
            // Load layout
            $layout = $template->layout ?? EmailLayout::default()->first();
            
            // Render blocks
            $blocksHtml = $this->renderBlocks($template->blocks ?? [], $context);
            
            // Compile HTML
            $html = $this->compileHtml($layout, $blocksHtml, $template->global_styles ?? []);
            
            // Resolve variables in subject
            $subject = $this->variableResolver->resolve($template->subject_template ?? $template->subject, $context);
            
            // Resolve variables in preheader
            $preheader = $this->variableResolver->resolve($template->preheader_template ?? '', $context);
            
            // Inline CSS
            $html = $this->inlineCss($html);
            
            return [
                'subject' => $subject,
                'preheader' => $preheader,
                'html' => $html,
                'text' => $this->htmlToText($html),
            ];
        });
    }

    /**
     * Render blocks recursively
     */
    private function renderBlocks(array $blockIds, array $context): string
    {
        if (empty($blockIds)) {
            return '';
        }

        $blocks = EmailBlock::whereIn('id', $blockIds)->ordered()->get();
        $html = '';

        foreach ($blocks as $block) {
            $html .= $this->renderBlock($block, $context);
        }

        return $html;
    }

    /**
     * Render single block
     */
    private function renderBlock(EmailBlock $block, array $context): string
    {
        // Check conditional rules
        if (!$this->evaluateConditionalRules($block->conditional_rules, $context)) {
            return '';
        }

        $method = 'render' . ucfirst($block->block_type) . 'Block';
        
        if (method_exists($this, $method)) {
            return $this->$method($block, $context);
        }

        return '';
    }

    /**
     * Render TEXT block
     */
    private function renderTextBlock(EmailBlock $block, array $context): string
    {
        $content = $block->content['html'] ?? '';
        $content = $this->variableResolver->resolve($content, $context);
        
        $styles = $this->compileStyles($block->styles ?? []);
        
        return "<div style=\"{$styles}\">{$content}</div>";
    }

    /**
     * Render IMAGE block
     */
    private function renderImageBlock(EmailBlock $block, array $context): string
    {
        $src = $this->variableResolver->resolve($block->content['src'] ?? '', $context);
        $alt = $this->variableResolver->resolve($block->content['alt'] ?? '', $context);
        $link = $this->variableResolver->resolve($block->content['link'] ?? '', $context);
        $width = $block->content['width'] ?? 'auto';
        $height = $block->content['height'] ?? 'auto';
        
        $styles = $this->compileStyles($block->styles ?? []);
        
        $img = "<img src=\"{$src}\" alt=\"{$alt}\" width=\"{$width}\" height=\"{$height}\" style=\"{$styles}\" />";
        
        if ($link) {
            return "<a href=\"{$link}\">{$img}</a>";
        }
        
        return $img;
    }

    /**
     * Render BUTTON block
     */
    private function renderButtonBlock(EmailBlock $block, array $context): string
    {
        $text = $this->variableResolver->resolve($block->content['text'] ?? '', $context);
        $url = $this->variableResolver->resolve($block->content['url'] ?? '', $context);
        
        $styles = $this->compileStyles($block->styles ?? []);
        
        return <<<HTML
        <table border="0" cellpadding="0" cellspacing="0" role="presentation" style="margin: 0 auto;">
            <tr>
                <td style="{$styles}">
                    <a href="{$url}" style="display: inline-block; text-decoration: none; color: inherit;">{$text}</a>
                </td>
            </tr>
        </table>
        HTML;
    }

    /**
     * Render DIVIDER block
     */
    private function renderDividerBlock(EmailBlock $block, array $context): string
    {
        $styles = $this->compileStyles($block->styles ?? [
            'border-top' => '1px solid #e0e0e0',
            'margin' => '20px 0',
        ]);
        
        return "<hr style=\"{$styles}\" />";
    }

    /**
     * Render SPACER block
     */
    private function renderSpacerBlock(EmailBlock $block, array $context): string
    {
        $height = $block->content['height'] ?? '20px';
        $backgroundColor = $block->styles['backgroundColor'] ?? 'transparent';
        
        return "<div style=\"height: {$height}; background-color: {$backgroundColor};\"></div>";
    }

    /**
     * Render COLUMNS block
     */
    private function renderColumnsBlock(EmailBlock $block, array $context): string
    {
        $columns = $block->content['columns'] ?? [];
        $gap = $block->styles['gap'] ?? '20px';
        
        $html = '<table border="0" cellpadding="0" cellspacing="0" width="100%" role="presentation"><tr>';
        
        foreach ($columns as $column) {
            $width = $column['width'] ?? '50%';
            $columnBlocks = $this->renderBlocks($column['blocks'] ?? [], $context);
            
            $html .= "<td width=\"{$width}\" style=\"padding: 0 {$gap};\">{$columnBlocks}</td>";
        }
        
        $html .= '</tr></table>';
        
        return $html;
    }

    /**
     * Render PRODUCT block
     */
    private function renderProductBlock(EmailBlock $block, array $context): string
    {
        $image = $this->variableResolver->resolve($block->content['image'] ?? '', $context);
        $name = $this->variableResolver->resolve($block->content['name'] ?? '', $context);
        $price = $this->variableResolver->resolve($block->content['price'] ?? '', $context);
        $description = $this->variableResolver->resolve($block->content['description'] ?? '', $context);
        $buttonText = $this->variableResolver->resolve($block->content['button_text'] ?? 'View Product', $context);
        $buttonUrl = $this->variableResolver->resolve($block->content['button_url'] ?? '', $context);
        
        $styles = $this->compileStyles($block->styles ?? []);
        
        return <<<HTML
        <table border="0" cellpadding="0" cellspacing="0" width="100%" style="{$styles}">
            <tr>
                <td align="center">
                    <img src="{$image}" alt="{$name}" style="max-width: 100%; height: auto;" />
                </td>
            </tr>
            <tr>
                <td style="padding: 15px;">
                    <h3 style="margin: 0 0 10px;">{$name}</h3>
                    <p style="font-size: 24px; font-weight: bold; margin: 0 0 10px;">{$price}</p>
                    <p style="margin: 0 0 15px;">{$description}</p>
                    <a href="{$buttonUrl}" style="display: inline-block; padding: 12px 30px; background-color: #007bff; color: #ffffff; text-decoration: none; border-radius: 4px;">{$buttonText}</a>
                </td>
            </tr>
        </table>
        HTML;
    }

    /**
     * Render SOCIAL block
     */
    private function renderSocialBlock(EmailBlock $block, array $context): string
    {
        $links = $block->content['links'] ?? [];
        $iconSize = $block->content['icon_size'] ?? '32px';
        $spacing = $block->styles['spacing'] ?? '10px';
        
        $html = '<table border="0" cellpadding="0" cellspacing="0" role="presentation" style="margin: 0 auto;"><tr>';
        
        foreach ($links as $platform => $url) {
            if ($url) {
                $html .= "<td style=\"padding: 0 {$spacing};\"><a href=\"{$url}\"><img src=\"/images/social/{$platform}.png\" alt=\"{$platform}\" width=\"{$iconSize}\" height=\"{$iconSize}\" /></a></td>";
            }
        }
        
        $html .= '</tr></table>';
        
        return $html;
    }

    /**
     * Compile HTML with layout
     */
    private function compileHtml(EmailLayout $layout, string $content, array $globalStyles): string
    {
        $html = $layout->html_structure;
        $html = str_replace('{{content}}', $content, $html);
        
        // Add global styles
        $styleTag = '<style>' . $this->compileGlobalStyles($globalStyles) . '</style>';
        $html = str_replace('</head>', $styleTag . '</head>', $html);
        
        return $html;
    }

    /**
     * Compile global styles to CSS
     */
    private function compileGlobalStyles(array $styles): string
    {
        $css = '';
        
        foreach ($styles as $selector => $rules) {
            $css .= "{$selector} {";
            foreach ($rules as $property => $value) {
                $css .= "{$property}: {$value};";
            }
            $css .= "}";
        }
        
        return $css;
    }

    /**
     * Compile styles array to inline CSS string
     */
    private function compileStyles(array $styles): string
    {
        $css = '';
        
        foreach ($styles as $property => $value) {
            $css .= "{$property}: {$value}; ";
        }
        
        return trim($css);
    }

    /**
     * Inline CSS for email client compatibility
     */
    private function inlineCss(string $html): string
    {
        try {
            return $this->cssInliner->convert($html);
        } catch (\Exception $e) {
            return $html;
        }
    }

    /**
     * Convert HTML to plain text
     */
    private function htmlToText(string $html): string
    {
        return strip_tags($html);
    }

    /**
     * Evaluate conditional rules
     */
    private function evaluateConditionalRules(?array $rules, array $context): bool
    {
        if (empty($rules)) {
            return true;
        }

        foreach ($rules as $rule) {
            $variable = $rule['variable'] ?? '';
            $operator = $rule['operator'] ?? '==';
            $value = $rule['value'] ?? '';
            
            $actualValue = $this->variableResolver->resolveVariable($variable, $context);
            
            if (!$this->compareValues($actualValue, $operator, $value)) {
                return false;
            }
        }

        return true;
    }

    /**
     * Compare values based on operator
     */
    private function compareValues($actual, string $operator, $expected): bool
    {
        return match($operator) {
            '==' => $actual == $expected,
            '!=' => $actual != $expected,
            '>' => $actual > $expected,
            '<' => $actual < $expected,
            '>=' => $actual >= $expected,
            '<=' => $actual <= $expected,
            'contains' => str_contains((string)$actual, (string)$expected),
            'not_contains' => !str_contains((string)$actual, (string)$expected),
            default => false,
        };
    }
}
