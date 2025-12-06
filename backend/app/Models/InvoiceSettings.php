<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Storage;

/**
 * Invoice Settings Model
 *
 * Stores customizable invoice template settings
 */
class InvoiceSettings extends Model
{
    protected $table = 'invoice_settings';

    protected $fillable = [
        'company_name',
        'company_email',
        'company_phone',
        'company_address',
        'company_city',
        'company_state',
        'company_zip',
        'company_country',
        'tax_id',
        'logo_path',
        'primary_color',
        'secondary_color',
        'accent_color',
        'font_family',
        'header_text',
        'footer_text',
        'payment_terms',
        'notes_template',
        'show_logo',
        'show_tax_id',
        'show_payment_method',
        'show_due_date',
        'invoice_prefix',
        'custom_css',
    ];

    protected $casts = [
        'show_logo' => 'boolean',
        'show_tax_id' => 'boolean',
        'show_payment_method' => 'boolean',
        'show_due_date' => 'boolean',
    ];

    /**
     * Get settings (singleton pattern with caching)
     */
    public static function get(): self
    {
        return Cache::remember('invoice_settings', 3600, function () {
            return self::first() ?? self::create(self::getDefaults());
        });
    }

    /**
     * Update settings and clear cache
     */
    public static function updateSettings(array $data): self
    {
        $settings = self::first() ?? new self();
        $settings->fill($data);
        $settings->save();

        Cache::forget('invoice_settings');

        return $settings;
    }

    /**
     * Get logo URL
     */
    public function getLogoUrlAttribute(): ?string
    {
        if (!$this->logo_path) {
            return null;
        }

        return Storage::disk('public')->url($this->logo_path);
    }

    /**
     * Default settings
     */
    public static function getDefaults(): array
    {
        return [
            'company_name' => config('app.name', 'Your Company'),
            'company_email' => config('mail.from.address', ''),
            'company_phone' => '',
            'company_address' => '',
            'company_city' => '',
            'company_state' => '',
            'company_zip' => '',
            'company_country' => 'US',
            'tax_id' => '',
            'logo_path' => null,
            'primary_color' => '#2563eb',
            'secondary_color' => '#1f2937',
            'accent_color' => '#10b981',
            'font_family' => 'Inter, sans-serif',
            'header_text' => 'INVOICE',
            'footer_text' => 'Thank you for your business!',
            'payment_terms' => 'Due upon receipt',
            'notes_template' => '',
            'show_logo' => true,
            'show_tax_id' => true,
            'show_payment_method' => true,
            'show_due_date' => true,
            'invoice_prefix' => 'INV-',
            'custom_css' => '',
        ];
    }
}
