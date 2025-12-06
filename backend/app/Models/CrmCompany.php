<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUuids;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Builder;

/**
 * CRM Company Model (B2B account-based CRM)
 *
 * @property string $id
 * @property string $name
 * @property string $slug
 * @property string|null $website
 * @property string|null $industry
 * @property string|null $size
 * @property float|null $annual_revenue
 * @property string|null $phone
 * @property string|null $email
 * @property string|null $description
 * @property string|null $address_line1
 * @property string|null $address_line2
 * @property string|null $city
 * @property string|null $state
 * @property string|null $postal_code
 * @property string|null $country
 * @property string|null $linkedin_url
 * @property string|null $twitter_handle
 * @property string|null $logo_url
 * @property array|null $custom_fields
 * @property string|null $owner_id
 * @property int $contacts_count
 * @property int $deals_count
 * @property float $total_deal_value
 */
class CrmCompany extends Model
{
    use HasUuids, SoftDeletes;

    protected $fillable = [
        'name',
        'slug',
        'website',
        'industry',
        'size',
        'annual_revenue',
        'phone',
        'email',
        'description',
        'address_line1',
        'address_line2',
        'city',
        'state',
        'postal_code',
        'country',
        'linkedin_url',
        'twitter_handle',
        'logo_url',
        'custom_fields',
        'owner_id',
    ];

    protected function casts(): array
    {
        return [
            'custom_fields' => 'array',
            'annual_revenue' => 'decimal:2',
            'contacts_count' => 'integer',
            'deals_count' => 'integer',
            'total_deal_value' => 'decimal:2',
        ];
    }

    // Company size options
    public const SIZES = [
        '1-10' => '1-10 employees',
        '11-50' => '11-50 employees',
        '51-200' => '51-200 employees',
        '201-500' => '201-500 employees',
        '501-1000' => '501-1000 employees',
        '1001-5000' => '1001-5000 employees',
        '5001+' => '5001+ employees',
    ];

    // Industry options
    public const INDUSTRIES = [
        'technology' => 'Technology',
        'finance' => 'Finance & Banking',
        'healthcare' => 'Healthcare',
        'education' => 'Education',
        'retail' => 'Retail & E-commerce',
        'manufacturing' => 'Manufacturing',
        'real_estate' => 'Real Estate',
        'consulting' => 'Consulting',
        'marketing' => 'Marketing & Advertising',
        'media' => 'Media & Entertainment',
        'nonprofit' => 'Non-profit',
        'government' => 'Government',
        'other' => 'Other',
    ];

    protected static function booted(): void
    {
        static::creating(function (self $company): void {
            $company->slug ??= str($company->name)->slug()->toString();
        });
    }

    // Relationships
    public function contacts(): HasMany
    {
        return $this->hasMany(Contact::class, 'company_id');
    }

    public function deals(): HasMany
    {
        return $this->hasMany(Deal::class, 'company_id');
    }

    public function owner(): BelongsTo
    {
        return $this->belongsTo(User::class, 'owner_id');
    }

    // Scopes
    public function scopeByIndustry(Builder $query, string $industry): Builder
    {
        return $query->where('industry', $industry);
    }

    public function scopeBySize(Builder $query, string $size): Builder
    {
        return $query->where('size', $size);
    }

    // Business Logic
    public function recalculateCounts(): void
    {
        $this->update([
            'contacts_count' => $this->contacts()->count(),
            'deals_count' => $this->deals()->count(),
            'total_deal_value' => $this->deals()->where('status', 'won')->sum('amount'),
        ]);
    }

    public function getFullAddress(): string
    {
        $parts = array_filter([
            $this->address_line1,
            $this->address_line2,
            $this->city,
            $this->state,
            $this->postal_code,
            $this->country,
        ]);

        return implode(', ', $parts);
    }

    public function getPrimaryContact(): ?Contact
    {
        return $this->contacts()
            ->where('is_vip', true)
            ->orWhere('status', 'customer')
            ->orderByDesc('lead_score')
            ->first();
    }

    public function getOpenDeals(): \Illuminate\Database\Eloquent\Collection
    {
        return $this->deals()->where('status', 'open')->get();
    }

    public function getTotalPipelineValue(): float
    {
        return (float) $this->deals()->where('status', 'open')->sum('amount');
    }

    public function getStats(): array
    {
        return [
            'contacts_count' => $this->contacts_count,
            'deals_count' => $this->deals_count,
            'total_won_value' => $this->total_deal_value,
            'open_pipeline_value' => $this->getTotalPipelineValue(),
            'win_rate' => $this->deals_count > 0
                ? round(($this->deals()->where('status', 'won')->count() / $this->deals_count) * 100, 2)
                : 0,
        ];
    }
}
