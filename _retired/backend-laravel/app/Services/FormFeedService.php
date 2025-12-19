<?php

declare(strict_types=1);

namespace App\Services;

use App\Models\Form;
use Illuminate\Support\Facades\URL;
use Illuminate\Support\Str;

/**
 * Form Feed Service - Embeds, oEmbed, and sharing
 *
 * Features:
 * - Embed code generation (iframe, JavaScript)
 * - oEmbed provider support
 * - QR code generation
 * - Shareable links
 * - Popup/modal embed
 * - Responsive embed options
 * - Custom branding options
 * - Usage tracking
 *
 * @version 1.0.0
 */
class FormFeedService
{
    /**
     * Embed types
     */
    private const EMBED_TYPES = [
        'iframe' => 'Standard iframe embed',
        'javascript' => 'JavaScript widget embed',
        'popup' => 'Popup/modal trigger',
        'inline' => 'Inline injection',
        'amp' => 'AMP-compatible embed',
    ];

    /**
     * Default embed options
     */
    private const DEFAULT_OPTIONS = [
        'width' => '100%',
        'height' => 'auto',
        'border' => false,
        'shadow' => true,
        'rounded' => true,
        'branding' => true,
        'theme' => 'light',
        'responsive' => true,
        'lazy_load' => true,
    ];

    /**
     * Generate embed code for a form
     */
    public function generateEmbed(Form $form, string $type = 'iframe', array $options = []): array
    {
        $options = array_merge(self::DEFAULT_OPTIONS, $options);
        $embedId = $this->generateEmbedId($form);

        $embed = match ($type) {
            'javascript' => $this->generateJavaScriptEmbed($form, $embedId, $options),
            'popup' => $this->generatePopupEmbed($form, $embedId, $options),
            'inline' => $this->generateInlineEmbed($form, $embedId, $options),
            'amp' => $this->generateAmpEmbed($form, $options),
            default => $this->generateIframeEmbed($form, $options),
        };

        return [
            'type' => $type,
            'code' => $embed['code'],
            'preview_url' => $this->getFormUrl($form),
            'embed_id' => $embedId,
            'options' => $options,
            'instructions' => $embed['instructions'] ?? null,
        ];
    }

    /**
     * Generate iframe embed
     */
    private function generateIframeEmbed(Form $form, array $options): array
    {
        $url = $this->getFormUrl($form, ['embed' => 'iframe']);
        $width = $options['width'];
        $height = $options['height'] === 'auto' ? '600' : $options['height'];

        $styles = [];
        if (!$options['border']) {
            $styles[] = 'border: none';
        }
        if ($options['shadow']) {
            $styles[] = 'box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1)';
        }
        if ($options['rounded']) {
            $styles[] = 'border-radius: 8px';
        }

        $styleAttr = !empty($styles) ? ' style="' . implode('; ', $styles) . '"' : '';

        $code = <<<HTML
<iframe
    src="{$url}"
    width="{$width}"
    height="{$height}"
    frameborder="0"
    allowfullscreen
    loading="{$this->getLazyLoad($options)}"
    title="{$this->escapeHtml($form->title)}"
    {$styleAttr}
></iframe>
HTML;

        if ($options['responsive']) {
            $code = <<<HTML
<div style="position: relative; padding-bottom: 75%; height: 0; overflow: hidden; max-width: 100%;">
    <iframe
        src="{$url}"
        style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; border: none;{$options['shadow'] ? ' box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);' : ''}{$options['rounded'] ? ' border-radius: 8px;' : ''}"
        frameborder="0"
        allowfullscreen
        loading="{$this->getLazyLoad($options)}"
        title="{$this->escapeHtml($form->title)}"
    ></iframe>
</div>
HTML;
        }

        return [
            'code' => trim($code),
            'instructions' => 'Copy and paste this code into your HTML where you want the form to appear.',
        ];
    }

    /**
     * Generate JavaScript widget embed
     */
    private function generateJavaScriptEmbed(Form $form, string $embedId, array $options): array
    {
        $baseUrl = config('app.url');
        $formSlug = $form->slug;
        $optionsJson = json_encode($options);

        $code = <<<HTML
<!-- Revolution Forms Widget -->
<div id="revolution-form-{$embedId}"></div>
<script>
(function(w,d,s,o,f,js,fjs){
    w['RevolutionForms']=o;w[o]=w[o]||function(){(w[o].q=w[o].q||[]).push(arguments)};
    js=d.createElement(s),fjs=d.getElementsByTagName(s)[0];
    js.id=o;js.src=f;js.async=1;fjs.parentNode.insertBefore(js,fjs);
}(window,document,'script','rf','{$baseUrl}/js/forms-widget.js'));

rf('render', '{$formSlug}', {
    container: '#revolution-form-{$embedId}',
    options: {$optionsJson}
});
</script>
HTML;

        return [
            'code' => trim($code),
            'instructions' => 'Add this code to your page. The form will load asynchronously.',
        ];
    }

    /**
     * Generate popup/modal embed
     */
    private function generatePopupEmbed(Form $form, string $embedId, array $options): array
    {
        $baseUrl = config('app.url');
        $formSlug = $form->slug;
        $buttonText = $options['button_text'] ?? 'Open Form';
        $buttonClass = $options['button_class'] ?? '';

        $code = <<<HTML
<!-- Revolution Forms Popup -->
<button
    type="button"
    class="revolution-form-trigger {$buttonClass}"
    data-form="{$formSlug}"
    data-embed-id="{$embedId}"
>
    {$this->escapeHtml($buttonText)}
</button>

<script src="{$baseUrl}/js/forms-popup.js" async></script>
<style>
.revolution-form-trigger {
    padding: 12px 24px;
    background: #3b82f6;
    color: white;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 16px;
    font-weight: 500;
    transition: background 0.2s;
}
.revolution-form-trigger:hover {
    background: #2563eb;
}
</style>
HTML;

        return [
            'code' => trim($code),
            'instructions' => 'Add this button to your page. Clicking it will open the form in a modal.',
        ];
    }

    /**
     * Generate inline injection embed
     */
    private function generateInlineEmbed(Form $form, string $embedId, array $options): array
    {
        $baseUrl = config('app.url');
        $formSlug = $form->slug;

        $code = <<<HTML
<!-- Revolution Forms Inline -->
<div
    class="revolution-form-inline"
    data-form="{$formSlug}"
    data-embed-id="{$embedId}"
    data-theme="{$options['theme']}"
></div>
<script src="{$baseUrl}/js/forms-inline.js" async></script>
HTML;

        return [
            'code' => trim($code),
            'instructions' => 'The form will be rendered inline at this location with your site\'s styles.',
        ];
    }

    /**
     * Generate AMP-compatible embed
     */
    private function generateAmpEmbed(Form $form, array $options): array
    {
        $url = $this->getFormUrl($form, ['embed' => 'amp']);
        $height = $options['height'] === 'auto' ? '600' : $options['height'];

        $code = <<<HTML
<amp-iframe
    width="100"
    height="{$height}"
    layout="responsive"
    sandbox="allow-scripts allow-same-origin allow-forms"
    src="{$url}"
    frameborder="0"
>
    <amp-img
        layout="fill"
        src="{$this->getFormUrl($form)}/placeholder.png"
        placeholder
    ></amp-img>
</amp-iframe>
HTML;

        return [
            'code' => trim($code),
            'instructions' => 'Use this code for AMP pages. Make sure amp-iframe is enabled.',
        ];
    }

    /**
     * Generate oEmbed response
     */
    public function getOEmbed(Form $form, array $params = []): array
    {
        $maxWidth = isset($params['maxwidth']) ? (int) $params['maxwidth'] : 800;
        $maxHeight = isset($params['maxheight']) ? (int) $params['maxheight'] : 600;

        $width = min($maxWidth, 800);
        $height = min($maxHeight, 600);

        $embedHtml = sprintf(
            '<iframe src="%s" width="%d" height="%d" frameborder="0" allowfullscreen title="%s"></iframe>',
            $this->getFormUrl($form, ['embed' => 'oembed']),
            $width,
            $height,
            $this->escapeHtml($form->title)
        );

        return [
            'version' => '1.0',
            'type' => 'rich',
            'provider_name' => config('app.name'),
            'provider_url' => config('app.url'),
            'title' => $form->title,
            'author_name' => $form->user?->name ?? 'Unknown',
            'width' => $width,
            'height' => $height,
            'html' => $embedHtml,
            'thumbnail_url' => $this->getFormThumbnail($form),
            'thumbnail_width' => 400,
            'thumbnail_height' => 300,
        ];
    }

    /**
     * Generate shareable link
     */
    public function generateShareableLink(Form $form, array $options = []): array
    {
        $baseUrl = $this->getFormUrl($form);

        // Add tracking parameters
        $params = [];
        if (!empty($options['utm_source'])) {
            $params['utm_source'] = $options['utm_source'];
        }
        if (!empty($options['utm_medium'])) {
            $params['utm_medium'] = $options['utm_medium'];
        }
        if (!empty($options['utm_campaign'])) {
            $params['utm_campaign'] = $options['utm_campaign'];
        }
        if (!empty($options['ref'])) {
            $params['ref'] = $options['ref'];
        }

        $url = $baseUrl . (!empty($params) ? '?' . http_build_query($params) : '');

        // Generate short URL if requested
        $shortUrl = $options['shorten'] ?? false
            ? $this->shortenUrl($url, $form)
            : null;

        return [
            'url' => $url,
            'short_url' => $shortUrl,
            'qr_code' => $this->generateQRCode($shortUrl ?? $url, $options['qr_size'] ?? 200),
            'social_links' => $this->generateSocialLinks($form, $shortUrl ?? $url),
        ];
    }

    /**
     * Generate QR code URL
     */
    public function generateQRCode(string $url, int $size = 200): string
    {
        // Using Google Charts API for QR generation
        $encodedUrl = urlencode($url);
        return "https://chart.googleapis.com/chart?chs={$size}x{$size}&cht=qr&chl={$encodedUrl}&choe=UTF-8";
    }

    /**
     * Generate social sharing links
     */
    public function generateSocialLinks(Form $form, string $url): array
    {
        $title = urlencode($form->title);
        $encodedUrl = urlencode($url);
        $description = urlencode($form->description ?? '');

        return [
            'facebook' => "https://www.facebook.com/sharer/sharer.php?u={$encodedUrl}",
            'twitter' => "https://twitter.com/intent/tweet?url={$encodedUrl}&text={$title}",
            'linkedin' => "https://www.linkedin.com/sharing/share-offsite/?url={$encodedUrl}",
            'whatsapp' => "https://wa.me/?text={$title}%20{$encodedUrl}",
            'telegram' => "https://t.me/share/url?url={$encodedUrl}&text={$title}",
            'email' => "mailto:?subject={$title}&body={$description}%0A%0A{$encodedUrl}",
            'reddit' => "https://reddit.com/submit?url={$encodedUrl}&title={$title}",
            'pinterest' => "https://pinterest.com/pin/create/button/?url={$encodedUrl}&description={$title}",
        ];
    }

    /**
     * Get all embed types
     */
    public function getEmbedTypes(): array
    {
        return array_map(fn($type, $desc) => [
            'type' => $type,
            'description' => $desc,
        ], array_keys(self::EMBED_TYPES), self::EMBED_TYPES);
    }

    /**
     * Track embed usage
     */
    public function trackEmbed(Form $form, string $type, array $metadata = []): void
    {
        \DB::table('form_embed_analytics')->insert([
            'form_id' => $form->id,
            'embed_type' => $type,
            'referrer' => $metadata['referrer'] ?? null,
            'user_agent' => $metadata['user_agent'] ?? null,
            'ip_hash' => $metadata['ip'] ? hash('sha256', $metadata['ip']) : null,
            'created_at' => now(),
        ]);
    }

    /**
     * Get embed analytics
     */
    public function getEmbedAnalytics(int $formId, int $days = 30): array
    {
        $since = now()->subDays($days);

        return [
            'total_embeds' => \DB::table('form_embed_analytics')
                ->where('form_id', $formId)
                ->where('created_at', '>=', $since)
                ->count(),

            'by_type' => \DB::table('form_embed_analytics')
                ->where('form_id', $formId)
                ->where('created_at', '>=', $since)
                ->select('embed_type')
                ->selectRaw('COUNT(*) as count')
                ->groupBy('embed_type')
                ->pluck('count', 'embed_type')
                ->toArray(),

            'top_referrers' => \DB::table('form_embed_analytics')
                ->where('form_id', $formId)
                ->where('created_at', '>=', $since)
                ->whereNotNull('referrer')
                ->select('referrer')
                ->selectRaw('COUNT(*) as count')
                ->groupBy('referrer')
                ->orderByDesc('count')
                ->limit(10)
                ->get()
                ->toArray(),

            'daily' => \DB::table('form_embed_analytics')
                ->where('form_id', $formId)
                ->where('created_at', '>=', $since)
                ->selectRaw('DATE(created_at) as date, COUNT(*) as count')
                ->groupBy('date')
                ->orderBy('date')
                ->pluck('count', 'date')
                ->toArray(),
        ];
    }

    /**
     * Generate unique embed ID
     */
    private function generateEmbedId(Form $form): string
    {
        return substr(md5($form->id . '-' . time()), 0, 8);
    }

    /**
     * Get form URL
     */
    private function getFormUrl(Form $form, array $params = []): string
    {
        $url = config('app.url') . '/forms/' . $form->slug;
        return !empty($params) ? $url . '?' . http_build_query($params) : $url;
    }

    /**
     * Get form thumbnail URL
     */
    private function getFormThumbnail(Form $form): string
    {
        return config('app.url') . '/api/forms/' . $form->id . '/thumbnail';
    }

    /**
     * Shorten URL
     */
    private function shortenUrl(string $url, Form $form): string
    {
        // Generate short code
        $shortCode = Str::random(8);

        \DB::table('form_short_urls')->insert([
            'code' => $shortCode,
            'form_id' => $form->id,
            'original_url' => $url,
            'created_at' => now(),
        ]);

        return config('app.url') . '/f/' . $shortCode;
    }

    /**
     * Resolve short URL
     */
    public function resolveShortUrl(string $code): ?string
    {
        $record = \DB::table('form_short_urls')
            ->where('code', $code)
            ->first();

        if ($record) {
            // Track click
            \DB::table('form_short_urls')
                ->where('code', $code)
                ->increment('clicks');

            return $record->original_url;
        }

        return null;
    }

    /**
     * Get lazy load attribute value
     */
    private function getLazyLoad(array $options): string
    {
        return ($options['lazy_load'] ?? true) ? 'lazy' : 'eager';
    }

    /**
     * Escape HTML entities
     */
    private function escapeHtml(string $text): string
    {
        return htmlspecialchars($text, ENT_QUOTES, 'UTF-8');
    }
}
