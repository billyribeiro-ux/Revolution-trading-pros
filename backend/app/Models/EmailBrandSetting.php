<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Support\Facades\Cache;

/**
 * EmailBrandSetting Model
 * 
 * Manages brand settings for email templates
 * 
 * @property int $id
 * @property string $brand_name
 * @property string|null $logo_url
 * @property string $primary_color
 * @property string $secondary_color
 * @property string $accent_color
 * @property string $font_family
 * @property int $font_size_base
 * @property array|null $button_style
 * @property string $link_color
 * @property string $background_color
 * @property string|null $footer_text
 * @property array|null $social_links
 * @property bool $is_active
 * @property \Carbon\Carbon $created_at
 * @property \Carbon\Carbon $updated_at
 */
class EmailBrandSetting extends Model
{
    use HasFactory;

    protected $fillable = [
        'brand_name',
        'logo_url',
        'primary_color',
        'secondary_color',
        'accent_color',
        'font_family',
        'font_size_base',
        'button_style',
        'link_color',
        'background_color',
        'footer_text',
        'social_links',
        'is_active',
    ];

    protected $casts = [
        'button_style' => 'array',
        'social_links' => 'array',
        'is_active' => 'boolean',
        'font_size_base' => 'integer',
    ];

    /**
     * Boot method
     */
    protected static function boot()
    {
        parent::boot();

        // Clear cache when brand settings are updated
        static::saved(function () {
            Cache::forget('email_brand_settings_active');
        });

        static::deleted(function () {
            Cache::forget('email_brand_settings_active');
        });
    }

    /**
     * Scopes
     */
    public function scopeActive(Builder $query): Builder
    {
        return $query->where('is_active', true);
    }

    /**
     * Get active brand settings (cached)
     */
    public static function getActive(): ?self
    {
        return Cache::remember('email_brand_settings_active', 3600, function () {
            return static::where('is_active', true)->first();
        });
    }

    /**
     * Activate this brand setting
     */
    public function activate(): void
    {
        // Deactivate all others
        static::where('id', '!=', $this->id)->update(['is_active' => false]);
        
        // Activate this one
        $this->update(['is_active' => true]);
    }

    /**
     * Get CSS variables for email templates
     */
    public function getCssVariables(): array
    {
        return [
            '--primary-color' => $this->primary_color,
            '--secondary-color' => $this->secondary_color,
            '--accent-color' => $this->accent_color,
            '--font-family' => $this->font_family,
            '--font-size-base' => $this->font_size_base . 'px',
            '--link-color' => $this->link_color,
            '--background-color' => $this->background_color,
        ];
    }

    /**
     * Get inline CSS string
     */
    public function getInlineCss(): string
    {
        $vars = $this->getCssVariables();
        $css = '';
        
        foreach ($vars as $key => $value) {
            $css .= "{$key}: {$value}; ";
        }
        
        return trim($css);
    }

    /**
     * Get button style CSS
     */
    public function getButtonCss(): string
    {
        if (!$this->button_style) {
            return "background-color: {$this->primary_color}; color: #ffffff; padding: 12px 24px; border-radius: 4px; text-decoration: none;";
        }

        $css = '';
        foreach ($this->button_style as $property => $value) {
            $css .= "{$property}: {$value}; ";
        }
        
        return trim($css);
    }

    /**
     * Get social links HTML
     */
    public function getSocialLinksHtml(): string
    {
        if (!$this->social_links) {
            return '';
        }

        $html = '<table border="0" cellpadding="0" cellspacing="0" role="presentation" style="margin: 0 auto;"><tr>';
        
        foreach ($this->social_links as $platform => $url) {
            if ($url) {
                $html .= "<td style=\"padding: 0 10px;\"><a href=\"{$url}\"><img src=\"/images/social/{$platform}.png\" alt=\"{$platform}\" width=\"32\" height=\"32\" /></a></td>";
            }
        }
        
        $html .= '</tr></table>';
        
        return $html;
    }
}
