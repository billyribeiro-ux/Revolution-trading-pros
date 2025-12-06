<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Support\Facades\Cache;

/**
 * ConsentBannerTemplate Model
 *
 * Manages consent banner template configurations.
 * Supports custom templates and pre-built system templates.
 *
 * @property int $id
 * @property string $name
 * @property string $slug
 * @property string $category
 * @property string|null $description
 * @property bool $is_active
 * @property bool $is_default
 * @property bool $is_system
 * @property string $layout_type
 * @property string $position
 * @property string $position_horizontal
 * @property string $background_color
 * @property string $text_color
 * @property string $link_color
 * @property string $title_color
 * @property string $border_color
 * @property string $border_style
 * @property int $border_width
 * @property string $accept_btn_bg
 * @property string $accept_btn_text
 * @property string $accept_btn_hover_bg
 * @property string $reject_btn_bg
 * @property string $reject_btn_text
 * @property string $reject_btn_hover_bg
 * @property string $settings_btn_bg
 * @property string $settings_btn_text
 * @property string $settings_btn_border
 * @property string $toggle_active_color
 * @property string $toggle_inactive_color
 * @property string $font_family
 * @property int $title_font_size
 * @property int $title_font_weight
 * @property int $body_font_size
 * @property int $body_font_weight
 * @property int $btn_font_size
 * @property int $btn_font_weight
 * @property int $padding_top
 * @property int $padding_bottom
 * @property int $padding_left
 * @property int $padding_right
 * @property int $btn_padding_x
 * @property int $btn_padding_y
 * @property int $btn_margin
 * @property int $btn_border_radius
 * @property int $container_border_radius
 * @property int $container_max_width
 * @property string $animation_type
 * @property int $animation_duration
 * @property string $title
 * @property string|null $description
 * @property string $accept_btn_text
 * @property string $reject_btn_text
 * @property string $settings_btn_text
 * @property string $privacy_link_text
 * @property string|null $privacy_link_url
 * @property string $cookie_link_text
 * @property string|null $cookie_link_url
 * @property bool $show_reject_btn
 * @property bool $show_settings_btn
 * @property bool $show_privacy_link
 * @property bool $show_cookie_link
 * @property bool $close_on_scroll
 * @property int $close_on_scroll_distance
 * @property bool $show_close_btn
 * @property bool $block_page_scroll
 * @property bool $show_powered_by
 * @property string|null $logo_url
 * @property int $logo_size
 * @property string $logo_position
 * @property array|null $full_config
 */
class ConsentBannerTemplate extends Model
{
    use HasFactory;

    protected $table = 'consent_banner_templates';

    protected $fillable = [
        'name',
        'slug',
        'category',
        'description',
        'is_active',
        'is_default',
        'is_system',
        'layout_type',
        'position',
        'position_horizontal',
        'background_color',
        'text_color',
        'link_color',
        'title_color',
        'border_color',
        'border_style',
        'border_width',
        'accept_btn_bg',
        'accept_btn_text',
        'accept_btn_hover_bg',
        'reject_btn_bg',
        'reject_btn_text',
        'reject_btn_hover_bg',
        'settings_btn_bg',
        'settings_btn_text',
        'settings_btn_border',
        'toggle_active_color',
        'toggle_inactive_color',
        'font_family',
        'title_font_size',
        'title_font_weight',
        'body_font_size',
        'body_font_weight',
        'btn_font_size',
        'btn_font_weight',
        'padding_top',
        'padding_bottom',
        'padding_left',
        'padding_right',
        'btn_padding_x',
        'btn_padding_y',
        'btn_margin',
        'btn_border_radius',
        'container_border_radius',
        'container_max_width',
        'animation_type',
        'animation_duration',
        'title',
        'message_text',
        'accept_btn_label',
        'reject_btn_label',
        'settings_btn_label',
        'privacy_link_text',
        'privacy_link_url',
        'cookie_link_text',
        'cookie_link_url',
        'show_reject_btn',
        'show_settings_btn',
        'show_privacy_link',
        'show_cookie_link',
        'close_on_scroll',
        'close_on_scroll_distance',
        'show_close_btn',
        'block_page_scroll',
        'show_powered_by',
        'logo_url',
        'logo_size',
        'logo_position',
        'full_config',
    ];

    protected $casts = [
        'is_active' => 'boolean',
        'is_default' => 'boolean',
        'is_system' => 'boolean',
        'border_width' => 'integer',
        'title_font_size' => 'integer',
        'title_font_weight' => 'integer',
        'body_font_size' => 'integer',
        'body_font_weight' => 'integer',
        'btn_font_size' => 'integer',
        'btn_font_weight' => 'integer',
        'padding_top' => 'integer',
        'padding_bottom' => 'integer',
        'padding_left' => 'integer',
        'padding_right' => 'integer',
        'btn_padding_x' => 'integer',
        'btn_padding_y' => 'integer',
        'btn_margin' => 'integer',
        'btn_border_radius' => 'integer',
        'container_border_radius' => 'integer',
        'container_max_width' => 'integer',
        'animation_duration' => 'integer',
        'close_on_scroll_distance' => 'integer',
        'logo_size' => 'integer',
        'show_reject_btn' => 'boolean',
        'show_settings_btn' => 'boolean',
        'show_privacy_link' => 'boolean',
        'show_cookie_link' => 'boolean',
        'close_on_scroll' => 'boolean',
        'show_close_btn' => 'boolean',
        'block_page_scroll' => 'boolean',
        'show_powered_by' => 'boolean',
        'full_config' => 'array',
    ];

    /**
     * Layout types
     */
    public const LAYOUT_BAR = 'bar';
    public const LAYOUT_POPUP = 'popup';
    public const LAYOUT_FLOATING = 'floating';
    public const LAYOUT_DRAWER = 'drawer';

    /**
     * Position options
     */
    public const POSITION_TOP = 'top';
    public const POSITION_BOTTOM = 'bottom';
    public const POSITION_CENTER = 'center';
    public const POSITION_LEFT = 'left';
    public const POSITION_RIGHT = 'right';

    /**
     * Animation types
     */
    public const ANIMATION_SLIDE = 'slide';
    public const ANIMATION_FADE = 'fade';
    public const ANIMATION_NONE = 'none';

    /**
     * Categories
     */
    public const CATEGORY_MINIMAL = 'minimal';
    public const CATEGORY_CENTERED = 'centered';
    public const CATEGORY_FLOATING = 'floating';
    public const CATEGORY_POPUP = 'popup';
    public const CATEGORY_CUSTOM = 'custom';

    /**
     * Cache keys
     */
    protected const CACHE_ACTIVE = 'consent_template:active';
    protected const CACHE_ALL = 'consent_template:all';
    protected const CACHE_TTL = 3600;

    /**
     * Get the active template
     */
    public static function getActive(): ?static
    {
        return Cache::remember(self::CACHE_ACTIVE, self::CACHE_TTL, function () {
            return static::where('is_active', true)->first()
                ?? static::where('is_default', true)->first();
        });
    }

    /**
     * Get all templates
     */
    public static function getAllTemplates(): \Illuminate\Database\Eloquent\Collection
    {
        return Cache::remember(self::CACHE_ALL, self::CACHE_TTL, function () {
            return static::orderBy('is_system', 'desc')
                ->orderBy('category')
                ->orderBy('name')
                ->get();
        });
    }

    /**
     * Set this template as active
     */
    public function setAsActive(): void
    {
        // Deactivate all other templates
        static::where('id', '!=', $this->id)->update(['is_active' => false]);

        // Activate this template
        $this->update(['is_active' => true]);

        // Clear cache
        self::clearCache();
    }

    /**
     * Clear all template caches
     */
    public static function clearCache(): void
    {
        Cache::forget(self::CACHE_ACTIVE);
        Cache::forget(self::CACHE_ALL);
    }

    /**
     * Boot method
     */
    protected static function boot(): void
    {
        parent::boot();

        static::saved(function () {
            self::clearCache();
        });

        static::deleted(function () {
            self::clearCache();
        });
    }

    /**
     * Get CSS variables for the template
     */
    public function getCssVariables(): array
    {
        return [
            '--consent-bg' => $this->background_color,
            '--consent-text' => $this->text_color,
            '--consent-link' => $this->link_color,
            '--consent-title' => $this->title_color,
            '--consent-border' => $this->border_color,
            '--consent-accept-bg' => $this->accept_btn_bg,
            '--consent-accept-text' => $this->accept_btn_text,
            '--consent-accept-hover' => $this->accept_btn_hover_bg,
            '--consent-reject-bg' => $this->reject_btn_bg,
            '--consent-reject-text' => $this->reject_btn_text,
            '--consent-reject-hover' => $this->reject_btn_hover_bg,
            '--consent-settings-bg' => $this->settings_btn_bg,
            '--consent-settings-text' => $this->settings_btn_text,
            '--consent-settings-border' => $this->settings_btn_border,
            '--consent-toggle-active' => $this->toggle_active_color,
            '--consent-toggle-inactive' => $this->toggle_inactive_color,
            '--consent-font' => $this->font_family,
            '--consent-title-size' => $this->title_font_size . 'px',
            '--consent-title-weight' => $this->title_font_weight,
            '--consent-body-size' => $this->body_font_size . 'px',
            '--consent-body-weight' => $this->body_font_weight,
            '--consent-btn-size' => $this->btn_font_size . 'px',
            '--consent-btn-weight' => $this->btn_font_weight,
            '--consent-padding-top' => $this->padding_top . 'px',
            '--consent-padding-bottom' => $this->padding_bottom . 'px',
            '--consent-padding-left' => $this->padding_left . 'px',
            '--consent-padding-right' => $this->padding_right . 'px',
            '--consent-btn-padding-x' => $this->btn_padding_x . 'px',
            '--consent-btn-padding-y' => $this->btn_padding_y . 'px',
            '--consent-btn-margin' => $this->btn_margin . 'px',
            '--consent-btn-radius' => $this->btn_border_radius . 'px',
            '--consent-radius' => $this->container_border_radius . 'px',
            '--consent-max-width' => $this->container_max_width . 'px',
        ];
    }

    /**
     * Export template as JSON
     */
    public function toExportArray(): array
    {
        return $this->only([
            'name',
            'category',
            'description',
            'layout_type',
            'position',
            'position_horizontal',
            'background_color',
            'text_color',
            'link_color',
            'title_color',
            'border_color',
            'border_style',
            'border_width',
            'accept_btn_bg',
            'accept_btn_text',
            'accept_btn_hover_bg',
            'reject_btn_bg',
            'reject_btn_text',
            'reject_btn_hover_bg',
            'settings_btn_bg',
            'settings_btn_text',
            'settings_btn_border',
            'toggle_active_color',
            'toggle_inactive_color',
            'font_family',
            'title_font_size',
            'title_font_weight',
            'body_font_size',
            'body_font_weight',
            'btn_font_size',
            'btn_font_weight',
            'padding_top',
            'padding_bottom',
            'padding_left',
            'padding_right',
            'btn_padding_x',
            'btn_padding_y',
            'btn_margin',
            'btn_border_radius',
            'container_border_radius',
            'container_max_width',
            'animation_type',
            'animation_duration',
            'title',
            'message_text',
            'accept_btn_label',
            'reject_btn_label',
            'settings_btn_label',
            'privacy_link_text',
            'privacy_link_url',
            'cookie_link_text',
            'cookie_link_url',
            'show_reject_btn',
            'show_settings_btn',
            'show_privacy_link',
            'show_cookie_link',
            'close_on_scroll',
            'close_on_scroll_distance',
            'show_close_btn',
            'block_page_scroll',
            'show_powered_by',
            'logo_url',
            'logo_size',
            'logo_position',
        ]);
    }

    /**
     * Create from import array
     */
    public static function createFromImport(array $data): static
    {
        $data['slug'] = $data['slug'] ?? \Illuminate\Support\Str::slug($data['name']) . '-' . time();
        $data['is_system'] = false;
        $data['is_active'] = false;
        $data['is_default'] = false;

        return static::create($data);
    }

    /**
     * Get system templates defaults
     */
    public static function getSystemTemplates(): array
    {
        return [
            [
                'name' => 'Minimal Dark',
                'slug' => 'minimal-dark',
                'category' => 'minimal',
                'is_system' => true,
                'is_default' => true,
                'layout_type' => 'bar',
                'position' => 'bottom',
                'background_color' => '#1a1f2e',
                'text_color' => '#ffffff',
                'accept_btn_bg' => '#3b82f6',
                'accept_btn_text' => '#ffffff',
                'title' => 'We value your privacy',
            ],
            [
                'name' => 'Minimal Light',
                'slug' => 'minimal-light',
                'category' => 'minimal',
                'is_system' => true,
                'layout_type' => 'bar',
                'position' => 'bottom',
                'background_color' => '#ffffff',
                'text_color' => '#1f2937',
                'accept_btn_bg' => '#2563eb',
                'accept_btn_text' => '#ffffff',
                'title' => 'We value your privacy',
            ],
            [
                'name' => 'Centered Popup',
                'slug' => 'centered-popup',
                'category' => 'popup',
                'is_system' => true,
                'layout_type' => 'popup',
                'position' => 'center',
                'background_color' => '#ffffff',
                'text_color' => '#1f2937',
                'container_border_radius' => 16,
                'accept_btn_bg' => '#059669',
                'accept_btn_text' => '#ffffff',
                'title' => 'Cookie Preferences',
            ],
            [
                'name' => 'Floating Bottom Right',
                'slug' => 'floating-bottom-right',
                'category' => 'floating',
                'is_system' => true,
                'layout_type' => 'floating',
                'position' => 'bottom',
                'position_horizontal' => 'right',
                'background_color' => '#1e293b',
                'text_color' => '#f1f5f9',
                'container_border_radius' => 12,
                'accept_btn_bg' => '#8b5cf6',
                'accept_btn_text' => '#ffffff',
                'title' => 'Cookie Notice',
            ],
            [
                'name' => 'GDPR Strict',
                'slug' => 'gdpr-strict',
                'category' => 'popup',
                'is_system' => true,
                'layout_type' => 'popup',
                'position' => 'center',
                'background_color' => '#ffffff',
                'text_color' => '#374151',
                'container_border_radius' => 8,
                'accept_btn_bg' => '#16a34a',
                'accept_btn_text' => '#ffffff',
                'show_reject_btn' => true,
                'show_settings_btn' => true,
                'block_page_scroll' => true,
                'title' => 'Your Privacy Choices',
            ],
        ];
    }

    /**
     * Initialize system templates
     */
    public static function initializeSystemTemplates(): void
    {
        foreach (self::getSystemTemplates() as $templateData) {
            if (!static::where('slug', $templateData['slug'])->exists()) {
                static::create($templateData);
            }
        }
    }
}
